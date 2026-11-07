//! Profiles of the players we meet on CodinGame, and our own weaknesses,
//! computed from traces alone (ADR 0056, decisions 7 and 8).
//!
//! A game provides the [`Profiling`] hooks (which metrics matter, what the
//! events of a ply are, what an opening looks like); everything else is
//! generic: every table is built from the trace's metric keys, the hooks'
//! event names and the trace's moments and phases, never from a game's
//! names. [`analyze`] computes an [`Analysis`] (which serializes to the
//! JSON of `--json`, see below) and [`markdown`] renders it.
//!
//! # The JSON
//!
//! `Analysis` has the five sections of the report: `sources` (games by
//! kind and batch, the leaderboard snapshot, the games left out),
//! `opponents` (one row per opponent of ours), `profiles` (one per focus
//! player: results by seat, `metrics` at game turns and by phase, `events`,
//! `moments`, `openings`, `wins`), `weaknesses` (our losses) and `games`
//! (game ids per opponent). Medians and means are `null` when there is no
//! data; counts of games sit beside every number (`games`, `n`).
//!
//! A side's metrics "at turn t" are read right after its own ply of turn t:
//! in the snapshot of the ply that follows it in the trace (a ply's snapshot
//! is taken after the player to act's start of turn, so the next ply, the
//! other side's, shows the first side's values as its play left them), or,
//! for a side's last ply, in that ply's own snapshot. The turn medians,
//! "stays below to the end" and the lead list all read so. The phase means
//! are different: they average each side's values at the starts of its own
//! plies in the phase.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use crate::trace::{Phase, Trace, SIDES};

pub(crate) mod render;
pub use render::markdown;

/// Samples smaller than this are flagged in the report.
pub const SMALL: usize = 5;
/// A player with fewer games than this is shown with a note.
pub const FEW_GAMES: usize = 3;
/// The openings a profile lists.
pub const OPENINGS_SHOWN: usize = 3;
/// The losses listed as the earliest to fall behind for good.
pub const EARLIEST_SHOWN: usize = 5;

/// What a game provides to profile its players.
pub trait Profiling {
    /// The game turns at which a profile compares metrics, increasing (a
    /// 20-turn game and a 200-turn game choose differently).
    fn turns() -> Vec<u32>;
    /// The own plies an opening covers.
    fn opening_plies() -> usize;
    /// The metrics a profile shows, by key, in order: a short list (about 8
    /// to 12).
    fn key_metrics() -> Vec<&'static str>;
    /// The events of one ply of `trace` (the player to act's), by name,
    /// for example "tower".
    fn events(trace: &Trace, ply: usize) -> Vec<String>;
    /// The first `plies` own plies of `seat`, one token string a ply, the
    /// same from both seats (a game with a symmetric board maps the second
    /// seat's cells onto the first's).
    fn opening(trace: &Trace, seat: usize, plies: usize) -> Vec<String>;
}

/// What the report must know about a game besides its trace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameMeta {
    pub game_id: u64,
    /// How it was played: `ranking`, `test`, `replay`.
    pub kind: String,
    pub batch: Option<String>,
    /// Our seat, or `None` when we are not a player.
    pub our_seat: Option<usize>,
}

/// A game that could not be used, and why.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LeftOut {
    pub game_id: Option<u64>,
    pub why: String,
}

/// Everything the analysis reads.
#[derive(Clone, Debug, Default)]
pub struct Input {
    pub games: Vec<(GameMeta, Trace)>,
    /// The rank of each player name in the leaderboard snapshot.
    pub ranks: BTreeMap<String, u32>,
    /// The snapshot's name, for the report.
    pub snapshot: Option<String>,
    pub left_out: Vec<LeftOut>,
    /// Things to tell the reader before the numbers (for example a name
    /// that does not match).
    pub warnings: Vec<String>,
    /// What the game says about how its traces were made (for example
    /// which commands they hold).
    pub notes: Vec<String>,
}

/// Which players get a profile.
#[derive(Clone, Debug, Default)]
pub struct Options {
    /// The top `top` of the snapshot that have a game.
    pub top: usize,
    /// These players instead, when not empty.
    pub players: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Tally {
    pub won: usize,
    pub lost: usize,
    pub drawn: usize,
}

impl Tally {
    pub fn games(&self) -> usize {
        self.won + self.lost + self.drawn
    }
    fn add(&mut self, result: Option<bool>) {
        match result {
            Some(true) => self.won += 1,
            Some(false) => self.lost += 1,
            None => self.drawn += 1,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct KindCount {
    pub kind: String,
    pub batch: Option<String>,
    pub games: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Sources {
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub by_kind: Vec<KindCount>,
    pub snapshot: Option<String>,
    pub left_out: Vec<LeftOut>,
}

/// One opponent of ours; the tallies are ours.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpponentRow {
    pub player: String,
    pub rank: Option<u32>,
    pub games: usize,
    pub ours: Tally,
    /// Ours when we sat in seat 0 and in seat 1.
    pub by_seat: [Tally; 2],
    pub median_turns: Option<f64>,
    pub ends: BTreeMap<String, usize>,
}

/// A metric at one game turn: medians over the games that reached it.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TurnCell {
    pub turn: u32,
    pub games: usize,
    pub theirs: Option<f64>,
    pub our_games: usize,
    pub ours: Option<f64>,
}

/// A metric in one phase: the mean over games of the game's mean.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PhaseCell {
    pub phase: String,
    pub games: usize,
    pub theirs: Option<f64>,
    pub our_games: usize,
    pub ours: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricRow {
    pub key: String,
    pub at_turn: Vec<TurnCell>,
    pub by_phase: Vec<PhaseCell>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EventRow {
    pub name: String,
    /// Their games, and those in which it happened for them.
    pub games: usize,
    pub theirs_in: usize,
    /// Games against us, and those in which it happened for us.
    pub our_games: usize,
    pub ours_in: usize,
    /// The median game turn of their first time.
    pub first_turn: Option<f64>,
    pub mean_theirs: Option<f64>,
    pub mean_ours: Option<f64>,
}

/// Games with at least one moment, out of `games`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Share {
    pub with: usize,
    pub games: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MomentRow {
    pub kind: String,
    /// `them` or `us`: whom the moments favoured.
    pub favours: String,
    pub in_wins: Share,
    pub in_losses: Share,
    pub total: usize,
    pub median_turn: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OpeningRow {
    pub opening: String,
    pub games: usize,
    pub wins: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FirstMoment {
    pub kind: String,
    pub wins: usize,
    pub median_turn: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Wins {
    pub wins: usize,
    pub ends: BTreeMap<String, usize>,
    pub first_moments: Vec<FirstMoment>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Profile {
    pub player: String,
    pub rank: Option<u32>,
    pub games: usize,
    pub few_games: bool,
    /// The own plies each opening covers.
    pub opening_plies: usize,
    /// The wanted turns that fewer than half of the games reach, left out
    /// of `metrics`' `at_turn`.
    pub turns_dropped: Vec<u32>,
    /// Their results against us, by their seat.
    pub vs_us: [Tally; 2],
    pub metrics: Vec<MetricRow>,
    pub events: Vec<EventRow>,
    pub moments: Vec<MomentRow>,
    pub openings: Vec<OpeningRow>,
    pub wins: Wins,
}

/// Moments against us in our losses, for one kind and group of opponents.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AgainstUs {
    pub kind: String,
    /// `focus` or `others`.
    pub group: String,
    /// Moments in the early, middle and endgame phases.
    pub by_phase: [usize; 3],
}

/// A metric of ours that stays below the opponent's to the end.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StaysBelow {
    pub key: String,
    pub losses: usize,
    pub below: usize,
    pub median_turn: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Earliest {
    pub game_id: u64,
    pub opponent: String,
    pub turn: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Weaknesses {
    pub losses: usize,
    /// Losses against focus players.
    pub focus_losses: usize,
    pub against_us: Vec<AgainstUs>,
    pub stays_below: Vec<StaysBelow>,
    pub lead: String,
    pub earliest: Vec<Earliest>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct GameList {
    pub label: String,
    /// Game id and a result letter: ours (W, L, D), or the winner's side
    /// (its first letter) when we are not a player.
    pub games: Vec<(u64, String)>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Analysis {
    pub sources: Sources,
    pub opponents: Vec<OpponentRow>,
    pub profiles: Vec<Profile>,
    pub weaknesses: Weaknesses,
    pub games: Vec<GameList>,
}

// ---- helpers

pub(crate) fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let n = sorted.len();
    Some(if n % 2 == 1 {
        sorted[n / 2]
    } else {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    })
}

pub(crate) fn mean(values: &[f64]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}

/// The last game turn of `trace`.
fn length(trace: &Trace) -> u32 {
    trace.plies.last().map_or(0, |p| p.game_turn)
}

/// The value of metric `index` for `seat` at game turn `turn`: right after
/// its own ply of that turn, that is in the snapshot of the next ply of the
/// trace (that ply's start of turn changes only the other side's values),
/// or in the side's own snapshot if it is the trace's last ply. `None` if
/// the side has no ply in `turn`, or the value is the metric's `none`.
fn value_at(trace: &Trace, seat: usize, turn: u32, index: usize) -> Option<i64> {
    let own = trace
        .plies
        .iter()
        .position(|p| p.to_act == seat && p.game_turn == turn)?;
    let ply = trace.plies.get(own + 1).unwrap_or(&trace.plies[own]);
    let value = *ply.metrics[seat].get(index)?;
    (trace.metrics[index].none != Some(value)).then_some(value)
}

/// The earliest game turn from which `seat`'s value of `key` stays below
/// the other seat's to the last turn at which both have one; `None` if it
/// is not below at that last turn.
pub fn stays_below(trace: &Trace, seat: usize, key: &str) -> Option<u32> {
    let index = trace.metric(key)?;
    let mut from = None;
    for turn in (1..=length(trace)).rev() {
        let (Some(mine), Some(theirs)) = (
            value_at(trace, seat, turn, index),
            value_at(trace, 1 - seat, turn, index),
        ) else {
            continue;
        };
        if mine < theirs {
            from = Some(turn);
        } else {
            break;
        }
    }
    from
}

/// The result of `seat` in `trace`: `Some(true)` for a win.
fn result(trace: &Trace, seat: usize) -> Option<bool> {
    trace.winner.map(|w| w == seat)
}

fn seat_of(trace: &Trace, player: &str) -> Option<usize> {
    trace.players.iter().position(|p| p == player)
}

/// Our seat and the opponent's name, when we are a player.
pub(crate) fn versus(meta: &GameMeta, trace: &Trace) -> Option<(usize, String)> {
    let us = meta.our_seat.filter(|&s| s < 2)?;
    Some((us, trace.players[1 - us].clone()))
}

/// The game turn of `ply`, clamped to the trace.
fn turn_of(trace: &Trace, ply: usize) -> u32 {
    trace
        .plies
        .get(ply)
        .or(trace.plies.last())
        .map_or(0, |p| p.game_turn)
}

type Counts = BTreeMap<String, (usize, u32)>;

/// The events of `seat`'s plies: the count and the first game turn of each.
fn event_counts<P: Profiling>(trace: &Trace, seat: usize) -> Counts {
    let mut out = Counts::new();
    for (ply, p) in trace.plies.iter().enumerate() {
        if p.to_act != seat {
            continue;
        }
        for name in P::events(trace, ply) {
            out.entry(name).or_insert((0, p.game_turn)).0 += 1;
        }
    }
    out
}

fn phase_index(phase: Phase) -> usize {
    match phase {
        Phase::Early => 0,
        Phase::Middle => 1,
        Phase::End => 2,
    }
}

// ---- the analysis

/// Computes the report's numbers.
pub fn analyze<P: Profiling>(input: &Input, options: &Options) -> Analysis {
    let games: Vec<&(GameMeta, Trace)> = input.games.iter().collect();
    let focus = focus_players(&games, &input.ranks, options);
    let profiles = focus
        .iter()
        .map(|name| profile::<P>(name, &games, &input.ranks))
        .collect();
    Analysis {
        sources: sources(input),
        opponents: opponents(&games, &input.ranks),
        profiles,
        weaknesses: weaknesses::<P>(&games, &focus),
        games: game_lists(&games, &input.ranks),
    }
}

pub(crate) fn sources(input: &Input) -> Sources {
    let mut counts: BTreeMap<(String, Option<String>), usize> = BTreeMap::new();
    for (meta, _) in &input.games {
        *counts
            .entry((meta.kind.clone(), meta.batch.clone()))
            .or_default() += 1;
    }
    Sources {
        by_kind: counts
            .into_iter()
            .map(|((kind, batch), games)| KindCount { kind, batch, games })
            .collect(),
        snapshot: input.snapshot.clone(),
        left_out: input.left_out.clone(),
        warnings: input.warnings.clone(),
        notes: input.notes.clone(),
    }
}

/// Sorted by rank, unranked last, then by name.
fn rank_key(rank: Option<u32>, name: &str) -> (u32, String) {
    (rank.unwrap_or(u32::MAX), name.to_string())
}

fn opponents(games: &[&(GameMeta, Trace)], ranks: &BTreeMap<String, u32>) -> Vec<OpponentRow> {
    let mut rows: BTreeMap<String, OpponentRow> = BTreeMap::new();
    let mut turns: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    for (meta, trace) in games {
        let Some((us, name)) = versus(meta, trace) else {
            continue;
        };
        let row = rows.entry(name.clone()).or_insert_with(|| OpponentRow {
            rank: ranks.get(&name).copied(),
            player: name.clone(),
            games: 0,
            ours: Tally::default(),
            by_seat: [Tally::default(), Tally::default()],
            median_turns: None,
            ends: BTreeMap::new(),
        });
        row.games += 1;
        row.ours.add(result(trace, us));
        row.by_seat[us].add(result(trace, us));
        *row.ends.entry(trace.end.clone()).or_default() += 1;
        turns
            .entry(name)
            .or_default()
            .push(f64::from(length(trace)));
    }
    let mut rows: Vec<OpponentRow> = rows
        .into_values()
        .map(|mut row| {
            row.median_turns = median(&turns[&row.player]);
            row
        })
        .collect();
    rows.sort_by_key(|r| rank_key(r.rank, &r.player));
    rows
}

pub fn focus_players(
    games: &[&(GameMeta, Trace)],
    ranks: &BTreeMap<String, u32>,
    options: &Options,
) -> Vec<String> {
    if !options.players.is_empty() {
        return options.players.clone();
    }
    let ours: BTreeSet<&str> = games
        .iter()
        .filter_map(|(meta, trace)| Some(trace.players[meta.our_seat.filter(|&s| s < 2)?].as_str()))
        .collect();
    let played: BTreeSet<&str> = games
        .iter()
        .flat_map(|(_, trace)| trace.players.iter().map(String::as_str))
        .collect();
    let mut ranked: Vec<(&u32, &String)> = ranks.iter().map(|(n, r)| (r, n)).collect();
    ranked.sort();
    ranked
        .into_iter()
        .filter(|(_, n)| played.contains(n.as_str()) && !ours.contains(n.as_str()))
        .take(options.top)
        .map(|(_, n)| n.clone())
        .collect()
}

/// One game of a focus player.
struct Mine<'a> {
    trace: &'a Trace,
    /// Their seat, and ours if we are the other player.
    seat: usize,
    us: Option<usize>,
}

fn profile<P: Profiling>(
    player: &str,
    games: &[&(GameMeta, Trace)],
    ranks: &BTreeMap<String, u32>,
) -> Profile {
    let mine: Vec<Mine> = games
        .iter()
        .filter_map(|(meta, trace)| {
            let seat = seat_of(trace, player)?;
            let us = meta.our_seat.filter(|&s| s < 2 && s != seat);
            Some(Mine { trace, seat, us })
        })
        .collect();
    let mut vs_us = [Tally::default(), Tally::default()];
    for g in mine.iter().filter(|g| g.us.is_some()) {
        vs_us[g.seat].add(result(g.trace, g.seat));
    }
    let metrics = metric_rows::<P>(&mine);
    Profile {
        player: player.to_string(),
        rank: ranks.get(player).copied(),
        games: mine.len(),
        few_games: mine.len() < FEW_GAMES,
        opening_plies: P::opening_plies(),
        turns_dropped: P::turns()
            .into_iter()
            .filter(|t| {
                !metrics
                    .iter()
                    .any(|m| m.at_turn.iter().any(|c| c.turn == *t))
            })
            .collect(),
        vs_us,
        metrics,
        events: event_rows::<P>(&mine),
        moments: moment_rows(&mine),
        openings: opening_rows::<P>(&mine),
        wins: win_rows(&mine),
    }
}

/// The mean of `seat`'s values of metric `index` over its plies in `phase`.
fn phase_mean(trace: &Trace, seat: usize, index: usize, phase: usize) -> Option<f64> {
    let values: Vec<f64> = trace
        .plies
        .iter()
        .enumerate()
        .filter(|(ply, p)| p.to_act == seat && phase_index(trace.phase(*ply)) == phase)
        .filter_map(|(_, p)| p.metrics[seat].get(index).copied())
        .filter(|&v| trace.metrics[index].none != Some(v))
        .map(|v| v as f64)
        .collect();
    mean(&values)
}

fn metric_rows<P: Profiling>(mine: &[Mine]) -> Vec<MetricRow> {
    let mut rows = Vec::new();
    for key in P::key_metrics() {
        let mut at_turn = Vec::new();
        for turn in P::turns() {
            let (mut theirs, mut ours) = (Vec::new(), Vec::new());
            for g in mine {
                let Some(index) = g.trace.metric(key) else {
                    continue;
                };
                if let Some(v) = value_at(g.trace, g.seat, turn, index) {
                    theirs.push(v as f64);
                }
                if let Some(v) = g.us.and_then(|us| value_at(g.trace, us, turn, index)) {
                    ours.push(v as f64);
                }
            }
            if !theirs.is_empty() && theirs.len() * 2 >= mine.len() {
                at_turn.push(TurnCell {
                    turn,
                    games: theirs.len(),
                    theirs: median(&theirs),
                    our_games: ours.len(),
                    ours: median(&ours),
                });
            }
        }
        let mut by_phase = Vec::new();
        for (i, phase) in [Phase::Early, Phase::Middle, Phase::End]
            .into_iter()
            .enumerate()
        {
            let (mut theirs, mut ours) = (Vec::new(), Vec::new());
            for g in mine {
                let Some(index) = g.trace.metric(key) else {
                    continue;
                };
                theirs.extend(phase_mean(g.trace, g.seat, index, i));
                if let Some(us) = g.us {
                    ours.extend(phase_mean(g.trace, us, index, i));
                }
            }
            by_phase.push(PhaseCell {
                phase: phase.name().to_string(),
                games: theirs.len(),
                theirs: mean(&theirs),
                our_games: ours.len(),
                ours: mean(&ours),
            });
        }
        rows.push(MetricRow {
            key: key.to_string(),
            at_turn,
            by_phase,
        });
    }
    rows
}

fn event_rows<P: Profiling>(mine: &[Mine]) -> Vec<EventRow> {
    let theirs: Vec<Counts> = mine
        .iter()
        .map(|g| event_counts::<P>(g.trace, g.seat))
        .collect();
    let ours: Vec<Counts> = mine
        .iter()
        .filter_map(|g| g.us.map(|us| event_counts::<P>(g.trace, us)))
        .collect();
    let names: BTreeSet<&String> = theirs.iter().chain(&ours).flat_map(|m| m.keys()).collect();
    let mut rows: Vec<EventRow> = names
        .into_iter()
        .map(|name| {
            let firsts: Vec<f64> = theirs
                .iter()
                .filter_map(|m| m.get(name).map(|&(_, t)| f64::from(t)))
                .collect();
            let count = |all: &[Counts]| -> Vec<f64> {
                all.iter()
                    .map(|m| m.get(name).map_or(0.0, |&(c, _)| c as f64))
                    .collect()
            };
            let counts_ours = count(&ours);
            EventRow {
                name: name.clone(),
                games: mine.len(),
                theirs_in: firsts.len(),
                our_games: ours.len(),
                ours_in: counts_ours.iter().filter(|&&c| c > 0.0).count(),
                first_turn: median(&firsts),
                mean_theirs: mean(&count(&theirs)),
                mean_ours: mean(&counts_ours),
            }
        })
        .collect();
    rows.sort_by(|a, b| b.theirs_in.cmp(&a.theirs_in).then(a.name.cmp(&b.name)));
    rows
}

fn moment_rows(mine: &[Mine]) -> Vec<MomentRow> {
    let kinds: BTreeSet<&str> = mine
        .iter()
        .flat_map(|g| g.trace.moments.iter())
        .filter(|m| m.favours.is_some())
        .map(|m| m.kind.as_str())
        .collect();
    let mut rows = Vec::new();
    for kind in kinds {
        for who in ["them", "us"] {
            let (mut in_wins, mut in_losses) = (Share::default(), Share::default());
            let (mut total, mut turns) = (0, Vec::new());
            for g in mine {
                let side = if who == "them" { Some(g.seat) } else { g.us };
                let Some(side) = side else { continue };
                // Wins and losses are theirs.
                let share = match result(g.trace, g.seat) {
                    Some(true) => &mut in_wins,
                    Some(false) => &mut in_losses,
                    None => continue,
                };
                share.games += 1;
                let found: Vec<f64> = g
                    .trace
                    .moments
                    .iter()
                    .filter(|m| m.kind == kind && m.favours == Some(side))
                    .map(|m| f64::from(turn_of(g.trace, m.ply)))
                    .collect();
                if !found.is_empty() {
                    share.with += 1;
                }
                total += found.len();
                turns.extend(found);
            }
            rows.push(MomentRow {
                kind: kind.to_string(),
                favours: who.to_string(),
                in_wins,
                in_losses,
                total,
                median_turn: median(&turns),
            });
        }
    }
    rows
}

fn opening_rows<P: Profiling>(mine: &[Mine]) -> Vec<OpeningRow> {
    let mut counts: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for g in mine {
        let opening = P::opening(g.trace, g.seat, P::opening_plies()).join(" / ");
        let entry = counts.entry(opening).or_default();
        entry.0 += 1;
        if result(g.trace, g.seat) == Some(true) {
            entry.1 += 1;
        }
    }
    let mut rows: Vec<OpeningRow> = counts
        .into_iter()
        .map(|(opening, (games, wins))| OpeningRow {
            opening,
            games,
            wins,
        })
        .collect();
    rows.sort_by(|a, b| b.games.cmp(&a.games).then(a.opening.cmp(&b.opening)));
    rows.truncate(OPENINGS_SHOWN);
    rows
}

fn win_rows(mine: &[Mine]) -> Wins {
    let mut wins = Wins::default();
    let mut firsts: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    for g in mine {
        if result(g.trace, g.seat) != Some(true) {
            continue;
        }
        wins.wins += 1;
        *wins.ends.entry(g.trace.end.clone()).or_default() += 1;
        let first = g
            .trace
            .moments
            .iter()
            .filter(|m| m.favours == Some(g.seat))
            .min_by_key(|m| m.ply);
        if let Some(m) = first {
            firsts
                .entry(m.kind.clone())
                .or_default()
                .push(f64::from(turn_of(g.trace, m.ply)));
        }
    }
    let mut moments: Vec<FirstMoment> = firsts
        .into_iter()
        .map(|(kind, turns)| FirstMoment {
            kind,
            wins: turns.len(),
            median_turn: median(&turns),
        })
        .collect();
    moments.sort_by(|a, b| b.wins.cmp(&a.wins).then(a.kind.cmp(&b.kind)));
    wins.first_moments = moments;
    wins
}

fn weaknesses<P: Profiling>(games: &[&(GameMeta, Trace)], focus: &[String]) -> Weaknesses {
    // Our losses: our seat, the opponent, the trace, the game's id.
    let losses: Vec<(usize, String, &Trace, u64)> = games
        .iter()
        .filter_map(|(meta, trace)| {
            let (us, opp) = versus(meta, trace)?;
            (trace.winner == Some(1 - us)).then_some((us, opp, trace, meta.game_id))
        })
        .collect();
    let in_focus = |opp: &str| focus.iter().any(|f| f == opp);
    let mut against: BTreeMap<(String, bool), [usize; 3]> = BTreeMap::new();
    for (us, opp, trace, _) in &losses {
        for m in trace.moments.iter().filter(|m| m.favours == Some(1 - us)) {
            let cell = against.entry((m.kind.clone(), in_focus(opp))).or_default();
            cell[phase_index(trace.phase(m.ply))] += 1;
        }
    }
    let against_us = against
        .into_iter()
        .map(|((kind, focused), by_phase)| AgainstUs {
            kind,
            group: if focused { "focus" } else { "others" }.to_string(),
            by_phase,
        })
        .collect();
    let stays = P::key_metrics()
        .into_iter()
        .map(|key| {
            let from: Vec<f64> = losses
                .iter()
                .filter_map(|(us, _, trace, _)| stays_below(trace, *us, key).map(f64::from))
                .collect();
            StaysBelow {
                key: key.to_string(),
                losses: losses.len(),
                below: from.len(),
                median_turn: median(&from),
            }
        })
        .collect();
    let mut earliest: Vec<Earliest> = losses
        .iter()
        .filter_map(|(us, opp, trace, game_id)| {
            Some(Earliest {
                game_id: *game_id,
                opponent: opp.clone(),
                turn: stays_below(trace, *us, &trace.lead)?,
            })
        })
        .collect();
    earliest.sort_by_key(|e| (e.turn, e.game_id));
    earliest.truncate(EARLIEST_SHOWN);
    Weaknesses {
        losses: losses.len(),
        focus_losses: losses.iter().filter(|(_, o, _, _)| in_focus(o)).count(),
        against_us,
        stays_below: stays,
        lead: losses
            .first()
            .map(|(_, _, trace, _)| trace.lead.clone())
            .unwrap_or_default(),
        earliest,
    }
}

fn game_lists(games: &[&(GameMeta, Trace)], ranks: &BTreeMap<String, u32>) -> Vec<GameList> {
    let mut lists: BTreeMap<String, Vec<(u64, String)>> = BTreeMap::new();
    for (meta, trace) in games {
        let (label, letter) = match versus(meta, trace) {
            Some((us, opp)) => {
                let letter = match result(trace, us) {
                    Some(true) => "W",
                    Some(false) => "L",
                    None => "D",
                };
                (opp, letter.to_string())
            }
            None => (
                format!("{} vs {} (not ours)", trace.players[0], trace.players[1]),
                trace.winner.map_or("d".to_string(), |w| {
                    SIDES[w].chars().take(1).collect::<String>()
                }),
            ),
        };
        lists.entry(label).or_default().push((meta.game_id, letter));
    }
    let mut lists: Vec<GameList> = lists
        .into_iter()
        .map(|(label, games)| GameList { label, games })
        .collect();
    lists.sort_by_key(|l| rank_key(ranks.get(&l.label).copied(), &l.label));
    lists
}

#[cfg(test)]
mod tests;
