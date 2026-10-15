//! The engine as a game for `cg-search`.

use cg_core::rng::Rng;
use cg_search::Game;

use crate::board::{Board, Status};
use crate::moves::{Move, MoveList};
use crate::nnue::{Accumulator, Network};

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

/// A position searched with the NNUE (step C1): a new leaf gets the network's estimate instead of a playout, and the search uses no priors (in E017 a pattern policy ordered a node's children). A side to move with a move that wins the game scores a win, exactly. A search that expands all children at once (step C2) gets their estimates from the node's accumulators, updated for each child. The position carries its accumulators and updates them with each move played, so a search walking down its tree never builds them from scratch (E022).
#[derive(Clone, Copy)]
pub struct NnueBoard<const N: usize = 256> {
    board: Board,
    network: &'static Network<N>,
    /// The accumulators of `board`, exactly as [`Network::refresh`] builds them.
    accumulator: Accumulator<N>,
}

impl<const N: usize> NnueBoard<N> {
    /// `board`, judged by `network`; builds its accumulators.
    pub fn new(board: Board, network: &'static Network<N>) -> Self {
        NnueBoard {
            board,
            network,
            accumulator: network.refresh(&board),
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn network(&self) -> &'static Network<N> {
        self.network
    }

    /// The accumulators of the position.
    pub fn accumulator(&self) -> &Accumulator<N> {
        &self.accumulator
    }

    /// Seat 0's expected score in a position where the game goes on.
    pub fn estimate(&self) -> f64 {
        self.estimate_from(&self.board, &self.accumulator)
    }

    /// Seat 0's expected score in `board`, where the game goes on, whose
    /// accumulators are `accumulator`.
    fn estimate_from(&self, board: &Board, accumulator: &Accumulator<N>) -> f64 {
        let value = if board.game_winning_move().is_some() {
            1.0
        } else {
            f64::from(self.network.evaluate(accumulator, board))
        };
        if board.to_move() == 0 {
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
        self.network
            .play(&mut self.accumulator, &mut self.board, mv);
    }

    fn score(&self) -> Option<f64> {
        score(self.board.status())
    }

    fn playout(&mut self, _rng: &mut Rng) -> f64 {
        self.estimate()
    }

    fn child_scores(&self, moves: &[Move], _rng: &mut Rng, scores: &mut Vec<(f64, bool)>) {
        scores.clear();
        for &mv in moves {
            let mut child = self.accumulator;
            let mut board = self.board;
            self.network.play(&mut child, &mut board, mv);
            scores.push(match score(board.status()) {
                Some(score) => (score, true),
                None => (self.estimate_from(&board, &child), false),
            });
        }
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
