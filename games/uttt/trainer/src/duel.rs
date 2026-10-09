//! Games between a search with the value network and one with the bot's
//! playouts, with the same budget per move: whether the network makes a
//! better search. ADR 0019 plays them at the bot's own time budgets.

use std::time::{Duration, Instant};

use cg_core::rng::Rng;
use cg_search::{Budget, Game, Mcts};
use uttt_engine::search::{MixBoard, PolicyBoard, ValueBoard};
use uttt_engine::value::Network;
use uttt_engine::{Board, Move, PlayoutPolicy, Status};

/// A search's budget for one move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Limit {
    Iterations(u64),
    Time(Duration),
}

impl Limit {
    fn budget(self) -> Budget {
        match self {
            Limit::Iterations(count) => Budget::Iterations(count),
            Limit::Time(time) => Budget::Until(Instant::now() + time),
        }
    }
}

/// How duel games are played.
#[derive(Clone, Copy, Debug)]
pub struct Duel {
    /// Each side's budget for its first searched move, which on CodinGame
    /// gets a longer time limit.
    pub first: Limit,
    /// Each side's budget for its later moves.
    pub later: Limit,
    /// Random moves at the start of each pair's games.
    pub opening_plies: u32,
    /// The exploration constant of the search with playouts, as in the bot.
    pub exploration: f64,
    /// The exploration constant of the search with the network, whose
    /// estimates spread less than playout results.
    pub network_exploration: f64,
    /// The network's share of each leaf's estimate, the rest coming from a
    /// playout: 1 for the network alone.
    pub network_share: f64,
}

/// Results from one side, the network's in a duel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Results {
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
    /// Pairs by the network's points in them: 0, ½, 1, 1½ or 2.
    pub pairs: [u32; 5],
}

/// An Elo difference with its 95% interval.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Elo {
    pub elo: f64,
    pub low: f64,
    pub high: f64,
}

impl Results {
    /// The network's Elo advantage, from the pairs' scores, as the arena
    /// computes it. `None` with fewer than two pairs or a score of 0 or 1.
    pub fn elo(&self) -> Option<Elo> {
        let count: u32 = self.pairs.iter().sum();
        if count < 2 {
            return None;
        }
        let n = f64::from(count);
        let value = |k: usize| k as f64 / 4.0;
        let mean = (0..5)
            .map(|k| value(k) * f64::from(self.pairs[k]))
            .sum::<f64>()
            / n;
        if mean <= 0.0 || mean >= 1.0 {
            return None;
        }
        let variance = (0..5)
            .map(|k| f64::from(self.pairs[k]) * (value(k) - mean).powi(2))
            .sum::<f64>()
            / n;
        let margin = 1.96 * (variance / n).sqrt();
        let elo = |score: f64| -400.0 * (1.0 / score.clamp(1e-6, 1.0 - 1e-6) - 1.0).log10();
        Some(Elo {
            elo: elo(mean),
            low: elo(mean - margin),
            high: elo(mean + margin),
        })
    }

    /// The gate of ADR 0018: the network is not clearly weaker, that is
    /// the 95% interval of its Elo advantage reaches 0 or above. Without an
    /// interval, its score must be at least half the points.
    pub fn not_clearly_weaker(&self) -> bool {
        match self.elo() {
            Some(elo) => elo.high >= 0.0,
            None => {
                let games = self.wins + self.draws + self.losses;
                games > 0 && 2 * self.wins + self.draws >= games
            }
        }
    }

    /// Adds `other`'s games to these.
    pub fn add(&mut self, other: &Results) {
        self.wins += other.wins;
        self.draws += other.draws;
        self.losses += other.losses;
        for (sum, count) in self.pairs.iter_mut().zip(other.pairs) {
            *sum += count;
        }
    }
}

/// One side of a game.
trait Side {
    fn choose(&mut self, board: &Board) -> Move;
}

/// A side that searches its own kind of position.
struct Searcher<G: Game<Move = Move>, F: Fn(Board) -> G> {
    mcts: Mcts<G>,
    position: F,
    first: Limit,
    later: Limit,
    searched: u32,
    moves: Vec<Move>,
}

impl<G: Game<Move = Move>, F: Fn(Board) -> G> Side for Searcher<G, F> {
    fn choose(&mut self, board: &Board) -> Move {
        let root = (self.position)(*board);
        root.legal_moves(&mut self.moves);
        let limit = if self.searched == 0 {
            self.first
        } else {
            self.later
        };
        self.searched += 1;
        self.mcts.search(&root, &self.moves, limit.budget()).best
    }
}

impl Duel {
    /// Plays `pairs` pairs: in each, the two games start from the same
    /// random opening and the sides swap seats. Spread over `threads`
    /// threads; the same seed gives the same games.
    pub fn play<const H: usize, const H2: usize>(
        &self,
        network: &'static Network<H, H2>,
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
                            results.add(&self.play_pair(network, policy, seed, pair));
                        }
                        results
                    })
                })
                .collect();
            handles
                .into_iter()
                .map(|handle| handle.join().expect("a duel thread panicked"))
                .collect()
        });
        let mut total = Results::default();
        for part in &parts {
            total.add(part);
        }
        total
    }

    fn play_pair<const H: usize, const H2: usize>(
        &self,
        network: &'static Network<H, H2>,
        policy: &'static PlayoutPolicy,
        seed: u64,
        pair: u32,
    ) -> Results {
        let mut rng = Rng::new(seed ^ u64::from(pair).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let mut opening = Board::new();
        for _ in 0..self.opening_plies {
            if opening.status() != Status::Ongoing {
                break;
            }
            opening.play(opening.random_move(&mut rng));
        }
        let mut results = Results::default();
        let mut points = 0;
        for network_seat in 0..2 {
            let network_seed = rng.next_u64();
            let mut with_network: Box<dyn Side> = if self.network_share >= 1.0 {
                Box::new(Searcher {
                    mcts: Mcts::new(self.network_exploration, network_seed),
                    position: |board| ValueBoard { board, network },
                    first: self.first,
                    later: self.later,
                    searched: 0,
                    moves: Vec::new(),
                })
            } else {
                let share = self.network_share;
                Box::new(Searcher {
                    mcts: Mcts::new(self.network_exploration, network_seed),
                    position: move |board| MixBoard {
                        board,
                        network,
                        policy,
                        share,
                    },
                    first: self.first,
                    later: self.later,
                    searched: 0,
                    moves: Vec::new(),
                })
            };
            let mut with_playouts = Searcher {
                mcts: Mcts::new(self.exploration, rng.next_u64()),
                position: |board| PolicyBoard { board, policy },
                first: self.first,
                later: self.later,
                searched: 0,
                moves: Vec::new(),
            };
            let mut board = opening;
            while board.status() == Status::Ongoing {
                let mv = if board.to_move() == network_seat {
                    with_network.choose(&board)
                } else {
                    with_playouts.choose(&board)
                };
                board.play(mv);
            }
            match board.status() {
                Status::Win(seat) if seat == network_seat => {
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
