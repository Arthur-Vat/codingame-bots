//! When the middle game starts (E004): at the first start of a player's turn
//! whose gold pays a chain of trainings to an enemy cell, its units' moves
//! counted as free (`Reach::new(state, seat, true)`). From then on the two
//! sides can touch each other, and the opening is over.

use ice_and_fire_engine::State;
use ice_and_fire_tactics::chain::Reach;

/// Whether the player to act can pay, with its gold, a chain of trainings
/// that takes an enemy cell (a unit's step onto one costs nothing). False
/// once the game is over.
pub fn reached(state: &State) -> bool {
    if state.outcome().is_some() {
        return false;
    }
    let me = state.to_act();
    let gold = state.gold(me);
    let reach = Reach::new(state, me, true);
    state
        .owned(1 - me)
        .iter()
        .any(|cell| reach.cost[cell] <= gold)
}

#[cfg(test)]
mod tests;
