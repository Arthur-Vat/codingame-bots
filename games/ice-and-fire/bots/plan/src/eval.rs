//! The bot's evaluation of a state, from the point of view of `seat`.
//!
//! It starts as the shared static evaluation
//! (`ice_and_fire_tactics::eval::evaluate`: what each side has, the threat to
//! the HQ and how far each side is from affording the chain to the other's
//! HQ). Terms of the bot's own go here, each documented:
//!
//! - **Debt** (E003): a side whose gold plus income is below 0 loses all its
//!   units at its next start of turn (RULES.md), so twice the training cost
//!   of its units on active cells is taken from its score: once for the
//!   units the shared evaluation counts, once for the loss.
//! - **Shape** (E010): a side's territory is weak where one capture cuts
//!   cells off from its HQ. For each side, [`SHAPE`] times the number of its
//!   joints (active cells whose loss cuts others off) and the most cells
//!   one loss cuts off (`ice_and_fire_opening::solo::severed`) is taken from
//!   its score, as the opening's plans do.
//! - **Level-2 units** (E020): a level-2 unit costs 4 gold of upkeep a turn
//!   against 1 for a level-1 one, and in v008's games a side with more of
//!   them than the score said won less often: each counts [`LEVEL2_DISCOUNT`]
//!   less than its training cost, 5 in all.
//! - **Threatened cells** (E023): in v009's games a cut the enemy could
//!   first afford with one more income was made within one turn about a
//!   quarter of the time, so a cell it can take at its next turn counts 0.75
//!   of a cell's value (4 as a cell, 3 as its income): [`threatened`] cells
//!   cost a side `7 / 4` each.

use ice_and_fire_engine::State;
use ice_and_fire_opening::solo::severed;
use ice_and_fire_tactics::chain::reach_within;
use ice_and_fire_tactics::eval::evaluate;

/// Whether [`threatened`] also counts the cells cut off by taking a
/// threatened cell (E023's second variant).
pub const CUT_OFFS: bool = true;

/// The active cells of `seat` that the enemy can take at its next turn:
/// those its chains of trainings reach for its gold plus its income (moves
/// left out, as in the pressure term), plus, with [`CUT_OFFS`], the most
/// cells that taking one of them cuts off from `seat`'s HQ.
pub fn threatened(state: &State, seat: usize) -> i64 {
    let enemy = 1 - seat;
    let budget = state.gold(enemy) + state.income(enemy);
    let cells = reach_within(state, enemy, false, budget) & state.active(seat);
    let mut count = i64::from(cells.count());
    if CUT_OFFS && count > 0 {
        let cut = severed(state, seat);
        count += cells.iter().map(|cell| cut[cell]).max().unwrap_or(0);
    }
    count
}

/// The weight of a joint and of a cell that one capture cuts off.
const SHAPE: i64 = 2;

/// The weakness of `seat`'s shape: its joints plus the most cells that one
/// capture cuts off.
fn weakness(state: &State, seat: usize) -> i64 {
    let cut = severed(state, seat);
    let mut joints = 0;
    let mut most = 0;
    for cell in state.active(seat).iter() {
        if cut[cell] > 0 {
            joints += 1;
            most = most.max(cut[cell]);
        }
    }
    joints + most
}

/// The shape term of `seat`'s score: [`SHAPE`] times the other side's
/// weakness less its own.
pub fn shape(state: &State, seat: usize) -> i64 {
    SHAPE * (weakness(state, 1 - seat) - weakness(state, seat))
}

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

/// How much less than its training cost a level-2 unit is worth (E020).
pub const LEVEL2_DISCOUNT: i64 = 15;

/// The level-2 units of `seat` on active cells.
fn twos(state: &State, seat: usize) -> i64 {
    (state.units(seat) & state.active(seat))
        .iter()
        .filter(|&cell| state.unit_level(cell) == Some(2))
        .count() as i64
}

/// The score of `state` for `seat`, higher is better for it.
pub fn eval(state: &State, seat: usize) -> i64 {
    let score = evaluate(state, seat);
    if state.outcome().is_some() {
        return score;
    }
    #[cfg(feature = "analysis")]
    let score = score - crate::analysis::removed(state, seat);
    score - 2 * debt(state, seat)
        + 2 * debt(state, 1 - seat)
        + shape(state, seat)
        + LEVEL2_DISCOUNT * (twos(state, 1 - seat) - twos(state, seat))
        + 7 * (threatened(state, 1 - seat) - threatened(state, seat)) / 4
}
