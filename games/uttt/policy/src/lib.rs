//! Move policies for Ultimate Tic-Tac-Toe, for the trainer: E011's class
//! policy ([`PlayoutPolicy`], ADR 0016) and E015's pattern policy
//! ([`PatternPolicy`]), with the searches that use them ([`PolicyBoard`],
//! [`PatternBoard`]).
//!
//! They moved out of `uttt-engine` because the bundler copies the whole
//! engine into every bot and no bot uses them any more. Their methods on
//! [`Board`] are in the extension trait [`PolicyMoves`]: import it to call
//! them.

use cg_core::rng::Rng;
use uttt_engine::{Board, Move, Status};

mod patterns;
mod policy;
pub mod search;

pub use patterns::{
    encode_pattern_bytes, encode_pattern_weights, symmetric_cell, PatternPolicy, DESTINATIONS,
    PATTERNS, PATTERN_BYTES, PATTERN_CELLS, PATTERN_FEATURES,
};
pub use policy::{
    PlayoutPolicy, BLOCKS, CENTRE, CLASSES, FEATURES, FEATURE_NAMES, GIVES_BOARD,
    GIVES_FREE_CHOICE, WINS_BOARD,
};
pub use search::{PatternBoard, PolicyBoard};

/// The policy moves of a [`Board`].
pub trait PolicyMoves {
    /// The index of `mv`'s weight in a pattern policy: its canonical
    /// pattern and cell, then its destination.
    fn pattern_feature(&self, mv: Move) -> usize;

    /// The weight `policy` gives each of `moves`, which must be legal, in
    /// order, written into `weights`: the search's priors.
    fn pattern_move_weights(&self, policy: &PatternPolicy, moves: &[Move], weights: &mut Vec<f32>);

    /// A move drawn from `policy`, after a game-winning move if there is
    /// one. The game must go on.
    fn pattern_move(&self, policy: &PatternPolicy, rng: &mut Rng) -> Move;

    /// Plays [`pattern_move`](PolicyMoves::pattern_move)s for `policy`'s
    /// plies, then decisive random moves, until the game ends; returns how
    /// it ended.
    fn pattern_playout(&mut self, policy: &PatternPolicy, rng: &mut Rng) -> Status;

    /// The class of a legal move `mv` of the player to move: its features
    /// as bits ([`WINS_BOARD`], [`BLOCKS`], ...). The features judge the
    /// position before the move, except that a move closing its own small
    /// board is seen to give a free choice when it sends the opponent
    /// there.
    fn move_class(&self, mv: Move) -> usize;

    /// The weight `policy` gives each of `moves`, which must be legal, in
    /// order, written into `weights`: the search's priors (E014). Each
    /// small board's features are computed once.
    fn move_weights(&self, policy: &PlayoutPolicy, moves: &[Move], weights: &mut Vec<f32>);

    /// A move for playouts: a move that wins the game if there is one (as
    /// [`Board::decisive_move`]), else a move drawn with probability
    /// proportional to its class's weight in `policy`; if all the weights
    /// of the legal moves are 0, a uniformly random move. The game must go
    /// on.
    fn policy_move(&self, policy: &PlayoutPolicy, rng: &mut Rng) -> Move;

    /// Plays [`policy_move`](PolicyMoves::policy_move)s until the game
    /// ends, and returns how it ended.
    fn policy_playout(&mut self, policy: &PlayoutPolicy, rng: &mut Rng) -> Status;
}

impl PolicyMoves for Board {
    fn pattern_feature(&self, mv: Move) -> usize {
        patterns::pattern_feature(self, mv)
    }

    fn pattern_move_weights(&self, policy: &PatternPolicy, moves: &[Move], weights: &mut Vec<f32>) {
        patterns::pattern_move_weights(self, policy, moves, weights);
    }

    fn pattern_move(&self, policy: &PatternPolicy, rng: &mut Rng) -> Move {
        patterns::pattern_move(self, policy, rng)
    }

    fn pattern_playout(&mut self, policy: &PatternPolicy, rng: &mut Rng) -> Status {
        patterns::pattern_playout(self, policy, rng)
    }

    fn move_class(&self, mv: Move) -> usize {
        policy::move_class(self, mv)
    }

    fn move_weights(&self, policy: &PlayoutPolicy, moves: &[Move], weights: &mut Vec<f32>) {
        policy::move_weights(self, policy, moves, weights);
    }

    fn policy_move(&self, policy: &PlayoutPolicy, rng: &mut Rng) -> Move {
        policy::policy_move(self, policy, rng)
    }

    fn policy_playout(&mut self, policy: &PlayoutPolicy, rng: &mut Rng) -> Status {
        policy::policy_playout(self, policy, rng)
    }
}
