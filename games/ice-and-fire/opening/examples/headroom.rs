//! The headroom of the opening's search (stage S0 of the goal-directed
//! opening): `ice-and-fire-v026`'s opening (its weight sets, objective and
//! 9-turn plans, copied from `bots/plan/src/main.rs`) played by one side
//! alone, the other passing, with more time for its first turn's plan than
//! the bot has. If more time builds a better position by turn 8, the
//! opening is limited by its search; if not, by what its plans can express.
//!
//! ```sh
//! cargo run --release -p ice-and-fire-opening --example headroom -- \
//!     --maps 100 --ms 608,2400,9600 --jobs 4
//! ```
//!
//! Each side of each source map (seeds from `--first-seed`) plays 7 turns
//! with each budget, the first turn's plan searched for that many
//! milliseconds (608: 76% of the bot's first turn), later replans with 34
//! (the bot's 45 ms turn); `rules` is the plan of the default step alone
//! (no search). The position at the start of the 8th turn is measured:
//! active cells, income, gold, the cells held in the *middle* (within 2
//! steps of even between the HQs) and the value of the plans' objective.

use std::time::{Duration, Instant};

use ice_and_fire_engine::{Bits, League, State, HQ};
use ice_and_fire_opening::econ::{distances, Params};
use ice_and_fire_opening::planned::{Planned, Planner};
use ice_and_fire_opening::solo::{self, Objective};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

/// v026's objective (`OBJECTIVE` in the plan bot).
const OBJECTIVE: Objective = Objective {
    cells: 3,
    income: 1,
    gold: 1,
    cut: 2,
    joints: 2,
    tips: 2,
    reach: 6,
    voronoi: 1,
};

/// v026's opening, as the plan bot builds it.
fn opening() -> Planned {
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
        objective: OBJECTIVE,
    };
    Planned::new(planner, sets, Params::V003)
}

#[derive(Clone, Copy, Default)]
struct Measure {
    cells: i64,
    income: i64,
    gold: i64,
    middle: i64,
    value: i64,
}

/// `seat` alone on the map of `seed` for 7 turns; `first_ms` 0 plays the
/// default step with no search.
fn play(seed: u64, seat: usize, first_ms: u64) -> Measure {
    let map = generate(seed, RefereeLeague::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    if seat == 1 {
        state.play_turn(&[]);
    }
    let mut planned = opening();
    let sets: Vec<Params> = planned.sets.clone();
    for turn in 0..7 {
        if state.outcome().is_some() {
            break;
        }
        let commands = if first_ms == 0 {
            solo::commands(&state, &sets, solo::REST)
        } else {
            let ms = if turn == 0 { first_ms } else { 34 };
            planned.turn(&state, Some(Instant::now() + Duration::from_millis(ms)))
        };
        state = solo::advance(&state, &commands);
    }
    let ours = distances(&state, Bits::cell(HQ[seat]));
    let theirs = distances(&state, Bits::cell(HQ[1 - seat]));
    let middle = state
        .owned(seat)
        .iter()
        .filter(|&c| (i32::from(ours[c]) - i32::from(theirs[c])).abs() < 3)
        .count() as i64;
    Measure {
        cells: i64::from(state.active(seat).count()),
        income: state.income(seat),
        gold: state.gold(seat),
        middle,
        value: solo::value(&state, seat, &OBJECTIVE),
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let (mut maps, mut first, mut jobs) = (100u64, 1u64, 4usize);
    let mut budgets: Vec<u64> = vec![608, 2400, 9600];
    while let Some(arg) = args.next() {
        let mut value = || args.next().unwrap_or_default();
        match arg.as_str() {
            "--maps" => maps = value().parse().unwrap_or(maps),
            "--first-seed" => first = value().parse().unwrap_or(first),
            "--jobs" => jobs = value().parse().unwrap_or(jobs).max(1),
            "--ms" => budgets = value().split(',').filter_map(|s| s.parse().ok()).collect(),
            other => {
                eprintln!("headroom: unknown argument {other}");
                std::process::exit(2);
            }
        }
    }
    budgets.insert(0, 0);
    let cases: Vec<(u64, usize)> = (first..first + maps)
        .flat_map(|seed| [(seed, 0), (seed, 1)])
        .collect();
    println!("| Seat | Plan | Sides | Cells | Income | Gold | Middle held | Objective | Objective over the bot's (95%) |");
    println!("| ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |");
    let mut base: Vec<Measure> = Vec::new();
    for seat in 0..2 {
        for &ms in &budgets {
            let mine: Vec<(u64, usize)> = cases.iter().copied().filter(|c| c.1 == seat).collect();
            let size = mine.len().div_ceil(jobs).max(1);
            let results: Vec<Measure> = std::thread::scope(|scope| {
                let handles: Vec<_> = mine
                    .chunks(size)
                    .map(|chunk| {
                        scope.spawn(move || {
                            chunk
                                .iter()
                                .map(|&(s, seat)| play(s, seat, ms))
                                .collect::<Vec<_>>()
                        })
                    })
                    .collect();
                handles
                    .into_iter()
                    .flat_map(|h| h.join().unwrap_or_default())
                    .collect()
            });
            if ms == budgets.get(1).copied().unwrap_or(0) {
                base = results.clone();
            }
            let n = results.len() as f64;
            let mean = |f: fn(&Measure) -> i64| results.iter().map(f).sum::<i64>() as f64 / n;
            let gain = if base.len() == results.len() && ms > budgets[1] {
                let d: Vec<f64> = results
                    .iter()
                    .zip(&base)
                    .map(|(a, b)| (a.value - b.value) as f64)
                    .collect();
                let m = d.iter().sum::<f64>() / n;
                let var = d.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (n - 1.0).max(1.0);
                format!("{m:+.1} ± {:.1}", 1.96 * (var / n).sqrt())
            } else {
                "-".to_string()
            };
            let name = if ms == 0 {
                "rules".to_string()
            } else {
                format!("{ms} ms")
            };
            println!(
                "| {seat} | {name} | {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {gain} |",
                results.len(),
                mean(|m| m.cells),
                mean(|m| m.income),
                mean(|m| m.gold),
                mean(|m| m.middle),
                mean(|m| m.value),
            );
        }
    }
}
