//! Replays game records (E083's study of lost fights) and writes, one JSON
//! line each, the board at the start of every ply up to a game turn, red's
//! view; with `--commands`, also the board after each command.
//! Usage: plies MAX_TURN [--commands] RECORD...

use cg_arena::record::Record;
use cg_arena::referee::{Answer, GameSetup, Referee};
use ice_and_fire_engine::{Command, League, State, SIZE};
use ice_and_fire_referee::IceAndFireReferee;

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
    let mines: Vec<_> = state
        .mines()
        .iter()
        .map(|cell| {
            serde_json::json!([
                cell % SIZE,
                cell / SIZE,
                usize::from(!state.owned(0).test(cell))
            ])
        })
        .collect();
    serde_json::json!({"rows": rows, "units": units, "towers": towers, "mines": mines,
        "gold": [state.gold(0), state.gold(1)], "income": [state.income(0), state.income(1)]})
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let max_turn: u32 = args[1].parse().expect("MAX_TURN");
    let commands = args.iter().any(|a| a == "--commands");
    for path in args.iter().skip(2).filter(|a| !a.starts_with("--")) {
        let text = std::fs::read_to_string(path).expect("a record");
        let record: Record = serde_json::from_str(&text).expect("a record");
        let id = std::path::Path::new(path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let mut referee = IceAndFireReferee::new(&GameSetup {
            seed: record.seed,
            opening_plies: 0,
        });
        for (ply, answers) in record.turns.iter().enumerate() {
            if referee.game_turn() > max_turn {
                break;
            }
            for answer in answers {
                let seat = answer.seat;
                let Ok(mut state) = State::read(
                    &referee.initial_input(seat),
                    &referee.turn_input(seat),
                    League::Bronze,
                    referee.game_turn(),
                ) else {
                    break;
                };
                let line = serde_json::json!({"game": id, "ply": ply, "seat": seat,
                    "turn": referee.game_turn(), "cmd": null, "ok": true, "board": board(&state)});
                println!("{line}");
                if commands {
                    for part in answer.lines.join(";").split(';') {
                        let Some(cmd) = command(part.trim()) else {
                            continue;
                        };
                        let ok = state.apply(cmd);
                        let line = serde_json::json!({"game": id, "ply": ply, "seat": seat,
                            "turn": referee.game_turn(), "cmd": cmd.to_text(), "ok": ok, "board": board(&state)});
                        println!("{line}");
                    }
                }
                let played = [Answer {
                    seat,
                    lines: answer.lines.clone(),
                }];
                if referee.play(&played).is_err() {
                    break;
                }
            }
        }
    }
}
