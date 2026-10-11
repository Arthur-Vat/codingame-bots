//! Tournament results from the first bot's point of view.

use std::collections::HashMap;
use std::fmt;

use crate::runner::EndReason;
use crate::tournament::GameRecord;

/// How often a bot lost a game by its own fault.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Faults {
    pub timeouts: u32,
    pub crashes: u32,
    pub invalid: u32,
}

impl Faults {
    pub fn total(&self) -> u32 {
        self.timeouts + self.crashes + self.invalid
    }

    /// Whether these faults, over `games` games, stay within what a fault
    /// check tolerates: no crash, no invalid answer, and at most
    /// [`allowed_timeouts`] timeouts.
    pub fn within(&self, games: u32, max_timeout_rate: f64) -> bool {
        self.crashes == 0
            && self.invalid == 0
            && self.timeouts <= allowed_timeouts(games, max_timeout_rate)
    }
}

/// The timeouts tolerated in `games` games at `max_timeout_rate`, a
/// fraction of the games, rounded down: 10 in 1,000 games at 0.01.
pub fn allowed_timeouts(games: u32, max_timeout_rate: f64) -> u32 {
    // The small margin keeps 0.01 × 1,000 at 10 despite rounding.
    (f64::from(games) * max_timeout_rate + 1e-9).floor() as u32
}

/// An Elo difference with its 95% confidence interval.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EloEstimate {
    pub elo: f64,
    pub low: f64,
    pub high: f64,
}

/// Running totals of a tournament, from the first bot's point of view.
#[derive(Clone, Debug)]
pub struct Summary {
    /// The first bot, then its opponent.
    pub names: [String; 2],
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
    /// Faults by bot, in `names` order.
    pub faults: [Faults; 2],
    /// Games the arena stopped; they count as draws.
    pub aborted: u32,
    /// Slowest answer seen, by bot, in milliseconds.
    pub max_answer_ms: [f64; 2],
    /// Every answer after a game's first, by bot, in milliseconds.
    answer_ms: [Vec<f32>; 2],
    /// Complete pairs by the first bot's points in the pair: 0, ½, 1, 1½, 2.
    pub pairs: [u32; 5],
    /// Points of pairs with one game played so far.
    pending: HashMap<u32, f64>,
}

impl Summary {
    pub fn new(names: [String; 2]) -> Self {
        Summary {
            names,
            wins: 0,
            draws: 0,
            losses: 0,
            faults: [Faults::default(); 2],
            aborted: 0,
            max_answer_ms: [0.0; 2],
            answer_ms: Default::default(),
            pairs: [0; 5],
            pending: HashMap::new(),
        }
    }

    pub fn add(&mut self, record: &GameRecord) {
        // The seat of the first bot in this game, and the bot in each seat.
        let first_seat = usize::from(record.swapped);
        let bot_in_seat = |seat: usize| usize::from(seat != first_seat);
        let game = &record.game;

        let points = match game.winner {
            Some(seat) if seat == first_seat => {
                self.wins += 1;
                1.0
            }
            Some(_) => {
                self.losses += 1;
                0.0
            }
            None => {
                self.draws += 1;
                0.5
            }
        };
        match &game.end {
            EndReason::Timeout { seat, .. } => self.faults[bot_in_seat(*seat)].timeouts += 1,
            EndReason::Crash { seat, .. } => self.faults[bot_in_seat(*seat)].crashes += 1,
            EndReason::Invalid { seat, .. } => self.faults[bot_in_seat(*seat)].invalid += 1,
            // The arena never resigns; if a record says so, it is not a fault.
            EndReason::Aborted { .. } | EndReason::Resigned { .. } => self.aborted += 1,
            EndReason::Finished => {}
        }
        for seat in 0..2 {
            let bot = bot_in_seat(seat);
            self.max_answer_ms[bot] = self.max_answer_ms[bot].max(game.max_answer_ms[seat]);
            self.answer_ms[bot].extend_from_slice(&game.later_answer_ms[seat]);
        }
        if let Some(other) = self.pending.remove(&record.pair) {
            let pair_points = other + points;
            self.pairs[(pair_points * 2.0).round() as usize] += 1;
        } else {
            self.pending.insert(record.pair, points);
        }
    }

    pub fn games(&self) -> u32 {
        self.wins + self.draws + self.losses
    }

    /// The first bot's average points per game, from 0 to 1.
    pub fn score(&self) -> Option<f64> {
        let games = self.games();
        (games > 0).then(|| (f64::from(self.wins) + 0.5 * f64::from(self.draws)) / f64::from(games))
    }

    /// The time below which a fraction `share` of a bot's answers after
    /// the first came (0.99 for the 99th percentile), in milliseconds;
    /// `None` without such answers.
    pub fn answer_percentile(&self, bot: usize, share: f64) -> Option<f32> {
        let mut times = self.answer_ms[bot].clone();
        if times.is_empty() {
            return None;
        }
        times.sort_by(f32::total_cmp);
        let rank = (share * times.len() as f64).ceil() as usize;
        Some(times[rank.clamp(1, times.len()) - 1])
    }

    /// Faults of both bots.
    pub fn total_faults(&self) -> u32 {
        self.faults[0].total() + self.faults[1].total()
    }

    /// The first bot's Elo advantage, from complete pairs. `None` until at
    /// least two pairs are complete, or when one bot scored every point.
    pub fn elo(&self) -> Option<EloEstimate> {
        let count: u32 = self.pairs.iter().sum();
        if count < 2 {
            return None;
        }
        let n = f64::from(count);
        // Per-pair score in [0, 1]: 0, 0.25, 0.5, 0.75 or 1.
        let value = |k: usize| k as f64 / 4.0;
        let mean = (0..5)
            .map(|k| value(k) * f64::from(self.pairs[k]))
            .sum::<f64>()
            / n;
        if mean <= 0.0 || mean >= 1.0 {
            return None;
        }
        let variance = (0..5)
            .map(|k| f64::from(self.pairs[k]) * (value(k) - mean).powi(2))
            .sum::<f64>()
            / n;
        let margin = 1.96 * (variance / n).sqrt();
        let bound = |score: f64| elo_from_score(score.clamp(1e-6, 1.0 - 1e-6));
        Some(EloEstimate {
            elo: elo_from_score(mean),
            low: bound(mean - margin),
            high: bound(mean + margin),
        })
    }
}

impl Summary {
    /// Whether the first bot is clearly weaker: the 95% interval of its Elo
    /// advantage lies entirely below 0, or it lost every game.
    pub fn is_clearly_worse(&self) -> bool {
        match self.elo() {
            Some(estimate) => estimate.high < 0.0,
            None => self.score() == Some(0.0),
        }
    }
}

/// The Elo difference that gives an expected score of `score`.
pub fn elo_from_score(score: f64) -> f64 {
    -400.0 * (1.0 / score - 1.0).log10()
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [first, second] = &self.names;
        let pairs: u32 = self.pairs.iter().sum();
        writeln!(
            f,
            "{first} vs {second}: {} games, {pairs} complete pairs",
            self.games()
        )?;
        let score = self
            .score()
            .map_or("n/a".to_string(), |s| format!("{:.1}%", 100.0 * s));
        writeln!(
            f,
            "  {first}: {} wins, {} draws, {} losses (score {score})",
            self.wins, self.draws, self.losses
        )?;
        match self.elo() {
            Some(e) => writeln!(
                f,
                "  Elo of {first} over {second}: {:+.1} [{:+.1}, {:+.1}] (95%, from pairs)",
                e.elo, e.low, e.high
            )?,
            None => writeln!(f, "  Elo of {first} over {second}: n/a")?,
        }
        writeln!(
            f,
            "  pairs by {first}'s points (0, 1/2, 1, 3/2, 2): {:?}",
            self.pairs
        )?;
        for (name, faults) in self.names.iter().zip(&self.faults) {
            writeln!(
                f,
                "  {name} faults: {} timeouts, {} crashes, {} invalid answers",
                faults.timeouts, faults.crashes, faults.invalid
            )?;
        }
        if self.aborted > 0 {
            writeln!(f, "  aborted by the arena: {}", self.aborted)?;
        }
        let percentiles = |bot: usize| -> Option<String> {
            let at = |share| self.answer_percentile(bot, share);
            Some(format!(
                "{} {:.1}, {:.1}, {:.1} ms",
                self.names[bot],
                at(0.5)?,
                at(0.99)?,
                at(0.999)?
            ))
        };
        if let (Some(first), Some(second)) = (percentiles(0), percentiles(1)) {
            writeln!(
                f,
                "  answers after the first (median, 99%, 99.9%): {first}; {second}"
            )?;
        }
        write!(
            f,
            "  slowest answer: {first} {:.1} ms, {second} {:.1} ms",
            self.max_answer_ms[0], self.max_answer_ms[1]
        )
    }
}

#[cfg(test)]
mod tests;
