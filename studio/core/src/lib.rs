//! The studio's logic that needs the rules, and nothing else
//! ([ADR 0030](../../../docs/adr/0030-studio-on-github-pages.md)).
//!
//! The local server (`studio/server`) and the WebAssembly program of the
//! browser (`studio/rules`) both call this crate, so that the rules are
//! judged by one piece of Rust code, not two copies. Nothing here starts a
//! process or a thread, reads a file, or reads the clock: a caller passes
//! the time in. That is what lets it build for `wasm32-wasip1`.
//!
//! - [`games`]: the games the studio registers, in one place.
//! - [`session`]: the state of a game in progress (seats, turns, result),
//!   its settling, the answers played into it, and its record.
//! - [`request`]: the validation of a request to start a game.
//! - [`history`]: the checking of a game record, and the listing rows and
//!   filters of the saved games.
//! - [`speed`]: the conversion from a think time to iterations.

pub mod games;
pub mod history;
pub mod request;
pub mod session;
pub mod speed;

#[cfg(test)]
mod tests;
