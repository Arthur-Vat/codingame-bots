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
//! - **Tower cap** (E024): a side's towers add to its score their static
//!   value (10 each when active) and their gain in the pressure term (how
//!   much they raise the enemy's cheapest chain to the side's HQ). What
//!   they add beyond [`TOWER_CAP`] a tower, a level-3 training's extra cost
//!   over level 1, is taken off ([`over_cap`]).

use ice_and_fire_engine::{Bits, State, HQ};
use ice_and_fire_opening::solo::severed;
use ice_and_fire_tactics::chain::{cost_to, cost_to_with};
use ice_and_fire_tactics::eval::{evaluate, pressure_for};

/// The most a tower may add to its side's score (E024).
pub const TOWER_CAP: i64 = 20;

/// What the towers of `seat` add to its score beyond [`TOWER_CAP`] each:
/// their static value (10 an active tower, in the shared evaluation) plus
/// the enemy's pressure term with the towers' protection less the same
/// with none of `seat`'s towers (the tower cells then enterable at level 1
/// like any other cell of `seat` without a unit), over `TOWER_CAP` times
/// the towers of `seat`, at least 0.
pub fn over_cap(state: &State, seat: usize) -> i64 {
    let enemy = 1 - seat;
    let towers = state.towers() & state.owned(seat);
    if towers.is_empty() {
        return 0;
    }
    let active = state.active(seat);
    // The cells the towers protect from levels 1 and 2 (RULES.md,
    // "Protection"), and on them the units of `seat` by level.
    let blocked = towers | (active & (active & towers).neighbours());
    let mut stronger = [Bits::EMPTY; 2];
    for cell in (blocked & state.units(seat)).iter() {
        let level = state.unit_level(cell).unwrap_or(0);
        for l in 1..=2u8 {
            if level >= l {
                stronger[usize::from(l) - 1] |= Bits::cell(cell);
            }
        }
    }
    let open = blocked & state.enterable(enemy, 3);
    let without = cost_to_with(state, enemy, false, HQ[seat], |level| {
        let bits = state.enterable(enemy, level);
        if level < 3 {
            bits | open.and_not(stronger[usize::from(level) - 1])
        } else {
            bits
        }
    });
    let with = cost_to(state, enemy, false, HQ[seat]);
    let gain = pressure_for(state, enemy, without) - pressure_for(state, enemy, with);
    let value = 10 * i64::from((towers & active).count()) + gain;
    (value - TOWER_CAP * i64::from(towers.count())).max(0)
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
        - over_cap(state, seat)
        + over_cap(state, 1 - seat)
}
