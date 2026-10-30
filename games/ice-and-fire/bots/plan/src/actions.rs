//! The actions of the search: what can be added to a plan in a state.
//!
//! Single commands: each move of a movable unit to a neighbouring cell it may
//! enter, each training at the lowest level that may enter a cell (so a
//! higher level only where it kills a unit or passes a tower), each tower on a
//! border cell that guards something (E026, [`guards`]) and each mine on a
//! mine spot. Macro actions: the best few cuts
//! of the enemy's territory and the lethal chain of trainings, if there is
//! one. Every action is checked by applying it on a clone, so a list may hold
//! some that the rules refuse.

use ice_and_fire_engine::state::BUILD_COST;
use ice_and_fire_engine::{Bits, Command, State, HQ, SIZE};
use ice_and_fire_tactics::chain::{chain_to, cost_to};
use ice_and_fire_tactics::cut::cuts;

use crate::eval::threatened;

/// Whether a tower that raises the enemy's chain to our HQ, while the enemy
/// can pay for it within two of its turns, is offered too (E026's second
/// variant).
pub const HQ_GUARD: bool = true;

/// Whether a tower on `cell` guards something for the player to act (E026):
/// it lowers the cells of ours the enemy can take at its next turn
/// ([`threatened`]), or, with [`HQ_GUARD`], it raises the enemy's chain to
/// our HQ while that chain costs at most the enemy's gold plus two of its
/// incomes. `threats` is `threatened(state, me)` and `chain` the enemy's
/// chain to our HQ, before the tower.
fn guards(state: &State, cell: usize, threats: i64, chain: i64) -> bool {
    let me = state.to_act();
    let enemy = 1 - me;
    let (x, y) = xy(cell);
    let mut after = state.clone();
    if !after.apply(Command::BuildTower { x, y }) {
        return false;
    }
    if threatened(&after, me) < threats {
        return true;
    }
    HQ_GUARD
        && chain <= state.gold(enemy) + 2 * state.income(enemy).max(0)
        && cost_to(&after, enemy, false, HQ[me]) > chain
}

/// Cuts offered as macro actions.
pub const MACRO_CUTS: usize = 4;
/// Longest cut, in trainings: 5 since E014 (4 before).
pub const CUT_LENGTH: usize = 5;

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

pub fn xy(cell: usize) -> (u8, u8) {
    ((cell % SIZE) as u8, (cell / SIZE) as u8)
}

/// The actions for the player to act in `state`, in `out` (cleared first), in
/// a fixed order.
pub fn generate(state: &State, out: &mut Vec<Action>) {
    out.clear();
    if state.outcome().is_some() {
        return;
    }
    let me = state.to_act();
    let max = state.league().max_level();
    let mut enter = [Bits::EMPTY; 4];
    for level in 1..=max {
        enter[usize::from(level)] = state.enterable(me, level);
    }

    // Moves.
    for from in state.movable_units().iter() {
        let Some((id, level)) = state.unit_at(from) else {
            continue;
        };
        let steps = Bits::cell(from).neighbours() & enter[usize::from(level)];
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
        let mut cells = near & enter[usize::from(level)];
        for lower in 1..level {
            cells = cells.and_not(enter[usize::from(lower)]);
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
            let threats = threatened(state, me);
            let chain = cost_to(state, 1 - me, false, HQ[me]);
            for cell in cells
                .iter()
                .filter(|&cell| guards(state, cell, threats, chain))
            {
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
}
