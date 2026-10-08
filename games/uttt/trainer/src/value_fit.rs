//! Trains the value network of ADR 0017 on self-play games, and measures
//! it against `uttt-v008`'s playouts: the gate of the ADR's decision 5.
//!
//! The parameters are one flat vector, in the order of
//! `uttt_engine::value::Network`. Training minimises the cross-entropy
//! between the network's output and the game's result for the side to
//! move (1, ½ or 0), with Adam on mini-batches spread over threads. Each
//! position is shown in one of the board's 8 symmetries, drawn at random.

use std::fmt::Write as _;

use cg_core::rng::Rng;
use uttt_engine::value::{
    active_inputs, ValueNetwork, CLOSED, INPUTS, MAX_ACTIVE, TARGET, THEIR_CELLS, THREATS,
};
use uttt_engine::{Board, PlayoutPolicy, Status};

use crate::data::GameRecord;

/// Units of the first hidden layer.
const H: usize = 64;
/// Units of the second hidden layer.
const H2: usize = 16;
/// Where each parameter group starts.
const INPUT: usize = 0;
const INPUT_BIAS: usize = INPUT + INPUTS * H;
const HIDDEN: usize = INPUT_BIAS + H;
const HIDDEN_BIAS: usize = HIDDEN + H2 * H;
const OUTPUT: usize = HIDDEN_BIAS + H2;
const OUTPUT_BIAS: usize = OUTPUT + H2;
/// The number of parameters.
pub const PARAMETERS: usize = OUTPUT_BIAS + 1;

/// One position to learn from.
#[derive(Clone, Copy, Debug)]
pub struct Example {
    pub board: Board,
    /// The game's result for the side to move: 1, 0.5 or 0.
    pub result: f32,
    /// The search's average score at the root for the side to move, or the
    /// result in data without scores.
    pub score: f32,
}

/// What the network learns to predict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    /// The game's result, as in AlphaZero (ADR 0017).
    Result,
    /// The search's root score at that position.
    Score,
    /// The average of the two.
    Mix,
}

impl Target {
    fn of(self, example: &Example) -> f32 {
        match self {
            Target::Result => example.result,
            Target::Score => example.score,
            Target::Mix => 0.5 * (example.result + example.score),
        }
    }
}

/// The examples of `games`, split into those to fit and those held out:
/// every 20th game is held out whole, since positions of one game are
/// alike. Positions where the side to move can win the game at once are
/// left out: the search never asks the network there.
pub fn examples(games: &[GameRecord]) -> (Vec<Example>, Vec<Example>) {
    let (mut fitted, mut held_out) = (Vec::new(), Vec::new());
    for (index, game) in games.iter().enumerate() {
        let status = game.status();
        if status == Status::Ongoing {
            continue;
        }
        let set = if index % 20 == 19 {
            &mut held_out
        } else {
            &mut fitted
        };
        for (board, searched) in game.positions() {
            if board.game_winning_move().is_some() {
                continue;
            }
            let result = match status {
                Status::Win(seat) if seat == board.to_move() => 1.0,
                Status::Win(_) => 0.0,
                _ => 0.5,
            };
            let score = searched
                .score
                .filter(|score| score.is_finite())
                .unwrap_or(result);
            set.push(Example {
                board,
                result,
                score,
            });
        }
    }
    (fitted, held_out)
}

/// The cell that `cell` (`3 * row + col`) becomes under symmetry `index`
/// of the 3×3 grid: the identity, three rotations and four reflections.
pub fn symmetric_cell(index: usize, cell: usize) -> usize {
    let (row, col) = (cell / 3, cell % 3);
    let (row, col) = match index {
        0 => (row, col),
        1 => (col, 2 - row),
        2 => (2 - row, 2 - col),
        3 => (2 - col, row),
        4 => (row, 2 - col),
        5 => (2 - row, col),
        6 => (col, row),
        _ => (2 - col, 2 - row),
    };
    3 * row + col
}

/// For each of the 8 symmetries, the input each input becomes when the same
/// symmetry is applied to the big grid and to every small board: the game
/// stays the same, since a move's cell names the next small board.
pub fn input_symmetries() -> Vec<[u16; INPUTS]> {
    (0..8)
        .map(|index| {
            let map = |cell: usize| symmetric_cell(index, cell);
            std::array::from_fn(|input| {
                let image = match input {
                    _ if input < CLOSED => {
                        let offset = if input < THEIR_CELLS { 0 } else { THEIR_CELLS };
                        let cell = input - offset;
                        offset + 9 * map(cell / 9) + map(cell % 9)
                    }
                    _ if input < TARGET => {
                        let closed = input - CLOSED;
                        CLOSED + 3 * map(closed / 3) + closed % 3
                    }
                    _ if input < THREATS => {
                        let target = input - TARGET;
                        TARGET + if target == 9 { 9 } else { map(target) }
                    }
                    _ => {
                        let threat = input - THREATS;
                        let offset = if threat < 9 { 0 } else { 9 };
                        THREATS + offset + map(threat - offset)
                    }
                };
                image as u16
            })
        })
        .collect()
}

/// Values of the network's layers for one position.
struct Forward {
    first_pre: [f32; H],
    first: [f32; H],
    second_pre: [f32; H2],
    second: [f32; H2],
    /// The output before the sigmoid.
    z: f32,
}

/// The dot product of two equally long slices, summed in 8 lanes.
fn dot(a: &[f32], b: &[f32]) -> f32 {
    let mut lanes = [0.0f32; 8];
    for (x, y) in a.chunks_exact(8).zip(b.chunks_exact(8)) {
        for (lane, (x, y)) in lanes.iter_mut().zip(x.iter().zip(y)) {
            *lane += x * y;
        }
    }
    let tail: f32 = a
        .chunks_exact(8)
        .remainder()
        .iter()
        .zip(b.chunks_exact(8).remainder())
        .map(|(x, y)| x * y)
        .sum();
    lanes.iter().sum::<f32>() + tail
}

fn forward(parameters: &[f32], active: &[u16]) -> Forward {
    let mut first_pre: [f32; H] = std::array::from_fn(|unit| parameters[INPUT_BIAS + unit]);
    for &input in active {
        let row = &parameters[INPUT + usize::from(input) * H..][..H];
        for (value, weight) in first_pre.iter_mut().zip(row) {
            *value += weight;
        }
    }
    let first = first_pre.map(|value| value.clamp(0.0, 1.0));
    let second_pre: [f32; H2] = std::array::from_fn(|unit| {
        parameters[HIDDEN_BIAS + unit] + dot(&first, &parameters[HIDDEN + unit * H..][..H])
    });
    let second = second_pre.map(|value| value.clamp(0.0, 1.0));
    let z = parameters[OUTPUT_BIAS] + dot(&second, &parameters[OUTPUT..][..H2]);
    Forward {
        first_pre,
        first,
        second_pre,
        second,
        z,
    }
}

fn sigmoid(z: f32) -> f32 {
    1.0 / (1.0 + (-z).exp())
}

/// The cross-entropy of output `z` (before the sigmoid) against `target`,
/// computed without overflow.
fn cross_entropy(z: f32, target: f32) -> f64 {
    let z = f64::from(z);
    z.max(0.0) - z * f64::from(target) + (-z.abs()).exp().ln_1p()
}

/// Adds the gradient of the cross-entropy of one position to `gradient`,
/// and returns the cross-entropy.
fn accumulate(parameters: &[f32], active: &[u16], target: f32, gradient: &mut [f32]) -> f64 {
    let values = forward(parameters, active);
    let dz = sigmoid(values.z) - target;
    gradient[OUTPUT_BIAS] += dz;
    let mut d_first = [0.0f32; H];
    for unit in 0..H2 {
        gradient[OUTPUT + unit] += dz * values.second[unit];
        let pre = values.second_pre[unit];
        if pre <= 0.0 || pre >= 1.0 {
            continue;
        }
        let d = dz * parameters[OUTPUT + unit];
        gradient[HIDDEN_BIAS + unit] += d;
        let row = HIDDEN + unit * H;
        let weights = &parameters[row..][..H];
        let row_gradient = &mut gradient[row..][..H];
        for (((g, x), w), back) in row_gradient
            .iter_mut()
            .zip(&values.first)
            .zip(weights)
            .zip(d_first.iter_mut())
        {
            *g += d * x;
            *back += d * w;
        }
    }
    for (back, pre) in d_first.iter_mut().zip(&values.first_pre) {
        if *pre <= 0.0 || *pre >= 1.0 {
            *back = 0.0;
        }
    }
    for &input in active {
        let row = &mut gradient[INPUT + usize::from(input) * H..][..H];
        for (g, back) in row.iter_mut().zip(&d_first) {
            *g += back;
        }
    }
    for (g, back) in gradient[INPUT_BIAS..][..H].iter_mut().zip(&d_first) {
        *g += back;
    }
    cross_entropy(values.z, target)
}

/// The active inputs of `board` seen through symmetry `map`.
fn symmetric_inputs(board: &Board, map: &[u16; INPUTS], list: &mut [u16; MAX_ACTIVE]) -> usize {
    let count = active_inputs(board, list);
    for input in &mut list[..count] {
        *input = map[usize::from(*input)];
    }
    count
}

/// How well a network predicts results.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Metrics {
    /// Average cross-entropy, in nats per position.
    pub cross_entropy: f64,
    /// Average squared difference between the output and the result.
    pub squared_error: f64,
}

/// The metrics of `parameters` on `examples`, without symmetries, over
/// `threads` threads.
pub fn measure(parameters: &[f32], examples: &[Example], threads: usize) -> Metrics {
    let chunk = examples.len().div_ceil(threads.max(1)).max(1);
    let sums: Vec<(f64, f64)> = std::thread::scope(|scope| {
        let handles: Vec<_> = examples
            .chunks(chunk)
            .map(|part| {
                scope.spawn(move || {
                    let mut list = [0; MAX_ACTIVE];
                    let (mut entropy, mut squared) = (0.0, 0.0);
                    for example in part {
                        let count = active_inputs(&example.board, &mut list);
                        let z = forward(parameters, &list[..count]).z;
                        entropy += cross_entropy(z, example.result);
                        squared += f64::from(sigmoid(z) - example.result).powi(2);
                    }
                    (entropy, squared)
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().expect("a measuring thread panicked"))
            .collect()
    });
    let count = examples.len().max(1) as f64;
    Metrics {
        cross_entropy: sums.iter().map(|sum| sum.0).sum::<f64>() / count,
        squared_error: sums.iter().map(|sum| sum.1).sum::<f64>() / count,
    }
}

/// How the network is trained.
#[derive(Clone, Copy, Debug)]
pub struct Training {
    /// Passes over the fitted positions.
    pub epochs: u32,
    /// Positions per gradient step.
    pub batch: usize,
    /// Adam's step size at the start; it falls along a cosine to a
    /// twentieth of it at the end.
    pub rate: f32,
    /// Weight of the penalty on the parameters' squares.
    pub penalty: f32,
    /// What the network learns to predict; it is measured against results
    /// whatever this is.
    pub target: Target,
    pub threads: usize,
    pub seed: u64,
}

/// What an epoch of training gave.
#[derive(Clone, Copy, Debug)]
pub struct Epoch {
    pub number: u32,
    /// Average cross-entropy over the epoch's fitted positions, in their
    /// symmetries, as the parameters changed.
    pub fitted: f64,
    pub held_out: Metrics,
}

/// Starting parameters: small random weights, biases that keep the hidden
/// units between their bounds.
pub fn initial_parameters(seed: u64) -> Vec<f32> {
    let mut rng = Rng::new(seed);
    let mut uniform = |spread: f64| ((rng.unit() * 2.0 - 1.0) * spread) as f32;
    let mut parameters = Vec::with_capacity(PARAMETERS);
    // About 40 inputs are active in a position.
    parameters.extend((0..INPUTS * H).map(|_| uniform((3.0f64 / 40.0).sqrt())));
    parameters.extend([0.5; H]);
    parameters.extend((0..H2 * H).map(|_| uniform(1.0 / (H as f64).sqrt())));
    parameters.extend([0.5; H2]);
    parameters.extend((0..H2).map(|_| uniform(1.0 / (H2 as f64).sqrt())));
    parameters.push(0.0);
    parameters
}

/// Trains from `parameters` on `fitted`, measuring on `held_out` after each
/// epoch, and calls `report` then.
pub fn train(
    mut parameters: Vec<f32>,
    fitted: &[Example],
    held_out: &[Example],
    settings: &Training,
    mut report: impl FnMut(&Epoch),
) -> Vec<f32> {
    let symmetries = input_symmetries();
    let threads = settings.threads.max(1);
    let mut rng = Rng::new(settings.seed);
    let (mut first, mut second) = (vec![0.0f32; PARAMETERS], vec![0.0f32; PARAMETERS]);
    let (beta1, beta2, epsilon) = (0.9f32, 0.999f32, 1e-8f32);
    let steps_per_epoch = fitted.len().div_ceil(settings.batch);
    let total_steps = (steps_per_epoch * settings.epochs as usize).max(1);
    let mut step: u32 = 0;
    let mut gradients = vec![vec![0.0f32; PARAMETERS]; threads];
    let mut order: Vec<u32> = (0..fitted.len() as u32).collect();
    for number in 1..=settings.epochs {
        rng.shuffle(&mut order);
        let mut epoch_loss = 0.0;
        for batch in order.chunks(settings.batch) {
            let views: Vec<(u32, u8)> = batch
                .iter()
                .map(|&index| (index, rng.below(8) as u8))
                .collect();
            let part = views.len().div_ceil(threads);
            let losses: Vec<f64> = std::thread::scope(|scope| {
                let parameters = &parameters;
                let symmetries = &symmetries;
                let handles: Vec<_> = gradients
                    .iter_mut()
                    .zip(views.chunks(part))
                    .map(|(gradient, chunk)| {
                        scope.spawn(move || {
                            gradient.fill(0.0);
                            let mut list = [0; MAX_ACTIVE];
                            let mut loss = 0.0;
                            for &(index, view) in chunk {
                                let example = &fitted[index as usize];
                                let map = &symmetries[usize::from(view)];
                                let count = symmetric_inputs(&example.board, map, &mut list);
                                loss += accumulate(
                                    parameters,
                                    &list[..count],
                                    settings.target.of(example),
                                    gradient,
                                );
                            }
                            loss
                        })
                    })
                    .collect();
                handles
                    .into_iter()
                    .map(|handle| handle.join().expect("a training thread panicked"))
                    .collect()
            });
            epoch_loss += losses.iter().sum::<f64>();
            let used = views.len().div_ceil(part);
            let (total, rest) = gradients.split_at_mut(1);
            for other in &rest[..used - 1] {
                for (sum, value) in total[0].iter_mut().zip(other) {
                    *sum += value;
                }
            }

            step += 1;
            let progress = step as f32 / total_steps as f32;
            let rate = settings.rate
                * (0.05 + 0.95 * 0.5 * (1.0 + (std::f32::consts::PI * progress).cos()));
            let scale = 1.0 / views.len() as f32;
            let (correction1, correction2) =
                (1.0 - beta1.powi(step as i32), 1.0 - beta2.powi(step as i32));
            for ((value, gradient), (m, v)) in parameters
                .iter_mut()
                .zip(&total[0])
                .zip(first.iter_mut().zip(second.iter_mut()))
            {
                let g = gradient * scale + settings.penalty * *value;
                *m = beta1 * *m + (1.0 - beta1) * g;
                *v = beta2 * *v + (1.0 - beta2) * g * g;
                *value -= rate * (*m / correction1) / ((*v / correction2).sqrt() + epsilon);
            }
        }
        report(&Epoch {
            number,
            fitted: epoch_loss / fitted.len().max(1) as f64,
            held_out: measure(&parameters, held_out, threads),
        });
    }
    parameters
}

/// Rounds the parameters as the bot stores them: per group, a scale (the
/// largest magnitude over 127) and one signed byte per parameter. Returns
/// the rounded parameters, as the bot will use them, and the base64 text
/// that `ValueNetwork::decode` reads.
pub fn quantize(parameters: &[f32]) -> (Vec<f32>, String) {
    let mut bytes = Vec::with_capacity(4 * ValueNetwork::GROUPS.len() + parameters.len());
    let mut rounded = Vec::with_capacity(parameters.len());
    let mut at = 0;
    for count in ValueNetwork::GROUPS {
        let group = &parameters[at..at + count];
        at += count;
        let largest = group.iter().fold(0.0f32, |max, value| max.max(value.abs()));
        let scale = if largest > 0.0 { largest / 127.0 } else { 1.0 };
        bytes.extend_from_slice(&scale.to_le_bytes());
        for value in group {
            let byte = (value / scale).round().clamp(-127.0, 127.0) as i8;
            bytes.push(byte as u8);
            rounded.push(f32::from(byte) * scale);
        }
    }
    (rounded, base64(&bytes))
}

/// Standard base64, with padding.
pub fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut text = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let mut word = [0u8; 3];
        word[..chunk.len()].copy_from_slice(chunk);
        let bits = u32::from(word[0]) << 16 | u32::from(word[1]) << 8 | u32::from(word[2]);
        for index in 0..4 {
            if index <= chunk.len() {
                text.push(char::from(
                    ALPHABET[(bits >> (18 - 6 * index)) as usize & 63],
                ));
            } else {
                text.push('=');
            }
        }
    }
    text
}

/// At most `count` of `examples`, spread evenly: the positions of the gate.
pub fn spread(examples: &[Example], count: usize) -> Vec<Example> {
    let step = examples.len().div_ceil(count.max(1)).max(1);
    examples.iter().step_by(step).copied().collect()
}

/// The squared error against the result of the average of the first `k`
/// playouts from each position, for each `k` of `counts`, with `policy`'s
/// playouts. Spread over `threads` threads.
pub fn playout_errors(
    examples: &[Example],
    policy: &PlayoutPolicy,
    counts: &[u32],
    seed: u64,
    threads: usize,
) -> Vec<f64> {
    let most = counts.iter().copied().max().unwrap_or(0);
    let chunk = examples.len().div_ceil(threads.max(1)).max(1);
    let sums: Vec<Vec<f64>> = std::thread::scope(|scope| {
        let handles: Vec<_> = examples
            .chunks(chunk)
            .enumerate()
            .map(|(part, examples)| {
                scope.spawn(move || {
                    let mut rng = Rng::new(seed.wrapping_add(part as u64));
                    let mut sums = vec![0.0; counts.len()];
                    for example in examples {
                        let mut total = 0.0;
                        for played in 1..=most {
                            let mut board = example.board;
                            total += match board.policy_playout(policy, &mut rng) {
                                Status::Win(seat) if seat == example.board.to_move() => 1.0,
                                Status::Win(_) => 0.0,
                                _ => 0.5,
                            };
                            for (sum, &count) in sums.iter_mut().zip(counts) {
                                if count == played {
                                    let average = total / f64::from(played);
                                    *sum += (average - f64::from(example.result)).powi(2);
                                }
                            }
                        }
                    }
                    sums
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().expect("a playout thread panicked"))
            .collect()
    });
    let count = examples.len().max(1) as f64;
    (0..counts.len())
        .map(|index| sums.iter().map(|part| part[index]).sum::<f64>() / count)
        .collect()
}

/// How many averaged playouts a network with squared error `network` is
/// worth, given the errors of averages of `counts` playouts.
pub fn worth_in_playouts(network: f64, counts: &[u32], errors: &[f64]) -> String {
    match errors.first() {
        Some(&one) if network >= one => return "less than one playout".to_string(),
        None => return "unknown".to_string(),
        _ => {}
    }
    for window in counts.iter().zip(errors).collect::<Vec<_>>().windows(2) {
        let ((&low, &low_error), (&high, &high_error)) = (window[0], window[1]);
        if network >= high_error && network < low_error {
            return format!("between {low} and {high} playouts");
        }
    }
    format!("more than {} playouts", counts.last().copied().unwrap_or(0))
}

/// The weights as Rust source for the bot: base64 text in lines of 96
/// characters, joined by the string's line continuations.
pub fn weights_source(text: &str, origin: &str) -> String {
    let mut source = String::new();
    let _ = writeln!(
        source,
        "//! Generated by `uttt-trainer fit-value`: {origin}."
    );
    let _ = writeln!(source);
    let _ = writeln!(
        source,
        "/// The value network's weights (ADR 0017), for\n/// `uttt_engine::value::ValueNetwork::decode`."
    );
    let _ = writeln!(source, "pub const VALUE_WEIGHTS: &str = \"\\");
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
    if lines.is_empty() {
        let _ = writeln!(source, "\";");
    }
    source
}

#[cfg(test)]
mod tests;
