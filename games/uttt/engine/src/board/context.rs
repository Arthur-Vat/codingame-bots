//! A context policy (E016): E015's pattern policy, plus a weight for the
//! small board the opponent is sent to. A move's log-weight is the sum of
//! two learned weights:
//!
//! 1. E015's: the canonical pattern and cell of the move's small board,
//!    with one of 5 kinds of destination ([`Board::pattern_feature`]);
//! 2. the destination's: the canonical pattern of the board the opponent
//!    is sent to, seen by the opponent after the move, with that board's
//!    role in the big board for the opponent (2 if winning it would win
//!    the game for the opponent, plus 1 if for the player); or a single
//!    weight for a free choice or a move that ends the game.
//!
//! The second weight depends on the destination board only, as long as
//! the move neither wins its own board nor sends the opponent back to it,
//! so playouts keep it by board in a cache and look it up.
//!
//! The weights travel packed ([`cg_core::packed`]): the first table in 5
//! interleaved segments (by kind of destination), then the second in
//! [`ROLES`]. Each weight is a log-weight in quarter steps, `(q - 32) / 4`,
//! so a move's weight is one lookup of the sum of its two `q`s.

use std::sync::OnceLock;

use cg_core::packed;
use cg_core::rng::Rng;

use super::patterns::{pattern_ids, B3};
use super::{Board, Status, ANY_BOARD, DESTINATIONS, PATTERN_FEATURES};
use crate::grid::{self, FULL};
use crate::moves::Move;

/// Roles of a small board in the big board.
pub const ROLES: usize = 4;
/// Canonical open small-board patterns: a free cell, no line.
pub const OPEN_PATTERNS: usize = 1_582;
/// Size of the destinations' table.
pub const DESTINATION_WEIGHTS: usize = 1 + OPEN_PATTERNS * ROLES;

/// `WEIGHTS[q]`: the weight of a move whose two `q`s sum to `q`.
fn weights() -> &'static [f32; 127] {
    static WEIGHTS: OnceLock<[f32; 127]> = OnceLock::new();
    WEIGHTS.get_or_init(|| std::array::from_fn(|q| ((q as f32 - 64.0) / 4.0).exp()))
}

/// `ids[pattern]`: the number of the canonical form of an open pattern
/// under the 8 symmetries, in increasing order of the canonical patterns;
/// 0 for other patterns, which are never looked up.
fn open_pattern_ids() -> &'static [u16] {
    static IDS: OnceLock<Vec<u16>> = OnceLock::new();
    IDS.get_or_init(|| {
        let mut ids = vec![0u16; super::PATTERNS];
        let mut count = 0;
        for pattern in 0..super::PATTERNS {
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
            if mask(0) == 0 || grid::has_line(mask(1)) || grid::has_line(mask(2)) {
                continue;
            }
            let canonical = (0..8)
                .map(|symmetry| {
                    (0..9)
                        .map(|cell| {
                            digits[cell] * 3usize.pow(super::symmetric_cell(symmetry, cell) as u32)
                        })
                        .sum::<usize>()
                })
                .min()
                .expect("eight symmetries");
            ids[pattern] = if canonical == pattern {
                count += 1;
                count - 1
            } else {
                ids[canonical]
            };
        }
        assert_eq!(usize::from(count), OPEN_PATTERNS, "open patterns");
        ids
    })
}

/// Learned weights of the context policy, as quarter-step numbers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextPolicy {
    /// E015's weights.
    patterns: Vec<u8>,
    /// The destinations' weights.
    destinations: Vec<u8>,
    /// Moves of each playout drawn by the policy; decisive random moves
    /// after that.
    plies: u32,
}

impl ContextPolicy {
    /// The policy with these tables of numbers from 0 to 63: E015's
    /// weights and the destinations'.
    pub fn new(patterns: Vec<u8>, destinations: Vec<u8>) -> Result<Self, String> {
        if patterns.len() != PATTERN_FEATURES {
            return Err(format!("{} pattern weights", patterns.len()));
        }
        if destinations.len() != DESTINATION_WEIGHTS {
            return Err(format!("{} destination weights", destinations.len()));
        }
        if patterns.iter().chain(&destinations).any(|&q| q >= 64) {
            return Err("a weight above 63".to_string());
        }
        // Built now rather than during the first search.
        open_pattern_ids();
        weights();
        Ok(ContextPolicy {
            patterns,
            destinations,
            plies: u32::MAX,
        })
    }

    /// Decodes the trainer's packed text.
    pub fn decode(text: &str) -> Result<Self, String> {
        let (patterns, rest) = packed::decode_strided(text, DESTINATIONS)?;
        let (destinations, rest) = packed::decode_strided(rest, ROLES)?;
        if !rest.is_empty() {
            return Err(format!("{} characters after the tables", rest.len()));
        }
        Self::new(patterns, destinations)
    }

    /// The same weights, used for the first `plies` moves of each playout
    /// only.
    pub fn for_plies(mut self, plies: u32) -> Self {
        self.plies = plies;
        self
    }
}

/// The destinations' weight (its `q`) for sending each side to each small
/// board, as the position stands: kept up to date along a playout.
#[derive(Clone, Copy, Debug)]
struct Destinations {
    /// `q[seat][board]`: when `seat` is sent to `board`.
    q: [[u8; 9]; 2],
}

impl Destinations {
    fn new(board: &Board, policy: &ContextPolicy) -> Self {
        let mut cache = Destinations { q: [[0; 9]; 2] };
        cache.refresh(board, policy);
        cache
    }

    /// Every board's entries: a closed board's is the weight of a free
    /// choice.
    fn refresh(&mut self, board: &Board, policy: &ContextPolicy) {
        for small in 0..9 {
            if board.closed & (1 << small) == 0 {
                self.set(board, policy, small);
            } else {
                self.q[0][small] = policy.destinations[0];
                self.q[1][small] = policy.destinations[0];
            }
        }
    }

    /// The entries of open board `small`.
    #[inline]
    fn set(&mut self, board: &Board, policy: &ContextPolicy, small: usize) {
        for seat in 0..2 {
            let index = board.destination_index(
                seat,
                small,
                board.marks[1 - seat][small],
                board.won[1 - seat],
                board.closed,
            );
            self.q[seat][small] = policy.destinations[index];
        }
    }

    /// After a move in small board `small`: its entries change, and every
    /// board's role may change when it closes.
    #[inline]
    fn update(&mut self, board: &Board, policy: &ContextPolicy, small: usize) {
        if board.closed & (1 << small) != 0 {
            self.refresh(board, policy);
        } else {
            self.set(board, policy, small);
        }
    }
}

impl Board {
    /// The index in the destinations' table for sending `seat` to open
    /// board `small`, where the other side holds `other` marks, has won
    /// `other_won` and the boards `closed` are closed: the position after
    /// a move by the other side.
    #[inline]
    fn destination_index(
        &self,
        seat: usize,
        small: usize,
        other: u16,
        other_won: u16,
        closed: u16,
    ) -> usize {
        let open = !closed & FULL;
        let own = self.marks[seat][small];
        let pattern = usize::from(B3[usize::from(own)]) + 2 * usize::from(B3[usize::from(other)]);
        let decides = |won: u16| usize::from((grid::completing_cells(won) & open) >> small & 1);
        let role = 2 * decides(self.won[seat]) + decides(other_won);
        1 + usize::from(open_pattern_ids()[pattern]) * ROLES + role
    }

    /// The indices of each empty cell of small board `board` in the two
    /// tables, for the player to move; zeros for other cells. With
    /// `cache`, the destinations' weights that do not change with the
    /// move come from it, as `usize::MAX - q`.
    #[inline]
    fn context_indices(&self, board: usize, cache: Option<&Destinations>) -> [[usize; 2]; 9] {
        let (me, opp) = (self.to_move(), 1 - self.to_move());
        let mine = self.marks[me][board];
        let empty = self.empty_cells(board);
        let here = 1u16 << board;
        let wins = grid::completing_cells(mine) & empty;
        let closes = wins | if grid::count(empty) == 1 { empty } else { 0 };
        // Moves that end the game, and free choices: one weight.
        let ends = if grid::has_line(self.won[me] | here) {
            wins
        } else {
            0
        } | if self.closed | here == FULL {
            closes
        } else {
            0
        };
        let free = (self.closed & !here) | (closes & here);
        let row = &pattern_ids()[9 * self.pattern(board)..][..9];
        let kinds = self.destinations(board);
        let mut indices = [[0usize; 2]; 9];
        for cell in grid::cells(empty) {
            let first = usize::from(row[cell]) * DESTINATIONS + kinds[cell];
            let other = self.marks[me][cell];
            let second = if (ends | free) & (1 << cell) != 0 {
                0
            } else if cell == board {
                // The opponent is sent back to this board, changed.
                self.destination_index(opp, board, mine | 1 << cell, self.won[me], self.closed)
            } else if wins & (1 << cell) != 0 {
                // Winning this board may make others decide the game.
                self.destination_index(opp, cell, other, self.won[me] | here, self.closed | here)
            } else {
                match cache {
                    Some(cache) => usize::MAX - usize::from(cache.q[opp][cell]),
                    None => self.destination_index(opp, cell, other, self.won[me], self.closed),
                }
            };
            indices[cell] = [first, second];
        }
        indices
    }

    /// The table indices of legal move `mv` in a context policy: for the
    /// trainer's checks.
    pub fn context_features(&self, mv: Move) -> [usize; 2] {
        debug_assert!(self.is_legal(mv), "illegal move {mv}");
        self.context_indices(mv.board(), None)[mv.cell()]
    }

    /// The weight `policy` gives each empty cell of small board `board`,
    /// 0 for the others.
    #[inline]
    fn context_weights(
        &self,
        policy: &ContextPolicy,
        board: usize,
        cache: Option<&Destinations>,
    ) -> [f32; 9] {
        let weights = weights();
        let indices = self.context_indices(board, cache);
        let mut result = [0f32; 9];
        for cell in grid::cells(self.empty_cells(board)) {
            let [first, second] = indices[cell];
            let destination = match policy.destinations.get(second) {
                Some(&q) => q,
                None => (usize::MAX - second) as u8,
            };
            result[cell] = weights[usize::from(policy.patterns[first]) + usize::from(destination)];
        }
        result
    }

    /// The weight `policy` gives each of `moves`, which must be legal, in
    /// order, written into `weights`: the search's priors.
    pub fn context_move_weights(
        &self,
        policy: &ContextPolicy,
        moves: &[Move],
        weights: &mut Vec<f32>,
    ) {
        let mut boards: [Option<[f32; 9]>; 9] = [None; 9];
        weights.clear();
        for &mv in moves {
            let board = mv.board();
            let by_cell =
                boards[board].get_or_insert_with(|| self.context_weights(policy, board, None));
            weights.push(by_cell[mv.cell()]);
        }
    }

    /// Running totals of the weights of small board `board`'s cells, cell
    /// 0 first, with 0 for occupied cells, along a playout: E015's
    /// branch-free loop with the destinations' weights from `cache`, then
    /// the few cells whose destination the move itself changes.
    #[inline]
    fn context_cumulative(
        &self,
        policy: &ContextPolicy,
        board: usize,
        cache: &Destinations,
    ) -> [f32; 9] {
        let weights = weights();
        let (me, opp) = (self.to_move(), 1 - self.to_move());
        let empty = self.empty_cells(board);
        let here = 1u16 << board;
        let wins = grid::completing_cells(self.marks[me][board]) & empty;
        let row = &pattern_ids()[9 * self.pattern(board)..][..9];
        let kinds = self.destinations(board);
        let patterns = &policy.patterns;
        let mut by_cell = [0f32; 9];
        for (cell, ((weight, &pair), &kind)) in by_cell.iter_mut().zip(row).zip(&kinds).enumerate()
        {
            let q = usize::from(patterns[usize::from(pair) * DESTINATIONS + kind])
                + usize::from(cache.q[opp][cell]);
            *weight = weights[q] * f32::from(u8::from((empty >> cell) & 1 == 1));
        }
        // Cells sent back to this board or winning it, and so moves that
        // end the game: their destination is not the cached one.
        for cell in grid::cells((wins | here) & empty) {
            let won = wins & (1 << cell) != 0;
            let left = empty & !(1 << cell);
            let closes = won || left == 0;
            let ends = (won && grid::has_line(self.won[me] | here))
                || (closes && self.closed | here == FULL);
            let second = if ends || (closes && cell == board) {
                0
            } else if cell == board {
                let mine = self.marks[me][board] | 1 << cell;
                self.destination_index(opp, board, mine, self.won[me], self.closed)
            } else if self.closed & (1 << cell) != 0 {
                0
            } else {
                let other = self.marks[me][cell];
                self.destination_index(opp, cell, other, self.won[me] | here, self.closed | here)
            };
            let first = usize::from(row[cell]) * DESTINATIONS + kinds[cell];
            by_cell[cell] =
                weights[usize::from(patterns[first]) + usize::from(policy.destinations[second])];
        }
        let mut total = 0.0;
        by_cell.map(|weight| {
            total += weight;
            total
        })
    }

    /// The first cell whose running total exceeds `pick`.
    #[inline]
    fn cell_at_f32(cumulative: &[f32; 9], pick: f32) -> usize {
        cumulative
            .iter()
            .map(|&sum| usize::from(sum <= pick))
            .sum::<usize>()
            .min(8)
    }

    /// A cell drawn along a playout, after a game-winning move if there is
    /// one.
    #[inline]
    fn context_playout_cell(
        &self,
        policy: &ContextPolicy,
        cache: &Destinations,
        rng: &mut Rng,
    ) -> (usize, usize) {
        if let Some(winning) = self.game_winning_cell() {
            return winning;
        }
        let unit = rng.unit() as f32;
        if self.target != ANY_BOARD {
            let board = usize::from(self.target);
            let cumulative = self.context_cumulative(policy, board, cache);
            let cell = Board::cell_at_f32(&cumulative, unit * cumulative[8]);
            // Rounding cannot pick an occupied cell: fall back to the last
            // empty one.
            return (board, Board::nearest_empty(self.empty_cells(board), cell));
        }
        let mut boards = [[0f32; 9]; 9];
        let mut totals = [0f32; 9];
        let mut total = 0.0;
        for board in grid::cells(!self.closed & FULL) {
            boards[board] = self.context_cumulative(policy, board, cache);
            total += boards[board][8];
            totals[board] = total;
        }
        let pick = unit * total;
        let board = totals
            .iter()
            .zip(0..9)
            .find(|&(&sum, board)| pick < sum && !self.is_closed(board))
            .map_or_else(
                || {
                    grid::cells(!self.closed & FULL)
                        .last()
                        .expect("an open board")
                },
                |(_, board)| board,
            );
        let before = totals[board] - boards[board][8];
        let cell = Board::cell_at_f32(&boards[board], pick - before);
        (board, Board::nearest_empty(self.empty_cells(board), cell))
    }

    /// `cell` if it is in `empty`, else the last cell of `empty` before
    /// it, or the first: a draw at the edge of rounding.
    #[inline]
    fn nearest_empty(empty: u16, cell: usize) -> usize {
        if empty & (1 << cell) != 0 {
            return cell;
        }
        let below = empty & ((1 << cell) - 1);
        if below != 0 {
            15 - below.leading_zeros() as usize
        } else {
            empty.trailing_zeros() as usize
        }
    }

    /// A cell drawn from the context weights, after a game-winning move if
    /// there is one; along a playout when `cache` is given.
    #[inline]
    fn context_cell(
        &self,
        policy: &ContextPolicy,
        cache: Option<&Destinations>,
        rng: &mut Rng,
    ) -> (usize, usize) {
        if let Some(winning) = self.game_winning_cell() {
            return winning;
        }
        let pick = rng.unit() as f32;
        if self.target != ANY_BOARD {
            let board = usize::from(self.target);
            let weights = self.context_weights(policy, board, cache);
            return (
                board,
                Board::weighted_cell(&weights, self.empty_cells(board), pick),
            );
        }
        let mut boards = [[0f32; 9]; 9];
        let mut totals = [0f32; 9];
        let mut total = 0.0;
        for board in grid::cells(!self.closed & FULL) {
            boards[board] = self.context_weights(policy, board, cache);
            totals[board] = boards[board].iter().sum();
            total += totals[board];
        }
        let mut left = pick * total;
        let mut last = 0;
        for board in grid::cells(!self.closed & FULL) {
            last = board;
            if left < totals[board] {
                break;
            }
            left -= totals[board];
        }
        // Rounding may leave `left` past the last board's total.
        let share = if totals[last] > 0.0 {
            left / totals[last]
        } else {
            0.0
        };
        (
            last,
            Board::weighted_cell(&boards[last], self.empty_cells(last), share),
        )
    }

    /// The cell of `empty` where the running total of `weights` passes
    /// `share` of their sum.
    #[inline]
    fn weighted_cell(weights: &[f32; 9], empty: u16, share: f32) -> usize {
        let total: f32 = weights.iter().sum();
        let mut left = share * total;
        let mut last = 0;
        for cell in grid::cells(empty) {
            last = cell;
            if left < weights[cell] {
                return cell;
            }
            left -= weights[cell];
        }
        last
    }

    /// A move drawn from `policy`, after a game-winning move if there is
    /// one. The game must go on.
    pub fn context_move(&self, policy: &ContextPolicy, rng: &mut Rng) -> Move {
        let (board, cell) = self.context_cell(policy, None, rng);
        Move::new(board, cell)
    }

    /// Plays [`context_move`](Board::context_move)s for `policy`'s plies,
    /// then decisive random moves, until the game ends; returns how it
    /// ended.
    pub fn context_playout(&mut self, policy: &ContextPolicy, rng: &mut Rng) -> Status {
        let mut plies = 0;
        let mut cache = Destinations::new(self, policy);
        while self.status == Status::Ongoing && plies < policy.plies {
            let (board, cell) = self.context_playout_cell(policy, &cache, rng);
            self.play_at(board, cell);
            cache.update(self, policy, board);
            plies += 1;
        }
        self.decisive_playout(rng)
    }
}

#[cfg(test)]
mod tests;
