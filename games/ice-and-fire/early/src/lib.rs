//! The early game of A Code of Ice and Fire in measures, on the engine's
//! [`State`](ice_and_fire_engine::State). Standard library only, so the
//! bundler can copy it into bots.
//!
//! - [`zones`]: the map's choke cells, centre cells and middle band, and a
//!   side's territory measures (active cells, Voronoi share, chokes held).
//! - [`score`]: a side's features at the start of its ply and the
//!   zasmu-likeness score (phase 0 of the early-game plan,
//!   `games/ice-and-fire/research/codingame/2026-11-19-early-game.md`): how
//!   much a side's position looks like zasmu's own at the same game turn and
//!   seat, as the log-likelihood ratio of a logistic regression fitted on the
//!   CodinGame games (`ice-and-fire-review early-fit`).

pub mod score;
pub mod zones;
