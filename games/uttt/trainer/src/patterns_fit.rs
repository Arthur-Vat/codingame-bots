//! Fits the pattern policy's log-weights to the moves searches prefer
//! (E015).
//!
//! The model gives a legal move the probability `e^θf / Σ e^θg`, where `f`
//! is the move's feature (`Board::pattern_feature`) and the sum runs over
//! the position's legal moves. θ minimises the cross-entropy against the
//! share of the root's visits each move got, plus a small penalty on θ's
//! size, by Adam on mini-batches of positions. The same code fits the 32
//! move classes of the playout policy, for comparison.

use std::fmt::Write as _;

use cg_core::rng::Rng;
use uttt_engine::board::{CLASSES, PATTERN_FEATURES};

use crate::data::GameRecord;

/// Searched positions reduced to their legal moves' features and visit
/// shares, stored flat.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Examples {
    /// Each move's pattern feature.
    pub patterns: Vec<u32>,
    /// Each move's class in the playout policy.
    pub classes: Vec<u8>,
    /// Each move's share of its position's visits.
    pub shares: Vec<f32>,
    /// Where each position's moves start; one more entry than positions.
    pub starts: Vec<u32>,
    /// Games added, finished or not.
    pub games: usize,
}

impl Examples {
    pub fn positions(&self) -> usize {
        self.starts.len().saturating_sub(1)
    }

    pub fn moves(&self) -> usize {
        self.shares.len()
    }

    fn range(&self, position: usize) -> std::ops::Range<usize> {
        self.starts[position] as usize..self.starts[position + 1] as usize
    }
}

/// The fitted and held-out examples of self-play games: every 20th game is
/// held out whole. Games are added file by file.
#[derive(Clone, Debug, Default)]
pub struct Split {
    pub fitted: Examples,
    pub held_out: Examples,
    games: usize,
}

impl Split {
    /// Adds the searched positions of `games` that recorded visits and have
    /// no game-winning move, which playouts and the search play at once.
    pub fn add(&mut self, games: &[GameRecord]) {
        for game in games {
            let set = if self.games % 20 == 19 {
                &mut self.held_out
            } else {
                &mut self.fitted
            };
            self.games += 1;
            set.games += 1;
            for (board, searched) in game.positions() {
                let total: u64 = searched.visits.iter().map(|&(_, n)| u64::from(n)).sum();
                if total == 0 || board.game_winning_move().is_some() {
                    continue;
                }
                if set.starts.is_empty() {
                    set.starts.push(0);
                }
                for &(mv, visits) in &searched.visits {
                    set.patterns.push(board.pattern_feature(mv) as u32);
                    set.classes.push(board.move_class(mv) as u8);
                    set.shares.push((f64::from(visits) / total as f64) as f32);
                }
                set.starts.push(set.shares.len() as u32);
            }
        }
    }
}

/// Which feature of a move a model weighs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Model {
    /// One weight per canonical pattern, cell and destination.
    Patterns,
    /// One weight per playout-policy class.
    Classes,
}

impl Model {
    pub fn features(self) -> usize {
        match self {
            Model::Patterns => PATTERN_FEATURES,
            Model::Classes => CLASSES,
        }
    }

    fn feature(self, examples: &Examples, index: usize) -> usize {
        match self {
            Model::Patterns => examples.patterns[index] as usize,
            Model::Classes => usize::from(examples.classes[index]),
        }
    }
}

/// How well a model predicts the search's visits.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Metrics {
    /// Average cross-entropy against the visit shares, in nats per
    /// position.
    pub cross_entropy: f64,
    /// Average probability of the most visited move.
    pub best_move: f64,
}

/// The probabilities `theta` gives the moves of position `position`,
/// written into `probabilities`.
fn probabilities(
    model: Model,
    theta: &[f32],
    examples: &Examples,
    position: usize,
    probabilities: &mut Vec<f64>,
) {
    probabilities.clear();
    let range = examples.range(position);
    let top = range
        .clone()
        .map(|index| theta[model.feature(examples, index)])
        .fold(f32::NEG_INFINITY, f32::max);
    probabilities.extend(
        range
            .clone()
            .map(|index| f64::from(theta[model.feature(examples, index)] - top).exp()),
    );
    let total: f64 = probabilities.iter().sum();
    for value in probabilities.iter_mut() {
        *value /= total;
    }
}

/// The metrics of `theta` on `examples`.
pub fn measure(model: Model, theta: &[f32], examples: &Examples) -> Metrics {
    let mut sums = (0.0, 0.0);
    let mut p = Vec::new();
    for position in 0..examples.positions() {
        probabilities(model, theta, examples, position, &mut p);
        let range = examples.range(position);
        let shares = &examples.shares[range];
        sums.0 -= shares
            .iter()
            .zip(&p)
            .map(|(&share, &prob)| f64::from(share) * prob.max(1e-12).ln())
            .sum::<f64>();
        let best = shares
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(b.1))
            .map_or(0, |(index, _)| index);
        sums.1 += p[best];
    }
    let count = examples.positions().max(1) as f64;
    Metrics {
        cross_entropy: sums.0 / count,
        best_move: sums.1 / count,
    }
}

/// How a model is fitted.
#[derive(Clone, Copy, Debug)]
pub struct Fitting {
    pub epochs: u32,
    /// Positions per step.
    pub batch: usize,
    /// Adam's step size.
    pub rate: f32,
    /// Weight of the penalty on θ's squares, per position.
    pub penalty: f32,
    pub seed: u64,
}

/// Fits θ for `model` on `examples`, from 0; calls `report` with the
/// epoch's number and θ after each epoch.
pub fn fit(
    model: Model,
    examples: &Examples,
    settings: &Fitting,
    mut report: impl FnMut(u32, &[f32]),
) -> Vec<f32> {
    let features = model.features();
    let mut theta = vec![0.0f32; features];
    let (mut first, mut second) = (vec![0.0f32; features], vec![0.0f32; features]);
    let mut gradient = vec![0.0f32; features];
    let (beta1, beta2, epsilon) = (0.9f32, 0.999f32, 1e-8f32);
    let mut order: Vec<u32> = (0..examples.positions() as u32).collect();
    let mut rng = Rng::new(settings.seed);
    let mut p = Vec::new();
    let mut step: i32 = 0;
    for epoch in 1..=settings.epochs {
        rng.shuffle(&mut order);
        for batch in order.chunks(settings.batch.max(1)) {
            gradient.fill(0.0);
            for &position in batch {
                let position = position as usize;
                probabilities(model, &theta, examples, position, &mut p);
                for (index, &prob) in examples.range(position).zip(&p) {
                    let feature = model.feature(examples, index);
                    gradient[feature] += prob as f32 - examples.shares[index];
                }
            }
            step += 1;
            let scale = 1.0 / batch.len() as f32;
            let (correction1, correction2) = (1.0 - beta1.powi(step), 1.0 - beta2.powi(step));
            for (((value, g), m), v) in theta
                .iter_mut()
                .zip(&gradient)
                .zip(first.iter_mut())
                .zip(second.iter_mut())
            {
                let g = g * scale + settings.penalty * *value;
                *m = beta1 * *m + (1.0 - beta1) * g;
                *v = beta2 * *v + (1.0 - beta2) * g * g;
                *value -=
                    settings.rate * (*m / correction1) / ((*v / correction2).sqrt() + epsilon);
            }
        }
        report(epoch, &theta);
    }
    theta
}

/// The pattern weights' text as Rust source for the bot: base64 digits in
/// lines of 96, joined by the string's line continuations.
pub fn weights_source(text: &str, origin: &str) -> String {
    let mut source = String::new();
    let _ = writeln!(
        source,
        "//! Generated by `uttt-trainer fit-patterns`: {origin}."
    );
    let _ = writeln!(source);
    let _ = writeln!(
        source,
        "/// The pattern policy's log-weights (E015), one base64 digit each, for\n/// `uttt_engine::PatternPolicy::decode`."
    );
    let _ = writeln!(source, "pub const PATTERN_TEXT: &str = \"\\");
    let lines: Vec<&str> = text
        .as_bytes()
        .chunks(96)
        .map(|line| std::str::from_utf8(line).expect("base64 is ASCII"))
        .collect();
    for (index, line) in lines.iter().enumerate() {
        let end = if index + 1 == lines.len() {
            "\";"
        } else {
            "\\"
        };
        let _ = writeln!(source, "{line}{end}");
    }
    source
}

#[cfg(test)]
mod tests;
