//! Games where one side searches longer on its first moves: the most that
//! an opening book of deeper searches could bring (journal entry E013).
//!
//! Both sides are the bot's search, with its playouts and its tree kept
//! from move to move. Every game starts from the empty board, as on
//! CodinGame. Each side's first move gets the first turn's longer budget;
//! the side with the head start gets `boost` times the usual budget on its
//! first `moves` moves, or the first turn's budget if that is larger.

use cg_core::rng::Rng;
use cg_search::{Budget, Game, Mcts};
use uttt_engine::search::PolicyBoard;
use uttt_engine::{Board, PlayoutPolicy, Status};

use crate::duel::Results;

/// How head-start games are played.
#[derive(Clone, Copy, Debug)]
pub struct HeadStart {
    /// Search iterations per move after the first, for both sides.
    pub iterations: u64,
    /// Search iterations of each side's first move.
    pub first_iterations: u64,
    /// How many times the usual budget the side with the head start gets.
    pub boost: u64,
    /// How many of its first moves get the boost.
    pub moves: u32,
    /// The exploration constant of both searches, as in the bot.
    pub exploration: f64,
}

impl HeadStart {
    /// The iterations of a side's move `number` (1 for its first move).
    pub fn budget(&self, number: u32, boosted: bool) -> u64 {
        let usual = if number == 1 {
            self.first_iterations
        } else {
            self.iterations
        };
        if boosted && number <= self.moves {
            usual.max(self.boost * self.iterations)
        } else {
            usual
        }
    }

    /// Plays `pairs` pairs, in which the side with the head start plays
    /// first, then second. Results are from its side. Spread over
    /// `threads` threads; the same seed gives the same games.
    pub fn play(
        &self,
        policy: &'static PlayoutPolicy,
        pairs: u32,
        seed: u64,
        threads: usize,
    ) -> Results {
        let threads = threads.clamp(1, pairs.max(1) as usize) as u32;
        let parts: Vec<Results> = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..threads)
                .map(|part| {
                    scope.spawn(move || {
                        let mut results = Results::default();
                        for pair in (part..pairs).step_by(threads as usize) {
                            results.add(&self.play_pair(policy, seed, pair));
                        }
                        results
                    })
                })
                .collect();
            handles
                .into_iter()
                .map(|handle| handle.join().expect("a game thread panicked"))
                .collect()
        });
        let mut total = Results::default();
        for part in &parts {
            total.add(part);
        }
        total
    }

    fn play_pair(&self, policy: &'static PlayoutPolicy, seed: u64, pair: u32) -> Results {
        let mut rng = Rng::new(seed ^ u64::from(pair).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let mut results = Results::default();
        let mut points = 0;
        for boosted_seat in 0..2 {
            let mut searches = [
                Mcts::new(self.exploration, rng.next_u64()),
                Mcts::new(self.exploration, rng.next_u64()),
            ];
            let mut played = [0u32; 2];
            let mut moves = Vec::new();
            let mut board = Board::new();
            while board.status() == Status::Ongoing {
                let seat = board.to_move();
                played[seat] += 1;
                let budget = self.budget(played[seat], seat == boosted_seat);
                let root = PolicyBoard { board, policy };
                root.legal_moves(&mut moves);
                let result = searches[seat].search(&root, &moves, Budget::Iterations(budget));
                board.play(result.best);
            }
            match board.status() {
                Status::Win(seat) if seat == boosted_seat => {
                    results.wins += 1;
                    points += 2;
                }
                Status::Win(_) => results.losses += 1,
                _ => {
                    results.draws += 1;
                    points += 1;
                }
            }
        }
        results.pairs[points] += 1;
        results
    }
}

#[cfg(test)]
mod tests;
