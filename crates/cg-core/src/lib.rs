//! Code shared by bots: reading CodinGame input and a seeded random generator.
//!
//! This crate uses the standard library only. `cg-bundler` copies it into
//! every bot that depends on it, so the result still compiles on CodinGame.

pub mod input;
pub mod rng;
