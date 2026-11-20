//! The exact early game of one side alone (E072, the "optimal plans" study
//! of phase 2 of the early-game plan,
//! `games/ice-and-fire/research/codingame/2026-11-19-early-game.md`).
//!
//! One side plays its first `T` turns, the other passing, under the
//! engine's rules: each turn its units step onto a neighbour or stay, then it
//! trains level-1 units on cells next to its active land (no mines, no
//! towers, no level 2 or 3: no opening builds them this early). States are
//! merged when they are the same for the rest of the game (our cells, our
//! units' places, our gold; unit ids do not matter).
//!
//! Every search cuts with one upper bound on the cells a state can still
//! reach: the economy with every unit taking a new cell every turn and the
//! gold spent on trainings as soon as it allows,
//!
//! ```text
//! each turn left: k = gold / 10; cells += units + k; units += k;
//!                 gold += cells − units − 10 k
//! ```
//!
//! A unit move that takes no cell (it stays or walks on our land) loses at
//! least one cell, so such moves are tried only when the bound leaves room.
//! For each start and horizon:
//!
//! 1. **The most cells:** a depth-first search for a plan with one more cell
//!    than the best opening below, again until none exists.
//! 2. **The most middle chokes, and the most cells of the middle band
//!    (within 2 steps of even between the HQs), within 0, 1 and 2 cells of
//!    the most:** branch and bound, a state cut when the zone cells it
//!    holds, plus those its land can still reach (one step a turn for the
//!    units and one for each training the gold can pay, [`reach`]; at most
//!    the cells it can still take), cannot beat the best found.
//! 3. **The optimal end territories,** distinct sets of cells, counted by a
//!    breadth-first enumeration while a layer holds at most `--cap` states;
//!    beyond, the start is reported as having more.
//!
//! 4. **The most central weight at the most cells (E073):** the same branch
//!    and bound with each cell weighted by the linear central weights of the
//!    opening's `Order::Central` (top 5, 7 and 9 less the cell's ring from
//!    the middle, `solo::rings`), the bound taking the heaviest cells within
//!    reach.
//!
//! A depth-first search stops after `--limit` states expanded (20,000 by
//! default: each costs about a millisecond, since it makes every turn of the
//! state) with the best plan found so far, and the result is not `proven`.
//! The searches try the most promising turns first, so a result cut short
//! is a good plan, a lower bound on the optimum. `--check` compares, on short
//! horizons, the pruned searches with a full enumeration without any cut, and
//! asserts the same results.
//!
//! **The plans are kept** (`--plans FILE`): for each start, horizon and goal
//! (the most cells; the most middle chokes, and the most middle cells, within
//! 0, 1 and 2 cells of the most), the best plan found, its value, its cells,
//! whether it is proven, and its commands turn by turn as CodinGame reads
//! them, so that later work need not search again:
//!
//! - `--replay FILE [--show]` plays every stored plan again through the
//!   engine and checks its values (and shows the board it ends on), in
//!   milliseconds;
//! - `--from FILE` starts each search from the stored plans of its start and
//!   horizon, so that a later run (with a larger limit, or a better bound)
//!   only improves them and turns lower bounds into proofs;
//! - `--bench FILE` plays the openings of [`OPENINGS`] on the stored starts
//!   and prints how far each is from the stored best plans: the cells short
//!   of the most, and the middle chokes and middle cells short of the most
//!   at its own cells. A new opening joins [`OPENINGS`] to be measured in
//!   seconds, without solving again.
//!
//! The starts are a suite's (`games/ice-and-fire/openings/`): the reference
//! player's maps and seats. Beside the optimum, the tool plays in the same
//! model v029's opening (its solo planner, untimed), the same with the
//! assignment's steps (`B`), the rules alone and the assignment alone, and
//! gives the reference player's own active cells at the next game turn from
//! the suite (its games are two-sided, so that is a pointer only).
//!
//! ```sh
//! cargo run --release -p ice-and-fire-opening --example oracle -- \
//!     --suite games/ice-and-fire/openings/suite-v1.tsv --player zasmu \
//!     --per-seat 50 --turns 6,7,8 --jobs 4 [--limit 20000] [--cap 20000] [--shard K/N] \
//!     [--csv FILE] [--plans FILE] [--from FILE] [--check]
//! cargo run --release -p ice-and-fire-opening --example oracle -- --replay plans.tsv [--show]
//! cargo run --release -p ice-and-fire-opening --example oracle -- --bench plans.tsv --jobs 4
//! ```

use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::time::Instant;

use ice_and_fire_early::zones::Zones;
use ice_and_fire_engine::{answer_line, Bits, Command, League, State, CELLS, HQ, SIZE};
use ice_and_fire_opening::assign::{self, Purchase};
use ice_and_fire_opening::econ::{distances, Params};
use ice_and_fire_opening::planned::{Planned, Planner};
use ice_and_fire_opening::solo::{self, Decay, Objective, Order};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

/// v029's objective (`OBJECTIVE` in the plan bot).
const OBJECTIVE: Objective = Objective {
    cells: 3,
    income: 1,
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

/// v029's opening weights (`opening_of` in the plan bot).
fn sets() -> Vec<Params> {
    solo::SETS
        .iter()
        .map(|&params| Params {
            safe_spot: u8::MAX,
            crowd: 2,
            horizon: 18,
            after_contact: 0,
            ..params
        })
        .collect()
}

/// v029's opening, untimed, with the `objective`.
fn opening(objective: Objective) -> Planned {
    let planner = Planner::Solo {
        turns: 9,
        width: 32,
        objective,
    };
    Planned::new(planner, sets(), Params::V003)
}

/// The start of `seat`'s first turn on the map of `seed`.
fn start(seed: u64, seat: usize) -> State {
    let map = generate(seed, RefereeLeague::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    if seat == 1 {
        state.play_turn(&[]);
    }
    state
}

/// A plan: our commands, a list a turn.
type Plan = Vec<Vec<Command>>;

/// A state's key for merging: our cells, our units, our gold.
type Key = ([u64; 3], [u64; 3], i64);

fn key(state: &State, me: usize) -> Key {
    (
        state.owned(me).words(),
        state.units(me).words(),
        state.gold(me),
    )
}

/// The most steps `me`'s land can reach beyond its cells in `left` more of
/// its turns from its start of turn in `state`. In one turn a unit steps
/// once from the land it stands on, and each training lands next to the
/// active land, so trainings chained after a unit's step reach one step
/// further each: at most `left` steps for the units and one for each
/// training. The trainings are at most the gold, now and from the most
/// income each later turn can bring (its most cells, [`bound`], upkeep left
/// out), over a unit's cost.
fn reach(state: &State, me: usize, left: u32) -> i64 {
    let income: i64 = (1..left).map(|j| bound(state, me, j)).sum();
    i64::from(left) + (state.gold(me).max(0) + income) / 10
}

/// The most active cells `me` can hold after `left` more of its turns (this
/// one included) from its start of turn in `state`: every unit takes a new
/// cell every turn and the gold buys level-1 units as soon as it allows.
fn bound(state: &State, me: usize, left: u32) -> i64 {
    let mut cells = i64::from(state.active(me).count());
    let mut units = i64::from(state.units(me).count());
    let mut gold = state.gold(me);
    for _ in 0..left {
        let k = (gold / 10).max(0);
        cells += units + k;
        units += k;
        gold += cells - units - 10 * k;
    }
    cells
}

fn xy(cell: usize) -> (u8, u8) {
    ((cell % SIZE) as u8, (cell / SIZE) as u8)
}

/// Every turn of `me` from `state` (moves, then trainings), each passed to
/// `out` as the state after it, before the turn ends, with its commands.
/// Moves that take no cell are tried only with `waste`; a unit with no cell
/// to take may always stay.
fn turns(state: &State, me: usize, waste: bool, out: &mut dyn FnMut(State, &[Command])) {
    let units: Vec<usize> = state.movable_units().iter().collect();
    let mut after_moves = Vec::new();
    moves(
        state.clone(),
        me,
        &units,
        waste,
        Vec::new(),
        &mut after_moves,
    );
    for (s, commands) in after_moves {
        trainings(s, me, None, Bits::EMPTY, commands, out);
    }
}

fn moves(
    state: State,
    me: usize,
    units: &[usize],
    waste: bool,
    commands: Vec<Command>,
    out: &mut Vec<(State, Vec<Command>)>,
) {
    let Some((&from, rest)) = units.split_first() else {
        out.push((state, commands));
        return;
    };
    let Some((id, level)) = state.unit_at(from) else {
        moves(state, me, rest, waste, commands, out);
        return;
    };
    let enter = state.enterable(me, level) & state.map().valid;
    let mut took = false;
    for to in (Bits::cell(from).neighbours() & enter).iter() {
        let capture = !state.owned(me).test(to);
        if !capture && !waste {
            continue;
        }
        let mut next = state.clone();
        let (x, y) = xy(to);
        let command = Command::Move { id, x, y };
        if next.apply(command) {
            took |= capture;
            let mut more = commands.clone();
            more.push(command);
            moves(next, me, rest, waste, more, out);
        }
    }
    if waste || !took {
        moves(state, me, rest, waste, commands, out);
    }
}

/// The level-1 trainings that may follow: any set of trainable cells, each
/// set once (a cell is picked after a higher one only when that one made it
/// trainable).
fn trainings(
    state: State,
    me: usize,
    last: Option<usize>,
    before: Bits,
    commands: Vec<Command>,
    out: &mut dyn FnMut(State, &[Command]),
) {
    out(state.clone(), &commands);
    if state.gold(me) < 10 {
        return;
    }
    let trainable = (state.near_active(me) & state.enterable(me, 1) & state.map().valid)
        .and_not(state.owned(me));
    for cell in trainable.iter() {
        let new = !before.test(cell);
        if last.is_some_and(|l| cell < l) && !new {
            continue;
        }
        let mut next = state.clone();
        let (x, y) = xy(cell);
        let command = Command::Train { level: 1, x, y };
        if next.apply(command) {
            let mut more = commands.clone();
            more.push(command);
            trainings(next, me, Some(cell), trainable, more, out);
        }
    }
}

/// Ends our turn and the passing opponent's: our next start of turn.
fn end(mut state: State) -> State {
    state.play_turn(&[]);
    if state.outcome().is_none() {
        state.play_turn(&[]);
    }
    state
}

/// The set of cells whose words, as [`Bits::words`] stores them, are
/// `words`: 48 cells a word.
fn from_words(words: [u64; 3]) -> Bits {
    let mut bits = Bits::EMPTY;
    for (i, &word) in words.iter().enumerate() {
        let mut w = word;
        while w != 0 {
            bits.set(i * 48 + w.trailing_zeros() as usize);
            w &= w - 1;
        }
    }
    bits
}

/// What the search needs to rebuild a state from its [`Key`]: the map, the
/// enemy's fixed cells, the initial input and our seat.
struct Frame {
    initial: String,
    valid: Bits,
    theirs: Bits,
    their_gold: i64,
    me: usize,
}

impl Frame {
    fn new(state: &State, me: usize) -> Frame {
        Frame {
            initial: state.initial_input(me),
            valid: state.map().valid,
            theirs: state.owned(1 - me),
            their_gold: state.gold(1 - me),
            me,
        }
    }

    /// The state at our start of `game_turn` with the cells, units and gold
    /// of `key`, all our units free to move. All our cells are active: a
    /// side alone only ever takes cells next to its active land.
    fn rebuild(&self, key: &Key, game_turn: u32) -> State {
        let ours = from_words(key.0);
        let units = from_words(key.1);
        let income = i64::from(ours.count()) - i64::from(units.count());
        let mut text = format!("{}\n{}\n{}\n0\n", key.2, income, self.their_gold);
        for y in 0..SIZE {
            for x in 0..SIZE {
                let c = y * SIZE + x;
                text.push(if !self.valid.test(c) {
                    '#'
                } else if ours.test(c) {
                    'O'
                } else if self.theirs.test(c) {
                    'X'
                } else {
                    '.'
                });
            }
            text.push('\n');
        }
        let (mx, my) = xy(HQ[self.me]);
        let (tx, ty) = xy(HQ[1 - self.me]);
        text.push_str(&format!(
            "2\n0 0 {mx} {my}\n1 0 {tx} {ty}\n{}\n",
            units.count()
        ));
        for (i, cell) in units.iter().enumerate() {
            let (x, y) = xy(cell);
            text.push_str(&format!("0 {} 1 {x} {y}\n", i + 1));
        }
        State::read(&self.initial, &text, League::Bronze, game_turn)
            .expect("a state the engine wrote")
    }
}

/// The end keys of `me`'s `t` turns from `state`: with `prune`, those whose
/// cells may still reach `target` (a superset of those that do); without
/// it, all of them. `None` past `cap` keys in a layer; else also the most
/// keys in a layer. Layers hold keys only; each state is rebuilt from its
/// key when it is expanded.
fn solve(
    state: &State,
    me: usize,
    t: u32,
    target: i64,
    prune: bool,
    cap: usize,
) -> Option<(Vec<Key>, usize)> {
    let frame = Frame::new(state, me);
    let first_turn = state.game_turn();
    let mut layer: HashSet<Key> = HashSet::new();
    layer.insert(key(state, me));
    let mut peak = 1;
    for done in 0..t {
        let left = t - done;
        let mut next: HashSet<Key> = HashSet::new();
        let mut over = false;
        for k in &layer {
            let s = if done == 0 {
                state.clone()
            } else {
                frame.rebuild(k, first_turn + done)
            };
            let slack = bound(&s, me, left) - target;
            if prune && slack < 0 {
                continue;
            }
            let waste = !prune || slack >= 1;
            turns(&s, me, waste, &mut |after, _| {
                if over {
                    return;
                }
                let child = end(after);
                if prune && bound(&child, me, left - 1) < target {
                    return;
                }
                next.insert(key(&child, me));
                if next.len() > cap {
                    over = true;
                }
            });
            if over {
                return None;
            }
        }
        peak = peak.max(next.len());
        layer = next;
    }
    Some((layer.into_iter().collect(), peak))
}

/// A weight for each cell.
type Weights = [i64; CELLS];

/// Weight 1 on the cells of `zone`.
fn ones(zone: Bits) -> Weights {
    let mut w = [0; CELLS];
    for c in zone.iter() {
        w[c] = 1;
    }
    w
}

/// The total weight of the cells of `owned`.
fn weighed(w: &Weights, owned: Bits) -> i64 {
    owned.iter().map(|c| w[c]).sum()
}

/// The linear central weights of E073 (`Decay::Linear(top)` of the
/// opening's `Order::Central`): `top` less a cell's ring from the middle.
fn central(state: &State, top: i64) -> Weights {
    let mut w = [0; CELLS];
    for (c, ring) in solo::rings(state).into_iter().enumerate() {
        if ring != i64::MAX {
            w[c] = Decay::Linear(top).weight(ring);
        }
    }
    w
}

/// The tops of the central goals.
const CENTRAL: [u8; 3] = [5, 7, 9];

/// A branch-and-bound search over `me`'s turns, depth first: among the
/// plans that end with at least `min_cells` active cells, the most cells of
/// `zone` held at the end (`None`: whether such a plan exists at all, which
/// it values 1). A state is cut when its cells cannot reach `min_cells`
/// ([`bound`]) or when its zone cannot beat the best found: the zone cells it
/// holds, plus those within the turns left of its land, at most as many as
/// the cells it can still take. States already searched with as many turns
/// left are not searched again. `nodes` counts the states expanded; past
/// `limit` the search stops and `aborted` is set.
struct Dfs {
    me: usize,
    t: u32,
    min_cells: i64,
    /// Each cell's weight in the value (a zone weighs 1 a cell).
    zone: Option<Weights>,
    best: i64,
    /// The plan of `best` when the search found it (else the caller's).
    plan: Option<Plan>,
    /// The commands of the turns from the root to the state searched.
    path: Plan,
    memo: HashSet<(Key, u32)>,
    nodes: u64,
    limit: u64,
    aborted: bool,
}

impl Dfs {
    fn new(me: usize, t: u32, min_cells: i64, zone: Option<Weights>, best: i64, limit: u64) -> Dfs {
        Dfs {
            me,
            t,
            min_cells,
            zone,
            best,
            plan: None,
            path: Vec::new(),
            memo: HashSet::new(),
            nodes: 0,
            limit,
            aborted: false,
        }
    }

    fn value(&self, owned: Bits) -> i64 {
        self.zone.as_ref().map_or(1, |z| weighed(z, owned))
    }

    fn zone_bound(&self, state: &State, left: u32) -> i64 {
        let Some(zone) = self.zone.as_ref() else {
            return 1;
        };
        let owned = state.owned(self.me);
        let held = weighed(zone, owned);
        let d = distances(state, owned);
        let far = reach(state, self.me, left);
        // The heaviest cells within reach, at most as many as the cells it
        // can still take.
        let mut near: Vec<i64> = (0..CELLS)
            .filter(|&c| zone[c] > 0 && !owned.test(c))
            .filter(|&c| d[c] != u8::MAX && i64::from(d[c]) <= far)
            .map(|c| zone[c])
            .collect();
        near.sort_unstable_by(|a, b| b.cmp(a));
        let room = (bound(state, self.me, left) - i64::from(owned.count())).max(0);
        held + near.iter().take(room as usize).sum::<i64>()
    }

    fn search(&mut self, state: State, done: u32) {
        if self.aborted {
            return;
        }
        let left = self.t - done;
        if left == 0 {
            let value = self.value(state.owned(self.me));
            if i64::from(state.active(self.me).count()) >= self.min_cells && value > self.best {
                self.best = value;
                self.plan = Some(self.path.clone());
            }
            return;
        }
        let most = bound(&state, self.me, left);
        if most < self.min_cells || self.zone_bound(&state, left) <= self.best {
            return;
        }
        if !self.memo.insert((key(&state, self.me), left)) {
            return;
        }
        self.nodes += 1;
        if self.nodes > self.limit {
            self.aborted = true;
            return;
        }
        let me = self.me;
        let mut children: Vec<(State, Vec<Command>)> = Vec::new();
        turns(&state, me, most > self.min_cells, &mut |after, commands| {
            children.push((end(after), commands.to_vec()))
        });
        // The most promising first: the zone held, then the cells bound.
        let zone = self.zone.as_ref();
        children.sort_by_cached_key(|(c, _)| {
            let held = zone.map_or(0, |z| weighed(z, c.owned(me)));
            std::cmp::Reverse((held, bound(c, me, left - 1)))
        });
        for (child, commands) in children {
            self.path.push(commands);
            self.search(child, done + 1);
            self.path.pop();
            if self.aborted {
                return;
            }
        }
    }
}

/// What a plan reaches.
#[derive(Clone, Copy, Debug, Default)]
struct Reached {
    cells: i64,
    chokes: i64,
    /// Our cells within 2 steps of even between the HQs.
    middle: i64,
    /// The central weight of our cells for each top of [`CENTRAL`].
    central: [i64; 3],
}

fn reached(state: &State, me: usize, chokes: Bits, band: Bits) -> Reached {
    let owned = state.owned(me);
    Reached {
        cells: i64::from(state.active(me).count()),
        chokes: i64::from((owned & chokes).count()),
        middle: i64::from((owned & band).count()),
        central: CENTRAL.map(|top| weighed(&central(state, i64::from(top)), owned)),
    }
}

/// An opening played for `t` turns by `me`, the other passing: the state at
/// the start of each turn, the first included, and its plan.
fn play(state: &State, t: u32, mut turn: impl FnMut(&State) -> Vec<Command>) -> (Vec<State>, Plan) {
    let mut s = state.clone();
    let mut states = vec![s.clone()];
    let mut plan = Vec::new();
    for _ in 0..t {
        if s.outcome().is_none() {
            let commands = turn(&s);
            s = solo::advance(&s, &commands);
            plan.push(commands);
        } else {
            plan.push(Vec::new());
        }
        states.push(s.clone());
    }
    (states, plan)
}

/// The state after `plan` from `state`, the other side passing.
fn replay(state: &State, plan: &[Vec<Command>]) -> State {
    let mut s = state.clone();
    for commands in plan {
        if s.outcome().is_some() {
            break;
        }
        s = solo::advance(&s, commands);
    }
    s
}

/// The openings compared with the optimum, by name, each played once to the
/// longest horizon: a new opening joins this list to be measured by `run`
/// and `--bench`.
const OPENINGS: [&str; 21] = [
    "v029", "b", "rules", "assign", "tie", "land", "tie3", "tie6", "tie12", "lin3", "lin6",
    "lin10", "half4", "half8", "sq6", "lin5", "lin7", "lin9", "fr4", "fr8", "ring",
];

struct Openings {
    played: Vec<(Vec<State>, Plan)>,
}

fn openings(state: &State, t: u32) -> Openings {
    let sets = sets();
    let mut v029 = opening(OBJECTIVE);
    let mut b = opening(Objective {
        assign: true,
        ..OBJECTIVE
    });
    // E073: the lexicographic orders.
    let tie_within = |within| {
        opening(Objective {
            order: Order::TieBreak { within },
            ..OBJECTIVE
        })
    };
    let centred = |decay| {
        opening(Objective {
            order: Order::Central(decay),
            ..OBJECTIVE
        })
    };
    let mut tie = tie_within(1);
    let mut lin3 = centred(Decay::Linear(3));
    let mut lin6 = centred(Decay::Linear(6));
    let mut lin10 = centred(Decay::Linear(10));
    let mut half4 = centred(Decay::Halving(4));
    let mut half8 = centred(Decay::Halving(8));
    let mut sq6 = centred(Decay::Square(6));
    let mut lin5 = centred(Decay::Linear(5));
    let mut lin7 = centred(Decay::Linear(7));
    let mut lin9 = centred(Decay::Linear(9));
    let ordered = |order| opening(Objective { order, ..OBJECTIVE });
    let mut fr4 = ordered(Order::Frontier { cap: 4 });
    let mut fr8 = ordered(Order::Frontier { cap: 8 });
    let mut ring = ordered(Order::ByRing);
    let mut tie3 = tie_within(3);
    let mut tie6 = tie_within(6);
    let mut tie12 = tie_within(12);
    let mut land = opening(Objective {
        order: Order::LandFirst,
        ..OBJECTIVE
    });
    let settings = assign::Settings {
        params: sets[0],
        buy: Purchase::Payback,
        ..assign::Settings::DEFAULT
    };
    Openings {
        played: vec![
            play(state, t, |s| v029.turn(s, None)),
            play(state, t, |s| b.turn(s, None)),
            play(state, t, |s| solo::commands(s, &sets, solo::REST)),
            play(state, t, |s| assign::turn(s, &settings)),
            play(state, t, |s| tie.turn(s, None)),
            play(state, t, |s| land.turn(s, None)),
            play(state, t, |s| tie3.turn(s, None)),
            play(state, t, |s| tie6.turn(s, None)),
            play(state, t, |s| tie12.turn(s, None)),
            play(state, t, |s| lin3.turn(s, None)),
            play(state, t, |s| lin6.turn(s, None)),
            play(state, t, |s| lin10.turn(s, None)),
            play(state, t, |s| half4.turn(s, None)),
            play(state, t, |s| half8.turn(s, None)),
            play(state, t, |s| sq6.turn(s, None)),
            play(state, t, |s| lin5.turn(s, None)),
            play(state, t, |s| lin7.turn(s, None)),
            play(state, t, |s| lin9.turn(s, None)),
            play(state, t, |s| fr4.turn(s, None)),
            play(state, t, |s| fr8.turn(s, None)),
            play(state, t, |s| ring.turn(s, None)),
        ],
    }
}

/// What a plan is the best one found for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Goal {
    /// The most active cells.
    Cells,
    /// The most middle chokes within `margin` cells of the most cells.
    Chokes,
    /// The most cells of the middle band within `margin` cells of the most.
    Middle,
    /// The most central weight (E073: the linear weights with this top) at
    /// the most cells.
    Central(u8),
}

impl Goal {
    fn name(self) -> String {
        match self {
            Goal::Cells => "cells".to_string(),
            Goal::Chokes => "chokes".to_string(),
            Goal::Middle => "middle".to_string(),
            Goal::Central(top) => format!("central{top}"),
        }
    }

    fn parse(text: &str) -> Option<Goal> {
        [Goal::Cells, Goal::Chokes, Goal::Middle]
            .into_iter()
            .chain(CENTRAL.map(Goal::Central))
            .find(|g| g.name() == text)
    }

    /// The goal's value of what a plan reaches.
    fn of(self, r: &Reached) -> i64 {
        match self {
            Goal::Cells => r.cells,
            Goal::Chokes => r.chokes,
            Goal::Middle => r.middle,
            Goal::Central(top) => CENTRAL
                .iter()
                .position(|&t| t == top)
                .map_or(0, |i| r.central[i]),
        }
    }
}

/// The best plan found for one goal of one start and horizon.
#[derive(Clone, Debug)]
struct Best {
    goal: Goal,
    margin: u8,
    value: i64,
    cells: i64,
    /// Whether no plan does better: the search ended without its limit.
    proven: bool,
    plan: Plan,
}

/// A start's stored plans (`--from`), by start, horizon, goal and margin.
type Stored = HashMap<(u64, usize, u32, Goal, u8), Plan>;

/// The middle chokes and the middle band of `state`'s map for `me`.
fn zones_of(state: &State, me: usize) -> (Bits, Bits) {
    let chokes = Zones::of(state).middle_chokes();
    let ours = distances(state, Bits::cell(HQ[me]));
    let theirs = distances(state, Bits::cell(HQ[1 - me]));
    let mut band = Bits::EMPTY;
    for c in state.map().valid.iter() {
        if ours[c] != u8::MAX && theirs[c] != u8::MAX && ours[c].abs_diff(theirs[c]) <= 2 {
            band.set(c);
        }
    }
    (chokes, band)
}

/// One start's results at one horizon.
#[derive(Clone, Debug, Default)]
struct Case {
    seed: u64,
    seat: usize,
    turns: u32,
    map_chokes: i64,
    best: Reached,
    /// Distinct end territories with the most cells, and distinct end states
    /// (with the units' places and the gold).
    territories: usize,
    states: usize,
    /// Of those territories, how many hold a middle choke.
    with_choke: usize,
    /// The most middle chokes and middle cells within 0, 1, 2 cells of the
    /// most.
    chokes_within: [i64; 3],
    middle_within: [i64; 3],
    peak: usize,
    ms: u128,
    /// What each of [`OPENINGS`] reaches.
    openings: Vec<Reached>,
    /// The best plan found for each goal and margin.
    plans: Vec<Best>,
    /// The reference player's mean active cells at the next game turn.
    player: Option<f64>,
    too_large: bool,
    /// Whether the most cells, and each zone result, were proven (no search
    /// stopped at its limit).
    proven: bool,
}

#[allow(clippy::too_many_arguments)]
fn run(
    seed: u64,
    seat: usize,
    t: u32,
    cap: usize,
    limit: u64,
    player: Option<f64>,
    open: &Openings,
    stored: &Stored,
) -> Case {
    let state = start(seed, seat);
    let me = seat;
    let (chokes, band) = zones_of(&state, me);
    let measure = |plan: &[Vec<Command>]| reached(&replay(&state, plan), me, chokes, band);
    let mut case = Case {
        seed,
        seat,
        turns: t,
        map_chokes: i64::from(chokes.count()),
        openings: open
            .played
            .iter()
            .map(|(states, _)| reached(&states[t as usize], me, chokes, band))
            .collect(),
        player,
        ..Case::default()
    };
    // Every plan known so far, measured: the openings', the stored ones of
    // this start and horizon, and those the searches below find.
    let mut known: Vec<(Reached, Plan)> = open
        .played
        .iter()
        .map(|(_, plan)| plan[..t as usize].to_vec())
        .chain(
            stored
                .iter()
                .filter(|(k, plan)| {
                    k.0 == seed && k.1 == seat && k.2 == t && plan.len() == t as usize
                })
                .map(|(_, plan)| plan.clone()),
        )
        .map(|plan| (measure(&plan), plan))
        .collect();
    let clock = Instant::now();
    // The most cells: one more than the best known, until no plan reaches it.
    let (mut top, mut top_plan) = known
        .iter()
        .max_by_key(|(r, _)| r.cells)
        .map(|(r, p)| (*r, p.clone()))
        .unwrap_or_default();
    let mut proven = true;
    loop {
        let mut dfs = Dfs::new(me, t, top.cells + 1, None, 0, limit);
        dfs.search(state.clone(), 0);
        if dfs.aborted {
            proven = false;
            break;
        }
        let Some(plan) = dfs.plan else {
            break;
        };
        top = measure(&plan);
        top_plan = plan;
        known.push((top, top_plan.clone()));
    }
    let most = top.cells;
    case.proven = proven;
    case.plans.push(Best {
        goal: Goal::Cells,
        margin: 0,
        value: most,
        cells: most,
        proven,
        plan: top_plan,
    });
    // The most middle chokes and middle cells within 0, 1, 2 cells of it,
    // each search starting from the best plan known for it.
    for (goal, zone) in [(Goal::Chokes, chokes), (Goal::Middle, band)] {
        for d in 0..3u8 {
            let floor = most - i64::from(d);
            let (value, plan) = known
                .iter()
                .filter(|(r, _)| r.cells >= floor)
                .max_by_key(|(r, _)| goal.of(r))
                .map(|(r, p)| (goal.of(r), p.clone()))
                .unwrap_or((-1, Vec::new()));
            let (value, plan, done) = if zone.is_empty() {
                (value.max(0), plan, true)
            } else {
                let mut dfs = Dfs::new(me, t, floor, Some(ones(zone)), value, limit);
                dfs.search(state.clone(), 0);
                match dfs.plan {
                    Some(found) => (dfs.best, found, !dfs.aborted),
                    None => (value, plan, !dfs.aborted),
                }
            };
            case.proven &= done;
            let r = measure(&plan);
            known.push((r, plan.clone()));
            match goal {
                Goal::Chokes => case.chokes_within[usize::from(d)] = value,
                _ => case.middle_within[usize::from(d)] = value,
            }
            case.plans.push(Best {
                goal,
                margin: d,
                value,
                cells: r.cells,
                // Proven only if the most cells it is measured from is.
                proven: done && proven,
                plan,
            });
        }
    }
    // The most central weight at the most cells (E073), for each top.
    let mut central_best = [0; 3];
    for (i, &top) in CENTRAL.iter().enumerate() {
        let goal = Goal::Central(top);
        let (value, plan) = known
            .iter()
            .filter(|(r, _)| r.cells >= most)
            .max_by_key(|(r, _)| goal.of(r))
            .map(|(r, p)| (goal.of(r), p.clone()))
            .unwrap_or((-1, Vec::new()));
        let mut dfs = Dfs::new(
            me,
            t,
            most,
            Some(central(&state, i64::from(top))),
            value,
            limit,
        );
        dfs.search(state.clone(), 0);
        let done = !dfs.aborted;
        let (value, plan) = match dfs.plan {
            Some(found) => (dfs.best, found),
            None => (value, plan),
        };
        case.proven &= done;
        let r = measure(&plan);
        known.push((r, plan.clone()));
        central_best[i] = value;
        case.plans.push(Best {
            goal,
            margin: 0,
            value,
            cells: r.cells,
            proven: done && proven,
            plan,
        });
    }
    case.best = Reached {
        cells: most,
        chokes: case.chokes_within[0],
        middle: case.middle_within[0],
        central: central_best,
    };
    // The optimal territories, counted while they fit.
    match solve(&state, me, t, most, true, cap) {
        Some((finals, peak)) => {
            case.peak = peak;
            let mut territories: HashMap<[u64; 3], bool> = HashMap::new();
            for k in &finals {
                let owned = from_words(k.0);
                if i64::from(owned.count()) == most {
                    case.states += 1;
                    let held = territories.entry(k.0).or_insert(false);
                    *held |= !(owned & chokes).is_empty();
                }
            }
            case.territories = territories.len();
            case.with_choke = territories.values().filter(|&&h| h).count();
        }
        None => case.too_large = true,
    }
    case.ms = clock.elapsed().as_millis();
    case
}

/// `--check`: the pruned search and the full one agree on short horizons.
fn check(starts: &[(u64, usize)]) {
    for &(seed, seat) in starts {
        for t in [3, 4, 5] {
            let state = start(seed, seat);
            let (full, _) =
                solve(&state, seat, t, i64::MIN / 2, false, usize::MAX).expect("no cap");
            let cells = |k: &Key| i64::from(from_words(k.0).count());
            let most = full.iter().map(cells).max().unwrap_or(0);
            let optimal = |keys: &[Key]| {
                let mut keys: Vec<Key> =
                    keys.iter().filter(|k| cells(k) == most).copied().collect();
                keys.sort_unstable();
                keys
            };
            for target in [most, most - 1] {
                let (pruned, _) = solve(&state, seat, t, target, true, usize::MAX).expect("no cap");
                assert_eq!(
                    optimal(&full),
                    optimal(&pruned),
                    "seed {seed}, seat {seat}, {t} turns, target {target}"
                );
            }
            // The depth-first searches agree with the enumeration: no plan
            // beats the most cells, and the zone's best among the plans
            // within 0 and 1 cells of it is the enumeration's.
            let mut over = Dfs::new(seat, t, most + 1, None, 0, u64::MAX);
            over.search(state.clone(), 0);
            assert_eq!(over.best, 0, "seed {seed}, seat {seat}, {t} turns");
            let zone = Zones::of(&state).middle_chokes()
                | (state.map().valid & !Bits::cell(HQ[0]) & !Bits::cell(HQ[1]))
                    .iter()
                    .filter(|&c| c % 2 == 0)
                    .fold(Bits::EMPTY, |b, c| b | Bits::cell(c));
            for d in 0..2 {
                let floor = most - d;
                let expected = full
                    .iter()
                    .filter(|k| cells(k) >= floor)
                    .map(|k| i64::from((from_words(k.0) & zone).count()))
                    .max()
                    .unwrap_or(0);
                let mut dfs = Dfs::new(seat, t, floor, Some(ones(zone)), -1, u64::MAX);
                dfs.search(state.clone(), 0);
                assert_eq!(
                    dfs.best, expected,
                    "seed {seed}, seat {seat}, {t} turns, floor {floor}"
                );
            }
            // The central goals (E073) at the most cells, likewise.
            for top in CENTRAL {
                let w = central(&state, i64::from(top));
                let expected = full
                    .iter()
                    .filter(|k| cells(k) >= most)
                    .map(|k| weighed(&w, from_words(k.0)))
                    .max()
                    .unwrap_or(0);
                let mut dfs = Dfs::new(seat, t, most, Some(w), -1, u64::MAX);
                dfs.search(state.clone(), 0);
                assert_eq!(
                    dfs.best, expected,
                    "seed {seed}, seat {seat}, {t} turns, central {top}"
                );
            }
            // The bound never falls below what a plan reaches.
            assert!(bound(&state, seat, t) >= most, "seed {seed}, seat {seat}");
            println!(
                "check: seed {seed} seat {seat} {t} turns: {} end states, most cells {most}, {} optimal",
                full.len(),
                optimal(&full).len()
            );
        }
    }
}

/// The reference player's starts in the suite: (seed, seat) with its mean
/// active cells at game turns 7, 8, 9 (`None` where the suite has no value).
fn suite_starts(path: &str, player: &str) -> Vec<(u64, usize, [Option<f64>; 3])> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("oracle: cannot read {path}: {e}");
        std::process::exit(2);
    });
    let mut lines = text.lines().filter(|l| !l.starts_with('#'));
    let header: Vec<&str> = lines.next().unwrap_or_default().split('\t').collect();
    let col = |name: &str| header.iter().position(|h| *h == name);
    let (Some(seed_c), Some(seat_c), Some(player_c)) = (col("seed"), col("seat"), col("player"))
    else {
        eprintln!("oracle: the suite has no seed, seat or player column");
        std::process::exit(2);
    };
    let active: Vec<Option<usize>> = [7, 8, 9]
        .iter()
        .map(|t| col(&format!("active{t}")))
        .collect();
    let mut sums: HashMap<(u64, usize), ([f64; 3], [u32; 3])> = HashMap::new();
    let mut order: Vec<(u64, usize)> = Vec::new();
    for line in lines {
        let f: Vec<&str> = line.split('\t').collect();
        if f.get(player_c) != Some(&player) {
            continue;
        }
        let (Some(Ok(seed)), Some(Ok(seat))) = (
            f.get(seed_c).map(|s| s.parse::<u64>()),
            f.get(seat_c).map(|s| s.parse::<usize>()),
        ) else {
            continue;
        };
        let entry = sums.entry((seed, seat)).or_insert_with(|| {
            order.push((seed, seat));
            ([0.0; 3], [0; 3])
        });
        for (i, c) in active.iter().enumerate() {
            if let Some(Ok(v)) = c.and_then(|c| f.get(c)).map(|s| s.parse::<f64>()) {
                entry.0[i] += v;
                entry.1[i] += 1;
            }
        }
    }
    order
        .into_iter()
        .map(|k| {
            let (s, n) = sums[&k];
            let mean = |i: usize| (n[i] > 0).then(|| s[i] / f64::from(n[i]));
            (k.0, k.1, [mean(0), mean(1), mean(2)])
        })
        .collect()
}

/// The columns of `--csv`.
const HEADER: &str = "seed,seat,turns,proven,too_large,map_chokes,most_cells,territories,states,with_choke,chokes0,chokes1,chokes2,middle0,middle1,middle2,v029_cells,v029_chokes,v029_middle,b_cells,b_chokes,rules_cells,assign_cells,assign_chokes,player_next,peak,ms";

/// One start and horizon as a line of `--csv`.
fn csv_line(c: &Case) -> String {
    format!(
        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
        c.seed,
        c.seat,
        c.turns,
        c.proven,
        c.too_large,
        c.map_chokes,
        c.best.cells,
        c.territories,
        c.states,
        c.with_choke,
        c.chokes_within[0],
        c.chokes_within[1],
        c.chokes_within[2],
        c.middle_within[0],
        c.middle_within[1],
        c.middle_within[2],
        c.openings[0].cells,
        c.openings[0].chokes,
        c.openings[0].middle,
        c.openings[1].cells,
        c.openings[1].chokes,
        c.openings[2].cells,
        c.openings[3].cells,
        c.openings[3].chokes,
        c.player.map(|p| format!("{p:.2}")).unwrap_or_default(),
        c.peak,
        c.ms
    )
}

/// The columns of `--plans`, tab-separated: the start, the horizon, the goal
/// and its margin, the value found, the plan's cells, whether no plan does
/// better, and the plan: each turn's answer as CodinGame reads it, the turns
/// separated by ` | `.
const PLANS_HEADER: &str = "seed\tseat\tturns\tgoal\tmargin\tvalue\tcells\tproven\tplan";

fn plan_text(plan: &[Vec<Command>]) -> String {
    plan.iter()
        .map(|commands| answer_line(commands))
        .collect::<Vec<_>>()
        .join(" | ")
}

/// One command as CodinGame writes it (`MOVE 3 4 5`, `TRAIN 1 2 3`, `BUILD
/// MINE 1 2`, `BUILD TOWER 1 2`, `WAIT`).
fn parse_command(text: &str) -> Option<Command> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let n = |i: usize| words.get(i).and_then(|w| w.parse::<u32>().ok());
    let b = |i: usize| words.get(i).and_then(|w| w.parse::<u8>().ok());
    match words.as_slice() {
        ["MOVE", _, _, _] => Some(Command::Move {
            id: n(1)?,
            x: b(2)?,
            y: b(3)?,
        }),
        ["TRAIN", _, _, _] => Some(Command::Train {
            level: b(1)?,
            x: b(2)?,
            y: b(3)?,
        }),
        ["BUILD", "MINE", _, _] => Some(Command::BuildMine { x: b(2)?, y: b(3)? }),
        ["BUILD", "TOWER", _, _] => Some(Command::BuildTower { x: b(2)?, y: b(3)? }),
        ["WAIT"] => Some(Command::Wait),
        _ => None,
    }
}

/// A plan written by [`plan_text`].
fn parse_plan(text: &str) -> Option<Plan> {
    if text.trim().is_empty() {
        return Some(Vec::new());
    }
    text.split(" | ")
        .map(|turn| {
            turn.split(';')
                .map(parse_command)
                .filter(|c| *c != Some(Command::Wait))
                .collect::<Option<Vec<Command>>>()
        })
        .collect()
}

/// One line of a plans file.
#[derive(Clone, Debug)]
struct Row {
    seed: u64,
    seat: usize,
    turns: u32,
    best: Best,
}

fn plans_line(seed: u64, seat: usize, turns: u32, b: &Best) -> String {
    format!(
        "{seed}\t{seat}\t{turns}\t{}\t{}\t{}\t{}\t{}\t{}\n",
        b.goal.name(),
        b.margin,
        b.value,
        b.cells,
        b.proven,
        plan_text(&b.plan)
    )
}

/// The rows of a plans file; a line that does not read is an error.
fn load_plans(path: &str) -> Vec<Row> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("oracle: cannot read {path}: {e}");
        std::process::exit(2);
    });
    let mut out = Vec::new();
    for (n, line) in text.lines().enumerate() {
        if n == 0 && line.starts_with("seed\t") || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        let row = (|| {
            Some(Row {
                seed: f.first()?.parse().ok()?,
                seat: f.get(1)?.parse().ok()?,
                turns: f.get(2)?.parse().ok()?,
                best: Best {
                    goal: Goal::parse(f.get(3)?)?,
                    margin: f.get(4)?.parse().ok()?,
                    value: f.get(5)?.parse().ok()?,
                    cells: f.get(6)?.parse().ok()?,
                    proven: f.get(7)?.parse().ok()?,
                    plan: parse_plan(f.get(8)?)?,
                },
            })
        })();
        match row.filter(|r| r.best.plan.len() == r.turns as usize) {
            Some(row) => out.push(row),
            None => {
                eprintln!(
                    "oracle: {path}, line {}: not a plan row of as many turns as it says",
                    n + 1
                );
                std::process::exit(2);
            }
        }
    }
    out
}

/// A board for `--show`: `#` void, `.` neutral, `*` a neutral middle choke,
/// `O` ours, `@` a middle choke of ours, `X` the enemy's.
fn board(state: &State, me: usize, chokes: Bits) -> String {
    let mut out = String::new();
    for y in 0..SIZE {
        out.push_str("    ");
        for x in 0..SIZE {
            let c = y * SIZE + x;
            out.push(if !state.map().valid.test(c) {
                '#'
            } else if state.owned(me).test(c) {
                if chokes.test(c) {
                    '@'
                } else {
                    'O'
                }
            } else if state.owned(1 - me).test(c) {
                'X'
            } else if chokes.test(c) {
                '*'
            } else {
                '.'
            });
        }
        out.push('\n');
    }
    out
}

/// `--replay`: plays every stored plan again and checks its cells and value.
fn replay_mode(path: &str, show: bool) {
    let rows = load_plans(path);
    let mut wrong = 0;
    for row in &rows {
        let state = start(row.seed, row.seat);
        let (chokes, band) = zones_of(&state, row.seat);
        let end = replay(&state, &row.best.plan);
        let r = reached(&end, row.seat, chokes, band);
        let ok = r.cells == row.best.cells
            && row.best.goal.of(&r) == row.best.value
            && row.best.plan.len() == row.turns as usize;
        if !ok {
            wrong += 1;
        }
        if show || !ok {
            println!(
                "{} {} seat {} turn {} {} within {}: value {} cells {}{}",
                if ok { "ok  " } else { "FAIL" },
                row.seed,
                row.seat,
                row.turns,
                row.best.goal.name(),
                row.best.margin,
                row.best.goal.of(&r),
                r.cells,
                if row.best.proven {
                    " (proven)"
                } else {
                    " (best found)"
                }
            );
        }
        if show {
            print!("{}", board(&end, row.seat, chokes));
        }
    }
    println!("replay: {} plans, {wrong} wrong", rows.len());
    if wrong > 0 {
        std::process::exit(1);
    }
}

/// `--bench`: each of [`OPENINGS`] on the stored starts, against the stored
/// best plans: its cells short of the most, and its middle chokes and middle
/// cells short of the most at its own cells (when within 2 of the most).
fn bench_mode(path: &str, jobs: usize) {
    let rows = load_plans(path);
    let mut by_start: HashMap<(u64, usize), Vec<&Row>> = HashMap::new();
    for row in &rows {
        by_start.entry((row.seed, row.seat)).or_default().push(row);
    }
    let mut starts: Vec<(u64, usize)> = by_start.keys().copied().collect();
    starts.sort_unstable();
    let longest = rows.iter().map(|r| r.turns).max().unwrap_or(0);
    let next = std::sync::atomic::AtomicUsize::new(0);
    // (turns, seat, opening) -> sums: starts, cell gap, at the most cells,
    // starts with chokes measured, choke gap, at the most chokes, starts with
    // the band measured, middle gap, all compared values proven.
    type Sums = [f64; 13];
    let gathered: Vec<((u32, usize, usize), Sums)> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..jobs.max(1))
            .map(|_| {
                scope.spawn(|| {
                    let mut out = Vec::new();
                    loop {
                        let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let Some(&(seed, seat)) = starts.get(i) else {
                            break;
                        };
                        let state = start(seed, seat);
                        let (chokes, band) = zones_of(&state, seat);
                        let open = openings(&state, longest);
                        let stored = &by_start[&(seed, seat)];
                        let find = |t: u32, goal: Goal, margin: u8| {
                            stored.iter().find(|r| {
                                r.turns == t && r.best.goal == goal && r.best.margin == margin
                            })
                        };
                        let mut ts: Vec<u32> = stored.iter().map(|r| r.turns).collect();
                        ts.sort_unstable();
                        ts.dedup();
                        for t in ts {
                            let Some(most) = find(t, Goal::Cells, 0) else {
                                continue;
                            };
                            for (k, (states, _)) in open.played.iter().enumerate() {
                                let r = reached(&states[t as usize], seat, chokes, band);
                                let gap = most.best.value - r.cells;
                                let mut sums: Sums = [0.0; 13];
                                sums[0] = 1.0;
                                sums[1] = gap as f64;
                                sums[2] = f64::from(u8::from(gap <= 0));
                                let mut proven = most.best.proven;
                                if (0..=2).contains(&gap) {
                                    let margin = gap as u8;
                                    if let Some(c) =
                                        find(t, Goal::Chokes, margin).filter(|_| !chokes.is_empty())
                                    {
                                        sums[3] = 1.0;
                                        sums[4] = (c.best.value - r.chokes) as f64;
                                        sums[5] = f64::from(u8::from(r.chokes >= c.best.value));
                                        proven &= c.best.proven;
                                    }
                                    if let Some(m) = find(t, Goal::Middle, margin) {
                                        sums[6] = 1.0;
                                        sums[7] = (m.best.value - r.middle) as f64;
                                        proven &= m.best.proven;
                                    }
                                }
                                sums[8] = f64::from(u8::from(proven));
                                // E073: the central weight short of the most
                                // at the most cells, for each top.
                                if gap == 0 {
                                    let stored: Vec<_> = CENTRAL
                                        .iter()
                                        .map(|&top| find(t, Goal::Central(top), 0))
                                        .collect();
                                    if stored.iter().all(Option::is_some) {
                                        sums[9] = 1.0;
                                        for (i, c) in stored.into_iter().flatten().enumerate() {
                                            sums[10 + i] = (c.best.value - r.central[i]) as f64;
                                        }
                                    }
                                }
                                out.push(((t, seat, k), sums));
                            }
                        }
                    }
                    out
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap_or_default())
            .collect()
    });
    let mut totals: HashMap<(u32, usize, usize), Sums> = HashMap::new();
    for (key, sums) in gathered {
        let total = totals.entry(key).or_insert([0.0; 13]);
        for (a, b) in total.iter_mut().zip(sums) {
            *a += b;
        }
    }
    let mut keys: Vec<(u32, usize, usize)> = totals.keys().copied().collect();
    keys.sort_unstable();
    println!("| Turns | Seat | Opening | Starts | Cells short of the most | At the most cells | Starts with middle chokes, compared | Chokes short of the most at its cells | At the most chokes | Middle cells short of the most at its cells | Compared against proven values | Starts at the most cells, central compared | Central weight short of the most: lin5 / lin7 / lin9 |");
    println!(
        "| ---: | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |"
    );
    for key in keys {
        let s = totals[&key];
        let per = |a: f64, n: f64| {
            if n > 0.0 {
                format!("{:.2}", a / n)
            } else {
                "-".to_string()
            }
        };
        let pct = |a: f64, n: f64| {
            if n > 0.0 {
                format!("{:.0}%", 100.0 * a / n)
            } else {
                "-".to_string()
            }
        };
        println!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} / {} / {} |",
            key.0,
            if key.1 == 0 { "red" } else { "blue" },
            OPENINGS[key.2],
            s[0],
            per(s[1], s[0]),
            pct(s[2], s[0]),
            s[3],
            per(s[4], s[3]),
            pct(s[5], s[3]),
            per(s[7], s[6]),
            pct(s[8], s[0]),
            s[9],
            per(s[10], s[9]),
            per(s[11], s[9]),
            per(s[12], s[9]),
        );
    }
}

fn mean(xs: impl Iterator<Item = f64>) -> f64 {
    let v: Vec<f64> = xs.collect();
    if v.is_empty() {
        f64::NAN
    } else {
        v.iter().sum::<f64>() / v.len() as f64
    }
}

fn median(mut v: Vec<usize>) -> usize {
    v.sort_unstable();
    v.get(v.len() / 2).copied().unwrap_or(0)
}

fn main() {
    let mut args = std::env::args().skip(1);
    let (mut suite, mut player) = (String::new(), "zasmu".to_string());
    let (mut per_seat, mut jobs, mut cap) = (50usize, 4usize, 20_000usize);
    let mut horizons: Vec<u32> = vec![6, 7, 8];
    let (mut csv, mut do_check) = (None::<String>, false);
    let (mut plans_out, mut from, mut replay_from, mut bench_from) = (
        None::<String>,
        None::<String>,
        None::<String>,
        None::<String>,
    );
    let mut show = false;
    // Most states each depth-first search expands before it stops with the
    // best plan found so far, unproven.
    let mut limit: u64 = 20_000;
    // This run's share of the starts: those whose place in the list is K
    // modulo N (`--shard K/N`), so that runners split a study.
    let mut shard: (usize, usize) = (0, 1);
    while let Some(arg) = args.next() {
        let mut value = || args.next().unwrap_or_default();
        match arg.as_str() {
            "--suite" => suite = value(),
            "--player" => player = value(),
            "--per-seat" => per_seat = value().parse().unwrap_or(per_seat),
            "--jobs" => jobs = value().parse().unwrap_or(jobs).max(1),
            "--cap" => cap = value().parse().unwrap_or(cap),
            "--turns" => horizons = value().split(',').filter_map(|s| s.parse().ok()).collect(),
            "--csv" => csv = Some(value()),
            "--check" => do_check = true,
            "--limit" => limit = value().parse().unwrap_or(limit),
            "--plans" => plans_out = Some(value()),
            "--from" => from = Some(value()),
            "--replay" => replay_from = Some(value()),
            "--show" => show = true,
            "--bench" => bench_from = Some(value()),
            "--shard" => {
                let v = value();
                let parsed = v
                    .split_once('/')
                    .and_then(|(k, n)| Some((k.parse().ok()?, n.parse().ok()?)));
                match parsed {
                    Some((k, n)) if n >= 1 && k < n => shard = (k, n),
                    _ => {
                        eprintln!("oracle: --shard takes K/N with K below N, not {v}");
                        std::process::exit(2);
                    }
                }
            }
            other => {
                eprintln!("oracle: unknown argument {other}");
                std::process::exit(2);
            }
        }
    }
    if let Some(path) = replay_from {
        replay_mode(&path, show);
        return;
    }
    if let Some(path) = bench_from {
        bench_mode(&path, jobs);
        return;
    }
    let mut stored: Stored = HashMap::new();
    if let Some(path) = &from {
        for row in load_plans(path) {
            stored.insert(
                (
                    row.seed,
                    row.seat,
                    row.turns,
                    row.best.goal,
                    row.best.margin,
                ),
                row.best.plan,
            );
        }
    }
    let stored = &stored;
    let mut starts = suite_starts(&suite, &player);
    starts.sort_by_key(|s| (s.1, s.0));
    let mut chosen: Vec<(u64, usize, [Option<f64>; 3])> = Vec::new();
    for seat in 0..2 {
        chosen.extend(
            starts
                .iter()
                .filter(|s| s.1 == seat)
                .take(per_seat)
                .copied(),
        );
    }
    let chosen: Vec<(u64, usize, [Option<f64>; 3])> = chosen
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i % shard.1 == shard.0)
        .map(|(_, s)| s)
        .collect();
    if do_check {
        let few: Vec<(u64, usize)> = chosen
            .iter()
            .take(3)
            .chain(chosen.iter().rev().take(3))
            .map(|s| (s.0, s.1))
            .collect();
        check(&few);
    }
    let longest = horizons.iter().copied().max().unwrap_or(0);
    // `--csv` gets each start's lines as soon as they are done, so that a run
    // stopped midway keeps them; the starts already in it are skipped.
    let mut done: HashSet<(u64, usize)> = HashSet::new();
    let sink = csv.as_ref().map(|path| {
        let old = std::fs::read_to_string(path).unwrap_or_default();
        for line in old.lines().skip(1) {
            let mut f = line.split(',');
            if let (Some(Ok(seed)), Some(Ok(seat))) =
                (f.next().map(str::parse), f.next().map(str::parse))
            {
                done.insert((seed, seat));
            }
        }
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|e| {
                eprintln!("oracle: cannot open {path}: {e}");
                std::process::exit(2);
            });
        if old.is_empty() {
            let _ = writeln!(file, "{HEADER}");
        }
        std::sync::Mutex::new(file)
    });
    // `--plans` gets each start's best plans in the same way.
    let plans_sink = plans_out.as_ref().map(|path| {
        let old = std::fs::read_to_string(path).unwrap_or_default();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|e| {
                eprintln!("oracle: cannot open {path}: {e}");
                std::process::exit(2);
            });
        if old.is_empty() {
            let _ = writeln!(file, "{PLANS_HEADER}");
        }
        std::sync::Mutex::new(file)
    });
    let done = &done;
    let sink = &sink;
    let plans_sink = &plans_sink;
    let next = std::sync::atomic::AtomicUsize::new(0);
    let chosen = &chosen;
    let horizons = &horizons;
    let mut rows: Vec<Case> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..jobs)
            .map(|_| {
                scope.spawn(|| {
                    let mut out = Vec::new();
                    loop {
                        let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        let Some(&(seed, seat, player)) = chosen.get(i) else {
                            break;
                        };
                        if done.contains(&(seed, seat)) {
                            continue;
                        }
                        let open = openings(&start(seed, seat), longest);
                        let mut lines = String::new();
                        let mut plan_lines = String::new();
                        for &t in horizons {
                            let p = player[(t as usize).saturating_sub(6).min(2)];
                            let case = run(seed, seat, t, cap, limit, p, &open, stored);
                            lines.push_str(&csv_line(&case));
                            for b in &case.plans {
                                plan_lines.push_str(&plans_line(seed, seat, t, b));
                            }
                            out.push(case);
                        }
                        if let Some(file) = &plans_sink {
                            let mut file = file.lock().unwrap_or_else(|e| e.into_inner());
                            if let Err(e) = file
                                .write_all(plan_lines.as_bytes())
                                .and_then(|()| file.flush())
                            {
                                eprintln!("oracle: cannot write the plans: {e}");
                            }
                        }
                        if let Some(file) = &sink {
                            let mut file = file.lock().unwrap_or_else(|e| e.into_inner());
                            if let Err(e) =
                                file.write_all(lines.as_bytes()).and_then(|()| file.flush())
                            {
                                eprintln!("oracle: cannot write the csv: {e}");
                            }
                        }
                        eprintln!("oracle: start {} of {} done", i + 1, chosen.len());
                    }
                    out
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap_or_default())
            .collect()
    });
    rows.sort_by_key(|c| (c.turns, c.seat, c.seed));
    println!("| Turns | Seat | Starts | Proven | Most cells | Starts with more optimal territories than the cap | Optimal territories, the others: median / max | Of those on maps with middle chokes: holding one | Middle chokes within 0 / 1 / 2 cells | v029: cells / chokes | B: cells / chokes | Rules: cells | Assignment: cells / chokes | Player at next turn | ms (max) |");
    println!("| ---: | --- | ---: | ---: | ---: | ---: | --- | --- | --- | --- | --- | ---: | --- | ---: | ---: |");
    for &t in horizons {
        for seat in 0..2 {
            let all: Vec<&Case> = rows
                .iter()
                .filter(|c| c.turns == t && c.seat == seat)
                .collect();
            if all.is_empty() {
                continue;
            }
            // Territories are counted where they fit the cap; every other
            // column is exact on every start (`proven`).
            let counted: Vec<&Case> = all.iter().copied().filter(|c| !c.too_large).collect();
            let m = |f: &dyn Fn(&Case) -> f64| mean(all.iter().map(|c| f(c)));
            let with_map: Vec<&Case> = counted
                .iter()
                .copied()
                .filter(|c| c.map_chokes > 0)
                .collect();
            let share = if with_map.is_empty() {
                "-".to_string()
            } else {
                let any = with_map.iter().filter(|c| c.with_choke > 0).count();
                let frac = mean(
                    with_map
                        .iter()
                        .map(|c| c.with_choke as f64 / c.territories.max(1) as f64),
                );
                format!("{any} of {} counted; {:.0}%", with_map.len(), 100.0 * frac)
            };
            let name = if seat == 0 { "red" } else { "blue" };
            println!(
                "| {t} | {name} | {} | {} | {:.2} | {} | {} / {} | {share} | {:.2} / {:.2} / {:.2} | {:.2} / {:.2} | {:.2} / {:.2} | {:.2} | {:.2} / {:.2} | {:.2} | {} |",
                all.len(),
                all.iter().filter(|c| c.proven).count(),
                m(&|c| c.best.cells as f64),
                all.iter().filter(|c| c.too_large).count(),
                median(counted.iter().map(|c| c.territories).collect()),
                counted.iter().map(|c| c.territories).max().unwrap_or(0),
                m(&|c| c.chokes_within[0] as f64),
                m(&|c| c.chokes_within[1] as f64),
                m(&|c| c.chokes_within[2] as f64),
                m(&|c| c.openings[0].cells as f64),
                m(&|c| c.openings[0].chokes as f64),
                m(&|c| c.openings[1].cells as f64),
                m(&|c| c.openings[1].chokes as f64),
                m(&|c| c.openings[2].cells as f64),
                m(&|c| c.openings[3].cells as f64),
                m(&|c| c.openings[3].chokes as f64),
                mean(all.iter().filter_map(|c| c.player)),
                all.iter().map(|c| c.ms).max().unwrap_or(0),
            );
        }
    }
}
