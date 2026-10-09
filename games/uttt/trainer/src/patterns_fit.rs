//! Fits move models to the moves searches prefer: E015's pattern policy,
//! and the larger models compared for E016 ([`models`]).
//!
//! A model gives a legal move the probability `e^l / Σ e^m`, where `l` is
//! the move's log-weight (the sum of its features' weights, one per table
//! of the model) and the sum runs over the position's legal moves. The
//! weights minimise the cross-entropy against the share of the root's
//! visits each move got, plus a small penalty on their size, by Adam on
//! mini-batches of positions.

use std::fmt::Write as _;

use cg_core::rng::Rng;

use crate::data::GameRecord;

mod models;

pub use models::{Model, ALL};

/// Searched positions reduced to their legal moves' features and visit
/// shares, for one model, stored flat.
#[derive(Clone, Debug, PartialEq)]
pub struct Examples {
    /// Each move's features, `per_move` indices in a row.
    pub features: Vec<u32>,
    /// Features per move: the model's tables.
    pub per_move: usize,
    /// Each move's share of its position's visits.
    pub shares: Vec<f32>,
    /// Where each position's moves start; one more entry than positions.
    pub starts: Vec<u32>,
    /// Games added, finished or not.
    pub games: usize,
}

impl Examples {
    fn new(per_move: usize) -> Self {
        Examples {
            features: Vec::new(),
            per_move,
            shares: Vec::new(),
            starts: vec![0],
            games: 0,
        }
    }

    pub fn positions(&self) -> usize {
        self.starts.len() - 1
    }

    pub fn moves(&self) -> usize {
        self.shares.len()
    }

    fn range(&self, position: usize) -> std::ops::Range<usize> {
        self.starts[position] as usize..self.starts[position + 1] as usize
    }

    /// The features of move `index`.
    fn of(&self, index: usize) -> &[u32] {
        &self.features[index * self.per_move..][..self.per_move]
    }
}

/// The fitted and held-out examples of self-play games for one model:
/// every 20th game is held out whole. Games are added file by file.
#[derive(Clone, Debug)]
pub struct Split {
    pub model: Model,
    pub fitted: Examples,
    pub held_out: Examples,
    games: usize,
}

impl Split {
    pub fn new(model: Model) -> Self {
        let per_move = model.tables().len();
        Split {
            model,
            fitted: Examples::new(per_move),
            held_out: Examples::new(per_move),
            games: 0,
        }
    }

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
                for &(mv, visits) in &searched.visits {
                    self.model.features(&board, mv, &mut set.features);
                    set.shares.push((f64::from(visits) / total as f64) as f32);
                }
                set.starts.push(set.shares.len() as u32);
            }
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

/// The log-weight `theta` gives move `index`.
fn log_weight(theta: &[f32], examples: &Examples, index: usize) -> f32 {
    examples
        .of(index)
        .iter()
        .map(|&feature| theta[feature as usize])
        .sum()
}

/// The probabilities `theta` gives the moves of position `position`,
/// written into `probabilities`.
fn probabilities(
    theta: &[f32],
    examples: &Examples,
    position: usize,
    probabilities: &mut Vec<f64>,
) {
    probabilities.clear();
    let range = examples.range(position);
    probabilities.extend(
        range
            .clone()
            .map(|index| f64::from(log_weight(theta, examples, index))),
    );
    let top = probabilities
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    for value in probabilities.iter_mut() {
        *value = (*value - top).exp();
    }
    let total: f64 = probabilities.iter().sum();
    for value in probabilities.iter_mut() {
        *value /= total;
    }
}

/// The metrics of `theta` on `examples`.
pub fn measure(theta: &[f32], examples: &Examples) -> Metrics {
    let mut sums = (0.0, 0.0);
    let mut p = Vec::new();
    for position in 0..examples.positions() {
        probabilities(theta, examples, position, &mut p);
        let shares = &examples.shares[examples.range(position)];
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
    /// Weight of the penalty on the weights' squares, per position.
    pub penalty: f32,
    pub seed: u64,
}

/// Fits `weights` log-weights on `examples`, from 0; calls `report` with
/// the epoch's number and the weights after each epoch.
pub fn fit(
    weights: usize,
    examples: &Examples,
    settings: &Fitting,
    mut report: impl FnMut(u32, &[f32]),
) -> Vec<f32> {
    let mut theta = vec![0.0f32; weights];
    let (mut first, mut second) = (vec![0.0f32; weights], vec![0.0f32; weights]);
    let mut gradient = vec![0.0f32; weights];
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
                probabilities(&theta, examples, position, &mut p);
                for (index, &prob) in examples.range(position).zip(&p) {
                    let error = prob as f32 - examples.shares[index];
                    for &feature in examples.of(index) {
                        gradient[feature as usize] += error;
                    }
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

/// The weights `theta` divided by `temperature`, each rounded to the
/// nearest quarter between -8 and 7.75, as numbers from 0 to 63: `q` for a
/// log-weight of `(q - 32) / 4`.
pub fn quantize(theta: &[f32], temperature: f32) -> Vec<u8> {
    theta
        .iter()
        .map(|&value| ((value / temperature * 4.0).round() + 32.0).clamp(0.0, 63.0) as u8)
        .collect()
}

/// `model`'s quantized weights packed for a bot: each table in turn, in
/// as many segments as its stride ([`Model::strides`]).
pub fn packed_text(model: Model, quantized: &[u8]) -> String {
    let mut text = String::new();
    let mut offset = 0;
    for (size, stride) in model.tables().into_iter().zip(model.strides()) {
        text += &cg_core::packed::encode_strided(&quantized[offset..offset + size], stride);
        offset += size;
    }
    text
}

/// The weights' text of `model` as Rust source: base64 digits in lines of
/// 96, joined by the string's line continuations. One digit per weight
/// for [`Model::Patterns`], E015's format; packed for the other models.
pub fn weights_source(model: Model, text: &str, origin: &str) -> String {
    let mut source = String::new();
    let _ = writeln!(
        source,
        "//! Generated by `uttt-trainer fit-patterns`: {origin}."
    );
    let _ = writeln!(source);
    let name = if model == Model::Patterns {
        let _ = writeln!(
            source,
            "/// The pattern policy's log-weights (E015), one base64 digit each, for\n/// `uttt_engine::PatternPolicy::decode`."
        );
        "PATTERN_TEXT"
    } else {
        let _ = writeln!(
            source,
            "/// The log-weights of the `{}` model of `uttt-trainer fit-patterns`,\n/// packed by `cg_core::packed`: tables of {:?} weights, in {:?}\n/// segments each.",
            model.name(),
            model.tables(),
            model.strides()
        );
        "PACKED_TEXT"
    };
    let _ = writeln!(source, "pub const {name}: &str = \"\\");
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
