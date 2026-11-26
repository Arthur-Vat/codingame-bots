use ice_and_fire_engine::{Bits, League, State, SIZE};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

use super::{arrivals, goal_cells, race_net};

/// The map of CodinGame's game 925707787 (its seed reproduces it), whose
/// fronts the turn-1 planner's prototype found (E080).
fn map_925707787() -> State {
    let seed = 4_283_710_958_304_316_400_i64 as u64;
    let map = generate(seed, RefereeLeague::Bronze);
    State::new(&map.void, &map.mine_spots, League::Bronze)
}

fn at(x: usize, y: usize) -> usize {
    y * SIZE + x
}

#[test]
fn the_goal_cells_are_the_prototypes() {
    let fronts = goal_cells(&map_925707787());
    assert_eq!(
        fronts,
        vec![
            (0, [at(3, 7), at(3, 8), at(5, 8)]),
            (1, [at(5, 5), at(5, 5), at(6, 6)]),
            (2, [at(7, 3), at(8, 3), at(8, 5)]),
        ]
    );
}

#[test]
fn a_side_arrives_at_once_on_its_land_and_a_turn_later_next_to_it() {
    let state = map_925707787();
    let red = arrivals(&state, 0);
    assert!(red[0].test(at(0, 0)));
    assert!(!red[0].test(at(1, 0)));
    assert!(red[1].test(at(1, 0)));
    // A chain of trainings arrives a turn after the steps from the land.
    assert!(!red[2].test(at(1, 1)));
    assert!(red[3].test(at(1, 1)));
}

#[test]
fn the_race_counts_wins_less_losses_and_ties_for_neither() {
    let state = map_925707787();
    let red = arrivals(&state, 0);
    let blue = arrivals(&state, 1);
    // Blue starts with two cells to red's one, so it is ahead.
    assert!(race_net(&red, &blue, state.map().valid) < 0);
    assert_eq!(race_net(&red, &blue, Bits::cell(at(1, 0))), 1);
    assert_eq!(race_net(&red, &blue, Bits::cell(at(10, 11))), -1);
}
