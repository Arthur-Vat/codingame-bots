//! Curves turn by turn: the median of a metric at each game turn for two
//! groups of games, how well the two groups tell apart (the AUC) and from
//! which turn they stay apart. Two sets are made: the top players against us
//! in the games between them, and our wins against our losses.

use serde::Serialize;

use super::data::{Group, Kind};
use super::{mean, Profiling, SMALL};
use crate::trace::Trace;

/// The AUC from which two groups count as apart: this or more in the
/// direction that matters (or this much less, `1 - SEPARATED`, in the other
/// direction when both directions count).
pub const SEPARATED: f64 = 0.7;

/// A metric, or an event's share, at one game turn.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Point {
    pub turn: u32,
    /// The games that have a value at this turn.
    pub games: usize,
    pub median: Option<f64>,
    /// The quartiles: the middle half of the games lies between them.
    pub p25: Option<f64>,
    pub p75: Option<f64>,
    pub mean: Option<f64>,
}

impl Point {
    /// A point whose games all have the value `value`.
    pub fn new(turn: u32, games: usize, value: Option<f64>) -> Point {
        Point {
            turn,
            games,
            median: value,
            p25: value,
            p75: value,
            mean: value,
        }
    }
}

/// The count, median, quartiles and mean of some values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stats {
    pub n: usize,
    pub median: f64,
    pub p25: f64,
    pub p75: f64,
    pub mean: f64,
}

/// The value at the fraction `q` of the sorted `values` (linear between the
/// two nearest).
fn percentile(sorted: &[f64], q: f64) -> f64 {
    let pos = q * (sorted.len() - 1) as f64;
    let (low, high) = (pos.floor() as usize, pos.ceil() as usize);
    sorted[low] + (sorted[high] - sorted[low]) * (pos - low as f64)
}

/// The statistics of `values`, `None` if there are none.
pub fn stats(values: &[f64]) -> Option<Stats> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    Some(Stats {
        n: sorted.len(),
        median: percentile(&sorted, 0.5),
        p25: percentile(&sorted, 0.25),
        p75: percentile(&sorted, 0.75),
        mean: sorted.iter().sum::<f64>() / sorted.len() as f64,
    })
}

/// One metric's curves for the two groups of a set.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Curve {
    pub key: String,
    /// What the metric measures (the trace's words).
    pub meaning: String,
    pub lower_is_better: bool,
    /// An event: the points' `mean` is the share of the games (0 to 1) in
    /// which the side does it at that turn, and the line shows it in percent.
    pub rate: bool,
    /// The two groups' values at turns 1 to the set's last turn.
    pub lines: [Vec<Point>; 2],
    /// At each turn from 1, the AUC of the first group against the second:
    /// the share of pairs of values, one from each group, where the first
    /// group's value is the better one (higher, or lower when less is
    /// better), a tie counting half; `null` when a group has fewer than
    /// `SMALL` values.
    pub auc: Vec<Option<f64>>,
    /// The first turn from which the AUC stays apart to the last turn that
    /// has one, if it does.
    pub separates_from: Option<u32>,
}

/// A set of curves: one chart a metric.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct CurveSet {
    /// File-name part of the set's charts.
    pub slug: String,
    pub labels: [String; 2],
    /// The entries of each group: a game gives one entry, a side of it.
    pub games: [usize; 2],
    /// The last game turn reached by at least half of the entries.
    pub last_turn: u32,
    /// Whether the groups count as apart when either one is clearly better;
    /// otherwise only when the first one is.
    pub either: bool,
    pub curves: Vec<Curve>,
    /// How often each event happens at each turn, for the two groups.
    pub events: Vec<Curve>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Curves {
    /// The turns at which the sets' metrics are ranked ([`CurveSet::compare`]).
    pub compare_turns: Vec<u32>,
    /// Us (first line) against the top players (second line), in the games
    /// between them.
    pub top_vs_us: CurveSet,
    /// Our wins (first line) against our losses (second line).
    pub wins_vs_losses: CurveSet,
    /// For each focus player, us (first line) against it, in the games
    /// between them, for the metrics that tell our wins from our losses best.
    pub players: Vec<CurveSet>,
}

/// A metric at one compared turn.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SepCell {
    pub turn: u32,
    /// The AUC of the first group against the second, `null` if too few.
    pub auc: Option<f64>,
    pub first: Option<f64>,
    pub second: Option<f64>,
    pub first_games: usize,
    pub second_games: usize,
}

/// A metric ranked by how well it tells the two groups apart.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SepRow {
    pub key: String,
    pub lower_is_better: bool,
    pub at: Vec<SepCell>,
    /// The mean over the compared turns of the AUC (of the larger of the AUC
    /// and its complement when either group counts), or `null`.
    pub score: Option<f64>,
    pub separates_from: Option<u32>,
}

/// The metrics of a set, ranked.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Comparison {
    pub slug: String,
    pub labels: [String; 2],
    pub games: [usize; 2],
    pub either: bool,
    pub rows: Vec<SepRow>,
}

/// A side of a game.
#[derive(Clone, Copy)]
pub(super) struct Entry<'a> {
    pub trace: &'a Trace,
    pub seat: usize,
}

/// The last game turn at which `seat` has a ply.
pub(super) fn last_own_turn(trace: &Trace, seat: usize) -> u32 {
    trace
        .plies
        .iter()
        .filter(|p| p.to_act == seat)
        .map(|p| p.game_turn)
        .max()
        .unwrap_or(0)
}

/// `seat`'s value of metric `index` at each game turn from 1 to `last`,
/// read as `value_at` reads it (right after its own ply).
pub(super) fn values(trace: &Trace, seat: usize, index: usize, last: u32) -> Vec<Option<i64>> {
    let mut out = vec![None; last as usize];
    let mut seen = vec![false; last as usize];
    for (i, p) in trace.plies.iter().enumerate() {
        let turn = p.game_turn as usize;
        if p.to_act != seat || turn == 0 || turn > out.len() || seen[turn - 1] {
            continue;
        }
        seen[turn - 1] = true;
        let ply = trace.plies.get(i + 1).unwrap_or(p);
        out[turn - 1] = ply.metrics[seat]
            .get(index)
            .copied()
            .filter(|&v| trace.metrics[index].none != Some(v));
    }
    out
}

/// The share of pairs (a value of `a`, a value of `b`) where `a`'s is
/// higher, a tie counting half; `None` if either has fewer than `SMALL`.
fn auc(a: &[f64], b: &[f64]) -> Option<f64> {
    if a.len() < SMALL || b.len() < SMALL {
        return None;
    }
    let mut score = 0.0;
    for x in a {
        for y in b {
            score += match x.total_cmp(y) {
                std::cmp::Ordering::Greater => 1.0,
                std::cmp::Ordering::Equal => 0.5,
                std::cmp::Ordering::Less => 0.0,
            };
        }
    }
    Some(score / (a.len() * b.len()) as f64)
}

/// The AUC read so that more is always better for the first group.
fn oriented(raw: f64, lower_is_better: bool) -> f64 {
    if lower_is_better {
        1.0 - raw
    } else {
        raw
    }
}

/// The first turn from which the oriented AUCs `by_turn` (turn 1 first) stay
/// apart to the last that exists. With `either`, the side of the last one
/// decides which way counts.
fn separates_from(by_turn: &[Option<f64>], either: bool) -> Option<u32> {
    let known: Vec<(u32, f64)> = by_turn
        .iter()
        .enumerate()
        .filter_map(|(i, v)| Some((i as u32 + 1, (*v)?)))
        .collect();
    let (_, last) = *known.last()?;
    let up = last >= SEPARATED;
    if !(up || either && last <= 1.0 - SEPARATED) {
        return None;
    }
    let mut from = None;
    for &(turn, v) in known.iter().rev() {
        let apart = if up {
            v >= SEPARATED
        } else {
            v <= 1.0 - SEPARATED
        };
        if !apart {
            break;
        }
        from = Some(turn);
    }
    from
}

/// What happens in `seat`'s first ply of each turn from 1 to `last`: its
/// events, or `None` if it has no ply then.
pub(super) fn event_turns<P: Profiling>(e: &Entry, last: u32) -> Vec<Option<Vec<String>>> {
    let mut out = vec![None; last as usize];
    for (i, p) in e.trace.plies.iter().enumerate() {
        let turn = p.game_turn as usize;
        if p.to_act == e.seat && turn >= 1 && turn <= out.len() && out[turn - 1].is_none() {
            out[turn - 1] = Some(P::events(e.trace, i));
        }
    }
    out
}

/// One curve from each group's entries' values by turn.
pub(super) fn build_curve(
    key: &str,
    meaning: String,
    lower_is_better: bool,
    rate: bool,
    columns: &[Vec<Vec<Option<f64>>>],
    last_turn: u32,
    either: bool,
) -> Curve {
    let mut lines = [Vec::new(), Vec::new()];
    let mut by_turn = Vec::new();
    for turn in 1..=last_turn {
        let at: Vec<Vec<f64>> = columns
            .iter()
            .map(|g| g.iter().filter_map(|v| v[turn as usize - 1]).collect())
            .collect();
        for (line, values) in lines.iter_mut().zip(&at) {
            let s = stats(values);
            line.push(Point {
                turn,
                games: values.len(),
                median: s.map(|s| s.median),
                p25: s.map(|s| s.p25),
                p75: s.map(|s| s.p75),
                mean: s.map(|s| s.mean),
            });
        }
        by_turn.push(auc(&at[0], &at[1]).map(|raw| oriented(raw, lower_is_better)));
    }
    Curve {
        key: key.to_string(),
        meaning,
        lower_is_better,
        rate,
        lines,
        separates_from: separates_from(&by_turn, either),
        auc: by_turn,
    }
}

/// The curves of `keys` (the key metrics if `None`), and of the events if
/// `events`, for two groups of entries.
pub(super) fn curve_set<P: Profiling>(
    slug: &str,
    labels: [&str; 2],
    either: bool,
    groups: [Vec<Entry>; 2],
    keys: Option<&[String]>,
    events: bool,
) -> CurveSet {
    let entries = groups[0].len() + groups[1].len();
    let lasts: Vec<u32> = groups
        .iter()
        .flatten()
        .map(|e| last_own_turn(e.trace, e.seat))
        .collect();
    let last_turn = (1..=lasts.iter().copied().max().unwrap_or(0))
        .rev()
        .find(|&t| lasts.iter().filter(|&&l| l >= t).count() * 2 >= entries)
        .unwrap_or(0);
    let mut set = CurveSet {
        slug: slug.to_string(),
        labels: labels.map(str::to_string),
        games: [groups[0].len(), groups[1].len()],
        last_turn,
        either,
        curves: Vec::new(),
        events: Vec::new(),
    };
    if last_turn == 0 {
        return set;
    }
    let keys: Vec<String> = match keys {
        Some(keys) => keys.to_vec(),
        None => P::key_metrics().into_iter().map(str::to_string).collect(),
    };
    for key in &keys {
        let meaning = groups.iter().flatten().find_map(|e| {
            e.trace
                .metric(key)
                .map(|i| e.trace.metrics[i].meaning.clone())
        });
        let Some(meaning) = meaning else { continue };
        // Each group's entries' values, by turn.
        let columns: Vec<Vec<Vec<Option<f64>>>> = groups
            .iter()
            .map(|g| {
                g.iter()
                    .map(|e| match e.trace.metric(key) {
                        Some(i) => values(e.trace, e.seat, i, last_turn)
                            .into_iter()
                            .map(|v| v.map(|v| v as f64))
                            .collect(),
                        None => vec![None; last_turn as usize],
                    })
                    .collect()
            })
            .collect();
        set.curves.push(build_curve(
            key,
            meaning,
            P::lower_is_better(key),
            false,
            &columns,
            last_turn,
            either,
        ));
    }
    if events {
        let by_entry: Vec<Vec<Vec<Option<Vec<String>>>>> = groups
            .iter()
            .map(|g| g.iter().map(|e| event_turns::<P>(e, last_turn)).collect())
            .collect();
        let names: std::collections::BTreeSet<&String> = by_entry
            .iter()
            .flatten()
            .flatten()
            .flatten()
            .flatten()
            .collect();
        for name in names {
            let columns: Vec<Vec<Vec<Option<f64>>>> = by_entry
                .iter()
                .map(|g| {
                    g.iter()
                        .map(|turns| {
                            turns
                                .iter()
                                .map(|t| t.as_ref().map(|v| f64::from(u8::from(v.contains(name)))))
                                .collect()
                        })
                        .collect()
                })
                .collect();
            set.events.push(build_curve(
                name,
                "the share of the games in which the side does it at that turn, in percent."
                    .to_string(),
                false,
                true,
                &columns,
                last_turn,
                either,
            ));
        }
    }
    set
}

/// The sets of curves from the data groups: us and the focus players in the
/// games between them, our wins and losses, and each focus player.
pub(super) fn curves<'a, P: Profiling>(groups: &[Group<'a>]) -> Curves {
    fn side<'a>(group: &Group<'a>, i: usize) -> Option<Vec<Entry<'a>>> {
        group.sides.get(i).map(|s| s.1.clone())
    }
    let kind = |k: Kind| groups.iter().find(|g| g.kind == k);
    let pair = |g: Option<&Group<'a>>| -> [Vec<Entry<'a>>; 2] {
        g.map_or([Vec::new(), Vec::new()], |g| {
            [
                side(g, 0).unwrap_or_default(),
                side(g, 1).unwrap_or_default(),
            ]
        })
    };
    let wins = kind(Kind::Wins)
        .and_then(|g| side(g, 0))
        .unwrap_or_default();
    let losses = kind(Kind::Losses)
        .and_then(|g| side(g, 0))
        .unwrap_or_default();
    let turns = P::compare_turns();
    let wins_vs_losses = curve_set::<P>(
        "wins-vs-losses",
        ["our wins", "our losses"],
        false,
        [wins, losses],
        None,
        true,
    );
    // Each focus player: the three metrics that tell our wins from our
    // losses best.
    let best: Vec<String> = wins_vs_losses
        .compare(&turns)
        .rows
        .into_iter()
        .filter(|r| r.score.is_some())
        .take(PLAYER_METRICS)
        .map(|r| r.key)
        .collect();
    let players = groups
        .iter()
        .filter(|g| g.kind == Kind::Player)
        .map(|g| {
            curve_set::<P>(
                &format!("player-{}", super::chart::slug(&g.label)),
                ["us", &g.label],
                true,
                pair(Some(g)),
                Some(&best),
                false,
            )
        })
        .collect();
    Curves {
        compare_turns: turns,
        top_vs_us: curve_set::<P>(
            "top-vs-us",
            ["us", "top players"],
            true,
            pair(kind(Kind::All)),
            None,
            true,
        ),
        wins_vs_losses,
        players,
    }
}

/// The metrics drawn for each focus player.
const PLAYER_METRICS: usize = 3;

impl CurveSet {
    /// The metrics ranked by how well they tell the groups apart at `turns`
    /// (those the set reaches): the mean over those turns of the AUC (with
    /// `either`, of whichever of the AUC and its complement is larger),
    /// highest first, a metric with no AUC last.
    pub fn compare(&self, turns: &[u32]) -> Comparison {
        let mut rows: Vec<SepRow> = self
            .curves
            .iter()
            .map(|c| {
                let at: Vec<SepCell> = turns
                    .iter()
                    .filter(|&&t| t >= 1 && t <= self.last_turn)
                    .map(|&turn| {
                        let i = turn as usize - 1;
                        SepCell {
                            turn,
                            auc: c.auc[i],
                            first: c.lines[0][i].median,
                            second: c.lines[1][i].median,
                            first_games: c.lines[0][i].games,
                            second_games: c.lines[1][i].games,
                        }
                    })
                    .collect();
                let strengths: Vec<f64> = at
                    .iter()
                    .filter_map(|cell| cell.auc)
                    .map(|a| if self.either { a.max(1.0 - a) } else { a })
                    .collect();
                SepRow {
                    key: c.key.clone(),
                    lower_is_better: c.lower_is_better,
                    score: mean(&strengths),
                    at,
                    separates_from: c.separates_from,
                }
            })
            .collect();
        // Stable: equal scores keep the order of the key metrics.
        rows.sort_by(|a, b| match (a.score, b.score) {
            (Some(x), Some(y)) => y.total_cmp(&x),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });
        Comparison {
            slug: self.slug.clone(),
            labels: self.labels.clone(),
            games: self.games,
            either: self.either,
            rows,
        }
    }
}

#[cfg(test)]
mod tests;
