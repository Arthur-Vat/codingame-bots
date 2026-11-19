//! The exact early game of one side alone (E072, the "optimal plans" study
//! of phase 2 of the early-game plan,
//! `games/ice-and-fire/research/codingame/2026-11-19-early-game.md`).
//!
//! One side plays its first `T` turns, the other passing, under the
//! engine's rules: each turn its units step onto a neighbour or stay, then it
//! trains level-1 units on cells next to its active land (no mines, no
//! towers, no level 2 or 3: no opening builds them this early). The search
//! goes turn by turn over every such turn, merging identical states (our
//! cells, our units' places, our gold; unit ids do not matter), and keeps a
//! state only while an upper bound on its cells at the end could still come
//! within `--delta` of the best plan known (the openings below give the first
//! one). The bound is the economy with every unit taking a new cell every
//! turn and the gold spent on trainings as soon as it allows:
//!
//! ```text
//! each turn left: k = gold / 10; cells += units + k; units += k;
//!                 gold += cells − units − 10 k
//! ```
//!
//! A unit move that takes no cell (it stays or walks on our land) loses at
//! least one cell, so such moves are tried only when the bound leaves room.
//! `--check` runs the search without any pruning on short horizons and
//! asserts the same results.
//!
//! For each start (a map and seat of an opening suite, the reference player's
//! starts) and each horizon, it reports the most cells, the number of
//! distinct end territories that reach it, how many of them hold a middle
//! choke (`ice_and_fire_early`'s zones), the most middle chokes within 0, 1
//! and 2 cells of the most, and what the openings reach in the same model:
//! v029's (its solo planner, untimed), the same with the assignment's steps
//! (`B`), the rules alone and the assignment alone; and the reference
//! player's own active cells at the next game turn, from the suite (its
//! games are two-sided, so they are a pointer only).
//!
//! ```sh
//! cargo run --release -p ice-and-fire-opening --example oracle -- \
//!     --suite games/ice-and-fire/openings/suite-v1.tsv --player zasmu \
//!     --per-seat 50 --turns 6,7,8 --delta 2 --jobs 4 [--csv FILE] [--check]
//! ```

use std::collections::HashMap;
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

/// The end states of `me`'s `t` turns from `state`: with `prune`, those
/// whose cells may still reach `target` (a superset of those that do);
/// without it, all of them. `None` past `cap` states in a layer; else also
/// the most states in a layer.
fn solve(
    state: &State,
    me: usize,
    t: u32,
    target: i64,
    prune: bool,
    cap: usize,
) -> Option<(Vec<State>, usize)> {
    let mut layer: HashMap<Key, State> = HashMap::new();
    layer.insert(key(state, me), state.clone());
    let mut peak = 1;
    for done in 0..t {
        let left = t - done;
        let mut next: HashMap<Key, State> = HashMap::new();
        let mut over = false;
        for s in layer.values() {
            let slack = bound(s, me, left) - target;
            if prune && slack < 0 {
                continue;
            }
            let waste = !prune || slack >= 1;
            turns(s, me, waste, &mut |after| {
                if over {
                    return;
                }
                let child = end(after);
                if prune && bound(&child, me, left - 1) < target {
                    return;
                }
                next.entry(key(&child, me)).or_insert(child);
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
    Some((layer.into_values().collect(), peak))
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
}

fn run(
    seed: u64,
    seat: usize,
    t: u32,
    delta: i64,
    cap: usize,
    player: Option<f64>,
    open: &Openings,
) -> Case {
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
    let Some((finals, peak)) = solve(&state, me, t, known - delta, true, cap) else {
        case.too_large = true;
        case.ms = clock.elapsed().as_millis();
        return case;
    };
    case.ms = clock.elapsed().as_millis();
    case.peak = peak;
    let all: Vec<Reached> = finals
        .iter()
        .map(|s| reached(s, me, chokes, band))
        .collect();
    let most = all.iter().map(|r| r.cells).max().unwrap_or(0);
    let mut territories: HashMap<[u64; 3], bool> = HashMap::new();
    for (s, r) in finals.iter().zip(&all) {
        if r.cells == most {
            case.states += 1;
            let held = territories.entry(s.owned(me).words()).or_insert(false);
            *held |= r.chokes > 0;
        }
    }
    case.territories = territories.len();
    case.with_choke = territories.values().filter(|&&h| h).count();
    for d in 0..3 {
        let within = all.iter().filter(|r| r.cells >= most - d as i64);
        case.chokes_within[d] = within.clone().map(|r| r.chokes).max().unwrap_or(0);
        case.middle_within[d] = within.map(|r| r.middle).max().unwrap_or(0);
    }
    case.best = Reached {
        cells: most,
        chokes: case.chokes_within[0],
        middle: case.middle_within[0],
    };
    case
}

/// `--check`: the pruned search and the full one agree on short horizons.
fn check(starts: &[(u64, usize)]) {
    for &(seed, seat) in starts {
        for t in [3, 4] {
            let state = start(seed, seat);
            let (full, _) =
                solve(&state, seat, t, i64::MIN / 2, false, usize::MAX).expect("no cap");
            let most = full
                .iter()
                .map(|s| i64::from(s.active(seat).count()))
                .max()
                .unwrap_or(0);
            let optimal = |states: &[State]| {
                let mut keys: Vec<Key> = states
                    .iter()
                    .filter(|s| i64::from(s.active(seat).count()) == most)
                    .map(|s| key(s, seat))
                    .collect();
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
    let (mut per_seat, mut jobs, mut delta, mut cap) = (50usize, 4usize, 2i64, 3_000_000usize);
    let mut horizons: Vec<u32> = vec![6, 7, 8];
    let (mut csv, mut do_check) = (None::<String>, false);
    while let Some(arg) = args.next() {
        let mut value = || args.next().unwrap_or_default();
        match arg.as_str() {
            "--suite" => suite = value(),
            "--player" => player = value(),
            "--per-seat" => per_seat = value().parse().unwrap_or(per_seat),
            "--jobs" => jobs = value().parse().unwrap_or(jobs).max(1),
            "--delta" => delta = value().parse().unwrap_or(delta).clamp(0, 2),
            "--cap" => cap = value().parse().unwrap_or(cap),
            "--turns" => horizons = value().split(',').filter_map(|s| s.parse().ok()).collect(),
            "--csv" => csv = Some(value()),
            "--check" => do_check = true,
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
                        let open = openings(&start(seed, seat), longest);
                        for &t in horizons {
                            let p = player[(t as usize).saturating_sub(6).min(2)];
                            out.push(run(seed, seat, t, delta, cap, p, &open));
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
    println!("| Turns | Seat | Starts | Solved | Most cells | Optimal territories: median / mean / max | Optimal territories holding a middle choke (maps with one) | Middle chokes within 0 / 1 / 2 cells | v029: cells / chokes | B: cells / chokes | Rules: cells | Assignment: cells / chokes | Player at next turn | Peak states, ms (max) |");
    println!("| ---: | --- | ---: | ---: | ---: | --- | --- | --- | --- | --- | ---: | --- | ---: | --- |");
    for &t in horizons {
        for seat in 0..2 {
            let all: Vec<&Case> = rows
                .iter()
                .filter(|c| c.turns == t && c.seat == seat)
                .collect();
            let solved: Vec<&Case> = all.iter().copied().filter(|c| !c.too_large).collect();
            if all.is_empty() {
                continue;
            }
            let m = |f: &dyn Fn(&Case) -> f64| mean(solved.iter().map(|c| f(c)));
            let with_map: Vec<&Case> = solved
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
                format!(
                    "{any} of {} maps; {:.0}% of their optimal territories",
                    with_map.len(),
                    100.0 * frac
                )
            };
            let name = if seat == 0 { "red" } else { "blue" };
            println!(
                "| {t} | {name} | {} | {} | {:.2} | {} / {:.0} / {} | {share} | {:.2} / {:.2} / {:.2} | {:.2} / {:.2} | {:.2} / {:.2} | {:.2} | {:.2} / {:.2} | {:.2} | {} , {} |",
                all.len(),
                solved.len(),
                m(&|c| c.best.cells as f64),
                median(solved.iter().map(|c| c.territories).collect()),
                m(&|c| c.territories as f64),
                solved.iter().map(|c| c.territories).max().unwrap_or(0),
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
                mean(solved.iter().filter_map(|c| c.player)),
                solved.iter().map(|c| c.peak).max().unwrap_or(0),
                solved.iter().map(|c| c.ms).max().unwrap_or(0),
            );
        }
    }
    if let Some(path) = csv {
        let mut out = String::from("seed,seat,turns,too_large,map_chokes,most_cells,territories,states,with_choke,chokes0,chokes1,chokes2,middle0,middle1,middle2,v029_cells,v029_chokes,v029_middle,b_cells,b_chokes,rules_cells,assign_cells,assign_chokes,player_next,peak,ms\n");
        for c in &rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                c.seed,
                c.seat,
                c.turns,
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
            ));
        }
        if let Err(e) = std::fs::write(&path, out) {
            eprintln!("oracle: cannot write {path}: {e}");
        }
    }
}
