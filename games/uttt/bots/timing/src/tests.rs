use std::time::Duration;

use uttt_referee::UtttReferee;

use super::*;

/// A game between two players of [`peaceful`] actions on the referee, each
/// picking with `rng`; returns the actions played, as (row, col).
fn peaceful_game(seed: u64) -> Vec<(usize, usize)> {
    let mut referee = UtttReferee::new(seed);
    let mut board = Board::new();
    let mut rng = Rng::new(seed);
    let mut played = Vec::new();
    while referee.result().is_none() {
        let actions: Vec<(i32, i32)> = referee
            .valid_actions()
            .iter()
            .map(|&(row, col)| (row as i32, col as i32))
            .collect();
        let action = *rng.pick(&peaceful(&board, &actions)).expect("an action");
        let cell = (action.0 as usize, action.1 as usize);
        referee.play_cell(cell).expect("a valid action");
        board.play(to_move(action).expect("a move"));
        played.push(cell);
    }
    played
}

#[test]
fn the_ramp_holds_answers_a_millisecond_longer_each_turn() {
    assert_eq!(target(1), None);
    assert_eq!(
        target(RAMP_FROM),
        Some(Duration::from_millis(RAMP_START_MS))
    );
    assert_eq!(
        target(RAMP_FROM + 8),
        Some(Duration::from_millis(RAMP_START_MS + 8))
    );
    // CodinGame's 100 ms is reached by turn 7, and a side's last answer, its
    // 41st at most, is held to 134 ms.
    assert_eq!(target(7), Some(Duration::from_millis(100)));
    assert_eq!(target(41), Some(Duration::from_millis(134)));
    assert_eq!(target(1_000), Some(Duration::from_millis(RAMP_MAX_MS)));
}

#[test]
fn the_workload_position_is_a_peaceful_game() {
    let game = peaceful_game(18);
    assert!(game.len() > WORK_PLIES.len());
    assert_eq!(game[..WORK_PLIES.len()], WORK_PLIES);
}

#[test]
fn each_unit_is_a_whole_search_of_the_same_size() {
    let work = work().expect("the workload's position plays");
    assert_eq!(work.root.board().status(), uttt_engine::Status::Ongoing);
    assert!(work.moves.len() > 1);
    // No move wins at once and the search proves nothing, so every unit
    // runs all its iterations, and the same ones.
    let mut mcts = Mcts::new(EXPLORATION, UNIT_SEED);
    mcts.expand_all = true;
    mcts.minimax_weight = MINIMAX_WEIGHT;
    mcts.value_temperature = VALUE_TEMPERATURE;
    mcts.prior_weight = PRIOR_WEIGHT;
    let result = mcts.search(&work.root, &work.moves, Budget::Iterations(UNIT_ITERATIONS));
    assert_eq!(result.iterations, UNIT_ITERATIONS);
    assert_eq!(result.proven, None);
    assert_eq!(unit(&work), unit(&work));
    assert_eq!(unit(&work), UNIT_ITERATIONS + result.nodes as u64);
}

#[test]
fn the_workload_works_until_the_deadline() {
    let work = work().expect("the workload's position plays");
    let until = Instant::now() + Duration::from_millis(50);
    // How long the units took depends on the machine's load, so only that
    // some were done and that the deadline was reached are checked.
    let (done, _) = work_until(&work, until);
    assert!(done > 0);
    assert!(Instant::now() >= until);
    // A deadline already passed does no unit.
    let (done, _) = work_until(&work, Instant::now());
    assert_eq!(done, 0);
}

#[test]
fn the_calibration_measures_each_stretch() {
    let speeds = calibrate(20, 3).unwrap();
    assert_eq!(speeds.len(), 3);
    assert!(speeds.iter().all(|&speed| speed > 0.0), "{speeds:?}");
    assert_eq!(measure("0"), 2);
    assert_eq!(measure("soon"), 2);
}

#[test]
fn peaceful_actions_win_no_small_board_while_others_remain() {
    // On every position of random games: the actions kept win no small
    // board and those left out all win one; when every action wins one,
    // all are kept.
    let (mut dropped, mut all_winning) = (0, 0);
    for seed in 0..50u64 {
        let mut rng = Rng::new(seed);
        let mut board = Board::new();
        let mut list = MoveList::new();
        while board.status() == uttt_engine::Status::Ongoing {
            board.legal_moves(&mut list);
            let actions: Vec<(i32, i32)> = list
                .iter()
                .map(|mv| {
                    let (row, col) = mv.row_col();
                    (row as i32, col as i32)
                })
                .collect();
            let me = board.to_move();
            let wins = |action: (i32, i32)| {
                let mut next = board;
                next.play(to_move(action).unwrap());
                next.won_boards(me) != board.won_boards(me)
            };
            let kept = peaceful(&board, &actions);
            if actions.iter().all(|&action| wins(action)) {
                assert_eq!(kept, actions);
                all_winning += 1;
            } else {
                assert!(kept.iter().all(|&action| !wins(action)));
                let left: Vec<_> = actions.iter().filter(|a| !kept.contains(a)).collect();
                assert!(left.iter().all(|&&action| wins(action)));
                dropped += left.len();
            }
            board.play(*rng.pick(&list).unwrap());
        }
    }
    assert!(dropped > 0 && all_winning > 0, "{dropped} {all_winning}");
    // An action the board does not allow: the list is kept as it is.
    let board = Board::new();
    assert_eq!(peaceful(&board, &[(4, 4), (9, 9)]), vec![(4, 4), (9, 9)]);
}

#[test]
fn games_against_itself_last_long_enough_to_reach_the_limit() {
    // The ramp passes CodinGame's 100 ms on a side's 7th answer and reaches
    // 120 ms on its 27th: games of 60 plies or more give each side 30
    // answers.
    let lengths: Vec<usize> = (0..200).map(|seed| peaceful_game(seed).len()).collect();
    let long = lengths.iter().filter(|&&len| len >= 60).count();
    assert!(long >= 150, "only {long} of 200 games reach 60 plies");
}

#[test]
fn each_line_holds_every_turn_so_far() {
    let mut game = Game {
        seat: "first",
        turns: Vec::new(),
    };
    game.turns.push(Turn {
        turn: 1,
        target: None,
        read: 0.04,
        gap: None,
        policy: 0.003,
        answer: Some(500.37),
        speed: Some(331.46),
    });
    game.turns.push(Turn {
        turn: 2,
        target: Some(95),
        read: 0.05,
        gap: Some(101.26),
        policy: 0.01,
        answer: None,
        speed: None,
    });
    assert_eq!(
        game.line(),
        "timing v1 first | turn:target/read/gap/policy/answer/speed | \
         1:-/0.04/-/0.00/500.37/331.5 2:95/0.05/101.3/0.01/-/-"
    );
}

#[test]
fn it_answers_each_turn_with_a_valid_action() {
    // Without a workload the bot holds nothing: two turns as the second
    // player, then the input ends.
    let input = "4 4\n8\n3 3\n3 4\n3 5\n4 3\n4 5\n5 3\n5 4\n5 5\n\
                 0 0\n2\n4 3\n3 4\n";
    let mut out = Vec::new();
    let mut log = Vec::new();
    let result = play(
        &mut Rng::new(1),
        None,
        Input::new(input.as_bytes()),
        &mut out,
        &mut log,
    );
    assert!(matches!(result, Err(InputError::Eof)), "{result:?}");
    let out = String::from_utf8(out).unwrap();
    let answers: Vec<&str> = out.lines().collect();
    assert_eq!(answers.len(), 2);
    assert!(["3 3", "3 4", "3 5", "4 3", "4 5", "5 3", "5 4", "5 5"].contains(&answers[0]));
    assert!(["4 3", "3 4"].contains(&answers[1]));
    let log = String::from_utf8(log).unwrap();
    let lines: Vec<&str> = log.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[1].starts_with("timing v1 second | "), "{}", lines[1]);
    assert!(lines[1].contains(" 2:95/"), "{}", lines[1]);
}
