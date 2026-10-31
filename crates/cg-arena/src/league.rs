//! The incremental league ([ADR 0038](../../../docs/adr/0038-incremental-league.md)):
//! the results file of stored pairings, the choice of a new release's
//! opponents, the seeds of its pairings and the canary comparison.
//!
//! A pairing is kept as one line of a JSON-lines file, so the ratings are
//! recomputed from the whole file whenever they are needed. A pairing may be
//! played again, each time with a fresh seed and as another line
//! ([ADR 0049](../../../docs/adr/0049-parallel-evaluations-and-nightly-league.md),
//! decision 5); a new result is compared with the stored ones by [`drift`].
//! The functions here do no playing; `league --results` and `play` in
//! [`crate::cli`] do.

use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use cg_core::rng::Rng;
use serde::{Deserialize, Serialize};

use crate::ratings::{elo_margins, elo_ratings, MatchupResult};

/// The result of one stored pairing, from the side of `a`, the newer release.
/// One line of the results file; the keys are in this order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PairingResult {
    /// The bot's name as given to `--bot`; results are from its side.
    pub a: String,
    pub b: String,
    /// Seat-swapped pairs played; `wins + draws + losses == 2 * pairs`.
    pub pairs: u32,
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
    /// Pairs in which `a` scored 0, ½, 1, 1½ and 2 points; sums to `pairs`.
    pub pair_points: [u32; 5],
    /// Games lost by a fault (timeouts, crashes, invalid answers): `[a, b]`.
    pub faults: [u32; 2],
    pub time_scale: f64,
    pub time_tolerance_ms: u64,
    pub opening_plies: u32,
    /// The pairing's tournament seed ([`pairing_seed`]).
    pub seed: u64,
    /// UTC date when the pairing finished, `YYYY-MM-DD`.
    pub date: String,
    /// The commit the pairing was played at, or `"unknown"`.
    pub commit: String,
    /// The runner's processor, or `"unknown"`.
    pub cpu: String,
    /// The machine's speed when the time scale was calibrated to
    /// CodinGame's (ADR 0048), in units a millisecond of the calibration
    /// workload; absent for pairings played at a fixed scale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
}

impl PairingResult {
    /// Checks what the line must satisfy beyond its types.
    pub fn validate(&self) -> Result<(), String> {
        if self.a == self.b {
            return Err(format!("a pairing of {:?} with itself", self.a));
        }
        let games = u64::from(self.wins) + u64::from(self.draws) + u64::from(self.losses);
        if games != 2 * u64::from(self.pairs) {
            return Err(format!(
                "{} wins, {} draws and {} losses are not {} games for {} pairs",
                self.wins,
                self.draws,
                self.losses,
                2 * u64::from(self.pairs),
                self.pairs
            ));
        }
        let counted: u64 = self.pair_points.iter().map(|&n| u64::from(n)).sum();
        if counted != u64::from(self.pairs) {
            return Err(format!(
                "pair_points add up to {counted}, not to {} pairs",
                self.pairs
            ));
        }
        Ok(())
    }

    /// The line as written to the results file, without the newline.
    pub fn to_line(&self) -> String {
        serde_json::to_string(self).expect("a pairing result is always valid JSON")
    }

    /// Whether `name` is one of the two bots.
    pub fn involves(&self, name: &str) -> bool {
        self.a == name || self.b == name
    }

    /// Games played, `2 * pairs`.
    pub fn games(&self) -> u32 {
        self.wins + self.draws + self.losses
    }

    /// `a`'s average points per game, from 0 to 1; `None` without games.
    pub fn score(&self) -> Option<f64> {
        let games = self.games();
        (games > 0).then(|| (f64::from(self.wins) + 0.5 * f64::from(self.draws)) / f64::from(games))
    }
}

/// The seeds already used by each unordered pair of bots, with the place of
/// the line that used them, so that no two lines of a pair repeat a seed.
#[derive(Debug, Default)]
pub struct Seeds {
    seen: HashMap<(String, String, u64), (String, usize)>,
}

impl Seeds {
    /// Records the seed of `result`, found on `line` of `source`. Errors
    /// when a line of the same two bots, in either order, used it before.
    pub fn add(&mut self, source: &str, line: usize, result: &PairingResult) -> Result<(), String> {
        let (low, high) = if result.a <= result.b {
            (&result.a, &result.b)
        } else {
            (&result.b, &result.a)
        };
        let key = (low.clone(), high.clone(), result.seed);
        if let Some((first_source, first_line)) = self.seen.get(&key) {
            let place = if first_source == source {
                format!("line {first_line}")
            } else {
                format!("{first_source}:{first_line}")
            };
            return Err(format!(
                "{source}:{line}: {} and {} repeat the seed of {place}",
                result.a, result.b
            ));
        }
        self.seen.insert(key, (source.to_string(), line));
        Ok(())
    }
}

/// Parses the text of a results file. Blank lines are ignored; any other line
/// must be a valid pairing. Several lines may pair the same two bots, in
/// either order, but not with the same seed. Errors name `source` and the line.
pub fn parse_results(text: &str, source: &str) -> Result<Vec<PairingResult>, String> {
    parse_results_with(text, source, &mut Seeds::default())
}

/// [`parse_results`] that also checks the seeds against those `seeds` already
/// holds, so that several files can be checked as one.
pub fn parse_results_with(
    text: &str,
    source: &str,
    seeds: &mut Seeds,
) -> Result<Vec<PairingResult>, String> {
    let mut results = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let number = index + 1;
        let at = |message: String| format!("{source}:{number}: {message}");
        let result: PairingResult =
            serde_json::from_str(line).map_err(|err| at(err.to_string()))?;
        result.validate().map_err(at)?;
        seeds.add(source, number, &result)?;
        results.push(result);
    }
    Ok(results)
}

/// Reads a results file. A missing file is an error; an empty one is fine.
pub fn read_results(path: &Path) -> Result<Vec<PairingResult>, String> {
    let text =
        fs::read_to_string(path).map_err(|err| format!("cannot read {}: {err}", path.display()))?;
    parse_results(&text, &path.display().to_string())
}

/// The 64-bit FNV-1a hash of `bytes`.
pub fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// The tournament seed of the pairing of `a` (the new release) with `b`:
/// `seed` mixed with the hash of `a`, a 0 byte and `b`. The order matters.
pub fn pairing_seed(seed: u64, a: &str, b: &str) -> u64 {
    let mut bytes = Vec::with_capacity(a.len() + b.len() + 1);
    bytes.extend_from_slice(a.as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(b.as_bytes());
    Rng::new(seed ^ fnv1a(&bytes)).next_u64()
}

/// The seed of the canary replay of the stored pairing of `a` with `b`, run
/// for the release `new`, so that each release's canary plays new openings.
pub fn canary_seed(seed: u64, a: &str, b: &str, new: &str) -> u64 {
    Rng::new(pairing_seed(seed, a, b) ^ fnv1a(new.as_bytes())).next_u64()
}

/// Today's date in UTC as `YYYY-MM-DD`.
pub fn today() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());
    date_from_unix(seconds)
}

/// The UTC date of a Unix time in seconds, as `YYYY-MM-DD`.
pub fn date_from_unix(seconds: u64) -> String {
    // Days since 1970-01-01 to a civil date (proleptic Gregorian calendar).
    let days = (seconds / 86_400) as i64 + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}

/// The first `model name` of `/proc/cpuinfo`, or `"unknown"`.
pub fn cpu_model() -> String {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|text| model_name(&text))
        .unwrap_or_else(|| "unknown".to_string())
}

/// The first `model name` line of a `/proc/cpuinfo` text, trimmed.
pub fn model_name(cpuinfo: &str) -> Option<String> {
    cpuinfo.lines().find_map(|line| {
        let (key, value) = line.split_once(':')?;
        let value = value.trim();
        (key.trim() == "model name" && !value.is_empty()).then(|| value.to_string())
    })
}

/// A bot's rating over a set of pairings.
#[derive(Clone, Debug, PartialEq)]
pub struct Rating {
    pub name: String,
    /// Elo, with the anchor at 0. Meaningless when `margin` is infinite.
    pub rating: f64,
    /// Half-width of the 95% interval; infinite for a bot with no chain of
    /// pairings to the anchor, 0 for the anchor.
    pub margin: f64,
    /// Games played in the pairings.
    pub games: u32,
}

impl Rating {
    /// Whether the bot is linked to the anchor by a chain of pairings.
    pub fn is_linked(&self) -> bool {
        self.margin.is_finite()
    }
}

/// Bradley-Terry ratings of `names` over `pairings`, with `anchor` at 0, in
/// the order of `names`. Every bot of a pairing and the anchor must be in
/// `names`.
pub fn rate(
    names: &[String],
    pairings: &[PairingResult],
    anchor: &str,
) -> Result<Vec<Rating>, String> {
    let index: HashMap<&str, usize> = names
        .iter()
        .enumerate()
        .map(|(position, name)| (name.as_str(), position))
        .collect();
    let find = |name: &str| {
        index
            .get(name)
            .copied()
            .ok_or_else(|| format!("{name:?} is not a known bot"))
    };
    let anchor = find(anchor)?;
    let mut matchups = Vec::with_capacity(pairings.len());
    let mut games = vec![0u32; names.len()];
    for pairing in pairings {
        let (a, b) = (find(&pairing.a)?, find(&pairing.b)?);
        matchups.push(MatchupResult {
            a,
            b,
            score_a: f64::from(pairing.wins) + 0.5 * f64::from(pairing.draws),
            games: pairing.games(),
        });
        games[a] += pairing.games();
        games[b] += pairing.games();
    }
    let ratings = elo_ratings(names.len(), &matchups, anchor);
    let margins = elo_margins(names.len(), &matchups, anchor, &ratings);
    Ok(names
        .iter()
        .enumerate()
        .map(|(bot, name)| Rating {
            name: name.clone(),
            rating: ratings[bot],
            margin: margins[bot],
            games: games[bot],
        })
        .collect())
}

/// The opponents a new release plays when the pool is small enough to play
/// them all: the previous release first, then the others in name order.
/// `None` when the pool is larger than the previous release, the distinct
/// fixed opponents and `nearest` bots, so that a choice has to be made.
///
/// `candidates` are the known bots other than the new release.
pub fn all_opponents(
    candidates: &[String],
    previous: Option<&str>,
    fixed: &[String],
    nearest: usize,
) -> Option<Vec<String>> {
    let wanted = usize::from(previous.is_some()) + distinct_fixed(previous, fixed).len() + nearest;
    if candidates.len() > wanted {
        return None;
    }
    let mut order: Vec<String> = previous.map(str::to_string).into_iter().collect();
    let mut rest: Vec<&String> = candidates
        .iter()
        .filter(|name| Some(name.as_str()) != previous)
        .collect();
    rest.sort();
    order.extend(rest.into_iter().cloned());
    Some(order)
}

/// The fixed opponents in the given order, without the previous release and
/// without repeats.
fn distinct_fixed<'a>(previous: Option<&str>, fixed: &'a [String]) -> Vec<&'a String> {
    let mut order: Vec<&String> = Vec::new();
    for name in fixed {
        if Some(name.as_str()) != previous && !order.contains(&name) {
            order.push(name);
        }
    }
    order
}

/// The opponents of a new release, in the order they are played: the previous
/// release, the fixed opponents in the given order (the previous release
/// counts once if it is also fixed), then the `nearest` candidates whose
/// rating is closest to `estimate`, the new release's rating. Those are
/// chosen among the candidates that are neither previous nor fixed and are
/// linked to the anchor (a finite margin in `ratings`); ties go to the name
/// that comes first.
///
/// When the pool is small ([`all_opponents`]), it returns every candidate.
/// Without a previous release, the caller's estimate is the anchor's rating,
/// 0; in practice the workflow always gives one once releases exist.
pub fn choose_opponents(
    candidates: &[String],
    previous: Option<&str>,
    fixed: &[String],
    nearest: usize,
    ratings: &[Rating],
    estimate: f64,
) -> Vec<String> {
    if let Some(all) = all_opponents(candidates, previous, fixed, nearest) {
        return all;
    }
    let mut order: Vec<String> = previous.map(str::to_string).into_iter().collect();
    let fixed = distinct_fixed(previous, fixed);
    order.extend(fixed.iter().map(|name| name.to_string()));
    let mut close: Vec<(f64, &str)> = candidates
        .iter()
        .filter(|name| !order.contains(name))
        .filter_map(|name| {
            let rating = ratings
                .iter()
                .find(|rating| rating.name == *name && rating.is_linked())?;
            Some(((rating.rating - estimate).abs(), name.as_str()))
        })
        .collect();
    close.sort_by(|x, y| x.0.total_cmp(&y.0).then_with(|| x.1.cmp(y.1)));
    order.extend(
        close
            .into_iter()
            .take(nearest)
            .map(|(_, name)| name.to_string()),
    );
    order
}

/// A canary replay or a new pairing compared with the stored ones.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CanaryReport {
    /// The first bot's score now and as stored, in percent.
    pub now: f64,
    pub stored: f64,
    /// `now - stored`, in percentage points.
    pub difference: f64,
    /// Half-width of the 95% interval of the difference, in points.
    pub interval: f64,
    pub games_now: u32,
}

impl CanaryReport {
    /// Whether the difference lies within the interval.
    pub fn within(&self) -> bool {
        self.difference.abs() <= self.interval
    }

    /// The report line for the canary of `a` against `b`.
    pub fn line(&self, a: &str, b: &str) -> String {
        self.labelled("canary", a, b)
    }

    /// The report lines for the drift of `a` against `b`: the report, then a
    /// warning when it is outside the stored interval.
    pub fn drift_lines(&self, a: &str, b: &str) -> Vec<String> {
        let mut lines = vec![self.labelled("drift", a, b)];
        if !self.within() {
            lines.push(format!(
                "warning: drift {a} vs {b} scored outside its stored interval: the runners' \
                 speed may have changed (ADR 0049)"
            ));
        }
        lines
    }

    fn labelled(&self, word: &str, a: &str, b: &str) -> String {
        format!(
            "{word} {a} vs {b}: {:.1}% of {} games now, {:.1}% stored, difference {:+.1} points, \
             95% interval ±{:.1}: {}",
            self.now,
            self.games_now,
            self.stored,
            self.difference,
            self.interval,
            if self.within() { "within" } else { "OUTSIDE" }
        )
    }
}

/// The mean and the variance of the mean of the score per pair, where a pair
/// that scored `p` points in {0, ½, 1, 1½, 2} counts `p / 2`. `None` without
/// pairs.
fn pair_score_moments(pair_points: &[u32; 5]) -> Option<(f64, f64)> {
    let pairs: u32 = pair_points.iter().sum();
    if pairs == 0 {
        return None;
    }
    let n = f64::from(pairs);
    let value = |k: usize| k as f64 / 4.0;
    let mean = (0..5)
        .map(|k| value(k) * f64::from(pair_points[k]))
        .sum::<f64>()
        / n;
    let variance = (0..5)
        .map(|k| f64::from(pair_points[k]) * (value(k) - mean).powi(2))
        .sum::<f64>()
        / n;
    Some((mean, variance / n))
}

/// Compares a replay of `stored`, whose pairs scored `now_pair_points` for the
/// first bot, with the stored result: the interval is 1.96 standard errors of
/// the difference, from the pairs' points of both. `None` when either has no
/// pair.
pub fn compare_canary(stored: &PairingResult, now_pair_points: &[u32; 5]) -> Option<CanaryReport> {
    compare_pair_points(&stored.pair_points, now_pair_points)
}

/// [`compare_canary`] for stored pair points given as an array.
pub fn compare_pair_points(
    stored_pair_points: &[u32; 5],
    now_pair_points: &[u32; 5],
) -> Option<CanaryReport> {
    let (stored_mean, stored_variance) = pair_score_moments(stored_pair_points)?;
    let (now_mean, now_variance) = pair_score_moments(now_pair_points)?;
    let games_now = 2 * now_pair_points.iter().sum::<u32>();
    Some(CanaryReport {
        now: 100.0 * now_mean,
        stored: 100.0 * stored_mean,
        difference: 100.0 * (now_mean - stored_mean),
        interval: 100.0 * 1.96 * (stored_variance + now_variance).sqrt(),
        games_now,
    })
}

/// The pair points of every line of the unordered pair `a`, `b`, added up and
/// seen from `a`: a line stored as (`b`, `a`) counts with its array
/// reversed. `None` when no line pairs them.
pub fn combined_pair_points(stored: &[PairingResult], a: &str, b: &str) -> Option<[u32; 5]> {
    let mut total = [0u32; 5];
    let mut found = false;
    for line in stored {
        let reversed = if line.a == a && line.b == b {
            false
        } else if line.a == b && line.b == a {
            true
        } else {
            continue;
        };
        found = true;
        for (k, slot) in total.iter_mut().enumerate() {
            *slot += line.pair_points[if reversed { 4 - k } else { k }];
        }
    }
    found.then_some(total)
}

/// Compares the new pairing with every stored line of the same two bots
/// combined ([`combined_pair_points`], seen from `new.a`), as a sign that the
/// runners' speed has changed (ADR 0049, decision 5). `None` when the pair has
/// no stored line or either side has no pair.
pub fn drift(stored: &[PairingResult], new: &PairingResult) -> Option<CanaryReport> {
    let combined = combined_pair_points(stored, &new.a, &new.b)?;
    compare_pair_points(&combined, &new.pair_points)
}

/// Every bot named in `pairings` or in `extra`, sorted, without repeats.
pub fn known_bots<'a>(
    pairings: &[PairingResult],
    extra: impl IntoIterator<Item = &'a str>,
) -> Vec<String> {
    let mut names: BTreeSet<String> = extra.into_iter().map(str::to_string).collect();
    for pairing in pairings {
        names.insert(pairing.a.clone());
        names.insert(pairing.b.clone());
    }
    names.into_iter().collect()
}

#[cfg(test)]
mod tests;
