//! A bank of middle-game positions drawn from recorded games, for the plan
//! bot's analysis mode (`ice-and-fire-bot-plan --features analysis -- bank`).
//!
//! A position is the start of a ply in the middle game as the trace cuts it:
//! from its first ply (the player to act can pay a chain of trainings to an
//! enemy cell, `ice_and_fire_opening::middle`) up to, not including, the
//! first ply of the endgame (one side has twice the other's active cells, or
//! can pay its cheapest chain to the enemy HQ with its gold plus one turn of
//! income). The positions are drawn at random without replacement, with a
//! seed, among those of all the records, then sorted by record and ply, so
//! the same records and seed give the same bank.
//!
//! The bank is text, one block a position, as the player to act receives it:
//!
//! ```text
//! position NAME PLY GAME_TURN
//! <the initial input>
//! turn
//! <the turn's input>
//! end
//! ```

use cg_core::rng::Rng;
use ice_and_fire_engine::State;
use ice_and_fire_opening::middle;

use crate::replay::Game;
use crate::trace::{hq_cost, NO_CHAIN};

/// Active cells, and income as the trace defines it: active cells, plus 4 a
/// mine on an active cell, less the upkeep of the units on active cells.
fn cells_income(state: &State, seat: usize) -> (i64, i64) {
    let active = state.active(seat);
    let mut upkeep = 0;
    for cell in (state.units(seat) & active).iter() {
        if let Some(level) = state.unit_level(cell) {
            upkeep += [1, 4, 20][usize::from(level) - 1];
        }
    }
    let cells = i64::from(active.count());
    (
        cells,
        cells + 4 * i64::from((state.mines() & active).count()) - upkeep,
    )
}

/// Whether `state` is in the endgame by the trace's rule.
fn endgame(state: &State) -> bool {
    let sides = [cells_income(state, 0), cells_income(state, 1)];
    (0..2).any(|side| {
        let (cells, income) = sides[side];
        let twice = cells >= 2 * sides[1 - side].0;
        let gap = hq_cost(state, side).map_or(NO_CHAIN, |cost| (cost - state.gold(side)).max(0));
        twice || (gap != NO_CHAIN && gap <= income.max(0))
    })
}

/// The middle game's plies of `game`: from the first to the last before the
/// endgame, empty when the game never reaches it.
pub fn middle_plies(game: &Game) -> std::ops::Range<usize> {
    let steps = &game.steps;
    let Some(from) = steps.iter().position(|s| middle::reached(&s.state)) else {
        return 0..0;
    };
    let to = (from..steps.len())
        .find(|&ply| endgame(&steps[ply].state))
        .unwrap_or(steps.len());
    from..to
}

/// Draws `count` of `candidates` (record index, ply) with `seed`, without
/// replacement, sorted.
pub fn draw(mut candidates: Vec<(usize, usize)>, count: usize, seed: u64) -> Vec<(usize, usize)> {
    let mut rng = Rng::new(seed);
    rng.shuffle(&mut candidates);
    candidates.truncate(count);
    candidates.sort_unstable();
    candidates
}

/// The block of the position at the start of `ply` of `game`.
pub fn block(game: &Game, ply: usize) -> String {
    let state = &game.steps[ply].state;
    let seat = state.to_act();
    format!(
        "position {} {ply} {}\n{}turn\n{}end\n",
        game.name,
        state.game_turn(),
        state.initial_input(seat),
        state.turn_input(seat)
    )
}

#[cfg(test)]
mod tests;
