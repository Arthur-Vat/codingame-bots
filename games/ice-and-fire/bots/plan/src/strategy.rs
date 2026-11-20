//! The strategic layer of the frontline plan
//! (`research/codingame/2026-11-20-frontline-plan.md`, Steps 2 and 3):
//! objectives with their values, the race to them, and the roles of our
//! units kept between turns. It only computes; nothing in the search reads
//! it yet, and [`STRATEGY`] is off, so the bot plays as before.

pub mod objectives;
pub mod race;
pub mod roles;
#[cfg(test)]
mod tests;

use ice_and_fire_engine::State;

pub use roles::Memory;
use roles::Roles;

/// Whether the bot computes objectives, races and roles each turn (Steps 2
/// and 3 change no behaviour: the results are not used yet).
pub const STRATEGY: bool = false;

/// One turn of the layer for the player to act: the objectives, the race
/// and the roles, the memory updated.
pub fn roles_of(state: &State, memory: &mut Memory) -> Roles {
    let seat = state.to_act();
    let objectives = objectives::objectives(state, seat);
    let races = race::race(state, seat, &objectives);
    roles::assign(state, seat, &objectives, &races, memory)
}
