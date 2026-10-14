//! An efficiently updatable network (NNUE) for the side to move's expected
//! score, in quantized integer arithmetic.
//!
//! The network is `(189 -> N) x 2 -> 10`: one first layer, shared by both
//! seats, is applied to the position from each seat's perspective, and the
//! two accumulators feed one of 10 output buckets. A move only ever turns
//! inputs on, because the inputs keep the cells of closed small boards, so
//! [`Network::play`] adds a few rows of weights instead of building the
//! accumulators again.
//!
//! # Inputs
//!
//! From the perspective of a seat, 189 inputs, each 0 or 1: [`OUR_CELLS`]
//! (the seat's marks, at `9 * board + cell`), [`THEIR_CELLS`], [`OUR_BOARDS`]
//! (small boards the seat won, at `162 + board`), [`THEIR_BOARDS`] and
//! [`DRAWN_BOARDS`] (full without a winner). The bucket is the small board
//! the side to move is sent to, or 9 for a free choice ([`bucket`]).
//!
//! # Arithmetic (exact; the trainer relies on it)
//!
//! `acc[seat][j]` is `feature_bias[j]` plus the sum of
//! `feature_weights[f * N + j]` over the active inputs `f` of `seat`, in
//! `i16`. It cannot overflow: weights are in `-127..=127`, at most
//! [`MAX_ACTIVE`] inputs are on, and a bias is at most [`MAX_BIAS`] in
//! magnitude. With `stm` the side to move, `k` its bucket and
//! `c(x) = clamp(x, 0, QA)`:
//!
//! ```text
//! sum   = sum_j c(acc[stm][j])^2 * w[k][j] + sum_j c(acc[1 - stm][j])^2 * w[k][N + j]
//! logit = (sum / QA + output_bias[k]) as f32 / (QA * QB) as f32
//! ```
//!
//! The division truncates toward zero, as Rust's `/` does. For `N` up to
//! [`MAX_WIDTH`] the sum stays within `i32`.
//!
//! # Meaning in floating point, for the trainer
//!
//! First-layer weights and biases are `q / QA`, output weights `v = q / QB`
//! and the output bias `c = q / (QA * QB)`. The network computes
//! `a_j = clamp(acc_j, 0, 1)` from the first layer's output (the squared
//! clipped ReLU), `logit = sum_j a_j^2 * v_j + c` over the side to move's
//! accumulator, then the other seat's, with the weights of the bucket, and
//! `score = sigmoid(logit)`.
//!
//! # Byte layout
//!
//! All little-endian: [`MAGIC`] (8 bytes); `N` as `u32`; the feature weights
//! (`FEATURES * N` `i8`, feature-major); the feature biases (`N` `i16`);
//! the output weights (`BUCKETS * 2N` `i8`, bucket-major, the side to
//! move's `N` then the other seat's `N`); the output biases (`BUCKETS`
//! `i32`). Weights are in `-127..=127`: `-128` is rejected.
//!
//! # Limits
//!
//! Only ongoing positions are evaluated: [`Network::logit`] reads the side
//! to move and the bucket, which mean nothing once the game is over.
//!
//! The weights are kept widened to `i16`, so that the compiler adds rows
//! lane by lane with SSE2, which every CodinGame machine has.

use crate::board::Board;
use crate::grid::{self, FULL};
use crate::moves::Move;

/// Inputs of one perspective, all 0 or 1.
pub const FEATURES: usize = 189;
/// `0..81`: cells marked by the perspective's seat, at `9 * board + cell`, closed boards included.
pub const OUR_CELLS: usize = 0;
/// `81..162`: cells marked by the other seat.
pub const THEIR_CELLS: usize = 81;
/// `162..171`: small boards won by the perspective's seat, at `162 + board`.
pub const OUR_BOARDS: usize = 162;
/// `171..180`: small boards won by the other seat.
pub const THEIR_BOARDS: usize = 171;
/// `180..189`: small boards full without a winner (drawn).
pub const DRAWN_BOARDS: usize = 180;
/// Features of one perspective that can be 1 at once: 81 cells and 9 boards.
pub const MAX_ACTIVE: usize = 90;
/// Output buckets: the small board the side to move is sent to (0..9), or 9 for a free choice.
pub const BUCKETS: usize = 10;
/// Scale of the first layer's weights and biases, and the accumulators' clipping bound.
pub const QA: i32 = 127;
/// Scale of the output weights.
pub const QB: i32 = 64;
/// The first bytes of a network's bytes.
pub const MAGIC: &[u8; 8] = b"UTTTNN01";
/// The largest accumulator width: sums of the output layer stay within i32 up to it.
pub const MAX_WIDTH: usize = 512;
/// The largest magnitude of a quantized first-layer bias, so that a bias plus 90 weights of at most 127 stays within i16: 32767 - 90 * 127.
pub const MAX_BIAS: i16 = 21_337;

/// The output bucket of `board`: `board.target().unwrap_or(9)`.
pub fn bucket(board: &Board) -> usize {
    board.target().unwrap_or(9)
}

/// Writes the indices of the features that are 1 in `board` from `seat`'s perspective into `list`, in increasing order, and returns how many.
pub fn active_features(board: &Board, seat: usize, list: &mut [u16; MAX_ACTIVE]) -> usize {
    let mut len = 0;
    for (seat, offset) in [(seat, OUR_CELLS), (1 - seat, THEIR_CELLS)] {
        for small in 0..9 {
            for cell in grid::cells(board.cells(seat, small)) {
                list[len] = (offset + 9 * small + cell) as u16;
                len += 1;
            }
        }
    }
    let drawn = board.closed_boards() & !board.won_boards(0) & !board.won_boards(1);
    for (mask, offset) in [
        (board.won_boards(seat), OUR_BOARDS),
        (board.won_boards(1 - seat), THEIR_BOARDS),
        (drawn, DRAWN_BOARDS),
    ] {
        for small in grid::cells(mask) {
            list[len] = (offset + small) as u16;
            len += 1;
        }
    }
    len
}

/// The features that become 1 from `seat`'s perspective when `mv` is played from `before`: the cell, then the small board's feature if the move wins it or fills it without a winner. Writes them into `list` and returns how many (1 or 2). Nothing ever becomes 0.
pub fn move_features(before: &Board, mv: Move, seat: usize, list: &mut [u16; 2]) -> usize {
    let mover = before.to_move();
    let board = mv.board();
    let ours = mover == seat;
    let offset = if ours { OUR_CELLS } else { THEIR_CELLS };
    list[0] = (offset + 9 * board + mv.cell()) as u16;

    let marks = before.cells(mover, board) | (1 << mv.cell());
    if grid::has_line(marks) {
        let offset = if ours { OUR_BOARDS } else { THEIR_BOARDS };
        list[1] = (offset + board) as u16;
        2
    } else if marks | before.cells(1 - mover, board) == FULL {
        list[1] = (DRAWN_BOARDS + board) as u16;
        2
    } else {
        1
    }
}

/// The accumulators of a position: `values[seat]` is the first layer's output (before clipping) from `seat`'s perspective, at scale QA.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Accumulator<const N: usize> {
    pub values: [[i16; N]; 2],
}

/// A quantized network of accumulator width `N`: (189 -> N) x 2 -> 10 buckets.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Network<const N: usize> {
    /// `FEATURES * N`, feature-major, widened to `i16`.
    feature_weights: Box<[i16]>,
    /// `N`.
    feature_bias: Box<[i16]>,
    /// `BUCKETS * 2 * N`, bucket-major, widened to `i16`.
    output_weights: Box<[i16]>,
    output_bias: [i32; BUCKETS],
}

/// `acc += row`, lane by lane.
#[inline(always)]
fn add_row<const N: usize>(acc: &mut [i16; N], row: &[i16]) {
    for (a, &w) in acc.iter_mut().zip(row) {
        *a += w;
    }
}

/// The sum of `clamp(acc, 0, QA)^2 * weight` over a row, in `i32`.
#[inline(always)]
fn squared_dot(acc: &[i16], weights: &[i16]) -> i32 {
    acc.iter()
        .zip(weights)
        .map(|(&a, &w)| {
            let c = i32::from(a.clamp(0, QA as i16));
            c * c * i32::from(w)
        })
        .sum()
}

impl<const N: usize> Network<N> {
    /// Bytes of a network of width N: 52 + 211 * N.
    pub const BYTES: usize = 8 + 4 + FEATURES * N + 2 * N + BUCKETS * 2 * N + 4 * BUCKETS;

    /// From its parts: `feature_weights` FEATURES * N (feature-major: feature f's N weights at f*N..), `feature_bias` N, `output_weights` BUCKETS * 2N (bucket-major; within a bucket, N weights for the side to move's accumulator then N for the other seat's), `output_bias` one per bucket. Fails on wrong lengths, N == 0 or N > MAX_WIDTH, a bias of magnitude above MAX_BIAS, or a weight of -128.
    pub fn from_parts(
        feature_weights: &[i8],
        feature_bias: &[i16],
        output_weights: &[i8],
        output_bias: &[i32; BUCKETS],
    ) -> Result<Self, String> {
        if N == 0 || N > MAX_WIDTH {
            return Err(format!("width {N} is not in 1..={MAX_WIDTH}"));
        }
        if feature_weights.len() != FEATURES * N {
            return Err(format!(
                "{} feature weights, expected {}",
                feature_weights.len(),
                FEATURES * N
            ));
        }
        if feature_bias.len() != N {
            return Err(format!(
                "{} feature biases, expected {N}",
                feature_bias.len()
            ));
        }
        if output_weights.len() != BUCKETS * 2 * N {
            return Err(format!(
                "{} output weights, expected {}",
                output_weights.len(),
                BUCKETS * 2 * N
            ));
        }
        if feature_weights.contains(&i8::MIN) || output_weights.contains(&i8::MIN) {
            return Err("a weight of -128 (weights are in -127..=127)".to_string());
        }
        if feature_bias
            .iter()
            .any(|bias| bias.unsigned_abs() > MAX_BIAS.unsigned_abs())
        {
            return Err(format!("a feature bias above {MAX_BIAS} in magnitude"));
        }
        Ok(Network {
            feature_weights: feature_weights.iter().map(|&w| i16::from(w)).collect(),
            feature_bias: feature_bias.into(),
            output_weights: output_weights.iter().map(|&w| i16::from(w)).collect(),
            output_bias: *output_bias,
        })
    }

    /// Reads the byte layout of the module documentation; fails (never panics) on a wrong magic, a width other than N, a wrong length, or what `from_parts` rejects.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() != Self::BYTES {
            return Err(format!("{} bytes, expected {}", bytes.len(), Self::BYTES));
        }
        let (magic, rest) = bytes.split_at(MAGIC.len());
        if magic != MAGIC {
            return Err("wrong magic".to_string());
        }
        let (width, rest) = rest.split_at(4);
        let width = u32::from_le_bytes([width[0], width[1], width[2], width[3]]);
        if usize::try_from(width) != Ok(N) {
            return Err(format!("width {width}, expected {N}"));
        }
        let (feature_weights, rest) = rest.split_at(FEATURES * N);
        let (feature_bias, rest) = rest.split_at(2 * N);
        let (output_weights, rest) = rest.split_at(BUCKETS * 2 * N);
        let feature_weights: Vec<i8> = feature_weights.iter().map(|&b| b as i8).collect();
        let feature_bias: Vec<i16> = feature_bias
            .chunks_exact(2)
            .map(|b| i16::from_le_bytes([b[0], b[1]]))
            .collect();
        let output_weights: Vec<i8> = output_weights.iter().map(|&b| b as i8).collect();
        let mut output_bias = [0i32; BUCKETS];
        for (bias, b) in output_bias.iter_mut().zip(rest.chunks_exact(4)) {
            *bias = i32::from_le_bytes([b[0], b[1], b[2], b[3]]);
        }
        Self::from_parts(
            &feature_weights,
            &feature_bias,
            &output_weights,
            &output_bias,
        )
    }

    /// The byte layout of the module documentation; `from_bytes(to_bytes())` gives the same network.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::BYTES);
        bytes.extend_from_slice(MAGIC);
        bytes.extend_from_slice(&(N as u32).to_le_bytes());
        bytes.extend(self.feature_weights.iter().map(|&w| w as i8 as u8));
        for bias in self.feature_bias.iter() {
            bytes.extend_from_slice(&bias.to_le_bytes());
        }
        bytes.extend(self.output_weights.iter().map(|&w| w as i8 as u8));
        for bias in &self.output_bias {
            bytes.extend_from_slice(&bias.to_le_bytes());
        }
        bytes
    }

    #[inline(always)]
    fn row(&self, feature: u16) -> &[i16] {
        let start = usize::from(feature) * N;
        &self.feature_weights[start..start + N]
    }

    /// The accumulators of `board`, from scratch.
    pub fn refresh(&self, board: &Board) -> Accumulator<N> {
        let mut values = [[0i16; N]; 2];
        let mut list = [0u16; MAX_ACTIVE];
        for (seat, accumulator) in values.iter_mut().enumerate() {
            accumulator.copy_from_slice(&self.feature_bias);
            let len = active_features(board, seat, &mut list);
            for &feature in &list[..len] {
                add_row(accumulator, self.row(feature));
            }
        }
        Accumulator { values }
    }

    /// Plays `mv` on `board` and updates `accumulator` incrementally, adding the rows of `move_features` for both seats. `mv` must be legal.
    pub fn play(&self, accumulator: &mut Accumulator<N>, board: &mut Board, mv: Move) {
        let mut list = [0u16; 2];
        for (seat, values) in accumulator.values.iter_mut().enumerate() {
            let len = move_features(board, mv, seat, &mut list);
            for &feature in &list[..len] {
                add_row(values, self.row(feature));
            }
        }
        board.play(mv);
    }

    /// The output before the sigmoid, for the side to move of `board` (an ongoing position), from `accumulator` (which must be `board`'s).
    pub fn logit(&self, accumulator: &Accumulator<N>, board: &Board) -> f32 {
        let stm = board.to_move();
        let start = bucket(board) * 2 * N;
        let weights = &self.output_weights[start..start + 2 * N];
        let (ours, theirs) = weights.split_at(N);
        let sum = squared_dot(&accumulator.values[stm], ours)
            + squared_dot(&accumulator.values[1 - stm], theirs);
        let total = i64::from(sum / QA) + i64::from(self.output_bias[bucket(board)]);
        total as f32 / (QA * QB) as f32
    }

    /// The side to move's expected score, 0 to 1: the sigmoid of `logit`.
    pub fn evaluate(&self, accumulator: &Accumulator<N>, board: &Board) -> f32 {
        1.0 / (1.0 + (-self.logit(accumulator, board)).exp())
    }

    /// `evaluate(refresh(board), board)`.
    pub fn evaluate_board(&self, board: &Board) -> f32 {
        self.evaluate(&self.refresh(board), board)
    }
}

#[cfg(test)]
mod tests;
