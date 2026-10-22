use ice_and_fire_engine::{Command, League, State};

use super::{generate, Action};
use crate::eval::Necks;

/// Red (to act, 30 gold) holds (0, 0) to (2, 0), then (2, 1), (2, 2), (3, 2)
/// and (4, 2): one line from its HQ, every cell but the tip a joint. Blue
/// holds its HQ only.
fn line() -> State {
    let rows = [
        "OOO.........",
        "..O.........",
        "..OOO.......",
        "............",
        "............",
        "............",
        "............",
        "............",
        "............",
        "............",
        "............",
        "...........X",
    ];
    let mut turn = String::from("30\n7\n0\n1\n");
    for row in rows {
        turn.push_str(row);
        turn.push('\n');
    }
    turn.push_str("2\n0 0 0 0\n1 0 11 11\n0\n");
    State::read("0\n", &turn, League::Bronze, 10).expect("a valid position")
}

/// The actions made of level 1 trainings only.
fn bypasses(actions: &[Action]) -> Vec<Vec<Command>> {
    actions
        .iter()
        .filter_map(|action| match action {
            Action::Many(commands)
                if commands
                    .iter()
                    .all(|c| matches!(c, Command::Train { level: 1, .. })) =>
            {
                Some(commands.clone())
            }
            _ => None,
        })
        .collect()
}

#[test]
fn a_joint_gets_a_bypass_of_two_trainings() {
    let state = line();
    let necks = Necks::of(&state, 0);
    // (1, 0) cuts off the most.
    assert_eq!(necks.joints.first().map(|j| j.0), Some(1));
    let mut actions = Vec::new();
    generate(&state, &necks, &mut actions);
    let found = bypasses(&actions);
    // (0, 1) next to the HQ, then (1, 1) next to (2, 1) beyond (1, 0).
    let train = |x, y| Command::Train { level: 1, x, y };
    assert!(found.contains(&vec![train(0, 1), train(1, 1)]), "{found:?}");
    // Each bypass is accepted, and leaves (1, 0) cutting nothing off.
    for commands in &found {
        let mut next = state.clone();
        assert!(commands.iter().all(|&c| next.apply(c)), "{commands:?}");
        assert!(!Necks::of(&next, 0).joints.iter().any(|j| j.0 == 1));
    }
}

#[test]
fn no_bypass_without_gold_for_two_trainings() {
    let mut state = line();
    let necks = Necks::of(&state, 0);
    // Spend 20 of the 30: 10 left.
    assert!(state.apply(Command::Train {
        level: 1,
        x: 3,
        y: 0
    }));
    assert!(state.apply(Command::Train {
        level: 1,
        x: 5,
        y: 2
    }));
    let mut actions = Vec::new();
    generate(&state, &necks, &mut actions);
    assert!(bypasses(&actions).is_empty());
}
