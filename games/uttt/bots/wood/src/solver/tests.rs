use super::*;

/// Our worst score when we follow the solver and the opponent tries every
/// possible move. `to_move_is_us` says whose turn it is.
fn worst_case(solver: &mut Solver, mine: u16, theirs: u16, to_move_is_us: bool) -> f64 {
    if has_line(mine) {
        return 1.0;
    }
    if has_line(theirs) {
        return 0.0;
    }
    if (mine | theirs) & FULL == FULL {
        return 0.5;
    }
    if to_move_is_us {
        let cell = solver.best_move(mine, theirs).unwrap();
        assert_eq!(
            (mine | theirs) & 1 << cell,
            0,
            "the solver chose a marked cell"
        );
        worst_case(solver, mine | 1 << cell, theirs, false)
    } else {
        cells(!(mine | theirs) & FULL)
            .map(|cell| worst_case(solver, mine, theirs | 1 << cell, true))
            .fold(1.0, f64::min)
    }
}

/// Our average score when we follow the solver and the opponent plays
/// uniformly at random, computed by exact enumeration.
fn against_random(solver: &mut Solver, mine: u16, theirs: u16, to_move_is_us: bool) -> f64 {
    if has_line(mine) {
        return 1.0;
    }
    if has_line(theirs) {
        return 0.0;
    }
    if (mine | theirs) & FULL == FULL {
        return 0.5;
    }
    if to_move_is_us {
        let cell = solver.best_move(mine, theirs).unwrap();
        against_random(solver, mine | 1 << cell, theirs, false)
    } else {
        let empty = !(mine | theirs) & FULL;
        let total: f64 = cells(empty)
            .map(|cell| against_random(solver, mine, theirs | 1 << cell, true))
            .sum();
        total / f64::from(empty.count_ones())
    }
}

#[test]
fn never_loses_moving_first_or_second() {
    let mut solver = Solver::new();
    assert_eq!(worst_case(&mut solver, 0, 0, true), 0.5);
    assert_eq!(worst_case(&mut solver, 0, 0, false), 0.5);
}

#[test]
fn wins_most_games_against_a_random_opponent() {
    let mut solver = Solver::new();
    let first = against_random(&mut solver, 0, 0, true);
    let second = against_random(&mut solver, 0, 0, false);
    eprintln!("expected score against random play: first {first:.4}, second {second:.4}");
    assert!(first > 0.99, "moving first: {first}");
    assert!(second > 0.95, "moving second: {second}");
    // The solver's own estimate agrees with the enumeration.
    assert!((solver.value(0, 0).expected - first).abs() < 1e-9);
}

#[test]
fn takes_a_win_and_blocks_a_threat() {
    let mut solver = Solver::new();
    // Cells are numbered 0 to 8, row by row.
    // We hold 0 and 1, the opponent 3 and 4: cell 2 wins at once.
    assert_eq!(solver.best_move(0b000_000_011, 0b000_011_000), Some(2));
    // We hold 4 and 8, the opponent 0 and 1: cell 2 blocks the top row
    // (and creates two threats of our own).
    assert_eq!(solver.best_move(0b100_010_000, 0b000_000_011), Some(2));
}

#[test]
fn has_no_move_when_the_game_is_over() {
    let mut solver = Solver::new();
    assert_eq!(solver.best_move(0b111, 0b11000), None);
    assert_eq!(solver.best_move(0b011_000_101, 0b100_111_010), None);
}
