//! The studio's local server ([ADR 0025](../../../docs/adr/0025-studio.md)).
//!
//! It serves the built front end and a JSON API to list games and releases,
//! play friend, computer and bot-against-bot games, take moves back by
//! replaying, and export a game record. The HTTP layer is thin: [`handle`]
//! holds all the routing and [`server`] only adapts `tiny_http` to it.
//!
//! - [`state`]: the registered games, paths and sessions.
//! - [`api`]: routing and the endpoints.
//! - [`session`]: one game in progress, its JSON view and its record.
//! - [`runner`]: the threads that start bots and play their turns.
//! - [`releases`]: lists a game's releases and compiles them on first use.
//! - [`speed`]: measures a release's iterations per millisecond.
//! - [`web`]: serves the front end's files.
//! - [`server`]: the sockets.

pub mod api;
pub mod releases;
pub mod runner;
pub mod server;
pub mod session;
pub mod speed;
pub mod state;
pub mod web;

pub use api::{handle, Response};
pub use state::State;

#[cfg(test)]
mod tests;

#[cfg(all(test, unix))]
mod bot_tests;
