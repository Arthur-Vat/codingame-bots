//! Writes a bank of the positions where a player built a tower (E090's
//! study of tower placement), for the plan bot's analysis mode: each
//! position as the player read it, with its answer as the `played` line.
//! Usage: tower_bank MAX_TURN RECORD...

use cg_arena::record::Record;
use cg_arena::referee::{Answer, GameSetup, Referee};
use ice_and_fire_referee::IceAndFireReferee;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let max_turn: u32 = args[1].parse().expect("MAX_TURN");
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
        'game: for (ply, answers) in record.turns.iter().enumerate() {
            for answer in answers {
                let turn = referee.game_turn();
                if turn > max_turn {
                    break 'game;
                }
                let line = answer.lines.join(";");
                if line.contains("BUILD TOWER") {
                    print!(
                        "position {id}-{} {ply} {turn}\n{}turn\n{}played {line}\nend\n",
                        answer.seat,
                        referee.initial_input(answer.seat),
                        referee.turn_input(answer.seat)
                    );
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
