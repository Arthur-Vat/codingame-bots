//! Plays bots against each other through a game's referee, the way
//! CodinGame does: each bot is a separate process that reads its turn input
//! on stdin and answers on stdout within the game's time limit.
//!
//! - [`referee`]: the trait each game implements.
//! - [`runner`]: one match between two bot processes.
//! - [`record`]: full game records, and the sample of a run to keep.
//! - [`codingame`]: CodinGame game logs converted into records.
//! - [`live`]: a game driven one turn at a time, and a bot restarted by
//!   replaying a game into it, for the studio.
//! - [`tournament`]: many matches in parallel, in seat-swapped pairs.
//! - [`summary`]: results from the first bot's point of view.
//! - [`sprt`]: the sequential test deciding whether a candidate is stronger.
//! - [`blocks`]: that test over blocks of games played apart, on several runners.
//! - [`leaderboard`]: the generated `LEADERBOARD.md` of a game.
//! - [`league`]: the results file and opponent choice of the incremental league.
//! - [`night`]: the nightly league's plan of pairings.
//! - [`ratings`]: Elo ratings from many matchups.
//! - [`cli`]: the command line every game's arena binary shares.

pub mod blocks;
pub mod cli;
pub mod codingame;
pub mod leaderboard;
pub mod league;
pub mod live;
pub mod night;
pub mod ratings;
pub mod record;
pub mod referee;
pub mod runner;
pub mod sprt;
pub mod summary;
pub mod tournament;
