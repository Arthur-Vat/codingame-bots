use ice_and_fire_engine::{Bits, League, State, SIZE};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

use super::{arrivals, race_net, with_fronts};

/// Each front's lane and turn-1 goal cells.
fn goal_cells(state: &State) -> Vec<(usize, [usize; 3])> {
    with_fronts(state, |fronts| {
        fronts.iter().map(|f| (f.lane, f.cells)).collect()
    })
}

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

#[test]
fn the_moving_entrance_lies_between_the_front_line_and_the_region() {
    let state = map_925707787();
    with_fronts(&state, |fronts| {
        for front in fronts {
            let [e, c, o] = super::moving(&state, 0, front);
            // At the start red holds its HQ alone: the entrance lies
            // halfway to the region's entrance cell, the centre and the
            // outpost are the turn-1 ones.
            let half = front.entrance.div_ceil(2).max(1).min(front.entrance);
            assert_eq!(e, front.axis[half]);
            assert_eq!(c, front.cells[1]);
            assert_eq!(o, front.cells[2]);
            // Blue's are the mirrors along its own axis.
            let [be, _, _] = super::moving(&state, 1, front);
            assert_ne!(be, e);
        }
    });
}

#[test]
fn the_pull_toward_the_centre_counts_the_steps_of_each_side() {
    let state = map_925707787();
    with_fronts(&state, |fronts| {
        let middle = fronts.iter().find(|f| f.lane == 1).expect("a middle front");
        // From red's HQ to (5, 5), and from blue's two cells to (6, 6).
        assert_eq!(middle.from_centre[0], 10);
        assert_eq!(middle.from_mirror[143], 10);
    });
}
