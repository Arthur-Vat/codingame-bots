use ice_and_fire_engine::{Command, SIZE};

use super::{carry_out, marginal, matched, open_objectives, Settings, Targets};
use crate::econ::Buy;
use crate::econ::Params;
use crate::plan::Step;
use crate::solo::{self, ROLES};
use crate::test_util::{start, Position};

fn cell(x: usize, y: usize) -> usize {
    y * SIZE + x
}

/// Red holds rows 0 to 3, blue rows 9 to 11.
const FACING: [&str; 12] = [
    "OOOOOOOOOOOO",
    "OOOOOOOOOOOO",
    "OOOOOOOOOOOO",
    "OOOOOOOOOOOO",
    "............",
    "............",
    "............",
    "............",
    "............",
    "XXXXXXXXXXXX",
    "XXXXXXXXXXXX",
    "XXXXXXXXXXXX",
];

const ALL: [Targets; 4] = [
    Targets::Value,
    Targets::Matched,
    Targets::Frontier,
    Targets::Marginal,
];

#[test]
fn the_code_names_the_settings() {
    for targets in ALL {
        for fill in [false, true] {
            let s = Settings { targets, fill };
            assert!(s.code() < 8);
            assert_eq!(Settings::of_code(s.code()), s);
        }
    }
}

#[test]
fn the_matching_does_not_cross_units() {
    let state = Position {
        rows: FACING,
        gold: [0, 0],
        buildings: &[],
        units: &[(0, 1, 1, 2, 3), (0, 2, 1, 9, 3)],
    }
    .state();
    // Listed crossed: the right target first.
    let orders = matched(&state, 0, &[cell(9, 6), cell(2, 6)]);
    let mut units = orders.units.clone();
    units.sort_unstable();
    assert_eq!(
        units,
        vec![(cell(2, 3), cell(2, 6)), (cell(9, 3), cell(9, 6))]
    );
    assert!(orders.trainings.is_empty());
}

#[test]
fn a_training_takes_a_target_no_unit_reaches_first() {
    let state = Position {
        rows: FACING,
        gold: [10, 0],
        buildings: &[],
        units: &[(0, 1, 1, 2, 3)],
    }
    .state();
    let orders = matched(&state, 0, &[cell(2, 5), cell(9, 5)]);
    assert_eq!(orders.units, vec![(cell(2, 3), cell(2, 5))]);
    assert_eq!(orders.trainings, vec![cell(9, 5)]);
    // Carried out: the unit steps, then the training goes on our trainable
    // cell nearest its target, after the moves.
    let plan = carry_out(&state, &orders, false, &Params::V003);
    assert_eq!(
        plan,
        vec![
            Command::Move { id: 1, x: 2, y: 4 },
            Command::Train {
                level: 1,
                x: 9,
                y: 4
            },
        ]
    );
}

#[test]
fn a_training_spawns_from_the_tip_a_unit_made() {
    let state = Position {
        rows: FACING,
        gold: [10, 0],
        buildings: &[],
        units: &[(0, 1, 1, 2, 3)],
    }
    .state();
    // The unit and the training share a target: the training goes next to
    // the cell the unit has just taken.
    let orders = super::Orders {
        units: vec![(cell(2, 3), cell(2, 7))],
        trainings: vec![cell(2, 7)],
    };
    let plan = carry_out(&state, &orders, false, &Params::V003);
    assert_eq!(
        plan,
        vec![
            Command::Move { id: 1, x: 2, y: 4 },
            Command::Train {
                level: 1,
                x: 2,
                y: 5
            },
        ]
    );
}

#[test]
fn a_target_the_enemy_reaches_first_takes_no_agent() {
    let state = Position {
        rows: FACING,
        gold: [0, 0],
        buildings: &[],
        units: &[(0, 1, 1, 2, 3)],
    }
    .state();
    // (2, 8) is next to blue's land: blue is there first.
    let orders = matched(&state, 0, &[cell(2, 8)]);
    assert!(orders.units.is_empty());
}

#[test]
fn marginal_picks_add_land_and_differ() {
    let state = start(2026112101);
    let open = open_objectives(&state, 0);
    let picks = marginal(&state, 0, &open, 6);
    assert!(!picks.is_empty());
    let mut unique = picks.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), picks.len());
    assert!(picks.iter().all(|p| open.iter().any(|o| o.cell == *p)));
}

#[test]
fn every_variant_plays_a_legal_turn_through_the_planner() {
    let sets = solo::SETS.to_vec();
    for seed in [1u64, 7, 2026112101] {
        let mut state = start(seed);
        for _ in 0..8 {
            for targets in ALL {
                for fill in [false, true] {
                    let settings = Settings { targets, fill };
                    let step = Step {
                        weights: ROLES | settings.code(),
                        buy: Buy::Payback,
                    };
                    let plan = solo::commands(&state, &sets, step);
                    let mut after = state.clone();
                    for &command in &plan {
                        assert!(
                            after.apply(command),
                            "{settings:?}: {command:?} in {plan:?}"
                        );
                    }
                }
            }
            let plan = solo::commands(
                &state,
                &sets,
                Step {
                    weights: ROLES
                        | Settings {
                            targets: Targets::Matched,
                            fill: true,
                        }
                        .code(),
                    buy: Buy::Payback,
                },
            );
            state = solo::advance(&state, &plan);
        }
    }
}
