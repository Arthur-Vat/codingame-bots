//! Replays game records and writes, one JSON line per record and game turn
//! asked, the mine spots each side holds at the start of red's ply (E089's
//! measure): active spots without a mine, built mines on active cells, and
//! all spots owned.
//! Usage: spots TURN,TURN,... RECORD...

use cg_arena::record::Record;
use cg_arena::referee::{Answer, GameSetup, Referee};
use ice_and_fire_engine::{League, State};
use ice_and_fire_referee::IceAndFireReferee;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let turns: Vec<u32> = args[1]
        .split(',')
        .map(|t| t.parse().expect("a turn"))
        .collect();
    let last = *turns.iter().max().expect("a turn");
    for path in args.iter().skip(2) {
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
        'game: for answers in &record.turns {
            for answer in answers {
                let turn = referee.game_turn();
                if turn > last {
                    break 'game;
                }
                if answer.seat == 0 && turns.contains(&turn) {
                    let Ok(state) = State::read(
                        &referee.initial_input(0),
                        &referee.turn_input(0),
                        League::Bronze,
                        turn,
                    ) else {
                        break 'game;
                    };
                    let spots = state.map().spots;
                    let side = |seat: usize| {
                        let active = state.active(seat);
                        let owned = state.owned(seat);
                        serde_json::json!({
                            "free": (spots & active).and_not(state.mines()).count(),
                            "mines": (state.mines() & active).count(),
                            "owned": (spots & owned).count(),
                            "income": state.income(seat),
                            "land": active.count(),
                            "nearfree": (spots & state.near_active(seat)).and_not(owned | state.owned(1 - seat)).count(),
                        })
                    };
                    let line = serde_json::json!({"game": id, "turn": turn,
                        "spots": spots.count(), "sides": [side(0), side(1)]});
                    println!("{line}");
                }
                let played = [Answer {
                    seat: answer.seat,
                    lines: answer.lines.clone(),
                }];
                if referee.play(&played).is_err() {
                    break 'game;
                }
            }
        }
    }
}
