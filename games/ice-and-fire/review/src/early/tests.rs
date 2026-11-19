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
        let (chokes, centre) = map_zones(&state);
        for cell in 0..CELLS {
            assert_eq!(
                chokes.test(cell),
                chokes.test(CELLS - 1 - cell),
                "seed {seed}"
            );
            assert_eq!(
                centre.test(cell),
                centre.test(CELLS - 1 - cell),
                "seed {seed}"
            );
        }
        assert!(!centre.is_empty());
        chokes_seen += chokes.count();
        let (a, b) = (voronoi(&state, 0), voronoi(&state, 1));
        assert!((a + b - 1.0).abs() < 1e-9, "{a} {b}");
        let (active, held, to_centre, share, _) = measures(&state, 0, chokes, centre);
        assert_eq!((active, held), (1, 0));
        assert!(to_centre > 0 && to_centre != u8::MAX);
        assert!(share <= 1.0);
    }
    assert!(chokes_seen > 0);
}
