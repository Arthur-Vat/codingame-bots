//! Many matches between two bots, in parallel, in seat-swapped pairs.
//!
//! Each pair plays one game seed twice, once with each bot in seat 0. This
//! cancels the advantage of moving first and the luck of the seed, including
//! the opening the referee imposes.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc;
use std::thread;

use cg_core::rng::Rng;
use serde::Serialize;

use crate::referee::{GameSetup, RefereeFactory};
use crate::runner::{run_match, ArenaError, BotSpec, MatchOptions, MatchRecord};

/// What to play.
#[derive(Clone, Debug)]
pub struct Tournament {
    /// The two bots. Results are reported from the first one's point of view.
    pub bots: [BotSpec; 2],
    /// Number of seat-swapped pairs; twice as many games are played.
    pub pairs: u32,
    /// Seed from which every game seed is derived.
    pub seed: u64,
    /// Moves the referee imposes at the start of each game (see
    /// [`GameSetup::opening_plies`]).
    pub opening_plies: u32,
    /// Matches played at the same time.
    pub jobs: usize,
    pub options: MatchOptions,
}

/// One game of a tournament.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GameRecord {
    pub pair: u32,
    /// False when the first bot sits in seat 0, true in the other game of the pair.
    pub swapped: bool,
    #[serde(flatten)]
    pub game: MatchRecord,
}

/// The seed of every game in pair `pair`.
pub fn pair_seed(tournament_seed: u64, pair: u32) -> u64 {
    Rng::new(tournament_seed ^ (u64::from(pair) << 32 | 0x5EED)).next_u64()
}

/// Whether to keep playing, as decided after each game.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flow {
    Continue,
    /// Start no new game. Games already running finish but are not reported.
    Stop,
}

/// Plays the tournament, calling `on_game` from the calling thread as games
/// finish, in completion order, until all games are played or `on_game`
/// returns [`Flow::Stop`]. `new_referee` builds each game.
///
/// Stops at the first match that cannot be played (for example a bot that
/// cannot be started) and returns that error.
pub fn run<F: RefereeFactory>(
    tournament: &Tournament,
    new_referee: &F,
    mut on_game: impl FnMut(&GameRecord) -> Flow,
) -> Result<(), ArenaError> {
    let total = tournament.pairs as usize * 2;
    let next = AtomicUsize::new(0);
    let stop = AtomicBool::new(false);
    let (sender, results) = mpsc::channel();
    let mut first_error = None;

    thread::scope(|scope| {
        for _ in 0..tournament.jobs.max(1) {
            let sender = sender.clone();
            let (next, stop) = (&next, &stop);
            scope.spawn(move || {
                while !stop.load(Ordering::Relaxed) {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    if index >= total {
                        break;
                    }
                    let result = play_game(tournament, new_referee, index);
                    if result.is_err() {
                        stop.store(true, Ordering::Relaxed);
                    }
                    if sender.send(result).is_err() {
                        break;
                    }
                }
            });
        }
        drop(sender);
        let mut stopped = false;
        for result in results {
            match result {
                Ok(_) if stopped => {}
                Ok(record) => {
                    if on_game(&record) == Flow::Stop {
                        stopped = true;
                        stop.store(true, Ordering::Relaxed);
                    }
                }
                Err(error) => {
                    first_error.get_or_insert(error);
                }
            }
        }
    });
    first_error.map_or(Ok(()), Err)
}

fn play_game<F: RefereeFactory>(
    tournament: &Tournament,
    new_referee: &F,
    index: usize,
) -> Result<GameRecord, ArenaError> {
    let pair = (index / 2) as u32;
    let swapped = index % 2 == 1;
    let seed = pair_seed(tournament.seed, pair);
    let [first, second] = &tournament.bots;
    let bots = if swapped {
        [second, first]
    } else {
        [first, second]
    };
    let mut referee = new_referee(&GameSetup {
        seed,
        opening_plies: tournament.opening_plies,
    });
    let game = run_match(referee.as_mut(), bots, seed, &tournament.options)?;
    Ok(GameRecord {
        pair,
        swapped,
        game,
    })
}

#[cfg(all(test, unix))]
mod tests;
