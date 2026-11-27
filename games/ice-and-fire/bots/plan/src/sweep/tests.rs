use ice_and_fire_engine::{Command, League, State, SIZE};

use super::sweep_with;

/// A position read at game turn 10 with red to act, from 12 rows of `O`
/// (red), `X` (blue) and `.`, and units as (owner, id, level, x, y).
fn position(rows: [&str; 12], units: &[(u8, u32, u8, u8, u8)]) -> State {
    let mut text = String::from("50\n10\n50\n10\n");
    for row in rows {
        assert_eq!(row.len(), SIZE);
        text.push_str(row);
        text.push('\n');
    }
    text.push_str("2\n0 0 0 0\n1 0 11 11\n");
    text.push_str(&format!("{}\n", units.len()));
    for (owner, id, level, x, y) in units {
        text.push_str(&format!("{owner} {id} {level} {x} {y}\n"));
    }
    State::read("0\n", &text, League::Bronze, 10).expect("a valid position")
}

const ROWS: [&str; 12] = [
    "OOOO........",
    "OOOO........",
    "OOOO........",
    "............",
    "............",
    "............",
    "............",
    "............",
    "........XXXX",
    "........XXXX",
    "........XXXX",
    "........XXXX",
];

#[test]
fn units_behind_the_front_head_for_different_free_cells() {
    // Two red units side by side at (1, 1) and (2, 1), far from blue's land:
    // each steps toward a free cell of its own, onto different cells.
    let state = position(ROWS, &[(0, 1, 1, 1, 1), (0, 2, 1, 2, 1)]);
    let commands = sweep_with(&state, 4, 6);
    assert_eq!(commands.len(), 2, "{commands:?}");
    let cells: Vec<(u8, u8)> = commands
        .iter()
        .map(|c| match *c {
            Command::Move { x, y, .. } => (x, y),
            _ => panic!("a move: {c:?}"),
        })
        .collect();
    assert_ne!(cells[0], cells[1]);
    // Every move is one step from its unit.
    for command in &commands {
        let Command::Move { id, x, y } = *command else {
            unreachable!()
        };
        let start: (u8, u8) = if id == 1 { (1, 1) } else { (2, 1) };
        assert_eq!(x.abs_diff(start.0) + y.abs_diff(start.1), 1, "{command:?}");
    }
    // The engine accepts them in order.
    let mut after = state.clone();
    assert!(commands.iter().all(|&c| after.apply(c)));
}

#[test]
fn units_near_the_enemy_and_strong_units_are_left_to_the_search() {
    // A red unit at (7, 5), 4 steps from blue's land at (8, 8), and a
    // level-2 unit behind: the sweeper moves neither.
    let rows = [
        "OOOO........",
        "OOOO........",
        "OOOOOOOO....",
        "OOOOOOOO....",
        "OOOOOOOO....",
        "OOOOOOOO....",
        "............",
        "............",
        "........XXXX",
        "........XXXX",
        "........XXXX",
        "........XXXX",
    ];
    let state = position(rows, &[(0, 1, 1, 7, 5), (0, 2, 2, 1, 1)]);
    assert!(sweep_with(&state, 4, 6).is_empty());
}
