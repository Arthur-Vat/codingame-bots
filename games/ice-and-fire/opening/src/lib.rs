//! The early game of A Code of Ice and Fire: the openings a bot plays until
//! the middle game, and the score that compares them there (experiments E003,
//! E004 and E010, `games/ice-and-fire/research/early-game.md`). Standard library
//! only, so the bundler can copy it into bots.
//!
//! - [`econ`]: the opening of `ice-and-fire-v003` (E003): units take free
//!   cells, trainings and mines are bought while they pay back.
//! - [`middle`]: when the middle game starts: the first start of a player's
//!   turn whose gold pays a chain of trainings to an enemy cell.
//! - [`score`]: a hand-built score of a state in gold, for comparing openings
//!   at the middle game: each side's economy 3 turns into the middle game,
//!   then its Voronoi share of the map, less what the enemy could cut from
//!   it.
//!
//! - [`solo`]: the early game planned for one side alone (E010): shape
//!   measures (joints, cells one capture cuts off, line ends), a value of a
//!   position, and an anytime beam search over whole turns.
//!
//! The harness (`examples/early`) plays openings against each other from both
//! seats of many maps, up to the middle game, and reports the score.

pub mod econ;
pub mod middle;
pub mod plan;
pub mod planned;
pub mod portfolio;
pub mod rollout;
pub mod score;
pub mod solo;

#[cfg(test)]
mod test_util;
