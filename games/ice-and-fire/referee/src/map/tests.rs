use super::*;

const LEAGUES: [League; 4] = [League::Wood3, League::Wood2, League::Wood1, League::Bronze];
const SEEDS: u64 = 1000;

/// Runs `check` on the map of every seed in every league.
fn for_all_maps(check: impl Fn(u64, League, &Map)) {
    for league in LEAGUES {
        for seed in 0..SEEDS {
            check(seed, league, &generate(seed, league));
        }
    }
}

fn open_cells(map: &Map) -> usize {
    map.void.iter().filter(|&&void| !void).count()
}

#[test]
fn void_cells_and_mine_spots_are_symmetric_under_a_half_turn() {
    for_all_maps(|seed, league, map| {
        for y in 0..SIZE {
            for x in 0..SIZE {
                let (mx, my) = (SIZE - 1 - x, SIZE - 1 - y);
                assert_eq!(map.is_void(x, y), map.is_void(mx, my), "{seed} {league:?}");
                assert_eq!(
                    map.is_mine_spot(x, y),
                    map.is_mine_spot(mx, my),
                    "{seed} {league:?}"
                );
            }
        }
    });
}

#[test]
fn smoothing_keeps_a_symmetric_grid_symmetric() {
    let mut rng = Rng::new(7);
    for _ in 0..200 {
        let grid = smooth(&random_voids(&mut rng));
        for (index, &cell) in grid.iter().enumerate() {
            assert_eq!(cell, grid[mirror(index)]);
        }
    }
}

#[test]
fn the_open_cells_form_one_area() {
    for_all_maps(|seed, league, map| {
        let area = component(&map.void, 0);
        for (index, (&joined, &is_void)) in area.iter().zip(&map.void).enumerate() {
            assert_eq!(joined, !is_void, "{seed} {league:?} {index}");
        }
    });
}

#[test]
fn the_corner_blocks_are_open() {
    for_all_maps(|seed, league, map| {
        for a in 0..3 {
            for b in 0..3 {
                assert!(!map.is_void(a, b), "{seed} {league:?}");
                assert!(
                    !map.is_void(SIZE - 1 - a, SIZE - 1 - b),
                    "{seed} {league:?}"
                );
            }
        }
    });
}

#[test]
fn the_mine_spot_count_is_even_and_in_range() {
    for_all_maps(|seed, league, map| {
        let count = map.mine_spots.len();
        if !league.has_mine_spots() {
            assert_eq!(count, 0, "{seed} {league:?}");
            return;
        }
        let most = (open_cells(map) * 15 / 100).max(6);
        assert_eq!(count % 2, 0, "{seed} {league:?}");
        assert!(
            count >= 6 && count <= most / 2 * 2,
            "{seed} {league:?}: {count}"
        );
    });
}

#[test]
fn mine_spots_avoid_forbidden_and_void_cells_and_are_sorted() {
    for_all_maps(|seed, league, map| {
        for &(x, y) in &map.mine_spots {
            assert!(!map.is_void(x, y), "{seed} {league:?}");
            for forbidden in [(0, 0), (11, 11), (0, 1), (11, 10)] {
                assert_ne!((x, y), forbidden, "{seed} {league:?}");
            }
        }
        let mut sorted = map.mine_spots.clone();
        sorted.sort_by_key(|&(x, y)| (y, x));
        sorted.dedup();
        assert_eq!(sorted, map.mine_spots, "{seed} {league:?}");
    });
}

#[test]
fn the_hq_neighbours_are_spots_from_wood_1_and_there_are_none_before() {
    for_all_maps(|seed, league, map| {
        if league.has_mine_spots() {
            assert!(map.is_mine_spot(1, 0), "{seed} {league:?}");
            assert!(map.is_mine_spot(10, 11), "{seed} {league:?}");
        } else {
            assert!(map.mine_spots.is_empty(), "{seed} {league:?}");
        }
    });
}

#[test]
fn the_same_seed_gives_the_same_map() {
    for league in LEAGUES {
        for seed in 0..SEEDS {
            assert_eq!(generate(seed, league), generate(seed, league));
        }
    }
}

#[test]
fn different_seeds_give_different_maps() {
    let first = generate(1, League::Bronze);
    assert!((2..50).any(|seed| generate(seed, League::Bronze) != first));
}

#[test]
fn off_map_cells_are_void() {
    let map = generate(1, League::Bronze);
    assert!(map.is_void(12, 0));
    assert!(map.is_void(0, 12));
    assert!(!map.is_void(0, 0));
}

#[test]
fn the_closest_pair_breaks_ties_by_the_index_of_a_then_of_b() {
    // The corner block as the area.
    let mut void = [true; CELLS];
    let mut area = [false; CELLS];
    for y in 0..3 {
        for x in 0..3 {
            void[y * SIZE + x] = false;
            area[y * SIZE + x] = true;
        }
    }
    // (5, 0) is 3 from (2, 0); (3, 2) is 1 from (2, 2): the nearest wins.
    void[5] = false;
    void[2 * SIZE + 3] = false;
    assert_eq!(
        closest_pair(&void, &area),
        Some((2 * SIZE + 2, 2 * SIZE + 3))
    );
    // (5, 0) and (0, 5) are both 3 away: the smaller index of a wins.
    void[2 * SIZE + 3] = true;
    void[5 * SIZE] = false;
    assert_eq!(closest_pair(&void, &area), Some((2, 5)));
    // (3, 0) and (3, 1) are both 1 away: the smaller index of a wins.
    void[SIZE + 3] = false;
    void[3] = false;
    assert_eq!(closest_pair(&void, &area), Some((2, 3)));
    // Nothing outside the area.
    assert_eq!(closest_pair(&void, &[true; CELLS]), None);
}

#[test]
fn connect_carves_along_a_row_then_a_column() {
    let mut void = [true; CELLS];
    for y in 0..3 {
        for x in 0..3 {
            void[y * SIZE + x] = false;
        }
    }
    void[4 * SIZE + 5] = false; // (5, 4), nearest to a = (2, 2)
    connect(&mut void);
    // Row 2 from x = 2 to 5, then column 5 from y = 2 to 4.
    for x in 3..=5 {
        assert!(!void[2 * SIZE + x], "({x}, 2)");
    }
    for y in 3..=4 {
        assert!(!void[y * SIZE + 5], "(5, {y})");
    }
    // The mirrored corridor joins the same area.
    assert!(!void[CELLS - 1 - (2 * SIZE + 4)]);
    let area = component(&void, 0);
    assert!((0..CELLS).all(|index| area[index] != void[index]));
}
