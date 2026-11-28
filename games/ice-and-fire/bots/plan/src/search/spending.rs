//! Our purchases by zasmu's spending table (E109, stage 6 of
//! `games/ice-and-fire/research/codingame/2026-11-28-type-planner.md`): a
//! step of a climb orders its trainings, towers and mines by how often
//! zasmu buys their kind in this turn band and gold band
//! (`types::SpendTable`), and may offer only the band's likeliest kinds.
//! The kinds and the band come from the root of the climb, as in the
//! counts.

use ice_and_fire_engine::{Command, State};

use super::TYPE_ORDER;
use crate::actions::Action;
use crate::types::{Buy, Gold, SpendTable, Typing, WIDTH};

#[cfg(test)]
mod tests;

/// The spending of one climb: the typing of its root and the table's row.
pub struct Spending {
    typing: Typing,
    row: usize,
}

impl Spending {
    /// The spending of the side to act in `root`.
    pub fn new(root: &State) -> Spending {
        let typing = Typing::new(root);
        let row = typing.row();
        Spending { typing, row }
    }

    /// The kind of `action` if it is a single training, tower or mine.
    fn buy(&self, action: &Action) -> Option<Buy> {
        match action {
            Action::One(command) => self.typing.buy(*command),
            Action::Many(_) => None,
        }
    }

    /// The row of the table.
    fn shares(&self) -> &'static [u8; WIDTH] {
        &SpendTable::embedded().rows[self.row]
    }

    /// Removes from `actions` the single purchases whose kind is not among
    /// the `k` most frequent of the row; every other action stays (cuts, the
    /// lethal chain), and the blockers are added after. On a turn when a
    /// chain to an HQ can be paid (the gold band `Kill`) nothing is removed.
    pub fn filter(&self, k: usize, actions: &mut Vec<Action>) {
        if self.typing.gold_band() == Gold::Kill {
            return;
        }
        let table = SpendTable::embedded();
        actions.retain(|action| match self.buy(action) {
            Some(buy) => table.among(self.row, buy, k),
            None => true,
        });
    }

    /// The tie-break key of `action`, higher first: a single purchase, 1 plus
    /// its kind's share; with [`TYPE_ORDER`] a single move, 257 plus its
    /// prior (`move_prior`, E106), so that the purchases come after the
    /// moves; all other actions 0. Without [`TYPE_ORDER`] the purchases come
    /// before the moves and the rest, which keep the order generated.
    pub fn tie(&self, action: &Action, move_prior: Option<u8>) -> u16 {
        if let Some(buy) = self.buy(action) {
            return 1 + u16::from(self.shares()[buy as usize]);
        }
        match (action, move_prior) {
            (Action::One(Command::Move { .. }), Some(prior)) if TYPE_ORDER => {
                257 + u16::from(prior)
            }
            _ => 0,
        }
    }
}
