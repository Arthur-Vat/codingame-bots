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
//! - **Discounted pressure** (E021): the shared evaluation's pressure term
//!   counted a threat to an HQ almost the same whether its side could pay
//!   for it next turn or many turns later. [`pressure`] replaces it: minus
//!   [`PRESSURE`] times `1 − DECAY^t`, `t` the turns of income beyond the
//!   next one that the side still lacks.

use ice_and_fire_engine::{State, HQ};
use ice_and_fire_opening::solo::severed;
use ice_and_fire_tactics::chain::{cost_to, UNREACHABLE};
use ice_and_fire_tactics::eval::evaluate_with;

/// The most a side's pressure term takes (E021).
pub const PRESSURE: f64 = 100.0;
/// How much a threat's weight falls with each turn it is further away: the
/// rate at which v009's threats were carried out within 5 turns
/// (`research/towers-and-threats.md`).
pub const DECAY: f64 = 0.42;

/// The pressure term of `seat` (E021): `−PRESSURE × (1 − DECAY^t)`, where
/// `t = max(0, cost − gold − income) / max(income, 1)` is how many turns of
/// income beyond its next one `seat` lacks to pay for its cheapest chain to
/// the enemy HQ (fractional, 0 when it can pay at its next turn), rounded to
/// the nearest integer; `−PRESSURE` when the HQ is unreachable.
pub fn pressure(state: &State, seat: usize) -> i64 {
    let cost = cost_to(state, seat, false, HQ[1 - seat]);
    if cost == UNREACHABLE {
        return -(PRESSURE as i64);
    }
    let income = state.income(seat);
    let short = (cost - state.gold(seat) - income).max(0);
    let t = short as f64 / income.max(1) as f64;
    (-PRESSURE * (1.0 - DECAY.powf(t))).round() as i64
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
    let score = evaluate_with(state, seat, pressure);
    if state.outcome().is_some() {
        return score;
    }
    #[cfg(feature = "analysis")]
    let score = score - crate::analysis::removed(state, seat);
    score - 2 * debt(state, seat)
        + 2 * debt(state, 1 - seat)
        + shape(state, seat)
        + LEVEL2_DISCOUNT * (twos(state, 1 - seat) - twos(state, seat))
}
