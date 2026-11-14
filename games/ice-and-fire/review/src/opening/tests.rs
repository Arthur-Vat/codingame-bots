use ice_and_fire_engine::{Command, League, State};

use super::*;

/// An open 12×12 map with no void cell and no mine spot.
fn open_state() -> State {
    State::new(&[false; ice_and_fire_engine::CELLS], &[], League::Bronze)
}

#[test]
fn zones_split_the_map_by_the_difference_of_distances() {
    let state = open_state();
    let zone = zones(&state, 0);
    // Red's HQ is home, blue's is far, the anti-diagonal is the middle.
    assert_eq!(zone[HQ[0]], 0);
    assert_eq!(zone[HQ[1]], 2);
    assert_eq!(zone[11], 1);
    assert_eq!(zones(&state, 1)[HQ[1]], 0);
}

#[test]
fn a_unit_with_free_land_left_is_free_and_one_beaten_to_it_is_beaten() {
    let mut state = open_state();
    assert!(state.apply(Command::Train {
        level: 1,
        x: 1,
        y: 0
    }));
    // Enough turns for the gold of two more trainings.
    for _ in 0..20 {
        state.play_turn(&[]);
    }
    // The unit at (1, 0) has free neighbours (2, 0) and (1, 1).
    let start = state.clone();
    assert_eq!(idle(&start, &start, 0, 1), UnitTurn::Free);
    let mut end = start.clone();
    assert!(end.apply(Command::Train {
        level: 1,
        x: 2,
        y: 0
    }));
    assert!(end.apply(Command::Train {
        level: 1,
        x: 1,
        y: 1
    }));
    assert_eq!(idle(&start, &end, 0, 1), UnitTurn::Beaten);
}

#[test]
fn a_move_onto_a_neutral_cell_counts_and_its_training_is_followed() {
    let state = open_state();
    let mut record_state = state.clone();
    // Red trains at (1, 0) on turn 1, then moves it each turn.
    let mut steps = Vec::new();
    let mut id = None;
    for turn in 0..16u8 {
        let commands = if record_state.to_act() == 0 {
            match id {
                None => vec![Command::Train {
                    level: 1,
                    x: 1,
                    y: 0,
                }],
                Some(id) => vec![Command::Move { id, x: 11, y: 0 }],
            }
        } else {
            Vec::new()
        };
        let before = record_state.clone();
        record_state.play_turn(&commands);
        if id.is_none() && turn == 0 {
            id = record_state.unit_id(1);
        }
        steps.push(crate::replay::Step {
            state: before,
            commands,
            ms: None,
        });
    }
    let game = Game {
        name: "t".to_string(),
        record: crate::test_util::random_record(1, ["a", "b"]),
        steps,
        end: record_state,
    };
    let [red, _] = study(&game, 3);
    assert_eq!(red.plies.len(), 3);
    assert_eq!(red.plies[0].trainings, 1);
    assert_eq!(red.plies[1].units[UnitTurn::Neutral as usize], 1);
    assert_eq!(red.trainings.len(), 1);
    assert_eq!(red.trainings[0].taken, Some(WINDOW));
}
