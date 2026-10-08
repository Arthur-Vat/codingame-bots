//! A pattern policy (E015): playout moves drawn, and tree children
//! ordered, with weights learned for every pattern of the small board a
//! move is played in, the move's cell, and where the move sends the
//! opponent; tens of thousands of weights instead of the 32 classes of
//! [`PlayoutPolicy`](super::PlayoutPolicy).
//!
//! A small board's pattern is its 9 cells seen by the player to move:
//! empty, its own, or the opponent's, read as a number in base 3. The 8
//! symmetries of the 3×3 grid give the same weight to a pattern and cell
//! and to their images, so only canonical (pattern, cell) pairs of open
//! boards have weights: 5,255 of them. Each comes with one of
//! [`DESTINATIONS`] kinds of destination: a free choice, or a board where
//! the opponent, the player, both or neither can win a small board at once.
//!
//! The weights travel as text: one character of the base64 alphabet per
//! weight, `q` from 0 to 63 for a log-weight of `(q - 32) / 4`.

use std::sync::OnceLock;

use cg_core::rng::Rng;

use super::{Board, Status, ANY_BOARD};
use crate::grid::{self, FULL};
use crate::moves::Move;

/// Kinds of destination of a move.
pub const DESTINATIONS: usize = 5;
/// Small-board patterns: 3 states for each of 9 cells.
pub const PATTERNS: usize = 19_683;
/// Canonical (pattern, cell) pairs of open small boards.
pub const PATTERN_CELLS: usize = 5_255;
/// Features, and so weights, of a pattern policy.
pub const PATTERN_FEATURES: usize = PATTERN_CELLS * DESTINATIONS;

/// The 6-bit digits of the weights' text.
const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// `B3[mask]`: the base-3 number with a 1 for each cell of `mask`.
const B3: [u16; 512] = {
    let mut table = [0u16; 512];
    let mut mask = 0;
    while mask < 512 {
        let (mut value, mut power, mut cell) = (0u16, 1u16, 0);
        while cell < 9 {
            if mask & (1 << cell) != 0 {
                value += power;
            }
            power *= 3;
            cell += 1;
        }
        table[mask] = value;
        mask += 1;
    }
    table
};

/// The cell that `cell` becomes under symmetry `symmetry` of the 3×3 grid:
/// the identity, three rotations and four reflections.
pub fn symmetric_cell(symmetry: usize, cell: usize) -> usize {
    let (row, col) = (cell / 3, cell % 3);
    let (row, col) = match symmetry {
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

/// `ids[9 * pattern + cell]`: the number of the canonical pair of an open
/// pattern and one of its empty cells, 0 to `PATTERN_CELLS - 1`, or 0 for
/// other pairs, which never get a weight.
fn pattern_ids() -> &'static [u16] {
    static IDS: OnceLock<Vec<u16>> = OnceLock::new();
    IDS.get_or_init(|| {
        let mut ids = vec![0u16; PATTERNS * 9];
        let mut count = 0usize;
        for pattern in 0..PATTERNS {
            let mut digits = [0usize; 9];
            let mut rest = pattern;
            for digit in &mut digits {
                *digit = rest % 3;
                rest /= 3;
            }
            let mask = |state: usize| {
                (0..9)
                    .filter(|&cell| digits[cell] == state)
                    .fold(0u16, |mask, cell| mask | 1 << cell)
            };
            let (empty, mine, theirs) = (mask(0), mask(1), mask(2));
            if empty == 0 || grid::has_line(mine) || grid::has_line(theirs) {
                continue;
            }
            let images: [usize; 8] = std::array::from_fn(|symmetry| {
                (0..9)
                    .map(|cell| digits[cell] * 3usize.pow(symmetric_cell(symmetry, cell) as u32))
                    .sum()
            });
            for cell in grid::cells(empty) {
                let canonical = (0..8)
                    .map(|symmetry| (images[symmetry], symmetric_cell(symmetry, cell)))
                    .min()
                    .expect("eight symmetries");
                // The canonical pair comes first in this order, so it has
                // its number already unless it is this pair.
                ids[9 * pattern + cell] = if canonical == (pattern, cell) {
                    count += 1;
                    (count - 1) as u16
                } else {
                    ids[9 * canonical.0 + canonical.1]
                };
            }
        }
        assert_eq!(count, PATTERN_CELLS, "canonical pattern cells");
        ids
    })
}

/// Learned weights by pattern, cell and destination.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatternPolicy {
    /// Integer weights, each below 2^24 so that 81 of them fit a `u32`.
    weights: Vec<u32>,
    /// Moves of each playout drawn by the policy; decisive random moves
    /// after that.
    plies: u32,
}

impl PatternPolicy {
    /// The policy with these log-weights, one per feature (see
    /// [`Board::pattern_feature`]), each from -8 to 7.75.
    pub fn from_log_weights(log_weights: &[f32]) -> Result<Self, String> {
        if log_weights.len() != PATTERN_FEATURES {
            return Err(format!(
                "{} weights instead of {PATTERN_FEATURES}",
                log_weights.len()
            ));
        }
        let weights = log_weights
            .iter()
            .map(|&log| {
                let scaled = f64::from(log.clamp(-8.0, 7.75) - 7.75).exp() * f64::from(1u32 << 24);
                (scaled.round() as u32).clamp(1, (1 << 24) - 1)
            })
            .collect();
        Ok(PatternPolicy {
            weights,
            plies: u32::MAX,
        })
    }

    /// Decodes the trainer's text: one base64 character per weight.
    pub fn decode(text: &str) -> Result<Self, String> {
        let log_weights = text
            .bytes()
            .map(|byte| {
                ALPHABET
                    .iter()
                    .position(|&digit| digit == byte)
                    .map(|q| (q as f32 - 32.0) / 4.0)
                    .ok_or_else(|| format!("{:?} is not a weight", char::from(byte)))
            })
            .collect::<Result<Vec<f32>, String>>()?;
        Self::from_log_weights(&log_weights)
    }

    /// The same weights, used for the first `plies` moves of each playout
    /// only.
    pub fn for_plies(mut self, plies: u32) -> Self {
        self.plies = plies;
        self
    }
}

/// The text of `log_weights` for [`PatternPolicy::decode`], each rounded to
/// the nearest quarter between -8 and 7.75.
pub fn encode_pattern_weights(log_weights: &[f32]) -> String {
    log_weights
        .iter()
        .map(|&log| {
            let q = ((log * 4.0).round() + 32.0).clamp(0.0, 63.0) as usize;
            char::from(ALPHABET[q])
        })
        .collect()
}

impl Board {
    /// The pattern of small board `board` from the player to move's view.
    #[inline(always)]
    fn pattern(&self, board: usize) -> usize {
        let seat = self.to_move();
        usize::from(B3[usize::from(self.marks[seat][board])])
            + 2 * usize::from(B3[usize::from(self.marks[1 - seat][board])])
    }

    /// The destination kind of each cell of small board `board`, from
    /// [`DESTINATIONS`]: 0 when the move gives a free choice, otherwise 1,
    /// plus 1 if the opponent could win the board it is sent to at once,
    /// plus 2 if the player could.
    #[inline(always)]
    fn destinations(&self, board: usize) -> [usize; 9] {
        let seat = self.to_move();
        let free = self.board_features(board).free;
        let (theirs, mine) = (self.threats[1 - seat], self.threats[seat]);
        std::array::from_fn(|cell| {
            let bit = |mask: u16| usize::from((mask >> cell) & 1);
            (1 + bit(theirs) + 2 * bit(mine)) * (1 - bit(free))
        })
    }

    /// The index of `mv`'s weight in a pattern policy: its canonical
    /// pattern and cell, then its destination.
    pub fn pattern_feature(&self, mv: Move) -> usize {
        debug_assert!(self.is_legal(mv), "illegal move {mv}");
        let (board, cell) = (mv.board(), mv.cell());
        let id = pattern_ids()[9 * self.pattern(board) + cell];
        usize::from(id) * DESTINATIONS + self.destinations(board)[cell]
    }

    /// The weight `policy` gives each of `moves`, which must be legal, in
    /// order, written into `weights`: the search's priors.
    pub fn pattern_move_weights(
        &self,
        policy: &PatternPolicy,
        moves: &[Move],
        weights: &mut Vec<f32>,
    ) {
        weights.clear();
        weights.extend(
            moves
                .iter()
                .map(|&mv| policy.weights[self.pattern_feature(mv)] as f32),
        );
    }

    /// Running totals of the weights of small board `board`'s cells, cell
    /// 0 first, with 0 for occupied cells. Branch-free.
    #[inline(always)]
    fn pattern_cumulative(&self, board: usize, ids: &[u16], policy: &PatternPolicy) -> [u32; 9] {
        let empty = self.empty_cells(board);
        let destinations = self.destinations(board);
        let row = &ids[9 * self.pattern(board)..][..9];
        let mut cumulative = [0u32; 9];
        let mut total = 0;
        for (cell, ((sum, &id), &destination)) in cumulative
            .iter_mut()
            .zip(row)
            .zip(&destinations)
            .enumerate()
        {
            let feature = usize::from(id) * DESTINATIONS + destination;
            total += policy.weights[feature] * u32::from((empty >> cell) & 1);
            *sum = total;
        }
        cumulative
    }

    #[inline(always)]
    fn pattern_cell(&self, policy: &PatternPolicy, ids: &[u16], rng: &mut Rng) -> (usize, usize) {
        if let Some(winning) = self.game_winning_cell() {
            return winning;
        }
        if self.target != ANY_BOARD {
            let board = usize::from(self.target);
            let cumulative = self.pattern_cumulative(board, ids, policy);
            let pick = rng.below(u64::from(cumulative[8])) as u32;
            return (board, Board::cell_at(&cumulative, pick));
        }
        let mut boards = [[0u32; 9]; 9];
        let mut totals = [0u32; 9];
        let mut total = 0;
        for board in grid::cells(!self.closed & FULL) {
            boards[board] = self.pattern_cumulative(board, ids, policy);
            total += boards[board][8];
            totals[board] = total;
        }
        let pick = rng.below(u64::from(total)) as u32;
        for board in grid::cells(!self.closed & FULL) {
            if pick < totals[board] {
                let before = totals[board] - boards[board][8];
                return (board, Board::cell_at(&boards[board], pick - before));
            }
        }
        unreachable!("the draw is below the total weight")
    }

    /// A move drawn from `policy`, after a game-winning move if there is
    /// one. The game must go on.
    pub fn pattern_move(&self, policy: &PatternPolicy, rng: &mut Rng) -> Move {
        let (board, cell) = self.pattern_cell(policy, pattern_ids(), rng);
        Move::new(board, cell)
    }

    /// Plays [`pattern_move`](Board::pattern_move)s for `policy`'s plies,
    /// then decisive random moves, until the game ends; returns how it
    /// ended.
    pub fn pattern_playout(&mut self, policy: &PatternPolicy, rng: &mut Rng) -> Status {
        let ids = pattern_ids();
        let mut plies = 0;
        while self.status == Status::Ongoing && plies < policy.plies {
            let (board, cell) = self.pattern_cell(policy, ids, rng);
            self.play_at(board, cell);
            plies += 1;
        }
        self.decisive_playout(rng)
    }
}

#[cfg(test)]
mod tests;
