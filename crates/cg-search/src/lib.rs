//! Game-independent search for bots.
//!
//! - [`game`]: what a game must offer to be searched.
//! - [`mcts`]: Monte Carlo tree search (UCT).
//! - [`budget`]: how long a search may run, from the turn's time limit, the
//!   arena's time scale, or a fixed number of iterations for tests.
//!
//! This crate uses the standard library only, so the bundler can copy it
//! into bots.

pub mod budget;
pub mod game;
pub mod mcts;

pub use budget::Budget;
pub use game::Game;
pub use mcts::{Mcts, Outcome, SearchResult};
