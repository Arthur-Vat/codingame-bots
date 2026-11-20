//! Scratch study for E073 (the owner's question of 2026-11-20): on maps whose
//! middle chokes lie in two or three separate places, how many of those
//! places does each opening hold, one side alone, the other passing?
//!
//! A place is a group of middle chokes (`ice_and_fire_early`'s zones), two
//! chokes within 2 steps of each other being in the same group. Starts: the
//! seeds and seats of the optimal plans' study (its `oracle.csv`).
//!
//! ```sh
//! cargo run --release -p ice-and-fire-opening --example places -- \
//!     games/ice-and-fire/research/codingame/2026-11-20-optimal-early-plans/oracle.csv [EXTRA_MAPS]
//! ```

use std::collections::BTreeSet;

use ice_and_fire_early::zones::Zones;
use ice_and_fire_engine::{Bits, Command, League, State};
use ice_and_fire_opening::econ::{distances, Params};
use ice_and_fire_opening::planned::{Planned, Planner};
use ice_and_fire_opening::solo::{self, Decay, Field, Objective, Order};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

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

/// Our turns played.
const TURNS: usize = 10;

fn opening(order: Order) -> Planned {
    let sets = solo::SETS
        .iter()
        .map(|&params| Params {
            safe_spot: u8::MAX,
            crowd: 2,
            horizon: 18,
            after_contact: 0,
            ..params
        })
        .collect();
    let planner = Planner::Solo {
        turns: 9,
        width: 32,
        objective: Objective { order, ..OBJECTIVE },
    };
    Planned::new(planner, sets, Params::V003)
}

fn start(seed: u64, seat: usize) -> State {
    let map = generate(seed, RefereeLeague::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    if seat == 1 {
        state.play_turn(&[]);
    }
    state
}

/// The middle chokes in groups, two within 2 steps in the same group.
fn places(state: &State) -> Vec<Bits> {
    let chokes = Zones::of(state).middle_chokes();
    let mut left = chokes;
    let mut out = Vec::new();
    while let Some(first) = left.iter().next() {
        let mut group = Bits::cell(first);
        loop {
            let mut grown = group;
            for c in group.iter() {
                let d = distances(state, Bits::cell(c));
                for o in left.iter() {
                    if d[o] <= 2 {
                        grown.set(o);
                    }
                }
            }
            if grown == group {
                break;
            }
            group = grown;
        }
        left = left.and_not(group);
        out.push(group);
    }
    out
}

// The turns index the per-turn tallies.
#[allow(clippy::needless_range_loop)]
fn main() {
    let path = std::env::args().nth(1).expect("oracle.csv");
    if path == "dump" {
        let args: Vec<String> = std::env::args().skip(2).collect();
        dump(&args[0], &args[1..]);
        return;
    }
    if path == "census" {
        census(
            std::env::args()
                .nth(2)
                .map_or(2000, |a| a.parse().expect("a count")),
        );
        return;
    }
    let text = std::fs::read_to_string(path).expect("read");
    let mut starts: BTreeSet<(u64, usize)> = text
        .lines()
        .skip(1)
        .map(|l| {
            let f: Vec<&str> = l.split(',').collect();
            (f[0].parse().unwrap(), f[1].parse().unwrap())
        })
        .collect();
    // More maps: the first `extra` seeds of a fixed sequence whose middle
    // chokes lie in 2 or 3 places, both seats.
    let extra: usize = std::env::args()
        .nth(2)
        .map_or(0, |a| a.parse().expect("a count"));
    let mut x: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut found = 0;
    while found < extra {
        x = x
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let seed = x >> 1;
        if (2..=3).contains(&places(&start(seed, 0)).len()) {
            starts.insert((seed, 0));
            starts.insert((seed, 1));
            found += 1;
        }
    }
    let orders = [
        ("v029", Order::Sum),
        ("tie12", Order::TieBreak { within: 12 }),
        ("land", Order::LandFirst),
    ];
    // Per order, places held after each of our turns, summed, and the
    // number of starts holding them all.
    let mut held = [[0usize; TURNS + 1]; 3];
    let mut all = [[0usize; TURNS + 1]; 3];
    let mut total_places = 0;
    let mut n = 0;
    println!("| Seed | Seat | Places (chokes each) | v029: places held after our turns 6 / 7 / 8 / 10 | tie12 | land |");
    println!("| --- | --- | --- | --- | --- | --- |");
    for &(seed, seat) in &starts {
        let state = start(seed, seat);
        let groups = places(&state);
        if groups.len() < 2 {
            continue;
        }
        n += 1;
        total_places += groups.len();
        let mut row = format!(
            "| {seed} | {} | {} ({}) |",
            ["red", "blue"][seat],
            groups.len(),
            groups
                .iter()
                .map(|g| g.count().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        for (i, &(_, order)) in orders.iter().enumerate() {
            let mut planned = opening(order);
            let mut s = state.clone();
            let mut counts = [0usize; TURNS + 1];
            for turn in 1..=TURNS {
                if s.outcome().is_none() {
                    let c = planned.turn(&s, None);
                    s = solo::advance(&s, &c);
                }
                let owned = s.owned(seat);
                counts[turn] = groups.iter().filter(|g| !(owned & **g).is_empty()).count();
                held[i][turn] += counts[turn];
                if counts[turn] == groups.len() {
                    all[i][turn] += 1;
                }
            }
            row += &format!(
                " {} / {} / {} / {} |",
                counts[6], counts[7], counts[8], counts[10]
            );
        }
        println!("{row}");
    }
    println!();
    println!("{n} starts with 2 or 3 places, {total_places} places in all.");
    println!();
    println!("| Opening | Our turn | Places held (share) | Starts holding every place |");
    println!("| --- | ---: | ---: | ---: |");
    for (i, (name, _)) in orders.iter().enumerate() {
        for turn in [4, 5, 6, 7, 8, 9, 10] {
            println!(
                "| {name} | {turn} | {} ({:.0}%) | {} of {n} |",
                held[i][turn],
                100.0 * held[i][turn] as f64 / total_places as f64,
                all[i][turn]
            );
        }
    }
}

/// `census N`: how many places the middle chokes of N maps lie in, and the
/// first seeds with 3 or more.
fn census(maps: usize) {
    let mut x: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut by: std::collections::BTreeMap<usize, usize> = Default::default();
    let mut three = Vec::new();
    for _ in 0..maps {
        x = x
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let seed = x >> 1;
        let state = start(seed, 0);
        let groups = places(&state);
        *by.entry(groups.len()).or_default() += 1;
        if groups.len() >= 3 && three.len() < 50 {
            three.push((seed, groups.iter().map(|g| g.count()).collect::<Vec<_>>()));
        }
    }
    println!("places: maps {by:?}");
    // With the grouping at 1 step (adjacent chokes only).
    for (seed, sizes) in three {
        println!("{seed}: {sizes:?}");
    }
}

/// One command as CodinGame reads it (moves and trainings only).
fn command(text: &str) -> Option<Command> {
    let w: Vec<&str> = text.split(' ').collect();
    match w.as_slice() {
        ["MOVE", id, x, y] => Some(Command::Move {
            id: id.parse().ok()?,
            x: x.parse().ok()?,
            y: y.parse().ok()?,
        }),
        ["TRAIN", level, x, y] => Some(Command::Train {
            level: level.parse().ok()?,
            x: x.parse().ok()?,
            y: y.parse().ok()?,
        }),
        _ => None,
    }
}

fn cells(bits: Bits) -> String {
    bits.iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

/// `dump PLANS SEED:SEAT...`: for pictures, each start's map, its middle
/// chokes and band, and the land and units after our turns 7 and 8 of
/// v029's opening, tie12, land and the study's best plan (the most cells,
/// then the most middle chokes).
fn dump(plans: &str, starts: &[String]) {
    let text = std::fs::read_to_string(plans).expect("plans");
    for spec in starts {
        let (seed, seat) = spec.split_once(':').expect("SEED:SEAT");
        let (seed, seat): (u64, usize) = (seed.parse().unwrap(), seat.parse().unwrap());
        let state = start(seed, seat);
        println!("START {seed} {seat}");
        println!("VALID {}", cells(state.map().valid));
        println!("CHOKES {}", cells(Zones::of(&state).middle_chokes()));
        println!("BAND {}", cells(solo::middle_band(&state)));
        for (name, field) in [
            ("corridor", Field::Corridor { top: 8 }),
            ("reach", Field::Reachable { top: 7, steps: 9 }),
            ("outward", Field::Outward),
        ] {
            println!(
                "FIELD {name} {}",
                solo::field_weights(&state, seat, field)
                    .iter()
                    .map(|w| w.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }
        println!(
            "RINGS {}",
            solo::rings(&state)
                .iter()
                .map(|r| if *r == i64::MAX { -1 } else { *r }.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
        let show = |name: &str, turn: usize, s: &State| {
            println!(
                "PANEL {name} {turn} {} | {} | {}",
                cells(s.owned(seat)),
                cells(s.active(seat)),
                cells(s.units(seat))
            );
        };
        for (name, order) in [
            ("v029", Order::Sum),
            ("tie12", Order::TieBreak { within: 12 }),
            ("land", Order::LandFirst),
            ("lin3", Order::Central(Decay::Linear(3))),
            ("half8", Order::Central(Decay::Halving(8))),
            ("lin5", Order::Central(Decay::Linear(5))),
            ("lin7", Order::Central(Decay::Linear(7))),
            ("lin9", Order::Central(Decay::Linear(9))),
            ("ring", Order::ByRing),
            ("fr8", Order::Frontier { cap: 8 }),
            ("corridor", Order::Field(Field::Corridor { top: 8 })),
            ("reach", Order::Field(Field::Reachable { top: 7, steps: 9 })),
            ("outward", Order::Field(Field::Outward)),
            ("voronoi", Order::Voronoi),
            ("behind", Order::BehindFront),
        ] {
            let mut planned = opening(order);
            let mut s = state.clone();
            for turn in 1..=8 {
                if s.outcome().is_none() {
                    let c = planned.turn(&s, None);
                    s = solo::advance(&s, &c);
                }
                if turn >= 7 {
                    show(name, turn, &s);
                }
            }
        }
        for turns in [7, 8] {
            let row = text.lines().find(|l| {
                let f: Vec<&str> = l.split('\t').collect();
                f.len() >= 9
                    && f[0] == seed.to_string()
                    && f[1] == seat.to_string()
                    && f[2] == turns.to_string()
                    && f[3] == "chokes"
                    && f[4] == "0"
            });
            let Some(row) = row else { continue };
            let plan = row.split('\t').nth(8).unwrap_or("");
            let mut s = state.clone();
            for turn in plan.split(" | ") {
                let c: Vec<Command> = turn.split(';').filter_map(command).collect();
                if s.outcome().is_none() {
                    s = solo::advance(&s, &c);
                }
            }
            show("best", turns, &s);
        }
    }
}
