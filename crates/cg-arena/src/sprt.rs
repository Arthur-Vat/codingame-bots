//! Sequential probability ratio test on game pairs.
//!
//! The test decides between two hypotheses about the candidate's Elo
//! advantage over its baseline: H0, "it is at most `elo0`", and H1, "it is at
//! least `elo1`". It plays pairs until the log-likelihood ratio (LLR) of H1
//! against H0 leaves the interval set by the error rates `alpha` (accepting
//! H1 when H0 is true) and `beta` (accepting H0 when H1 is true).
//!
//! Results are counted per seat-swapped pair, as a pentanomial distribution
//! of the candidate's points in the pair (0, ½, 1, 1½ or 2), and the LLR
//! uses the normal approximation of the generalized SPRT, as chess engine
//! testing does. Elo here is logistic Elo: an advantage of `e` gives an
//! expected score of `1 / (1 + 10^(-e / 400))`.
//!
//! The approximation needs an estimate of the variance of pair results,
//! which is unreliable over a handful of pairs: one pair won twice would
//! accept the candidate at once. So no verdict is given before
//! [`MIN_PAIRS`] pairs. Simulations of the whole test (see the tests) keep
//! both error rates near their targets with this guard, and far above them
//! without it (about 30% instead of 5%).
//!
//! To make a verdict reproducible, pairs are taken in order: the test only
//! looks at pairs 0..n once all of them are complete, whatever order the
//! games finish in.

use std::collections::BTreeMap;
use std::fmt;

use crate::tournament::GameRecord;

/// Pairs played before the test may decide.
pub const MIN_PAIRS: u32 = 30;

/// The hypotheses and error rates of a test.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SprtSettings {
    pub elo0: f64,
    pub elo1: f64,
    pub alpha: f64,
    pub beta: f64,
}

/// The decision of a test so far.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Not enough evidence yet.
    Continue,
    /// H0: the candidate is not stronger by `elo1`.
    Rejected,
    /// H1: the candidate is stronger.
    Accepted,
}

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Verdict::Continue => "undecided",
            Verdict::Rejected => "rejected",
            Verdict::Accepted => "accepted",
        })
    }
}

/// Expected score for an Elo advantage.
pub fn score_from_elo(elo: f64) -> f64 {
    1.0 / (1.0 + 10f64.powf(-elo / 400.0))
}

impl SprtSettings {
    /// Checks that the settings make sense.
    pub fn validate(&self) -> Result<(), String> {
        let rate = |value: f64| value > 0.0 && value < 0.5;
        if !(rate(self.alpha) && rate(self.beta)) {
            return Err("alpha and beta must be between 0 and 0.5".to_string());
        }
        if !(self.elo0.is_finite() && self.elo1.is_finite() && self.elo0 < self.elo1) {
            return Err("elo0 must be below elo1".to_string());
        }
        Ok(())
    }

    /// The LLR bounds: H0 is accepted below the first, H1 above the second.
    pub fn bounds(&self) -> (f64, f64) {
        (
            (self.beta / (1.0 - self.alpha)).ln(),
            ((1.0 - self.beta) / self.alpha).ln(),
        )
    }

    /// The LLR of H1 against H0 for these pair counts, indexed by the
    /// candidate's points in the pair times two (0 to 4).
    pub fn llr(&self, pairs: &[u32; 5]) -> f64 {
        let total: u32 = pairs.iter().sum();
        if total == 0 {
            return 0.0;
        }
        // An empty cell gets a tiny count, so a sweep has a finite variance.
        let counts = pairs.map(|count| if count == 0 { 1e-3 } else { f64::from(count) });
        let n: f64 = counts.iter().sum();
        let value = |k: usize| k as f64 / 4.0;
        let mean = (0..5).map(|k| counts[k] * value(k)).sum::<f64>() / n;
        let variance = (0..5)
            .map(|k| counts[k] * (value(k) - mean).powi(2))
            .sum::<f64>()
            / n;
        let (s0, s1) = (score_from_elo(self.elo0), score_from_elo(self.elo1));
        f64::from(total) * (s1 - s0) * (2.0 * mean - s0 - s1) / (2.0 * variance)
    }

    /// The verdict for these pair counts: always [`Verdict::Continue`]
    /// before [`MIN_PAIRS`] pairs.
    pub fn verdict(&self, pairs: &[u32; 5]) -> Verdict {
        if pairs.iter().sum::<u32>() < MIN_PAIRS {
            return Verdict::Continue;
        }
        let llr = self.llr(pairs);
        let (lower, upper) = self.bounds();
        if llr >= upper {
            Verdict::Accepted
        } else if llr <= lower {
            Verdict::Rejected
        } else {
            Verdict::Continue
        }
    }
}

/// A test running over a tournament's games, the candidate being the
/// tournament's first bot.
#[derive(Clone, Debug)]
pub struct SequentialTest {
    pub settings: SprtSettings,
    /// Complete pairs counted so far, by the candidate's points times two.
    pub pairs: [u32; 5],
    verdict: Verdict,
    /// The next pair the test is waiting for.
    next_pair: u32,
    /// The candidate's points in the first finished game of each pair.
    half_done: BTreeMap<u32, f64>,
    /// The candidate's points in complete pairs not counted yet.
    done: BTreeMap<u32, f64>,
}

impl SequentialTest {
    pub fn new(settings: SprtSettings) -> Self {
        SequentialTest {
            settings,
            pairs: [0; 5],
            verdict: Verdict::Continue,
            next_pair: 0,
            half_done: BTreeMap::new(),
            done: BTreeMap::new(),
        }
    }

    pub fn verdict(&self) -> Verdict {
        self.verdict
    }

    /// Complete pairs counted.
    pub fn counted_pairs(&self) -> u32 {
        self.next_pair
    }

    pub fn llr(&self) -> f64 {
        self.settings.llr(&self.pairs)
    }

    /// Adds a game; returns the verdict, which stays fixed once decided.
    pub fn add(&mut self, record: &GameRecord) -> Verdict {
        if self.verdict != Verdict::Continue {
            return self.verdict;
        }
        let first_seat = usize::from(record.swapped);
        let points = match record.game.winner {
            Some(seat) if seat == first_seat => 1.0,
            Some(_) => 0.0,
            None => 0.5,
        };
        match self.half_done.remove(&record.pair) {
            Some(other) => {
                self.done.insert(record.pair, other + points);
            }
            None => {
                self.half_done.insert(record.pair, points);
            }
        }
        while let Some(points) = self.done.remove(&self.next_pair) {
            self.pairs[(points * 2.0).round() as usize] += 1;
            self.next_pair += 1;
            self.verdict = self.settings.verdict(&self.pairs);
            if self.verdict != Verdict::Continue {
                break;
            }
        }
        self.verdict
    }
}

#[cfg(test)]
mod tests;
