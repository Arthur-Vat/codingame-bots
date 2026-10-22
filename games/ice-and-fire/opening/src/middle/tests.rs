use ice_and_fire_engine::{Command, State};

use super::reached;
use crate::econ;
use crate::test_util::{start, Position};

/// Red holds a row from its HQ to (3, 0); blue holds the end of the row from
/// (6, 0) and a column down to its HQ. Two free cells lie between them.
fn row(gold: i64, units: &[(u8, u32, u8, u8, u8)]) -> State {
    let column = "###########X";
    Position {
        rows: [
            "OOOO..XXXXXX",
            column,
            column,
            column,
            column,
            column,
            column,
            column,
            column,
            column,
            column,
            column,
        ],
        gold: [gold, 0],
        buildings: &[],
        units,
    }
    .state()
}

#[test]
fn a_chain_to_an_enemy_cell_needs_its_price_in_gold() {
    // (4, 0), (5, 0), then (6, 0): three trainings of level 1.
    assert!(!reached(&row(20, &[])));
    assert!(reached(&row(30, &[])));
}

#[test]
fn a_unit_s_step_is_free() {
    // The unit on (3, 0) steps onto (4, 0): two trainings remain.
    let unit = [(0, 1, 1, 3, 0)];
    assert!(!reached(&row(10, &unit)));
    assert!(reached(&row(20, &unit)));
}

#[test]
fn the_start_of_a_game_is_not_the_middle_game() {
    for seed in 1..=50 {
        assert!(!reached(&start(seed)), "seed {seed}");
    }
}

#[test]
fn the_econ_opening_reaches_the_middle_game() {
    for seed in 1..=50 {
        let mut state = start(seed);
        while !reached(&state) {
            assert!(state.game_turn() < 20, "seed {seed}: no middle game");
            let commands: Vec<Command> = econ::rules_turn(&state);
            state.play_turn(&commands);
        }
        assert!(
            state.game_turn() >= 4,
            "seed {seed}: at turn {}",
            state.game_turn()
        );
    }
}
