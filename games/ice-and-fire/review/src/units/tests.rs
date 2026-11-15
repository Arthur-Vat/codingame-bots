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
    // The unit on (1, 0) may enter (2, 0) and (1, 1); red's HQ cell (0, 0)
    // is two steps from a cell that is not red's.
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
