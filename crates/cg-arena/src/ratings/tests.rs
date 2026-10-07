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
