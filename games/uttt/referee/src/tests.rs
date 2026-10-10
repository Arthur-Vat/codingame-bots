use std::collections::HashSet;

use super::*;

/// A position built from marks, for tests. Small-board winners and points
/// are derived from the marks.
fn position(marks: &[(Cell, usize)], to_move: usize, last: Option<Cell>) -> UtttReferee {
    let mut game = UtttReferee::new(1);
    for &((row, col), seat) in marks {
        game.marks[row][col] = Some(seat);
    }
    for board_row in 0..3 {
        for board_col in 0..3 {
            if let Some(seat) = game.board_winner(board_row, board_col) {
                game.small_winners[board_row][board_col] = Some(seat);
                game.points[seat] += 1;
            }
        }
    }
    game.main_winner = line_owner(&game.small_winners);
    game.to_move = to_move;
    game.last = last;
    game.update_valid_actions();
    game
}

/// Marks that win the small board `(board_row, board_col)` for `seat`
/// with its top row.
fn won_board(board_row: usize, board_col: usize, seat: usize) -> Vec<(Cell, usize)> {
    (0..3)
        .map(|c| ((3 * board_row, 3 * board_col + c), seat))
        .collect()
}

/// Marks that fill the small board with no line:
/// X O X / X O O / O X X.
fn drawn_board(board_row: usize, board_col: usize) -> Vec<(Cell, usize)> {
    let pattern = [[0, 1, 0], [0, 1, 1], [1, 0, 0]];
    let mut marks = Vec::new();
    for (r, row) in pattern.iter().enumerate() {
        for (c, &seat) in row.iter().enumerate() {
            marks.push(((3 * board_row + r, 3 * board_col + c), seat));
        }
    }
    marks
}

fn sorted(cells: &[Cell]) -> Vec<Cell> {
    let mut cells = cells.to_vec();
    cells.sort_unstable();
    cells
}

fn board_cells(board_row: usize, board_col: usize) -> Vec<Cell> {
    let mut cells = Vec::new();
    for row in 3 * board_row..3 * board_row + 3 {
        for col in 3 * board_col..3 * board_col + 3 {
            cells.push((row, col));
        }
    }
    cells
}

fn answer(seat: usize, line: &str) -> Vec<Answer> {
    vec![Answer {
        seat,
        lines: vec![line.to_string()],
    }]
}

#[test]
fn the_first_player_may_play_anywhere() {
    let game = UtttReferee::new(5);
    assert_eq!(game.players_to_act(), vec![0]);
    assert_eq!(game.valid_actions().len(), 81);
    let distinct: HashSet<Cell> = game.valid_actions().iter().copied().collect();
    assert_eq!(distinct.len(), 81);
    assert!(game.turn_input(0).starts_with("-1 -1\n81\n"));
    assert_eq!(game.turn_input(0).lines().count(), 83);
}

#[test]
fn a_move_sends_the_opponent_to_the_matching_small_board() {
    let mut game = UtttReferee::new(5);
    // Centre of the centre board: the opponent stays in the centre board.
    game.play(&answer(0, "4 4")).unwrap();
    let mut expected = board_cells(1, 1);
    expected.retain(|&cell| cell != (4, 4));
    assert_eq!(sorted(game.valid_actions()), expected);
    assert_eq!(game.players_to_act(), vec![1]);
    assert!(game.turn_input(1).starts_with("4 4\n8\n"));

    // Top-right cell of the centre board: sent to the top-right board.
    game.play(&answer(1, "3 5")).unwrap();
    assert_eq!(sorted(game.valid_actions()), board_cells(0, 2));
}

#[test]
fn being_sent_to_a_won_board_frees_the_choice_except_won_boards() {
    let mut marks = won_board(0, 0, 0);
    marks.push(((3, 0), 1));
    // The last move (3, 0) sends seat 0 to board (0, 0), which seat 0 won.
    let game = position(&marks, 0, Some((3, 0)));
    let valid: HashSet<Cell> = game.valid_actions().iter().copied().collect();
    // All empty cells outside the won board, including none of its own.
    assert_eq!(valid.len(), 81 - 9 - 1);
    assert!(board_cells(0, 0).iter().all(|cell| !valid.contains(cell)));
    assert!(!valid.contains(&(3, 0)));
}

#[test]
fn being_sent_to_a_full_board_frees_the_choice() {
    let marks = drawn_board(2, 2);
    // The last move (8, 8) is in board (2, 2) and sends to board (2, 2).
    let game = position(&marks, 0, Some((8, 8)));
    assert_eq!(game.small_winner(2, 2), None);
    assert_eq!(game.valid_actions().len(), 72);
}

#[test]
fn a_small_board_is_closed_when_won_or_full() {
    let mut marks = won_board(0, 0, 0);
    marks.extend(drawn_board(1, 1));
    marks.push(((6, 6), 1));
    let game = position(&marks, 0, None);
    assert!(game.small_closed(0, 0), "won");
    assert!(game.small_closed(1, 1), "full without a winner");
    assert_eq!(game.small_winner(1, 1), None);
    assert!(!game.small_closed(2, 2), "one mark");
    assert!(!game.small_closed(0, 2), "empty");
}

#[test]
fn winning_a_small_board_scores_a_point() {
    let mut game = position(&[((0, 0), 0), ((0, 1), 0)], 0, None);
    game.play_cell((0, 2)).unwrap();
    assert_eq!(game.small_winner(0, 0), Some(0));
    assert_eq!(game.points(), [1, 0]);
    assert_eq!(game.result(), None);
}

#[test]
fn aligning_three_small_boards_wins_at_once() {
    let mut marks = won_board(0, 0, 1);
    marks.extend(won_board(1, 1, 1));
    marks.extend([((6, 6), 1), ((6, 7), 1)]);
    let mut game = position(&marks, 1, None);
    assert_eq!(game.result(), None);
    game.play(&answer(1, "6 8")).unwrap();
    assert_eq!(game.result(), Some(Outcome::Win(1)));
    assert_eq!(game.outcome(), Some(Outcome::Win(1)));
    assert!(game.players_to_act().is_empty());
    assert!(game.valid_actions().is_empty());
}

/// Eight small boards won, with no line on the main board; board (2, 2) is
/// left empty for each test to settle:
///
/// ```text
/// 0 1 0
/// 0 1 1
/// 1 0 .
/// ```
fn eight_boards_won() -> Vec<(Cell, usize)> {
    let owners = [
        (0, 0, 0),
        (0, 1, 1),
        (0, 2, 0),
        (1, 0, 0),
        (1, 1, 1),
        (1, 2, 1),
        (2, 0, 1),
        (2, 1, 0),
    ];
    owners
        .iter()
        .flat_map(|&(board_row, board_col, seat)| won_board(board_row, board_col, seat))
        .collect()
}

#[test]
fn with_no_valid_action_left_more_small_boards_wins() {
    // Board (2, 2) won by seat 0: 5 boards to 4, and still no main line.
    let mut marks = eight_boards_won();
    marks.extend(won_board(2, 2, 0));
    marks.push(((7, 7), 1));
    let game = position(&marks, 1, Some((7, 7)));
    assert_eq!(game.points(), [5, 4]);
    assert!(game.valid_actions().is_empty());
    assert_eq!(game.result(), Some(Outcome::Win(0)));
}

#[test]
fn with_no_valid_action_left_equal_points_is_a_draw() {
    // Board (2, 2) full with no line: 4 boards each.
    let mut marks = eight_boards_won();
    marks.extend(drawn_board(2, 2));
    let game = position(&marks, 0, Some((8, 8)));
    assert_eq!(game.points(), [4, 4]);
    assert!(game.valid_actions().is_empty());
    assert_eq!(game.result(), Some(Outcome::Draw));
}

#[test]
fn rejects_invalid_answers() {
    let mut game = UtttReferee::new(5);
    game.play(&answer(0, "4 4")).unwrap();
    for bad in ["0 0", "9 9", "-1 -1", "4", "a b", "", " 3 3"] {
        let error = game.clone().play(&answer(1, bad)).unwrap_err();
        assert_eq!(error.seat, 1, "{bad:?}");
    }
    // Text after the two integers is ignored, as on CodinGame.
    game.play(&answer(1, "3 3 hello")).unwrap();
    assert_eq!(game.mark((3, 3)), Some(1));
}

#[test]
fn the_seed_only_changes_the_order_of_valid_actions() {
    let a = UtttReferee::new(1);
    let b = UtttReferee::new(2);
    assert_ne!(a.valid_actions(), b.valid_actions());
    assert_eq!(sorted(a.valid_actions()), sorted(b.valid_actions()));
}

#[test]
fn random_games_always_end_consistently() {
    let mut rng = Rng::new(99);
    let mut results = [0u32; 3];
    for seed in 0..2000 {
        let mut game = UtttReferee::new(seed);
        let mut moves = 0;
        while game.result().is_none() {
            assert!(!game.valid_actions().is_empty());
            let cell = *rng.pick(game.valid_actions()).unwrap();
            game.play_cell(cell).unwrap();
            moves += 1;
            assert!(moves <= 81);
        }
        // Points always match the small boards won.
        let mut counted = [0u32; 2];
        for board_row in 0..3 {
            for board_col in 0..3 {
                if let Some(seat) = game.small_winner(board_row, board_col) {
                    counted[seat] += 1;
                }
            }
        }
        assert_eq!(counted, game.points());
        match game.result().unwrap() {
            Outcome::Win(seat) => results[seat] += 1,
            Outcome::Draw => results[2] += 1,
        }
    }
    // Random play: both seats win often, draws happen but are rarer.
    assert!(results[0] > 600 && results[1] > 600, "{results:?}");
    assert!(results[2] > 0, "{results:?}");
}

#[test]
fn an_opening_imposes_its_actions_then_frees_the_game() {
    let mut game = UtttReferee::with_opening(42, 4);
    let opening = game.opening().to_vec();
    assert_eq!(opening.len(), 4);
    for &cell in &opening {
        assert_eq!(game.valid_actions(), [cell]);
        let line = format!("{} {}", cell.0, cell.1);
        let seat = game.to_move();
        game.play(&answer(seat, &line)).unwrap();
    }
    // Afterwards the list is the full one again.
    let mut free = UtttReferee::new(1);
    for &cell in &opening {
        free.play_cell(cell).unwrap();
    }
    assert_eq!(sorted(game.valid_actions()), sorted(free.valid_actions()));
    assert!(game.valid_actions().len() > 1);
}

#[test]
fn an_opening_depends_only_on_the_seed() {
    assert_eq!(
        UtttReferee::with_opening(7, 6).opening(),
        UtttReferee::with_opening(7, 6).opening()
    );
    assert_ne!(
        UtttReferee::with_opening(7, 6).opening(),
        UtttReferee::with_opening(8, 6).opening()
    );
    assert!(UtttReferee::with_opening(7, 0).opening().is_empty());
    // A longer opening extends a shorter one.
    assert_eq!(
        UtttReferee::with_opening(7, 10).opening()[..6],
        *UtttReferee::with_opening(7, 6).opening()
    );
}

#[test]
fn an_opening_never_ends_the_game() {
    for seed in 0..50 {
        let mut game = UtttReferee::with_opening(seed, 81);
        let opening = game.opening().to_vec();
        assert!(!opening.is_empty());
        for cell in opening {
            game.play_cell(cell).unwrap();
            assert!(game.result().is_none(), "seed {seed}");
        }
    }
}

#[test]
fn an_action_outside_the_opening_is_invalid() {
    let mut game = UtttReferee::with_opening(3, 2);
    let imposed = game.opening()[0];
    let other = if imposed == (4, 4) { (0, 0) } else { (4, 4) };
    assert!(game.play_cell(other).is_err());
    assert!(game.play_cell(imposed).is_ok());
}
