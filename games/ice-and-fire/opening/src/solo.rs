//! One player alone (E010): the opening planned for our side only, the
//! opponent passing, up to the end of the early game.
//!
//! Before the territories meet, the opponent hardly changes what our side can
//! build, so the early game is planned as if we were alone. A plan is a list
//! of [`Step`]s, one a turn (the opening's weights and how it buys), found by
//! a beam search over whole turns ([`beam`]) and judged by the position it
//! reaches ([`value`]): territory first, then income and gold, less the
//! weaknesses of its shape (cells a single capture would cut off, joints,
//! the ends of lines), plus its progress toward the enemy HQ.

use std::collections::HashMap;
use std::time::Instant;

use ice_and_fire_engine::{Bits, Command, State, CELLS, HQ};

use crate::econ::{self, Params};
use crate::plan::{side_key, Step, BUYS};

/// How a side stands at the end of the early game.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Shape {
    /// Active cells.
    pub cells: i64,
    pub income: i64,
    pub gold: i64,
    /// Active cells whose loss cuts other cells off from the HQ.
    pub joints: i64,
    /// The most cells one such loss cuts off.
    pub cut: i64,
    /// Active cells other than the HQ with at most one own active neighbour:
    /// the ends of lines.
    pub tips: i64,
    /// Steps from our active cells to the enemy HQ.
    pub reach: i64,
}

/// The weights of [`value`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Objective {
    pub cells: i64,
    pub income: i64,
    pub gold: i64,
    pub cut: i64,
    pub joints: i64,
    pub tips: i64,
    pub reach: i64,
}

impl Objective {
    /// A cell is worth 4 and a turn of income 3, as in the shared evaluation;
    /// each cell a single capture would cut off costs 2, each joint 2 and
    /// each end of a line 2; each step still to go toward the enemy HQ 6
    /// (with 3 the plans ended a step further from it than the rules', with
    /// 10 their shape began to suffer: E010).
    pub const DEFAULT: Objective = Objective {
        cells: 4,
        income: 3,
        gold: 1,
        cut: 2,
        joints: 2,
        tips: 2,
        reach: 6,
    };
}

/// For each active cell of `seat`, the cells its loss would cut off from the
/// HQ (0 for a cell that is no joint, and for the HQ).
pub fn severed(state: &State, seat: usize) -> [i64; CELLS] {
    struct Dfs {
        active: Bits,
        disc: [u16; CELLS],
        low: [u16; CELLS],
        size: [i64; CELLS],
        cut: [i64; CELLS],
        time: u16,
    }
    fn visit(d: &mut Dfs, v: usize) {
        d.time += 1;
        d.disc[v] = d.time;
        d.low[v] = d.time;
        d.size[v] = 1;
        for w in (Bits::cell(v).neighbours() & d.active).iter() {
            if d.disc[w] == 0 {
                visit(d, w);
                d.size[v] += d.size[w];
                d.low[v] = d.low[v].min(d.low[w]);
                if d.low[w] >= d.disc[v] {
                    d.cut[v] += d.size[w];
                }
            } else {
                d.low[v] = d.low[v].min(d.disc[w]);
            }
        }
    }
    let mut d = Dfs {
        active: state.active(seat),
        disc: [0; CELLS],
        low: [0; CELLS],
        size: [0; CELLS],
        cut: [0; CELLS],
        time: 0,
    };
    let root = HQ[seat];
    if d.active.test(root) {
        visit(&mut d, root);
    }
    d.cut[root] = 0;
    d.cut
}

/// How `seat` stands in `state`.
pub fn shape(state: &State, seat: usize) -> Shape {
    let active = state.active(seat);
    let cut = severed(state, seat);
    let to_hq = econ::distances(state, Bits::cell(HQ[1 - seat]));
    let mut out = Shape {
        cells: i64::from(active.count()),
        income: state.income(seat),
        gold: state.gold(seat),
        reach: i64::MAX,
        ..Shape::default()
    };
    for cell in active.iter() {
        if cut[cell] > 0 {
            out.joints += 1;
            out.cut = out.cut.max(cut[cell]);
        }
        if cell != HQ[seat] && (Bits::cell(cell).neighbours() & active).count() <= 1 {
            out.tips += 1;
        }
        out.reach = out.reach.min(i64::from(to_hq[cell]));
    }
    out
}

/// The value of `seat`'s position in `state` by `objective`.
pub fn value(state: &State, seat: usize, objective: &Objective) -> i64 {
    let s = shape(state, seat);
    let o = objective;
    o.cells * s.cells + o.income * s.income + o.gold * s.gold
        - o.cut * s.cut
        - o.joints * s.joints
        - o.tips * s.tips
        - o.reach * s.reach.min(30)
}

/// Our `commands`, then the opponent passes: our next start of turn.
pub fn advance(state: &State, commands: &[Command]) -> State {
    let mut next = state.clone();
    next.play_turn(commands);
    if next.outcome().is_none() {
        next.play_turn(&[]);
    }
    next
}

/// The commands of `step` in `state` with the weight `sets`.
pub fn commands(state: &State, sets: &[Params], step: Step) -> Vec<Command> {
    econ::turn_with(
        state,
        &sets[usize::from(step.weights) % sets.len()],
        step.buy,
    )
}

/// The position after `turns` of our turns, playing `plan` and then `rest`.
pub fn play(state: &State, sets: &[Params], plan: &[Step], rest: Step, turns: usize) -> State {
    let mut sim = state.clone();
    for turn in 0..turns {
        if sim.outcome().is_some() {
            break;
        }
        let step = plan.get(turn).copied().unwrap_or(rest);
        sim = advance(&sim, &commands(&sim, sets, step));
    }
    sim
}

/// The distinct steps tried in `state`: each weight set with the payback
/// rule, and the first set with every purchase of [`BUYS`]; steps whose
/// commands repeat an earlier one's are left out.
fn steps(state: &State, sets: &[Params]) -> Vec<(Step, Vec<Command>)> {
    let mut options: Vec<Step> = (0..sets.len().min(usize::from(u8::MAX)))
        .map(|weights| Step {
            weights: weights as u8,
            buy: econ::Buy::Payback,
        })
        .collect();
    options.extend(BUYS.iter().skip(1).map(|&buy| Step { weights: 0, buy }));
    let mut out: Vec<(Step, Vec<Command>)> = Vec::new();
    for step in options {
        let c = commands(state, sets, step);
        if !out.iter().any(|(_, other)| *other == c) {
            out.push((step, c));
        }
    }
    out
}

/// A beam search over whole turns for `turns` of our turns, `width` nodes a
/// turn, the opponent passing: the plan whose final position [`value`]
/// rates best, the plan of `rest` alone among the candidates. Stops early,
/// with the best plan found, at `deadline`.
pub fn beam(
    state: &State,
    sets: &[Params],
    rest: Step,
    turns: usize,
    width: usize,
    deadline: Option<Instant>,
    objective: &Objective,
) -> Vec<Step> {
    let me = state.to_act();
    let out_of_time = || deadline.is_some_and(|t| Instant::now() >= t);
    let rated = |plan: &[Step]| value(&play(state, sets, plan, rest, turns), me, objective);
    let mut best: (i64, Vec<Step>) = (rated(&[]), Vec::new());
    let mut frontier: Vec<(State, Vec<Step>)> = vec![(state.clone(), Vec::new())];
    for depth in 0..turns {
        let mut children: Vec<(i64, State, Vec<Step>)> = Vec::new();
        let mut seen: HashMap<u64, usize> = HashMap::new();
        for (node, plan) in &frontier {
            if out_of_time() {
                break;
            }
            for (step, c) in steps(node, sets) {
                let next = advance(node, &c);
                let mut child = plan.clone();
                child.push(step);
                let v = value(&next, me, objective);
                let key = side_key(&next, me);
                match seen.get(&key) {
                    Some(&at) if children[at].0 >= v => {}
                    Some(&at) => children[at] = (v, next, child),
                    None => {
                        seen.insert(key, children.len());
                        children.push((v, next, child));
                    }
                }
            }
        }
        if children.is_empty() {
            break;
        }
        children.sort_by(|a, b| b.0.cmp(&a.0));
        children.truncate(width);
        let last = depth + 1 == turns || out_of_time();
        // The nodes are rated at the end of the early game: their plan
        // played on with `rest`.
        for (_, _, plan) in children.iter().take(if last { width } else { 1 }) {
            let v = rated(plan);
            if v > best.0 {
                best = (v, plan.clone());
            }
        }
        if last {
            break;
        }
        frontier = children.into_iter().map(|(_, s, p)| (s, p)).collect();
    }
    best.1
}

#[cfg(test)]
mod tests;
