use ice_and_fire_engine::{Command, State};

use super::*;
use crate::tests::position;

/// Red (to act, game turn 10, no gold) holds the column x = 0 down to
/// (0, 8); blue holds the whole row 11. Red's joints are (0, 1) to (0, 7).
fn board() -> State {
    let mut rows = vec![vec![b'.'; 12]; 12];
    for row in rows.iter_mut().take(9) {
        row[0] = b'O';
    }
    for cell in rows[11].iter_mut() {
        *cell = b'X';
    }
    let rows: Vec<String> = rows
        .into_iter()
        .map(|r| String::from_utf8(r).expect("ascii"))
        .collect();
    let refs: [&str; 12] = std::array::from_fn(|i| rows[i].as_str());
    position(refs, [0, 0], &[])
}

#[test]
fn trainings_are_told_by_level_and_by_distance_to_the_enemy() {
    let typing = Typing::new(&board());
    let train = |level, x, y| typing.buy(Command::Train { level, x, y });
    // On the enemy's row, next to it, within 3 steps, farther.
    assert_eq!(train(1, 5, 11), Some(Buy::Train1Capture));
    assert_eq!(train(2, 5, 11), Some(Buy::Train2Capture));
    assert_eq!(train(3, 5, 10), Some(Buy::Train3Front));
    assert_eq!(train(1, 5, 10), Some(Buy::Train1Front));
    assert_eq!(train(1, 5, 9), Some(Buy::Train1Near));
    assert_eq!(train(2, 5, 8), Some(Buy::Train2Near));
    assert_eq!(train(1, 5, 7), Some(Buy::Train1Far));
    assert_eq!(train(3, 5, 2), Some(Buy::Train3Far));
}

#[test]
fn towers_are_told_by_the_joints_and_the_distance_to_the_enemy() {
    let state = board();
    let typing = Typing::new(&state);
    let tower = |x, y| typing.buy(Command::BuildTower { x, y });
    assert_eq!(tower(0, 3), Some(Buy::TowerJoint));
    assert_eq!(tower(1, 3), Some(Buy::TowerNextJoint));
    assert_eq!(tower(5, 10), Some(Buy::TowerFront));
    assert_eq!(tower(5, 9), Some(Buy::TowerNear));
    assert_eq!(tower(5, 5), Some(Buy::TowerFar));
}

#[test]
fn mines_count_and_other_commands_do_not() {
    let typing = Typing::new(&board());
    assert_eq!(
        typing.buy(Command::BuildMine { x: 3, y: 3 }),
        Some(Buy::Mine)
    );
    assert_eq!(typing.buy(Command::Move { id: 1, x: 1, y: 1 }), None);
    assert_eq!(typing.buy(Command::Wait), None);
}

#[test]
fn the_row_is_the_turn_band_and_the_gold_band() {
    // Game turn 10 is band T9 (2); no gold is under 40 (1).
    assert_eq!(Typing::new(&board()).row(), 2 * 4 + 1);
}

#[test]
fn the_kinds_are_numbered_as_the_table_has_them() {
    for (i, buy) in Buy::ALL.iter().enumerate() {
        assert_eq!(*buy as usize, i);
    }
}

#[test]
fn bad_bytes_are_refused_and_good_ones_read_back() {
    assert!(SpendTable::from_bytes(&[0; 5]).is_err());
    let mut bytes = vec![0u8; ROWS * WIDTH];
    bytes[0] = 200;
    bytes[1] = 100;
    assert!(SpendTable::from_bytes(&bytes).is_err(), "shares over 255");
    bytes[1] = 55;
    bytes[BUYS] = 9;
    let table = SpendTable::from_bytes(&bytes).expect("a table");
    assert_eq!(table.rows[0][0], 200);
    assert_eq!(table.rows[0][BUYS], 9);
    assert_eq!(SpendTable::decode(""), Ok(SpendTable::empty()));
    assert!(SpendTable::decode("abc").is_err());
}

#[test]
fn among_ranks_by_share_then_by_the_kinds_order() {
    let mut table = SpendTable::empty();
    // A row without purchases offers everything.
    assert!(table.among(0, Buy::Mine, 1));
    table.rows[0][3] = 100;
    table.rows[0][5] = 100;
    table.rows[0][7] = 55;
    assert!(table.among(0, Buy::Train1Far, 1));
    assert!(!table.among(0, Buy::Train2Front, 1));
    assert!(table.among(0, Buy::Train2Front, 2));
    assert!(table.among(0, Buy::Train2Far, 3));
    assert!(!table.among(0, Buy::Train2Far, 2));
    assert!(!table.among(0, Buy::Mine, 3));
}
