//! The bot's evaluation of a state, from the point of view of `seat`.
//!
//! It starts as the shared static evaluation
//! (`ice_and_fire_tactics::eval::evaluate`: what each side has, the threat to
//! the HQ and how far each side is from affording the chain to the other's
//! HQ). Terms of the bot's own go here, each documented:
//!
//! - **Debt** (E003): a side whose gold plus income is below 0 loses all its
//!   units at its next start of turn (RULES.md), so twice the training cost
//!   of its units on active cells is taken from its score: once for the
//!   units the shared evaluation counts, once for the loss.
//! - **Necks** (E006): a joint of a side's territory is an active cell whose
//!   capture cuts other cells off from the HQ (an articulation point of the
//!   territory, the HQ as root). The games of
//!   `games/ice-and-fire/research/game-study.md` were decided by such cuts,
//!   made with the gold the enemy had banked for a turn or two. Half the
//!   value cut off by the most damaging joint the enemy could take by a
//!   chain of trainings within its gold plus two turns of income (the cells
//!   beyond it, not the joint itself) is taken from the side's score; half, since the threat is two turns away and the side
//!   has a turn to answer. Only the side's own necks count: crediting it
//!   with the enemy's, within its own banked gold, made the search hoard
//!   gold instead of growing (−79.5 Elo against v004, E006); its cuts of the
//!   enemy's necks are already among its actions. A cell is valued as the shared evaluation values
//!   it: 7 (4, and 3 times its income), 12 more with a mine, 10 more with a
//!   tower, and the training cost of its unit.

use ice_and_fire_engine::{Bits, State, CELLS, HQ};
use ice_and_fire_tactics::chain::Reach;
use ice_and_fire_tactics::eval::evaluate;

/// The training cost of the units of `seat` on active cells, if it cannot
/// pay its upkeep, else 0.
fn debt(state: &State, seat: usize) -> i64 {
    if state.gold(seat) + state.income(seat) >= 0 {
        return 0;
    }
    (state.units(seat) & state.active(seat))
        .iter()
        .filter_map(|cell| state.unit_level(cell))
        .map(|level| 10 * i64::from(level))
        .sum()
}

/// The value of `cell` of the territory of `seat` (see the module's docs).
fn cell_value(state: &State, cell: usize) -> i64 {
    let mut value = 7;
    if state.mines().test(cell) {
        value += 12;
    }
    if state.towers().test(cell) {
        value += 10;
    }
    if let Some(level) = state.unit_level(cell) {
        value += 10 * i64::from(level);
    }
    value
}

/// Depth-first search for the joints of a territory (Tarjan's articulation
/// points), with the value each one's capture cuts off.
struct Joints {
    active: Bits,
    disc: [u16; CELLS],
    low: [u16; CELLS],
    time: u16,
    value: [i64; CELLS],
    /// The value of the cell's subtree in the search.
    subtree: [i64; CELLS],
    /// The value lost if the cell is taken: its own and that of the subtrees
    /// it alone joins to the root.
    severed: [i64; CELLS],
}

impl Joints {
    fn visit(&mut self, v: usize) {
        self.time += 1;
        self.disc[v] = self.time;
        self.low[v] = self.time;
        self.subtree[v] = self.value[v];
        self.severed[v] = self.value[v];
        for w in (Bits::cell(v).neighbours() & self.active).iter() {
            if self.disc[w] == 0 {
                self.visit(w);
                self.subtree[v] += self.subtree[w];
                self.low[v] = self.low[v].min(self.low[w]);
                if self.low[w] >= self.disc[v] {
                    self.severed[v] += self.subtree[w];
                }
            } else {
                self.low[v] = self.low[v].min(self.disc[w]);
            }
        }
    }
}

/// Half the value cut off by the most damaging joint of `seat`'s territory
/// that the enemy can take within its gold plus two turns of income (see the
/// module's docs); 0 if there is none.
pub fn neck(state: &State, seat: usize) -> i64 {
    let active = state.active(seat);
    let root = HQ[seat];
    if !active.test(root) {
        return 0;
    }
    let mut joints = Joints {
        active,
        disc: [0; CELLS],
        low: [0; CELLS],
        time: 0,
        value: [0; CELLS],
        subtree: [0; CELLS],
        severed: [0; CELLS],
    };
    for cell in active.iter() {
        joints.value[cell] = cell_value(state, cell);
    }
    joints.visit(root);
    let enemy = 1 - seat;
    let budget = state.gold(enemy) + 2 * state.income(enemy).max(0);
    let reach = Reach::new(state, enemy, false);
    let worst = active
        .iter()
        .filter(|&cell| cell != root && reach.cost[cell] <= budget)
        .map(|cell| joints.severed[cell] - joints.value[cell])
        .max()
        .unwrap_or(0);
    worst / 2
}

/// The score of `state` for `seat`, higher is better for it.
pub fn eval(state: &State, seat: usize) -> i64 {
    let score = evaluate(state, seat);
    if state.outcome().is_some() {
        return score;
    }
    score - 2 * debt(state, seat) + 2 * debt(state, 1 - seat) - neck(state, seat)
}

#[cfg(test)]
mod tests;
