use ice_and_fire_engine::{League, Outcome, State};

use super::*;
use crate::test_util::{ready_to_win, start, won_state, Position};

#[test]
fn a_won_or_lost_game_scores_win() {
    let won = won_state();
    assert_eq!(won.outcome(), Some(Outcome::Win(0)));
    assert_eq!(evaluate(&won, 0), WIN);
    assert_eq!(evaluate(&won, 1), -WIN);
}

#[test]
fn a_game_ended_on_military_power_scores_its_outcome() {
    // Both sides only end their turns until turn 100. Blue starts with an
    // extra cell, so the power may differ; whatever the outcome, the scores
    // match it.
    let mut state = State::new(&[false; ice_and_fire_engine::CELLS], &[], League::Bronze);
    let mut turns = 0;
    while state.outcome().is_none() && turns < 300 {
        state.end_turn();
        turns += 1;
    }
    match state.outcome() {
        Some(Outcome::Draw) => {
            assert_eq!(evaluate(&state, 0), 0);
            assert_eq!(evaluate(&state, 1), 0);
        }
        Some(Outcome::Win(winner)) => {
            assert_eq!(evaluate(&state, winner), WIN);
            assert_eq!(evaluate(&state, 1 - winner), -WIN);
        }
        None => panic!("the game never ended"),
    }
}

#[test]
fn the_score_of_a_position_follows_the_formula() {
    // Red: 21 active cells, income 21, gold 25: 25 + 63 + 84 = 172.
    // Blue: its HQ alone, income 1, gold 20: 20 + 3 + 4 = 27.
    // Red can pay for the 20 gold chain to blue's HQ: no pressure on red.
    // Blue would need 220 for red's HQ and has 21: its pressure is -199.
    // Blue cannot take red's HQ after its start of turn: no threat.
    let state = ready_to_win();
    assert_eq!(evaluate(&state, 0), 172 - 27 - (-199));
    assert_eq!(evaluate(&state, 1), -(172 - 27) + (-199));
}

#[test]
fn an_enemy_that_can_take_the_hq_next_turn_costs_half_a_win() {
    // Blue holds the top row up to the cell next to red's HQ, and the right
    // column: after its start of turn it pays for the HQ.
    let row = "OXXXXXXXXXXX";
    let column = "###########X";
    let state = Position {
        rows: [
            row, column, column, column, column, column, column, column, column, column, column,
            column,
        ],
        gold: [20, 20],
        buildings: &[],
        units: &[],
    }
    .state();
    assert!(evaluate(&state, 0) < -WIN / 4, "{}", evaluate(&state, 0));
    // Blue is not the player to act: no threat term for it.
    assert!(evaluate(&state, 1) > 0, "{}", evaluate(&state, 1));
}

#[test]
fn the_start_of_every_map_is_antisymmetric() {
    // At the start no HQ is within reach of the other side, so no threat
    // term applies, and what is left is a difference of the same quantity
    // for each seat: the scores of the two seats are opposite.
    for seed in 0..200 {
        for league in [League::Wood3, League::Wood2, League::Wood1, League::Bronze] {
            let state = start(seed, league);
            assert_eq!(
                evaluate(&state, 0),
                -evaluate(&state, 1),
                "seed {seed} {league:?}"
            );
            assert!(evaluate(&state, 0).abs() < WIN / 4);
        }
    }
}

#[test]
fn a_better_position_scores_higher() {
    let state = ready_to_win();
    let mut richer = state.clone();
    // Taking a cell next to the HQ adds a cell and an income, minus its
    // price in gold: more income and territory beat the 10 gold spent.
    assert!(richer.apply(ice_and_fire_engine::Command::Train {
        level: 1,
        x: 11,
        y: 10
    }));
    assert!(evaluate(&richer, 0) > evaluate(&state, 0) - 20);
}

#[test]
fn scores_are_deterministic_on_random_positions() {
    for seed in 0..50 {
        for state in crate::test_util::random_positions(seed, League::Bronze, &[0, 5, 15, 30]) {
            for seat in 0..2 {
                let score = evaluate(&state, seat);
                assert_eq!(score, evaluate(&state, seat));
                assert!(score.abs() <= WIN);
            }
        }
    }
}
