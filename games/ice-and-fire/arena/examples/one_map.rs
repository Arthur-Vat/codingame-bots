//! Plays one game (E080's turn-1 planner prototype) on the map of a given seed
//! and writes its record as JSON, or prints the map's initial input. With
//! ACTIONS, also writes one JSON line per command, each played on its own,
//! with the board after it, red's view (seat 0).
//! Usage: one_map SEED [MAX_TURNS TIME_SCALE NAME=CMD NAME=CMD OUT [ACTIONS]]

use cg_arena::referee::{Answer, GameSetup, Referee};
use cg_arena::runner::{run_match, BotSpec, MatchOptions};
use ice_and_fire_engine::{Command, League, State, SIZE};
use ice_and_fire_referee::IceAndFireReferee;

fn spec(s: &str) -> BotSpec {
    let (name, cmd) = s.split_once('=').expect("NAME=CMD");
    let mut parts = cmd.split(' ').map(String::from);
    BotSpec {
        name: name.into(),
        program: parts.next().expect("a program"),
        args: parts.collect(),
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let seed: u64 = a[1].parse::<i64>().map(|s| s as u64).expect("seed");
    let mut referee = IceAndFireReferee::new(&GameSetup {
        seed,
        opening_plies: 0,
    });
    if a.len() == 2 {
        println!("{}", referee.turn_input(0));
        return;
    }
    let options = MatchOptions {
        time_scale: a[3].parse().expect("time scale"),
        max_turns: a[2].parse().expect("max turns"),
        show_bot_stderr: true,
        ..MatchOptions::default()
    };
    let (b0, b1) = (spec(&a[4]), spec(&a[5]));
    let record = run_match(&mut referee, [&b0, &b1], seed, &options).expect("the match");
    std::fs::write(&a[6], serde_json::to_string(&record).expect("json")).expect("write");
    eprintln!("end {:?} turns {}", record.end, record.turns);
    if let Some(path) = a.get(7) {
        std::fs::write(path, actions(seed, &record.recorded_turns)).expect("write");
    }
}

/// A command of an answer's text, or `None` for what is not a command.
fn command(part: &str) -> Option<Command> {
    let w: Vec<&str> = part.split_whitespace().collect();
    let n = |i: usize| w.get(i).and_then(|v| v.parse::<u32>().ok());
    let c = |i: usize| n(i).and_then(|v| u8::try_from(v).ok());
    match w.first().copied()? {
        "MOVE" => Some(Command::Move {
            id: n(1)?,
            x: c(2)?,
            y: c(3)?,
        }),
        "TRAIN" => Some(Command::Train {
            level: c(1)?,
            x: c(2)?,
            y: c(3)?,
        }),
        "BUILD" if w.get(1) == Some(&"TOWER") => Some(Command::BuildTower { x: c(2)?, y: c(3)? }),
        "BUILD" if w.get(1) == Some(&"MINE") => Some(Command::BuildMine { x: c(2)?, y: c(3)? }),
        _ => None,
    }
}

/// The board as 12 rows, red's view: O red, X blue (lower case inactive).
fn board(state: &State) -> serde_json::Value {
    let map = state.map();
    let rows: Vec<String> = (0..SIZE)
        .map(|y| {
            (0..SIZE)
                .map(|x| {
                    let cell = y * SIZE + x;
                    if !map.valid.test(cell) {
                        '#'
                    } else if state.owned(0).test(cell) {
                        if state.active(0).test(cell) {
                            'O'
                        } else {
                            'o'
                        }
                    } else if state.owned(1).test(cell) {
                        if state.active(1).test(cell) {
                            'X'
                        } else {
                            'x'
                        }
                    } else {
                        '.'
                    }
                })
                .collect()
        })
        .collect();
    let mut units = Vec::new();
    for seat in 0..2 {
        for cell in state.units(seat).iter() {
            let (id, level) = state.unit_at(cell).unwrap_or((0, 1));
            units.push(serde_json::json!([
                cell % SIZE,
                cell / SIZE,
                seat,
                id,
                level
            ]));
        }
    }
    let towers: Vec<_> = state
        .towers()
        .iter()
        .map(|cell| {
            serde_json::json!([
                cell % SIZE,
                cell / SIZE,
                usize::from(!state.owned(0).test(cell))
            ])
        })
        .collect();
    serde_json::json!({"rows": rows, "units": units, "towers": towers, "gold": [state.gold(0), state.gold(1)]})
}

/// Every command of the game played on its own: one JSON line each.
fn actions(seed: u64, turns: &[Vec<cg_arena::record::RecordedAnswer>]) -> String {
    let mut referee = IceAndFireReferee::new(&GameSetup {
        seed,
        opening_plies: 0,
    });
    let mut out = String::new();
    for (ply, answers) in turns.iter().enumerate() {
        for answer in answers {
            let seat = answer.seat;
            let Ok(mut state) = State::read(
                &referee.initial_input(seat),
                &referee.turn_input(seat),
                League::Bronze,
                referee.game_turn(),
            ) else {
                return out;
            };
            let start = serde_json::json!({"ply": ply, "seat": seat, "turn": referee.game_turn(), "cmd": null, "ok": true, "board": board(&state)});
            out.push_str(&start.to_string());
            out.push('\n');
            for part in answer.lines.join(";").split(';') {
                let Some(cmd) = command(part.trim()) else {
                    continue;
                };
                let ok = state.apply(cmd);
                let line = serde_json::json!({"ply": ply, "seat": seat, "turn": referee.game_turn(), "cmd": cmd.to_text(), "ok": ok, "board": board(&state)});
                out.push_str(&line.to_string());
                out.push('\n');
            }
            let played = [Answer {
                seat,
                lines: answer.lines.clone(),
            }];
            if referee.play(&played).is_err() {
                return out;
            }
        }
    }
    out
}
