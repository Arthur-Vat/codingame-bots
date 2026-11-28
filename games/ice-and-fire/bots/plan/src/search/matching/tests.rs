use ice_and_fire_engine::{Command, State, SIZE};

use super::*;
use crate::tests::position;

/// Red holds the corner block x and y 0 to 3 with units on (2, 2), (3, 3)
/// and (1, 1); blue holds the row `wall` and has `blue` gold.
fn board(wall: usize, blue: i64) -> State {
    let mut rows = vec![vec![b'.'; 12]; 12];
    for row in rows.iter_mut().take(4) {
        for cell in row.iter_mut().take(4) {
            *cell = b'O';
        }
    }
    rows[11][11] = b'X';
    for cell in rows[wall].iter_mut() {
        *cell = b'X';
    }
    let rows: Vec<String> = rows
        .into_iter()
        .map(|r| String::from_utf8(r).expect("ascii"))
        .collect();
    let refs: [&str; 12] = std::array::from_fn(|i| rows[i].as_str());
    position(
        refs,
        [30, blue],
        &[(0, 1, 1, 2, 2), (0, 2, 1, 3, 3), (0, 3, 1, 1, 1)],
    )
}

fn cell_of(x: u8, y: u8) -> usize {
    usize::from(y) * SIZE + usize::from(x)
}

#[test]
fn a_sampled_turn_is_legal_keeps_units_apart_and_uncovers_nothing_in_reach() {
    let root = board(4, 60);
    let matcher = Matcher::new(&root);
    assert!(matcher.danger.test(cell_of(3, 3)), "the wall's reach");
    let mut moved = 0;
    for sample in 0..60 {
        let (commands, end) = matcher.sample(&root, matcher.seed(&root, sample));
        let mut scratch = root.clone();
        for &command in &commands {
            let Command::Move { id, x, y } = command else {
                panic!("a sampled turn only moves: {command:?}");
            };
            let from = (0..SIZE * SIZE)
                .find(|&c| scratch.unit_at(c).is_some_and(|(i, _)| i == id))
                .expect("the unit is there");
            let capture = scratch.owned(1).test(cell_of(x, y));
            assert!(
                !matcher.danger.test(from) || capture,
                "unit {id} uncovers {from}"
            );
            assert!(scratch.apply(command), "{command:?} is refused");
            moved += 1;
        }
        // One unit a cell: as many units as before, on the state returned.
        assert_eq!(scratch.units(0).count(), 3);
        assert_eq!(end.units(0), scratch.units(0));
    }
    assert!(moved > 0, "some unit moved");
}

#[test]
fn the_same_seed_gives_the_same_turn() {
    let root = board(11, 0);
    let matcher = Matcher::new(&root);
    let seed = matcher.seed(&root, 7);
    assert_eq!(matcher.sample(&root, seed).0, matcher.sample(&root, seed).0);
    let all: Vec<_> = (0..20)
        .map(|s| matcher.sample(&root, matcher.seed(&root, s)).0)
        .collect();
    assert!(all.iter().any(|c| c != &all[0]), "the samples differ");
}

#[test]
fn a_unit_in_reach_may_capture() {
    // The wall is blue's row 4, with no unit: stepping onto it captures.
    let root = board(4, 60);
    let matcher = Matcher::new(&root);
    let captured = (0..200).any(|s| {
        let (_, end) = matcher.sample(&root, matcher.seed(&root, s));
        end.owned(1).count() < root.owned(1).count()
    });
    assert!(captured, "no sample captured");
}
