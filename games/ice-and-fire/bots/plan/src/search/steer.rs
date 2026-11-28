//! The type-guided climb (E106, stage 2 of
//! `games/ice-and-fire/research/codingame/2026-11-28-type-planner.md`):
//! each unit of ours is offered only the moves of its type's likeliest
//! kinds, tried in the tables' order.

use ice_and_fire_engine::{Command, State, SIZE};

use crate::actions::Action;
use crate::types::{likeliest, prior, Kind, Typing, KINDS};

/// The typing of one step of a climb and the table's probabilities for each
/// unit that can move in it.
pub struct Steer {
    typing: Typing,
    /// For each movable unit: its id, its cell and the probabilities of its
    /// key.
    units: Vec<(u32, usize, [u8; KINDS])>,
}

impl Steer {
    /// The typing of the side to act in `state`, one per step.
    pub fn new(state: &State) -> Steer {
        let typing = Typing::new(state);
        let units = state
            .movable_units()
            .iter()
            .filter_map(|cell| {
                let (id, _) = state.unit_at(cell)?;
                let key = typing.key(state, cell)?;
                Some((id, cell, prior(&key)))
            })
            .collect();
        Steer { typing, units }
    }

    /// The kind of `action` and the probabilities of its unit's key, if it
    /// is a single move of a unit of the step.
    fn kind_of(&self, state: &State, action: &Action) -> Option<(Kind, [u8; KINDS])> {
        let Action::One(Command::Move { id, x, y }) = action else {
            return None;
        };
        let &(_, from, p) = self.units.iter().find(|u| u.0 == *id)?;
        let to = usize::from(*y) * SIZE + usize::from(*x);
        Some((self.typing.kind(state, from, to), p))
    }

    /// The prior of `action`: that of its kind for a single move, 0 for any
    /// other action.
    pub fn prior(&self, state: &State, action: &Action) -> u8 {
        self.kind_of(state, action)
            .map_or(0, |(kind, p)| p[kind as usize])
    }

    /// Removes from `actions` the single moves whose kind is not among the
    /// `k` likeliest of their unit's key; every other action stays.
    pub fn filter(&self, state: &State, k: usize, actions: &mut Vec<Action>) {
        actions.retain(|action| match self.kind_of(state, action) {
            Some((kind, p)) => {
                let order = likeliest(&p);
                order.is_empty() || order.iter().take(k).any(|&top| top == kind)
            }
            None => true,
        });
    }
}

#[cfg(test)]
mod tests;
