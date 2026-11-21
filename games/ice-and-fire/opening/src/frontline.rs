//! The frontline plan's strategic layer
//! (`games/ice-and-fire/research/codingame/2026-11-20-frontline-plan.md`):
//! the objectives of a map with the land each decides, the race of both
//! sides to them, and the roles of our units kept between turns (Steps 2
//! and 3, moved here from the plan bot), and the opening by roles
//! ([`step`], Step 7, E080).

pub mod objectives;
pub mod race;
pub mod roles;
pub mod step;

use ice_and_fire_engine::{Bits, State};

use crate::solo::{middle_chokes, rings};

/// The band's width: cells within this many rings of the line even between
/// the HQs (the plan bot's `eval::GOAL_BAND`).
pub const BAND: i64 = 2;

/// The map's band cells and middle chokes, computed once a map.
struct Zones {
    valid: Bits,
    band: Bits,
    chokes: Bits,
}

thread_local! {
    static ZONES: std::cell::RefCell<Option<Zones>> = const { std::cell::RefCell::new(None) };
}

/// The band cells (within [`BAND`] rings of the middle) and the middle
/// chokes of `state`'s map, as the plan bot's `eval::goal_zones` has them.
pub fn goal_zones(state: &State) -> (Bits, Bits) {
    ZONES.with(|cell| {
        let mut zones = cell.borrow_mut();
        let valid = state.map().valid;
        if zones.as_ref().is_none_or(|z| z.valid != valid) {
            let ring = rings(state);
            let mut band = Bits::EMPTY;
            for c in valid.iter() {
                if ring[c] <= BAND {
                    band.set(c);
                }
            }
            *zones = Some(Zones {
                valid,
                band,
                chokes: middle_chokes(state),
            });
        }
        zones
            .as_ref()
            .map_or((Bits::EMPTY, Bits::EMPTY), |z| (z.band, z.chokes))
    })
}
