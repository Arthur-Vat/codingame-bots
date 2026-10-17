//! Fast A Code of Ice and Fire rules for bots, as described in
//! `games/ice-and-fire/RULES.md`.
//!
//! The 144 cells are bitboards ([`bits::Bits`]); a [`State`] clones in a
//! memcpy of about 1 KB and shares the map's distance table. The engine
//! parses CodinGame's input ([`State::read`]), applies commands one by one
//! ([`State::apply`]) or a whole turn ([`State::play_turn`]), draws random
//! turns ([`State::random_turn`]) and writes the input back out
//! ([`State::turn_input`]).
//!
//! It uses the standard library and `cg-core` only, so the bundler can copy
//! it into bots. The engine's parity tests check it against the readable
//! reference referee (`ice-and-fire-referee`) turn by turn.

pub mod bits;
pub mod command;
pub mod league;
pub mod map;
pub mod random;
pub mod state;
pub mod text;

pub use bits::{Bits, CELLS, SIZE};
pub use command::{answer_line, Command};
pub use league::League;
pub use state::{Outcome, State, HQ, MAX_TURNS};

#[cfg(test)]
mod tests;
