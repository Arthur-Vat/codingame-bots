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
    opening_over(order, 9)
}

/// The opening planned over `turns` of our turns (the bot plans over 9).
fn opening_over(order: Order, turns: usize) -> Planned {
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
        turns,
        width: 32,
        objective: Objective { order, ..OBJECTIVE },
        replan_from: 0,
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
    if path == "replay" {
        let file = std::env::args().nth(2).expect("a plies file");
        replay(&std::fs::read_to_string(file).expect("read"));
        return;
    }
    if path == "adhere" {
        let args: Vec<String> = std::env::args().skip(2).collect();
        adhere(&args);
        return;
    }
    if path == "towers" {
        let args: Vec<String> = std::env::args().skip(2).collect();
        towers(&args);
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
        ["BUILD", "TOWER", x, y] => Some(Command::BuildTower {
            x: x.parse().ok()?,
            y: y.parse().ok()?,
        }),
        ["BUILD", "MINE", x, y] => Some(Command::BuildMine {
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
            ("reach", Field::Reachable { top: 7, within: 9 }),
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
        let orders = [
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
            (
                "reach",
                Order::Field(Field::Reachable { top: 7, within: 9 }),
            ),
            ("outward", Order::Field(Field::Outward)),
            ("voronoi", Order::Voronoi),
            ("behind", Order::BehindFront),
            (
                "claim1",
                Order::Claim {
                    band: 1,
                    future_first: false,
                    moves: false,
                },
            ),
            (
                "claim2",
                Order::Claim {
                    band: 2,
                    future_first: false,
                    moves: false,
                },
            ),
            (
                "future",
                Order::Claim {
                    band: 2,
                    future_first: true,
                    moves: false,
                },
            ),
            (
                "moves",
                Order::Claim {
                    band: 2,
                    future_first: false,
                    moves: true,
                },
            ),
        ];
        // Longer plans (the owner, 2026-11-20): the same orders planned over
        // 12 and 15 turns.
        let claim2 = Order::Claim {
            band: 2,
            future_first: false,
            moves: false,
        };
        let longer = [
            ("claim2-12", claim2, 12),
            ("claim2-15", claim2, 15),
            ("fr8-12", Order::Frontier { cap: 8 }, 12),
            ("fr8-15", Order::Frontier { cap: 8 }, 15),
            ("v029-15", Order::Sum, 15),
        ];
        // `PLACES_ONLY=a,b`: only these openings.
        let only = std::env::var("PLACES_ONLY").ok();
        let wanted = |name: &str| {
            only.as_ref()
                .is_none_or(|o| o.split(',').any(|n| n == name))
        };
        for (name, order, turns) in orders
            .into_iter()
            .map(|(n, o)| (n, o, 9))
            .chain(longer)
            .filter(|(n, _, _)| wanted(n))
        {
            let mut planned = opening_over(order, turns);
            let mut s = state.clone();
            for turn in 1..=10 {
                if s.outcome().is_none() {
                    let c = planned.turn(&s, None);
                    s = solo::advance(&s, &c);
                }
                if turn >= 7 && turn != 9 {
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

/// `replay FILE`: a game from its seed (line `SEED n`) and its answers, one
/// ply a line (the commands separated by `;`), played through the engine;
/// prints both sides' land, units (`cell:level`), towers and mines at the
/// end of each game turn.
fn replay(text: &str) {
    let mut lines = text.lines();
    let seed: u64 = lines
        .next()
        .and_then(|l| l.strip_prefix("SEED "))
        .and_then(|n| n.trim().parse().ok())
        .expect("SEED n");
    let map = generate(seed, RefereeLeague::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    println!("VALID {}", cells(state.map().valid));
    for (ply, line) in lines.enumerate() {
        if state.outcome().is_some() {
            break;
        }
        let commands: Vec<Command> = line.split(';').filter_map(|c| command(c.trim())).collect();
        state.play_turn(&commands);
        if ply % 2 == 1 {
            let units = |seat: usize| {
                state
                    .units(seat)
                    .iter()
                    .map(|c| format!("{c}:{}", state.unit_level(c).unwrap_or(0)))
                    .collect::<Vec<_>>()
                    .join(",")
            };
            println!(
                "TURN {} | {} | {} | {} | {} | {} | {} | {} | {} | {} {} {} {}",
                ply / 2 + 1,
                cells(state.owned(0)),
                cells(state.active(0)),
                cells(state.owned(1)),
                cells(state.active(1)),
                units(0),
                units(1),
                cells(state.towers()),
                cells(state.mines()),
                state.gold(0),
                state.income(0),
                state.gold(1),
                state.income(1)
            );
        }
    }
}

/// `adhere ORDER SEAT FILE...`: how closely a game followed the opening's
/// plan (E073, measurement only). Each FILE is a game as `replay` reads it,
/// SEAT the side measured, ORDER `v029` or `lin7` (the opening's order).
/// At the start of that side's game turn 6, its last opening turn (the
/// search plays from game turn 7), the opening plans again, untimed (beams
/// up to 32), the opponent passing; the plan's cells at the start of game
/// turns 8, 10 and 12 are its targets. Prints, per game and turn, the
/// targets, the cells held then, and the share of the targets held.
fn adhere(args: &[String]) {
    let order = match args[0].as_str() {
        "v029" => Order::Sum,
        "lin7" => Order::Central(Decay::Linear(7)),
        other => panic!("unknown order {other}"),
    };
    let seat: usize = args[1].parse().expect("a seat");
    let sets: Vec<Params> = solo::SETS
        .iter()
        .map(|&params| Params {
            safe_spot: u8::MAX,
            crowd: 2,
            horizon: 18,
            after_contact: 0,
            ..params
        })
        .collect();
    let objective = Objective { order, ..OBJECTIVE };
    let mut sums = [(0.0f64, 0usize); 3];
    for file in &args[2..] {
        let text = std::fs::read_to_string(file).expect("read");
        let mut lines = text.lines();
        let seed: u64 = lines
            .next()
            .and_then(|l| l.strip_prefix("SEED "))
            .and_then(|n| n.trim().parse().ok())
            .expect("SEED n");
        let map = generate(seed, RefereeLeague::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let mut targets: Option<[Bits; 3]> = None;
        let mut shares = Vec::new();
        for line in lines {
            if state.outcome().is_some() {
                break;
            }
            if state.to_act() == seat {
                let turn = state.game_turn();
                if turn == 6 {
                    let plan = solo::plan(&state, &sets, solo::REST, 9, 32, None, &objective);
                    let at = |k: usize| solo::play(&state, &sets, &plan, solo::REST, k).owned(seat);
                    targets = Some([at(2), at(4), at(6)]);
                }
                if let Some(t) = targets {
                    if let Some(i) = [8, 10, 12].iter().position(|&g| g == turn) {
                        let held = state.owned(seat);
                        let share =
                            f64::from((t[i] & held).count()) / f64::from(t[i].count().max(1));
                        shares.push(format!("{turn}:{:.2}", share));
                        sums[i].0 += share;
                        sums[i].1 += 1;
                    }
                }
            }
            let commands: Vec<Command> =
                line.split(';').filter_map(|c| command(c.trim())).collect();
            state.play_turn(&commands);
        }
        println!("{file} {}", shares.join(" "));
    }
    for (i, turn) in [8, 10, 12].iter().enumerate() {
        let (sum, n) = sums[i];
        println!(
            "mean share of the plan's cells held at game turn {turn}: {:.3} over {n} games",
            sum / n.max(1) as f64
        );
    }
}

/// `towers SEAT FILE...`: for each tower the side SEAT builds in each game
/// (files as `replay` reads them), whether it was needed then: the turn is
/// played again without the tower, and the tower counts as needed when the
/// enemy, at its next turn, can take one of the cells it would guard (its
/// own cell and the side's active cells next to it) with its gold
/// (`Reach`, its units' moves free); else, whether the side's gold at its
/// next start of turn (gold left plus income, the enemy's turn aside) would
/// pay for it then. Also the game turns of the side's first tower and first
/// mine. Measurement only (E075's question, the owner, 2026-11-20).
fn towers(args: &[String]) {
    use ice_and_fire_tactics::chain::Reach;
    let seat: usize = args[0].parse().expect("a seat");
    let (mut built, mut needed, mut could_wait) = (0usize, 0usize, 0usize);
    let mut by_turn: std::collections::BTreeMap<u32, (usize, usize)> = Default::default();
    let (mut first_tower, mut first_mine, mut games) = (Vec::new(), Vec::new(), 0usize);
    let (mut mines_by_10, mut towers_by_10) = (0usize, 0usize);
    // Towers by what they spare at the enemy's next turn: nothing, at most a
    // cell (1 to 5), 2 to 9 cells' worth (6 to 29), more (30 to 999), the HQ.
    let mut prevented = [0usize; 5];
    let mut idle_affordable = 0usize;
    for file in &args[1..] {
        let text = std::fs::read_to_string(file).expect("read");
        let mut lines = text.lines();
        let seed: u64 = lines
            .next()
            .and_then(|l| l.strip_prefix("SEED "))
            .and_then(|n| n.trim().parse().ok())
            .expect("SEED n");
        let map = generate(seed, RefereeLeague::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        games += 1;
        let (mut tower_turn, mut mine_turn) = (None, None);
        for line in lines {
            if state.outcome().is_some() {
                break;
            }
            let commands: Vec<Command> =
                line.split(';').filter_map(|c| command(c.trim())).collect();
            if state.to_act() == seat {
                let turn = state.game_turn();
                for (i, c) in commands.iter().enumerate() {
                    match *c {
                        Command::BuildTower { x, y } => {
                            built += 1;
                            tower_turn.get_or_insert(turn);
                            if turn <= 10 {
                                towers_by_10 += 1;
                            }
                            // The turn without this tower.
                            let mut without = state.clone();
                            let others: Vec<Command> = commands
                                .iter()
                                .enumerate()
                                .filter(|&(j, _)| j != i)
                                .map(|(_, c)| *c)
                                .collect();
                            without.play_turn(&others);
                            if without.outcome().is_some() {
                                continue;
                            }
                            let cell = usize::from(y) * 12 + usize::from(x);
                            let guarded = (Bits::cell(cell) | Bits::cell(cell).neighbours())
                                & without.active(seat);
                            let enemy = 1 - seat;
                            let reach = Reach::new(&without, enemy, true);
                            let gold = without.gold(enemy);
                            let need = guarded.iter().any(|g| reach.cost[g] <= gold);
                            // What the enemy's best cut at its next turn takes
                            // without the tower, beyond what it takes with it
                            // (score::exposure: 3 a cell or a turn of a mine's
                            // income, units' value, 1000 for the HQ).
                            let mut with = state.clone();
                            with.play_turn(&commands);
                            let settings = ice_and_fire_opening::score::Settings::DEFAULT;
                            let gain =
                                ice_and_fire_opening::score::exposure(&without, seat, settings)
                                    - ice_and_fire_opening::score::exposure(&with, seat, settings);
                            let bucket = match gain {
                                i64::MIN..=0 => 0,
                                1..=5 => 1,
                                6..=29 => 2,
                                30..=999 => 3,
                                _ => 4,
                            };
                            prevented[bucket] += 1;
                            if bucket <= 1 && without.gold(seat) + without.income(seat) >= 15 {
                                idle_affordable += 1;
                            }
                            let entry = by_turn.entry(turn).or_default();
                            entry.0 += 1;
                            if need {
                                needed += 1;
                                entry.1 += 1;
                            } else if without.gold(seat) + without.income(seat) >= 15 {
                                could_wait += 1;
                            }
                        }
                        Command::BuildMine { .. } => {
                            mine_turn.get_or_insert(turn);
                            if turn <= 10 {
                                mines_by_10 += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
            state.play_turn(&commands);
        }
        if let Some(t) = tower_turn {
            first_tower.push(t);
        }
        if let Some(t) = mine_turn {
            first_mine.push(t);
        }
    }
    let median = |v: &mut Vec<u32>| {
        v.sort_unstable();
        v.get(v.len() / 2).copied().unwrap_or(0)
    };
    println!(
        "games {games}, towers built {built} ({:.1} a game)",
        built as f64 / games.max(1) as f64
    );
    println!(
        "needed (the enemy could take a guarded cell at its next turn): {needed} ({:.0}%)",
        100.0 * needed as f64 / built.max(1) as f64
    );
    println!(
        "not needed, and our gold next turn would have paid: {could_wait} ({:.0}%)",
        100.0 * could_wait as f64 / built.max(1) as f64
    );
    println!(
        "first tower: in {} games, median game turn {}; first mine: in {} games, median game turn {}",
        first_tower.len(),
        median(&mut first_tower),
        first_mine.len(),
        median(&mut first_mine)
    );
    println!("towers by game turn 10: {towers_by_10}; mines by game turn 10: {mines_by_10}");
    let pct = |n: usize| 100.0 * n as f64 / built.max(1) as f64;
    println!(
        "what the tower spares at the enemy's next cut: nothing {} ({:.0}%), a cell at most {} ({:.0}%), 2-9 cells {} ({:.0}%), more {} ({:.0}%), the HQ {} ({:.0}%)",
        prevented[0], pct(prevented[0]), prevented[1], pct(prevented[1]), prevented[2], pct(prevented[2]),
        prevented[3], pct(prevented[3]), prevented[4], pct(prevented[4])
    );
    println!(
        "sparing a cell at most, with our gold next turn enough for it: {idle_affordable} ({:.0}%)",
        pct(idle_affordable)
    );
    let line: Vec<String> = by_turn
        .iter()
        .filter(|(t, _)| **t <= 20)
        .map(|(t, (b, n))| format!("{t}:{b}/{n}"))
        .collect();
    println!("by game turn (built/needed): {}", line.join(" "));
}
