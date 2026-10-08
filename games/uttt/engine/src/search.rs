//! The engine as a game for `cg-search`.

use cg_core::rng::Rng;
use cg_search::Game;

use crate::board::{Board, PlayoutPolicy, Status};
use crate::moves::{Move, MoveList};
use crate::value::Network;

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

/// A position searched with a value network (ADR 0017): [`Board`]'s game,
/// except that a new leaf gets an estimate instead of a playout. A side to
/// move with a move that wins the game scores a win, exactly; otherwise
/// the network judges the position. The network has `H` and `H2` hidden
/// units, those of [`crate::value::ValueNetwork`] by default.
#[derive(Clone, Copy)]
pub struct ValueBoard<const H: usize = 64, const H2: usize = 16> {
    pub board: Board,
    pub network: &'static Network<H, H2>,
}

impl<const H: usize, const H2: usize> ValueBoard<H, H2> {
    /// Seat 0's expected score in a position where the game goes on.
    pub fn estimate(&self) -> f64 {
        network_estimate(&self.board, self.network)
    }
}

/// Seat 0's expected score in `board`, where the game goes on: a win when
/// the side to move can win the game at once, otherwise the network's.
fn network_estimate<const H: usize, const H2: usize>(
    board: &Board,
    network: &Network<H, H2>,
) -> f64 {
    let value = if board.game_winning_move().is_some() {
        1.0
    } else {
        f64::from(network.evaluate(board))
    };
    if board.to_move() == 0 {
        value
    } else {
        1.0 - value
    }
}

/// Positions are equal when their boards are: a search compares them to
/// find the current position in its tree.
impl<const H: usize, const H2: usize> PartialEq for ValueBoard<H, H2> {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl<const H: usize, const H2: usize> Game for ValueBoard<H, H2> {
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

/// A position searched with a value network and playouts together, the
/// fallback of ADR 0017: a new leaf's estimate is `share` of the network's
/// ([`ValueBoard::estimate`]) and the rest the result of a playout that
/// follows `policy`. A side to move that can win the game at once still
/// scores a win, exactly.
#[derive(Clone, Copy)]
pub struct MixBoard<const H: usize = 64, const H2: usize = 16> {
    pub board: Board,
    pub network: &'static Network<H, H2>,
    pub policy: &'static PlayoutPolicy,
    /// The network's share of the estimate, from 0 to 1.
    pub share: f64,
}

impl<const H: usize, const H2: usize> PartialEq for MixBoard<H, H2> {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl<const H: usize, const H2: usize> Game for MixBoard<H, H2> {
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
        if self.board.game_winning_move().is_some() {
            return network_estimate(&self.board, self.network);
        }
        let estimate = network_estimate(&self.board, self.network);
        let mut board = self.board;
        let played =
            score(board.policy_playout(self.policy, rng)).expect("a playout ends the game");
        self.share * estimate + (1.0 - self.share) * played
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
