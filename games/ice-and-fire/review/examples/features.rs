//! Writes the terms of the `plan` bot's evaluation, measured at the start of
//! every middle-game ply of recorded games, with whether the side to act went
//! on to win: the data to fit the evaluation's weights (E017).
//!
//! ```sh
//! cargo run --release -p ice-and-fire-review --example features -- RECORDS_DIR > features.csv
//! ```
//!
//! One CSV line a ply from the first ply where the side to act can pay a
//! chain of trainings to an enemy cell (`ice_and_fire_opening::middle`), for
//! games that end with a winner. Every term is the side to act's (`own_`) and
//! its opponent's (`opp_`), as the evaluation computes them.

use std::fs;
use std::path::Path;

use cg_arena::record::Record;
use ice_and_fire_engine::{Command, League, Outcome, State, HQ};
use ice_and_fire_opening::{middle, solo};
use ice_and_fire_referee::map::generate;
use ice_and_fire_tactics::chain::{cost_to, UNREACHABLE};

/// One command of an answer, as the referee reads it (as the review tool).
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

/// The terms of one side: gold, income, active cells, training cost of its
/// units on active cells, active towers, active mines, the pressure (gold and
/// income less the cheapest chain to the enemy HQ, from -200 to 0), the debt
/// (units' cost when gold and income are below 0), joints and the most cells
/// one capture cuts off.
fn side(state: &State, seat: usize) -> [i64; 10] {
    let active = state.active(seat);
    let units: i64 = (state.units(seat) & active)
        .iter()
        .filter_map(|cell| state.unit_level(cell))
        .map(|level| 10 * i64::from(level))
        .sum();
    let cost = cost_to(state, seat, false, HQ[1 - seat]);
    let pressure = if cost == UNREACHABLE {
        -200
    } else {
        (state.gold(seat) + state.income(seat) - cost).clamp(-200, 0)
    };
    let debt = if state.gold(seat) + state.income(seat) < 0 {
        units
    } else {
        0
    };
    let cut = solo::severed(state, seat);
    let (mut joints, mut most) = (0, 0);
    for cell in active.iter() {
        if cut[cell] > 0 {
            joints += 1;
            most = most.max(cut[cell]);
        }
    }
    [
        state.gold(seat),
        state.income(seat),
        i64::from(active.count()),
        units,
        i64::from((state.towers() & active).count()),
        i64::from((state.mines() & active).count()),
        pressure,
        debt,
        joints,
        most,
    ]
}

const NAMES: [&str; 10] = [
    "gold", "income", "cells", "units", "towers", "mines", "pressure", "debt", "joints", "cut",
];

/// Whether the opponent, after the side to act ends its turn, could pay a
/// chain of trainings (or a unit's step) that takes its HQ.
fn threat(state: &State) -> bool {
    let seat = state.to_act();
    let other = 1 - seat;
    let mut next = state.clone();
    next.end_turn();
    next.outcome().is_none() && cost_to(&next, other, true, HQ[seat]) <= next.gold(other)
}

fn main() {
    let dir = std::env::args().nth(1).expect("usage: features RECORDS_DIR");
    let mut header = vec!["game", "game_turn", "blue", "won", "threat"].join(",");
    for who in ["own", "opp"] {
        for name in NAMES {
            header.push_str(&format!(",{who}_{name}"));
        }
    }
    println!("{header}");
    let mut paths: Vec<_> = fs::read_dir(Path::new(&dir))
        .expect("a directory of records")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    for (game, path) in paths.iter().enumerate() {
        let text = fs::read_to_string(path).expect("a readable record");
        let record: Record = serde_json::from_str(&text).expect("a record");
        let map = generate(record.seed, ice_and_fire_referee::League::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let mut rows = Vec::new();
        let mut middle_game = false;
        for answers in &record.turns {
            if state.outcome().is_some() {
                break;
            }
            let seat = state.to_act();
            middle_game = middle_game || middle::reached(&state);
            if middle_game {
                let (own, opp) = (side(&state, seat), side(&state, 1 - seat));
                rows.push((seat, state.game_turn(), threat(&state), own, opp));
            }
            let line = answers
                .iter()
                .find(|a| a.seat == seat)
                .and_then(|a| a.lines.first())
                .cloned()
                .unwrap_or_default();
            let commands: Vec<Command> = line.split(';').filter_map(|p| parse(p.trim())).collect();
            state.play_turn(&commands);
        }
        let Some(Outcome::Win(winner)) = state.outcome() else {
            continue;
        };
        for (seat, turn, threat, own, opp) in rows {
            let mut line = format!(
                "{game},{turn},{},{},{}",
                seat,
                u8::from(seat == winner),
                u8::from(threat)
            );
            for value in own.iter().chain(opp.iter()) {
                line.push_str(&format!(",{value}"));
            }
            println!("{line}");
        }
    }
}
