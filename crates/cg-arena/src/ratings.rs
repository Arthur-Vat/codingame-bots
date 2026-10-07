//! Elo ratings from the results of many matchups (Bradley-Terry model,
//! maximum likelihood).
//!
//! Each bot has a strength `γ`; the expected score of `a` against `b` is
//! `γa / (γa + γb)`, and its Elo is `400 · log10(γ)`. The strengths that best
//! explain the results are found with the classic minorization-maximization
//! iteration. Every matchup gets one extra virtual draw, so a bot that won
//! or lost everything still gets a finite rating.
//!
//! Confidence intervals come from the Fisher information of the same model,
//! treating games as independent. Draws and seat-swapped pairs make real
//! results a little less noisy than that, so the intervals are slightly
//! conservative.

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

/// Half-widths of the 95% confidence intervals of `ratings` (from
/// [`elo_ratings`] with the same arguments), in Elo, relative to the anchor:
/// 0 for the anchor, infinite for a bot with no path of games to it.
pub fn elo_margins(
    count: usize,
    results: &[MatchupResult],
    anchor: usize,
    ratings: &[f64],
) -> Vec<f64> {
    let per_elo = std::f64::consts::LN_10 / 400.0;
    // Bots linked to the anchor by a chain of matchups; the others have no
    // rating relative to it.
    let mut linked = vec![false; count];
    linked[anchor] = true;
    let mut grew = true;
    while grew {
        grew = false;
        for result in results {
            if linked[result.a] != linked[result.b] {
                linked[result.a] = true;
                linked[result.b] = true;
                grew = true;
            }
        }
    }

    // Fisher information of the natural strengths ln γ of the linked bots
    // other than the anchor, which is fixed.
    let others: Vec<usize> = (0..count)
        .filter(|&bot| bot != anchor && linked[bot])
        .collect();
    let index = |bot: usize| others.iter().position(|&other| other == bot);
    let mut information = vec![vec![0.0; others.len()]; others.len()];
    for result in results.iter().filter(|result| linked[result.a]) {
        let n = f64::from(result.games) + 1.0;
        let p = 1.0 / (1.0 + ((ratings[result.b] - ratings[result.a]) * per_elo).exp());
        let weight = n * p * (1.0 - p);
        let (a, b) = (index(result.a), index(result.b));
        for (i, j) in [(a, b), (b, a)] {
            if let Some(i) = i {
                information[i][i] += weight;
                if let Some(j) = j {
                    information[i][j] -= weight;
                }
            }
        }
    }
    let covariance = invert(information);
    (0..count)
        .map(|bot| {
            if bot == anchor {
                return 0.0;
            }
            match (index(bot), &covariance) {
                (Some(i), Some(covariance)) if covariance[i][i] > 0.0 => {
                    1.96 * covariance[i][i].sqrt() / per_elo
                }
                _ => f64::INFINITY,
            }
        })
        .collect()
}

/// The inverse of a square matrix by Gauss-Jordan elimination, or `None`
/// when it is singular.
fn invert(mut matrix: Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
    let n = matrix.len();
    let mut inverse: Vec<Vec<f64>> = (0..n)
        .map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect())
        .collect();
    for column in 0..n {
        let pivot = (column..n)
            .max_by(|&x, &y| matrix[x][column].abs().total_cmp(&matrix[y][column].abs()))?;
        if matrix[pivot][column].abs() < 1e-12 {
            return None;
        }
        matrix.swap(column, pivot);
        inverse.swap(column, pivot);
        let scale = matrix[column][column];
        for j in 0..n {
            matrix[column][j] /= scale;
            inverse[column][j] /= scale;
        }
        for row in 0..n {
            if row != column {
                let factor = matrix[row][column];
                for j in 0..n {
                    matrix[row][j] -= factor * matrix[column][j];
                    inverse[row][j] -= factor * inverse[column][j];
                }
            }
        }
    }
    Some(inverse)
}

#[cfg(test)]
mod tests;
