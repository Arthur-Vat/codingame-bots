//! The objectives of a map and the land each decides.
//!
//! An objective is a cell of the contested band or a middle choke
//! (`crate::eval::goal_zones`). Its value for a side is the land it gains by
//! holding it: the number of valid cells strictly nearer (steps over valid
//! cells) to the side's active land than to the enemy's, with the cell added
//! to its land, less the same count as it is (the enemy's land unchanged).
//! For a cell the side already holds, the land it would lose: the count as
//! it is less the count with the cell removed. With [`VALUE_SWING`] the value
//! is instead the swing of the review tool's `codingame-roles`: the count
//! with the cell added to the side's land less the count with it added to
//! the enemy's.

use ice_and_fire_engine::{Bits, State, CELLS, HQ};
use ice_and_fire_opening::econ::distances;

use crate::eval::goal_zones;

/// The least value for an objective to count: `None` counts every objective
/// that gains land (value above 0), `Some(3)` is the first cut.
pub const OBJECTIVE_MIN_VALUE: Option<i64> = None;

/// Whether the value is the swing (the land with the cell held by us less
/// held by the enemy) rather than the land gained.
pub const VALUE_SWING: bool = false;

/// What kind of cell an objective is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A middle choke (also when it is in the band).
    Choke,
    /// A cell of the contested band.
    Band,
}

/// An objective for one side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Objective {
    pub cell: usize,
    pub kind: Kind,
    /// Whether the side holds it: it is one of its active cells.
    pub held: bool,
    /// The land it decides.
    pub value: i64,
}

/// The objectives of `state` for `seat`, by value, highest first, then
/// chokes before band cells, then by cell.
pub fn objectives(state: &State, seat: usize) -> Vec<Objective> {
    objectives_valued(state, seat, VALUE_SWING)
}

/// [`objectives`] with the value as the swing (`swing`) or as the land
/// gained.
pub fn objectives_valued(state: &State, seat: usize, swing: bool) -> Vec<Objective> {
    let (band, chokes) = goal_zones(state);
    let (mine, theirs) = (state.active(seat), state.active(1 - seat));
    let (dm, dt) = (distances(state, mine), distances(state, theirs));
    let base = nearer(state, &dm, &dm, &dt);
    let mut out = Vec::new();
    for cell in (band | chokes).iter() {
        let dg = distances(state, Bits::cell(cell));
        let held = mine.test(cell);
        let value = if swing {
            nearer(state, &dm, &dg, &dt) - nearer(state, &dt, &dg, &dm)
        } else if held {
            let without = distances(state, flood_without(mine, seat, cell));
            base - nearer(state, &without, &without, &dt)
        } else {
            nearer(state, &dm, &dg, &dt) - base
        };
        out.push(Objective {
            cell,
            kind: if chokes.test(cell) {
                Kind::Choke
            } else {
                Kind::Band
            },
            held,
            value,
        });
    }
    out.sort_by_key(|o| (-o.value, o.kind != Kind::Choke, o.cell));
    out
}

/// The valid cells strictly nearer to the land of distances `a` plus the
/// cell of distances `g` than to the land of distances `b`.
fn nearer(state: &State, a: &[u8; CELLS], g: &[u8; CELLS], b: &[u8; CELLS]) -> i64 {
    let mut n = 0;
    for c in state.map().valid.iter() {
        let d = a[c].min(g[c]);
        if d != u8::MAX && d < b[c] {
            n += 1;
        }
    }
    n
}

/// The objectives that count: of value at least [`OBJECTIVE_MIN_VALUE`], or
/// above 0 when there is none.
pub fn counted(objectives: &[Objective]) -> Vec<Objective> {
    objectives
        .iter()
        .filter(|o| o.value >= OBJECTIVE_MIN_VALUE.unwrap_or(1))
        .copied()
        .collect()
}

/// The active land of `seat` if `cell` were not its: what is still joined
/// to its HQ over the rest of `active`.
fn flood_without(active: Bits, seat: usize, cell: usize) -> Bits {
    let rest = active.and_not(Bits::cell(cell));
    let mut reached = Bits::cell(HQ[seat]) & rest;
    loop {
        let next = reached | (reached.neighbours() & rest);
        if next == reached {
            return reached;
        }
        reached = next;
    }
}
