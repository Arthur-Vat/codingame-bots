use ice_and_fire_engine::State;

use super::*;
use crate::econ;
use crate::middle;
use crate::portfolio::PORTFOLIO;
use crate::test_util::start;

fn opening(width: usize) -> Planned {
    let mut sets = vec![Params::V003];
    sets.extend_from_slice(&PORTFOLIO[..4]);
    Planned::new(Planner::Beam { width, replan: 2 }, sets, Params::V003)
}

/// Plays our opening in `seat` against `econ` (the model) or another
/// opponent until the middle game; checks every command on the way.
fn play(
    seed: u64,
    seat: usize,
    opponent: fn(&State) -> Vec<ice_and_fire_engine::Command>,
) -> (State, Planned) {
    let mut state = start(seed);
    let mut ours = opening(3);
    while state.outcome().is_none() && !middle::reached(&state) {
        let commands = if state.to_act() == seat {
            let commands = ours.turn(&state, None);
            let mut scratch = state.clone();
            for &command in &commands {
                assert!(scratch.apply(command), "seed {seed}: {command:?} refused");
            }
            commands
        } else {
            opponent(&state)
        };
        state.play_turn(&commands);
        assert!(state.game_turn() < 25, "seed {seed}: no middle game");
    }
    (state, ours)
}

#[test]
fn against_the_model_the_first_plan_is_played_to_the_end() {
    for seed in 1..=4 {
        for seat in 0..2 {
            let (_, ours) = play(seed, seat, econ::rules_turn);
            // The prediction is exact: a new plan only when the first runs
            // out.
            assert!(
                ours.plans <= 2,
                "seed {seed}, seat {seat}: {} plans",
                ours.plans
            );
        }
    }
}

#[test]
fn against_another_opponent_every_command_is_accepted() {
    // An opponent that never moves: the plan is made again when our side
    // differs from the prediction.
    for seed in 5..=7 {
        play(seed, 0, |_| Vec::new());
    }
}

#[test]
fn the_plan_scores_at_least_the_default_opening_on_its_own_model() {
    for seed in 1..=4 {
        let state = start(seed);
        let sets = [Params::V003];
        let ctx = crate::plan::Context {
            sets: &sets,
            model: Params::V003,
            rest: Step {
                weights: 0,
                buy: Buy::Payback,
            },
            deadline: None,
            settings: crate::score::Settings::DEFAULT,
        };
        let plan = crate::plan::beam(&state, &ctx, 20);
        let full = ctx.settings;
        assert!(
            ctx.value(&state, &plan, full) >= ctx.value(&state, &[], full),
            "seed {seed}"
        );
    }
}
