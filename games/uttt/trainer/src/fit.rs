//! Fits the playout policy's class weights to the moves searches prefer.
//!
//! The model gives a move of class `k` the probability `e^θk / Σ e^θc`
//! over the legal moves `c`, and θ minimises the cross-entropy between
//! that and the share of the root's visits each move got, plus a small
//! penalty on θ's size. The problem is convex; plain gradient descent
//! with Adam's step sizes solves it.

use std::fmt::Write as _;

use uttt_engine::board::{CLASSES, FEATURE_NAMES};

use crate::data::GameRecord;

/// One searched position, reduced to its move classes: how many legal
/// moves fall in each class, and the share of the visits they got.
#[derive(Clone, Debug, PartialEq)]
pub struct Example {
    pub moves: [u16; CLASSES],
    pub visits: [f64; CLASSES],
    /// The class of the most visited move.
    pub best: usize,
}

/// The examples of every searched position of `games`, skipping those
/// with a game-winning move (playouts play it anyway) or without visits.
pub fn examples(games: &[GameRecord]) -> Vec<Example> {
    let mut all = Vec::new();
    for game in games {
        for (board, searched) in game.positions() {
            let total: u64 = searched.visits.iter().map(|&(_, n)| u64::from(n)).sum();
            if total == 0 || board.game_winning_move().is_some() {
                continue;
            }
            let mut example = Example {
                moves: [0; CLASSES],
                visits: [0.0; CLASSES],
                best: 0,
            };
            let mut most = 0;
            for &(mv, visits) in &searched.visits {
                let class = board.move_class(mv);
                example.moves[class] += 1;
                example.visits[class] += f64::from(visits) / total as f64;
                if visits > most {
                    most = visits;
                    example.best = class;
                }
            }
            all.push(example);
        }
    }
    all
}

/// The average cross-entropy of the policy `theta` on `examples`, in nats
/// per position, and its gradient; without the penalty.
pub fn loss_and_gradient(theta: &[f64; CLASSES], examples: &[Example]) -> (f64, [f64; CLASSES]) {
    let mut loss = 0.0;
    let mut gradient = [0.0; CLASSES];
    for example in examples {
        let shift = (0..CLASSES)
            .filter(|&k| example.moves[k] > 0)
            .map(|k| theta[k])
            .fold(f64::NEG_INFINITY, f64::max);
        let partition: f64 = (0..CLASSES)
            .map(|k| f64::from(example.moves[k]) * (theta[k] - shift).exp())
            .sum();
        let log_partition = partition.ln() + shift;
        for k in 0..CLASSES {
            if example.moves[k] == 0 {
                continue;
            }
            let probability = f64::from(example.moves[k]) * (theta[k] - log_partition).exp();
            loss -= example.visits[k] * (theta[k] - log_partition);
            gradient[k] += probability - example.visits[k];
        }
    }
    let count = examples.len().max(1) as f64;
    for value in &mut gradient {
        *value /= count;
    }
    (loss / count, gradient)
}

/// The probability the policy gives the class of the most visited move,
/// on average: how often it would pick what the search preferred.
pub fn best_move_probability(theta: &[f64; CLASSES], examples: &[Example]) -> f64 {
    let mut sum = 0.0;
    for example in examples {
        let partition: f64 = (0..CLASSES)
            .map(|k| f64::from(example.moves[k]) * theta[k].exp())
            .sum();
        sum += theta[example.best].exp() / partition;
    }
    sum / examples.len().max(1) as f64
}

/// Fits θ by gradient descent with Adam's step sizes; `penalty` weighs
/// `θ²/2`. Returns θ and the final loss without the penalty.
pub fn fit(examples: &[Example], steps: u32, penalty: f64) -> ([f64; CLASSES], f64) {
    let mut theta = [0.0; CLASSES];
    let (mut first, mut second) = ([0.0; CLASSES], [0.0; CLASSES]);
    let (rate, beta1, beta2, epsilon) = (0.05, 0.9, 0.999, 1e-8);
    for step in 1..=steps {
        let (_, mut gradient) = loss_and_gradient(&theta, examples);
        for k in 0..CLASSES {
            gradient[k] += penalty * theta[k];
            first[k] = beta1 * first[k] + (1.0 - beta1) * gradient[k];
            second[k] = beta2 * second[k] + (1.0 - beta2) * gradient[k] * gradient[k];
            let first_hat = first[k] / (1.0 - beta1.powi(step as i32));
            let second_hat = second[k] / (1.0 - beta2.powi(step as i32));
            theta[k] -= rate * first_hat / (second_hat.sqrt() + epsilon);
        }
    }
    let (loss, _) = loss_and_gradient(&theta, examples);
    (theta, loss)
}

/// Integer weights for the bot: the largest is `scale`, none below 1, and
/// `temperature` above 1 flattens the policy.
pub fn weights(theta: &[f64; CLASSES], temperature: f64, scale: u32) -> [u32; CLASSES] {
    let top = theta.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    std::array::from_fn(|k| {
        let weight = f64::from(scale) * ((theta[k] - top) / temperature).exp();
        (weight.round() as u32).clamp(1, scale)
    })
}

/// The features of a class, by name.
pub fn class_name(class: usize) -> String {
    let names: Vec<&str> = FEATURE_NAMES
        .iter()
        .enumerate()
        .filter(|&(bit, _)| class & (1 << bit) != 0)
        .map(|(_, &name)| name)
        .collect();
    if names.is_empty() {
        "none".to_string()
    } else {
        names.join(", ")
    }
}

/// The weights as Rust source for the bot.
pub fn weights_source(weights: &[u32; CLASSES], origin: &str) -> String {
    let mut source = String::new();
    writeln!(
        source,
        "//! Generated by `uttt-trainer fit-policy`: {origin}."
    )
    .unwrap();
    writeln!(source).unwrap();
    writeln!(
        source,
        "/// Weights of the playout policy's move classes (ADR 0016); see"
    )
    .unwrap();
    writeln!(source, "/// `uttt_engine::board::PlayoutPolicy`.").unwrap();
    writeln!(source, "pub const PLAYOUT_WEIGHTS: [u32; {CLASSES}] = [").unwrap();
    for chunk in weights.chunks(8) {
        let row: Vec<String> = chunk.iter().map(u32::to_string).collect();
        writeln!(source, "    {},", row.join(", ")).unwrap();
    }
    writeln!(source, "];").unwrap();
    source
}

#[cfg(test)]
mod tests;
