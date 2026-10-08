use super::*;

fn settings() -> HeadStart {
    HeadStart {
        iterations: 100,
        first_iterations: 1_000,
        boost: 20,
        moves: 4,
        exploration: 0.5,
    }
}

#[test]
fn the_head_start_covers_the_first_moves_only() {
    let head_start = settings();
    // The first move: the first turn's budget, or the boost if larger.
    assert_eq!(head_start.budget(1, false), 1_000);
    assert_eq!(head_start.budget(1, true), 2_000);
    assert_eq!(head_start.budget(2, false), 100);
    assert_eq!(head_start.budget(4, true), 2_000);
    assert_eq!(head_start.budget(5, true), 100);
    let small = HeadStart {
        boost: 5,
        ..head_start
    };
    assert_eq!(small.budget(1, true), 1_000);
}

#[test]
fn games_count_and_do_not_depend_on_threads() {
    let weights =
        crate::selfplay::read_policy_weights(include_str!("../../../bots/mcts/src/weights.rs"))
            .unwrap();
    let policy: &'static PlayoutPolicy =
        Box::leak(Box::new(PlayoutPolicy::new(weights).for_plies(16)));
    let head_start = HeadStart {
        iterations: 50,
        first_iterations: 200,
        boost: 4,
        moves: 2,
        exploration: 0.5,
    };
    let results = head_start.play(policy, 4, 7, 1);
    assert_eq!(results.wins + results.draws + results.losses, 8);
    assert_eq!(results.pairs.iter().sum::<u32>(), 4);
    assert_eq!(head_start.play(policy, 4, 7, 3), results);
}
