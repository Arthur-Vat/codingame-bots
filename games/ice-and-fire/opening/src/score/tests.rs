use ice_and_fire_engine::{Command, State};

use super::*;
use crate::test_util::{start, Position};

const VOID: &str = "############";
const BLUE_HQ: &str = "###########X";

/// Red holds the top row up to (3, 0); the four cells below it are free;
/// blue's HQ is walled off.
fn pocket_position() -> Position<'static> {
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
        units: &[],
    }
}

fn pocket(units: &[(u8, u32, u8, u8, u8)]) -> State {
    Position {
        units,
        ..pocket_position()
    }
    .state()
}

#[test]
fn the_voronoi_share_counts_the_cells_a_side_reaches_first() {
    let state = pocket(&[]);
    // Red reaches its 4 cells and the 4 below; blue only its HQ.
    assert_eq!(voronoi_halves(&state, 0), 16);
    assert_eq!(voronoi_halves(&state, 1), 2);
    assert_eq!(terms(&state, 0, Settings::DEFAULT).voronoi, 8 * CELL);
}

#[test]
fn a_tie_counts_half() {
    // Red holds (0, 0) and (1, 0); blue holds (9, 0) to (11, 0) and the last
    // column. (2, 0) to (4, 0) are nearer red, (5, 0) is a tie at 4 steps.
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
    assert_eq!(voronoi_halves(&state, 0), 2 * 5 + 1);
    // The other 23 − 5.5 cells are blue's.
    assert_eq!(voronoi_halves(&state, 1), 2 * 17 + 1);
}

#[test]
fn the_economy_is_counted_a_few_turns_into_the_middle_game() {
    let state = Position {
        gold: [7, 0],
        ..pocket_position()
    }
    .state();
    // Income 4: four cells, no units.
    let terms = terms(&state, 0, Settings::DEFAULT);
    assert_eq!((terms.gold, terms.income), (7, TURNS * 4));
    let five = super::terms(
        &state,
        0,
        Settings {
            turns: 5,
            ..Settings::DEFAULT
        },
    );
    assert_eq!(five.income, 20);
}

#[test]
fn a_side_that_cannot_pay_its_upkeep_loses_its_units() {
    // Four cells, upkeep 1 + 1 + 4: income -2 with no gold.
    let units = [(0, 1, 1, 1, 0), (0, 2, 1, 2, 0), (0, 3, 2, 3, 0)];
    let state = pocket(&units);
    assert_eq!(state.income(0), -2);
    let terms = terms(&state, 0, Settings::DEFAULT);
    assert_eq!(terms.debt, 40);
    assert_eq!(terms.income, -2 * TURNS);
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
fn a_chain_the_enemy_pays_next_turn_is_a_loss_counted_once() {
    // Blue is not to act: its gold plus its income (22) pays the training
    // of level 1 on red's HQ.
    let state = next_door();
    assert!(threatened(&state, 0));
    let red = terms(&state, 0, Settings::DEFAULT);
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
    assert!(state.apply(Command::Train {
        level: 1,
        x: 0,
        y: 0
    }));
    assert_eq!(score(&state, 1, Settings::DEFAULT), LOSS);
    assert_eq!(score(&state, 0, Settings::DEFAULT), -LOSS);
}

/// Red's territory runs along the top row to (5, 0), down to (5, 2) and east
/// to a dead end: a mine on (6, 2) and a level 1 unit on (7, 2). Blue's
/// territory comes up column 4 to (4, 2), next to (5, 2), the cell that joins
/// the dead end to red's HQ. Blue has no gold; its income is 17.
fn dead_end() -> State {
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
        gold: [0, 0],
        buildings: &[(0, 1, 6, 2)],
        units: &[(0, 1, 1, 7, 2)],
    }
    .state()
}

#[test]
fn exposure_is_the_enemy_s_best_cut_on_its_next_turn() {
    let state = dead_end();
    assert_eq!(state.income(1), 17);
    // After red's turn blue has 17 gold: one training, on (5, 2), takes three
    // cells and the mine (their income until the horizon) and the unit.
    assert_eq!(exposure(&state, 0, Settings::DEFAULT), TURNS * (3 + 4) + 10);
    // Blue's territory is a single line: red has no gold to cut it.
    assert_eq!(exposure(&state, 1, Settings::DEFAULT), 0);
}

#[test]
fn far_apart_territories_are_not_exposed() {
    for seed in 1..=20 {
        let state = start(seed);
        for seat in 0..2 {
            assert_eq!(exposure(&state, seat, Settings::DEFAULT), 0, "seed {seed}");
        }
    }
}

#[test]
fn the_score_of_one_seat_is_the_other_s_negated() {
    let state = dead_end();
    let red = score(&state, 0, Settings::DEFAULT);
    assert_eq!(red, -score(&state, 1, Settings::DEFAULT));
    assert_ne!(red, 0);
}
