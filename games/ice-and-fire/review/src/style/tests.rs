use ice_and_fire_engine::{Command, League, State, CELLS};

use super::*;
use crate::replay::Step;
use crate::test_util::random_record;

/// An open 12×12 map with no void cell and no mine spot.
fn open_state() -> State {
    State::new(&[false; CELLS], &[], League::Bronze)
}

#[test]
fn the_centre_is_the_anti_diagonal_and_its_neighbours() {
    let state = open_state();
    let centre = centre(&state);
    // (11, 0) and (0, 11) are 11 steps from both HQs; (5, 5) is 10 and 12.
    assert!(centre.test(11));
    assert!(centre.test(11 * 12));
    assert!(!centre.test(5 * 12 + 5));
    assert!(!centre.test(HQ[0]));
}

#[test]
fn the_two_shares_of_the_land_add_up_and_blue_starts_ahead() {
    let state = open_state();
    // Blue starts with two cells (RULES.md), so it is nearer more land.
    let (red, blue) = (voronoi(&state, 0), voronoi(&state, 1));
    assert!((red + blue - 1.0).abs() < 1e-9);
    assert!(blue > red);
}

#[test]
fn a_tower_guards_its_cell_and_its_neighbours_on_the_frontier() {
    let mut state = open_state();
    // Red takes (1, 0) and (2, 0) by trainings, with the gold of a few turns.
    for _ in 0..20 {
        state.play_turn(&[]);
    }
    assert!(state.apply(Command::Train {
        level: 1,
        x: 1,
        y: 0
    }));
    assert!(state.apply(Command::Train {
        level: 1,
        x: 2,
        y: 0
    }));
    // No enemy cell is next to red's land yet: no frontier.
    assert_eq!(frontier(&state, 0), (0, 0));
    // Upkeep: two level 1 units on active cells.
    assert_eq!(upkeep(&state, 0), 2);
    assert_eq!(unit_value(&state, 0), 20);
}

#[test]
fn nearest_training_compares_with_every_trainable_cell() {
    let state = open_state();
    // Red's trainable cells at the start: (1, 0) and (0, 1), both 21 steps
    // from blue's HQ.
    assert_eq!(nearest_training(&state, 0, 1), Some(true));
    assert_eq!(nearest_training(&state, 0, 12), Some(true));
}

/// A game on the open map: red trains at (1, 0) on its first turn, then
/// both sides wait for `turns` game turns.
fn short_game(turns: u32) -> Game {
    let mut state = open_state();
    let mut steps = Vec::new();
    for ply in 0..2 * turns {
        let commands = if ply == 0 {
            vec![Command::Train {
                level: 1,
                x: 1,
                y: 0,
            }]
        } else {
            Vec::new()
        };
        let before = state.clone();
        state.play_turn(&commands);
        steps.push(Step {
            state: before,
            commands,
            ms: None,
        });
    }
    Game {
        name: "short".to_string(),
        record: random_record(1, ["red", "blue"]),
        steps,
        end: state,
    }
}

#[test]
fn a_short_game_gives_the_sides_their_measures() {
    let [red, blue] = sides(&short_game(12));
    assert_eq!(red.values[ADVANCED_TRAINING], Some(1.0));
    assert_eq!(red.values[TOWERS_BUILT], Some(0.0));
    assert_eq!(red.values[MINES_BUILT], Some(0.0));
    assert_eq!(red.values[FIRST_MINE], None);
    assert_eq!(blue.values[ADVANCED_TRAINING], None);
    // Turn 10 was reached by both; neither holds a centre cell.
    assert!(red.values[VORONOI_10].is_some());
    assert_eq!(blue.values[CENTRE_10], Some(0.0));
    // No turn 20, and nobody won.
    assert_eq!(red.values[UPKEEP], None);
    assert_eq!(red.values[WIN_TURN], None);
    assert_eq!(blue.values[LOSS_WARNING], None);
}

#[test]
fn the_winner_of_a_random_game_has_its_win_measures() {
    let game = replay("random", random_record(7, ["red", "blue"]));
    let sides = sides(&game);
    if let Some(winner) = game.hq_taken_by() {
        assert!(sides[winner].values[WIN_TURN].is_some());
        assert_eq!(sides[1 - winner].values[WIN_TURN], None);
        assert_eq!(
            sides[1 - winner].values[LOSS_WARNING],
            sides[winner].values[WIN_LEAD]
        );
    }
}

#[test]
fn medians_and_means_summarise_as_named() {
    assert_eq!(summarise(Summary::Median, &mut [3.0, 1.0, 2.0]), Some(2.0));
    assert_eq!(
        summarise(Summary::Median, &mut [4.0, 1.0, 2.0, 3.0]),
        Some(2.5)
    );
    assert_eq!(summarise(Summary::Mean, &mut [1.0, 2.0]), Some(1.5));
    assert_eq!(summarise(Summary::Mean, &mut []), None);
}
