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
    let covariance = Covariance::new(count, results, anchor, ratings);
    (0..count).map(|bot| covariance.margin(bot)).collect()
}

/// Natural strength `ln γ` per Elo.
const PER_ELO: f64 = std::f64::consts::LN_10 / 400.0;

/// The Fisher information of `games` games between bots rated `a` and `b`
/// (Elo), in natural strengths: `games · p · (1 − p)`, with `p` the expected
/// score.
pub(crate) fn games_information(games: f64, a: f64, b: f64) -> f64 {
    let p = 1.0 / (1.0 + ((b - a) * PER_ELO).exp());
    games * p * (1.0 - p)
}

/// The covariance of the natural strengths `ln γ` of the bots linked to the
/// anchor by a chain of matchups, other than the anchor itself, which is
/// fixed: the inverse of their Fisher information, every bot's rating held
/// at its estimate. Bots that share a link to the anchor share its
/// uncertainty, so their margins are read from the whole matrix.
#[derive(Clone, Debug)]
pub(crate) struct Covariance {
    /// The row of each bot in `matrix`: `None` for the anchor and for bots
    /// without a path of games to it.
    rows: Vec<Option<usize>>,
    anchor: usize,
    /// `None` when the information is singular.
    matrix: Option<Vec<Vec<f64>>>,
}

impl Covariance {
    /// The covariance after `results`, at `ratings` (Elo).
    pub(crate) fn new(
        count: usize,
        results: &[MatchupResult],
        anchor: usize,
        ratings: &[f64],
    ) -> Self {
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
        let mut rows = vec![None; count];
        let mut size = 0;
        for bot in (0..count).filter(|&bot| bot != anchor && linked[bot]) {
            rows[bot] = Some(size);
            size += 1;
        }
        let mut information = vec![vec![0.0; size]; size];
        for result in results.iter().filter(|result| linked[result.a]) {
            // Every matchup has one extra virtual draw, as in `elo_ratings`.
            let weight = games_information(
                f64::from(result.games) + 1.0,
                ratings[result.a],
                ratings[result.b],
            );
            let (a, b) = (rows[result.a], rows[result.b]);
            for (i, j) in [(a, b), (b, a)] {
                if let Some(i) = i {
                    information[i][i] += weight;
                    if let Some(j) = j {
                        information[i][j] -= weight;
                    }
                }
            }
        }
        Covariance {
            rows,
            anchor,
            matrix: invert(information),
        }
    }

    /// Whether `bot` has a rating relative to the anchor.
    pub(crate) fn is_linked(&self, bot: usize) -> bool {
        bot == self.anchor || (self.rows[bot].is_some() && self.matrix.is_some())
    }

    /// The half-width of `bot`'s 95% interval, in Elo: 0 for the anchor,
    /// infinite for a bot without a rating.
    pub(crate) fn margin(&self, bot: usize) -> f64 {
        if bot == self.anchor {
            return 0.0;
        }
        match (self.rows[bot], &self.matrix) {
            (Some(i), Some(matrix)) if matrix[i][i] > 0.0 => 1.96 * matrix[i][i].sqrt() / PER_ELO,
            _ => f64::INFINITY,
        }
    }

    /// The column `C·u` and the number `uᵀ·C·u` for a game between `a` and
    /// `b`, with `u` the difference of their unit vectors, or `None` when the
    /// game would tell nothing about the linked bots.
    fn direction(&self, a: usize, b: usize) -> Option<(Vec<f64>, f64)> {
        let matrix = self.matrix.as_ref()?;
        if a == b || !self.is_linked(a) || !self.is_linked(b) {
            return None;
        }
        let (ra, rb) = (self.rows[a], self.rows[b]);
        let column: Vec<f64> = matrix
            .iter()
            .map(|row| ra.map_or(0.0, |i| row[i]) - rb.map_or(0.0, |i| row[i]))
            .collect();
        let spread = ra.map_or(0.0, |i| column[i]) - rb.map_or(0.0, |i| column[i]);
        (spread > 0.0).then_some((column, spread))
    }

    /// How much games between `a` and `b` worth `information` (from
    /// [`games_information`]) would shrink the square of each margin of
    /// `bots`, in Elo², summed with the weights given.
    pub(crate) fn squared_margin_drop(
        &self,
        a: usize,
        b: usize,
        information: f64,
        bots: &[(usize, f64)],
    ) -> f64 {
        if information <= 0.0 || !information.is_finite() {
            return 0.0;
        }
        let Some((column, spread)) = self.direction(a, b) else {
            return 0.0;
        };
        let scale = (1.96 / PER_ELO).powi(2) / (1.0 / information + spread);
        bots.iter()
            .filter_map(|&(bot, weight)| self.rows[bot].map(|i| weight * column[i] * column[i]))
            .sum::<f64>()
            * scale
    }

    /// Adds games between `a` and `b` worth `information`: the covariance
    /// as if they had been played and scored as expected (Sherman-Morrison).
    /// Games with a bot without a rating change nothing.
    pub(crate) fn add(&mut self, a: usize, b: usize, information: f64) {
        if information <= 0.0 || !information.is_finite() {
            return;
        }
        let Some((column, spread)) = self.direction(a, b) else {
            return;
        };
        let scale = 1.0 / (1.0 / information + spread);
        if let Some(matrix) = self.matrix.as_mut() {
            for (i, row) in matrix.iter_mut().enumerate() {
                for (j, cell) in row.iter_mut().enumerate() {
                    *cell -= scale * column[i] * column[j];
                }
            }
        }
    }
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
