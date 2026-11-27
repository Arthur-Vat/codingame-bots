//! The sweeper (E097): a layer before the search for the units behind the
//! front. In v032's CodinGame games the top players keep a unit or two away
//! from the fight that take the free land far from it, while our search
//! spends its time where the score moves, near the front, and our units
//! behind it crowd or stand still (E096's measures). Valuing that land in
//! the evaluation lost (E096); the owner asked for a structural change.
//!
//! Each turn from game turn [`FROM`] to [`UNTIL`], every level-1 unit of
//! ours on an active cell more than [`DIST`] steps from the enemy's land is
//! matched to its own free cell (neutral, as far from the enemy's land), the
//! nearest pairs first, each cell and each unit used once, within [`RANGE`]
//! steps: no two units head for the same cell. Each matched unit takes one
//! step along a shortest path to its cell, preferring a neutral cell (taken
//! on the way) and never the cell another unit steps onto, so that units
//! spread over the free land instead of walking the same path (the owner's
//! collaborative snake).
//!
//! [`MODE`] says what the search does with these moves: none (v033), plays
//! them and plans the rest of the turn after them, or is offered them as
//! one macro action that it takes only when it scores better.

use ice_and_fire_engine::map::NONE;
use ice_and_fire_engine::{Bits, Command, State, SIZE};

/// What the search does with the sweeper's moves: 0 nothing (v033); 1 they
/// are played first and the search plans the rest of the turn; 2 they are
/// offered to the search as one macro action.
pub const MODE: u8 = 1;
/// A unit or a free cell more than this many steps from the enemy's land is
/// behind the front.
pub const DIST: u32 = 5;
/// The farthest free cell a unit is sent to, in steps.
pub const RANGE: u8 = 6;
/// The first game turn of the sweeper (the search's first, E067).
pub const FROM: u32 = 7;
/// The last game turn of the sweeper.
pub const UNTIL: u32 = 20;

/// The valid cells within `steps` of `cells`, `cells` included.
fn grown(state: &State, cells: Bits, steps: u32) -> Bits {
    let valid = state.map().valid;
    let mut out = cells & valid;
    for _ in 0..steps {
        out = (out | out.neighbours()) & valid;
    }
    out
}

/// The sweeper's moves for the player to act, none outside game turns
/// [`FROM`] to [`UNTIL`] or when [`MODE`] is 0.
pub fn sweep(state: &State) -> Vec<Command> {
    if MODE == 0 || !(FROM..=UNTIL).contains(&state.game_turn()) || state.outcome().is_some() {
        return Vec::new();
    }
    sweep_with(state, DIST, RANGE)
}

/// The sweeper's moves for the player to act with these settings, each
/// accepted in order by the engine.
pub fn sweep_with(state: &State, dist: u32, range: u8) -> Vec<Command> {
    let me = state.to_act();
    let map = state.map();
    let near = grown(state, state.owned(1 - me), dist);
    let neutral = map.valid.and_not(state.owned(0) | state.owned(1));
    let targets = neutral.and_not(near);
    let units: Vec<usize> = (state.movable_units() & state.active(me))
        .and_not(near)
        .iter()
        .filter(|&cell| state.unit_level(cell) == Some(1))
        .collect();
    let mut pairs = Vec::new();
    for &unit in &units {
        for target in targets.iter() {
            let d = map.distance(unit, target);
            if d != NONE && d >= 1 && d <= range {
                pairs.push((d, unit, target));
            }
        }
    }
    pairs.sort_unstable();
    let mut moved = Bits::EMPTY;
    let mut taken = Bits::EMPTY;
    let mut steps = Bits::EMPTY;
    let mut commands = Vec::new();
    for (d, unit, target) in pairs {
        if moved.test(unit) || taken.test(target) {
            continue;
        }
        // The step: a neighbour one step nearer the target, free of our
        // units and of other steps, a neutral one first.
        let mut best: Option<(bool, usize)> = None;
        for &next in map.neighbours[me][unit].iter().take_while(|&&n| n != NONE) {
            let next = usize::from(next);
            if map.distance(next, target) != d - 1
                || steps.test(next)
                || state.units(me).test(next)
                || state.owned(1 - me).test(next)
            {
                continue;
            }
            let free = neutral.test(next);
            if best.is_none_or(|(was_free, _)| free && !was_free) {
                best = Some((free, next));
            }
        }
        let (Some((_, next)), Some(id)) = (best, state.unit_id(unit)) else {
            continue;
        };
        moved.set(unit);
        taken.set(target);
        steps.set(next);
        commands.push(Command::Move {
            id,
            x: (next % SIZE) as u8,
            y: (next / SIZE) as u8,
        });
    }
    // Keep the moves the engine accepts, in order.
    let mut after = state.clone();
    commands.retain(|&command| after.apply(command));
    commands
}

#[cfg(test)]
mod tests;
