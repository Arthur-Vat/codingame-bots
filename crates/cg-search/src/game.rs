//! What a game must offer to be searched.

use cg_core::rng::Rng;

/// A two-player game of perfect information in which players take turns.
///
/// The position is cloned at every search iteration, so it should be small
/// and cheap to copy.
pub trait Game: Clone {
    type Move: Copy + PartialEq + std::fmt::Debug;

    /// The player to move: 0 or 1.
    fn to_move(&self) -> usize;

    /// Replaces the content of `moves` with the legal moves. There is at
    /// least one while the game goes on.
    fn legal_moves(&self, moves: &mut Vec<Self::Move>);

    /// Plays a legal move for the player to move.
    fn play(&mut self, mv: Self::Move);

    /// Player 0's score once the game is over: 1 for a win, 0.5 for a draw,
    /// 0 for a loss. `None` while the game goes on.
    fn score(&self) -> Option<f64>;

    /// Plays to the end and returns player 0's score. The default plays
    /// uniformly random legal moves; games may provide a faster or smarter
    /// version.
    fn playout(&mut self, rng: &mut Rng) -> f64 {
        let mut moves = Vec::new();
        loop {
            if let Some(score) = self.score() {
                return score;
            }
            self.legal_moves(&mut moves);
            let mv = *rng.pick(&moves).expect("a game that goes on has moves");
            self.play(mv);
        }
    }
}
