//! The map's zones and a side's territory measures.

use ice_and_fire_engine::{Bits, State, CELLS, HQ};

/// The share of the shortest HQ-to-HQ paths a cell must carry to be a choke
/// cell (as in the zasmu study: 40%).
pub const CHOKE_SHARE: f64 = 0.4;

/// The middle band: cells at most this many steps from even between the two
/// HQs; its choke cells are the contested ones (the others crowd around the
/// HQs, where every shortest path starts or ends).
pub const MIDDLE: u8 = 4;

/// The zones of a map, which depend on its void cells only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Zones {
    /// Cells that at least [`CHOKE_SHARE`] of the shortest HQ-to-HQ paths go
    /// through, the HQs left out.
    pub chokes: Bits,
    /// Cells within one step of even between the HQs.
    pub centre: Bits,
    /// Cells at most [`MIDDLE`] steps from even between the HQs.
    pub middle: Bits,
}

impl Zones {
    /// The zones of `state`'s map.
    pub fn of(state: &State) -> Zones {
        let (a, da) = paths_from(state, HQ[0]);
        let (b, db) = paths_from(state, HQ[1]);
        let total = a[HQ[1]];
        let length = da[HQ[1]];
        let mut zones = Zones {
            chokes: Bits::EMPTY,
            centre: Bits::EMPTY,
            middle: Bits::EMPTY,
        };
        for cell in state.map().valid.iter() {
            if da[cell] == u8::MAX || db[cell] == u8::MAX {
                continue;
            }
            if da[cell].abs_diff(db[cell]) <= 1 {
                zones.centre.set(cell);
            }
            if da[cell].abs_diff(db[cell]) <= MIDDLE {
                zones.middle.set(cell);
            }
            let on_path = u16::from(da[cell]) + u16::from(db[cell]) == u16::from(length);
            if total > 0.0
                && on_path
                && cell != HQ[0]
                && cell != HQ[1]
                && a[cell] * b[cell] / total >= CHOKE_SHARE
            {
                zones.chokes.set(cell);
            }
        }
        zones
    }

    /// The choke cells of the middle band, the contested ones.
    pub fn middle_chokes(&self) -> Bits {
        self.chokes & self.middle
    }
}

/// Steps from `sources` to each valid cell, `u8::MAX` where none reaches.
pub fn steps_from(state: &State, sources: Bits) -> [u8; CELLS] {
    let valid = state.map().valid;
    let mut out = [u8::MAX; CELLS];
    let mut reached = sources & valid;
    let mut layer = reached;
    let mut step = 0u8;
    while !layer.is_empty() {
        for cell in layer.iter() {
            out[cell] = step;
        }
        let next = (layer.neighbours() & valid).and_not(reached);
        reached |= next;
        layer = next;
        step = step.saturating_add(1);
    }
    out
}

/// The number of shortest paths from `from` to each cell (as `f64`, which
/// does not overflow), and the steps.
fn paths_from(state: &State, from: usize) -> ([f64; CELLS], [u8; CELLS]) {
    let dist = steps_from(state, Bits::cell(from));
    let mut order: Vec<usize> = state
        .map()
        .valid
        .iter()
        .filter(|&c| dist[c] != u8::MAX)
        .collect();
    order.sort_by_key(|&c| dist[c]);
    let mut count = [0.0; CELLS];
    count[from] = 1.0;
    for cell in order {
        if cell == from {
            continue;
        }
        let around = Bits::cell(cell).neighbours() & state.map().valid;
        count[cell] = around
            .iter()
            .filter(|&n| dist[n] != u8::MAX && dist[n] + 1 == dist[cell])
            .map(|n| count[n])
            .sum();
    }
    (count, dist)
}

/// The share of the cells nearer `seat`'s land than the enemy's, over the
/// cells nearer one side.
pub fn voronoi(state: &State, seat: usize) -> f64 {
    let ours = steps_from(state, state.owned(seat));
    let theirs = steps_from(state, state.owned(1 - seat));
    let (mut near_us, mut near_them) = (0u32, 0u32);
    for cell in state.map().valid.iter() {
        match ours[cell].cmp(&theirs[cell]) {
            std::cmp::Ordering::Less => near_us += 1,
            std::cmp::Ordering::Greater => near_them += 1,
            std::cmp::Ordering::Equal => {}
        }
    }
    let all = near_us + near_them;
    if all == 0 {
        0.0
    } else {
        f64::from(near_us) / f64::from(all)
    }
}

/// The share of `zone` that `seat` owns, 0 for an empty zone.
pub fn share(state: &State, seat: usize, zone: Bits) -> f64 {
    if zone.is_empty() {
        0.0
    } else {
        f64::from((state.owned(seat) & zone).count()) / f64::from(zone.count())
    }
}

/// The fewest steps from `seat`'s cells to a centre cell (0 when it holds
/// one, `u8::MAX` when none is reachable).
pub fn to_centre(state: &State, seat: usize, zones: &Zones) -> u8 {
    let d = steps_from(state, state.owned(seat));
    zones.centre.iter().map(|c| d[c]).min().unwrap_or(u8::MAX)
}

#[cfg(test)]
mod tests;
