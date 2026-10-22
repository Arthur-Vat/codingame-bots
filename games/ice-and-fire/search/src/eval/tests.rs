use ice_and_fire_engine::{League, State};

use super::neck;

/// Red (to act, no gold) holds the top row to (5, 0), then (5, 1) and a dead
/// end (5, 2), (6, 2), (7, 2), a level 1 unit on (7, 2). Blue (no gold,
/// income 17) comes up column 4 to (4, 2), next to (5, 2).
fn dead_end() -> State {
    let rows = [
        "OOOOOO######",
        "#####O######",
        "####XOOO####",
        "####X#######",
        "####X#######",
        "####X#######",
        "####X#######",
        "####X#######",
        "####X#######",
        "####X#######",
        "####X#######",
        "####XXXXXXXX",
    ];
    // Red: 10 cells, upkeep 1. Blue: 17 cells.
    let mut turn = String::from("0\n9\n0\n17\n");
    for row in rows {
        turn.push_str(row);
        turn.push('\n');
    }
    turn.push_str("2\n0 0 0 0\n1 0 11 11\n1\n0 1 1 7 2\n");
    State::read("0\n", &turn, League::Bronze, 10).expect("a valid position")
}

#[test]
fn a_neck_the_enemy_can_reach_costs_half_what_it_cuts_off() {
    let state = dead_end();
    assert_eq!(state.income(1), 17);
    // Blue's budget: 0 + 2 x 17 = 34, three trainings of level 1. It can take
    // (5, 2) for 10, (5, 1) for 20, (5, 0) for 30. Taking (5, 0) cuts off
    // (5, 1), (5, 2), (6, 2) and (7, 2) with its unit: 7 + 7 + 7 + 17 = 38.
    assert_eq!(neck(&state, 0), 38 / 2);
    // Red's budget, 0 + 2 x 9 = 18, takes the tip (4, 2), which cuts off
    // nothing.
    assert_eq!(neck(&state, 1), 0);
}

#[test]
fn a_territory_without_joints_has_no_neck() {
    // The start of a game: an HQ and nothing to cut off.
    let map = ice_and_fire_referee::map::generate(1, ice_and_fire_referee::League::Bronze);
    let state = State::new(&map.void, &map.mine_spots, League::Bronze);
    assert_eq!(neck(&state, 0), 0);
    assert_eq!(neck(&state, 1), 0);
}
