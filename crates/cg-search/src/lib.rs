//! Game-independent search for bots.
//!
//! - [`game`]: what a game must offer to be searched.
//! - [`mcts`]: Monte Carlo tree search (UCT), which can keep thinking on one
//!   position ([`Mcts::think`]).
//! - [`report`]: what the search knows of its root moves (visits, scores,
//!   draws, proofs, lines), read-only, for analysis.
//! - [`budget`]: how long a search may run, from the turn's time limit, the
//!   arena's time scale, or a fixed number of iterations for tests.
//!
//! This crate uses the standard library only, so the bundler can copy it
//! into bots.

pub mod budget;
pub mod game;
pub mod mcts;
pub mod report;

pub use budget::Budget;
pub use game::Game;
pub use mcts::{CountDraws, Draws, Mcts, NoDraws, SearchResult};
pub use report::{MoveReport, Report};
