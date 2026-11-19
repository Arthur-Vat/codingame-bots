use ice_and_fire_engine::{Command, HQ};

use super::*;
use crate::test_util::start;

#[test]
fn the_hungarian_method_finds_the_cheapest_assignment() {
    let cost = vec![vec![4, 1, 3], vec![2, 0, 5], vec![3, 2, 2]];
    let rows = hungarian(&cost);
    let total: i64 = rows.iter().enumerate().map(|(r, &c)| cost[r][c]).sum();
    assert_eq!(total, 5);
    let mut cols = rows.clone();
    cols.sort_unstable();
    assert_eq!(cols, vec![0, 1, 2]);
}

#[test]
fn targets_are_spaced_and_not_ours() {
    for seed in 1..20 {
        let state = start(seed);
        let zones = Zones::of(&state);
        let goals = targets(&state, &zones, &Settings::DEFAULT, 8);
        assert!(!goals.is_empty(), "seed {seed}");
        for (i, &a) in goals.iter().enumerate() {
            assert!(!state.owned(0).test(a), "seed {seed}");
            for &b in &goals[i + 1..] {
                let close = state.map().distance(a, b) < Settings::DEFAULT.spacing;
                let chokes = zones.middle_chokes();
                assert!(
                    !close || chokes.test(a) || chokes.test(b),
                    "seed {seed}: {a} and {b}"
                );
            }
        }
    }
}

#[test]
fn every_command_of_a_game_is_accepted_and_trainings_head_out() {
    // Both sides play the assignment for 12 game turns on many maps; every
    // command must be accepted, and both sides must grow.
    for seed in 1..30 {
        let mut state = start(seed);
        for _ in 0..24 {
            if state.outcome().is_some() {
                break;
            }
            let commands = turn(&state, &Settings::DEFAULT);
            let mut check = state.clone();
            for &command in &commands {
                assert!(check.apply(command), "seed {seed}: {command:?}");
            }
            state.play_turn(&commands);
        }
        for (seat, &hq) in HQ.iter().enumerate() {
            assert!(state.owned(seat).count() > 10, "seed {seed}, seat {seat}");
            assert!(state.owned(seat).test(hq));
        }
    }
}

#[test]
fn the_first_training_goes_toward_the_middle() {
    for seed in 1..10 {
        let state = start(seed);
        let settings = Settings {
            buy: Purchase::Affordable,
            ..Settings::DEFAULT
        };
        let commands = turn(&state, &settings);
        let Some(Command::Train { x, y, .. }) = commands.first().copied() else {
            panic!("seed {seed}: {commands:?}");
        };
        let cell = usize::from(y) * SIZE + usize::from(x);
        let neighbours = Bits::cell(HQ[0]).neighbours() & state.map().valid;
        let best = neighbours
            .iter()
            .map(|c| state.map().distance(c, HQ[1]))
            .min()
            .unwrap_or(u8::MAX);
        assert!(state.map().distance(cell, HQ[1]) <= best + 1, "seed {seed}");
    }
}
