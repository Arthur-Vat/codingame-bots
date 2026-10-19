//! A static evaluation of a state.
//!
//! The weights are placeholders, to be tuned by later experiments (with the
//! SPRT, one change at a time); they are not fitted to anything yet.

use ice_and_fire_engine::{Outcome, State, HQ};

use crate::chain::{Reach, UNREACHABLE};

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

/// How far `seat` is from affording the chain to the enemy HQ: 0 when its
/// gold and income pay for it, else minus the shortfall, at least
/// `-MAX_PRESSURE`.
fn pressure(state: &State, seat: usize) -> i64 {
    let cost = Reach::new(state, seat, false).cost[HQ[1 - seat]];
    if cost == UNREACHABLE {
        return -MAX_PRESSURE;
    }
    (state.gold(seat) + state.income(seat) - cost).clamp(-MAX_PRESSURE, 0)
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
    let other = 1 - seat;
    match state.outcome() {
        Some(Outcome::Win(winner)) => return if winner == seat { WIN } else { -WIN },
        Some(Outcome::Draw) => return 0,
        None => {}
    }
    let mut score = side(state, seat) - side(state, other);
    if seat == state.to_act() {
        let mut next = state.clone();
        next.end_turn();
        if next.outcome().is_none()
            && Reach::new(&next, other, true).cost[HQ[seat]] <= next.gold(other)
        {
            score -= WIN / 2;
        }
    }
    score + pressure(state, seat) - pressure(state, other)
}

#[cfg(test)]
mod tests;
