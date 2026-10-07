use cg_core::rng::Rng;
use uttt_engine::{MoveList, Status};

use super::*;

fn run(seed: u64, input: &str) -> (Result<(), InputError>, String, String) {
    let mut output = Vec::new();
    let mut log = Vec::new();
    let result = play(
        &mut Rng::new(seed),
        Input::new(input.as_bytes()),
        &mut output,
        &mut log,
    );
    let text = |bytes: Vec<u8>| String::from_utf8(bytes).expect("ASCII");
    (result, text(output), text(log))
}

#[test]
fn plays_from_the_list_until_the_input_ends() {
    // Turn 1: anywhere (three choices offered). Turn 2: the opponent
    // answered 3 3 after our move; the list is whatever the referee sends.
    let (result, output, log) = run(1, "-1 -1\n3\n4 4\n0 0\n8 8\n");
    assert_eq!(result, Err(InputError::Eof));
    assert!(["4 4", "0 0", "8 8"].contains(&output.trim()), "{output}");
    assert!(log.is_empty(), "{log}");
}

#[test]
fn takes_the_only_action_offered() {
    // An imposed opening move: a one-item list.
    let (_, output, log) = run(1, "-1 -1\n1\n0 5\n");
    assert_eq!(output, "0 5\n");
    assert!(log.is_empty(), "{log}");
}

#[test]
fn follows_a_game_and_wins_a_small_board_when_it_can() {
    // As seat 0: (1,0) (1,1) and (5,3) in (board, cell) are our moves,
    // the opponent's are (0,1), (1,5) and (3,1). Board 1 cell 2 then wins.
    let ours = [(1, 0), (1, 1), (5, 3)];
    let theirs = [(0, 1), (1, 5), (3, 1)];
    let row_col = |(b, c): (usize, usize)| Move::new(b, c).row_col();
    let mut input = String::new();
    for turn in 0..4 {
        let last = if turn == 0 {
            (-1, -1)
        } else {
            let (r, c) = row_col(theirs[turn - 1]);
            (r as i32, c as i32)
        };
        // Offer only our scripted move, then the real list on the last turn.
        let offered: Vec<(usize, usize)> = if turn < 3 {
            vec![row_col(ours[turn])]
        } else {
            (0..9)
                .filter(|&c| ![0, 1, 5].contains(&c))
                .map(|c| row_col((1, c)))
                .collect()
        };
        input.push_str(&format!("{} {}\n{}\n", last.0, last.1, offered.len()));
        for (r, c) in offered {
            input.push_str(&format!("{r} {c}\n"));
        }
    }
    let (result, output, log) = run(3, &input);
    assert_eq!(result, Err(InputError::Eof));
    assert!(log.is_empty(), "{log}");
    let (r, c) = row_col((1, 2));
    assert_eq!(
        output.lines().last(),
        Some(format!("{r} {c}").as_str()),
        "{output}"
    );
}

#[test]
fn falls_back_to_random_when_the_input_contradicts_the_rules() {
    // The opponent "plays" a cell we just took.
    let (result, output, log) = run(1, "-1 -1\n1\n4 4\n4 4\n2\n3 3\n3 4\n");
    assert_eq!(result, Err(InputError::Eof));
    assert!(log.contains("lost track"), "{log}");
    assert_eq!(output.lines().count(), 2);
}

#[test]
fn rejects_a_turn_without_actions() {
    let (result, _, _) = run(1, "-1 -1\n0\n");
    assert!(matches!(result, Err(InputError::Parse { .. })));
}

/// Greedy's points against random over `games` games, alternating seats.
fn score_against_random(games: u64) -> f64 {
    let mut points = 0.0;
    let mut moves = MoveList::new();
    for game in 0..games {
        let mut rng = Rng::new(game);
        let greedy_seat = (game % 2) as usize;
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            board.legal_moves(&mut moves);
            let mv = if board.to_move() == greedy_seat {
                moves[best(&board, &moves, &mut rng)]
            } else {
                *rng.pick(&moves).unwrap()
            };
            board.play(mv);
        }
        points += match board.status() {
            Status::Win(seat) if seat == greedy_seat => 1.0,
            Status::Win(_) => 0.0,
            _ => 0.5,
        };
    }
    points / games as f64
}

#[test]
fn clearly_beats_random() {
    let score = score_against_random(200);
    assert!(score > 0.85, "greedy scored {score} against random");
}
