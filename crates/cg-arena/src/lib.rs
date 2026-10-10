//! Plays bots against each other through a game's referee, the way
//! CodinGame does: each bot is a separate process that reads its turn input
//! on stdin and answers on stdout within the game's time limit.
//!
//! - [`referee`]: the trait each game implements.
//! - [`runner`]: one match between two bot processes.
//! - [`record`]: full game records, and the sample of a run to keep.
//! - [`tournament`]: many matches in parallel, in seat-swapped pairs.
//! - [`summary`]: results from the first bot's point of view.
//! - [`sprt`]: the sequential test deciding whether a candidate is stronger.
//! - [`ratings`]: Elo ratings from many matchups.
//! - [`cli`]: the command line every game's arena binary shares.

pub mod cli;
pub mod ratings;
pub mod record;
pub mod referee;
pub mod runner;
pub mod sprt;
pub mod summary;
pub mod tournament;
