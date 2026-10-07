//! The engine as a game for `cg-search`.

use cg_core::rng::Rng;
use cg_search::Game;

use crate::board::{Board, Status};
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
        score(self.random_playout(rng)).expect("a playout ends the game")
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
