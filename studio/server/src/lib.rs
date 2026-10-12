//! The studio's local server ([ADR 0025](../../../docs/adr/0025-studio.md)).
//!
//! It serves the built front end and a JSON API to list games and releases,
//! play friend, computer and bot-against-bot games, take moves back by
//! replaying, and export a game record, and keeps the history of saved games. The
//! HTTP layer is thin: [`handle`] holds all the routing and [`server`] only
//! adapts `tiny_http` to it.
//!
//! The logic that needs the rules, and no process, file or clock, is in
//! `studio-core`, which the browser's WebAssembly program shares.
//!
//! - [`state`]: the registered games, paths and sessions.
//! - [`api`]: routing and the endpoints.
//! - [`history`]: the saved games, one file each.
//! - [`archive`]: builds an archive of arena games from folders of records
//!   (the `studio-archive` tool, [ADR 0031](../../../docs/adr/0031-arena-games-archive.md)).
//! - [`session`]: one game in progress, its JSON view and its record.
//! - [`runner`]: the threads that start bots and play their turns.
//! - [`releases`]: lists a game's releases and compiles them on first use.
//! - [`speed`]: measures a release's iterations per millisecond.
//! - [`web`]: serves the front end's files.
//! - [`server`]: the sockets.

pub mod api;
pub mod archive;
pub mod history;
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

#[cfg(test)]
mod history_tests;

#[cfg(test)]
mod archive_tests;

#[cfg(all(test, unix))]
mod bot_tests;
