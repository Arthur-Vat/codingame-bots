//! The strategic turn (Step 4 of the frontline plan, E077): one plan made of
//! the frontline units' steps toward their objectives, then trainings at
//! their tips within the gold, then (with [`super::TRAININGS`] at 2) the
//! trainings the roles give the objectives no unit covers. The back-line
//! units are left to the search, which goes on climbing from the plan.
//!
//! Every command is checked on a scratch copy, in order, so the plan applies
//! whole on the state it was made for.

use ice_and_fire_engine::{Bits, Command, State};

use super::roles::Roles;
use crate::actions::xy;

/// The gold of a level-1 unit.
const TRAIN_COST: i64 = 10;

/// The cell of the unit `id` of `seat`, if it lives.
fn cell_of(state: &State, seat: usize, id: u32) -> Option<usize> {
    state
        .units(seat)
        .iter()
        .find(|&c| state.unit_id(c) == Some(id))
}

/// The cells next to `from` one step nearer to `goal`, the cells not ours
/// first (a step that takes land), then by cell.
fn steps_toward(state: &State, seat: usize, from: usize, goal: usize) -> Vec<usize> {
    let map = state.map();
    let here = map.distance(from, goal);
    if here == u8::MAX || here == 0 {
        return Vec::new();
    }
    let mut out: Vec<usize> = (Bits::cell(from).neighbours() & map.valid)
        .iter()
        .filter(|&n| map.distance(n, goal) < here)
        .collect();
    out.sort_by_key(|&n| (state.owned(seat).test(n), n));
    out
}

/// Applies `command` on `scratch` and adds it to `out` if the rules accept
/// it; whether they did.
fn accepted(scratch: &mut State, out: &mut Vec<Command>, command: Command) -> bool {
    let ok = scratch.apply(command);
    if ok {
        out.push(command);
    }
    ok
}

/// The commands of the strategic turn of the player to act in `state`, with
/// `roles` its roles this turn; with `role_trainings`, also the trainings the
/// roles give the objectives no frontline unit covers.
pub fn strategic_turn(state: &State, roles: &Roles, role_trainings: bool) -> Vec<Command> {
    let seat = state.to_act();
    let map = state.map();
    let mut scratch = state.clone();
    let mut out = Vec::new();

    // 1. The frontline units step toward their objectives.
    for &(id, goal) in &roles.frontline {
        let Some(from) = cell_of(&scratch, seat, id) else {
            continue;
        };
        if !scratch.movable_units().test(from) {
            continue;
        }
        for to in steps_toward(&scratch, seat, from, goal) {
            let (x, y) = xy(to);
            if accepted(&mut scratch, &mut out, Command::Move { id, x, y }) {
                break;
            }
        }
    }

    // 2. A training at the tip of each frontline unit: on a cell next to it
    // nearer its objective, the nearest first.
    for &(id, goal) in &roles.frontline {
        if scratch.gold(seat) < TRAIN_COST {
            break;
        }
        let Some(at) = cell_of(&scratch, seat, id) else {
            continue;
        };
        for to in steps_toward(&scratch, seat, at, goal) {
            let (x, y) = xy(to);
            if accepted(&mut scratch, &mut out, Command::Train { level: 1, x, y }) {
                break;
            }
        }
    }

    // 3. The objectives no unit covers: a training at our trainable cell
    // nearest each, after the moves.
    if role_trainings {
        for &(_, goal) in &roles.trainings {
            if scratch.gold(seat) < TRAIN_COST {
                break;
            }
            let trainable = scratch.active(seat).neighbours() & map.valid;
            let mut cells: Vec<usize> = trainable
                .iter()
                .filter(|&c| map.distance(c, goal) != u8::MAX)
                .collect();
            cells.sort_by_key(|&c| (map.distance(c, goal), c));
            for to in cells {
                let (x, y) = xy(to);
                if accepted(&mut scratch, &mut out, Command::Train { level: 1, x, y }) {
                    break;
                }
            }
        }
    }
    out
}
