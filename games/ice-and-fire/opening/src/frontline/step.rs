//! The opening by roles (Step 7 of the frontline plan, E080): one turn in
//! which our units walk to targets, the turn's trainings go at the tips of
//! our land nearest their targets, after the moves, and, with
//! [`Settings::fill`], the rest of the turn (units with no target, gold
//! left) follows the `econ` opening's rules.
//!
//! The targets ([`Targets`]) are what the variants change:
//! - [`Targets::Value`]: the objectives by the land each gains, given out by
//!   [`super::roles::assign`] (greedy, by value then arrival);
//! - [`Targets::Matched`]: the same objectives, the most valuable first,
//!   matched to units and trainings so that the total arrival is least
//!   (the Hungarian method), so that two units do not cross to each
//!   other's target;
//! - [`Targets::Frontier`]: E072's targets (the middle chokes, then cells
//!   along the Voronoi frontier spaced apart), matched the same way;
//! - [`Targets::Marginal`]: objectives chosen one at a time by the land
//!   each adds to what the ones already chosen decide (objectives overlap,
//!   so their values do not add up), then matched.
//!
//! An agent (a unit, or a training of this turn) takes a target only where
//! it arrives no later than the enemy's land (race margin 0 or more).
//! Every command is checked on a scratch copy, in order.

use ice_and_fire_engine::{Bits, Command, State, SIZE};

use super::objectives::{counted, objectives, Objective};
use super::race::race;
use super::roles::{assign, Memory};
use crate::assign::hungarian;
use crate::econ::{self, distances, Buy, Params};

/// The gold of a level-1 unit.
const TRAIN_COST: i64 = 10;
/// The cost of an assignment the race forbids.
const FORBIDDEN: i64 = 1_000_000;

/// How the targets are chosen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Targets {
    Value,
    Matched,
    Frontier,
    Marginal,
}

/// The settings of the step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub targets: Targets,
    /// Whether the rest of the turn follows the `econ` rules (the step's
    /// weights and the payback rule); otherwise units with no target wait
    /// and the gold no target takes is kept.
    pub fill: bool,
}

impl Settings {
    /// The step's code within a plan's `Step::weights` (`solo::ROLES` and
    /// up): the targets in the low two bits, `fill` in the third.
    pub const fn code(self) -> u8 {
        let t = match self.targets {
            Targets::Value => 0,
            Targets::Matched => 1,
            Targets::Frontier => 2,
            Targets::Marginal => 3,
        };
        t | if self.fill { 4 } else { 0 }
    }

    /// The settings of a [`Settings::code`].
    pub const fn of_code(code: u8) -> Settings {
        Settings {
            targets: match code & 3 {
                0 => Targets::Value,
                1 => Targets::Matched,
                2 => Targets::Frontier,
                _ => Targets::Marginal,
            },
            fill: code & 4 != 0,
        }
    }
}

fn xy(cell: usize) -> (u8, u8) {
    ((cell % SIZE) as u8, (cell / SIZE) as u8)
}

/// Applies `command` on `scratch` and adds it to `out` if the rules accept
/// it; whether they did.
fn accepted(scratch: &mut State, out: &mut Vec<Command>, command: Command) -> bool {
    let ok = scratch.apply(command);
    if ok {
        out.push(command);
    }
    ok
}

/// The cells next to `from` one step nearer to `goal`, the cells not ours
/// first (a step that takes land), then by cell.
fn steps_toward(state: &State, seat: usize, from: usize, goal: usize) -> Vec<usize> {
    let map = state.map();
    let here = map.distance(from, goal);
    if here == u8::MAX || here == 0 {
        return Vec::new();
    }
    let mut out: Vec<usize> = (Bits::cell(from).neighbours() & map.valid)
        .iter()
        .filter(|&n| map.distance(n, goal) < here)
        .collect();
    out.sort_by_key(|&n| (state.owned(seat).test(n), n));
    out
}

/// What the agents of a turn are to do: units (cell) to targets, and
/// trainings to targets.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Orders {
    pub units: Vec<(usize, usize)>,
    pub trainings: Vec<usize>,
}

/// The steps for the enemy's land to reach each cell, its trainings
/// included (a cell next to its land is one turn away).
fn enemy_arrival(state: &State, seat: usize) -> [u8; ice_and_fire_engine::CELLS] {
    distances(state, state.active(1 - seat))
}

/// The open objectives of `seat`: counted, not held, and won or tied.
fn open_objectives(state: &State, seat: usize) -> Vec<Objective> {
    let all = objectives(state, seat);
    let races = race(state, seat, &all);
    counted(&all)
        .into_iter()
        .filter(|o| {
            !o.held
                && races
                    .iter()
                    .any(|r| r.cell == o.cell && r.ours.is_some() && r.margin >= 0)
        })
        .collect()
}

/// Objectives picked one at a time by the cells each adds, nearer our land
/// plus the picks than the enemy's, at most `most`, while one adds any.
fn marginal(state: &State, seat: usize, open: &[Objective], most: usize) -> Vec<usize> {
    let map = state.map();
    let theirs = distances(state, state.active(1 - seat));
    let mut best = distances(state, state.active(seat));
    let count = |d: &[u8; ice_and_fire_engine::CELLS]| {
        map.valid
            .iter()
            .filter(|&c| d[c] != u8::MAX && d[c] < theirs[c])
            .count() as i64
    };
    let each: Vec<(usize, [u8; ice_and_fire_engine::CELLS])> = open
        .iter()
        .map(|o| (o.cell, distances(state, Bits::cell(o.cell))))
        .collect();
    let mut picks = Vec::new();
    let mut base = count(&best);
    while picks.len() < most {
        let mut top: Option<(i64, usize)> = None;
        for (i, (cell, d)) in each.iter().enumerate() {
            if picks.contains(cell) {
                continue;
            }
            let mut merged = best;
            for (m, &x) in merged.iter_mut().zip(d.iter()) {
                *m = (*m).min(x);
            }
            let gain = count(&merged) - base;
            if gain > 0 && top.is_none_or(|(g, _)| gain > g) {
                top = Some((gain, i));
            }
        }
        let Some((gain, i)) = top else {
            break;
        };
        picks.push(each[i].0);
        for (b, &x) in best.iter_mut().zip(each[i].1.iter()) {
            *b = (*b).min(x);
        }
        base += gain;
    }
    picks
}

/// Matches our movable units and the trainings the gold pays to `targets`
/// so that the total arrival (a unit in its steps, a training in one turn
/// plus the steps from our nearest trainable cell) is least, each agent
/// arriving no later than the enemy's land.
fn matched(state: &State, seat: usize, targets: &[usize]) -> Orders {
    let map = state.map();
    if targets.is_empty() {
        return Orders::default();
    }
    let theirs = enemy_arrival(state, seat);
    let units: Vec<usize> = state.movable_units().iter().collect();
    let trainable = state.active(seat).neighbours() & map.valid & state.enterable(seat, 1);
    let paid = usize::try_from(state.gold(seat) / TRAIN_COST).unwrap_or(0);
    let trainings = paid.min(targets.len());
    let agents = units.len() + trainings;
    let n = agents.max(targets.len());
    let mut cost = vec![vec![0i64; n]; n];
    for (a, row) in cost.iter_mut().enumerate().take(agents) {
        for (t, &target) in targets.iter().enumerate() {
            let arrival = if a < units.len() {
                i64::from(map.distance(units[a], target))
            } else {
                trainable
                    .iter()
                    .map(|c| i64::from(map.distance(c, target)))
                    .min()
                    .map_or(i64::from(u8::MAX), |d| d + 1)
            };
            let enemy = i64::from(theirs[target]);
            row[t] = if arrival >= i64::from(u8::MAX) || arrival > enemy {
                FORBIDDEN
            } else {
                arrival
            };
        }
        // Columns beyond the targets: no target, which costs more than any
        // allowed one, so that agents take targets first.
        for col in row.iter_mut().skip(targets.len()) {
            *col = FORBIDDEN / 2;
        }
    }
    let pick = hungarian(&cost);
    let mut out = Orders::default();
    for (a, &t) in pick.iter().enumerate().take(agents) {
        if t >= targets.len() || cost[a][t] >= FORBIDDEN / 2 {
            continue;
        }
        if a < units.len() {
            out.units.push((units[a], targets[t]));
        } else {
            out.trainings.push(targets[t]);
        }
    }
    out
}

/// The orders of the turn of the player to act in `state` by `settings`.
pub fn orders(state: &State, settings: Settings) -> Orders {
    let seat = state.to_act();
    let open = open_objectives(state, seat);
    // Enough targets for every agent, and a few to choose among.
    let most = state.movable_units().count() as usize
        + usize::try_from(state.gold(seat) / TRAIN_COST).unwrap_or(0)
        + 2;
    match settings.targets {
        Targets::Value => {
            let all = objectives(state, seat);
            let races = race(state, seat, &all);
            let roles = assign(state, seat, &all, &races, &mut Memory::default());
            let units = roles
                .frontline
                .iter()
                .filter_map(|&(id, goal)| {
                    state
                        .units(seat)
                        .iter()
                        .find(|&c| state.unit_id(c) == Some(id))
                        .map(|c| (c, goal))
                })
                .collect();
            Orders {
                units,
                trainings: roles.trainings.iter().map(|&(_, goal)| goal).collect(),
            }
        }
        Targets::Matched => {
            let targets: Vec<usize> = open.iter().take(most).map(|o| o.cell).collect();
            matched(state, seat, &targets)
        }
        Targets::Frontier => {
            let zones = ice_and_fire_early::zones::Zones::of(state);
            let targets =
                crate::assign::targets(state, &zones, &crate::assign::Settings::DEFAULT, most);
            matched(state, seat, &targets)
        }
        Targets::Marginal => {
            let targets = marginal(state, seat, &open, most);
            matched(state, seat, &targets)
        }
    }
}

/// The commands of the turn of the player to act in `state`: the orders of
/// `settings` (units step toward their targets, then each training on our
/// trainable cell nearest its target, after the moves, so at the tip), then
/// with `fill` the `econ` turn of `params` with the payback rule.
pub fn turn(state: &State, settings: Settings, params: &Params) -> Vec<Command> {
    carry_out(state, &orders(state, settings), settings.fill, params)
}

/// The commands that carry out `plan` in `state` (see [`turn`]).
pub fn carry_out(state: &State, plan: &Orders, fill: bool, params: &Params) -> Vec<Command> {
    let seat = state.to_act();
    let map = state.map();
    let mut scratch = state.clone();
    let mut out = Vec::new();
    for &(from, goal) in &plan.units {
        let Some(id) = scratch.unit_id(from) else {
            continue;
        };
        if !scratch.movable_units().test(from) {
            continue;
        }
        for to in steps_toward(&scratch, seat, from, goal) {
            let (x, y) = xy(to);
            if accepted(&mut scratch, &mut out, Command::Move { id, x, y }) {
                break;
            }
        }
    }
    for &goal in &plan.trainings {
        if scratch.gold(seat) < TRAIN_COST {
            break;
        }
        let trainable = scratch.active(seat).neighbours() & map.valid;
        let mut cells: Vec<usize> = trainable
            .iter()
            .filter(|&c| map.distance(c, goal) != u8::MAX)
            .collect();
        cells.sort_by_key(|&c| (map.distance(c, goal), c));
        for to in cells {
            let (x, y) = xy(to);
            if accepted(&mut scratch, &mut out, Command::Train { level: 1, x, y }) {
                break;
            }
        }
    }
    if fill {
        for command in econ::turn_with(&scratch, params, Buy::Payback) {
            accepted(&mut scratch, &mut out, command);
        }
    }
    out
}

#[cfg(test)]
mod tests;
