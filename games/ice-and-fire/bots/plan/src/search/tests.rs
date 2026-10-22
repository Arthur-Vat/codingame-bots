use ice_and_fire_engine::{Command, League, State};

use super::{breaks_reserve, RESERVE};
use crate::actions::Action;

/// Red (to act, `gold`) holds the top-left corner with a level 1 unit on
/// (0, 1); a blue cell (2, 1) is next to it; blue's HQ is in its corner.
fn corner(gold: i64) -> State {
    let rows = [
        "OOO.........",
        "OOX.........",
        "............",
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
    let mut turn = format!("{gold}\n4\n0\n2\n");
    for row in rows {
        turn.push_str(row);
        turn.push('\n');
    }
    turn.push_str("2\n0 0 0 0\n1 0 11 11\n1\n0 1 1 0 1\n");
    State::read("0\n", &turn, League::Bronze, 10).expect("a valid position")
}

/// Whether `command`, accepted in `state`, breaks the reserve.
fn breaks(state: &State, command: Command) -> bool {
    let mut next = state.clone();
    assert!(next.apply(command), "{command:?}");
    breaks_reserve(state, &Action::One(command), &next)
}

#[test]
fn only_attacks_may_spend_the_reserve() {
    assert_eq!(RESERVE, 20);
    let state = corner(25);
    let neutral = Command::Train {
        level: 1,
        x: 3,
        y: 0,
    };
    let enemy = Command::Train {
        level: 1,
        x: 2,
        y: 1,
    };
    let tower = Command::BuildTower { x: 1, y: 1 };
    let step = Command::Move { id: 1, x: 0, y: 2 };
    // 25 - 10 leaves 15: a training on a neutral cell or a tower may not.
    assert!(breaks(&state, neutral));
    assert!(breaks(&state, tower));
    // A training on an enemy cell is an attack; a move spends nothing.
    assert!(!breaks(&state, enemy));
    assert!(!breaks(&state, step));
    // A macro action (a cut or a chain) is an attack.
    let mut next = state.clone();
    assert!(
        next.apply(neutral)
            && next.apply(Command::Train {
                level: 1,
                x: 4,
                y: 0
            })
    );
    let many = Action::Many(vec![
        neutral,
        Command::Train {
            level: 1,
            x: 4,
            y: 0,
        },
    ]);
    assert!(!breaks_reserve(&state, &many, &next));
}

#[test]
fn spending_above_the_reserve_is_free() {
    // 40 - 10 leaves 30.
    let state = corner(40);
    assert!(!breaks(
        &state,
        Command::Train {
            level: 1,
            x: 3,
            y: 0
        }
    ));
    // 30 - 10 leaves exactly 20.
    let state = corner(30);
    assert!(!breaks(
        &state,
        Command::Train {
            level: 1,
            x: 3,
            y: 0
        }
    ));
}
