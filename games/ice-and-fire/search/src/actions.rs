//! The actions of the search: what can be added to a plan in a state.
//!
//! Single commands: each move of a movable unit to a neighbouring cell it may
//! enter, each training at the lowest level that may enter a cell (so a
//! higher level only where it kills a unit or passes a tower), each tower on a
//! border cell and each mine on a mine spot. Macro actions: the best few cuts
//! of the enemy's territory and the lethal chain of trainings, if there is
//! one; and bypasses (E006): two level 1 trainings that join the cells beyond
//! one of our joints at the start of the turn ([`Necks`]) to the rest of our
//! territory by a second path. Every action is checked by applying it on a clone, so a list may hold
//! some that the rules refuse.

use ice_and_fire_engine::state::BUILD_COST;
use ice_and_fire_engine::{Bits, Command, State, HQ, SIZE};
use ice_and_fire_tactics::chain::chain_to;
use ice_and_fire_tactics::cut::cuts;

use crate::eval::Necks;

/// Cuts offered as macro actions.
pub const MACRO_CUTS: usize = 4;
/// Longest cut, in trainings.
pub const CUT_LENGTH: usize = 4;
/// Joints, the most valuable, for which bypasses are offered.
pub const BYPASS_JOINTS: usize = 2;
/// Bypasses offered for one joint.
pub const BYPASSES: usize = 4;

/// One step of a plan: a command, or a short chain of commands that go
/// together.
pub enum Action {
    One(Command),
    Many(Vec<Command>),
}

impl Action {
    /// The commands, in the order they are applied.
    pub fn commands(&self) -> &[Command] {
        match self {
            Action::One(command) => std::slice::from_ref(command),
            Action::Many(commands) => commands,
        }
    }
}

fn xy(cell: usize) -> (u8, u8) {
    ((cell % SIZE) as u8, (cell / SIZE) as u8)
}

/// The cells of `within` joined to `from` through `within`.
fn flood(from: Bits, within: Bits) -> Bits {
    let mut seen = from & within;
    loop {
        let next = (seen | seen.neighbours()) & within;
        if next == seen {
            return seen;
        }
        seen = next;
    }
}

/// Bypasses of the joints of `necks` that are still joints of our territory
/// in `state`: two level 1 trainings, the first next to the cells joined to
/// the HQ without the joint, the second next to the first and to the cells
/// beyond the joint. A single training that does it is a training already.
fn bypasses(state: &State, necks: &Necks, enter: Bits, out: &mut Vec<Action>) {
    let me = necks.seat;
    if state.to_act() != me || state.gold(me) < 20 {
        return;
    }
    let active = state.active(me);
    let free = enter.and_not(state.owned(me));
    for &(joint, _) in necks.joints.iter().take(BYPASS_JOINTS) {
        if !active.test(joint) {
            continue;
        }
        let rest = active.and_not(Bits::cell(joint));
        let near = flood(Bits::cell(HQ[me]), rest);
        let beyond = rest.and_not(near);
        if beyond.is_empty() {
            continue;
        }
        let mut found = 0;
        for first in (free & near.neighbours())
            .and_not(beyond.neighbours())
            .iter()
        {
            let seconds = free & beyond.neighbours() & Bits::cell(first).neighbours();
            for second in seconds.iter() {
                let ((x1, y1), (x2, y2)) = (xy(first), xy(second));
                out.push(Action::Many(vec![
                    Command::Train {
                        level: 1,
                        x: x1,
                        y: y1,
                    },
                    Command::Train {
                        level: 1,
                        x: x2,
                        y: y2,
                    },
                ]));
                found += 1;
                if found == BYPASSES {
                    break;
                }
            }
            if found == BYPASSES {
                break;
            }
        }
    }
}

/// The actions for the player to act in `state`, in `out` (cleared first), in
/// a fixed order. `necks` are the joints of our territory at the start of
/// the turn, for the bypasses.
pub fn generate(state: &State, necks: &Necks, out: &mut Vec<Action>) {
    out.clear();
    if state.outcome().is_some() {
        return;
    }
    let me = state.to_act();
    let max = state.league().max_level();
    let mut enter = [Bits::EMPTY; 3];
    for level in 1..=max {
        enter[usize::from(level) - 1] = state.enterable(me, level);
    }

    // Moves.
    for from in state.movable_units().iter() {
        let Some((id, level)) = state.unit_at(from) else {
            continue;
        };
        let steps = Bits::cell(from).neighbours() & enter[usize::from(level) - 1];
        for to in steps.iter() {
            let (x, y) = xy(to);
            out.push(Action::One(Command::Move { id, x, y }));
        }
    }

    // Trainings, at the lowest level that may enter the cell.
    let gold = state.gold(me);
    let near = state.near_active(me);
    for level in 1..=max {
        let cost = 10 * i64::from(level);
        if gold < cost {
            break;
        }
        let mut cells = near & enter[usize::from(level) - 1];
        for lower in 1..level {
            cells = cells.and_not(enter[usize::from(lower) - 1]);
        }
        for cell in cells.iter() {
            let (x, y) = xy(cell);
            out.push(Action::One(Command::Train { level, x, y }));
        }
    }

    // Buildings.
    if gold >= BUILD_COST {
        let free = state.owned(me) & near & state.empty_cells();
        if state.league().allows_mines() {
            for cell in (free & state.map().spots).iter() {
                let (x, y) = xy(cell);
                out.push(Action::One(Command::BuildMine { x, y }));
            }
        }
        if state.league().allows_towers() {
            // Towers only on cells next to a cell that is not ours.
            let border = state.map().valid.and_not(state.owned(me)).neighbours();
            let cells = (free & border).and_not(state.map().spots);
            for cell in cells.iter() {
                let (x, y) = xy(cell);
                out.push(Action::One(Command::BuildTower { x, y }));
            }
        }
    }

    // Macro actions: the best cuts, and the lethal chain. A cut of one
    // training is a single training already.
    for cut in cuts(state, CUT_LENGTH, gold, MACRO_CUTS) {
        if cut.commands.len() > 1 {
            out.push(Action::Many(cut.commands));
        }
    }
    if let Some(chain) = chain_to(state, HQ[1 - me], gold) {
        if chain.commands.len() > 1 {
            out.push(Action::Many(chain.commands));
        }
    }
    bypasses(state, necks, enter[0], out);
}

#[cfg(test)]
mod tests;
