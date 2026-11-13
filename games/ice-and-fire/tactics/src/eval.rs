//! A static evaluation of a state.
//!
//! The weights are placeholders, to be tuned by later experiments (with the
//! SPRT, one change at a time); they are not fitted to anything yet.

use ice_and_fire_engine::{Outcome, State, HQ};

use crate::chain::{cost_to, UNREACHABLE};

/// Score of a won or lost game.
pub const WIN: i64 = 1_000_000;

/// The lowest pressure term of one side.
const MAX_PRESSURE: i64 = 200;

/// What a side has: gold, income, active cells, units on active cells (by
/// training cost) and its active towers.
fn side(state: &State, seat: usize) -> i64 {
    let active = state.active(seat);
    let units: i64 = (state.units(seat) & active)
        .iter()
        .filter_map(|cell| state.unit_level(cell))
        .map(|level| 10 * i64::from(level))
        .sum();
    state.gold(seat)
        + 3 * state.income(seat)
        + 4 * i64::from(active.count())
        + units
        + 10 * i64::from((state.towers() & active).count())
}

/// What `seat`'s gold and income leave once it has paid for its cheapest
/// chain of trainings to the enemy HQ: negative when it cannot pay,
/// `i64::MIN` when no chain reaches the HQ.
pub fn margin(state: &State, seat: usize) -> i64 {
    let cost = cost_to(state, seat, false, HQ[1 - seat]);
    if cost == UNREACHABLE {
        return i64::MIN;
    }
    state.gold(seat) + state.income(seat) - cost
}

/// The pressure term of a side whose [`margin`] is `margin`: 0 when it pays
/// for the chain to the enemy HQ, else minus the shortfall, at least
/// `-MAX_PRESSURE`.
fn pressure(margin: i64) -> i64 {
    margin.clamp(-MAX_PRESSURE, 0)
}

/// A static score of `state` for `seat`, higher is better for it.
///
/// A finished game is [`WIN`], `-WIN` or 0. Else the score is the difference
/// of what each side has (gold, 3 × income, 4 × active cells, the cost of its
/// units on active cells, 10 × its active towers), minus `WIN / 2` when the
/// seat is to act and the opponent, after its start of turn, could pay for a
/// chain of trainings (or a step of a unit) that takes the seat's HQ, plus
/// the difference of the sides' pressure (how far each is from affording the
/// chain to the other's HQ, at most 200). Without the threat term the score
/// of one seat is the other's negated.
pub fn evaluate(state: &State, seat: usize) -> i64 {
    evaluate_with_margins(state, seat).0
}

/// [`evaluate`], and the [`margin`] of `seat` and of the other seat, as the
/// pressure term found them (both 0 for a finished game).
pub fn evaluate_with_margins(state: &State, seat: usize) -> (i64, [i64; 2]) {
    let other = 1 - seat;
    match state.outcome() {
        Some(Outcome::Win(winner)) => return (if winner == seat { WIN } else { -WIN }, [0, 0]),
        Some(Outcome::Draw) => return (0, [0, 0]),
        None => {}
    }
    let mut score = side(state, seat) - side(state, other);
    if seat == state.to_act() {
        let mut next = state.clone();
        next.end_turn();
        if next.outcome().is_none() && cost_to(&next, other, true, HQ[seat]) <= next.gold(other) {
            score -= WIN / 2;
        }
    }
    let margins = [margin(state, seat), margin(state, other)];
    (score + pressure(margins[0]) - pressure(margins[1]), margins)
}

#[cfg(test)]
mod tests;
