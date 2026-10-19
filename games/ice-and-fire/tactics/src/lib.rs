//! Tactics shared by the real bots of A Code of Ice and Fire, on the engine's
//! [`State`](ice_and_fire_engine::State). Standard library only, so the
//! bundler can copy it into bots.
//!
//! - [`chain`]: the cheapest chain of trainings that takes a cell, aware of
//!   towers (a capture can cut off a tower, whose protection then lapses).
//! - [`cut`]: short chains of trainings that remove enemy cells from its HQ's
//!   territory.
//! - [`eval`]: a static score of a state.
//!
//! Each function takes engine states only and never panics on a cell off the
//! map or void: such a cell is unreachable. The weights of [`eval`] and the
//! scores of [`cut`] are placeholders, to be tuned by later experiments.

pub mod chain;
pub mod cut;
pub mod eval;

#[cfg(test)]
mod test_util;
