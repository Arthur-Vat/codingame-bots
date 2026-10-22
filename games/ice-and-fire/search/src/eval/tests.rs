use ice_and_fire_engine::{League, State};

use ice_and_fire_engine::Command;

use super::Necks;

/// Red (to act, no gold) holds the top row to (5, 0), then (5, 1) and a dead
/// end (5, 2), (6, 2), (7, 2), a level 1 unit on (7, 2); (6, 1) is free. Blue (no gold,
/// income 17) comes up column 4 to (4, 2), next to (5, 2).
fn dead_end() -> State {
    dead_end_with(0)
}

/// [`dead_end`] with `gold` for red.
fn dead_end_with(gold: i64) -> State {
    let rows = [
        "OOOOOO######",
        "#####O.#####",
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
    let mut turn = format!("{gold}\n9\n0\n17\n");
    for row in rows {
        turn.push_str(row);
        turn.push('\n');
    }
    turn.push_str("2\n0 0 0 0\n1 0 11 11\n1\n0 1 1 7 2\n");
    State::read("0\n", &turn, League::Bronze, 10).expect("a valid position")
}

#[test]
fn a_joint_the_enemy_can_reach_costs_half_what_it_cuts_off() {
    let state = dead_end();
    assert_eq!(state.income(1), 17);
    let necks = Necks::of(&state, 0);
    // (5, 0) cuts off (5, 1), (5, 2), (6, 2) and (7, 2) with its unit: 7 + 7
    // + 7 + 17 = 38; (5, 1) cuts off 31; each cell of the top row cuts off
    // what lies beyond it, (1, 0) the most: 4 x 7 + 38.
    assert_eq!(necks.joints.first(), Some(&(1, 66)));
    assert!(necks.joints.contains(&(5, 38)));
    assert!(necks.joints.contains(&(17, 31)));
    assert!(necks.joints.contains(&(17, 31)));
    // Blue's budget: 0 + 2 x 17 = 34, three trainings of level 1. It can take
    // (5, 2) for 10, (5, 1) for 20, (5, 0) for 30: the worst it can take is
    // (5, 0).
    assert_eq!(necks.exposed(&state), 38 / 2);
    // Red's budget, 0 + 2 x 9 = 18, takes the tip (4, 2), which cuts off
    // nothing.
    assert_eq!(Necks::of(&state, 1).exposed(&state), 0);
}

#[test]
fn growing_beyond_a_joint_costs_nothing_and_covering_it_gains() {
    let state = dead_end_with(40);
    let necks = Necks::of(&state, 0);
    assert_eq!(necks.exposed(&state), 38 / 2);
    // A unit trained beyond the joints, at (6, 1): the region beyond (5, 0)
    // grows, the value lost stays the one at the start of the turn.
    let mut grown = state.clone();
    assert!(grown.apply(Command::Train {
        level: 1,
        x: 6,
        y: 1
    }));
    assert!(Necks::of(&grown, 0).joints.contains(&(5, 38 + 17)));
    assert_eq!(necks.exposed(&grown), 38 / 2);
    // A unit on (5, 0) costs blue a level 2 training there: 20 + 10 + 10 =
    // 40, more than its 34. (5, 1) is the worst it can take now: 31 / 2.
    let mut covered = state.clone();
    assert!(covered.apply(Command::Train {
        level: 1,
        x: 5,
        y: 0
    }));
    assert_eq!(necks.exposed(&covered), 31 / 2);
}

#[test]
fn a_territory_without_joints_has_no_neck() {
    // The start of a game: an HQ and nothing to cut off.
    let map = ice_and_fire_referee::map::generate(1, ice_and_fire_referee::League::Bronze);
    let state = State::new(&map.void, &map.mine_spots, League::Bronze);
    assert!(Necks::of(&state, 0).joints.is_empty());
    assert_eq!(Necks::of(&state, 0).exposed(&state), 0);
    assert_eq!(Necks::of(&state, 1).exposed(&state), 0);
}
