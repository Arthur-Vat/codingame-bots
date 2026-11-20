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

use crate::assign;
use crate::econ::{self, Buy, Params};
use crate::plan::{side_key, Step, BUYS};

/// The weight sets a plan combines, turn by turn: v003's weights with every
/// combination of the shape terms `group` (0, 2, 4) and `thin` (0, 3, 6) and
/// of the pull `toward` the enemy HQ (0, 1, 2), with the `inside` penalty 4
/// whenever a shape term is on. Index `9 g + 3 t + w`.
pub const SETS: [Params; 27] = sets();

/// The set a plan plays after its last step: group 2, thin 6, toward 1.
pub const REST: Step = Step {
    weights: 16,
    buy: crate::econ::Buy::Payback,
};

const fn sets() -> [Params; 27] {
    let group = [0, 2, 4];
    let thin = [0, 3, 6];
    let toward = [0, 1, 2];
    let mut out = [Params::V003; 27];
    let mut i = 0;
    while i < 27 {
        let (g, t, w) = (i / 9, (i / 3) % 3, i % 3);
        let inside = if g > 0 || t > 0 { 4 } else { 0 };
        out[i] = Params::V003
            .with_shape(group[g], thin[t], 0)
            .with_course(inside, toward[w]);
        i += 1;
    }
    out
}

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
    /// Non-void cells nearer our active cells than the enemy's (a tie counts
    /// half, rounded down): the land we are likely to get.
    pub voronoi: i64,
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
    pub voronoi: i64,
    /// Each middle choke held (E072; `ice_and_fire_early`'s zones).
    pub chokes: i64,
    /// Whether the plans' steps include the joint frontier assignment
    /// (E072, [`crate::assign`]), with the payback rule and spending all
    /// the gold.
    pub assign: bool,
    /// How plans are compared (E073).
    pub order: Order,
}

/// How the beam compares two positions (E073): by the weighted sum alone,
/// or lexicographically with the middle of the map as tie-breaks, so that,
/// among the many plans that reach the most land, it keeps those that hold
/// the middle without giving up land for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Order {
    /// The weighted sum [`value`] alone.
    Sum,
    /// The weighted sum in steps of `within` (1: exact ties only), then the
    /// middle chokes held, then the cells of the middle band held, then the
    /// weighted sum.
    TieBreak { within: i64 },
    /// The active cells, then the middle chokes held, then the cells of the
    /// middle band held, then the weighted sum.
    LandFirst,
    /// The active cells, then the sum of their weights by their ring from
    /// the middle ([`ring`]: the central cells weigh most), then the
    /// weighted sum (the owner's evaluation, 2026-11-20).
    Central(Decay),
    /// The active cells, then the least total distance from them to the
    /// frontier's cells (ring 0: even between the HQs), each distance at
    /// most `cap`, then the weighted sum: near the whole frontier, not one
    /// flank (the owner, 2026-11-20).
    Frontier { cap: i64 },
    /// The active cells, then the active cells on ring 0, then on ring 1,
    /// and so on up to ring 6, compared in that order, then the weighted
    /// sum.
    ByRing,
    /// The active cells, then the sum of their weights in a static weight
    /// map of the map, computed at turn 1 ([`Field`]), then the weighted sum.
    Field(Field),
    /// The active cells, then the Voronoi share (twice it: a tie counts 1,
    /// [`crate::score::voronoi_halves`]), then the weighted sum (the owner,
    /// 2026-11-20).
    Voronoi,
    /// The active cells, then twice the non-void cells behind the
    /// frontline that are nearer our land than the frontline (a tie counts
    /// 1, [`behind_front`]), then the weighted sum (the owner, 2026-11-20).
    BehindFront,
    /// The owner's three goals (2026-11-20), against an enemy assumed to
    /// hold as many cells as our active land, the nearest to its HQ: first
    /// the active cells (`moves`: the cells taken by unit moves, active
    /// cells less units), then [`Claim`]'s two shares in the order of
    /// `future_first`, then the weighted sum.
    Claim {
        band: i64,
        future_first: bool,
        moves: bool,
    },
}

/// What a side would claim against an enemy holding as many cells as its
/// active land, those nearest the enemy HQ by steps (all cells at the last
/// distance included): twice the cells nearer its active land than that
/// enemy land (a tie counts 1), among the cells within `band` rings of the
/// frontline (`contested`) and among all cells (`future`). The enemy's land
/// depends only on the map and the number of our cells, so plans with as
/// many cells meet the same enemy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Claim {
    pub contested: i64,
    pub future: i64,
}

/// [`Claim`] of `seat` in `state`, `rings` from [`rings`].
pub fn claim(state: &State, seat: usize, rings: &[i64; CELLS], band: i64) -> Claim {
    let active = state.active(seat);
    let from_hq = econ::distances(state, Bits::cell(HQ[1 - seat]));
    let mut steps: Vec<u8> = state
        .map()
        .valid
        .iter()
        .map(|c| from_hq[c])
        .filter(|&d| d != u8::MAX)
        .collect();
    steps.sort_unstable();
    let n = (active.count() as usize).clamp(1, steps.len().max(1));
    let radius = steps.get(n - 1).copied().unwrap_or(0);
    let mut enemy = Bits::EMPTY;
    for c in state.map().valid.iter() {
        if from_hq[c] <= radius {
            enemy.set(c);
        }
    }
    let ours = econ::distances(state, active);
    let theirs = econ::distances(state, enemy);
    let mut out = Claim::default();
    for c in state.map().valid.iter() {
        if ours[c] == u8::MAX {
            continue;
        }
        let v = match ours[c].cmp(&theirs[c]) {
            std::cmp::Ordering::Less => 2,
            std::cmp::Ordering::Equal => 1,
            std::cmp::Ordering::Greater => 0,
        };
        out.future += v;
        if rings[c] <= band {
            out.contested += v;
        }
    }
    out
}

/// For each cell on `seat`'s side of the frontline (nearer its HQ than the
/// enemy's, or as near: the frontline itself), its steps to the frontline;
/// `u8::MAX` for the other cells. Fixed from turn 1.
pub fn to_front(state: &State, seat: usize) -> [u8; CELLS] {
    let ours = econ::distances(state, Bits::cell(HQ[seat]));
    let theirs = econ::distances(state, Bits::cell(HQ[1 - seat]));
    let mut front = Bits::EMPTY;
    let mut side = Bits::EMPTY;
    for c in state.map().valid.iter() {
        if ours[c] == u8::MAX || theirs[c] == u8::MAX {
            continue;
        }
        if ours[c] == theirs[c] {
            front.set(c);
        }
        if ours[c] <= theirs[c] {
            side.set(c);
        }
    }
    let d = econ::distances(state, front);
    let mut out = [u8::MAX; CELLS];
    for c in side.iter() {
        out[c] = d[c];
    }
    out
}

/// Twice the cells behind `seat`'s frontline nearer its active land than
/// the frontline (`front`: [`to_front`]), a tie counting 1.
pub fn behind_front(state: &State, seat: usize, front: &[u8; CELLS]) -> i64 {
    let d = econ::distances(state, state.active(seat));
    (0..CELLS)
        .filter(|&c| front[c] != u8::MAX && d[c] != u8::MAX)
        .map(|c| match d[c].cmp(&front[c]) {
            std::cmp::Ordering::Less => 2,
            std::cmp::Ordering::Equal => 1,
            std::cmp::Ordering::Greater => 0,
        })
        .sum()
}

/// A static weight map, from each cell's walking steps `a` from our HQ and
/// `b` from the enemy's, `d` the steps between the HQs, `ring` = |a − b| / 2
/// (the owner's request, 2026-11-20: computed at turn 1, closer to where
/// contact happens).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Field {
    /// `top − ring − excess / 2`, at least 0, with `excess = a + b − d` the
    /// detour of the cell off the shortest paths between the HQs: the
    /// meeting point weighs most, the far ends of the even line less.
    Corridor { top: i64 },
    /// `top − ring`, at least 0, for cells within `steps` of our HQ; 0
    /// beyond: only the frontier the opening can reach.
    Reachable { top: i64, steps: i64 },
    /// `min(a, d / 2)`: farther from our HQ is better, up to halfway.
    Outward,
}

impl Field {
    /// The weight of a cell `a` steps from our HQ and `b` from the enemy's,
    /// the HQs `d` steps apart.
    pub fn weight(self, a: i64, b: i64, d: i64) -> i64 {
        let ring = (a - b).abs() / 2;
        match self {
            Field::Corridor { top } => (top - ring - (a + b - d) / 2).max(0),
            Field::Reachable { top, steps } if a <= steps => (top - ring).max(0),
            Field::Reachable { .. } => 0,
            Field::Outward => a.min(d / 2),
        }
    }
}

/// Each cell's weight in `field` for `seat`; 0 where an HQ is not reachable.
pub fn field_weights(state: &State, seat: usize, field: Field) -> [i64; CELLS] {
    let ours = econ::distances(state, Bits::cell(HQ[seat]));
    let theirs = econ::distances(state, Bits::cell(HQ[1 - seat]));
    let d = i64::from(ours[HQ[1 - seat]]);
    let mut out = [0; CELLS];
    for c in 0..CELLS {
        if ours[c] != u8::MAX && theirs[c] != u8::MAX {
            out[c] = field.weight(i64::from(ours[c]), i64::from(theirs[c]), d);
        }
    }
    out
}

/// How a cell's weight falls with its ring from the middle (`Order::Central`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decay {
    /// `top - ring`, at least 0.
    Linear(i64),
    /// `2^(top - ring)`, 0 beyond `top` rings.
    Halving(i64),
    /// `(top - ring)²`, 0 beyond `top` rings.
    Square(i64),
}

impl Decay {
    /// The weight of a cell `ring` rings from the middle.
    pub fn weight(self, ring: i64) -> i64 {
        match self {
            Decay::Linear(top) => (top - ring).max(0),
            Decay::Halving(top) if ring <= top => 1 << (top - ring).min(40),
            Decay::Halving(_) => 0,
            Decay::Square(top) => (top - ring).max(0).pow(2),
        }
    }
}

/// A position's rank by an [`Order`]: larger is better, compared
/// lexicographically.
pub type Key = [i64; 4];

impl Objective {
    /// A cell is worth 4, a turn of income 3 and a gold 1, as in the shared
    /// evaluation (gold weighted 2 or 3 played worse: E010); each cell a single capture would cut off costs 2, each joint 2 and
    /// each end of a line 2; each step still to go toward the enemy HQ 6
    /// (with 3 the plans ended a step further from it than the rules', with
    /// 10 their shape began to suffer: E010); each cell of the land nearer
    /// us than the enemy 1, so that the territory grows toward the middle
    /// rather than along an edge.
    pub const DEFAULT: Objective = Objective {
        cells: 4,
        income: 3,
        gold: 1,
        cut: 2,
        joints: 2,
        tips: 2,
        reach: 6,
        voronoi: 1,
        chokes: 0,
        assign: false,
        order: Order::Sum,
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
        voronoi: crate::score::voronoi_halves(state, seat) / 2,
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
    value_with(state, seat, objective, Bits::EMPTY)
}

/// [`value`] with the map's middle chokes `middle` for its `chokes` term.
pub fn value_with(state: &State, seat: usize, objective: &Objective, middle: Bits) -> i64 {
    let held = i64::from((state.owned(seat) & middle).count());
    let s = shape(state, seat);
    let o = objective;
    o.cells * s.cells + o.income * s.income + o.gold * s.gold
        - o.cut * s.cut
        - o.joints * s.joints
        - o.tips * s.tips
        - o.reach * s.reach.min(30)
        + o.voronoi * s.voronoi
        + o.chokes * held
}

/// The cells of the middle band of `state`'s map: those whose steps to the
/// two HQs differ by at most 2 (as the optimal plans' study counts them,
/// `examples/oracle.rs`).
pub fn middle_band(state: &State) -> Bits {
    let ours = econ::distances(state, Bits::cell(HQ[0]));
    let theirs = econ::distances(state, Bits::cell(HQ[1]));
    let mut band = Bits::EMPTY;
    for c in state.map().valid.iter() {
        if ours[c] != u8::MAX && theirs[c] != u8::MAX && ours[c].abs_diff(theirs[c]) <= 2 {
            band.set(c);
        }
    }
    band
}

/// For each cell, its ring from the middle: half of the difference of its
/// steps to the two HQs (always even on this grid, since its two colours
/// alternate), so that each step away from the cells even between the HQs
/// adds a ring; `i64::MAX` for cells one HQ cannot reach.
pub fn rings(state: &State) -> [i64; CELLS] {
    let a = econ::distances(state, Bits::cell(HQ[0]));
    let b = econ::distances(state, Bits::cell(HQ[1]));
    let mut out = [i64::MAX; CELLS];
    for c in 0..CELLS {
        if a[c] != u8::MAX && b[c] != u8::MAX {
            out[c] = i64::from(a[c].abs_diff(b[c])) / 2;
        }
    }
    out
}

/// The zones of `state`'s map that `objective` needs: its middle chokes and
/// its middle band.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Zones {
    pub chokes: Bits,
    pub band: Bits,
    /// Each cell's weight by `Order::Central` (0 for other orders).
    pub weights: [i64; CELLS],
    /// Each cell's ring ([`rings`]) for `Order::Frontier` and
    /// `Order::ByRing` (`i64::MAX` for other orders).
    pub rings: [i64; CELLS],
    /// The frontier's cells (ring 0) for `Order::Frontier`.
    pub frontier: Bits,
    /// Steps to the frontline behind it for `Order::BehindFront`
    /// ([`to_front`]).
    pub front: [u8; CELLS],
}

impl Default for Zones {
    fn default() -> Zones {
        Zones {
            chokes: Bits::EMPTY,
            band: Bits::EMPTY,
            weights: [0; CELLS],
            rings: [i64::MAX; CELLS],
            frontier: Bits::EMPTY,
            front: [u8::MAX; CELLS],
        }
    }
}

impl Zones {
    /// The zones of `state`'s map that `objective` uses, empty otherwise.
    pub fn of(state: &State, objective: &Objective) -> Zones {
        let lexicographic = matches!(objective.order, Order::TieBreak { .. } | Order::LandFirst);
        let mut weights = [0; CELLS];
        if let Order::Central(decay) = objective.order {
            for (w, ring) in weights.iter_mut().zip(rings(state)) {
                *w = if ring == i64::MAX {
                    0
                } else {
                    decay.weight(ring)
                };
            }
        }
        if let Order::Field(field) = objective.order {
            weights = field_weights(state, state.to_act(), field);
        }
        let (ring_of, frontier) = match objective.order {
            Order::Frontier { .. } | Order::ByRing | Order::Claim { .. } => {
                let r = rings(state);
                let mut f = Bits::EMPTY;
                for (c, &ring) in r.iter().enumerate() {
                    if ring == 0 {
                        f.set(c);
                    }
                }
                (r, f)
            }
            _ => ([i64::MAX; CELLS], Bits::EMPTY),
        };
        Zones {
            weights,
            rings: ring_of,
            frontier,
            front: if objective.order == Order::BehindFront {
                to_front(state, state.to_act())
            } else {
                [u8::MAX; CELLS]
            },
            chokes: if objective.chokes != 0 || lexicographic {
                ice_and_fire_early::zones::Zones::of(state).middle_chokes()
            } else {
                Bits::EMPTY
            },
            band: if lexicographic {
                middle_band(state)
            } else {
                Bits::EMPTY
            },
        }
    }
}

/// The rank of `seat`'s position in `state` by `objective` and its
/// [`Order`].
pub fn key(state: &State, seat: usize, objective: &Objective, zones: Zones) -> Key {
    let sum = value_with(state, seat, objective, zones.chokes);
    match objective.order {
        Order::Sum => [sum, 0, 0, 0],
        Order::Central(_) | Order::Field(_) => {
            let active = state.active(seat);
            let weighed: i64 = active.iter().map(|c| zones.weights[c]).sum();
            [i64::from(active.count()), weighed, sum, 0]
        }
        Order::Frontier { cap } => {
            let active = state.active(seat);
            let d = econ::distances(state, active);
            let far: i64 = zones
                .frontier
                .iter()
                .map(|c| i64::from(d[c]).min(cap))
                .sum();
            [i64::from(active.count()), -far, sum, 0]
        }
        Order::Claim {
            band,
            future_first,
            moves,
        } => {
            let active = i64::from(state.active(seat).count());
            let first = if moves {
                active - i64::from(state.units(seat).count())
            } else {
                active
            };
            let c = claim(state, seat, &zones.rings, band);
            if future_first {
                [first, c.future, c.contested, sum]
            } else {
                [first, c.contested, c.future, sum]
            }
        }
        Order::BehindFront => [
            i64::from(state.active(seat).count()),
            behind_front(state, seat, &zones.front),
            sum,
            0,
        ],
        Order::Voronoi => [
            i64::from(state.active(seat).count()),
            crate::score::voronoi_halves(state, seat),
            sum,
            0,
        ],
        Order::ByRing => {
            let active = state.active(seat);
            // Seven counts of at most 144, 8 bits each, ring 0 highest.
            let mut packed = 0i64;
            for ring in 0..7 {
                let n = active.iter().filter(|&c| zones.rings[c] == ring).count();
                packed = (packed << 8) | n as i64;
            }
            [i64::from(active.count()), packed, sum, 0]
        }
        Order::TieBreak { .. } | Order::LandFirst => {
            let owned = state.owned(seat);
            let chokes = i64::from((owned & zones.chokes).count());
            let band = i64::from((owned & zones.band).count());
            match objective.order {
                Order::TieBreak { within } => [sum.div_euclid(within.max(1)), chokes, band, sum],
                _ => [i64::from(state.active(seat).count()), chokes, band, sum],
            }
        }
    }
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

/// The step's weights that stand for the joint frontier assignment (E072),
/// with the first weight set's payback rule: [`Buy::Payback`] buys by it,
/// any other purchase spends all the gold.
pub const ASSIGN: u8 = u8::MAX;

/// The commands of `step` in `state` with the weight `sets`.
pub fn commands(state: &State, sets: &[Params], step: Step) -> Vec<Command> {
    if step.weights == ASSIGN {
        let settings = assign::Settings {
            params: sets.first().copied().unwrap_or(Params::V003),
            buy: if step.buy == Buy::Payback {
                assign::Purchase::Payback
            } else {
                assign::Purchase::Affordable
            },
            ..assign::Settings::DEFAULT
        };
        return assign::turn(state, &settings);
    }
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

/// The distinct steps tried in `state`: every weight set with every
/// purchase of [`BUYS`]; steps whose commands repeat an earlier one's are
/// left out.
fn steps(state: &State, sets: &[Params], assign: bool) -> Vec<(Step, Vec<Command>)> {
    let mut out: Vec<(Step, Vec<Command>)> = Vec::new();
    if assign {
        for buy in [
            Buy::Payback,
            Buy::Exact {
                mines: 0,
                units: u8::MAX,
            },
        ] {
            let step = Step {
                weights: ASSIGN,
                buy,
            };
            let c = commands(state, sets, step);
            if !out.iter().any(|(_, other)| *other == c) {
                out.push((step, c));
            }
        }
    }
    for weights in 0..sets.len().min(usize::from(u8::MAX)) {
        for &buy in &BUYS {
            let step = Step {
                weights: weights as u8,
                buy,
            };
            let c = commands(state, sets, step);
            if !out.iter().any(|(_, other)| *other == c) {
                out.push((step, c));
            }
        }
    }
    out
}

/// A beam search over whole turns for `turns` of our turns, `width` nodes a
/// turn, the opponent passing: the plan whose final position [`key`]
/// ranks best, the plan of `rest` alone among the candidates. Stops early,
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
    let zones = Zones::of(state, objective);
    let rated = |plan: &[Step]| key(&play(state, sets, plan, rest, turns), me, objective, zones);
    let mut best: (Key, Vec<Step>) = (rated(&[]), Vec::new());
    let mut frontier: Vec<(State, Vec<Step>)> = vec![(state.clone(), Vec::new())];
    for depth in 0..turns {
        let mut children: Vec<(Key, State, Vec<Step>)> = Vec::new();
        let mut seen: HashMap<u64, usize> = HashMap::new();
        for (node, plan) in &frontier {
            if out_of_time() {
                break;
            }
            for (step, c) in steps(node, sets, objective.assign) {
                let next = advance(node, &c);
                let mut child = plan.clone();
                child.push(step);
                let v = key(&next, me, objective, zones);
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
        // played on with `rest`; past the deadline, the top node only.
        for (i, (_, _, plan)) in children
            .iter()
            .take(if last { width } else { 1 })
            .enumerate()
        {
            if i > 0 && out_of_time() {
                break;
            }
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

/// An anytime plan: [`beam`] with a width of 2, then 4, 8 and so on, the
/// best plan of all kept, until `deadline` (each search stops there too) or,
/// without one, after the width `max_width`. With a deadline the search uses
/// the time it is given.
pub fn plan(
    state: &State,
    sets: &[Params],
    rest: Step,
    turns: usize,
    max_width: usize,
    deadline: Option<Instant>,
    objective: &Objective,
) -> Vec<Step> {
    let me = state.to_act();
    let zones = Zones::of(state, objective);
    let rated = |plan: &[Step]| key(&play(state, sets, plan, rest, turns), me, objective, zones);
    let mut best: (Key, Vec<Step>) = (rated(&[]), Vec::new());
    let mut width = 2;
    loop {
        let found = beam(state, sets, rest, turns, width, deadline, objective);
        let v = rated(&found);
        if v > best.0 {
            best = (v, found);
        }
        let out_of_time = deadline.is_some_and(|t| Instant::now() >= t);
        if out_of_time || (deadline.is_none() && width >= max_width) {
            break;
        }
        width *= 2;
    }
    best.1
}

#[cfg(test)]
mod tests;
