//! Game-independent search for bots.
//!
//! - [`game`]: what a game must offer to be searched.
//! - [`mcts`]: Monte Carlo tree search (UCT).
//! - [`budget`]: how long a search may run, from the turn's time limit, the
//!   arena's time scale, or a fixed number of iterations for tests.
//!
//! This crate uses the standard library only, so the bundler can copy it
//! into bots.
//!
//! # Analysis
//!
//! With the cargo feature `analysis` (off by default), `Mcts` also offers
//! `think`, which keeps iterating on one position, and `report`, which
//! describes the root: every move's visits, average score, share of draws,
//! proof and most likely line (`report::Report`, `report::MoveReport`). Bots
//! never enable it. The code is in the `mcts::analysis` and `report`
//! modules and in a few lines of `mcts` marked `#[cfg(feature =
//! "analysis")]`, which the bundler drops with the line that follows
//! them, so a bundled bot is the same text with or without this feature
//! in the sources. Since rustc ignores the marked lines when the feature is
//! off, a wrong drop can only fail to compile, never change what a bot does.

pub mod budget;
pub mod game;
pub mod mcts;
#[cfg(feature = "analysis")]
pub mod report;

pub use budget::Budget;
pub use game::Game;
pub use mcts::{Mcts, SearchResult};
#[cfg(feature = "analysis")]
pub use report::{MoveReport, Report};
