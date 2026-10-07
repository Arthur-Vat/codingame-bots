//! Plays bots against each other through a game's referee, the way
//! CodinGame does: each bot is a separate process that reads its turn input
//! on stdin and answers on stdout within the game's time limit.
//!
//! - [`referee`]: the trait each game implements.
//! - [`runner`]: one match between two bot processes.
//! - [`tournament`]: many matches in parallel, in seat-swapped pairs.
//! - [`summary`]: results from the first bot's point of view.
//! - [`cli`]: the command line every game's arena binary shares.

pub mod cli;
pub mod referee;
pub mod runner;
pub mod summary;
pub mod tournament;
