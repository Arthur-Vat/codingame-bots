use super::*;

fn run(input: &str) -> (Result<(), InputError>, Vec<String>) {
    let mut output = Vec::new();
    let result = play(&mut Rng::new(1), Input::new(input.as_bytes()), &mut output);
    let moves = String::from_utf8(output)
        .expect("moves are ASCII")
        .lines()
        .map(str::to_string)
        .collect();
    (result, moves)
}

/// The Wood league input of a turn: the opponent's last move and every
/// empty cell, given the marked cells.
fn wood_turn(opponent: &str, marked: u16) -> String {
    let empty: Vec<usize> = (0..9).filter(|&cell| marked & 1 << cell == 0).collect();
    let mut turn = format!("{opponent}\n{}\n", empty.len());
    for cell in empty {
        turn.push_str(&format!("{} {}\n", cell / 3, cell % 3));
    }
    turn
}

fn as_move(cell: usize) -> String {
    format!("{} {}", cell / 3, cell % 3)
}

#[test]
fn follows_the_solver_turn_after_turn() {
    let mut solver = Solver::new();
    // We move first.
    let first = solver.best_move(0, 0).unwrap();
    // The opponent answers in the first empty cell.
    let reply = (0..9).find(|&cell| cell != first).unwrap();
    let (mine, theirs) = (1u16 << first, 1u16 << reply);
    let second = solver.best_move(mine, theirs).unwrap();

    let input = [
        wood_turn("-1 -1", 0),
        wood_turn(&as_move(reply), mine | theirs),
    ]
    .concat();
    let (result, moves) = run(&input);
    assert_eq!(result, Err(InputError::Eof));
    assert_eq!(moves, vec![as_move(first), as_move(second)]);
}

#[test]
fn moving_second_it_reads_the_opponents_cells_from_the_valid_actions() {
    let mut solver = Solver::new();
    // The opponent opened in the centre.
    let expected = solver.best_move(0, 1 << 4).unwrap();
    let (_, moves) = run(&wood_turn("1 1", 1 << 4));
    assert_eq!(moves, vec![as_move(expected)]);
    assert_ne!(expected, 4);
}

#[test]
fn falls_back_to_random_valid_actions_on_the_9x9_board() {
    let input = "-1 -1\n3\n4 4\n0 8\n8 0\n";
    let (result, moves) = run(input);
    assert_eq!(result, Err(InputError::Eof));
    assert!(
        ["4 4", "0 8", "8 0"].contains(&moves[0].as_str()),
        "{moves:?}"
    );
}

#[test]
fn wood_move_takes_the_winning_cell() {
    let mut solver = Solver::new();
    // We hold cells 0 and 1, the opponent 3 and 4; cell 2 wins.
    let actions = [(0, 2), (1, 2), (2, 0), (2, 1), (2, 2)];
    assert_eq!(
        wood_move(&mut solver, 0b000_000_011, &actions),
        Some((0, 2))
    );
    // Nothing to play when no cell is empty.
    assert_eq!(wood_move(&mut solver, 0b011_000_101, &[]), None);
}
