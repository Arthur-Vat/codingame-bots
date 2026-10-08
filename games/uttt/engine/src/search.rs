//! The engine as a game for `cg-search`.

use cg_core::rng::Rng;
use cg_search::Game;

use crate::board::{Board, PlayoutPolicy, Status};
use crate::moves::{Move, MoveList};

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

    fn playout(&mut self, rng: &mut Rng) -> f64 {
        score(self.board.policy_playout(self.policy, rng)).expect("a playout ends the game")
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
