//! One player alone: the opening of one side on an empty map, the other side
//! waiting, up to the end of the early game (E010). It compares opening
//! policies by what they build alone, before any contact: territory, economy
//! and shape (`ice_and_fire_opening::solo`).
//!
//! ```sh
//! cargo run --release -p ice-and-fire-opening --example solo -- \
//!     --policy current:rules --policy compact:rules:2:6 \
//!     --policy planned:beam:200:600 --maps 200 --boards 10
//! ```
//!
//! A policy is either `NAME:rules[:GROUP:THIN:SPOT:INSIDE:TOWARD]`, the
//! opening's rules with v003's weights and those terms (`econ::Params`), or
//! `NAME:beam:WIDTH:MS`, a plan of all the turns found on the first by
//! `solo::plan` (beams of growing width until `MS` milliseconds are spent;
//! `WIDTH` is unused with a deadline) over the weight sets of `solo::SETS`,
//! then followed. Each side of each map plays
//! `--turns` turns (7: the median middle game starts at turn 8), then the
//! position at the start of its next turn is measured. Both seats of each map
//! are played, so a map counts twice.
//!
//! Shape: a **joint** is an active cell whose loss cuts other cells off from
//! the HQ; **cut** is the most cells one joint cuts off; a **tip** is an
//! active cell other than the HQ with at most one active neighbour of its
//! own, the end of a line; **reach** is the steps from the territory to the
//! enemy HQ; **value** is `solo::value` with `solo::Objective::DEFAULT`
//! (`--reach W` and `--income W` set its weights of the steps to the enemy
//! HQ and of the income, for the beam and the value).

use std::fmt::Write as _;
use std::time::{Duration, Instant};

use ice_and_fire_engine::{League, State, HQ, SIZE};
use ice_and_fire_opening::econ::{rules_turn_with, Params};
use ice_and_fire_opening::solo::{self, Objective, Shape};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

enum Policy {
    Rules(Params),
    Beam { width: usize, ms: u64 },
}

/// The board of `seat`'s side only, two characters a cell: `R`/`B` the
/// owner (lower case when cut off), then `.` empty, `1`-`3` a unit, `M` a
/// mine, `H` the HQ, `*` an empty mine spot; ` .` neutral, `##` void.
fn board(state: &State, seat: usize) -> Vec<String> {
    let mut lines = vec![format!(
        "   {}",
        (0..SIZE)
            .map(|x| format!("{x:>2}"))
            .collect::<Vec<_>>()
            .join(" ")
    )];
    for y in 0..SIZE {
        let mut line = format!("{y:>2}");
        for x in 0..SIZE {
            let cell = y * SIZE + x;
            let text = if !state.map().valid.test(cell) {
                "##".to_string()
            } else {
                let first = if state.owned(seat).test(cell) {
                    let letter = ['R', 'B'][seat];
                    if state.active(seat).test(cell) {
                        letter
                    } else {
                        letter.to_ascii_lowercase()
                    }
                } else {
                    ' '
                };
                let second = if cell == HQ[seat] {
                    'H'
                } else if let Some(level) = state.unit_level(cell) {
                    char::from(b'0' + level)
                } else if state.mines().test(cell) {
                    'M'
                } else if state.map().spots.test(cell) {
                    '*'
                } else {
                    '.'
                };
                format!("{first}{second}")
            };
            let _ = write!(line, " {text}");
        }
        lines.push(line);
    }
    lines
}

/// Plays `seat`'s opening alone for `turns` turns on the map of `seed`,
/// printing each turn when `trace`; also the first turn's thinking time.
fn play(
    seed: u64,
    seat: usize,
    policy: &Policy,
    turns: usize,
    trace: bool,
    objective: &Objective,
) -> (State, Duration) {
    let map = generate(seed, RefereeLeague::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    if seat == 1 {
        state.play_turn(&[]);
    }
    let started = Instant::now();
    let plan = match policy {
        Policy::Beam { width, ms } => solo::plan(
            &state,
            &solo::SETS,
            solo::REST,
            turns,
            *width,
            Some(started + Duration::from_millis(*ms)),
            objective,
        ),
        Policy::Rules(_) => Vec::new(),
    };
    let thought = started.elapsed();
    for turn in 0..turns {
        if state.outcome().is_some() {
            break;
        }
        let commands = match policy {
            Policy::Rules(params) => rules_turn_with(&state, params),
            Policy::Beam { .. } => solo::commands(
                &state,
                &solo::SETS,
                plan.get(turn).copied().unwrap_or(solo::REST),
            ),
        };
        if trace {
            println!(
                "  turn {}: gold {:3} income {:3} | {}",
                turn + 1,
                state.gold(seat),
                state.income(seat),
                ice_and_fire_engine::answer_line(&commands)
            );
        }
        state = solo::advance(&state, &commands);
    }
    (state, thought)
}

fn parse_policy(spec: &str) -> (String, Policy) {
    let parts: Vec<&str> = spec.split(':').collect();
    let num = |i: usize| {
        parts
            .get(i)
            .and_then(|p| p.parse::<i64>().ok())
            .unwrap_or(0)
    };
    let name = parts[0].to_string();
    match parts.get(1).copied() {
        Some("beam") => (
            name,
            Policy::Beam {
                width: num(2).max(1) as usize,
                ms: num(3).max(1) as u64,
            },
        ),
        _ => (
            name,
            Policy::Rules(
                Params::V003
                    .with_shape(num(2), num(3), num(4))
                    .with_course(num(5), num(6)),
            ),
        ),
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mut policies: Vec<(String, Policy)> = Vec::new();
    let (mut maps, mut first, mut turns, mut boards) = (100u64, 1u64, 7usize, 0usize);
    let mut trace: Option<u64> = None;
    let mut objective = Objective::DEFAULT;
    while let Some(arg) = args.next() {
        let mut value = || args.next().unwrap_or_default();
        match arg.as_str() {
            "--policy" => policies.push(parse_policy(&value())),
            "--maps" => maps = value().parse().unwrap_or(maps),
            "--first-seed" => first = value().parse().unwrap_or(first),
            "--turns" => turns = value().parse().unwrap_or(turns),
            "--boards" => boards = value().parse().unwrap_or(boards),
            "--trace" => trace = value().parse().ok(),
            "--reach" => objective.reach = value().parse().unwrap_or(objective.reach),
            "--income" => objective.income = value().parse().unwrap_or(objective.income),
            other => {
                eprintln!("solo: unknown argument {other}");
                std::process::exit(2);
            }
        }
    }
    if policies.is_empty() {
        policies.push(("current".to_string(), Policy::Rules(Params::V003)));
    }
    if let Some(seed) = trace {
        for (name, policy) in &policies {
            println!("{name}, map {seed}, red alone:");
            play(seed, 0, policy, turns, true, &objective);
        }
        return;
    }

    println!("| Policy | Value | Cells | Income | Gold | Joints | Most cut off by one cell | Tips | Reach | First turn ms (max) |");
    println!("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");
    for (name, policy) in &policies {
        let (mut total, mut value, mut slowest) = (Shape::default(), 0i64, Duration::ZERO);
        for seed in first..first + maps {
            for seat in 0..2 {
                let (state, thought) = play(seed, seat, policy, turns, false, &objective);
                let s = solo::shape(&state, seat);
                value += solo::value(&state, seat, &Objective::DEFAULT);
                total.cells += s.cells;
                total.income += s.income;
                total.gold += s.gold;
                total.joints += s.joints;
                total.cut += s.cut;
                total.tips += s.tips;
                total.reach += s.reach;
                slowest = slowest.max(thought);
            }
        }
        let n = (2 * maps) as f64;
        let mean = |v: i64| v as f64 / n;
        println!(
            "| {name} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {} |",
            mean(value),
            mean(total.cells),
            mean(total.income),
            mean(total.gold),
            mean(total.joints),
            mean(total.cut),
            mean(total.tips),
            mean(total.reach),
            slowest.as_millis()
        );
    }

    // Boards: red alone on the first `boards` maps, each policy side by side.
    for seed in first..first + boards as u64 {
        println!("\nMap {seed}, red alone after {turns} turns:");
        let states: Vec<(String, State)> = policies
            .iter()
            .map(|(name, p)| (name.clone(), play(seed, 0, p, turns, false, &objective).0))
            .collect();
        for (name, state) in &states {
            let s = solo::shape(state, 0);
            println!(
                "{name}: {} cells, {} income, {} joints, {} tips, reach {}",
                s.cells, s.income, s.joints, s.tips, s.reach
            );
        }
        let boards: Vec<Vec<String>> = states.iter().map(|(_, s)| board(s, 0)).collect();
        for row in 0..boards[0].len() {
            let line: Vec<String> = boards.iter().map(|b| format!("{:<40}", b[row])).collect();
            println!("{}", line.join("").trim_end());
        }
    }
}
