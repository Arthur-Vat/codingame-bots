//! E090's changes to how the search explores (the owner, 2026-11-26: the
//! greedy climb adds one action at a time, so a mix of purchases that only
//! pays as a whole is rarely found, and a purchase made early is never
//! undone): spending splits as the climbs' starts, steps that remove or
//! swap an action, and climbs from the best plan with actions removed.

use ice_and_fire_engine::{Command, State};
use ice_and_fire_tactics::cut::cuts;

use super::{apply_all, Reply, Search};
use crate::actions::{generate, Action, CUT_LENGTH};
use crate::eval::threatened;

/// Whether `action` spends gold.
pub(super) fn is_purchase(action: &Action) -> bool {
    action.commands().iter().any(|c| {
        matches!(
            c,
            Command::Train { .. } | Command::BuildTower { .. } | Command::BuildMine { .. }
        )
    })
}

/// The active cells of ours that the enemy's best cut removes once the turn
/// that ends in `end` is over.
fn best_cut(end: &State) -> i64 {
    let mut next = end.clone();
    next.end_turn();
    if next.outcome().is_some() {
        return 0;
    }
    let gold = next.gold(next.to_act());
    cuts(&next, CUT_LENGTH, gold, 3)
        .iter()
        .map(|c| c.cells)
        .max()
        .unwrap_or(0)
        .max(0)
}

impl Search {
    /// The spending splits of `root` (E090's [`super::SPLITS`]): for each
    /// number of towers up to [`super::SPLIT_TOWERS`], each tower on the cell
    /// offered that leaves the enemy's best cut smallest, then each number
    /// of trainings up to [`super::SPLIT_TRAININGS`], the trainings by their
    /// gain alone from `root`. The `count` best against `memory`, best
    /// last.
    pub(super) fn splits(&mut self, root: &State, memory: &[Reply], count: usize) -> Vec<Action> {
        let mut actions = Vec::new();
        generate(root, &mut actions);
        let towers: Vec<Command> = actions
            .iter()
            .filter_map(|a| match a {
                Action::One(c @ Command::BuildTower { .. }) => Some(*c),
                _ => None,
            })
            .collect();
        let mut trainings: Vec<(i64, Command)> = Vec::new();
        for action in &actions {
            if let Action::One(c @ Command::Train { .. }) = action {
                let mut next = root.clone();
                if next.apply(*c) && !self.breaks_reserve(&next) {
                    trainings.push((self.score(&next, memory, i64::MIN), *c));
                }
            }
            if self.spent() {
                return Vec::new();
            }
        }
        trainings.sort_by(|a, b| b.0.cmp(&a.0));
        let mut found: Vec<(i64, Vec<Command>)> = Vec::new();
        let mut cur = root.clone();
        let mut bought: Vec<Command> = Vec::new();
        for t in 0..=super::SPLIT_TOWERS {
            if t > 0 {
                let mut best: Option<((i64, i64), Command, State)> = None;
                for &tower in &towers {
                    let mut next = cur.clone();
                    if !next.apply(tower) {
                        continue;
                    }
                    let rank = (best_cut(&next), threatened(&next, self.me));
                    if best.as_ref().is_none_or(|(r, _, _)| rank < *r) {
                        best = Some((rank, tower, next));
                    }
                }
                let Some((_, tower, next)) = best else {
                    break;
                };
                bought.push(tower);
                cur = next;
            }
            let mut state = cur.clone();
            let mut commands = bought.clone();
            if !commands.is_empty() && !self.breaks_reserve(&state) {
                found.push((self.score(&state, memory, i64::MIN), commands.clone()));
            }
            let mut trained = 0;
            for &(_, training) in &trainings {
                if trained == super::SPLIT_TRAININGS || self.spent() {
                    break;
                }
                let mut next = state.clone();
                if !next.apply(training) {
                    continue;
                }
                trained += 1;
                state = next;
                commands.push(training);
                if !self.breaks_reserve(&state) {
                    found.push((self.score(&state, memory, i64::MIN), commands.clone()));
                }
            }
        }
        found.sort_by(|a, b| b.0.cmp(&a.0));
        found.truncate(count);
        found.reverse();
        found.into_iter().map(|(_, c)| Action::Many(c)).collect()
    }

    /// One improvement of the plan `taken` from `root` (E090's
    /// [`super::SWAPS`]): the best of removing one of its actions or
    /// replacing one of its purchases by another purchase, if it scores
    /// above `cur_score`; then `taken`, `cur` and `cur_score` are its.
    pub(super) fn improve(
        &mut self,
        root: &State,
        taken: &mut Vec<Action>,
        cur: &mut State,
        cur_score: &mut i64,
        memory: &[Reply],
    ) -> bool {
        let mut best: Option<(Vec<Action>, State, i64)> = None;
        let mut best_score = *cur_score;
        let mut actions = Vec::new();
        for i in 0..taken.len() {
            if self.spent() {
                break;
            }
            let rest: Vec<Action> = taken
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, a)| a.clone())
                .collect();
            let mut state = root.clone();
            if !rest.iter().all(|a| apply_all(&mut state, a.commands())) {
                continue;
            }
            if !self.breaks_reserve(&state) {
                let score = self.score(&state, memory, best_score);
                if score > best_score {
                    best_score = score;
                    best = Some((rest.clone(), state.clone(), score));
                }
            }
            if !is_purchase(&taken[i]) {
                continue;
            }
            generate(&state, &mut actions);
            for action in actions.iter().filter(|a| is_purchase(a)) {
                if action.commands() == taken[i].commands() || self.spent() {
                    continue;
                }
                let mut next = state.clone();
                if !apply_all(&mut next, action.commands()) || self.breaks_reserve(&next) {
                    continue;
                }
                let score = self.score(&next, memory, best_score);
                if score > best_score {
                    best_score = score;
                    let mut with = rest.clone();
                    with.push(action.clone());
                    best = Some((with, next, score));
                }
            }
        }
        match best {
            Some((plan, state, score)) => {
                *taken = plan;
                *cur = state;
                *cur_score = score;
                true
            }
            None => false,
        }
    }
}

/// The starts of the climbs from the plan `commands` (E090's
/// [`super::PERTURB`]): the plan without its purchases, then without each
/// of its first commands in turn, `count` at most, the first to try last.
pub(super) fn perturbations(commands: &[Command], count: usize) -> Vec<Action> {
    let mut out = Vec::new();
    let moves: Vec<Command> = commands
        .iter()
        .copied()
        .filter(|c| !is_purchase(&Action::One(*c)))
        .collect();
    if !moves.is_empty() && moves.len() < commands.len() {
        out.push(Action::Many(moves));
    }
    for i in 0..commands.len() {
        if out.len() >= count {
            break;
        }
        let rest: Vec<Command> = commands
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, c)| *c)
            .collect();
        if !rest.is_empty() {
            out.push(Action::Many(rest));
        }
    }
    out.truncate(count);
    out.reverse();
    out
}

#[cfg(test)]
mod tests;
