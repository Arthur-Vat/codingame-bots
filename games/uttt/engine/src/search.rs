//! The engine as a game for `cg-search`.

use cg_core::rng::Rng;
use cg_search::Game;

use crate::board::{Board, PatternPolicy, PlayoutPolicy, Status};
use crate::moves::{Move, MoveList};
use crate::value::ValueNetwork;

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

/// A position searched with a value network (ADR 0017): [`Board`]'s game,
/// except that a new leaf gets an estimate instead of a playout. A side to
/// move with a move that wins the game scores a win, exactly; otherwise
/// the network judges the position.
#[derive(Clone, Copy)]
pub struct ValueBoard {
    pub board: Board,
    pub network: &'static ValueNetwork,
}

impl ValueBoard {
    /// Seat 0's expected score in a position where the game goes on.
    pub fn estimate(&self) -> f64 {
        let value = if self.board.game_winning_move().is_some() {
            1.0
        } else {
            f64::from(self.network.evaluate(&self.board))
        };
        if self.board.to_move() == 0 {
            value
        } else {
            1.0 - value
        }
    }
}

/// Positions are equal when their boards are: a search compares them to
/// find the current position in its tree.
impl PartialEq for ValueBoard {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl Game for ValueBoard {
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

    fn playout(&mut self, _rng: &mut Rng) -> f64 {
        self.estimate()
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
