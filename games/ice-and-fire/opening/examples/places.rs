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
use ice_and_fire_engine::{Bits, League, State};
use ice_and_fire_opening::econ::{distances, Params};
use ice_and_fire_opening::planned::{Planned, Planner};
use ice_and_fire_opening::solo::{self, Objective, Order};
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

fn main() {
    let path = std::env::args().nth(1).expect("oracle.csv");
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
        if !(2..=3).contains(&groups.len()) {
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
