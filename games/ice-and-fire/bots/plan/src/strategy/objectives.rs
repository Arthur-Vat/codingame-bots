//! The objectives of a map and the land each decides.
//!
//! An objective is a cell of the contested band or a middle choke
//! (`crate::eval::goal_zones`). Its value for a side is the number of valid
//! cells strictly nearer (steps over valid cells) to the side's active land
//! plus the cell than to the enemy's active land, less the same count with
//! the cell added to the enemy's land instead (the definition of the review
//! tool's `codingame-roles`).

use ice_and_fire_engine::{State, CELLS};
use ice_and_fire_opening::econ::distances;

use crate::eval::goal_zones;

/// The least value for an objective to count (the owner's default).
pub const OBJECTIVE_MIN_VALUE: i64 = 3;

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

/// The objectives of `state` for `seat`, by value, highest first, then by
/// cell.
pub fn objectives(state: &State, seat: usize) -> Vec<Objective> {
    let (band, chokes) = goal_zones(state);
    let (mine, theirs) = (state.active(seat), state.active(1 - seat));
    let (dm, dt) = (distances(state, mine), distances(state, theirs));
    let mut out = Vec::new();
    for cell in (band | chokes).iter() {
        let dg = distances(state, ice_and_fire_engine::Bits::cell(cell));
        out.push(Objective {
            cell,
            kind: if chokes.test(cell) {
                Kind::Choke
            } else {
                Kind::Band
            },
            held: mine.test(cell),
            value: nearer(state, &dm, &dg, &dt) - nearer(state, &dt, &dg, &dm),
        });
    }
    out.sort_by_key(|o| (-o.value, o.cell));
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

/// The objectives that count: of value at least [`OBJECTIVE_MIN_VALUE`].
pub fn counted(objectives: &[Objective]) -> Vec<Objective> {
    objectives
        .iter()
        .filter(|o| o.value >= OBJECTIVE_MIN_VALUE)
        .copied()
        .collect()
}
