//! What a game must offer to be searched.

use cg_core::rng::Rng;

/// A two-player game of perfect information in which players take turns.
///
/// The position is cloned at every search iteration, so it should be small
/// and cheap to copy. Positions are compared to find the current one in the
/// tree of an earlier search.
pub trait Game: Clone + PartialEq {
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

    /// How promising each of `moves`, legal here, looks: non-negative
    /// weights in the same order, written into `weights`. A search told to
    /// use priors tries the most promising moves first and favours them a
    /// little. The default leaves `weights` empty: no priors.
    fn priors(&self, moves: &[Self::Move], weights: &mut Vec<f32>) {
        let _ = moves;
        weights.clear();
    }

    /// Estimates the positions after each of `moves`, legal here, for a
    /// search that expands all the children of a leaf at once: writes into
    /// `scores`, in the same order, player 0's score and whether the game is
    /// over there, in which case the score is exact. The default plays each
    /// move and runs a playout from the positions where the game goes on;
    /// games may provide a faster version.
    fn child_scores(&self, moves: &[Self::Move], rng: &mut Rng, scores: &mut Vec<(f64, bool)>) {
        scores.clear();
        for &mv in moves {
            let mut next = self.clone();
            next.play(mv);
            scores.push(match next.score() {
                Some(score) => (score, true),
                None => (next.playout(rng), false),
            });
        }
    }

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
