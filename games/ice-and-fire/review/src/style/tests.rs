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

/// The cell at (x, y).
fn at(x: u8, y: u8) -> usize {
    usize::from(y) * 12 + usize::from(x)
}

/// The open map after each side trained two units next to its HQ and
/// walked them toward the enemy HQ until `until` holds at red's turn.
fn walk(until: impl Fn(&State) -> bool) -> State {
    let mut state = open_state();
    for _ in 0..120 {
        let seat = state.to_act();
        if seat == 0 && until(&state) {
            return state;
        }
        let (target, near) = if seat == 0 {
            (HQ[1], [(1, 0), (0, 1)])
        } else {
            (HQ[0], [(10, 11), (11, 10)])
        };
        let mut commands = Vec::new();
        if state.game_turn() == 1 {
            for (x, y) in near {
                commands.push(Command::Train { level: 1, x, y });
            }
        }
        for cell in state.units(seat).iter() {
            if let Some(id) = state.unit_id(cell) {
                let (x, y) = ((target % 12) as u8, (target / 12) as u8);
                commands.push(Command::Move { id, x, y });
            }
        }
        state.play_turn(&commands);
    }
    panic!("the walk never reached its state");
}

/// [`walk`] until the two lands touch.
fn contact() -> State {
    walk(|state| frontier(state, 0).0 > 0)
}

#[test]
fn a_tower_guards_its_cell_and_its_neighbours_on_the_frontier() {
    let state = contact();
    let front = state.active(0) & state.owned(1).neighbours();
    assert_eq!(frontier(&state, 0), (front.count(), 0));
    // A tower on an empty active cell of red's on or next to the frontier.
    let near_front = front | front.neighbours();
    let cell = (state.active(0) & near_front & state.empty_cells())
        .iter()
        .next()
        .expect("an empty cell near the frontier");
    assert!(state.gold(0) >= 15, "red can pay for a tower");
    let mut after = state.clone();
    let (x, y) = ((cell % 12) as u8, (cell / 12) as u8);
    assert!(after.apply(Command::BuildTower { x, y }));
    let zone = Bits::cell(cell) | Bits::cell(cell).neighbours();
    assert_eq!(frontier(&after, 0), (front.count(), (front & zone).count()));
    assert!(frontier(&after, 0).1 > 0);
}

#[test]
fn units_and_upkeep_count_by_level() {
    let state = contact();
    let units = state.units(0) & state.active(0);
    assert_eq!(upkeep(&state, 0), i64::from(units.count()));
    assert_eq!(
        unit_value(&state, 0),
        10 * i64::from(state.units(0).count())
    );
}

#[test]
fn a_kill_is_removed_and_a_pass_removes_nothing() {
    // Until a blue unit stands next to red's land.
    let state = walk(|state| !(state.units(1) & state.active(0).neighbours()).is_empty());
    let mut next = state.clone();
    next.play_turn(&[]);
    assert_eq!(removed(&state, &[], &next, 1), 0);
    // A level 2 training onto a blue unit next to red's land kills it.
    let cell = (state.units(1) & state.active(0).neighbours())
        .iter()
        .next()
        .expect("a blue unit next to red's land");
    assert!(state.gold(0) >= 20, "red can pay for a level 2 unit");
    let (x, y) = ((cell % 12) as u8, (cell / 12) as u8);
    let commands = [Command::Train { level: 2, x, y }];
    let mut next = state.clone();
    next.play_turn(&commands);
    assert!(removed(&state, &commands, &next, 1) >= 10);
}

#[test]
fn nearest_training_compares_with_every_trainable_cell() {
    let mut state = open_state();
    // Red's trainable cells at the start: (1, 0) and (0, 1), both 21 steps
    // from blue's HQ.
    assert_eq!(nearest_training(&state, 0, at(1, 0)), Some(true));
    assert_eq!(nearest_training(&state, 0, at(0, 1)), Some(true));
    // With (1, 0) taken, (2, 0) and (1, 1) are nearer than (0, 1).
    assert!(state.apply(Command::Train {
        level: 1,
        x: 1,
        y: 0
    }));
    assert_eq!(nearest_training(&state, 0, at(0, 1)), Some(false));
    assert_eq!(nearest_training(&state, 0, at(1, 1)), Some(true));
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
    // No cut: no gold before one. No turn 20, and nobody won.
    assert_eq!(red.values[CUT_GOLD], None);
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
