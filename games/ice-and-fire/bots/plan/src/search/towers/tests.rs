use ice_and_fire_engine::{Command, State, SIZE};

use super::*;
use crate::actions::Action;
use crate::tests::position;

/// Red holds the rows 0 to 5, blue the rows 6 to 11; no units.
fn board() -> State {
    let mut rows = vec![vec![b'.'; 12]; 12];
    for (y, row) in rows.iter_mut().enumerate() {
        for cell in row.iter_mut() {
            *cell = if y < 6 { b'O' } else { b'X' };
        }
    }
    let rows: Vec<String> = rows
        .into_iter()
        .map(|r| String::from_utf8(r).expect("ascii"))
        .collect();
    let refs: [&str; 12] = std::array::from_fn(|i| rows[i].as_str());
    position(refs, [60, 60], &[])
}

fn tower(x: usize, y: usize) -> Action {
    Action::One(Command::BuildTower {
        x: x as u8,
        y: y as u8,
    })
}

#[test]
fn towers_far_from_the_enemy_and_from_any_threat_are_dropped() {
    let state = board();
    let me = state.to_act();
    // Row 5 is the border, row 4 two steps... the enemy's land starts at row 6.
    let mut actions = vec![
        tower(3, 5),
        tower(3, 4),
        tower(3, 3),
        tower(3, 1),
        Action::One(Command::Wait),
    ];
    keep_towers(&state, me, 2, Bits::EMPTY, &mut actions);
    let kept: Vec<_> = actions.iter().map(|a| a.commands().to_vec()).collect();
    assert_eq!(
        kept,
        vec![
            tower(3, 5).commands().to_vec(),
            tower(3, 4).commands().to_vec(),
            vec![Command::Wait]
        ]
    );
    // A threat next to a far tower keeps it.
    let mut actions = vec![tower(3, 1)];
    keep_towers(&state, me, 2, Bits::cell(2 * SIZE + 3), &mut actions);
    assert_eq!(actions.len(), 1);
}

#[test]
fn the_rank_is_the_distance_to_the_enemy() {
    let state = board();
    let me = state.to_act();
    assert_eq!(rank(&state, me, &tower(3, 5)), 1);
    assert_eq!(rank(&state, me, &tower(3, 2)), 4);
    assert_eq!(rank(&state, me, &Action::One(Command::Wait)), 0);
}
