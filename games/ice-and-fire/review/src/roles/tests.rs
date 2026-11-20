use ice_and_fire_engine::{Command, League, State, CELLS};

use super::*;
use crate::replay::{Game, Step};
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
    let state = two_units();
    let id = state.unit_id(at(1, 0)).unwrap();
    // The unit is told to go to (3, 0), two cells away, and steps to (2, 0).
    let mv = Command::Move { id, x: 3, y: 0 };
    let next_to_unit = Command::Train {
        level: 1,
        x: 3,
        y: 0,
    };
    let next_to_target = Command::Train {
        level: 1,
        x: 4,
        y: 0,
    };
    let mut after = state.clone();
    after.play_turn(&[mv]);
    let landed = |i: u32| {
        let cell = cell_of(&after, 0, i)?;
        (cell_of(&state, 0, i) != Some(cell)).then_some(cell)
    };
    assert_eq!(landed(id), Some(at(2, 0)));
    assert_eq!(
        tip_trainings(&[mv, next_to_unit, next_to_target], landed),
        (2, 1)
    );
    // Trained before the move, it is not at the tip.
    assert_eq!(tip_trainings(&[next_to_unit, mv], landed), (1, 0));
    // A unit that did not move leaves no tip.
    assert_eq!(tip_trainings(&[mv, next_to_unit], |_| None), (1, 0));
    assert_eq!(tip_trainings(&[], landed), (0, 0));
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

/// A hand-made game on a corridor with a spur: the L of row 0 and column 11
/// from red's HQ (0, 0) to blue's (11, 11), and a spur of five cells,
/// (10, 4) to (6, 4), off (11, 4). Each side trains one unit and walks it
/// along `route`; blue's takes the spur and then stands still, and at game
/// turn 15 red's captures (11, 4), which cuts the spur off.
fn spur_game() -> Game {
    let mut void = [true; CELLS];
    for x in 0..12 {
        void[at(x, 0)] = false;
    }
    for y in 1..12 {
        void[at(11, y)] = false;
    }
    for x in 6..11 {
        void[at(x, 4)] = false;
    }
    let mut state = State::new(&void, &[], League::Bronze);
    let mut red: Vec<(u8, u8)> = (2..12).map(|x| (x, 0)).collect();
    red.extend((1..5).map(|y| (11, y)));
    let mut blue: Vec<(u8, u8)> = (4..10).rev().map(|y| (11, y)).collect();
    blue.extend((6..11).rev().map(|x| (x, 4)));
    let mut steps = Vec::new();
    for ply in 0..30 {
        let seat = state.to_act();
        let turn = state.game_turn() as usize;
        let route = if seat == 0 { &red } else { &blue };
        let mut commands = Vec::new();
        if turn == 1 {
            let (x, y) = if seat == 0 { (1, 0) } else { (11, 10) };
            commands.push(Command::Train { level: 1, x, y });
        } else if let Some(&(x, y)) = route.get(turn - 2) {
            let id = state.unit_id(state.units(seat).nth(0).unwrap()).unwrap();
            commands.push(Command::Move { id, x, y });
        }
        let before = state.clone();
        state.play_turn(&commands);
        steps.push(Step {
            state: before,
            commands,
            ms: None,
        });
        assert!(
            state.outcome().is_none(),
            "ply {ply} turn {turn} {:?}",
            state.outcome()
        );
        if state.game_turn() > 15 {
            break;
        }
    }
    Game {
        name: "spur".to_string(),
        record: random_record(1, ["a", "b"]),
        steps,
        end: state,
    }
}

#[test]
fn a_hand_made_game_gives_the_counts_of_every_measure() {
    let game = spur_game();
    // Every objective counts at every ply.
    let [red, blue] = sides(&game, 15, i32::MIN);
    let on = |side: &Side, t: u32| side.turns[&t].clone();
    // Red: one training at game turn 1, none after a move; a frontline unit
    // from turn 4 while objectives are open; it holds the five objectives
    // (cells 9 to 13 of the corridor) one by one from turn 10 to 14.
    assert_eq!((on(&red, 1).trainings, on(&red, 1).tips), (1, 0));
    assert_eq!((on(&red, 3).fronted, on(&red, 4).fronted), (0, 1));
    assert_eq!(
        (on(&red, 9).held, on(&red, 11).held, on(&red, 15).held),
        (0, 2, 5)
    );
    assert_eq!(on(&red, 14).fronted, 0);
    assert_eq!(
        (red.first_held.clone(), red.first_any),
        (vec![10, 11, 12, 13, 14], Some(10))
    );
    assert_eq!(red.ever_counted, 5);
    // At turn 15 red captures (11, 4), near an objective, one region, and
    // that cuts off the spur, 5 steps from the capture.
    let l = &red.locality;
    assert_eq!(
        (
            l.captured,
            l.near,
            l.capture_plies,
            l.multi,
            l.cut_plies,
            l.cut_far
        ),
        (1, 1, 1, 0, 1, 1)
    );
    // Blue walks into the spur (its pockets: neutral cells it fences off),
    // stands still at turns 13 and 14, boxed in, and the cut kills its unit.
    let pockets: Vec<u32> = (8..=13).map(|t| on(&blue, t).pockets).collect();
    assert_eq!(pockets, vec![5, 4, 3, 2, 1, 0]);
    let still: Vec<u32> = (11..=15).map(|t| on(&blue, t).still).collect();
    let idle: Vec<u32> = (11..=15).map(|t| on(&blue, t).idle).collect();
    assert_eq!((still, idle), (vec![0, 0, 1, 1, 0], vec![0, 0, 1, 1, 0]));
    assert_eq!(on(&blue, 14).still_edge, 0);
    assert_eq!(on(&blue, 15).units, 0);
    assert_eq!((blue.first_any, blue.first_held.len()), (None, 0));
    assert_eq!(blue.locality.captured, 0);
    // Red never stands still.
    assert!((1..=15).all(|t| on(&red, t).still == 0));
}

#[test]
fn objectives_more_than_two_apart_make_a_spread() {
    let (state, path) = corridor();
    let mut regions = Bits::cell(path[9]);
    regions.set(path[11]);
    assert!(!spread_apart(&state, regions));
    regions.set(path[13]);
    assert!(spread_apart(&state, regions));
    assert!(!spread_apart(&state, Bits::cell(path[9])));
}
