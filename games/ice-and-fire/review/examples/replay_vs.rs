//! E095 comparison tool (2026-11-27): plays our bot (a binary) on the map of a
//! converted CodinGame record, in our seat, against the opponent's recorded
//! answers, up to a game turn, and prints each ply's board and commands as
//! one JSON line. The opponent's answers are replayed as recorded, so the
//! game is faithful until the territories meet.
//!
//! replay_vs RECORD.json SEAT BOT TURNS
use std::io::{BufRead, BufReader, Write};
use std::process::{Command as Process, Stdio};

use cg_arena::record::Record;
use ice_and_fire_engine::{Command, League, State, HQ, SIZE};
use ice_and_fire_referee::map::generate;

fn parse(part: &str) -> Option<Command> {
    let w: Vec<&str> = part.split_whitespace().collect();
    let n = |i: usize| w.get(i).and_then(|s| s.parse::<u32>().ok());
    let b = |i: usize| n(i).and_then(|v| u8::try_from(v).ok());
    match *w.first()? {
        "MOVE" => Some(Command::Move {
            id: n(1)?,
            x: b(2)?,
            y: b(3)?,
        }),
        "TRAIN" => Some(Command::Train {
            level: b(1)?,
            x: b(2)?,
            y: b(3)?,
        }),
        "BUILD" => match *w.get(1)? {
            "MINE" => Some(Command::BuildMine { x: b(2)?, y: b(3)? }),
            "TOWER" => Some(Command::BuildTower { x: b(2)?, y: b(3)? }),
            _ => None,
        },
        "WAIT" => Some(Command::Wait),
        _ => None,
    }
}

fn board(s: &State) -> Vec<String> {
    let m = s.map();
    (0..SIZE)
        .map(|y| {
            let mut r = String::new();
            for x in 0..SIZE {
                let c = y * SIZE + x;
                if m.void.test(c) {
                    r.push_str("## ");
                    continue;
                }
                let o = if s.owned(0).test(c) {
                    if s.active(0).test(c) {
                        'R'
                    } else {
                        'r'
                    }
                } else if s.owned(1).test(c) {
                    if s.active(1).test(c) {
                        'B'
                    } else {
                        'b'
                    }
                } else {
                    ' '
                };
                let k = if let Some(l) = s.unit_level(c) {
                    char::from(b'0' + l)
                } else if c == HQ[0] || c == HQ[1] {
                    'H'
                } else if s.mines().test(c) {
                    'M'
                } else if s.towers().test(c) {
                    'T'
                } else if m.spots.test(c) {
                    '*'
                } else {
                    '.'
                };
                r.push(o);
                r.push(k);
                r.push(' ');
            }
            r.trim_end().to_string()
        })
        .collect()
}

/// Whether the two sides' cells touch.
fn contact(s: &State) -> bool {
    !(s.owned(0).neighbours() & s.owned(1)).is_empty()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let record: Record = serde_json::from_str(&std::fs::read_to_string(&args[1]).unwrap()).unwrap();
    let us: usize = args[2].parse().unwrap();
    let bot = &args[3];
    let until: u32 = args[4].parse().unwrap();
    let map = generate(record.seed, ice_and_fire_referee::League::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let mut child = Process::new(bot)
        .env(
            "CG_FIXED_ITERS",
            std::env::var("ITERS").unwrap_or_else(|_| "800".into()),
        )
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    stdin.write_all(state.initial_input(us).as_bytes()).unwrap();
    // the opponent's answers in order
    let theirs: Vec<String> = record
        .turns
        .iter()
        .filter_map(|answers| answers.iter().find(|a| a.seat == 1 - us))
        .map(|a| a.lines.first().cloned().unwrap_or_default())
        .collect();
    let mut k = 0;
    let mut plies = Vec::new();
    let mut contact_turn = None;
    while state.outcome().is_none() && state.game_turn() <= until {
        if contact_turn.is_none() && contact(&state) {
            contact_turn = Some(state.game_turn());
        }
        let seat = state.to_act();
        let start = board(&state);
        let line = if seat == us {
            stdin.write_all(state.turn_input(us).as_bytes()).unwrap();
            stdin.flush().unwrap();
            let mut l = String::new();
            stdout.read_line(&mut l).unwrap();
            l.trim().to_string()
        } else {
            k += 1;
            theirs.get(k - 1).cloned().unwrap_or_default()
        };
        let mut done = Vec::new();
        let first_new = state.next_unit_id();
        for part in line
            .split(';')
            .map(str::trim)
            .filter(|p| !p.is_empty() && !p.starts_with("MSG"))
        {
            let ok = match parse(part) {
                Some(Command::Move { id, .. })
                    if id >= first_new
                        || !state
                            .units(seat)
                            .iter()
                            .any(|c| state.unit_id(c) == Some(id)) =>
                {
                    false
                }
                Some(cmd) => state.apply(cmd),
                None => false,
            };
            done.push(serde_json::json!({"c": part, "ok": ok || part.starts_with("WAIT")}));
            if state.outcome().is_some() {
                break;
            }
        }
        plies.push(serde_json::json!({"turn": state.game_turn(), "side": if seat == 0 {"red"} else {"blue"}, "board": start, "cmds": done}));
        state.end_turn();
    }
    let _ = child.kill();
    println!(
        "{}",
        serde_json::json!({"contact": contact_turn, "plies": plies, "end": board(&state)})
    );
}
