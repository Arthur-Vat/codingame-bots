//! The early game of A Code of Ice and Fire: the openings a bot plays until
//! the middle game, and the score that compares them there (experiments E003
//! and E004, `games/ice-and-fire/research/early-game.md`). Standard library
//! only, so the bundler can copy it into bots.
//!
//! - [`econ`]: the opening of `ice-and-fire-v003` (E003): units take free
//!   cells, trainings and mines are bought while they pay back.
//! - [`middle`]: when the middle game starts: the first start of a player's
//!   turn whose gold pays a chain of trainings to an enemy cell.
//! - [`score`]: a hand-built score of a state in gold, for comparing openings
//!   at the middle game: each side's Voronoi share of the map, then its
//!   economy 3 turns into the middle game, less what the enemy could cut
//!   from it.
//!
//! The harness (`examples/early`) plays openings against each other from both
//! seats of many maps, up to the middle game, and reports the score.

pub mod econ;
pub mod middle;
pub mod score;

#[cfg(test)]
mod test_util;
