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

/// v003's weights and the portfolio's, chosen by rollouts, as the bot plays.
fn chooser() -> Planned {
    let mut sets = vec![Params::V003];
    sets.extend_from_slice(&PORTFOLIO);
    Planned::new(Planner::Choose, sets, Params::V003)
}

#[test]
fn against_the_model_the_portfolio_s_choice_is_made_once() {
    for seed in 1..=6 {
        for seat in 0..2 {
            let mut state = start(seed);
            let mut ours = chooser();
            let mut turns: usize = 0;
            while state.outcome().is_none() {
                if state.to_act() == seat {
                    if !ours.playing(&state) {
                        break;
                    }
                    turns += 1;
                    let commands = ours.turn(&state, None);
                    state.play_turn(&commands);
                } else {
                    let commands = econ::rules_turn(&state);
                    state.play_turn(&commands);
                }
                assert!(
                    state.game_turn() < 25,
                    "seed {seed}: the opening never ends"
                );
            }
            assert_eq!(ours.plans, 1, "seed {seed}, seat {seat}");
            assert!(turns >= 3, "seed {seed}, seat {seat}: {turns} turns");
        }
    }
}

#[test]
fn once_the_middle_game_has_begun_the_opening_never_comes_back() {
    let mut ours = chooser();
    assert!(ours.playing(&start(1)));
    // Red holds a row to (3, 0), blue the end of the row from (6, 0): 30 gold
    // pay the chain of three trainings to (6, 0).
    let column = "###########X";
    let near = crate::test_util::Position {
        rows: [
            "OOOO..XXXXXX",
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
        ],
        gold: [30, 0],
        buildings: &[],
        units: &[],
    }
    .state();
    assert!(middle::reached(&near));
    assert!(!ours.playing(&near));
    // A later turn short of gold is still the middle game.
    assert!(!ours.playing(&start(1)));
}

fn solo_opening() -> Planned {
    let sets = crate::solo::SETS.to_vec();
    Planned::new(
        Planner::Solo {
            turns: 3,
            width: 2,
            objective: crate::solo::Objective::DEFAULT,
        },
        sets,
        Params::V003,
    )
}

#[test]
fn the_solo_opening_follows_its_plan_and_plays_accepted_commands() {
    for seed in 1..=3 {
        for seat in 0..2 {
            let mut state = start(seed);
            let mut ours = solo_opening();
            let mut turns: usize = 0;
            while state.outcome().is_none() && ours.playing(&state) {
                let commands = if state.to_act() == seat {
                    turns += 1;
                    let commands = ours.turn(&state, None);
                    let mut scratch = state.clone();
                    for &command in &commands {
                        assert!(scratch.apply(command), "seed {seed}: {command:?} refused");
                    }
                    commands
                } else {
                    Vec::new()
                };
                state.play_turn(&commands);
                assert!(state.game_turn() < 40, "seed {seed}: no middle game");
            }
            // Against a passing opponent the prediction is exact: a new plan
            // only when one runs out, every 3 turns.
            assert_eq!(
                ours.plans as usize,
                turns.div_ceil(3),
                "seed {seed}, seat {seat}"
            );
        }
    }
}
