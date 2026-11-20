use ice_and_fire_engine::{Command, League, State, CELLS};

use super::*;
use crate::test_util::random_record;

fn open_state() -> State {
    State::new(&[false; CELLS], &[], League::Bronze)
}

fn at(x: usize, y: usize) -> usize {
    y * 12 + x
}

/// A corridor: row 0 and column 11, from red's HQ (0, 0) to blue's (11, 11),
/// 23 cells; `index` is the step along it.
fn corridor() -> (State, Vec<usize>) {
    let mut void = [true; CELLS];
    let mut path = Vec::new();
    for x in 0..12 {
        void[at(x, 0)] = false;
        path.push(at(x, 0));
    }
    for y in 1..12 {
        void[at(11, y)] = false;
        path.push(at(11, y));
    }
    (State::new(&void, &[], League::Bronze), path)
}

#[test]
fn an_objective_is_worth_the_land_it_decides() {
    let (state, path) = corridor();
    let (mine, theirs) = (Bits::cell(path[0]), Bits::cell(path[22]));
    // On the 23-cell path, with the goal at step 16, 19 cells are nearer
    // to our land and the goal (steps 0 to 18), 14 to the enemy's with it
    // (steps 9 to 22): value 5. At step 11, the middle, the two are equal.
    assert_eq!(value_of(&state, mine, theirs, path[16]), 5);
    assert_eq!(value_of(&state, mine, theirs, path[11]), 0);
    // A goal near our HQ is worth less than nothing: the enemy gains more.
    assert_eq!(value_of(&state, mine, theirs, path[5]), -6);
}

#[test]
fn objectives_are_the_band_and_the_chokes() {
    let (state, path) = corridor();
    let set = objectives(&state);
    // Every cell of a corridor is a choke; the band is the middle steps.
    assert!(set.test(path[11]));
    assert!(!set.test(path[0]));
}

#[test]
fn pockets_are_the_neutral_cells_our_land_fences_off() {
    let state = open_state();
    let valid = state.map().valid;
    let mut fence = Bits::EMPTY;
    for y in 0..12 {
        fence.set(at(5, y));
    }
    let enemy = Bits::cell(at(11, 11));
    // Columns 0 to 4 are cut off from the enemy by the fence of column 5.
    let p = pockets(valid, fence, enemy, enemy);
    assert_eq!(p.count(), 60);
    assert!(p.test(at(0, 0)) && !p.test(at(6, 0)) && !p.test(at(5, 0)));
    // With a gap in the fence the enemy gets through: no pocket.
    let mut gap = fence;
    gap.clear(at(5, 6));
    assert!(pockets(valid, gap, enemy, enemy).is_empty());
}

#[test]
fn a_frontline_unit_needs_two_approaching_moves() {
    let state = open_state();
    let goal = Bits::cell(at(6, 6));
    let (a, b, c) = (at(3, 3), at(4, 4), at(5, 5));
    assert_eq!(frontline_of(&state, [a, b, c], goal), Some(at(6, 6)));
    // One approaching move after one sideways is not enough.
    assert_eq!(frontline_of(&state, [at(3, 4), at(3, 3), b], goal), None);
    // Nor is a move away, and nothing without an objective.
    assert_eq!(frontline_of(&state, [c, b, a], goal), None);
    assert_eq!(frontline_of(&state, [a, b, c], Bits::EMPTY), None);
}

#[test]
fn a_tip_training_follows_a_move_to_a_neighbour() {
    let mv = Command::Move { id: 1, x: 4, y: 4 };
    let near = Command::Train {
        level: 1,
        x: 5,
        y: 4,
    };
    let far = Command::Train {
        level: 1,
        x: 8,
        y: 8,
    };
    assert_eq!(tip_trainings(&[mv, near, far]), (2, 1));
    // Trained before the move, it is not at the tip.
    assert_eq!(tip_trainings(&[near, mv]), (1, 0));
    assert_eq!(tip_trainings(&[]), (0, 0));
}

#[test]
fn a_unit_is_boxed_in_when_every_cell_within_two_steps_is_ours() {
    let state = open_state();
    let mut mine = Bits::EMPTY;
    for x in 0..5 {
        for y in 0..5 {
            mine.set(at(x, y));
        }
    }
    // (1, 1): everything within 2 steps lies in the 5x5 block.
    assert!(is_boxed_in(&state, mine, at(1, 1)));
    // (3, 3): (5, 3) is two steps away and outside the block.
    assert!(!is_boxed_in(&state, mine, at(3, 3)));
}

#[test]
fn a_random_game_is_measured_for_both_sides() {
    let game = replay("t", random_record(3, ["a", "b"]));
    let [a, b] = sides(&game, 15, 3);
    for side in [&a, &b] {
        assert!(side.turns.contains_key(&1));
        assert!(side.turns.keys().all(|&t| t <= 15));
        assert!(side.turns.values().all(|t| t.tips <= t.trainings));
    }
    let mut groups = Groups::new();
    groups.insert(("x".to_string(), 0), vec![a]);
    groups.insert(("x".to_string(), 1), vec![b]);
    let text = report(&groups, 15, 2, 3, 0);
    assert!(text.contains("| x red (1) |") && text.contains("Locality"));
}

/// Red's two trained units on the open map.
fn two_units() -> State {
    let mut state = open_state();
    state.play_turn(&[
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
    ]);
    state.play_turn(&[]);
    state
}

#[test]
fn the_nearest_unit_approaching_and_the_tip_unit_are_read_per_objective() {
    let state = two_units();
    let u1 = at(1, 0);
    let id1 = state.unit_id(u1).unwrap();
    let g = at(6, 0);
    // The unit on (1, 0) is nearest to (6, 0); it was one step farther.
    assert!(nearest_approaching(&state, 0, g, |id| (id == id1).then_some(at(0, 0))));
    // It stood on the same cell, or was not there: not approaching.
    assert!(!nearest_approaching(&state, 0, g, |id| (id == id1).then_some(u1)));
    assert!(!nearest_approaching(&state, 0, g, |_| None));
    // Red's land is its HQ and the two trained cells: the active cell
    // nearest to (6, 0) is (1, 0), which holds a unit; blue's is its HQ.
    assert!(tip_unit(&state, 0, g));
    assert!(!tip_unit(&state, 1, g));
}
