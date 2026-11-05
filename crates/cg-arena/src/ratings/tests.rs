use super::*;

fn matchup(a: usize, b: usize, score_a: f64, games: u32) -> MatchupResult {
    MatchupResult {
        a,
        b,
        score_a,
        games,
    }
}

#[test]
fn a_75_percent_score_is_about_190_elo() {
    let ratings = elo_ratings(2, &[matchup(0, 1, 7500.0, 10_000)], 1);
    assert_eq!(ratings[1], 0.0);
    // 400 * log10(3) = 190.8; the virtual draw barely moves it.
    assert!((ratings[0] - 190.8).abs() < 0.5, "{ratings:?}");
}

#[test]
fn even_results_give_equal_ratings() {
    let ratings = elo_ratings(2, &[matchup(0, 1, 50.0, 100)], 0);
    assert!(ratings[1].abs() < 1e-6, "{ratings:?}");
}

#[test]
fn transitive_results_line_up() {
    // A beats B 75%, B beats C 75%, A beats C 90%.
    let results = [
        matchup(0, 1, 750.0, 1000),
        matchup(1, 2, 750.0, 1000),
        matchup(0, 2, 900.0, 1000),
    ];
    let ratings = elo_ratings(3, &results, 2);
    assert_eq!(ratings[2], 0.0);
    assert!(
        ratings[0] > ratings[1] && ratings[1] > ratings[2],
        "{ratings:?}"
    );
    assert!((ratings[1] - 190.0).abs() < 40.0, "{ratings:?}");
}

#[test]
fn a_sweep_still_gets_a_finite_rating() {
    let ratings = elo_ratings(2, &[matchup(0, 1, 20.0, 20)], 1);
    assert!(ratings[0].is_finite() && ratings[0] > 400.0, "{ratings:?}");
}

#[test]
fn margins_match_the_binomial_error_of_one_matchup() {
    // 1,000 games at 50%: a score error of 1.96 * 0.5 / sqrt(1001), which
    // is about 43 Elo at 50%.
    let results = [matchup(0, 1, 500.0, 1000)];
    let ratings = elo_ratings(2, &results, 1);
    let margins = elo_margins(2, &results, 1, &ratings);
    assert_eq!(margins[1], 0.0);
    let expected = 1.96 * 0.5 / 1001f64.sqrt() * 400.0 / (std::f64::consts::LN_10 * 0.25);
    assert!(
        (margins[0] - expected).abs() < 0.1,
        "{margins:?} vs {expected}"
    );
}

#[test]
fn margins_grow_with_distance_from_the_anchor() {
    // A chain 0 - 1 - 2: bot 2 is only linked to the anchor through bot 1.
    let results = [matchup(0, 1, 500.0, 1000), matchup(1, 2, 500.0, 1000)];
    let ratings = elo_ratings(3, &results, 0);
    let margins = elo_margins(3, &results, 0, &ratings);
    assert!(margins[2] > margins[1] * 1.3, "{margins:?}");
}

#[test]
fn bots_not_linked_to_the_anchor_have_infinite_margins() {
    // 0 - 1 played; 2 played nobody; 3 - 4 played each other only.
    let results = [matchup(0, 1, 50.0, 100), matchup(3, 4, 50.0, 100)];
    let ratings = elo_ratings(5, &results, 0);
    let margins = elo_margins(5, &results, 0, &ratings);
    assert!(margins[1].is_finite(), "{margins:?}");
    assert!(margins[2..].iter().all(|m| m.is_infinite()), "{margins:?}");
}

#[test]
fn adding_games_to_the_covariance_is_rating_them_again() {
    let results = [
        matchup(0, 1, 300.0, 400),
        matchup(1, 2, 250.0, 400),
        matchup(2, 3, 100.0, 200),
    ];
    let ratings = elo_ratings(5, &results, 0);
    let mut added = Covariance::new(5, &results, 0, &ratings);
    // The game of 1 and 3, and one of 3 and the anchor, played later.
    added.add(1, 3, games_information(101.0, ratings[1], ratings[3]));
    added.add(3, 0, games_information(51.0, ratings[3], ratings[0]));
    // A bot without games stays without a rating.
    added.add(4, 1, games_information(1000.0, ratings[4], ratings[1]));
    let mut all = results.to_vec();
    all.push(matchup(1, 3, 50.0, 100));
    all.push(matchup(3, 0, 25.0, 50));
    let rebuilt = Covariance::new(5, &all, 0, &ratings);
    for bot in 0..5 {
        let (x, y) = (added.margin(bot), rebuilt.margin(bot));
        assert!(x == y || (x - y).abs() < 1e-9, "bot {bot}: {x} {y}");
    }
    assert_eq!(added.margin(0), 0.0);
    assert!(added.margin(4).is_infinite());
    // The drop it predicts is what adding gives.
    let before = Covariance::new(5, &results, 0, &ratings);
    let information = games_information(101.0, ratings[1], ratings[3]);
    let drop = before.squared_margin_drop(1, 3, information, &[(2, 1.0), (3, 0.5)]);
    let mut after = before.clone();
    after.add(1, 3, information);
    let squared = |c: &Covariance, bot: usize| c.margin(bot).powi(2);
    let expected = (squared(&before, 2) - squared(&after, 2))
        + 0.5 * (squared(&before, 3) - squared(&after, 3));
    assert!((drop - expected).abs() < 1e-6, "{drop} {expected}");
    assert!(drop > 0.0);
}
