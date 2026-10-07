//! Elo ratings from the results of many matchups (Bradley-Terry model,
//! maximum likelihood).
//!
//! Each bot has a strength `γ`; the expected score of `a` against `b` is
//! `γa / (γa + γb)`, and its Elo is `400 · log10(γ)`. The strengths that best
//! explain the results are found with the classic minorization-maximization
//! iteration. Every matchup gets one extra virtual draw, so a bot that won
//! or lost everything still gets a finite rating.

/// The total result of one matchup.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MatchupResult {
    pub a: usize,
    pub b: usize,
    /// Points of `a`: wins plus half the draws.
    pub score_a: f64,
    pub games: u32,
}

/// Elo ratings of `count` bots, with bot `anchor` at 0.
///
/// # Panics
///
/// If a matchup names a bot outside `0..count`, or `anchor` does.
pub fn elo_ratings(count: usize, results: &[MatchupResult], anchor: usize) -> Vec<f64> {
    assert!(anchor < count, "anchor {anchor} is not one of {count} bots");
    // Points and games per pair of bots, with the virtual draw.
    let mut points = vec![0.0; count];
    let mut games = vec![vec![0.0; count]; count];
    for result in results {
        assert!(result.a < count && result.b < count && result.a != result.b);
        let n = f64::from(result.games) + 1.0;
        points[result.a] += result.score_a + 0.5;
        points[result.b] += f64::from(result.games) - result.score_a + 0.5;
        games[result.a][result.b] += n;
        games[result.b][result.a] += n;
    }

    let mut strength = vec![1.0; count];
    for _ in 0..10_000 {
        let mut largest_change: f64 = 0.0;
        for bot in 0..count {
            let denominator: f64 = (0..count)
                .filter(|&other| games[bot][other] > 0.0)
                .map(|other| games[bot][other] / (strength[bot] + strength[other]))
                .sum();
            if denominator == 0.0 {
                continue; // a bot that played nobody keeps its strength
            }
            let updated = points[bot] / denominator;
            largest_change = largest_change.max((updated / strength[bot]).ln().abs());
            strength[bot] = updated;
        }
        if largest_change < 1e-10 {
            break;
        }
    }
    let base = strength[anchor];
    strength
        .iter()
        .map(|s| 400.0 * (s / base).log10())
        .collect()
}

#[cfg(test)]
mod tests;
