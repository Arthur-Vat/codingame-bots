use ice_and_fire_engine::{League, State, CELLS};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

use super::*;

/// The maps are symmetric through their centre, so are their zones; the
/// two sides' Voronoi shares add up to one (blue starts with two cells, red
/// with one, so they are not even).
#[test]
fn the_zones_are_symmetric_and_the_voronoi_shares_add_up() {
    let mut chokes_seen = 0;
    for seed in 0..20 {
        let map = generate(seed, RefereeLeague::Bronze);
        let state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let zones = Zones::of(&state);
        for cell in 0..CELLS {
            for zone in [zones.chokes, zones.centre, zones.middle] {
                assert_eq!(zone.test(cell), zone.test(CELLS - 1 - cell), "seed {seed}");
            }
        }
        assert!(!zones.centre.is_empty());
        assert!(zones.centre.and_not(zones.middle).is_empty());
        assert!(zones.middle_chokes().and_not(zones.chokes).is_empty());
        chokes_seen += zones.chokes.count();
        let (a, b) = (voronoi(&state, 0), voronoi(&state, 1));
        assert!((a + b - 1.0).abs() < 1e-9, "{a} {b}");
        assert_eq!(share(&state, 0, zones.centre), 0.0);
        let steps = to_centre(&state, 0, &zones);
        assert!(steps > 0 && steps != u8::MAX);
        assert!(share(&state, 0, zones.chokes) <= 1.0);
    }
    assert!(chokes_seen > 0);
}

/// Steps grow by one a layer and stop at void cells.
#[test]
fn steps_count_layers_from_the_sources() {
    let map = generate(3, RefereeLeague::Bronze);
    let state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let steps = steps_from(&state, Bits::cell(0));
    assert_eq!(steps[0], 0);
    for cell in state.map().valid.iter() {
        if cell != 0 && steps[cell] != u8::MAX {
            let around = Bits::cell(cell).neighbours() & state.map().valid;
            assert!(around.iter().any(|n| steps[n] + 1 == steps[cell]));
        }
    }
    for cell in state.map().void.iter() {
        assert_eq!(steps[cell], u8::MAX);
    }
}
