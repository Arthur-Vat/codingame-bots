//! One player alone: the opening of one side on an empty map, the other side
//! waiting, up to the end of the early game (E010). It compares opening
//! policies by what they build alone, before any contact: territory, economy
//! and shape.
//!
//! ```sh
//! cargo run --release -p ice-and-fire-opening --example solo -- \
//!     --policy current:0:0 --policy compact:6:6 --maps 200 --boards 10
//! ```
//!
//! A policy is `NAME:GROUP:THIN:SPOT`: v003's weights with the shape terms
//! `group`, `thin` and `spot` (`econ::Params::with_shape`). Each side of each map
//! plays `--turns` turns (7: the median middle game starts at turn 8), then
//! the position at the start of its next turn is measured. Both seats of each
//! map are played, so a map counts twice.
//!
//! Shape: a **joint** is an active cell whose loss cuts other cells off from
//! the HQ; **cut** is the most cells one joint cuts off; a **tip** is an
//! active cell other than the HQ with at most one active neighbour of its
//! own, the end of a line.

use std::fmt::Write as _;

use ice_and_fire_engine::{Bits, League, State, CELLS, HQ, SIZE};
use ice_and_fire_opening::econ::{rules_turn_with, Params};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

/// What a side built alone.
#[derive(Clone, Copy, Default)]
struct Built {
    cells: i64,
    gold: i64,
    income: i64,
    units: i64,
    mines: i64,
    joints: i64,
    cut: i64,
    tips: i64,
}

impl Built {
    fn add(&mut self, other: &Built) {
        self.cells += other.cells;
        self.gold += other.gold;
        self.income += other.income;
        self.units += other.units;
        self.mines += other.mines;
        self.joints += other.joints;
        self.cut += other.cut;
        self.tips += other.tips;
    }
}

/// Depth-first search from the HQ over the active cells: for each cell, the
/// number of cells its loss would cut off (0 for a cell that is no joint).
fn severed(active: Bits, root: usize) -> [i64; CELLS] {
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
        active,
        disc: [0; CELLS],
        low: [0; CELLS],
        size: [0; CELLS],
        cut: [0; CELLS],
        time: 0,
    };
    if active.test(root) {
        visit(&mut d, root);
    }
    d.cut[root] = 0;
    d.cut
}

fn measure(state: &State, seat: usize) -> Built {
    let active = state.active(seat);
    let cut = severed(active, HQ[seat]);
    let mut built = Built {
        cells: i64::from(active.count()),
        gold: state.gold(seat),
        income: state.income(seat),
        units: i64::from((state.units(seat) & active).count()),
        mines: i64::from((state.mines() & active).count()),
        ..Built::default()
    };
    for cell in active.iter() {
        if cut[cell] > 0 {
            built.joints += 1;
            built.cut = built.cut.max(cut[cell]);
        }
        let own = (Bits::cell(cell).neighbours() & active).count();
        if cell != HQ[seat] && own <= 1 {
            built.tips += 1;
        }
    }
    built
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

/// Plays `seat`'s opening alone for `turns` turns on the map of `seed`.
fn play(seed: u64, seat: usize, params: &Params, turns: u32) -> State {
    play_traced(seed, seat, params, turns, false)
}

/// [`play`], printing each turn's gold and commands when `trace`.
fn play_traced(seed: u64, seat: usize, params: &Params, turns: u32, trace: bool) -> State {
    let map = generate(seed, RefereeLeague::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let mut played = 0;
    while state.outcome().is_none() {
        if state.to_act() == seat {
            if played == turns {
                break;
            }
            let commands = rules_turn_with(&state, params);
            if trace {
                println!(
                    "  turn {}: gold {:3} income {:3} | {}",
                    played + 1,
                    state.gold(seat),
                    state.income(seat),
                    ice_and_fire_engine::answer_line(&commands)
                );
            }
            state.play_turn(&commands);
            played += 1;
        } else {
            state.play_turn(&[]);
        }
    }
    state
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mut policies: Vec<(String, Params)> = Vec::new();
    let (mut maps, mut first, mut turns, mut boards) = (100u64, 1u64, 7u32, 0usize);
    let mut trace: Option<u64> = None;
    while let Some(arg) = args.next() {
        let mut value = || args.next().unwrap_or_default();
        match arg.as_str() {
            "--policy" => {
                let v = value();
                let parts: Vec<&str> = v.split(':').collect();
                let num = |i: usize| parts.get(i).and_then(|p| p.parse().ok()).unwrap_or(0);
                policies.push((
                    parts[0].to_string(),
                    Params::V003.with_shape(num(1), num(2), num(3)),
                ));
            }
            "--maps" => maps = value().parse().unwrap_or(maps),
            "--first-seed" => first = value().parse().unwrap_or(first),
            "--turns" => turns = value().parse().unwrap_or(turns),
            "--boards" => boards = value().parse().unwrap_or(boards),
            "--trace" => trace = value().parse().ok(),
            other => {
                eprintln!("solo: unknown argument {other}");
                std::process::exit(2);
            }
        }
    }
    if policies.is_empty() {
        policies.push(("current".to_string(), Params::V003));
    }
    if let Some(seed) = trace {
        for (name, params) in &policies {
            println!("{name}, map {seed}, red alone:");
            play_traced(seed, 0, params, turns, true);
        }
        return;
    }

    println!("| Policy | Cells | Income | Gold | Units | Mines | Joints | Most cut off by one cell | Tips |");
    println!("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");
    for (name, params) in &policies {
        let mut total = Built::default();
        for seed in first..first + maps {
            for seat in 0..2 {
                total.add(&measure(&play(seed, seat, params, turns), seat));
            }
        }
        let n = (2 * maps) as f64;
        let mean = |v: i64| v as f64 / n;
        println!(
            "| {name} | {:.1} | {:.1} | {:.1} | {:.1} | {:.2} | {:.1} | {:.1} | {:.1} |",
            mean(total.cells),
            mean(total.income),
            mean(total.gold),
            mean(total.units),
            mean(total.mines),
            mean(total.joints),
            mean(total.cut),
            mean(total.tips)
        );
    }

    // Boards: red alone on the first `boards` maps, each policy side by side.
    for seed in first..first + boards as u64 {
        println!("\nMap {seed}, red alone after {turns} turns:");
        let states: Vec<(String, State)> = policies
            .iter()
            .map(|(name, p)| (name.clone(), play(seed, 0, p, turns)))
            .collect();
        let mut header = String::new();
        for (name, state) in &states {
            let b = measure(state, 0);
            let _ = write!(
                header,
                "{:<40}",
                format!(
                    "{name}: {} cells, {} joints, {} tips",
                    b.cells, b.joints, b.tips
                )
            );
        }
        println!("{}", header.trim_end());
        let boards: Vec<Vec<String>> = states.iter().map(|(_, s)| board(s, 0)).collect();
        for row in 0..boards[0].len() {
            let line: Vec<String> = boards.iter().map(|b| format!("{:<40}", b[row])).collect();
            println!("{}", line.join("").trim_end());
        }
    }
}
