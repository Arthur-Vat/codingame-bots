use super::*;
use crate::middle;
use crate::test_util::start;

#[test]
fn a_rollout_stops_at_the_middle_game() {
    for seed in 1..=20 {
        let state = start(seed);
        let end = play(&state, &Params::V003, |_, s| econ::rules_turn(s));
        assert!(end.outcome().is_none(), "seed {seed}");
        assert!(middle::reached(&end), "seed {seed}: no middle game");
        assert!(end.game_turn() < CAP, "seed {seed}");
    }
}

#[test]
fn our_turns_are_counted_from_zero() {
    let state = start(3);
    let mut seen = Vec::new();
    play(&state, &Params::V003, |i, s| {
        seen.push(i);
        econ::rules_turn(s)
    });
    let expected: Vec<usize> = (0..seen.len()).collect();
    assert_eq!(seen, expected);
    assert!(seen.len() >= 3);
}

#[test]
fn the_chooser_picks_the_set_whose_rollout_scores_best() {
    let state = start(5);
    let sets = [
        Params::V003,
        Params {
            horizon: 11,
            ..Params::V003
        },
    ];
    let chosen = choose(&state, &sets, &Params::V003, Settings::DEFAULT, None);
    let values: Vec<i64> = sets
        .iter()
        .map(|p| value_of(&state, p, &Params::V003, Settings::DEFAULT))
        .collect();
    let best = values.iter().copied().max().unwrap_or_default();
    assert_eq!(values[chosen], best);
    assert_eq!(values.iter().position(|&v| v == best), Some(chosen));
}
