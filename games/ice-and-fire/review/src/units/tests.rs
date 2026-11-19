use ice_and_fire_engine::{Command, League, State};

use super::*;

/// An open 12×12 map with no void cell and no mine spot.
fn open_state() -> State {
    State::new(&[false; ice_and_fire_engine::CELLS], &[], League::Bronze)
}

#[test]
fn a_unit_next_to_free_land_can_take_it_and_is_one_step_out() {
    let mut state = open_state();
    assert!(state.apply(Command::Train {
        level: 1,
        x: 1,
        y: 0
    }));
    // The unit on (1, 0) may enter (2, 0) and (1, 1); it and red's HQ cell
    // (0, 0) are each next to a cell that is not red's.
    assert_eq!(takes(&state, 0, 1).count(), 2);
    assert_eq!(steps_out(&state, 0, 1), 1);
    assert_eq!(steps_out(&state, 0, 0), 1);
    // No unit stands on (5, 5): nothing to take from there.
    assert!(takes(&state, 0, 5 * 12 + 5).is_empty());
}

#[test]
fn the_phases_cover_every_turn() {
    assert_eq!(phase(1), "01-04");
    assert_eq!(phase(7), "05-07");
    assert_eq!(phase(8), "08-10");
    assert_eq!(phase(13), "11-13");
    assert_eq!(phase(17), "14-17");
    assert_eq!(phase(40), "18+");
}

#[test]
fn a_game_is_scanned_unit_turn_by_unit_turn() {
    // Red trains two units on its first turn, which are not counted (they
    // may not move); blue passes. On its second turn red moves the unit of
    // (1, 0) onto the neutral (2, 0) and leaves the unit of (0, 1) where it
    // is, next to the neutral (0, 2).
    let mut state = open_state();
    let first = state.clone();
    let trainings = vec![
        Command::Train {
            level: 1,
            x: 1,
            y: 0,
        },
        Command::Train {
            level: 1,
            x: 0,
            y: 1,
        },
    ];
    state.play_turn(&trainings);
    let pass = state.clone();
    state.play_turn(&[]);
    let second = state.clone();
    let id = second.unit_id(1).expect("the unit on (1, 0)");
    let moves = vec![Command::Move { id, x: 2, y: 0 }];
    let step = |state: State, commands: Vec<Command>| Step {
        state,
        commands,
        ms: None,
    };
    let steps = [
        step(first, trainings),
        step(pass, Vec::new()),
        step(second, moves),
    ];
    let mut sums = BTreeMap::new();
    scan(&steps, &mut sums);
    let red = &sums[&(0, "01-04")];
    assert_eq!(red.plies, 2);
    assert_eq!(red.trained, 2);
    // One unit took a neutral cell, the other stayed with one to take.
    assert_eq!(red.kinds, [1, 0, 0, 0, 1, 0, 0, 0, 0]);
    let blue = &sums[&(1, "01-04")];
    assert_eq!((blue.plies, blue.kinds), (1, [0; 9]));
}
