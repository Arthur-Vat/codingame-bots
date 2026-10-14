//! The policies' positions as games for `cg-search`.

use cg_core::rng::Rng;
use cg_search::Game;
use uttt_engine::search::score;
use uttt_engine::{Board, Move};

use crate::{PatternPolicy, PlayoutPolicy, PolicyMoves};

/// A position searched with policy playouts: [`Board`]'s game, except that
/// playouts follow `policy` ([`PolicyMoves::policy_playout`]) instead of
/// [`Board::decisive_playout`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PolicyBoard {
    pub board: Board,
    pub policy: &'static PlayoutPolicy,
}

impl Game for PolicyBoard {
    type Move = Move;

    fn to_move(&self) -> usize {
        self.board.to_move()
    }

    fn legal_moves(&self, moves: &mut Vec<Move>) {
        Game::legal_moves(&self.board, moves);
    }

    fn play(&mut self, mv: Move) {
        self.board.play(mv);
    }

    fn score(&self) -> Option<f64> {
        score(self.board.status())
    }

    fn priors(&self, moves: &[Move], weights: &mut Vec<f32>) {
        self.board.move_weights(self.policy, moves, weights);
    }

    fn playout(&mut self, rng: &mut Rng) -> f64 {
        score(self.board.policy_playout(self.policy, rng)).expect("a playout ends the game")
    }
}

/// A position searched with a pattern policy (E015): its weights order a
/// node's children, when the search uses priors, and draw the playouts'
/// first moves ([`PolicyMoves::pattern_playout`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PatternBoard {
    pub board: Board,
    pub policy: &'static PatternPolicy,
}

impl Game for PatternBoard {
    type Move = Move;

    fn to_move(&self) -> usize {
        self.board.to_move()
    }

    fn legal_moves(&self, moves: &mut Vec<Move>) {
        Game::legal_moves(&self.board, moves);
    }

    fn play(&mut self, mv: Move) {
        self.board.play(mv);
    }

    fn score(&self) -> Option<f64> {
        score(self.board.status())
    }

    fn priors(&self, moves: &[Move], weights: &mut Vec<f32>) {
        self.board.pattern_move_weights(self.policy, moves, weights);
    }

    fn playout(&mut self, rng: &mut Rng) -> f64 {
        score(self.board.pattern_playout(self.policy, rng)).expect("a playout ends the game")
    }
}
