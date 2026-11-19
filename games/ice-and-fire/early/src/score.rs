//! A side's early-game features and the zasmu-likeness score.
//!
//! The score of a side at the start of its ply of game turn
//! [`SCORE_TURN`] is the log-odds, by a logistic regression of its seat,
//! that the position is zasmu's rather than another player's: fitted by
//! `ice-and-fire-review early-fit` on the CodinGame games of the data branch
//! (all against our releases; our own sides left out), which writes
//! `score/weights.rs`. The owner chose it as the judge of the early game on
//! 2026-11-19 ("Zasmu-likeness score"); its validation, against full-game
//! results on CodinGame and in self-play, is in
//! `games/ice-and-fire/research/codingame/2026-11-19-zasmu-likeness.md`.

use ice_and_fire_engine::State;

use crate::zones::{share, to_centre, voronoi, Zones};

mod weights;

pub use weights::{MODELS, SCORE_TURN};

/// The number of features.
pub const FEATURES: usize = 16;

/// The features' names, in the order of [`features`].
pub const NAMES: [&str; FEATURES] = [
    "active",
    "voronoi",
    "mid",
    "units",
    "l2",
    "l3",
    "gold",
    "income",
    "owned",
    "mines",
    "towers",
    "centre",
    "to_centre",
    "chokes",
    "map_cells",
    "has_mid",
];

/// The middle chokes a map needs for its share to count: on maps with fewer
/// (most have none at [`CHOKE_SHARE`](crate::zones::CHOKE_SHARE)) the share
/// is 0 and `has_mid` tells them apart.
pub const MIN_MIDDLE_CHOKES: u32 = 2;

/// The largest `to_centre` counted, so that an unreachable centre stays a
/// number.
pub const MAX_TO_CENTRE: u8 = 20;

/// `seat`'s features in `state`, in the order of [`NAMES`]: its active
/// cells; its Voronoi share; its share of the middle chokes (0 on a map
/// with fewer than [`MIN_MIDDLE_CHOKES`]); its units, and those of level 2
/// and 3; its gold and income; its cells; its mines and towers; its centre
/// cells; the steps from its land to the centre (at most
/// [`MAX_TO_CENTRE`]); its share of all the chokes; the map's valid cells;
/// 1 if the map has at least [`MIN_MIDDLE_CHOKES`] middle chokes, else 0.
pub fn features(state: &State, seat: usize, zones: &Zones) -> [f64; FEATURES] {
    let owned = state.owned(seat);
    let units = state.units(seat);
    let level = |l: u8| {
        units
            .iter()
            .filter(|&c| state.unit_level(c) == Some(l))
            .count() as f64
    };
    let mid = zones.middle_chokes();
    let has_mid = mid.count() >= MIN_MIDDLE_CHOKES;
    [
        f64::from(state.active(seat).count()),
        voronoi(state, seat),
        if has_mid {
            share(state, seat, mid)
        } else {
            0.0
        },
        f64::from(units.count()),
        level(2),
        level(3),
        state.gold(seat) as f64,
        state.income(seat) as f64,
        f64::from(owned.count()),
        f64::from((owned & state.mines()).count()),
        f64::from((owned & state.towers()).count()),
        f64::from((owned & zones.centre).count()),
        f64::from(to_centre(state, seat, zones).min(MAX_TO_CENTRE)),
        share(state, seat, zones.chokes),
        f64::from(state.map().valid.count()),
        if has_mid { 1.0 } else { 0.0 },
    ]
}

/// A logistic regression on standardized features: the log-odds of
/// `x` is `bias + sum(weight[i] * (x[i] - mean[i]) / scale[i])`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Model {
    pub mean: [f64; FEATURES],
    pub scale: [f64; FEATURES],
    pub weight: [f64; FEATURES],
    pub bias: f64,
}

impl Model {
    /// The log-odds of the features `x`.
    pub fn logit(&self, x: &[f64; FEATURES]) -> f64 {
        let mut sum = self.bias;
        for (i, value) in x.iter().enumerate() {
            sum += self.weight[i] * (value - self.mean[i]) / self.scale[i];
        }
        sum
    }
}

/// `seat`'s zasmu-likeness in `state`, meant for the start of its ply of
/// game turn [`SCORE_TURN`]: the log-odds of its seat's model (`MODELS`,
/// red then blue). Higher is more like zasmu.
pub fn zasmu_likeness(state: &State, seat: usize, zones: &Zones) -> f64 {
    MODELS[seat].logit(&features(state, seat, zones))
}

#[cfg(test)]
mod tests;
