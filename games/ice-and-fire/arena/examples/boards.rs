//! Text boards of recorded games, for reading how the bots play.
//!
//! ```sh
//! cargo run --release -p ice-and-fire-arena --example boards -- GAME.json [--past N]
//! ```
//!
//! It replays each record through the engine and prints, at the start of
//! every red turn, both sides' gold, income, active cells, units, Voronoi
//! share and what the enemy's best cut would take from it (cells, units'
//! value) if the enemy had 30 or 50 gold, the board, then the commands each side played. From the middle
//! game (`ice_and_fire_opening::middle`) on it prints `--past` more turns in
//! full (6 by default), then one line a turn to the end. With `--every K` the
//! board itself is drawn only every K turns and at the middle game; the
//! other turns get one line.
//!
//! Board: `#` void, `.` neutral; red: `o` active cell, `;` inactive, `1` to
//! `3` a unit of that level, `m` a mine, `t` a tower, `H` the HQ; blue: `x`
//! active cell, `:` inactive, `A` to `C` a unit of level 1 to 3, `M`, `T`,
//! `Q` the HQ.

use std::fs;
use std::process::ExitCode;

use cg_arena::record::Record;
use ice_and_fire_engine::{Command, League, State, HQ, SIZE};
use ice_and_fire_opening::{middle, score};
use ice_and_fire_referee::map::generate;
use ice_and_fire_tactics::cut::cuts;

/// One command of an answer line, as the referee reads it; anything else is
/// skipped.
fn parse(part: &str) -> Option<Command> {
    let words: Vec<&str> = part.split_whitespace().collect();
    let num = |i: usize| words.get(i).and_then(|w| w.parse::<u32>().ok());
    let small = |i: usize| num(i).and_then(|n| u8::try_from(n).ok());
    match words.first().copied()? {
        "MOVE" => Some(Command::Move {
            id: num(1)?,
            x: small(2)?,
            y: small(3)?,
        }),
        "TRAIN" => Some(Command::Train {
            level: small(1)?,
            x: small(2)?,
            y: small(3)?,
        }),
        "BUILD" => {
            let (x, y) = (small(2)?, small(3)?);
            match words.get(1).copied()? {
                "MINE" => Some(Command::BuildMine { x, y }),
                "TOWER" => Some(Command::BuildTower { x, y }),
                _ => None,
            }
        }
        _ => None,
    }
}

/// The board as 12 lines.
fn board(state: &State) -> Vec<String> {
    let valid = state.map().valid;
    (0..SIZE)
        .map(|y| {
            (0..SIZE)
                .map(|x| {
                    let cell = y * SIZE + x;
                    if !valid.test(cell) {
                        return '#';
                    }
                    let seat = if state.owned(0).test(cell) {
                        0
                    } else if state.owned(1).test(cell) {
                        1
                    } else {
                        return '.';
                    };
                    let red = seat == 0;
                    if cell == HQ[seat] {
                        return if red { 'H' } else { 'Q' };
                    }
                    if let Some(level) = state.unit_level(cell) {
                        let base = if red { b'0' } else { b'@' };
                        return char::from(base + level);
                    }
                    if state.mines().test(cell) {
                        return if red { 'm' } else { 'M' };
                    }
                    if state.towers().test(cell) {
                        return if red { 't' } else { 'T' };
                    }
                    match (red, state.active(seat).test(cell)) {
                        (true, true) => 'o',
                        (true, false) => ';',
                        (false, true) => 'x',
                        (false, false) => ':',
                    }
                })
                .collect()
        })
        .collect()
}

/// What the enemy's best cut would take from `victim` if the enemy, to
/// act, had `budget` gold: cells taken or cut off, and units' value.
fn vulnerable(state: &State, victim: usize, budget: i64) -> (i64, i64) {
    let enemy = 1 - victim;
    let mut text = state.turn_input(enemy);
    let rest = text
        .split_once('\n')
        .map_or(String::new(), |(_, r)| r.to_string());
    text = format!("{budget}\n{rest}");
    let Ok(view) = State::read(
        &state.initial_input(enemy),
        &text,
        League::Bronze,
        state.game_turn(),
    ) else {
        return (0, 0);
    };
    cuts(&view, 5, budget, 8)
        .iter()
        .map(|c| (c.cells, c.units))
        .max_by_key(|&(cells, units)| 3 * cells + units)
        .unwrap_or((0, 0))
}

/// One side's numbers.
fn side(state: &State, seat: usize) -> String {
    let units = (state.units(seat) & state.active(seat)).count();
    let mines = (state.mines() & state.active(seat)).count();
    let (c30, u30) = vulnerable(state, seat, 30);
    let (c50, u50) = vulnerable(state, seat, 50);
    format!(
        "gold {:3} income {:3} cells {:2} units {:2} mines {} voronoi {:4.1} cut30 {c30}/{u30} cut50 {c50}/{u50}",
        state.gold(seat),
        state.income(seat),
        state.active(seat).count(),
        units,
        mines,
        score::voronoi_halves(state, seat) as f64 / 2.0
    )
}

fn print_game(path: &str, record: &Record, past: u32, every: u32) -> Result<(), String> {
    let map = generate(record.seed, ice_and_fire_referee::League::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let names = [&record.players[0].name, &record.players[1].name];
    let winner = match record.winner {
        Some(seat) => format!("{} ({})", names[seat], ["red", "blue"][seat]),
        None => "draw".to_string(),
    };
    println!(
        "=== {path}: red {} vs blue {}; winner {winner}; {:?}",
        names[0], names[1], record.end
    );
    let mut middle_turn: Option<u32> = None;
    let mut lines: [String; 2] = Default::default();
    for answers in &record.turns {
        if state.outcome().is_some() {
            break;
        }
        let seat = state.to_act();
        let turn = state.game_turn();
        if middle_turn.is_none() && middle::reached(&state) {
            middle_turn = Some(turn);
            println!(
                "--- middle game: {} can pay a chain to an enemy cell at turn {turn}",
                ["red", "blue"][seat]
            );
        }
        let full = middle_turn.is_none_or(|m| turn <= m + past);
        let drawn = full && (turn % every == 1 % every || middle_turn == Some(turn));
        if seat == 0 {
            if drawn {
                println!("turn {turn}");
                println!("  red  {}", side(&state, 0));
                println!("  blue {}", side(&state, 1));
                for row in board(&state) {
                    println!("  {row}");
                }
            } else {
                println!(
                    "turn {turn}: red {} | blue {}",
                    side(&state, 0),
                    side(&state, 1)
                );
            }
        }
        let line = answers
            .iter()
            .find(|a| a.seat == seat)
            .and_then(|a| a.lines.first())
            .cloned()
            .unwrap_or_default();
        let commands: Vec<Command> = line.split(';').filter_map(|p| parse(p.trim())).collect();
        lines[seat] = line;
        state.play_turn(&commands);
        if seat == 1 && full {
            println!("  red:  {}", lines[0]);
            println!("  blue: {}", lines[1]);
        }
    }
    println!(
        "--- end at turn {}: {:?}",
        state.game_turn(),
        state.outcome()
    );
    println!();
    Ok(())
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let mut files = Vec::new();
    let mut past = 6;
    let mut every = 1;
    while let Some(arg) = args.next() {
        if arg == "--past" {
            past = args.next().and_then(|n| n.parse().ok()).unwrap_or(past);
        } else if arg == "--every" {
            every = args
                .next()
                .and_then(|n| n.parse().ok())
                .unwrap_or(1u32)
                .max(1);
        } else {
            files.push(arg);
        }
    }
    if files.is_empty() {
        eprintln!("usage: boards GAME.json... [--past N] [--every K]");
        return ExitCode::from(2);
    }
    for path in files {
        let parsed = fs::read_to_string(&path)
            .map_err(|e| e.to_string())
            .and_then(|text| serde_json::from_str::<Record>(&text).map_err(|e| e.to_string()))
            .and_then(|record| print_game(&path, &record, past, every));
        if let Err(err) = parsed {
            eprintln!("boards: {path}: {err}");
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
