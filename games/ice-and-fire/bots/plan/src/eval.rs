//! The bot's evaluation of a state, from the point of view of `seat`.
//!
//! It starts as the shared static evaluation
//! (`ice_and_fire_tactics::eval::evaluate`: what each side has, the threat to
//! the HQ and how far each side is from affording the chain to the other's
//! HQ). Terms of the bot's own go here, each documented:
//!
//! - **Debt** (with `Config::debt`): a side whose gold plus income is below 0
//!   loses all its units at its next start of turn (RULES.md), so its units
//!   on active cells count as lost.

use ice_and_fire_engine::State;
use ice_and_fire_tactics::eval::evaluate;

use crate::opening::config;

/// The training cost of the units of `seat` on active cells, if it cannot
/// pay its upkeep, else 0.
fn debt(state: &State, seat: usize) -> i64 {
    if state.gold(seat) + state.income(seat) >= 0 {
        return 0;
    }
    (state.units(seat) & state.active(seat))
        .iter()
        .filter_map(|cell| state.unit_level(cell))
        .map(|level| 10 * i64::from(level))
        .sum()
}

/// The score of `state` for `seat`, higher is better for it.
pub fn eval(state: &State, seat: usize) -> i64 {
    let score = evaluate(state, seat);
    if config().debt && state.outcome().is_none() {
        score - 2 * debt(state, seat) + 2 * debt(state, 1 - seat)
    } else {
        score
    }
}
