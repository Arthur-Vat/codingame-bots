//! A cut finder: short chains of trainings that take enemy cells or cut them
//! off from the enemy HQ.
//!
//! A cell is active only while it is joined to its owner's HQ through the
//! owner's cells (RULES.md, "Active cells"). A unit on a cell that is no longer
//! active dies at its owner's start of turn, and an inactive cell earns
//! nothing, so taking the one cell that joins a region to its HQ takes the
//! whole region out of the game. [`cuts`] looks for such cells with a
//! depth-first search over the chains of trainings that stay on the enemy's
//! territory and the cells next to it.

use std::collections::HashMap;

use ice_and_fire_engine::{Bits, Command, Outcome, State, SIZE};

/// Most trainings simulated by one call of [`cuts`], to bound its time.
pub const MAX_NODES: usize = 20_000;

/// A short chain of trainings that removes opponent cells from its HQ's territory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cut {
    pub commands: Vec<Command>,
    /// Gold the trainings cost.
    pub cost: i64,
    /// Opponent active cells before minus after (taken or cut off).
    pub cells: i64,
    /// Training cost (10, 20, 30 by level) of opponent units on active cells, before minus after (killed, or left on cells that are no longer active, where they die at the opponent's start of turn).
    pub units: i64,
    /// Opponent mines and towers on its active cells, before minus after.
    pub buildings: i64,
    /// The cutting player's income after minus before.
    pub income: i64,
    /// 3 × cells + units + 10 × buildings + 2 × income − cost.
    pub score: i64,
}

/// What a side holds that a cut can take from it.
#[derive(Clone, Copy)]
struct Holdings {
    cells: i64,
    units: i64,
    buildings: i64,
}

/// The active cells of `seat`, the training cost of its units on them, and its
/// mines and towers on them.
fn holdings(state: &State, seat: usize) -> Holdings {
    let active = state.active(seat);
    let units = state.units(seat) & active;
    let value = units
        .iter()
        .filter_map(|cell| state.unit_level(cell))
        .map(|level| 10 * i64::from(level))
        .sum();
    Holdings {
        cells: i64::from(active.count()),
        units: value,
        buildings: i64::from(((state.mines() | state.towers()) & active).count()),
    }
}

/// The search: what it measures against, and what it found.
struct Search {
    me: usize,
    /// The opponent's cells and the cells next to them, on the original state.
    zone: Bits,
    before: Holdings,
    income: i64,
    max_len: usize,
    budget: i64,
    nodes: usize,
    path: Vec<Command>,
    /// The cuts found, in the order found.
    found: Vec<Cut>,
    /// For each set of cells (a [`Key`]) the index in `found` of its cut.
    /// Only looked up, never iterated, so the output does not depend on its
    /// order.
    index: HashMap<Key, usize>,
}

/// The words of a set of cells, to key a map by.
type Key = [u64; 3];

/// `key` with `cell` added.
fn with_cell(mut key: Key, cell: usize) -> Key {
    key[cell / 64] |= 1 << (cell % 64);
    key
}

impl Search {
    /// Extends the chain `self.path` (its last cell is `last`, the cells of
    /// `taken` in all, `cost` gold) on `scratch`. False when the node limit
    /// ends the search.
    fn descend(&mut self, scratch: &State, last: Option<usize>, taken: Key, cost: i64) -> bool {
        let me = self.me;
        let options = match last {
            None => scratch.near_active(me),
            Some(cell) => Bits::cell(cell).neighbours(),
        } & self.zone;
        let options = options.and_not(scratch.owned(me));
        let mut enter = [Bits::EMPTY; 3];
        for level in 1..=scratch.league().max_level() {
            enter[usize::from(level) - 1] = scratch.enterable(me, level);
        }
        for cell in options.iter() {
            let max = usize::from(scratch.league().max_level());
            let Some(index) = enter[..max].iter().position(|bits| bits.test(cell)) else {
                continue;
            };
            let level = index as u8 + 1;
            let price = 10 * i64::from(level);
            if cost + price > self.budget {
                continue;
            }
            if self.nodes >= MAX_NODES {
                return false;
            }
            self.nodes += 1;
            let mut next = scratch.clone();
            let command = Command::Train {
                level,
                x: (cell % SIZE) as u8,
                y: (cell / SIZE) as u8,
            };
            if !next.apply(command) {
                continue;
            }
            self.path.push(command);
            let set = with_cell(taken, cell);
            self.record(&next, set, cost + price);
            let deeper = self.path.len() < self.max_len && next.outcome().is_none();
            let go_on = !deeper || self.descend(&next, Some(cell), set, cost + price);
            self.path.pop();
            if !go_on {
                return false;
            }
        }
        true
    }

    /// Keeps the chain `self.path` as a cut if `after`, the state it leads
    /// to, shows a gain.
    fn record(&mut self, after: &State, set: Key, cost: i64) {
        let opp = 1 - self.me;
        let now = holdings(after, opp);
        let cells = self.before.cells - now.cells;
        let units = self.before.units - now.units;
        if cells <= 0 && units <= 0 {
            return;
        }
        let buildings = self.before.buildings - now.buildings;
        let income = after.income(self.me) - self.income;
        let won = after.outcome() == Some(Outcome::Win(self.me));
        let score = if won {
            i64::MAX / 2
        } else {
            3 * cells + units + 10 * buildings + 2 * income - cost
        };
        if let Some(&at) = self.index.get(&set) {
            if cost >= self.found[at].cost {
                return;
            }
        }
        let cut = Cut {
            commands: self.path.clone(),
            cost,
            cells,
            units,
            buildings,
            income,
            score,
        };
        match self.index.get(&set) {
            Some(&at) => self.found[at] = cut,
            None => {
                self.index.insert(set, self.found.len());
                self.found.push(cut);
            }
        }
    }
}

/// The best cuts for the player to act: at most `limit`, best `score` first.
///
/// Depth-first over chains of 1 to `max_len` trainings, each on a cell of the
/// opponent's or next to one, at the lowest level that may enter it, the
/// first next to the territory and each next one next to the one before, for
/// at most `budget` gold. Every prefix of a chain is a candidate, kept when
/// it takes opponent cells or units out of play. A chain that takes the HQ
/// scores highest. The same set of cells is kept once, the cheapest. At most
/// [`MAX_NODES`] trainings are simulated. Ties in score go to the lower cost,
/// then to the first found; the output is deterministic.
pub fn cuts(state: &State, max_len: usize, budget: i64, limit: usize) -> Vec<Cut> {
    search(state, max_len, budget, limit).0
}

/// [`cuts`], and the number of trainings simulated.
fn search(state: &State, max_len: usize, budget: i64, limit: usize) -> (Vec<Cut>, usize) {
    if state.outcome().is_some() || max_len == 0 || limit == 0 {
        return (Vec::new(), 0);
    }
    let me = state.to_act();
    let opp = 1 - me;
    let mut run = Search {
        me,
        zone: state.owned(opp) | state.owned(opp).neighbours(),
        before: holdings(state, opp),
        income: state.income(me),
        max_len,
        budget,
        nodes: 0,
        path: Vec::new(),
        found: Vec::new(),
        index: HashMap::new(),
    };
    run.descend(state, None, [0; 3], 0);
    let nodes = run.nodes;
    let mut found = run.found;
    // The sort is stable: the first found stays first on a tie.
    found.sort_by(|a, b| b.score.cmp(&a.score).then(a.cost.cmp(&b.cost)));
    found.truncate(limit);
    (found, nodes)
}

#[cfg(test)]
mod tests;
