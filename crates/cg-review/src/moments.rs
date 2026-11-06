//! The decisive moment of each of our losses: the first moment against us
//! of the game's decisive kinds, with the board before and after it, how
//! exposed we were before it, and what each side held then (ADR 0056).
//!
//! A game provides the [`Decisive`] hooks; everything else is generic.
//! [`analyze`] computes [`Findings`] and [`markdown`] renders them.
//!
//! A side's metrics "before" a moment are read right after its own last ply
//! before the moment's ply, as in [`crate::profile`]: in the snapshot of the
//! ply that follows it in the trace, which shows the side's values as its
//! play left them. The moment's ply is the one whose play caused it, so its
//! own snapshot is the board before the play and the next ply's snapshot is
//! the board after it.

use std::collections::BTreeSet;

use crate::profile::{
    focus_players, median, sources, versus, GameMeta, Input, Options, Profiling, Sources,
};
use crate::trace::Trace;

mod render;
pub use render::markdown;

/// The own plies before a moment whose exposure is read.
pub const EXPOSURE_PLIES: usize = 3;

/// What a game provides to find the decisive moments of our losses, besides
/// the key metrics of [`Profiling`].
pub trait Decisive: Profiling {
    /// The kinds of moments that decide a game, in order of interest.
    fn decisive_kinds() -> Vec<&'static str>;
    /// The key of the metric that says how much of a side the opponent can
    /// take next (higher is worse), if the game has one.
    fn exposure_metric() -> Option<&'static str>;
}

/// The decisive moment of one loss.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Found {
    pub kind: String,
    pub ply: usize,
    pub turn: u32,
    pub text: String,
    /// What the opponent answered at that ply.
    pub commands: String,
    /// The board of the ply, before its play.
    pub before: Vec<String>,
    /// The board of the next ply, or `None` when the moment is on the
    /// trace's last ply.
    pub after: Option<Vec<String>>,
    /// The key metrics, in the order of [`Findings::metrics`], read right
    /// after each side's last own ply before the moment; `None` when the
    /// side had no ply yet or the value means "none".
    pub theirs: Vec<Option<i64>>,
    pub ours: Vec<Option<i64>>,
    /// Our exposure right after our last, second to last, ... own ply
    /// before the moment ([`EXPOSURE_PLIES`] values).
    pub exposure: Vec<Option<i64>>,
}

/// One of our losses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Loss {
    pub game_id: u64,
    pub opponent: String,
    pub rank: Option<u32>,
    pub focus: bool,
    pub our_seat: usize,
    /// The last game turn.
    pub turns: u32,
    pub end: String,
    pub found: Option<Found>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Findings {
    pub sources: Sources,
    /// The games used.
    pub games: usize,
    pub kinds: Vec<String>,
    pub metrics: Vec<String>,
    pub exposure_metric: Option<String>,
    /// The focus players, by rank.
    pub focus: Vec<String>,
    /// The first trace's primer, to read the boards.
    pub primer: Vec<String>,
    /// Examples per focus player in the report.
    pub examples: usize,
    pub losses: Vec<Loss>,
}

/// The value of metric `index` for `seat` after its own ply `own`.
fn value_after(trace: &Trace, seat: usize, own: usize, index: usize) -> Option<i64> {
    let snapshot = trace.plies.get(own + 1).unwrap_or(&trace.plies[own]);
    let value = *snapshot.metrics[seat].get(index)?;
    (trace.metrics[index].none != Some(value)).then_some(value)
}

/// The last `n` plies of `seat` before `ply`, latest first.
fn own_plies_before(trace: &Trace, seat: usize, ply: usize, n: usize) -> Vec<usize> {
    (0..ply.min(trace.plies.len()))
        .rev()
        .filter(|&q| trace.plies[q].to_act == seat)
        .take(n)
        .collect()
}

fn find<P: Decisive>(trace: &Trace, us: usize, kinds: &[&str], keys: &[&str]) -> Option<Found> {
    let rank = |kind: &str| kinds.iter().position(|k| *k == kind);
    let moment = trace
        .moments
        .iter()
        .filter(|m| m.favours == Some(1 - us) && m.ply < trace.plies.len())
        .filter_map(|m| Some((m, rank(&m.kind)?)))
        .min_by_key(|(m, r)| (m.ply, *r))?
        .0;
    let ply = &trace.plies[moment.ply];
    let read = |seat: usize| -> Vec<Option<i64>> {
        let last = own_plies_before(trace, seat, moment.ply, 1)
            .first()
            .copied();
        keys.iter()
            .map(|k| value_after(trace, seat, last?, trace.metric(k)?))
            .collect()
    };
    let exposure_index = P::exposure_metric().and_then(|k| trace.metric(k));
    let mut exposure: Vec<Option<i64>> = own_plies_before(trace, us, moment.ply, EXPOSURE_PLIES)
        .into_iter()
        .map(|own| exposure_index.and_then(|i| value_after(trace, us, own, i)))
        .collect();
    exposure.resize(EXPOSURE_PLIES, None);
    Some(Found {
        kind: moment.kind.clone(),
        ply: moment.ply,
        turn: ply.game_turn,
        text: moment.text.clone(),
        commands: ply.commands.clone(),
        before: ply.board.clone(),
        after: trace.plies.get(moment.ply + 1).map(|p| p.board.clone()),
        theirs: read(1 - us),
        ours: read(us),
        exposure,
    })
}

/// Finds the decisive moment of each of our losses.
pub fn analyze<P: Decisive>(input: &Input, options: &Options, examples: usize) -> Findings {
    let games: Vec<&(GameMeta, Trace)> = input.games.iter().collect();
    let focus = focus_players(&games, &input.ranks, options);
    let kinds = P::decisive_kinds();
    let keys = P::key_metrics();
    let mut losses = Vec::new();
    for (meta, trace) in &games {
        let Some((us, opponent)) = versus(meta, trace) else {
            continue;
        };
        if trace.winner != Some(1 - us) {
            continue;
        }
        losses.push(Loss {
            game_id: meta.game_id,
            rank: input.ranks.get(&opponent).copied(),
            focus: focus.contains(&opponent),
            opponent,
            our_seat: us,
            turns: trace.plies.last().map_or(0, |p| p.game_turn),
            end: trace.end.clone(),
            found: find::<P>(trace, us, &kinds, &keys),
        });
    }
    losses.sort_by_key(|l| l.game_id);
    Findings {
        sources: sources(input),
        games: games.len(),
        kinds: kinds.iter().map(|k| (*k).to_string()).collect(),
        metrics: keys.iter().map(|k| (*k).to_string()).collect(),
        exposure_metric: P::exposure_metric().map(str::to_string),
        focus,
        primer: games
            .first()
            .map(|(_, t)| t.primer.clone())
            .unwrap_or_default(),
        examples,
        losses,
    }
}

impl Findings {
    /// The kinds that at least one loss has, in the order of `kinds`.
    pub fn kinds_found(&self) -> Vec<&str> {
        let seen: BTreeSet<&str> = self
            .losses
            .iter()
            .filter_map(|l| l.found.as_ref().map(|f| f.kind.as_str()))
            .collect();
        self.kinds
            .iter()
            .map(String::as_str)
            .filter(|k| seen.contains(k))
            .collect()
    }
}

/// The median game turn of the moments `founds`.
pub(crate) fn median_turn<'a>(founds: impl Iterator<Item = &'a Found>) -> Option<f64> {
    let turns: Vec<f64> = founds.map(|f| f64::from(f.turn)).collect();
    median(&turns)
}

#[cfg(test)]
mod tests;
