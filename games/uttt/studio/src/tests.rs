use cg_arena::live::LiveGame;
use serde_json::{json, Value};
use studio_game::live_game;

use super::*;

fn setup(opening_plies: u32) -> GameSetup {
    GameSetup {
        seed: 1,
        opening_plies,
    }
}

fn answer(seat: usize, line: &str) -> Vec<RecordedAnswer> {
    vec![RecordedAnswer {
        seat,
        lines: vec![line.to_string()],
        ms: 0.0,
    }]
}

/// Plays the first human move of the side to act until the game ends or
/// `limit` turns are played.
fn play_out(setup: &GameSetup, limit: usize) -> Vec<Vec<RecordedAnswer>> {
    let mut turns = Vec::new();
    while turns.len() < limit {
        let game = live_game(&Uttt, *setup, &turns).unwrap();
        let Some(&seat) = game.to_act().first() else {
            break;
        };
        let moves = Uttt.human_moves(setup, &turns, seat).unwrap();
        turns.push(answer(seat, &moves[0].lines[0]));
    }
    turns
}

fn marks(frame: &Value) -> usize {
    frame["cells"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|row| row.as_array().unwrap())
        .filter(|cell| !cell.is_null())
        .count()
}

#[test]
fn info_names_the_game() {
    let info = Uttt.info();
    assert_eq!(info.id, "uttt");
    assert_eq!(info.name, "Ultimate Tic-Tac-Toe");
}

#[test]
fn the_start_frame_is_an_empty_board() {
    let frames = Uttt.frames(&setup(0), &[]).unwrap();
    assert_eq!(frames.len(), 1);
    let start = &frames[0];
    assert_eq!(marks(start), 0);
    assert_eq!(start["cells"].as_array().unwrap().len(), 9);
    assert_eq!(
        start["small"],
        json!([[null, null, null], [null, null, null], [null, null, null]])
    );
    assert_eq!(start["last"], Value::Null);
    assert_eq!(start["playable"].as_array().unwrap().len(), 81);
    assert_eq!(start["to_move"], json!(0));
    assert_eq!(start["points"], json!([0, 0]));
    assert_eq!(start["result"], Value::Null);
}

#[test]
fn a_move_gives_a_frame_and_the_moves_of_the_next_seat() {
    let turns = vec![answer(0, "4 4")];
    let frames = Uttt.frames(&setup(0), &turns).unwrap();
    assert_eq!(frames.len(), 2);
    let frame = &frames[1];
    assert_eq!(frame["cells"][4][4], json!(0));
    assert_eq!(frame["last"], json!([4, 4]));
    assert_eq!(frame["to_move"], json!(1));
    let center: Vec<Value> = (3..6)
        .flat_map(|row| (3..6).map(move |col| (row, col)))
        .filter(|&cell| cell != (4, 4))
        .map(|(row, col)| json!([row, col]))
        .collect();
    assert_eq!(center.len(), 8);
    assert_eq!(frame["playable"], Value::Array(center.clone()));

    let moves = Uttt.human_moves(&setup(0), &turns, 1).unwrap();
    let expected: Vec<HumanMove> = center
        .iter()
        .map(|cell| {
            let (row, col) = (cell[0].as_u64().unwrap(), cell[1].as_u64().unwrap());
            HumanMove {
                action: json!({ "row": row, "col": col }),
                lines: vec![format!("{row} {col}")],
            }
        })
        .collect();
    assert_eq!(moves, expected);
    assert!(Uttt.human_moves(&setup(0), &turns, 0).unwrap().is_empty());
}

#[test]
fn an_invalid_answer_is_an_error() {
    let turns = vec![answer(0, "4 4"), answer(1, "4 4")];
    let invalid = Uttt.frames(&setup(0), &turns).unwrap_err();
    assert_eq!((invalid.turn, invalid.invalid.seat), (1, 1));
    assert!(Uttt.human_moves(&setup(0), &turns, 0).is_err());
}

#[test]
fn a_turn_from_the_wrong_seat_is_an_error() {
    let turns = vec![answer(1, "4 4")];
    assert!(Uttt.frames(&setup(0), &turns).is_err());
    assert!(Uttt.frames(&setup(0), &[Vec::new()]).is_err());
}

#[test]
fn a_full_game_ends_with_a_result_and_no_moves() {
    let setup = setup(0);
    let turns = play_out(&setup, 200);
    let frames = Uttt.frames(&setup, &turns).unwrap();
    assert_eq!(frames.len(), turns.len() + 1);
    let last = frames.last().unwrap();
    assert!(!last["result"].is_null());
    assert!(last["playable"].as_array().unwrap().is_empty());
    assert_eq!(last["to_move"], Value::Null);
    let won = last["small"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|row| row.as_array().unwrap())
        .any(|board| board == &json!(0) || board == &json!(1));
    assert!(won, "{last}");
    for seat in 0..2 {
        assert!(Uttt.human_moves(&setup, &turns, seat).unwrap().is_empty());
    }
}

#[test]
fn a_full_small_board_without_a_winner_is_a_draw() {
    let mut draws = 0;
    for seed in 0..40u64 {
        let setup = GameSetup {
            seed,
            opening_plies: 0,
        };
        // Moves picked by a fixed arithmetic rule, so that games differ.
        let mut turns: Vec<Vec<RecordedAnswer>> = Vec::new();
        loop {
            let frames = Uttt.frames(&setup, &turns).unwrap();
            let Some(seat) = frames.last().unwrap()["to_move"].as_u64() else {
                break;
            };
            let moves = Uttt.human_moves(&setup, &turns, seat as usize).unwrap();
            let pick = (turns.len() * 7 + seed as usize) % moves.len();
            turns.push(answer(seat as usize, &moves[pick].lines[0]));
        }
        for frame in Uttt.frames(&setup, &turns).unwrap() {
            for board_row in 0..3 {
                for board_col in 0..3 {
                    let small = &frame["small"][board_row][board_col];
                    let full = (0..3).all(|r| {
                        (0..3).all(|c| {
                            !frame["cells"][3 * board_row + r][3 * board_col + c].is_null()
                        })
                    });
                    if small == &json!("draw") {
                        assert!(full);
                        draws += 1;
                    } else if small.is_null() {
                        assert!(!full);
                    }
                }
            }
        }
    }
    assert!(draws > 0, "no drawn small board in 40 games");
}

#[test]
fn an_opening_is_part_of_the_start() {
    let setup = setup(4);
    let frames = Uttt.frames(&setup, &[]).unwrap();
    assert_eq!(frames.len(), 1);
    assert_eq!(marks(&frames[0]), 4);

    // The referee asks for the imposed plies as turns, which add no frame.
    let turns = play_out(&setup, 6);
    assert_eq!(turns.len(), 6);
    let frames = Uttt.frames(&setup, &turns[..4]).unwrap();
    assert_eq!(frames, vec![frames[0].clone()]);
    let frames = Uttt.frames(&setup, &turns).unwrap();
    assert_eq!(frames.len(), 3);
    assert_eq!(marks(&frames[1]), 5);
    assert_eq!(marks(&frames[2]), 6);
}

#[test]
fn live_game_replays_to_the_same_position() {
    let setup = setup(0);
    let turns = play_out(&setup, 11);
    let replayed = live_game(&Uttt, setup, &turns).unwrap();

    let mut stepped = LiveGame::new(Uttt.new_referee(&setup), setup);
    for turn in &turns {
        stepped.play(turn.clone()).unwrap();
    }
    assert_eq!(replayed.to_act(), stepped.to_act());
    assert_eq!(replayed.to_act(), vec![1]);
    assert_eq!(replayed.turns(), &turns[..]);
    assert_eq!(replayed.input_for(1), stepped.input_for(1));
}

#[test]
fn live_game_rejects_invalid_turns() {
    let turns = vec![answer(0, "4 4"), answer(1, "0 0")];
    assert!(live_game(&Uttt, setup(0), &turns).is_err());
}
