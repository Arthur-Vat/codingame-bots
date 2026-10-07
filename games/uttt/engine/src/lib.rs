//! Fast Ultimate Tic-Tac-Toe rules for bots, as described in
//! `games/uttt/RULES.md`.
//!
//! The whole position fits in a few machine words and is `Copy`: search
//! copies it instead of undoing moves. Each small board is a 9-bit mask per
//! player, with cell `3 * row + col` at bit `3 * row + col`.
//!
//! The engine uses the standard library only, so the bundler can copy it
//! into bots. [`search`] makes [`Board`] a game that `cg-search` can
//! search. Its parity tests check it against the readable reference referee
//! (`uttt-referee`) move by move.

pub mod board;
pub mod grid;
pub mod moves;
pub mod search;

pub use board::{Board, Status};
pub use moves::{Move, MoveList};
