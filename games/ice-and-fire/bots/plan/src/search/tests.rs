use ice_and_fire_engine::{Command, League, State};

use super::breaks_upkeep;

/// Red, to play with `gold`, holds the top row (12 cells) with a level 1
/// unit on (11, 0): income 12 - 1 = 11. Blue holds its HQ corner.
fn row(gold: i64) -> State {
    let mut turn = format!("{gold}\n11\n0\n2\n");
    turn.push_str("OOOOOOOOOOOO\n");
    for _ in 0..10 {
        turn.push_str("............\n");
    }
    turn.push_str("..........XX\n");
    turn.push_str("2\n0 0 0 0\n1 0 11 11\n1\n0 1 1 11 0\n");
    State::read("0\n", &turn, League::Bronze, 20).expect("a valid position")
}

fn after(state: &State, command: Command) -> State {
    let mut next = state.clone();
    assert!(next.apply(command), "{command:?}");
    next
}

#[test]
fn a_purchase_that_makes_income_negative_is_refused() {
    let state = row(100);
    assert_eq!(state.income(0), 11);
    // A level 3 unit: upkeep 20, a new cell: income 11 + 1 - 20 = -8.
    let level3 = after(
        &state,
        Command::Train {
            level: 3,
            x: 0,
            y: 1,
        },
    );
    assert_eq!(level3.income(0), -8);
    assert!(breaks_upkeep(&state, &level3));
    // A level 2 unit: income 11 + 1 - 4 = 8, still positive.
    let level2 = after(
        &state,
        Command::Train {
            level: 2,
            x: 0,
            y: 1,
        },
    );
    assert!(!breaks_upkeep(&state, &level2));
    // A move costs nothing.
    let step = after(&state, Command::Move { id: 1, x: 11, y: 1 });
    assert!(!breaks_upkeep(&state, &step));
}

#[test]
fn income_already_negative_may_still_rise() {
    // After the level 3, a level 1 elsewhere adds a cell and costs 1: the
    // income stays -8, which is not lower, so it is allowed.
    let state = after(
        &row(100),
        Command::Train {
            level: 3,
            x: 0,
            y: 1,
        },
    );
    let next = after(
        &state,
        Command::Train {
            level: 1,
            x: 1,
            y: 1,
        },
    );
    assert_eq!(next.income(0), -8);
    assert!(!breaks_upkeep(&state, &next));
    // A second level 3 lowers it further: refused.
    let worse = after(
        &state,
        Command::Train {
            level: 3,
            x: 2,
            y: 1,
        },
    );
    assert!(breaks_upkeep(&state, &worse));
}
