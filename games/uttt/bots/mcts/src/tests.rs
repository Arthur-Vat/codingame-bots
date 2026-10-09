use std::cell::RefCell;

use uttt_engine::{MoveList, Status};

use super::*;

/// Runs the bot on `input` with a fixed number of iterations per search.
fn run(seed: u64, input: &str, iterations: u64) -> (Result<(), InputError>, String, String) {
    let mut output = Vec::new();
    let mut log = Vec::new();
    let result = play(
        &mut Mcts::new(EXPLORATION, seed),
        &mut Rng::new(seed),
        Input::new(input.as_bytes()),
        &mut output,
        &mut log,
        |_, _| Budget::Iterations(iterations),
    );
    let text = |bytes: Vec<u8>| String::from_utf8(bytes).expect("ASCII");
    (result, text(output), text(log))
}

#[test]
fn plays_from_the_list_until_the_input_ends() {
    let (result, output, log) = run(1, "-1 -1\n3\n4 4\n0 0\n8 8\n", 100);
    assert_eq!(result, Err(InputError::Eof));
    assert!(["4 4", "0 0", "8 8"].contains(&output.trim()), "{output}");
    assert!(log.contains("iterations"), "{log}");
}

#[test]
fn takes_the_only_action_offered_without_searching() {
    let (_, output, log) = run(1, "-1 -1\n1\n0 5\n", 100);
    assert_eq!(output, "0 5\n");
    assert!(log.is_empty(), "{log}");
}

#[test]
fn gives_the_first_turn_its_longer_limit() {
    let limits = RefCell::new(Vec::new());
    // Two turns: anywhere, then the opponent answers in the board we sent
    // it to; each turn offers every legal action.
    let mut board = Board::new();
    let mut list = MoveList::new();
    let mut input = String::from("-1 -1\n");
    board.legal_moves(&mut list);
    input.push_str(&format!("{}\n", list.len()));
    for mv in list.iter() {
        input.push_str(&format!("{mv}\n"));
    }
    let mut output = Vec::new();
    let _ = play(
        &mut Mcts::new(EXPLORATION, 1),
        &mut Rng::new(1),
        Input::new(input.as_bytes()),
        &mut output,
        &mut Vec::new(),
        |_, limit| {
            limits.borrow_mut().push(limit);
            Budget::Iterations(50)
        },
    );
    let first: Vec<&str> = std::str::from_utf8(&output).unwrap().lines().collect();
    let (row, col) = first[0].split_once(' ').unwrap();
    let mine = Move::from_row_col(row.parse().unwrap(), col.parse().unwrap()).unwrap();
    board.play(mine);
    board.legal_moves(&mut list);
    let reply = list[0];
    board.play(reply);
    board.legal_moves(&mut list);
    input.push_str(&format!("{reply}\n{}\n", list.len()));
    for mv in list.iter() {
        input.push_str(&format!("{mv}\n"));
    }
    limits.borrow_mut().clear();
    let _ = play(
        &mut Mcts::new(EXPLORATION, 1),
        &mut Rng::new(1),
        Input::new(input.as_bytes()),
        &mut Vec::new(),
        &mut Vec::new(),
        |_, limit| {
            limits.borrow_mut().push(limit);
            Budget::Iterations(50)
        },
    );
    assert_eq!(*limits.borrow(), vec![FIRST_LIMIT, LIMIT]);
}

#[test]
fn falls_back_to_random_when_the_input_contradicts_the_rules() {
    // The opponent "plays" a cell we just took.
    let (result, output, log) = run(1, "-1 -1\n1\n4 4\n4 4\n2\n3 3\n3 4\n", 100);
    assert_eq!(result, Err(InputError::Eof));
    assert!(log.contains("lost track"), "{log}");
    assert_eq!(output.lines().count(), 2);
}

#[test]
fn rejects_a_turn_without_actions() {
    let (result, _, _) = run(1, "-1 -1\n0\n", 100);
    assert!(matches!(result, Err(InputError::Parse { .. })));
}

/// MCTS's points against random over `games` games, alternating seats, with
/// `iterations` per move.
fn score_against_random(games: u64, iterations: u64) -> f64 {
    let mut points = 0.0;
    let mut moves = MoveList::new();
    let mut mcts = Mcts::new(EXPLORATION, 9);
    for game in 0..games {
        let mut rng = Rng::new(game);
        let mcts_seat = (game % 2) as usize;
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            board.legal_moves(&mut moves);
            let mv = if board.to_move() == mcts_seat {
                let root = ContextBoard {
                    board,
                    policy: policy(),
                };
                mcts.search(&root, &moves, Budget::Iterations(iterations))
                    .best
            } else {
                *rng.pick(&moves).unwrap()
            };
            board.play(mv);
        }
        points += match board.status() {
            Status::Win(seat) if seat == mcts_seat => 1.0,
            Status::Win(_) => 0.0,
            _ => 0.5,
        };
    }
    points / games as f64
}

#[test]
fn clearly_beats_random_even_with_few_iterations() {
    let score = score_against_random(20, 200);
    assert!(score >= 0.9, "MCTS scored {score} against random");
}
