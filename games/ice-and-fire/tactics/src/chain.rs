//! The cheapest chain of trainings that takes a cell, aware of towers.
//!
//! A chain is a list of trainings, each on a cell next to the territory the
//! ones before it made (RULES.md, "Training"). Each cell costs the lowest
//! level that may enter it, 10 gold per level (RULES.md, "Capture"). The
//! costs of [`Reach`] are those of the state it is built from. They do not
//! change while a chain is planned on that state, except that a capture can
//! cut off an enemy tower from its HQ, whose protection then lapses; so
//! [`chain_to`] plans one training at a time and replans after each, and
//! also tries to take a nearby tower first.

use ice_and_fire_engine::map::NONE;
use ice_and_fire_engine::{Bits, Command, Outcome, State, CELLS, SIZE};

/// Cost of a cell that cannot be reached.
pub const UNREACHABLE: i64 = i64::MAX;

/// Most enemy towers [`chain_to`] tries to take on the way.
const MAX_TOWER_VARIANTS: usize = 8;

/// The cheapest cost, in gold, for a seat to take each cell by a chain of
/// trainings, on the costs of the state it was built from.
pub struct Reach {
    /// `cost[cell]`: gold to take `cell`; 0 on the sources; UNREACHABLE when no chain reaches it.
    pub cost: [i64; CELLS],
    /// `before[cell]`: the previous cell of the cheapest chain, `u8::MAX` (map::NONE) on sources and unreached cells.
    pub before: [u8; CELLS],
}

impl Reach {
    /// Dijkstra from the seat's active cells. Entering a cell costs 10 × the
    /// lowest level L in 1..=league.max_level() with `state.enterable(seat, L)`
    /// containing it; a cell no level may enter is a wall. Moving between cells
    /// uses the 4-neighbourhood `state.map().neighbours[0]`. Own active cells are
    /// sources and are never entered. With `moves` and `seat == state.to_act()`,
    /// each cell that a movable unit of the seat (level L) may step onto
    /// (a neighbour in `enterable(seat, L)`) is also a source, at cost 0: an
    /// estimate that lets every unit go everywhere next to it at once.
    pub fn new(state: &State, seat: usize, moves: bool) -> Reach {
        let max = usize::from(state.league().max_level());
        let mut enter = [Bits::EMPTY; 3];
        for (index, bits) in enter.iter_mut().enumerate().take(max) {
            *bits = state.enterable(seat, index as u8 + 1);
        }
        let active = state.active(seat);
        let neighbours = &state.map().neighbours[0];
        let mut cost = [UNREACHABLE; CELLS];
        let mut before = [NONE; CELLS];
        // The cells with a cost that are not settled yet.
        let mut open = [0u8; CELLS];
        let mut count = 0;
        let mut settled = Bits::EMPTY;
        for cell in active.iter() {
            cost[cell] = 0;
            open[count] = cell as u8;
            count += 1;
        }
        if moves && seat == state.to_act() {
            for from in state.movable_units().iter() {
                let Some(level) = state.unit_level(from) else {
                    continue;
                };
                let steps = Bits::cell(from).neighbours() & enter[usize::from(level) - 1];
                for cell in steps.iter() {
                    if cost[cell] != 0 {
                        cost[cell] = 0;
                        open[count] = cell as u8;
                        count += 1;
                    }
                }
            }
        }
        while count > 0 {
            // The cheapest open cell, the lowest index on a tie.
            let mut best = 0;
            for index in 1..count {
                let (a, b) = (usize::from(open[index]), usize::from(open[best]));
                if (cost[a], a) < (cost[b], b) {
                    best = index;
                }
            }
            let cell = usize::from(open[best]);
            count -= 1;
            open[best] = open[count];
            settled.set(cell);
            for &next in &neighbours[cell] {
                if next == NONE {
                    break;
                }
                let next = usize::from(next);
                if settled.test(next) || active.test(next) {
                    continue;
                }
                let Some(level) = enter[..max].iter().position(|bits| bits.test(next)) else {
                    continue;
                };
                let through = cost[cell] + 10 * (level as i64 + 1);
                if through < cost[next] {
                    if cost[next] == UNREACHABLE {
                        open[count] = next as u8;
                        count += 1;
                    }
                    cost[next] = through;
                    before[next] = cell as u8;
                }
            }
        }
        Reach { cost, before }
    }

    /// The cells of the cheapest chain to `target`, first entered first,
    /// `target` last; empty when `target` is a source or unreachable.
    pub fn path(&self, target: usize) -> Vec<usize> {
        let mut path = Vec::new();
        if target >= CELLS || self.cost[target] == UNREACHABLE {
            return path;
        }
        let mut cell = target;
        while self.before[cell] != NONE {
            path.push(cell);
            cell = usize::from(self.before[cell]);
        }
        path.reverse();
        path
    }
}

/// A chain of trainings for the player to act.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chain {
    pub commands: Vec<Command>,
    /// Gold the trainings cost.
    pub cost: i64,
}

/// The lowest level at which `seat` may enter `cell` in `state`.
fn lowest_level(state: &State, seat: usize, cell: usize) -> Option<u8> {
    (1..=state.league().max_level()).find(|&level| state.enterable(seat, level).test(cell))
}

/// Trains, one cell at a time with the costs replanned after each training,
/// until `scratch` has `goal` among the active cells of `me` (or `me` has
/// won). Adds to `commands` and `spent`. False when no chain within `budget`
/// is found or a training is refused.
fn replan(
    scratch: &mut State,
    me: usize,
    goal: usize,
    budget: i64,
    commands: &mut Vec<Command>,
    spent: &mut i64,
) -> bool {
    loop {
        if scratch.outcome() == Some(Outcome::Win(me)) {
            return true;
        }
        if scratch.owned(me).test(goal) && scratch.active(me).test(goal) {
            return true;
        }
        let reach = Reach::new(scratch, me, false);
        let cost = reach.cost[goal];
        if cost == UNREACHABLE || *spent + cost > budget {
            return false;
        }
        let Some(&cell) = reach.path(goal).first() else {
            return false;
        };
        let Some(level) = lowest_level(scratch, me, cell) else {
            return false;
        };
        let command = Command::Train {
            level,
            x: (cell % SIZE) as u8,
            y: (cell / SIZE) as u8,
        };
        if !scratch.apply(command) {
            return false;
        }
        *spent += 10 * i64::from(level);
        commands.push(command);
    }
}

/// The chain that takes each of `goals` in turn, within `budget` in all.
fn attempt(state: &State, me: usize, goals: &[usize], budget: i64) -> Option<Chain> {
    let mut scratch = state.clone();
    let mut commands = Vec::new();
    let mut spent = 0;
    for &goal in goals {
        if !replan(&mut scratch, me, goal, budget, &mut commands, &mut spent) {
            return None;
        }
    }
    (spent <= budget).then_some(Chain {
        commands,
        cost: spent,
    })
}

/// The cheapest chain of trainings found by which the player to act takes
/// `target`, costing at most `budget` gold, or None.
///
/// The plain attempt trains along the cheapest path of [`Reach`], replanned
/// after each training. Up to 8 enemy towers (the nearest to `target` that
/// the first [`Reach`] can reach) are also tried as a stop on the way, since
/// taking a tower, or cutting it off, can make the cells it protected
/// cheaper. The cheapest successful attempt wins, the plain one on a tie.
/// Applying its commands in order to `state` succeeds, and then `target` is
/// the player's (or the player has won).
pub fn chain_to(state: &State, target: usize, budget: i64) -> Option<Chain> {
    if state.outcome().is_some() || target >= CELLS || state.map().void.test(target) {
        return None;
    }
    let me = state.to_act();
    if state.owned(me).test(target) && state.active(me).test(target) {
        return Some(Chain {
            commands: Vec::new(),
            cost: 0,
        });
    }
    let mut best = attempt(state, me, &[target], budget);
    let first = Reach::new(state, me, false);
    let mut towers: Vec<usize> = (state.towers() & state.owned(1 - me))
        .iter()
        .filter(|&tower| tower != target && first.cost[tower] != UNREACHABLE)
        .collect();
    towers.sort_by_key(|&tower| (state.map().distance(tower, target), tower));
    towers.truncate(MAX_TOWER_VARIANTS);
    for tower in towers {
        if let Some(chain) = attempt(state, me, &[tower, target], budget) {
            if best.as_ref().is_none_or(|top| chain.cost < top.cost) {
                best = Some(chain);
            }
        }
    }
    best
}

#[cfg(test)]
mod tests;
