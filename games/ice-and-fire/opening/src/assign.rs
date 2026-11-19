//! The joint frontier assignment (step 1 of phase 2 of the early-game plan,
//! experiment E072, `games/ice-and-fire/research/codingame/2026-11-19-early-game.md`):
//! each turn of the opening, our units and the trainings we can pay are
//! matched to target cells all at once, so that they work together and
//! spread along the front rather than each choosing its own cell.
//!
//! - **Targets:** the middle choke cells we do not hold (the cells most
//!   shortest HQ-to-HQ paths cross near the middle, `ice_and_fire_early`'s
//!   zones), then neutral cells spaced along the Voronoi frontier (as near
//!   our land as the enemy's, within [`Settings::slack`] steps), at least
//!   [`Settings::spacing`] steps from each other, nearest the middle first.
//! - **Agents:** our units that may move, and the trainings of this turn
//!   ([`Purchase`]), a training standing on the trainable cell nearest its
//!   target (the frontier cell nearest the middle).
//! - **The assignment** minimizes the total arrival turns (a moving unit
//!   arrives after `d − 1` more turns, a training after `d`), less
//!   [`Settings::choke_bonus`] for a middle choke, plus [`Settings::late`]
//!   for each turn the enemy's land could reach the target before us; it is
//!   solved exactly (the Hungarian method) every turn.
//! - **Moves:** each assigned unit steps to a neighbour one step nearer its
//!   target, a neutral cell first; the other units take the free neighbour
//!   with most free cells next to it, or walk toward the nearest free cell
//!   of our side. **Trainings** follow on the cells nearest their targets.
//!
//! There is no penalty for a thin shape: the zasmu study found none early.
//! Every command is applied to a scratch copy of the state as it is chosen,
//! so the turn holds only commands the rules accept.

use ice_and_fire_early::zones::Zones;
use ice_and_fire_engine::{Bits, Command, State, CELLS, SIZE};

use crate::econ::{self, Buy, Params};

/// How many trainings a turn makes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Purchase {
    /// As many as the `econ` opening's payback rule would buy with
    /// [`Settings::params`]; those no target takes go to the free cell with
    /// most free cells next to it when [`Settings::fill`] is set.
    Payback,
    /// One for each target that a training reaches first, while the gold
    /// pays a level-1 unit.
    Affordable,
}

/// The assignment's settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    /// The fewest steps between two targets.
    pub spacing: u8,
    /// A frontier cell is at most this many steps nearer one side's land
    /// than the other's.
    pub slack: u8,
    /// Taken off a middle choke's cost, so that the chokes are served first.
    pub choke_bonus: i64,
    /// Added to a target's cost for each turn the enemy's land could reach
    /// it before our agent.
    pub late: i64,
    /// Targets beyond the number of agents, so that the matching can choose.
    pub extra: usize,
    /// A unit walks to its target along the path, at most this many steps
    /// longer than the shortest, that takes the most cells.
    pub detour: u8,
    /// Whether a unit next to a cell we do not hold always takes one.
    pub take_first: bool,
    /// Whether the cells on a unit's path are kept from the later units'.
    pub reserve: bool,
    /// Whether the units with the fewest cells to take move first.
    pub fewest_first: bool,
    /// How many trainings a turn makes.
    pub buy: Purchase,
    /// Whether the payback rule's trainings that no target takes are made.
    pub fill: bool,
    /// The `econ` weights of the payback rule.
    pub params: Params,
}

impl Settings {
    /// The settings screened first in E072.
    pub const DEFAULT: Settings = Settings {
        spacing: 3,
        slack: 1,
        choke_bonus: 4,
        late: 2,
        extra: 2,
        detour: 2,
        take_first: true,
        reserve: true,
        fewest_first: true,
        buy: Purchase::Payback,
        fill: true,
        params: Params::V003,
    };
}

/// The cost of leaving a target without an agent: more than any real
/// assignment.
const UNSERVED: i64 = 1_000;

/// Cost of a level-1 unit.
const UNIT_COST: i64 = 10;

fn xy(cell: usize) -> (u8, u8) {
    ((cell % SIZE) as u8, (cell / SIZE) as u8)
}

/// Applies `command` to `state` and keeps it in `out` if the rules accept it;
/// whether they did.
fn play(state: &mut State, command: Command, out: &mut Vec<Command>) -> bool {
    let accepted = state.apply(command);
    if accepted {
        out.push(command);
    }
    accepted
}

/// Moves the unit on `from` to its neighbour `to`; whether the rules accepted
/// it.
fn step(state: &mut State, from: usize, to: usize, out: &mut Vec<Command>) -> bool {
    let Some((id, _)) = state.unit_at(from) else {
        return false;
    };
    let (x, y) = xy(to);
    play(state, Command::Move { id, x, y }, out)
}

/// The targets of `state` for the player to act: the middle chokes it does
/// not hold, then frontier cells, at most `limit` in all.
pub fn targets(state: &State, zones: &Zones, settings: &Settings, limit: usize) -> Vec<usize> {
    let me = state.to_act();
    let ours = econ::distances(state, state.owned(me));
    let theirs = econ::distances(state, state.owned(1 - me));
    let neutral = state.map().valid.and_not(state.owned(0) | state.owned(1));
    let reachable = |c: usize| ours[c] != u8::MAX && theirs[c] != u8::MAX;
    // A middle choke counts while we can still enter it: neutral, or the
    // enemy's.
    let mut chokes: Vec<usize> = zones
        .middle_chokes()
        .and_not(state.owned(me))
        .iter()
        .filter(|&c| reachable(c))
        .collect();
    chokes.sort_by_key(|&c| (ours[c], c));
    let mut frontier: Vec<usize> = neutral
        .iter()
        .filter(|&c| reachable(c) && ours[c].abs_diff(theirs[c]) <= settings.slack)
        .collect();
    // Nearest the middle first, then nearest us.
    frontier.sort_by_key(|&c| (ours[c].abs_diff(theirs[c]), ours[c], c));
    let mut out: Vec<usize> = Vec::new();
    for cell in chokes.into_iter().chain(frontier) {
        if out.len() >= limit {
            break;
        }
        let far = out
            .iter()
            .all(|&t| state.map().distance(t, cell) >= settings.spacing);
        let choke = zones.middle_chokes().test(cell);
        if !out.contains(&cell) && (choke || far) {
            out.push(cell);
        }
    }
    out
}

/// The minimum-cost assignment of a square `n × n` matrix (`cost[row][col]`),
/// by the Hungarian method with potentials: for each row, its column.
pub fn hungarian(cost: &[Vec<i64>]) -> Vec<usize> {
    let n = cost.len();
    let inf = i64::MAX / 4;
    // 1-based, with column 0 as the root of each augmenting search.
    let mut u = vec![0i64; n + 1];
    let mut v = vec![0i64; n + 1];
    let mut p = vec![0usize; n + 1];
    let mut way = vec![0usize; n + 1];
    for i in 1..=n {
        p[0] = i;
        let mut j0 = 0usize;
        let mut minv = vec![inf; n + 1];
        let mut used = vec![false; n + 1];
        loop {
            used[j0] = true;
            let i0 = p[j0];
            let mut delta = inf;
            let mut j1 = 0usize;
            for j in 1..=n {
                if !used[j] {
                    let cur = cost[i0 - 1][j - 1] - u[i0] - v[j];
                    if cur < minv[j] {
                        minv[j] = cur;
                        way[j] = j0;
                    }
                    if minv[j] < delta {
                        delta = minv[j];
                        j1 = j;
                    }
                }
            }
            for j in 0..=n {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }
            j0 = j1;
            if p[j0] == 0 {
                break;
            }
        }
        loop {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
            if j0 == 0 {
                break;
            }
        }
    }
    let mut out = vec![0usize; n];
    for j in 1..=n {
        if p[j] > 0 {
            out[p[j] - 1] = j - 1;
        }
    }
    out
}

/// One agent of the assignment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Agent {
    /// The unit on this cell.
    Unit(usize),
    /// A training of this turn.
    Training,
}

/// The cells a level-1 training of the player to act may take now: next to
/// its active land, enterable, and not its own.
fn trainable(state: &State) -> Bits {
    let me = state.to_act();
    (state.near_active(me) & state.enterable(me, 1)).and_not(state.owned(me))
}

/// The number of trainings the payback rule would make in `state`.
fn payback_count(state: &State, params: &Params) -> usize {
    econ::turn_with(state, params, Buy::Payback)
        .iter()
        .filter(|c| matches!(c, Command::Train { .. }))
        .count()
}

/// The free cells next to `cell`.
fn open(cell: usize, free: Bits) -> u32 {
    (Bits::cell(cell).neighbours() & free).count()
}

/// Layer `k` holds, for each cell, the most cells of `gain` a path from it
/// to `target` of at most `k` steps takes (the cell itself left out, the
/// target counted), `i64::MIN` where no such path exists; layers 0 to `len`.
/// Paths go through valid cells.
pub fn gains_to(state: &State, gain: Bits, target: usize, len: u8) -> Vec<[i64; CELLS]> {
    let valid = state.map().valid;
    let mut best = [i64::MIN; CELLS];
    best[target] = 0;
    let mut layers = vec![best];
    for _ in 0..len {
        let mut next = best;
        for cell in valid.iter() {
            for n in (Bits::cell(cell).neighbours() & valid).iter() {
                if best[n] > i64::MIN {
                    let value = best[n] + i64::from(gain.test(n));
                    if value > next[cell] {
                        next[cell] = value;
                    }
                }
            }
        }
        best = next;
        layers.push(best);
    }
    layers
}

/// The cells of the path that [`gains_to`]'s `layers` value from `from`
/// within `k` steps, `from` left out: the cells of `gain` it would take.
fn path_gains(state: &State, layers: &[[i64; CELLS]], gain: Bits, from: usize, k: usize) -> Bits {
    let valid = state.map().valid;
    let mut out = Bits::EMPTY;
    let mut cell = from;
    let mut k = k.min(layers.len() - 1);
    while k > 0 && layers[k][cell] > 0 {
        let Some(next) = (Bits::cell(cell).neighbours() & valid)
            .iter()
            .filter(|&n| layers[k - 1][n] > i64::MIN)
            .max_by_key(|&n| {
                (
                    i64::from(gain.test(n)) + layers[k - 1][n],
                    std::cmp::Reverse(n),
                )
            })
        else {
            break;
        };
        if gain.test(next) {
            out.set(next);
        }
        cell = next;
        k -= 1;
    }
    out
}

/// The opening's commands for the player to act in `state`.
pub fn turn(state: &State, settings: &Settings) -> Vec<Command> {
    let me = state.to_act();
    let zones = Zones::of(state);
    let mut scratch = state.clone();
    let mut out = Vec::new();
    let theirs = econ::distances(state, state.owned(1 - me));

    // The agents: units that may move, then this turn's trainings.
    let units: Vec<usize> = state.movable_units().iter().collect();
    let gold_units = (state.gold(me) / UNIT_COST).max(0) as usize;
    let trainings = match settings.buy {
        Purchase::Payback => payback_count(state, &settings.params).min(gold_units),
        Purchase::Affordable => gold_units,
    };
    let mut agents: Vec<Agent> = units.iter().map(|&c| Agent::Unit(c)).collect();
    agents.extend(std::iter::repeat_n(Agent::Training, trainings));
    let goals = targets(state, &zones, settings, agents.len() + settings.extra);

    // The cost of each agent for each target, in arrival turns.
    let train_cells = trainable(state);
    let chokes = zones.middle_chokes();
    let n = agents.len().max(goals.len());
    let mut cost = vec![vec![0i64; n]; n];
    for (row, &target) in goals.iter().enumerate() {
        for (col, slot) in cost[row].iter_mut().enumerate() {
            let Some(&agent) = agents.get(col) else {
                *slot = UNSERVED;
                continue;
            };
            let arrival = match agent {
                Agent::Unit(cell) => {
                    let d = state.map().distance(cell, target);
                    if d == u8::MAX {
                        None
                    } else {
                        Some(i64::from(d) - 1)
                    }
                }
                Agent::Training => train_cells
                    .iter()
                    .map(|c| state.map().distance(c, target))
                    .filter(|&d| d != u8::MAX)
                    .min()
                    .map(i64::from),
            };
            *slot = match arrival {
                None => UNSERVED,
                Some(arrival) => {
                    // The enemy's land reaches the target in `theirs − 1`
                    // turns of moves.
                    let enemy = i64::from(theirs[target]) - 1;
                    let late = settings.late * (arrival - enemy).max(0);
                    let bonus = if chokes.test(target) {
                        settings.choke_bonus
                    } else {
                        0
                    };
                    (arrival + late - bonus).max(0)
                }
            };
        }
    }
    // Rows past the targets are dummy targets: an agent there is free.
    let assigned = hungarian(&cost);
    let mut goal_of_unit: Vec<(usize, usize)> = Vec::new();
    let mut training_goals: Vec<(i64, usize)> = Vec::new();
    for (row, &target) in goals.iter().enumerate() {
        let col = assigned[row];
        if cost[row][col] >= UNSERVED {
            continue;
        }
        match agents.get(col) {
            Some(Agent::Unit(cell)) => goal_of_unit.push((*cell, target)),
            Some(Agent::Training) => training_goals.push((cost[row][col], target)),
            None => {}
        }
    }

    // Moves of the assigned units, the fewest options first.
    let mut free = state.map().valid.and_not(state.owned(0) | state.owned(1));
    let mut moved: Vec<usize> = Vec::new();
    let mut reserved = Bits::EMPTY;
    // The units with the fewest cells to take next to them first, as in
    // `econ`, so that the others leave those cells to them.
    let options = |cell: usize| {
        let level = state.unit_level(cell).unwrap_or(1);
        let take = free | state.owned(1 - me);
        (Bits::cell(cell).neighbours() & take & state.enterable(me, level)).count()
    };
    goal_of_unit.sort_by_key(|&(cell, target)| {
        let fewest = if settings.fewest_first {
            options(cell)
        } else {
            0
        };
        (fewest, state.map().distance(cell, target), cell)
    });
    for &(from, target) in &goal_of_unit {
        let Some(level) = scratch.unit_level(from) else {
            continue;
        };
        let enter = scratch.enterable(me, level);
        let d = scratch.map().distance(from, target);
        // The first step of the path to the target, at most `detour` steps
        // longer than the shortest, that takes the most cells.
        let gain = (free | scratch.owned(1 - me)).and_not(reserved);
        let len = d.saturating_add(settings.detour);
        let layers = gains_to(&scratch, gain, target, len.saturating_sub(1));
        let ahead = layers[layers.len() - 1];
        let next = (Bits::cell(from).neighbours() & enter)
            .iter()
            .filter(|&c| ahead[c] > i64::MIN)
            .max_by_key(|&c| {
                let closer = d.saturating_sub(scratch.map().distance(c, target));
                // A cell we do not hold first, whatever the path.
                let take = free.test(c) || scratch.owned(1 - me).test(c);
                (
                    take && settings.take_first,
                    i64::from(gain.test(c)) + ahead[c],
                    closer,
                    open(c, free),
                    std::cmp::Reverse(c),
                )
            })
            .filter(|&c| {
                free.test(c)
                    || scratch.owned(1 - me).test(c)
                    || scratch.map().distance(c, target) < d
            });
        if let Some(to) = next {
            if step(&mut scratch, from, to, &mut out) {
                free.clear(to);
                moved.push(from);
                // Later units plan around the cells this one will take.
                if settings.reserve {
                    reserved |= path_gains(&scratch, &layers, gain, to, layers.len() - 1);
                }
            }
        }
    }
    // The other units: a free neighbour, else toward the nearest free cell
    // of our side, else the nearest free cell.
    let ours = econ::distances(state, state.owned(me));
    let mut side = Bits::EMPTY;
    for c in free.iter() {
        if ours[c] <= theirs[c] {
            side.set(c);
        }
    }
    for &from in &units {
        if moved.contains(&from) || !scratch.movable_units().test(from) {
            continue;
        }
        let Some(level) = scratch.unit_level(from) else {
            continue;
        };
        let enter = scratch.enterable(me, level);
        let take = (Bits::cell(from).neighbours() & free & enter)
            .iter()
            .max_by_key(|&c| (open(c, free), theirs[c] == u8::MAX, std::cmp::Reverse(c)));
        if let Some(to) = take {
            if step(&mut scratch, from, to, &mut out) {
                free.clear(to);
            }
            continue;
        }
        let goal = [side & free, free].into_iter().find_map(|cells| {
            cells
                .iter()
                .min_by_key(|&c| (scratch.map().distance(from, c), c))
        });
        if let Some(goal) = goal {
            let d = scratch.map().distance(from, goal);
            let next = (Bits::cell(from).neighbours() & enter)
                .iter()
                .filter(|&c| scratch.map().distance(c, goal) < d)
                .min_by_key(|&c| c);
            if let Some(to) = next {
                step(&mut scratch, from, to, &mut out);
            }
        }
    }

    // Trainings on the cells nearest their targets, the cheapest first.
    training_goals.sort();
    let mut made = 0usize;
    for &(_, target) in &training_goals {
        if scratch.gold(me) < UNIT_COST {
            break;
        }
        let cells = trainable(&scratch);
        let Some(near) = cells
            .iter()
            .map(|c| scratch.map().distance(c, target))
            .min()
        else {
            break;
        };
        let gain = (free | scratch.owned(1 - me)).and_not(reserved);
        let layers = gains_to(&scratch, gain, target, near.saturating_add(settings.detour));
        let ahead = layers[layers.len() - 1];
        let cell = cells
            .iter()
            .filter(|&c| scratch.map().distance(c, target) == near)
            .max_by_key(|&c| (ahead[c], open(c, free), std::cmp::Reverse(c)));
        let Some(cell) = cell else {
            break;
        };
        let (x, y) = xy(cell);
        if play(&mut scratch, Command::Train { level: 1, x, y }, &mut out) {
            free.clear(cell);
            made += 1;
            if settings.reserve {
                reserved |= path_gains(&scratch, &layers, gain, cell, layers.len() - 1);
            }
        }
    }
    // The payback rule's other trainings, on the free cell with most free
    // cells next to it.
    if settings.fill && settings.buy == Purchase::Payback {
        while made < trainings && scratch.gold(me) >= UNIT_COST {
            let cell = (trainable(&scratch) & free).iter().max_by_key(|&c| {
                (
                    open(c, free),
                    std::cmp::Reverse(theirs[c]),
                    std::cmp::Reverse(c),
                )
            });
            let Some(cell) = cell else {
                break;
            };
            let (x, y) = xy(cell);
            if !play(&mut scratch, Command::Train { level: 1, x, y }, &mut out) {
                break;
            }
            free.clear(cell);
            made += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests;
