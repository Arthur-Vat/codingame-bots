//! The engine as a game for `cg-search`.

use cg_core::rng::Rng;
use cg_search::Game;

use crate::board::{Board, PatternPolicy, PlayoutPolicy, Status};
use crate::moves::{Move, MoveList};
use crate::nnue::Network;

impl Game for Board {
    type Move = Move;

    fn to_move(&self) -> usize {
        Board::to_move(self)
    }

    fn legal_moves(&self, moves: &mut Vec<Move>) {
        let mut list = MoveList::new();
        Board::legal_moves(self, &mut list);
        moves.clear();
        moves.extend_from_slice(&list);
    }

    fn play(&mut self, mv: Move) {
        Board::play(self, mv);
    }

    fn score(&self) -> Option<f64> {
        score(self.status())
    }

    fn playout(&mut self, rng: &mut Rng) -> f64 {
        score(self.decisive_playout(rng)).expect("a playout ends the game")
    }
}

/// A position searched with policy playouts: [`Board`]'s game, except that
/// playouts follow `policy` ([`Board::policy_playout`]) instead of
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
/// first moves ([`Board::pattern_playout`]).
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

/// A position searched with the NNUE (step C1): the pattern policy orders a node's children, and a new leaf gets the network's estimate instead of a playout. A side to move with a move that wins the game scores a win, exactly.
#[derive(Clone, Copy)]
pub struct NnueBoard<const N: usize = 256> {
    pub board: Board,
    pub network: &'static Network<N>,
    pub policy: &'static PatternPolicy,
}

impl<const N: usize> NnueBoard<N> {
    /// Seat 0's expected score in a position where the game goes on.
    pub fn estimate(&self) -> f64 {
        let value = if self.board.game_winning_move().is_some() {
            1.0
        } else {
            f64::from(self.network.evaluate_board(&self.board))
        };
        if self.board.to_move() == 0 {
            value
        } else {
            1.0 - value
        }
    }
}

/// Positions are equal when their boards are: a search compares them to find the current position in its tree.
impl<const N: usize> PartialEq for NnueBoard<N> {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl<const N: usize> Game for NnueBoard<N> {
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

    fn playout(&mut self, _rng: &mut Rng) -> f64 {
        self.estimate()
    }
}

/// Seat 0's score for a game status: 1, 0.5 or 0 once the game is over.
pub fn score(status: Status) -> Option<f64> {
    match status {
        Status::Ongoing => None,
        Status::Win(0) => Some(1.0),
        Status::Win(_) => Some(0.0),
        Status::Draw => Some(0.5),
    }
}

#[cfg(test)]
mod tests;
