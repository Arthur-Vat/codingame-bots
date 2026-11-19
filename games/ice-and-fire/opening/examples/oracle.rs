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
//!    holds, plus those within the turns left of its land (at most the cells
//!    it can still take), cannot beat the best found.
//! 3. **The optimal end territories,** distinct sets of cells, counted by a
//!    breadth-first enumeration while a layer holds at most `--cap` states;
//!    beyond, the start is reported as having more.
//!
//! A search that expands 4 million states stops, and the start is not
//! `proven`. `--check` compares, on short horizons, the pruned searches with
//! a full enumeration without any cut, and asserts the same results.
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
//!     --per-seat 50 --turns 6,7,8 --jobs 4 [--cap 1000000] [--shard K/N] [--csv FILE] [--check]
//! ```

use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::time::Instant;

use ice_and_fire_early::zones::Zones;
use ice_and_fire_engine::{Bits, Command, League, State, HQ, SIZE};
use ice_and_fire_opening::assign::{self, Purchase};
use ice_and_fire_opening::econ::{distances, Params};
use ice_and_fire_opening::planned::{Planned, Planner};
use ice_and_fire_opening::solo::{self, Objective};
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

/// A state's key for merging: our cells, our units, our gold.
type Key = ([u64; 3], [u64; 3], i64);

fn key(state: &State, me: usize) -> Key {
    (
        state.owned(me).words(),
        state.units(me).words(),
        state.gold(me),
    )
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
/// `out` as the state after it, before the turn ends. Moves that take no
/// cell are tried only with `waste`; a unit with no cell to take may always
/// stay.
fn turns(state: &State, me: usize, waste: bool, out: &mut dyn FnMut(State)) {
    let units: Vec<usize> = state.movable_units().iter().collect();
    let mut after_moves = Vec::new();
    moves(state.clone(), me, &units, waste, &mut after_moves);
    for s in after_moves {
        trainings(s, me, None, Bits::EMPTY, out);
    }
}

fn moves(state: State, me: usize, units: &[usize], waste: bool, out: &mut Vec<State>) {
    let Some((&from, rest)) = units.split_first() else {
        out.push(state);
        return;
    };
    let Some((id, level)) = state.unit_at(from) else {
        moves(state, me, rest, waste, out);
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
        if next.apply(Command::Move { id, x, y }) {
            took |= capture;
            moves(next, me, rest, waste, out);
        }
    }
    if waste || !took {
        moves(state, me, rest, waste, out);
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
    out: &mut dyn FnMut(State),
) {
    out(state.clone());
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
        if next.apply(Command::Train { level: 1, x, y }) {
            trainings(next, me, Some(cell), trainable, out);
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
            turns(&s, me, waste, &mut |after| {
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
    zone: Option<Bits>,
    best: i64,
    memo: HashSet<(Key, u32)>,
    nodes: u64,
    limit: u64,
    aborted: bool,
}

impl Dfs {
    fn new(me: usize, t: u32, min_cells: i64, zone: Option<Bits>, best: i64, limit: u64) -> Dfs {
        Dfs {
            me,
            t,
            min_cells,
            zone,
            best,
            memo: HashSet::new(),
            nodes: 0,
            limit,
            aborted: false,
        }
    }

    fn value(&self, owned: Bits) -> i64 {
        self.zone.map_or(1, |z| i64::from((owned & z).count()))
    }

    fn zone_bound(&self, state: &State, left: u32) -> i64 {
        let Some(zone) = self.zone else {
            return 1;
        };
        let owned = state.owned(self.me);
        let held = i64::from((owned & zone).count());
        let d = distances(state, owned);
        let near = zone
            .and_not(owned)
            .iter()
            .filter(|&c| d[c] != u8::MAX && u32::from(d[c]) <= left)
            .count() as i64;
        let room = (bound(state, self.me, left) - i64::from(owned.count())).max(0);
        held + near.min(room)
    }

    fn search(&mut self, state: State, done: u32) {
        if self.aborted {
            return;
        }
        let left = self.t - done;
        if left == 0 {
            if i64::from(state.active(self.me).count()) >= self.min_cells {
                self.best = self.best.max(self.value(state.owned(self.me)));
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
        let mut children: Vec<State> = Vec::new();
        turns(&state, me, most > self.min_cells, &mut |after| {
            children.push(end(after))
        });
        // The most promising first: the zone held, then the cells bound.
        let zone = self.zone;
        children.sort_by_cached_key(|c| {
            let held = zone.map_or(0, |z| (c.owned(me) & z).count());
            std::cmp::Reverse((held, bound(c, me, left - 1)))
        });
        for child in children {
            self.search(child, done + 1);
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
}

fn reached(state: &State, me: usize, chokes: Bits, band: Bits) -> Reached {
    Reached {
        cells: i64::from(state.active(me).count()),
        chokes: i64::from((state.owned(me) & chokes).count()),
        middle: i64::from((state.owned(me) & band).count()),
    }
}

/// An opening played for `t` turns by `me`, the other passing: the state at
/// the start of each turn, the first included.
fn play(state: &State, t: u32, mut turn: impl FnMut(&State) -> Vec<Command>) -> Vec<State> {
    let mut s = state.clone();
    let mut out = vec![s.clone()];
    for _ in 0..t {
        if s.outcome().is_none() {
            let commands = turn(&s);
            s = solo::advance(&s, &commands);
        }
        out.push(s.clone());
    }
    out
}

/// The openings of one start, played once to the longest horizon.
struct Openings {
    v029: Vec<State>,
    b: Vec<State>,
    rules: Vec<State>,
    assign: Vec<State>,
}

fn openings(state: &State, t: u32) -> Openings {
    let sets = sets();
    let mut v029 = opening(OBJECTIVE);
    let mut b = opening(Objective {
        assign: true,
        ..OBJECTIVE
    });
    let settings = assign::Settings {
        params: sets[0],
        buy: Purchase::Payback,
        ..assign::Settings::DEFAULT
    };
    Openings {
        v029: play(state, t, |s| v029.turn(s, None)),
        b: play(state, t, |s| b.turn(s, None)),
        rules: play(state, t, |s| solo::commands(s, &sets, solo::REST)),
        assign: play(state, t, |s| assign::turn(s, &settings)),
    }
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
    v029: Reached,
    b: Reached,
    rules: Reached,
    assign: Reached,
    /// The reference player's mean active cells at the next game turn.
    player: Option<f64>,
    too_large: bool,
    /// Whether the most cells, and each zone result, were proven (no search
    /// stopped at its limit).
    proven: bool,
}

fn run(seed: u64, seat: usize, t: u32, cap: usize, player: Option<f64>, open: &Openings) -> Case {
    let state = start(seed, seat);
    let me = seat;
    let zones = Zones::of(&state);
    let chokes = zones.middle_chokes();
    let ours = distances(&state, Bits::cell(HQ[me]));
    let theirs = distances(&state, Bits::cell(HQ[1 - me]));
    let mut band = Bits::EMPTY;
    for c in state.map().valid.iter() {
        if ours[c] != u8::MAX && theirs[c] != u8::MAX && ours[c].abs_diff(theirs[c]) <= 2 {
            band.set(c);
        }
    }
    let at = |states: &[State]| reached(&states[t as usize], me, chokes, band);
    let mut case = Case {
        seed,
        seat,
        turns: t,
        map_chokes: i64::from(chokes.count()),
        v029: at(&open.v029),
        b: at(&open.b),
        rules: at(&open.rules),
        assign: at(&open.assign),
        player,
        ..Case::default()
    };
    let known = [case.v029, case.b, case.rules, case.assign]
        .iter()
        .map(|r| r.cells)
        .max()
        .unwrap_or(0);
    let clock = Instant::now();
    let limit = 4_000_000;
    // The most cells: one more than the best known, until no plan reaches it.
    let mut most = known;
    case.proven = true;
    loop {
        let mut dfs = Dfs::new(me, t, most + 1, None, 0, limit);
        dfs.search(state.clone(), 0);
        if dfs.aborted {
            case.proven = false;
            break;
        }
        if dfs.best < 1 {
            break;
        }
        most += 1;
    }
    // The most middle chokes and middle cells within 0, 1, 2 cells of it,
    // starting from what the openings reach.
    let openings = [case.v029, case.b, case.rules, case.assign];
    for (zone, which) in [(chokes, 0), (band, 1)] {
        let mut previous = -1;
        for d in 0..3 {
            let floor = most - d as i64;
            let of = |r: &Reached| if which == 0 { r.chokes } else { r.middle };
            // A plan within d − 1 cells is within d: the narrower margin's
            // result is a floor too.
            let known = openings
                .iter()
                .filter(|r| r.cells >= floor)
                .map(of)
                .max()
                .unwrap_or(-1)
                .max(previous);
            let found = if zone.is_empty() {
                0
            } else {
                let mut dfs = Dfs::new(me, t, floor, Some(zone), known, limit);
                dfs.search(state.clone(), 0);
                case.proven &= !dfs.aborted;
                dfs.best.max(0)
            };
            previous = found;
            if which == 0 {
                case.chokes_within[d] = found;
            } else {
                case.middle_within[d] = found;
            }
        }
    }
    case.best = Reached {
        cells: most,
        chokes: case.chokes_within[0],
        middle: case.middle_within[0],
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
        for t in [3, 4] {
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
                    .filter(|&c| c % 5 == 0)
                    .fold(Bits::EMPTY, |b, c| b | Bits::cell(c));
            for d in 0..2 {
                let floor = most - d;
                let expected = full
                    .iter()
                    .filter(|k| cells(k) >= floor)
                    .map(|k| i64::from((from_words(k.0) & zone).count()))
                    .max()
                    .unwrap_or(0);
                let mut dfs = Dfs::new(seat, t, floor, Some(zone), -1, u64::MAX);
                dfs.search(state.clone(), 0);
                assert_eq!(
                    dfs.best, expected,
                    "seed {seed}, seat {seat}, {t} turns, floor {floor}"
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
        c.v029.cells,
        c.v029.chokes,
        c.v029.middle,
        c.b.cells,
        c.b.chokes,
        c.rules.cells,
        c.assign.cells,
        c.assign.chokes,
        c.player.map(|p| format!("{p:.2}")).unwrap_or_default(),
        c.peak,
        c.ms
    )
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
    let (mut per_seat, mut jobs, mut cap) = (50usize, 4usize, 1_000_000usize);
    let mut horizons: Vec<u32> = vec![6, 7, 8];
    let (mut csv, mut do_check) = (None::<String>, false);
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
    let done = &done;
    let sink = &sink;
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
                        for &t in horizons {
                            let p = player[(t as usize).saturating_sub(6).min(2)];
                            let case = run(seed, seat, t, cap, p, &open);
                            lines.push_str(&csv_line(&case));
                            out.push(case);
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
                m(&|c| c.v029.cells as f64),
                m(&|c| c.v029.chokes as f64),
                m(&|c| c.b.cells as f64),
                m(&|c| c.b.chokes as f64),
                m(&|c| c.rules.cells as f64),
                m(&|c| c.assign.cells as f64),
                m(&|c| c.assign.chokes as f64),
                mean(all.iter().filter_map(|c| c.player)),
                all.iter().map(|c| c.ms).max().unwrap_or(0),
            );
        }
    }
}
