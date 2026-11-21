//! The roles of our units: at most one frontline unit per counted objective
//! we can reach first or at the same time, kept between turns.
//!
//! Rules of [`assign`]:
//! - Only counted objectives (`objectives::counted`: every one that gains
//!   land, or those of value at least `OBJECTIVE_MIN_VALUE`) that we do not
//!   hold and whose race margin is 0 or more take a role, in rank order
//!   (value, chokes first, then our arrival, then cell), while the side can
//!   cover them: a free unit, or a training it can pay for.
//! - A unit keeps its objective while it lives and the objective takes a
//!   role, unless another unit that is not frontline is nearer to it by at
//!   least one step: the handover, the old unit going to the back line (and
//!   staying there for this turn).
//! - The other such objectives, by value then our arrival, take the
//!   nearest free unit, or a training when that is as near or nearer
//!   (`Race::ours_training` before the unit): the cell to train on is then
//!   listed instead of a unit. A training cell serves one objective (the best
//!   ranked), and trainings are assigned only while the gold left after
//!   those already assigned pays 10 for one.

use std::collections::{BTreeMap, BTreeSet};

use ice_and_fire_engine::{Bits, State};

use super::objectives::{counted, Kind, Objective};
use super::race::{How, Race};

/// The gold of a level-1 unit.
const TRAIN_COST: i64 = 10;

/// The objective cell of each frontline unit, by unit id, between turns.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Memory {
    pub objective_of: BTreeMap<u32, usize>,
}

/// The roles of one turn.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Roles {
    /// Frontline units, with their objective cells, by objective order.
    pub frontline: Vec<(u32, usize)>,
    /// The cell to train on, with the objective it serves.
    pub trainings: Vec<(usize, usize)>,
    /// The other units, by id.
    pub back: Vec<u32>,
}

/// The roles of the units of `seat`, `memory` read and updated.
pub fn assign(
    state: &State,
    seat: usize,
    objectives: &[Objective],
    races: &[Race],
    memory: &mut Memory,
) -> Roles {
    assign_among(state, seat, objectives, races, memory, state.units(seat))
}

/// [`assign`] with only the units on `eligible` cells taking a frontline
/// role (E077's back units walking); the others are back-line units.
pub fn assign_among(
    state: &State,
    seat: usize,
    objectives: &[Objective],
    races: &[Race],
    memory: &mut Memory,
    eligible: Bits,
) -> Roles {
    let map = state.map();
    // The units of ours that may take a role, by id, with their cells.
    let units: BTreeMap<u32, usize> = (state.units(seat) & eligible)
        .iter()
        .filter_map(|c| state.unit_id(c).map(|id| (id, c)))
        .collect();
    // The objectives that take a role, in order: value, our arrival, cell.
    let mut open: Vec<(Objective, Race)> = Vec::new();
    for o in counted(objectives).into_iter().filter(|o| !o.held) {
        if let Some(r) = races.iter().find(|r| r.cell == o.cell) {
            if r.ours.is_some() && r.margin >= 0 {
                open.push((o, *r));
            }
        }
    }
    open.sort_by_key(|(o, r)| {
        (
            -o.value,
            o.kind != Kind::Choke,
            r.ours.map_or(u32::MAX, |a| a.turns),
            o.cell,
        )
    });

    let dist = |unit: u32, goal: usize| u32::from(map.distance(units[&unit], goal));
    let in_play = |holder: &BTreeMap<usize, u32>, released: &BTreeSet<u32>, id: u32| {
        released.contains(&id) || holder.values().any(|&u| u == id)
    };

    // Keep the roles of the units that live and whose objectives still take
    // one (the lowest id on an objective two units hold).
    let mut holder: BTreeMap<usize, u32> = BTreeMap::new(); // objective -> unit
    for (&id, &goal) in &memory.objective_of {
        if units.contains_key(&id)
            && open.iter().any(|(o, _)| o.cell == goal)
            && !holder.contains_key(&goal)
        {
            holder.insert(goal, id);
        }
    }

    // The handover: a unit that is not frontline and is nearer by a step.
    let mut released: BTreeSet<u32> = BTreeSet::new();
    for (o, _) in &open {
        let Some(&keeper) = holder.get(&o.cell) else {
            continue;
        };
        let nearest = units
            .keys()
            .copied()
            .filter(|&id| !in_play(&holder, &released, id))
            .map(|id| (dist(id, o.cell), id))
            .filter(|&(d, _)| d != u32::from(u8::MAX))
            .min();
        if let Some((d, id)) = nearest {
            if d < dist(keeper, o.cell) {
                released.insert(keeper);
                holder.insert(o.cell, id);
            }
        }
    }

    // The objectives left: the nearest free unit, or a training.
    let mut trainings = Vec::new();
    for (o, r) in &open {
        if holder.contains_key(&o.cell) {
            continue;
        }
        let free = units
            .keys()
            .copied()
            .filter(|&id| !in_play(&holder, &released, id))
            .map(|id| (dist(id, o.cell), id))
            .filter(|&(d, _)| d != u32::from(u8::MAX))
            .min();
        let unit_turns = free.map(|(d, _)| i64::from(d));
        // A training needs a cell no better objective uses and the gold
        // left after the trainings already assigned.
        let train = r.ours_training.and_then(|t| match t.how {
            How::Train(cell)
                if state.gold(seat) - TRAIN_COST * trainings.len() as i64 >= TRAIN_COST
                    && !trainings.iter().any(|&(c, _)| c == cell) =>
            {
                Some((cell, i64::from(t.turns)))
            }
            _ => None,
        });
        let train_turns = train.map(|(_, t)| t);
        let use_train = match (unit_turns, train_turns) {
            (Some(u), Some(t)) => t < u,
            (None, Some(_)) => true,
            _ => false,
        };
        let chosen = if use_train { train_turns } else { unit_turns };
        let Some(turns) = chosen else {
            continue;
        };
        if r.theirs.map_or(100, |a| i64::from(a.turns)) < turns {
            continue;
        }
        match (use_train, train, free) {
            (true, Some((cell, _)), _) => trainings.push((cell, o.cell)),
            (false, _, Some((_, id))) => {
                holder.insert(o.cell, id);
            }
            _ => {}
        }
    }

    let mut out = Roles {
        trainings,
        ..Roles::default()
    };
    for (o, _) in &open {
        if let Some(&id) = holder.get(&o.cell) {
            out.frontline.push((id, o.cell));
        }
    }
    out.back = state
        .units(seat)
        .iter()
        .filter_map(|c| state.unit_id(c))
        .filter(|id| !out.frontline.iter().any(|&(u, _)| u == *id))
        .collect();
    out.back.sort_unstable();
    memory.objective_of = out.frontline.iter().copied().collect();
    out
}
