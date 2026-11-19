use ice_and_fire_engine::{League, State};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

use super::*;

/// At the start red holds its HQ alone, away from the centre.
#[test]
fn the_measures_of_a_new_game() {
    for seed in 0..10 {
        let map = generate(seed, RefereeLeague::Bronze);
        let state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let zones = Zones::of(&state);
        let (active, held, to_centre, share, vor) = measures(&state, 0, &zones);
        assert_eq!((active, held), (1, 0));
        assert!(to_centre > 0 && to_centre != u8::MAX);
        assert!(share <= 1.0);
        assert!((vor + voronoi(&state, 1) - 1.0).abs() < 1e-9);
    }
}
