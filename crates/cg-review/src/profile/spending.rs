//! Spending: what a side spends of its resource (gold) each own turn, by
//! category and phase, and the share it leaves unspent.

use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use super::{mean, phase_index, result, versus, GameMeta, Profiling};
use crate::trace::{Phase, Trace};

/// One phase of a group.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpendCell {
    pub phase: String,
    /// Own plies in the phase, over all games of the group.
    pub plies: usize,
    /// The mean spent per own ply on each category of [`Spending::categories`].
    pub spent: Vec<f64>,
    /// The mean over the plies of the share of the resource in hand that was
    /// left unspent at the end of the ply, in percent (a ply that started
    /// with nothing is left out), or `null` if there is none.
    pub kept: Option<f64>,
}

/// A group of sides, by phase.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpendGroup {
    pub label: String,
    /// The sides (one a game).
    pub games: usize,
    pub by_phase: Vec<SpendCell>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Spending {
    /// The categories the hook named, by name.
    pub categories: Vec<String>,
    /// The focus players pooled, us in the same games, our wins, our losses.
    pub groups: Vec<SpendGroup>,
    /// Each focus player, in the games against us.
    pub players: Vec<SpendGroup>,
}

const PHASES: [Phase; 3] = [Phase::Early, Phase::Middle, Phase::End];

#[derive(Default)]
struct Sums {
    plies: usize,
    spent: BTreeMap<String, i64>,
    kept: Vec<f64>,
}

/// A side of a game.
type Side<'a> = (&'a Trace, usize);

fn group<P: Profiling>(label: &str, sides: &[Side], categories: &[String]) -> SpendGroup {
    let mut sums: Vec<Sums> = PHASES.iter().map(|_| Sums::default()).collect();
    for (trace, seat) in sides {
        let resource = trace.metric(P::resource());
        for (ply, p) in trace.plies.iter().enumerate() {
            if p.to_act != *seat {
                continue;
            }
            let sum = &mut sums[phase_index(trace.phase(ply))];
            sum.plies += 1;
            let mut total = 0;
            for (name, amount) in P::spending(trace, ply) {
                total += amount;
                *sum.spent.entry(name).or_default() += amount;
            }
            let held = resource.and_then(|i| p.metrics[*seat].get(i)).copied();
            if let Some(held) = held.filter(|&h| h > 0) {
                sum.kept
                    .push(((held - total).max(0) * 100) as f64 / held as f64);
            }
        }
    }
    SpendGroup {
        label: label.to_string(),
        games: sides.len(),
        by_phase: PHASES
            .iter()
            .zip(sums)
            .map(|(phase, sum)| SpendCell {
                phase: phase.name().to_string(),
                plies: sum.plies,
                spent: categories
                    .iter()
                    .map(|c| {
                        if sum.plies == 0 {
                            0.0
                        } else {
                            sum.spent.get(c).copied().unwrap_or(0) as f64 / sum.plies as f64
                        }
                    })
                    .collect(),
                kept: mean(&sum.kept),
            })
            .collect(),
    }
}

/// The categories named anywhere in `sides`' plies, sorted by name.
fn categories<P: Profiling>(sides: &[Side]) -> Vec<String> {
    let mut names = BTreeSet::new();
    for (trace, seat) in sides {
        for (ply, p) in trace.plies.iter().enumerate() {
            if p.to_act == *seat {
                names.extend(P::spending(trace, ply).into_iter().map(|(name, _)| name));
            }
        }
    }
    names.into_iter().collect()
}

pub(super) fn spending<P: Profiling>(games: &[&(GameMeta, Trace)], focus: &[String]) -> Spending {
    let (mut top, mut us) = (Vec::new(), Vec::new());
    let (mut wins, mut losses) = (Vec::new(), Vec::new());
    let mut players: BTreeMap<&str, Vec<Side>> = BTreeMap::new();
    for (meta, trace) in games {
        let Some((seat, opponent)) = versus(meta, trace) else {
            continue;
        };
        if let Some(name) = focus.iter().find(|f| **f == opponent) {
            us.push((trace, seat));
            top.push((trace, 1 - seat));
            players
                .entry(name.as_str())
                .or_default()
                .push((trace, 1 - seat));
        }
        match result(trace, seat) {
            Some(true) => wins.push((trace, seat)),
            Some(false) => losses.push((trace, seat)),
            None => {}
        }
    }
    let all: Vec<Side> = [&top, &us, &wins, &losses]
        .into_iter()
        .flatten()
        .copied()
        .collect();
    let categories = categories::<P>(&all);
    Spending {
        groups: vec![
            group::<P>("top players", &top, &categories),
            group::<P>("us, same games", &us, &categories),
            group::<P>("our wins", &wins, &categories),
            group::<P>("our losses", &losses, &categories),
        ],
        players: focus
            .iter()
            .map(|name| {
                let sides = players.get(name.as_str()).cloned().unwrap_or_default();
                group::<P>(name, &sides, &categories)
            })
            .collect(),
        categories,
    }
}

#[cfg(test)]
mod tests;
