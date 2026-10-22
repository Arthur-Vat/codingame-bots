use ice_and_fire_engine::State;

use super::*;
use crate::test_util::{start, Position};

const VOID: &str = "############";
const BLUE_HQ: &str = "###########X";

/// Red holds the top row up to (3, 0); the four cells below it are free;
/// blue's HQ is walled off. Game turn 10: 10 turns to the horizon.
fn pocket(units: &[(u8, u32, u8, u8, u8)]) -> State {
    Position {
        rows: [
            "OOOO########",
            "....########",
            VOID,
            VOID,
            VOID,
            VOID,
            VOID,
            VOID,
            VOID,
            VOID,
            VOID,
            BLUE_HQ,
        ],
        gold: [0, 0],
        buildings: &[],
        units,
    }
    .state()
}

#[test]
fn turns_left_count_to_the_horizon_and_never_below_one() {
    let state = pocket(&[]);
    assert_eq!(state.game_turn(), 10);
    assert_eq!(turns_left(&state, 20), 10);
    assert_eq!(turns_left(&state, 5), 1);
}

#[test]
fn territory_counts_one_cell_a_turn() {
    // Four free cells, each 1 step away: taken on turns 1 to 4, worth
    // 9 + 8 + 7 + 6, whatever the units.
    assert_eq!(territory(&pocket(&[]), 0, 10), 30);
    let three = [(0, 1, 1, 1, 0), (0, 2, 1, 2, 0), (0, 3, 1, 3, 0)];
    assert_eq!(territory(&pocket(&three), 0, 10), 30);
    // With 3 turns left only the first two earn anything.
    assert_eq!(territory(&pocket(&[]), 0, 3), 2 + 1);
    // Blue reaches none of them.
    assert_eq!(territory(&pocket(&[]), 1, 10), 0);
}

#[test]
fn territory_splits_a_row_and_counts_a_tie_half() {
    // Free cells (2, 0) to (8, 0) between red (to (1, 0)) and blue (from
    // (9, 0)): red is nearer (2, 0) to (4, 0), at 1 to 3 steps, (5, 0) is a
    // tie at 4 steps, blue is nearer the rest.
    let state = Position {
        rows: [
            "OO.......XXX",
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
        ],
        gold: [0, 0],
        buildings: &[],
        units: &[],
    }
    .state();
    // 9 + 8 + 7, and half of 10 - 4.
    assert_eq!(territory(&state, 0, 10), 24 + 3);
    // Blue: (8, 0) to (6, 0) at 1 to 3 steps, and half the tie.
    assert_eq!(territory(&state, 1, 10), 24 + 3);
}

#[test]
fn a_side_that_cannot_pay_its_upkeep_loses_its_units() {
    // Four cells, upkeep 1 + 1 + 4: income -2 with no gold.
    let units = [(0, 1, 1, 1, 0), (0, 2, 1, 2, 0), (0, 3, 2, 3, 0)];
    let state = pocket(&units);
    assert_eq!(state.income(0), -2);
    let terms = terms(&state, 0, Settings::FULL);
    assert_eq!(terms.debt, 40);
    assert_eq!(terms.income, -20);
}

/// Red's HQ is next to blue's territory, which runs along the top row and
/// down the last column; blue has no gold.
fn next_door() -> State {
    Position {
        rows: [
            "OXXXXXXXXXXX",
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
            BLUE_HQ,
        ],
        gold: [0, 0],
        buildings: &[],
        units: &[],
    }
    .state()
}

#[test]
fn a_chain_the_enemy_pays_next_turn_is_a_loss() {
    // Blue is not to act: its gold plus its income (22) pays the training
    // of level 1 on red's HQ.
    let state = next_door();
    assert!(threatened(&state, 0));
    let red = terms(&state, 0, Settings::FULL);
    assert_eq!((red.loss, red.exposure), (LOSS, 0));
    // Red, to act, can pay nothing: blue's HQ is safe.
    assert!(!threatened(&state, 1));
    assert!(!threatened(&start(1), 0));
}

#[test]
fn a_finished_game_scores_the_loss_only() {
    let mut state = next_door();
    state.end_turn();
    // Blue trains on red's HQ.
    assert!(state.apply(ice_and_fire_engine::Command::Train {
        level: 1,
        x: 0,
        y: 0
    }));
    assert_eq!(score(&state, 1, Settings::FULL), LOSS);
    assert_eq!(score(&state, 0, Settings::FULL), -LOSS);
}

/// Red's territory runs along the top row to (5, 0), down to (5, 2) and east
/// to a dead end: a mine on (6, 2) and a level 1 unit on (7, 2). Blue's
/// territory comes up column 4 to (4, 2), next to (5, 2), the cell that joins
/// the dead end to red's HQ. Blue has no gold; its income is 17.
fn dead_end(red_gold: i64) -> State {
    let column = "####X#######";
    Position {
        rows: [
            "OOOOOO######",
            "#####O######",
            "####XOOO####",
            column,
            column,
            column,
            column,
            column,
            column,
            column,
            column,
            "####XXXXXXXX",
        ],
        gold: [red_gold, 0],
        buildings: &[(0, 1, 6, 2)],
        units: &[(0, 1, 1, 7, 2)],
    }
    .state()
}

#[test]
fn exposure_is_what_the_enemy_cuts_after_banking() {
    let state = dead_end(0);
    assert_eq!(state.income(1), 17);
    let one = Settings {
        horizon: 20,
        lookahead: 1,
    };
    // After one turn of banking, blue's 17 gold pay one training, on (5, 2):
    // three cells and a mine for 9 turns, and a unit of 10.
    assert_eq!(exposure(&state, 0, one), 3 * 9 + 10 + 4 * 9);
    // Banking longer can only cost more.
    assert!(exposure(&state, 0, Settings::FULL) >= exposure(&state, 0, one));
}

#[test]
fn a_tower_that_blocks_the_cut_lowers_the_exposure() {
    let one = Settings {
        horizon: 20,
        lookahead: 1,
    };
    // With 15 gold red builds a tower next to (5, 2): blue's 17 gold no
    // longer pay the level 3 unit that passes it; the loss is the tower.
    assert_eq!(exposure(&dead_end(15), 0, one), 15);
    assert!(exposure(&dead_end(15), 0, Settings::FULL) < exposure(&dead_end(0), 0, Settings::FULL));
}

#[test]
fn far_apart_territories_are_not_exposed() {
    for seed in 1..=20 {
        let state = start(seed);
        for seat in 0..2 {
            assert_eq!(exposure(&state, seat, Settings::FULL), 0, "seed {seed}");
        }
    }
}

#[test]
fn the_score_of_one_seat_is_the_other_s_negated() {
    let state = dead_end(15);
    let red = score(&state, 0, Settings::FULL);
    assert_eq!(red, -score(&state, 1, Settings::FULL));
    assert_ne!(red, 0);
}
