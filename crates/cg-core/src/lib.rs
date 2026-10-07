//! Code shared by bots: reading CodinGame input, a seeded random generator
//! and the arena's time factor.
//!
//! This crate uses the standard library only. `cg-bundler` copies it into
//! every bot that depends on it, so the result still compiles on CodinGame.

pub mod input;
pub mod rng;
pub mod time;
