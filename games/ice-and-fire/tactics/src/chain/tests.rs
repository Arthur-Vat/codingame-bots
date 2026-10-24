use ice_and_fire_engine::{League, Outcome, State, CELLS, HQ, SIZE};

use super::*;
use crate::test_util::{random_positions, ready_to_win, start, won_state, Position};

fn cell(x: usize, y: usize) -> usize {
    y * SIZE + x
}

fn train(level: u8, x: u8, y: u8) -> Command {
    Command::Train { level, x, y }
}

/// Red (seat 0) holds the top-left corner; a blue region reaches from its HQ
/// up the right column and along the top row, with a tower at (9, 0), so the
/// cells (8, 0), (9, 0) and (10, 0) need a level 3 unit (30 gold each).
/// The target of the tests is (11, 3). The straight way costs 200 in the
/// first estimate and 180 once the tower is taken, as (10, 0) is no longer
/// next to a tower. With `detour`, a longer way through neutral cells and a
/// blue level 1 unit at (11, 5) costs 190: the estimate prefers it, but the
/// tower is the cheaper way.
fn tower_world(detour: bool) -> State {
    let column = "###########X";
    let way = "#######.###X";
    let bottom = "#######....X";
    let rows = if detour {
        [
            "O.......XXXX",
            way,
            way,
            way,
            way,
            bottom,
            column,
            column,
            column,
            column,
            column,
            column,
        ]
    } else {
        [
            "O.......XXXX",
            column,
            column,
            column,
            column,
            column,
            column,
            column,
            column,
            column,
            column,
            column,
        ]
    };
    Position {
        rows,
        gold: [500, 20],
        buildings: &[(1, 2, 9, 0)],
        units: &[(1, 1, 1, 11, 5)],
    }
    .state()
}

const GOAL: usize = 3 * SIZE + 11;

/// Applies `commands` in order to a clone of `state`, which must accept all.
fn apply_all(state: &State, commands: &[Command]) -> State {
    let mut after = state.clone();
    for (index, &command) in commands.iter().enumerate() {
        assert!(after.apply(command), "command {index} {command:?} refused");
    }
    after
}

fn spent(commands: &[Command]) -> i64 {
    commands
        .iter()
        .map(|command| match command {
            Command::Train { level, .. } => 10 * i64::from(*level),
            other => panic!("not a training: {other:?}"),
        })
        .sum()
}

#[test]
fn reach_on_an_empty_start() {
    let state = State::new(&[false; CELLS], &[], League::Bronze);
    let reach = Reach::new(&state, 0, false);
    assert_eq!(reach.cost[HQ[0]], 0);
    assert_eq!(reach.before[HQ[0]], u8::MAX);
    assert_eq!(reach.cost[cell(1, 0)], 10);
    assert_eq!(reach.cost[cell(0, 1)], 10);
    assert_eq!(reach.cost[cell(2, 0)], 20);
    assert_eq!(reach.cost[cell(1, 1)], 20);
    assert_eq!(reach.cost[cell(5, 3)], 80);
    // Blue's HQ is 22 steps away, and nothing protects it.
    assert_eq!(reach.cost[HQ[1]], 220);
    assert_eq!(reach.cost[cell(11, 10)], 210);
    assert_eq!(reach.path(cell(1, 0)), vec![cell(1, 0)]);
    let path = reach.path(HQ[1]);
    assert_eq!(path.len(), 22);
    assert_eq!(path.last(), Some(&HQ[1]));
    // Each step of the path is a neighbour of the one before.
    let mut previous = HQ[0];
    for &next in &path {
        assert_eq!(state.map().distance(previous, next), 1);
        previous = next;
    }
    // Blue starts with a second cell, (11, 10), so it is one step nearer.
    let blue = Reach::new(&state, 1, false);
    assert_eq!(blue.cost[HQ[0]], 210);
    assert_eq!(blue.cost[HQ[1]], 0);
}

#[test]
fn reach_has_no_path_for_a_source_or_a_cell_off_the_map() {
    let state = State::new(&[false; CELLS], &[], League::Bronze);
    let reach = Reach::new(&state, 0, false);
    assert!(reach.path(HQ[0]).is_empty());
    assert!(reach.path(CELLS).is_empty());
    assert!(reach.path(usize::MAX).is_empty());
}

#[test]
fn a_cell_next_to_an_active_enemy_tower_costs_30() {
    let state = tower_world(false);
    let reach = Reach::new(&state, 0, false);
    assert_eq!(reach.cost[cell(7, 0)], 70);
    assert_eq!(reach.cost[cell(8, 0)], 100);
    assert_eq!(reach.cost[cell(9, 0)], 130);
    assert_eq!(reach.cost[cell(10, 0)], 160);
    assert_eq!(reach.cost[cell(11, 0)], 170);
    assert_eq!(reach.cost[GOAL], 200);
    // Blue's side: its own cells are its sources, and red has no tower.
    let blue = Reach::new(&state, 1, false);
    assert_eq!(blue.cost[cell(9, 0)], 0);
    assert_eq!(blue.cost[cell(7, 0)], 10);
    assert_eq!(blue.cost[HQ[0]], 80);
}

#[test]
fn regions_that_do_not_touch_are_unreachable() {
    let rows = [
        "OOO#########",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
    ];
    let state = Position {
        rows,
        gold: [100, 100],
        buildings: &[],
        units: &[],
    }
    .state();
    let reach = Reach::new(&state, 0, false);
    // The corridors are not joined: the other HQ is out of reach.
    assert_eq!(reach.cost[HQ[1]], UNREACHABLE);
    assert_eq!(reach.before[HQ[1]], u8::MAX);
    assert!(reach.path(HQ[1]).is_empty());
    assert_eq!(chain_to(&state, HQ[1], 1000), None);
}

#[test]
fn a_cell_no_level_may_enter_is_a_wall() {
    // The only way to (4, 0) is through (3, 0), a cell of red's own that no
    // chain reaches (it is not joined to the HQ) and that holds a unit: a
    // unit cannot be trained on it, so it is a wall. A level 3 unit enters
    // an enemy unit's cell, but not a cell of its own side.
    let rows = [
        "OO.o.#######",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
    ];
    let state = Position {
        rows,
        gold: [100, 100],
        buildings: &[],
        units: &[(0, 1, 1, 3, 0)],
    }
    .state();
    let reach = Reach::new(&state, 0, false);
    assert_eq!(reach.cost[cell(2, 0)], 10);
    assert_eq!(reach.cost[cell(3, 0)], UNREACHABLE);
    assert_eq!(reach.cost[cell(4, 0)], UNREACHABLE);
}

#[test]
fn reach_with_moves_starts_next_to_the_movable_units() {
    let rows = [
        "OOOO........",
        "###########.",
        "###########.",
        "###########.",
        "###########.",
        "###########.",
        "###########.",
        "###########.",
        "###########.",
        "###########.",
        "###########.",
        "###########X",
    ];
    let state = Position {
        rows,
        gold: [100, 20],
        buildings: &[],
        units: &[(0, 1, 1, 3, 0)],
    }
    .state();
    let plain = Reach::new(&state, 0, false);
    assert_eq!(plain.cost[cell(4, 0)], 10);
    let moving = Reach::new(&state, 0, true);
    assert_eq!(moving.cost[cell(4, 0)], 0);
    assert_eq!(moving.before[cell(4, 0)], u8::MAX);
    assert_eq!(moving.cost[cell(5, 0)], 10);
    assert!(moving.path(cell(4, 0)).is_empty());
    assert_eq!(moving.path(cell(6, 0)), vec![cell(5, 0), cell(6, 0)]);
    // The other seat has no unit to move, and `moves` means the player to act.
    let blue = Reach::new(&state, 1, true);
    let blue_plain = Reach::new(&state, 1, false);
    assert_eq!(blue.cost, blue_plain.cost);
}

#[test]
fn chain_to_a_neighbour_and_a_far_cell_on_an_empty_start() {
    let state = State::new(&[false; CELLS], &[], League::Bronze);
    let chain = chain_to(&state, cell(1, 0), 10).expect("one training");
    assert_eq!(chain.commands, vec![train(1, 1, 0)]);
    assert_eq!(chain.cost, 10);
    // Red has 21 gold at the start.
    let chain = chain_to(&state, cell(1, 1), 21).expect("two trainings");
    assert_eq!(chain.cost, 20);
    assert_eq!(spent(&chain.commands), 20);
    assert!(apply_all(&state, &chain.commands).owned(0).test(cell(1, 1)));
    // Not enough budget.
    assert_eq!(chain_to(&state, cell(1, 1), 19), None);
    assert_eq!(chain_to(&state, cell(1, 0), 0), None);
    assert_eq!(chain_to(&state, cell(1, 0), -5), None);
    // A budget above the gold does not make the chain affordable: three
    // trainings need 30 gold and red has 21.
    assert_eq!(chain_to(&state, cell(2, 1), 1000), None);
}

#[test]
fn chain_to_a_cell_already_taken_is_free() {
    let state = State::new(&[false; CELLS], &[], League::Bronze);
    let chain = chain_to(&state, HQ[0], 0).expect("already ours");
    assert!(chain.commands.is_empty());
    assert_eq!(chain.cost, 0);
    // A negative budget is no budget at all, even for a cell already held.
    assert_eq!(chain_to(&state, HQ[0], -1), None);
}

#[test]
fn chain_to_refuses_cells_off_the_map_void_cells_and_finished_games() {
    let mut void = [false; CELLS];
    void[cell(5, 5)] = true;
    let state = State::new(&void, &[], League::Bronze);
    assert_eq!(chain_to(&state, cell(5, 5), 1000), None);
    assert_eq!(chain_to(&state, CELLS, 1000), None);
    assert_eq!(chain_to(&state, usize::MAX, 1000), None);
    let reach = Reach::new(&state, 0, false);
    assert_eq!(reach.cost[cell(5, 5)], UNREACHABLE);
    assert!(reach.path(cell(5, 5)).is_empty());

    let mut won = won_state();
    assert_eq!(won.outcome(), Some(Outcome::Win(0)));
    assert_eq!(chain_to(&won, HQ[1], 1000), None);
    assert_eq!(chain_to(&won, cell(1, 0), 1000), None);
    won.end_turn();
    assert_eq!(chain_to(&won, HQ[0], 1000), None);
}

#[test]
fn chain_to_the_hq_takes_it() {
    let state = ready_to_win();
    let chain = chain_to(&state, HQ[1], 25).expect("two trainings");
    assert_eq!(chain.commands, vec![train(1, 11, 10), train(1, 11, 11)]);
    assert_eq!(chain.cost, 20);
    let after = apply_all(&state, &chain.commands);
    assert_eq!(after.outcome(), Some(Outcome::Win(0)));
    assert_eq!(chain_to(&state, HQ[1], 19), None);
}

#[test]
fn replanning_beats_the_estimate_when_a_capture_removes_a_tower() {
    let state = tower_world(false);
    let estimate = Reach::new(&state, 0, false).cost[GOAL];
    assert_eq!(estimate, 200);
    let chain = chain_to(&state, GOAL, 500).expect("a chain");
    assert!(chain.cost < estimate);
    assert_eq!(chain.cost, 180);
    assert_eq!(chain.cost, spent(&chain.commands));
    let after = apply_all(&state, &chain.commands);
    assert!(after.owned(0).test(GOAL));
    // The tower cell and its neighbour (8, 0) cost a level 3 unit each;
    // (10, 0) is cheap once the tower is gone.
    assert!(chain.commands.contains(&train(3, 8, 0)));
    assert!(chain.commands.contains(&train(3, 9, 0)));
    assert!(chain.commands.contains(&train(1, 10, 0)));
    // The budget is a bound on the whole chain.
    assert_eq!(chain_to(&state, GOAL, 179), None);
    assert!(chain_to(&state, GOAL, 180).is_some());
}

#[test]
fn the_tower_variant_beats_a_detour_the_estimate_prefers() {
    let state = tower_world(true);
    let estimate = Reach::new(&state, 0, false).cost[GOAL];
    assert_eq!(estimate, 190, "the detour is the estimate's way");
    let chain = chain_to(&state, GOAL, 500).expect("a chain");
    assert_eq!(chain.cost, 180);
    assert!(chain.commands.contains(&train(3, 9, 0)), "takes the tower");
    assert!(apply_all(&state, &chain.commands).owned(0).test(GOAL));
    // The plain attempt needs 190, the variant 180.
    let plain = attempt(&state, 0, &[GOAL], 500).expect("the detour");
    assert_eq!(plain.cost, 190);
    assert_eq!(
        chain_to(&state, GOAL, 185).map(|chain| chain.cost),
        Some(180)
    );
    assert_eq!(chain_to(&state, GOAL, 179), None);
}

#[test]
fn chain_to_is_deterministic() {
    let state = tower_world(true);
    assert_eq!(chain_to(&state, GOAL, 500), chain_to(&state, GOAL, 500));
}

/// The number of seeds of the property tests (a debug build is slower).
const SEEDS: u64 = if cfg!(debug_assertions) { 100 } else { 200 };

const PLIES: [u32; 6] = [0, 3, 8, 16, 30, 60];

const LEAGUES: [League; 4] = [League::Wood3, League::Wood2, League::Wood1, League::Bronze];

#[test]
fn chains_to_the_enemy_hq_apply_cleanly_and_never_cost_more_than_the_estimate() {
    let mut positions = 0;
    let mut chains = 0;
    for seed in 0..SEEDS {
        let league = LEAGUES[(seed % 4) as usize];
        for state in random_positions(seed, league, &PLIES) {
            positions += 1;
            let me = state.to_act();
            let gold = state.gold(me);
            let target = HQ[1 - me];
            let estimate = Reach::new(&state, me, false).cost[target];
            let found = chain_to(&state, target, gold);
            assert_eq!(found, chain_to(&state, target, gold), "deterministic");
            if estimate <= gold {
                let chain = found
                    .as_ref()
                    .unwrap_or_else(|| panic!("seed {seed}: affordable at {estimate}, no chain"));
                assert!(chain.cost <= estimate, "seed {seed}");
            }
            if let Some(chain) = found {
                chains += 1;
                assert!(chain.cost <= gold, "seed {seed}");
                assert_eq!(chain.cost, spent(&chain.commands), "seed {seed}");
                let after = apply_all(&state, &chain.commands);
                assert_eq!(after.outcome(), Some(Outcome::Win(me)), "seed {seed}");
            }
        }
    }
    assert!(positions > 400, "{positions} positions");
    assert!(chains > 0, "no chain to an HQ in {positions} positions");
}

#[test]
fn chains_to_any_cell_apply_cleanly_and_take_it() {
    let mut taken = 0;
    for seed in 0..SEEDS / 2 {
        let league = LEAGUES[(seed % 4) as usize];
        for state in random_positions(seed, league, &PLIES) {
            let me = state.to_act();
            let gold = state.gold(me);
            let first = Reach::new(&state, me, false);
            for target in (0..CELLS).step_by(7) {
                let found = chain_to(&state, target, gold);
                if first.cost[target] <= gold && !state.map().void.test(target) {
                    let chain = found.as_ref().unwrap_or_else(|| {
                        panic!("seed {seed} cell {target}: affordable, no chain")
                    });
                    assert!(
                        chain.cost <= first.cost[target],
                        "seed {seed} cell {target}"
                    );
                }
                let Some(chain) = found else {
                    continue;
                };
                taken += 1;
                assert!(chain.cost <= gold);
                assert_eq!(chain.cost, spent(&chain.commands));
                let after = apply_all(&state, &chain.commands);
                assert!(
                    after.outcome() == Some(Outcome::Win(me))
                        || (after.owned(me).test(target) && after.active(me).test(target)),
                    "seed {seed} cell {target}"
                );
            }
        }
    }
    assert!(taken > 100, "{taken} chains");
}

#[test]
fn the_start_of_every_map_has_costs_of_distance_times_ten_for_the_neighbours() {
    for seed in 0..SEEDS {
        let state = start(seed, League::Bronze);
        let reach = Reach::new(&state, 0, false);
        for (next, &cost) in reach.cost.iter().enumerate() {
            if cost == UNREACHABLE || next == HQ[0] {
                continue;
            }
            // Everything is level 1 on an empty start.
            assert_eq!(cost, 10 * i64::from(state.map().distance(HQ[0], next)));
        }
    }
}

#[test]
fn cost_to_is_the_cost_of_reach_on_every_cell() {
    let mut checked = 0;
    for seed in 1..=40 {
        for league in [League::Bronze, League::Wood1, League::Wood2, League::Wood3] {
            // Odd plies too, so that both seats are to act (the units' steps).
            let plies = [3, 4, 9, 10, 17, 18, 25, 26, 33, 34, 43, 44, 59, 60];
            for state in random_positions(seed, league, &plies) {
                for seat in 0..2 {
                    for moves in [false, true] {
                        let reach = Reach::new(&state, seat, moves);
                        for target in 0..CELLS {
                            assert_eq!(
                                cost_to(&state, seat, moves, target),
                                reach.cost[target],
                                "seed {seed}, {league:?}, turn {}, seat {seat}, moves {moves}, cell {target}",
                                state.game_turn()
                            );
                            checked += 1;
                        }
                    }
                }
            }
        }
    }
    assert!(checked > 100_000, "{checked} cells checked");
}
