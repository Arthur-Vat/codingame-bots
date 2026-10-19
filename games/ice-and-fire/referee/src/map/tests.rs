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

/// The cells 4-connected to cell `start` through land.
fn component(void: &[bool; CELLS], start: usize) -> [bool; CELLS] {
    let mut seen = [false; CELLS];
    seen[start] = true;
    let mut stack = vec![start];
    while let Some(index) = stack.pop() {
        let (x, y) = (index % SIZE, index / SIZE);
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push(index - 1);
        }
        if x + 1 < SIZE {
            neighbours.push(index + 1);
        }
        if y > 0 {
            neighbours.push(index - SIZE);
        }
        if y + 1 < SIZE {
            neighbours.push(index + SIZE);
        }
        for next in neighbours {
            if !void[next] && !seen[next] {
                seen[next] = true;
                stack.push(next);
            }
        }
    }
    seen
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

fn league_named(name: &str) -> League {
    match name {
        "WOOD3" => League::Wood3,
        "WOOD2" => League::Wood2,
        "WOOD1" => League::Wood1,
        "BRONZE" => League::Bronze,
        _ => panic!("golden.txt: unknown league {name:?}"),
    }
}

/// The cells of a map as 12 rows, `#` for void, for failure messages.
fn drawing(map: &Map) -> String {
    let mut text = String::new();
    for y in 0..SIZE {
        text.push('\n');
        for x in 0..SIZE {
            let spot = map.is_mine_spot(x, y);
            text.push(match (map.is_void(x, y), spot) {
                (true, _) => '#',
                (false, true) => 'o',
                (false, false) => '.',
            });
        }
    }
    text.push_str(&format!("\nspots {:?}", map.mine_spots));
    text
}

/// Parses one non-comment line of `golden.txt`, failing loudly if it is malformed.
fn parse_golden(line: &str) -> (u64, League, usize, Map) {
    let mut fields = line.split(' ');
    let mut field = |what: &str| {
        fields
            .next()
            .unwrap_or_else(|| panic!("golden.txt: missing {what} in {line:?}"))
    };
    let seed = field("seed").parse::<u64>().expect("golden.txt: bad seed");
    let league = league_named(field("league"));
    let count = field("count")
        .parse::<usize>()
        .expect("golden.txt: bad count");
    let cells = field("cells");
    assert_eq!(cells.len(), CELLS, "golden.txt: {CELLS} cells, seed {seed}");
    let mut void = [false; CELLS];
    for (cell, symbol) in void.iter_mut().zip(cells.chars()) {
        *cell = match symbol {
            '#' => true,
            '.' => false,
            other => panic!("golden.txt: bad cell {other:?}, seed {seed}"),
        };
    }
    let mine_spots = fields
        .map(|spot| {
            let (x, y) = spot.split_once(',').expect("golden.txt: bad mine spot");
            let x: usize = x.parse().expect("golden.txt: bad mine spot x");
            let y: usize = y.parse().expect("golden.txt: bad mine spot y");
            assert!(
                x < SIZE && y < SIZE,
                "golden.txt: spot off the map, seed {seed}"
            );
            (x, y)
        })
        .collect();
    (seed, league, count, Map { void, mine_spots })
}

#[test]
fn it_draws_the_maps_that_the_source_printed() {
    let mut checked = 0;
    for line in include_str!("golden.txt").lines() {
        if line.starts_with('#') {
            continue;
        }
        let (seed, league, count, expected) = parse_golden(line);
        let map = generate(seed, league);
        assert_eq!(
            map.mine_spots.len(),
            count,
            "seed {seed} {league:?}: mine spot count"
        );
        assert!(
            map == expected,
            "seed {seed} {league:?}\nexpected:{}\ngot:{}",
            drawing(&expected),
            drawing(&map)
        );
        checked += 1;
    }
    assert!(checked >= 100, "golden.txt holds only {checked} maps");
}
