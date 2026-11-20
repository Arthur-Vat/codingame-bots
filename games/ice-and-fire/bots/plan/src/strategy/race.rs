//! The race to each objective we do not hold: when each side can first be
//! there, in turns of the side.
//!
//! A unit arrives in its map distance. A training arrives in 1 (the unit
//! cannot move the turn it is trained) plus the map distance from the side's
//! nearest trainable cell (a valid cell next to its active land that a
//! level-1 unit may enter), plus the turns of income the side needs to
//! afford the unit (10 gold) if it cannot now; with no income it never can.
//! The side's arrival is the smaller, a unit winning a tie. We act first in
//! a turn, so a tie is ours.

use ice_and_fire_engine::State;

use super::objectives::Objective;

/// The gold of a level-1 unit.
const TRAIN_COST: i64 = 10;
/// The arrival turn counted for a side that cannot arrive, in `margin`.
const NEVER: i64 = 100;

/// How a side arrives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum How {
    /// By the unit of this id.
    Unit(u32),
    /// By a training on this cell.
    Train(usize),
}

/// A side's arrival at an objective.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arrival {
    pub turns: u32,
    pub how: How,
}

/// The race to one objective.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Race {
    pub cell: usize,
    /// Our best arrival.
    pub ours: Option<Arrival>,
    /// Our arrival by a training, whether or not it is the best.
    pub ours_training: Option<Arrival>,
    /// The enemy's best arrival.
    pub theirs: Option<Arrival>,
    /// The enemy's arrival turn less ours (a side that cannot arrive counts
    /// as 100): 0 or more means we are there first or tied.
    pub margin: i64,
}

/// The races of the objectives `seat` does not hold.
pub fn race(state: &State, seat: usize, objectives: &[Objective]) -> Vec<Race> {
    let mut out = Vec::new();
    for objective in objectives.iter().filter(|o| !o.held) {
        let cell = objective.cell;
        let (ours_unit, ours_training) = arrivals(state, seat, cell);
        let ours = best(ours_unit, ours_training);
        let (their_unit, their_training) = arrivals(state, 1 - seat, cell);
        let theirs = best(their_unit, their_training);
        let turns = |a: Option<Arrival>| a.map_or(NEVER, |a| i64::from(a.turns));
        out.push(Race {
            cell,
            ours,
            ours_training,
            theirs,
            margin: turns(theirs) - turns(ours),
        });
    }
    out
}

/// The smaller arrival, the unit's on a tie.
fn best(unit: Option<Arrival>, training: Option<Arrival>) -> Option<Arrival> {
    match (unit, training) {
        (Some(u), Some(t)) => Some(if t.turns < u.turns { t } else { u }),
        (u, t) => u.or(t),
    }
}

/// The arrival of `side` at `cell` by a unit and by a training.
fn arrivals(state: &State, side: usize, cell: usize) -> (Option<Arrival>, Option<Arrival>) {
    let map = state.map();
    let mut unit: Option<Arrival> = None;
    for from in state.units(side).iter() {
        let d = map.distance(from, cell);
        let Some(id) = state.unit_id(from) else {
            continue;
        };
        if d != u8::MAX && unit.is_none_or(|u| u32::from(d) < u.turns) {
            unit = Some(Arrival {
                turns: u32::from(d),
                how: How::Unit(id),
            });
        }
    }
    let wait = match (state.gold(side), state.income(side)) {
        (gold, _) if gold >= TRAIN_COST => Some(0),
        (_, income) if income <= 0 => None,
        (gold, income) => Some((TRAIN_COST - gold + income - 1) / income),
    };
    let mut training: Option<Arrival> = None;
    if let Some(wait) = wait {
        let trainable = state.active(side).neighbours() & map.valid & state.enterable(side, 1);
        for from in trainable.iter() {
            let d = map.distance(from, cell);
            if d != u8::MAX && training.is_none_or(|t| 1 + wait as u32 + u32::from(d) < t.turns) {
                training = Some(Arrival {
                    turns: 1 + wait as u32 + u32::from(d),
                    how: How::Train(from),
                });
            }
        }
    }
    (unit, training)
}
