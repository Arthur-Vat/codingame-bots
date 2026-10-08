use super::*;

#[test]
fn elo_follows_the_pairs_scores() {
    let even = Results {
        pairs: [0, 0, 10, 0, 0],
        ..Results::default()
    };
    assert_eq!(
        even.elo(),
        Some(Elo {
            elo: 0.0,
            low: 0.0,
            high: 0.0
        })
    );
    let ahead = Results {
        pairs: [0, 0, 5, 5, 0],
        ..Results::default()
    };
    let estimate = ahead.elo().unwrap();
    // A score of 0.625: -400 log10(1 / 0.625 - 1).
    assert!((estimate.elo - 88.74).abs() < 0.01, "{estimate:?}");
    assert!(estimate.low < estimate.elo && estimate.elo < estimate.high);
    let sweep = Results {
        pairs: [0, 0, 0, 0, 4],
        ..Results::default()
    };
    assert_eq!(sweep.elo(), None);
}

#[test]
fn duels_count_every_game_and_do_not_depend_on_threads() {
    let mut rng = Rng::new(31);
    let parameters: Vec<f32> = (0..ValueNetwork::PARAMETERS)
        .map(|_| ((rng.unit() * 2.0 - 1.0) * 0.3) as f32)
        .collect();
    let network: &'static ValueNetwork = Box::leak(Box::new(
        ValueNetwork::from_parameters(&parameters).unwrap(),
    ));
    let weights =
        crate::selfplay::read_policy_weights(include_str!("../../../bots/mcts/src/weights.rs"))
            .unwrap();
    let policy: &'static PlayoutPolicy =
        Box::leak(Box::new(PlayoutPolicy::new(weights).for_plies(16)));
    let duel = Duel {
        iterations: 200,
        opening_plies: 4,
        exploration: 0.5,
        network_exploration: 0.3,
    };
    let results = duel.play(network, policy, 6, 9, 1);
    assert_eq!(results.wins + results.draws + results.losses, 12);
    assert_eq!(results.pairs.iter().sum::<u32>(), 6);
    let points: u32 = results
        .pairs
        .iter()
        .enumerate()
        .map(|(half_points, &count)| half_points as u32 * count)
        .sum();
    assert_eq!(points, 2 * results.wins + results.draws);
    assert_eq!(duel.play(network, policy, 6, 9, 3), results);
}
