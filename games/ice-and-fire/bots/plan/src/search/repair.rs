//! E091's repair of a plan against its worst reply (the owner, 2026-11-26:
//! our turn, the opponent's reply, then our turn changed to counter it, and
//! so on). After a climb's plan has had its replies searched, the plan
//! itself is changed, a step at a time, by the best of: dropping one of its
//! commands, adding a counter to its worst reply (a tower on or next to the
//! cells the reply enters, a unit on such a cell), or adding any action
//! offered; each step must raise the plan's score against the memory, which
//! holds that reply. The repaired plan then gets its own replies searched
//! (`think_with`), so the next repair meets the replies the change opens.

use ice_and_fire_engine::{Bits, Command, State, SIZE};

use super::{apply_all, Reply, Search};
use crate::actions::{generate, xy, Action};

/// The cells the commands of `reply` move a unit onto or train on.
pub(super) fn entered(reply: &[Command]) -> Bits {
    let mut out = Bits::EMPTY;
    for command in reply {
        if let Command::Move { x, y, .. } | Command::Train { x, y, .. } = *command {
            out.set(usize::from(y) * SIZE + usize::from(x));
        }
    }
    out
}

/// The counters to `reply` for the player to act in `state`: a tower on
/// each free cell of ours among the cells the reply enters and their
/// neighbours, and a unit of level 1 and 2 on each free cell of ours it
/// enters.
pub(super) fn counters(state: &State, reply: &[Command]) -> Vec<Action> {
    let me = state.to_act();
    let cells = entered(reply);
    let free = state.owned(me) & state.empty_cells();
    let mut out = Vec::new();
    for cell in ((cells | cells.neighbours()) & free)
        .and_not(state.map().spots)
        .iter()
    {
        let (x, y) = xy(cell);
        out.push(Action::One(Command::BuildTower { x, y }));
    }
    for cell in (cells & free).iter() {
        let (x, y) = xy(cell);
        for level in 1..=2 {
            out.push(Action::One(Command::Train { level, x, y }));
        }
    }
    out
}

impl Search {
    /// The plan `commands` from `root` repaired against `memory` (in which
    /// `worst`, its worst reply, is), at most [`super::REPAIR_STEPS`] steps,
    /// or `None` if no step raises its score.
    pub(super) fn repair(
        &mut self,
        root: &State,
        commands: &[Command],
        worst: &[Command],
        memory: &[Reply],
    ) -> Option<(Vec<Command>, State)> {
        let mut cur_commands = commands.to_vec();
        let mut cur = root.clone();
        if !apply_all(&mut cur, &cur_commands) {
            return None;
        }
        let mut cur_score = self.score(&cur, memory, i64::MIN);
        let mut improved = false;
        let mut actions = Vec::new();
        for _ in 0..super::REPAIR_STEPS {
            let mut best: Option<(Vec<Command>, State, i64)> = None;
            let mut best_score = cur_score;
            // Drop one command.
            for i in 0..cur_commands.len() {
                if self.spent() {
                    break;
                }
                let rest: Vec<Command> = cur_commands
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, c)| *c)
                    .collect();
                let mut state = root.clone();
                if !apply_all(&mut state, &rest) || self.breaks_reserve(&state) {
                    continue;
                }
                let score = self.score(&state, memory, best_score);
                if score > best_score {
                    best_score = score;
                    best = Some((rest, state, score));
                }
            }
            // Add a counter, then any action offered.
            actions.clear();
            actions.extend(counters(&cur, worst));
            if super::REPAIR_ANY {
                let mut more = Vec::new();
                generate(&cur, &mut more);
                actions.extend(more);
            }
            for action in &actions {
                if self.spent() {
                    break;
                }
                let mut state = cur.clone();
                if !apply_all(&mut state, action.commands()) || self.breaks_reserve(&state) {
                    continue;
                }
                let score = self.score(&state, memory, best_score);
                if score > best_score {
                    best_score = score;
                    let mut with = cur_commands.clone();
                    with.extend_from_slice(action.commands());
                    best = Some((with, state, score));
                }
            }
            let Some((with, state, score)) = best else {
                break;
            };
            cur_commands = with;
            cur = state;
            cur_score = score;
            improved = true;
        }
        improved.then_some((cur_commands, cur))
    }
}

#[cfg(test)]
mod tests;
