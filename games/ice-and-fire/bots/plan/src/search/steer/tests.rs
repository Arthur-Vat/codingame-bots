use ice_and_fire_engine::{Command, State, SIZE};

use super::*;
use crate::actions::generate;
use crate::tests::position;

/// Red holds the corner block x and y 0 to 3 and has a level-1 unit on
/// (2, 2) and one on (3, 3); blue holds the row 11. Red has gold for
/// trainings and towers.
fn board() -> State {
    let mut rows = vec![vec![b'.'; 12]; 12];
    for row in rows.iter_mut().take(4) {
        for cell in row.iter_mut().take(4) {
            *cell = b'O';
        }
    }
    for cell in rows[11].iter_mut() {
        *cell = b'X';
    }
    let rows: Vec<String> = rows
        .into_iter()
        .map(|r| String::from_utf8(r).expect("ascii"))
        .collect();
    let refs: [&str; 12] = std::array::from_fn(|i| rows[i].as_str());
    position(refs, [60, 0], &[(0, 1, 1, 2, 2), (0, 2, 1, 3, 3)])
}

fn is_move(action: &Action) -> bool {
    matches!(action, Action::One(Command::Move { .. }))
}

#[test]
fn the_filter_keeps_every_other_action_and_only_moves_of_the_top_kinds() {
    let state = board();
    let steer = Steer::new(&state);
    let mut all = Vec::new();
    generate(&state, &mut all);
    all.push(Action::Many(vec![Command::Wait]));
    let others = all.iter().filter(|a| !is_move(a)).count();
    let moves = all.len() - others;
    assert!(others > 0 && moves > 4, "{others} others, {moves} moves");
    let mut previous = moves;
    for k in [8, 3, 2, 1] {
        let mut kept = all.clone();
        steer.filter(&state, k, &mut kept);
        // Everything but the moves stays, in the order generated.
        let rest = |list: &[Action]| -> Vec<Vec<Command>> {
            list.iter()
                .filter(|a| !is_move(a))
                .map(|a| a.commands().to_vec())
                .collect()
        };
        assert_eq!(rest(&kept), rest(&all), "k {k}");
        let left = kept.iter().filter(|a| is_move(a)).count();
        assert!(left <= previous, "k {k}: {left} moves after {previous}");
        previous = left;
        // Exactly the moves whose kind is among the unit's k likeliest stay,
        // in the order generated.
        let in_top = |action: &Action| -> bool {
            let Action::One(Command::Move { id, x, y }) = action else {
                unreachable!()
            };
            let &(_, from, p) = steer
                .units
                .iter()
                .find(|u| u.0 == *id)
                .expect("a movable unit");
            let to = usize::from(*y) * SIZE + usize::from(*x);
            let kind = steer.typing.kind(&state, from, to);
            likeliest(&p).into_iter().take(k).any(|top| top == kind)
        };
        let expected: Vec<Vec<Command>> = all
            .iter()
            .filter(|a| is_move(a) && in_top(a))
            .map(|a| a.commands().to_vec())
            .collect();
        let got: Vec<Vec<Command>> = kept
            .iter()
            .filter(|a| is_move(a))
            .map(|a| a.commands().to_vec())
            .collect();
        assert_eq!(got, expected, "k {k}");
    }
    // All eight kinds keep every move; one kind removes some.
    let mut every = all.clone();
    steer.filter(&state, KINDS, &mut every);
    assert_eq!(every.len(), all.len());
    assert!(previous < moves, "k 1 removed nothing");
}

#[test]
fn the_prior_of_an_action_that_is_not_a_move_is_zero() {
    let state = board();
    let steer = Steer::new(&state);
    let mut all = Vec::new();
    generate(&state, &mut all);
    for action in all.iter().filter(|a| !is_move(a)) {
        assert_eq!(steer.prior(&state, action), 0);
    }
}
