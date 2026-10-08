//! A playout policy: moves drawn with weights that depend on a few cheap
//! features of each move, learned offline from self-play (ADR 0016).
//!
//! Each feature is a yes-or-no question about a move of the player to
//! move, and together they give the move a class, 0 to `CLASSES - 1`; the
//! policy holds one weight per class. Every feature is a 9-bit mask over
//! the cells of a small board, so classifying a board's empty cells takes
//! a few mask operations.

use cg_core::rng::Rng;

use super::{Board, Status, ANY_BOARD};
use crate::grid::{self, FULL};
use crate::moves::Move;

/// The move wins its small board.
pub const WINS_BOARD: u8 = 1;
/// The move takes a cell where the opponent would win the small board.
pub const BLOCKS: u8 = 2;
/// The move sends the opponent to a closed small board: a free choice.
pub const GIVES_FREE_CHOICE: u8 = 4;
/// The move sends the opponent to an open small board where it can win
/// that board at once.
pub const GIVES_BOARD: u8 = 8;
/// The move takes the centre cell of its small board.
pub const CENTRE: u8 = 16;

/// Number of features.
pub const FEATURES: usize = 5;
/// Number of move classes: every combination of features.
pub const CLASSES: usize = 1 << FEATURES;

/// Names of the features, lowest bit first, for reports.
pub const FEATURE_NAMES: [&str; FEATURES] = [
    "wins board",
    "blocks",
    "gives free choice",
    "gives board",
    "centre",
];

/// Relative weights of the move classes. A move is drawn with probability
/// proportional to the weight of its class.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayoutPolicy {
    weights: [u32; CLASSES],
}

impl PlayoutPolicy {
    /// A policy with these weights by class. Weights are kept below 2^20,
    /// so that the sum over 81 moves fits in a `u32`.
    pub fn new(weights: [u32; CLASSES]) -> Self {
        assert!(
            weights.iter().all(|&weight| weight < 1 << 20),
            "weights must stay below 2^20"
        );
        PlayoutPolicy { weights }
    }

    /// Every move equally likely.
    pub fn uniform() -> Self {
        PlayoutPolicy::new([1; CLASSES])
    }

    pub fn weights(&self) -> &[u32; CLASSES] {
        &self.weights
    }
}

/// Feature masks of the empty cells of one small board, for the player to
/// move.
struct BoardFeatures {
    empty: u16,
    wins: u16,
    blocks: u16,
    free: u16,
}

impl Board {
    /// The small boards where the opponent of the player to move could win
    /// the small board with its next move, if sent there.
    #[inline(always)]
    fn opponent_threats(&self) -> u16 {
        let opponent = 1 - self.to_move();
        let mut threats = 0;
        for board in grid::cells(!self.closed & FULL) {
            if grid::completing_cells(self.marks[opponent][board]) & self.empty_cells(board) != 0 {
                threats |= 1 << board;
            }
        }
        threats
    }

    #[inline(always)]
    fn board_features(&self, board: usize) -> BoardFeatures {
        let seat = self.to_move();
        let empty = self.empty_cells(board);
        let wins = grid::completing_cells(self.marks[seat][board]) & empty;
        let blocks = grid::completing_cells(self.marks[1 - seat][board]) & empty;
        // A cell sends the opponent to the board of the same index: a free
        // choice if that board is closed, or if it is this board and the
        // move closes it, by winning it or taking its last cell.
        let mut closing = wins;
        if grid::count(empty) == 1 {
            closing |= empty;
        }
        let free = (self.closed | (closing & (1 << board))) & empty;
        BoardFeatures {
            empty,
            wins,
            blocks,
            free,
        }
    }

    /// The class of `cell` given its board's features and the opponent's
    /// threats.
    #[inline(always)]
    fn cell_class(features: &BoardFeatures, threats: u16, cell: usize) -> usize {
        let bit = 1 << cell;
        let has = |mask: u16| usize::from(mask & bit != 0);
        let free = has(features.free);
        // A free choice is its own feature; "gives board" is for a single
        // target board.
        has(features.wins)
            | has(features.blocks) << 1
            | free << 2
            | (has(threats) & (1 - free)) << 3
            | usize::from(cell == 4) << 4
    }

    /// The class of a legal move `mv` of the player to move: its features
    /// as bits ([`WINS_BOARD`], [`BLOCKS`], ...). The features judge the
    /// position before the move, except that a move closing its own small
    /// board is seen to give a free choice when it sends the opponent there.
    pub fn move_class(&self, mv: Move) -> usize {
        debug_assert!(self.is_legal(mv), "illegal move {mv}");
        let features = self.board_features(mv.board());
        Board::cell_class(&features, self.opponent_threats(), mv.cell())
    }

    /// A move for playouts: a move that wins the game if there is one (as
    /// [`decisive_move`](Board::decisive_move)), else a move drawn with
    /// probability proportional to its class's weight in `policy`; if all
    /// the weights of the legal moves are 0, a uniformly random move. The
    /// game must go on.
    pub fn policy_move(&self, policy: &PlayoutPolicy, rng: &mut Rng) -> Move {
        let (board, cell) = self.policy_cell(policy, rng);
        Move::new_unchecked(board, cell)
    }

    #[inline(always)]
    fn policy_cell(&self, policy: &PlayoutPolicy, rng: &mut Rng) -> (usize, usize) {
        if let Some(winning) = self.game_winning_cell() {
            return winning;
        }
        let threats = self.opponent_threats();
        let boards = if self.target == ANY_BOARD {
            !self.closed & FULL
        } else {
            1 << self.target
        };
        let mut total = 0;
        for board in grid::cells(boards) {
            let features = self.board_features(board);
            for cell in grid::cells(features.empty) {
                total += policy.weights[Board::cell_class(&features, threats, cell)];
            }
        }
        if total == 0 {
            return self.random_cell(rng);
        }
        let mut pick = rng.below(u64::from(total)) as u32;
        for board in grid::cells(boards) {
            let features = self.board_features(board);
            for cell in grid::cells(features.empty) {
                let weight = policy.weights[Board::cell_class(&features, threats, cell)];
                if pick < weight {
                    return (board, cell);
                }
                pick -= weight;
            }
        }
        unreachable!("the draw is below the total weight")
    }

    /// Plays [`policy_move`](Board::policy_move)s until the game ends, and
    /// returns how it ended.
    pub fn policy_playout(&mut self, policy: &PlayoutPolicy, rng: &mut Rng) -> Status {
        while self.status == Status::Ongoing {
            let (board, cell) = self.policy_cell(policy, rng);
            self.play_at(board, cell);
        }
        self.status
    }
}

#[cfg(test)]
mod tests;
