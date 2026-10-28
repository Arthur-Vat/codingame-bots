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

use ice_and_fire_engine::{Outcome, State, HQ};
use ice_and_fire_opening::solo::severed;
use ice_and_fire_tactics::chain::{cost_to, UNREACHABLE};
use ice_and_fire_tactics::eval::{evaluate, WIN};

/// E017 screening only: weights (gold = 10) of the terms, own less the
/// opponent's: gold, income, active cells, units' cost, towers, mines,
/// joints, the most cells one capture cuts off; then own and opponent's
/// pressure, own and opponent's debt. From the build's environment.
fn weights() -> Option<[i64; 12]> {
    match option_env!("E017") {
        Some("fit") => Some([10, 67, 62, 7, 254, 0, -68, -16, 10, -9, -8, 0]),
        Some("half") => Some([10, 48, 51, 8, 177, 0, -44, -18, 10, -9, -14, 10]),
        Some("fitmines") => Some([10, 43, 79, 4, 252, 435, -72, -13, 11, -9, -10, 4]),
        _ => None,
    }
}

/// The terms of `seat`: gold, income, active cells, units' cost on them,
/// active towers, active mines, joints, the most cells one capture cuts off,
/// pressure (from -200 to 0) and debt.
fn terms(state: &State, seat: usize) -> [i64; 10] {
    let active = state.active(seat);
    let units: i64 = (state.units(seat) & active)
        .iter()
        .filter_map(|cell| state.unit_level(cell))
        .map(|level| 10 * i64::from(level))
        .sum();
    let cost = cost_to(state, seat, false, HQ[1 - seat]);
    let pressure = if cost == UNREACHABLE {
        -200
    } else {
        (state.gold(seat) + state.income(seat) - cost).clamp(-200, 0)
    };
    let cut = severed(state, seat);
    let (mut joints, mut most) = (0, 0);
    for cell in active.iter() {
        if cut[cell] > 0 {
            joints += 1;
            most = most.max(cut[cell]);
        }
    }
    [
        state.gold(seat),
        state.income(seat),
        i64::from(active.count()),
        units,
        i64::from((state.towers() & active).count()),
        i64::from((state.mines() & active).count()),
        joints,
        most,
        pressure,
        debt(state, seat),
    ]
}

/// The evaluation with the weights `w` (E017), the threat to the HQ as in
/// the shared evaluation.
fn weighted(state: &State, seat: usize, w: &[i64; 12]) -> i64 {
    let other = 1 - seat;
    match state.outcome() {
        Some(Outcome::Win(winner)) => return if winner == seat { WIN } else { -WIN },
        Some(Outcome::Draw) => return 0,
        None => {}
    }
    let (a, b) = (terms(state, seat), terms(state, other));
    let mut score: i64 = (0..8).map(|i| w[i] * (a[i] - b[i])).sum();
    score += w[8] * a[8] + w[9] * b[8] + w[10] * a[9] + w[11] * b[9];
    if seat == state.to_act() {
        let mut next = state.clone();
        next.end_turn();
        if next.outcome().is_none() && cost_to(&next, other, true, HQ[seat]) <= next.gold(other) {
            score -= WIN / 2;
        }
    }
    score
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

/// The score of `state` for `seat`, higher is better for it.
pub fn eval(state: &State, seat: usize) -> i64 {
    if let Some(w) = weights() {
        return weighted(state, seat, &w);
    }
    let score = evaluate(state, seat);
    if state.outcome().is_some() {
        return score;
    }
    score - 2 * debt(state, seat) + 2 * debt(state, 1 - seat) + shape(state, seat)
}
