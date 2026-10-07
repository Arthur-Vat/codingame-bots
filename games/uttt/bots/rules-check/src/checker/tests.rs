use cg_core::rng::Rng;

use super::*;

fn coordinates(mv: Move) -> (i32, i32) {
    let (row, col) = mv.row_col();
    (row as i32, col as i32)
}

fn legal(board: &Board) -> Vec<(i32, i32)> {
    let mut moves = MoveList::new();
    board.legal_moves(&mut moves);
    moves.iter().map(|mv| coordinates(*mv)).collect()
}

/// Plays a random game where CodinGame is played by a second engine, with
/// the checker in `seat`. Returns every line the checker printed.
fn simulate(seat: usize, seed: u64) -> Vec<String> {
    let mut rng = Rng::new(seed);
    let mut game = Board::new();
    let mut checker = Checker::new();
    let mut lines = Vec::new();
    let mut last_opponent = (-1, -1);
    while game.status() == Status::Ongoing {
        let mut actions = legal(&game);
        if game.to_move() == seat {
            rng.shuffle(&mut actions);
            lines.extend(checker.check_turn(last_opponent, &actions));
            let action = *rng.pick(&actions).unwrap();
            lines.extend(checker.record_own_move(action));
            game.play(cell(action).unwrap());
        } else {
            let action = *rng.pick(&actions).unwrap();
            game.play(cell(action).unwrap());
            last_opponent = action;
        }
    }
    lines
}

#[test]
fn reports_no_difference_when_codingame_follows_the_rules() {
    let mut endings_seen = 0;
    for seed in 0..200 {
        for seat in 0..2 {
            let lines = simulate(seat, seed);
            assert!(
                lines.iter().all(|line| !line.contains("DIFFERENCE")),
                "{lines:?}"
            );
            let last_check = lines
                .iter()
                .rev()
                .find(|line| line.contains("turns checked"))
                .unwrap();
            assert!(last_check.ends_with("0 with differences"), "{last_check}");
            if lines
                .iter()
                .any(|line| line.contains("my move ends the game"))
            {
                endings_seen += 1;
            }
        }
    }
    // The checker's own move ends about half the games.
    assert!(endings_seen > 100, "{endings_seen}");
}

#[test]
fn reports_missing_and_extra_actions() {
    let mut checker = Checker::new();
    // First turn of the game: every cell is valid. Drop one, add a bogus one.
    let mut actions = legal(&Board::new());
    actions.retain(|&action| action != (4, 4));
    actions.push((9, 9));
    let lines = checker.check_turn((-1, -1), &actions);
    let difference = lines
        .iter()
        .find(|line| line.contains("DIFFERENCE"))
        .unwrap();
    assert!(
        difference.contains("Only CodinGame lists: 9 9"),
        "{difference}"
    );
    assert!(
        difference.contains("Only the engine lists: 4 4"),
        "{difference}"
    );
    assert!(lines.last().unwrap().ends_with("1 with differences"));
    assert_eq!(checker.differences(), 1);
}

#[test]
fn stops_following_after_an_impossible_opponent_move() {
    let mut checker = Checker::new();
    checker.check_turn((-1, -1), &legal(&Board::new()));
    checker.record_own_move((4, 4));
    // The opponent must play in the centre board; (0, 0) is not allowed.
    let lines = checker.check_turn((0, 0), &[(1, 1)]);
    assert!(lines[0].contains("the opponent played 0 0"), "{lines:?}");
    assert!(lines
        .last()
        .unwrap()
        .contains("no longer following the game"));
    // Later turns are counted but not compared.
    let lines = checker.check_turn((2, 2), &[(7, 7)]);
    assert_eq!(lines.len(), 1);
    assert_eq!(checker.turns(), 3);
    assert_eq!(checker.differences(), 1);
}

#[test]
fn notices_a_misplaced_first_turn_marker() {
    let mut checker = Checker::new();
    checker.check_turn((-1, -1), &legal(&Board::new()));
    checker.record_own_move((0, 0));
    let lines = checker.check_turn((-1, -1), &[(0, 1)]);
    assert!(lines
        .iter()
        .any(|line| line.contains("-1 -1 after the first turn")));
}

#[test]
fn lists_long_differences_briefly() {
    let actions: Vec<(i32, i32)> = (0..20).map(|index| (index, 0)).collect();
    let text = list(actions.iter());
    assert!(text.starts_with("0 0, 1 0"), "{text}");
    assert!(text.ends_with("and 8 more"), "{text}");
    assert_eq!(list([].iter()), "none");
}
