//! The planner of the nightly league
//! ([ADR 0049](../../../docs/adr/0049-parallel-evaluations-and-nightly-league.md),
//! decision 4): from the results file and the game's pool of bots it chooses
//! the pairings to play tonight, as the lines of a plan.
//!
//! The night has three steps. Bots that lack games get the opponents of the
//! incremental league first (`new`). Then most of the budget goes to the
//! pairings that shrink the intervals of the top bots most (`top`), and the
//! rest to pairings among the other bots, the least recently played first
//! (`low`). The planner plays nothing and reads no file; `plan` and `play` in
//! [`crate::cli`] do.

use std::collections::{BTreeSet, HashMap, HashSet};

use cg_core::rng::Rng;
use serde::{Deserialize, Serialize};

use crate::league::{self, fnv1a, pairing_seed, PairingResult, Rating};

/// Why a pairing is planned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// A bot that lacks games plays the opponents of ADR 0038.
    New,
    /// Precision near the top of the ratings.
    Top,
    /// The other bots, so that they play from time to time.
    Low,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::New => "new",
            Kind::Top => "top",
            Kind::Low => "low",
        }
    }
}

/// One line of a plan: a pairing to play tonight. The keys are in this order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanLine {
    /// The first bot of the pairing; the new bot, or the higher-rated one.
    pub a: String,
    pub b: String,
    /// Seat-swapped pairs to play.
    pub pairs: u32,
    /// The pairing's tournament seed.
    pub seed: u64,
    pub kind: Kind,
    /// The runner that plays the line, from 0.
    pub shard: usize,
}

impl PlanLine {
    /// Checks what the line must satisfy beyond its types.
    pub fn validate(&self) -> Result<(), String> {
        if self.a == self.b {
            return Err(format!("a pairing of {:?} with itself", self.a));
        }
        if self.pairs == 0 {
            return Err("a pairing of 0 pairs".to_string());
        }
        Ok(())
    }

    /// The line as written to the plan file, without the newline.
    pub fn to_line(&self) -> String {
        serde_json::to_string(self).expect("a plan line is always valid JSON")
    }
}

/// Parses the text of a plan file. Blank lines are ignored; any other line
/// must be a valid plan line. Errors name `source` and the line.
pub fn parse_plan(text: &str, source: &str) -> Result<Vec<PlanLine>, String> {
    let mut lines = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let at = |message: String| format!("{source}:{}: {message}", index + 1);
        let parsed: PlanLine = serde_json::from_str(line).map_err(|err| at(err.to_string()))?;
        parsed.validate().map_err(at)?;
        lines.push(parsed);
    }
    Ok(lines)
}

/// What the planner needs besides the results.
#[derive(Clone, Debug)]
pub struct Settings {
    /// The bots the night may play.
    pub pool: Vec<String>,
    /// The releases in release order, a subset of the pool.
    pub releases: Vec<String>,
    /// The bot rated 0 Elo; in the pool.
    pub anchor: String,
    /// Opponents every bot plays, whatever its rating; in the pool.
    pub fixed: Vec<String>,
    /// Opponents beyond the previous release and the fixed ones.
    pub nearest: usize,
    /// Pairs of each `new` line.
    pub new_pairs: u32,
    /// Pairs for the night; `new` lines come first and are always planned.
    pub budget_pairs: u64,
    /// Pairs in one step of `top` or `low`.
    pub block_pairs: u32,
    /// How many of the best bots the target covers.
    pub top: usize,
    /// The half-width, in Elo, the top bots' intervals should reach.
    pub target_margin: f64,
    /// The share of what the new bots leave that goes to `low`, 0 to 1.
    pub low_share: f64,
    /// The night, `YYYY-MM-DD`; it varies the seeds.
    pub night: String,
    /// The seed the pairings' seeds derive from.
    pub seed: u64,
    pub shards: usize,
}

impl Settings {
    /// Checks the names and the numbers.
    pub fn validate(&self) -> Result<(), String> {
        let distinct = |option: &str, names: &[String]| {
            for (index, name) in names.iter().enumerate() {
                if name.is_empty() {
                    return Err(format!("{option} has an empty name"));
                }
                if names[..index].contains(name) {
                    return Err(format!("{option} names {name:?} twice"));
                }
            }
            Ok(())
        };
        distinct("--pool", &self.pool)?;
        distinct("--releases", &self.releases)?;
        distinct("--fixed-opponents", &self.fixed)?;
        for (option, names) in [
            ("--releases", &self.releases),
            ("--fixed-opponents", &self.fixed),
        ] {
            if let Some(name) = names.iter().find(|name| !self.pool.contains(name)) {
                return Err(format!("{option} {name:?} is not in the --pool"));
            }
        }
        if !self.pool.contains(&self.anchor) {
            return Err(format!("--anchor {:?} is not in the --pool", self.anchor));
        }
        for (option, value) in [
            ("--block-pairs", u64::from(self.block_pairs)),
            ("--top", self.top as u64),
            ("--shards", self.shards as u64),
            ("--new-pairs", u64::from(self.new_pairs)),
        ] {
            if value == 0 {
                return Err(format!("{option} must be at least 1"));
            }
        }
        if !(0.0..=1.0).contains(&self.low_share) {
            return Err("--low-share must be from 0 to 1".to_string());
        }
        if !self.target_margin.is_finite() || self.target_margin < 0.0 {
            return Err("--target-margin must be 0 or more".to_string());
        }
        if !is_date(&self.night) {
            return Err(format!(
                "--night {:?} must look like YYYY-MM-DD",
                self.night
            ));
        }
        Ok(())
    }
}

/// Whether `text` is a calendar-shaped date, `YYYY-MM-DD`.
fn is_date(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.len() != 10
        || !bytes.iter().enumerate().all(|(at, byte)| {
            if at == 4 || at == 7 {
                *byte == b'-'
            } else {
                byte.is_ascii_digit()
            }
        })
    {
        return false;
    }
    let number = |range: std::ops::Range<usize>| text[range].parse::<u32>().unwrap_or(0);
    (1..=12).contains(&number(5..7)) && (1..=31).contains(&number(8..10))
}

/// A night's plan.
#[derive(Clone, Debug, PartialEq)]
pub struct Plan {
    /// The pairings: `new`, then `top`, then `low`.
    pub lines: Vec<PlanLine>,
    /// What to tell the user, one line each; the last says whether the
    /// target is met.
    pub report: Vec<String>,
}

/// An unordered pair of bots.
fn key(x: &str, y: &str) -> (String, String) {
    if x <= y {
        (x.to_string(), y.to_string())
    } else {
        (y.to_string(), x.to_string())
    }
}

/// The lines being planned, at most one per unordered pair.
struct Builder<'a> {
    settings: &'a Settings,
    /// The seeds stored for each unordered pair.
    stored_seeds: HashSet<(String, String, u64)>,
    lines: Vec<(PlanLine, u64)>,
    index: HashMap<(String, String), usize>,
}

impl<'a> Builder<'a> {
    fn new(settings: &'a Settings, stored: &[PairingResult]) -> Self {
        let stored_seeds = stored
            .iter()
            .map(|line| {
                let (low, high) = key(&line.a, &line.b);
                (low, high, line.seed)
            })
            .collect();
        Builder {
            settings,
            stored_seeds,
            lines: Vec::new(),
            index: HashMap::new(),
        }
    }

    fn is_stored(&self, a: &str, b: &str, seed: u64) -> bool {
        let (low, high) = key(a, b);
        self.stored_seeds.contains(&(low, high, seed))
    }

    /// The night's seed for `a` against `b`: it differs between nights and is
    /// not one stored for the pair.
    fn night_seed(&self, a: &str, b: &str) -> u64 {
        let base = pairing_seed(self.settings.seed, a, b);
        let mut rng = Rng::new(base ^ fnv1a(self.settings.night.as_bytes()));
        loop {
            let seed = rng.next_u64();
            if !self.is_stored(a, b, seed) {
                return seed;
            }
        }
    }

    /// Plans `pairs` pairs of `a` against `b` as `kind`, adding them to the
    /// pair's line if there is one. `new` lines keep the incremental league's
    /// seed unless it is stored.
    fn add(&mut self, a: &str, b: &str, pairs: u32, kind: Kind) {
        if let Some(&at) = self.index.get(&key(a, b)) {
            self.lines[at].1 += u64::from(pairs);
            return;
        }
        let seed = match kind {
            Kind::New => {
                let seed = pairing_seed(self.settings.seed, a, b);
                if self.is_stored(a, b, seed) {
                    self.night_seed(a, b)
                } else {
                    seed
                }
            }
            Kind::Top | Kind::Low => self.night_seed(a, b),
        };
        self.index.insert(key(a, b), self.lines.len());
        self.lines.push((
            PlanLine {
                a: a.to_string(),
                b: b.to_string(),
                pairs: 0,
                seed,
                kind,
                shard: 0,
            },
            u64::from(pairs),
        ));
    }

    fn contains(&self, a: &str, b: &str) -> bool {
        self.index.contains_key(&key(a, b))
    }
}

/// The opponents each bot has in the stored lines.
fn opponents(stored: &[PairingResult]) -> HashMap<&str, HashSet<&str>> {
    let mut found: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in stored {
        found.entry(&line.a).or_default().insert(&line.b);
        found.entry(&line.b).or_default().insert(&line.a);
    }
    found
}

/// What a block adds to the information of one bot's rating, given the
/// block's own information `base` (as if the opponent's rating were exact) and
/// the information of the opponent's rating: the opponent's variance reduces
/// it. An opponent with no information adds nothing; the anchor's infinite
/// information leaves `base`.
fn added_information(base: f64, opponent_information: f64) -> f64 {
    if base <= 0.0 || !base.is_finite() || opponent_information <= 0.0 {
        return 0.0;
    }
    1.0 / (1.0 / base + 1.0 / opponent_information)
}

/// The expected score of a bot rated `x` against one rated `y`.
fn expected(x: f64, y: f64) -> f64 {
    1.0 / (1.0 + 10f64.powf((y - x) / 400.0))
}

/// Plans the night: `stored` are the lines of the results file.
pub fn plan(stored: &[PairingResult], settings: &Settings) -> Result<Plan, String> {
    settings.validate()?;
    let names = league::known_bots(stored, settings.pool.iter().map(String::as_str));
    let ratings = league::rate(&names, stored, &settings.anchor)?;
    let rating_of: HashMap<&str, &Rating> = ratings
        .iter()
        .map(|rating| (rating.name.as_str(), rating))
        .collect();
    let played = opponents(stored);
    let has_played = |x: &str, y: &str| played.get(x).is_some_and(|set| set.contains(y));

    // The pool in order: releases in release order, then the others by name.
    let mut order: Vec<String> = settings.releases.clone();
    let mut others: Vec<String> = settings
        .pool
        .iter()
        .filter(|name| !settings.releases.contains(name))
        .cloned()
        .collect();
    others.sort();
    order.extend(others);
    let previous_of = |name: &str| -> Option<&str> {
        let at = settings
            .releases
            .iter()
            .position(|release| release == name)?;
        at.checked_sub(1)
            .map(|before| settings.releases[before].as_str())
    };

    let mut builder = Builder::new(settings, stored);
    let mut report = Vec::new();

    // Step N: the bots that lack games.
    let mut incomplete: Vec<&str> = Vec::new();
    for bot in &order {
        let previous = previous_of(bot);
        let mut required: Vec<&str> = previous.into_iter().collect();
        for fixed in &settings.fixed {
            if fixed != bot && !required.contains(&fixed.as_str()) {
                required.push(fixed);
            }
        }
        let missing_required: Vec<String> = required
            .iter()
            .filter(|name| !has_played(bot, name))
            .map(|name| name.to_string())
            .collect();
        let rest: Vec<String> = order
            .iter()
            .filter(|name| *name != bot && !required.contains(&name.as_str()))
            .cloned()
            .collect();
        let wanted = settings.nearest.min(rest.len());
        let have = rest.iter().filter(|name| has_played(bot, name)).count();
        let missing_nearest = wanted.saturating_sub(have);
        if missing_required.is_empty() && missing_nearest == 0 {
            continue;
        }
        incomplete.push(bot);

        let has_line = played.contains_key(bot.as_str());
        let mut chosen = missing_required;
        if (has_line || previous.is_none()) && missing_nearest > 0 {
            let candidates: Vec<String> = rest
                .iter()
                .filter(|name| !has_played(bot, name))
                .cloned()
                .collect();
            let linked_rating = |name: &str| {
                rating_of
                    .get(name)
                    .filter(|rating| rating.is_linked())
                    .map(|rating| rating.rating)
            };
            let estimate = linked_rating(bot)
                .or_else(|| previous.and_then(linked_rating))
                .unwrap_or(0.0);
            chosen.extend(league::choose_opponents(
                &candidates,
                None,
                &[],
                missing_nearest,
                &ratings,
                estimate,
            ));
        }
        for opponent in chosen {
            if !builder.contains(bot, &opponent) {
                builder.add(bot, &opponent, settings.new_pairs, Kind::New);
            }
        }
    }
    let new_pairs: u64 = builder.lines.iter().map(|(_, pairs)| pairs).sum();
    let new_lines = builder.lines.len();

    // The budget that remains.
    let rest = settings.budget_pairs.saturating_sub(new_pairs);
    let block = u64::from(settings.block_pairs);
    let low = ((rest as f64 * settings.low_share).round() as u64).min(rest);
    let top = rest - low;
    let low_blocks = low / block;
    let top_blocks = top / block + (low % block + top % block) / block;

    // The linked pool bots, best first.
    let mut linked: Vec<&Rating> = settings
        .pool
        .iter()
        .filter_map(|name| rating_of.get(name.as_str()).copied())
        .filter(|rating| rating.is_linked())
        .collect();
    linked.sort_by(|x, y| {
        y.rating
            .total_cmp(&x.rating)
            .then_with(|| x.name.cmp(&y.name))
    });
    let top_len = settings.top.min(linked.len());
    let target = settings.target_margin;

    // Step T: precision near the top.
    let met = incomplete.is_empty() && linked[..top_len].iter().all(|r| r.margin <= target);
    // The information of every linked bot's rating, the anchor's infinite.
    let mut information: Vec<f64> = linked
        .iter()
        .map(|rating| {
            if rating.margin > 0.0 {
                (1.96 / rating.margin).powi(2)
            } else {
                f64::INFINITY
            }
        })
        .collect();
    let margin_of = |information: f64| 1.96 / information.sqrt();
    let mut top_pairs: HashSet<(String, String)> = HashSet::new();
    let mut top_planned = 0u64;
    if !met {
        let c = std::f64::consts::LN_10 / 400.0;
        let mut candidates: BTreeSet<(usize, usize)> = BTreeSet::new();
        for i in 0..top_len {
            for j in i.saturating_sub(2)..(i + 3).min(linked.len()) {
                if j != i {
                    candidates.insert((i.min(j), i.max(j)));
                }
            }
        }
        let candidates: Vec<(usize, usize, f64)> = candidates
            .into_iter()
            .map(|(x, y)| {
                let p = expected(linked[x].rating, linked[y].rating);
                (
                    x,
                    y,
                    2.0 * f64::from(settings.block_pairs) * p * (1.0 - p) * c * c,
                )
            })
            .collect();
        let mut chosen: Vec<u64> = vec![0; candidates.len()];
        for _ in 0..top_blocks {
            let mut best: Option<(usize, f64)> = None;
            for (at, &(x, y, delta)) in candidates.iter().enumerate() {
                let mut gain = 0.0;
                for (k, o) in [(x, y), (y, x)] {
                    if k < top_len && information[k].is_finite() {
                        let now = margin_of(information[k]);
                        if now > target {
                            let after = margin_of(
                                information[k] + added_information(delta, information[o]),
                            );
                            gain += (now * now - after * after) / (1.0 + k as f64);
                        }
                    }
                }
                if gain <= 0.0 {
                    continue;
                }
                let better = match best {
                    None => true,
                    Some((held, held_gain)) => {
                        let (hx, hy, _) = candidates[held];
                        gain.total_cmp(&held_gain)
                            .then_with(|| hx.cmp(&x))
                            .then_with(|| {
                                key(&linked[hx].name, &linked[hy].name)
                                    .cmp(&key(&linked[x].name, &linked[y].name))
                            })
                            .is_gt()
                    }
                };
                if better {
                    best = Some((at, gain));
                }
            }
            let Some((at, _)) = best else { break };
            let (x, y, delta) = candidates[at];
            let (for_x, for_y) = (
                added_information(delta, information[y]),
                added_information(delta, information[x]),
            );
            information[x] += for_x;
            information[y] += for_y;
            chosen[at] += 1;
            top_planned += block;
        }
        for (at, &blocks) in chosen.iter().enumerate() {
            if blocks > 0 {
                let (x, y, _) = candidates[at];
                let pairs = blocks * block;
                add_pairs(
                    &mut builder,
                    &linked[x].name,
                    &linked[y].name,
                    pairs,
                    Kind::Top,
                )?;
                top_pairs.insert(key(&linked[x].name, &linked[y].name));
            }
        }
    }

    // Step L: the other bots.
    let mut low_planned = 0u64;
    let mut others: Vec<&Rating> = linked[top_len..].to_vec();
    if others.len() >= 2 && low_blocks > 0 {
        let latest = |name: &str| {
            stored
                .iter()
                .filter(|line| line.involves(name))
                .map(|line| line.date.as_str())
                .max()
                .unwrap_or("")
        };
        others.sort_by(|x, y| {
            latest(&x.name)
                .cmp(latest(&y.name))
                .then_with(|| x.games.cmp(&y.games))
                .then_with(|| x.name.cmp(&y.name))
        });
        let mut pending = Vec::new();
        let mut left = low_blocks;
        let (mut at, mut idle) = (0usize, 0usize);
        while left > 0 && idle < others.len() {
            let bot = others[at % others.len()];
            at += 1;
            let partner = linked[top_len..]
                .iter()
                .filter(|other| other.name != bot.name)
                .filter(|other| !top_pairs.contains(&key(&bot.name, &other.name)))
                .min_by(|x, y| {
                    (x.rating - bot.rating)
                        .abs()
                        .total_cmp(&(y.rating - bot.rating).abs())
                        .then_with(|| x.name.cmp(&y.name))
                });
            match partner {
                Some(partner) => {
                    pending.push((bot, *partner));
                    left -= 1;
                    idle = 0;
                }
                None => idle += 1,
            }
        }
        for (bot, partner) in pending {
            let (first, second) = if bot.rating > partner.rating
                || (bot.rating == partner.rating && bot.name < partner.name)
            {
                (bot, partner)
            } else {
                (partner, bot)
            };
            add_pairs(&mut builder, &first.name, &second.name, block, Kind::Low)?;
            low_planned += block;
        }
    }

    // The lines in order, then the shards.
    let mut lines: Vec<(PlanLine, u64)> = Vec::new();
    for kind in [Kind::New, Kind::Top, Kind::Low] {
        lines.extend(
            builder
                .lines
                .iter()
                .filter(|(line, _)| line.kind == kind)
                .cloned(),
        );
    }
    let mut result: Vec<PlanLine> = Vec::with_capacity(lines.len());
    for (mut line, pairs) in lines {
        line.pairs = u32::try_from(pairs)
            .map_err(|_| format!("{} against {} would play too many pairs", line.a, line.b))?;
        result.push(line);
    }
    let sizes: Vec<u64> = result.iter().map(|line| u64::from(line.pairs)).collect();
    for (line, shard) in result
        .iter_mut()
        .zip(assign_shards(&sizes, settings.shards))
    {
        line.shard = shard;
    }

    // The report.
    let count = |kind: Kind| {
        let selected = result.iter().filter(|line| line.kind == kind);
        (
            selected.clone().count(),
            selected.map(|line| u64::from(line.pairs)).sum::<u64>(),
        )
    };
    let counts: Vec<String> = [Kind::New, Kind::Top, Kind::Low]
        .iter()
        .map(|&kind| {
            let (lines, pairs) = count(kind);
            format!("{}: {lines} lines, {pairs} pairs", kind.name())
        })
        .collect();
    report.push(format!(
        "plan for {} with a budget of {} pairs: {}",
        settings.night,
        settings.budget_pairs,
        counts.join("; ")
    ));
    report.push(format!(
        "new lines before the budget: {new_lines}; left {rest}: {top} for top ({top_planned} \
         planned), {low} for low ({low_planned} planned)"
    ));
    let mut over = Vec::new();
    for (rank, rating) in linked[..top_len].iter().enumerate() {
        let projected = margin_of(information[rank]);
        report.push(format!(
            "top {}: {} margin ±{:.1} now, ±{:.1} estimated after tonight",
            rank + 1,
            rating.name,
            rating.margin,
            projected
        ));
        if projected > target {
            over.push(rating.name.clone());
        }
    }
    if met {
        report.push(format!("target met: top {top_len} within ±{target} Elo"));
    } else {
        let mut reasons = Vec::new();
        if !incomplete.is_empty() {
            reasons.push(format!("bots lacking games: {}", incomplete.join(", ")));
        }
        let now_over: Vec<&str> = linked[..top_len]
            .iter()
            .filter(|rating| rating.margin > target)
            .map(|rating| rating.name.as_str())
            .collect();
        if !now_over.is_empty() {
            reasons.push(format!(
                "over ±{target} Elo now: {} (still over after tonight: {})",
                now_over.join(", "),
                if over.is_empty() {
                    "none".to_string()
                } else {
                    over.join(", ")
                }
            ));
        }
        report.push(format!("target not met: {}", reasons.join("; ")));
    }
    Ok(Plan {
        lines: result,
        report,
    })
}

/// The shard of each line, from the lines' sizes: the largest first (equal
/// sizes in the order given), each to the shard with the fewest pairs so far,
/// the lowest index on a tie.
pub fn assign_shards(sizes: &[u64], shards: usize) -> Vec<usize> {
    let mut by_size: Vec<usize> = (0..sizes.len()).collect();
    by_size.sort_by(|&x, &y| sizes[y].cmp(&sizes[x]));
    let mut load = vec![0u64; shards.max(1)];
    let mut assigned = vec![0usize; sizes.len()];
    for at in by_size {
        let shard = (0..load.len())
            .min_by_key(|&shard| (load[shard], shard))
            .unwrap_or(0);
        load[shard] += sizes[at];
        assigned[at] = shard;
    }
    assigned
}

/// Adds `pairs` pairs of `a` against `b` to the builder, checking that the
/// total fits a line.
fn add_pairs(
    builder: &mut Builder,
    a: &str,
    b: &str,
    pairs: u64,
    kind: Kind,
) -> Result<(), String> {
    let pairs =
        u32::try_from(pairs).map_err(|_| format!("{a} against {b} would play too many pairs"))?;
    builder.add(a, b, pairs, kind);
    Ok(())
}

#[cfg(test)]
mod tests;
