//! Economy or fighting: is a strategy better because of its economy or
//! because of its fighting? The numbers that tell them apart, for the
//! groups of games the report compares:
//!
//! - the exchange of units: the value of the enemy units a side removed
//!   against the value of its own that it lost, by phase and turn;
//! - efficiency: what a side gained (cells, value killed) for each unit of
//!   resource it spent so far;
//! - the investment mix: the share of its spending going to each role
//!   (economy, army, defence) in each phase;
//! - the upkeep burden: upkeep as a share of gross income;
//! - tempo: the turn of first contact, and the turns from first contact to
//!   the first decisive moment in the side's favour;
//! - the value of a lead: how often we won from each size of lead at
//!   a few turns.
//!
//! A game names its metrics and categories in [`EconomyHooks`]; the code
//! below knows none of their names. Turn-by-turn series are curves of the
//! same kind as the report's others ([`CurveSet`]), so they are charted the
//! same way.

use serde::Serialize;

use super::curves::{build_curve, curve_set, last_own_turn, values, Entry};
use super::data::{num, Group, Kind};
use super::spending::{side_groups, Side};
use super::{
    chart, mean, median, phase_index, turn_of, value_at, versus, CurveSet, GameMeta, Profiling,
};
use crate::trace::{Phase, Trace};

/// What a game says about its economy and its fighting.
#[derive(Clone, Debug)]
pub struct EconomyHooks {
    /// The cumulative metrics of the exchange of units: its balance
    /// (killed less lost), the value the side removed and the value of its
    /// own units it lost.
    pub balance: &'static str,
    pub killed: &'static str,
    pub lost: &'static str,
    /// How much a side gets for what it spends.
    pub efficiency: Vec<Efficiency>,
    /// What each category of spending is for.
    pub roles: Vec<Role>,
    /// The upkeep burden is the first metric (per turn) over the second.
    pub upkeep: &'static str,
    pub gross_income: &'static str,
    /// The first turn this metric is above zero is the turn of first
    /// contact.
    pub contact: &'static str,
    /// The income metric: the value of a lead is read on it and on the
    /// trace's lead metric.
    pub income: &'static str,
    /// The kinds of moments that are decisive (the tempo reads the first
    /// that favours a side).
    pub decisive: Vec<&'static str>,
}

/// Efficiency: the metrics a side gained (cumulative, summed) over the
/// resource it spent so far on some categories.
#[derive(Clone, Debug)]
pub struct Efficiency {
    pub name: &'static str,
    pub gained: Vec<&'static str>,
    pub spent: Vec<&'static str>,
}

/// A role of spending and the categories (of `Profiling::spending`) that
/// serve it.
#[derive(Clone, Debug)]
pub struct Role {
    pub name: &'static str,
    pub categories: Vec<&'static str>,
}

/// The role of spending that no [`Role`] names.
const OTHER: &str = "other";

/// A group's exchange at the end of a phase: the mean over its sides, each
/// read right after its own last ply of the phase.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExchangeRow {
    pub group: String,
    pub phase: String,
    /// The sides with a ply in the phase.
    pub games: usize,
    pub killed: Option<f64>,
    pub lost: Option<f64>,
    pub balance: Option<f64>,
}

/// A group's efficiency at a turn: the median over its sides of the
/// cumulative gain over the cumulative spending.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EfficiencyRow {
    pub group: String,
    pub name: String,
    pub turn: u32,
    /// The sides that have a value at the turn (they have spent something).
    pub games: usize,
    pub median: Option<f64>,
}

/// A group's spending on a role in a phase.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MixRow {
    pub group: String,
    pub phase: String,
    /// Own plies in the phase, over the group's sides.
    pub plies: usize,
    pub role: String,
    pub spent_per_ply: f64,
    /// The share of all that was spent in the phase, in percent; `null`
    /// when nothing was spent.
    pub share: Option<f64>,
}

/// A group's upkeep burden in a phase: the mean over its own plies of the
/// upkeep as a percentage of the gross income (plies with no gross income
/// are left out).
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpkeepRow {
    pub group: String,
    pub phase: String,
    pub plies: usize,
    pub burden: Option<f64>,
}

/// A group's tempo.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TempoRow {
    pub group: String,
    pub games: usize,
    /// The sides that had contact, and the median turn of the first.
    pub contact_games: usize,
    pub contact_turn: Option<f64>,
    /// The sides with a decisive moment in their favour, and the median
    /// turn of the first.
    pub decisive_games: usize,
    pub decisive_turn: Option<f64>,
    /// The sides with both, and the median of the turns from first contact
    /// to the first decisive moment (negative if the moment came first).
    pub both_games: usize,
    pub to_decisive: Option<f64>,
}

/// How our games went from a lead of some size at a turn.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LeadRow {
    /// The metric of the lead (ours less theirs, read as that metric's
    /// direction says: a positive lead is always an advantage).
    pub metric: String,
    pub turn: u32,
    pub bin: String,
    pub games: usize,
    pub wins: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Economy {
    /// The kinds of moments the tempo counts as decisive.
    pub decisive: Vec<String>,
    pub exchange: Vec<ExchangeRow>,
    pub efficiency: Vec<EfficiencyRow>,
    pub mix: Vec<MixRow>,
    pub upkeep: Vec<UpkeepRow>,
    pub tempo: Vec<TempoRow>,
    pub lead: Vec<LeadRow>,
    /// The sets of curves that are charted: us against the top players,
    /// our wins against our losses.
    pub sets: Vec<CurveSet>,
    /// Us against each focus player (data only).
    pub players: Vec<CurveSet>,
    /// One line for each set, from its curves: the two explanations side by
    /// side, in numbers.
    pub verdicts: Vec<Verdict>,
}

/// The two explanations compared for one set, as a sentence.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Verdict {
    /// The set's slug.
    pub slug: String,
    pub text: String,
}

/// A series by group, entry and game turn.
type Columns = Vec<Vec<Vec<Option<f64>>>>;

const PHASES: [Phase; 3] = [Phase::Early, Phase::Middle, Phase::End];

/// The key of the upkeep burden's curve.
pub const BURDEN: &str = "upkeep burden";

/// The value of metric `index` for `seat` right after its last own ply in
/// `phase`.
fn end_of_phase(trace: &Trace, seat: usize, index: usize, phase: usize) -> Option<f64> {
    let last = (0..trace.plies.len())
        .rev()
        .find(|&i| trace.plies[i].to_act == seat && phase_index(trace.phase(i)) == phase)?;
    let ply = trace.plies.get(last + 1).unwrap_or(&trace.plies[last]);
    let value = *ply.metrics[seat].get(index)?;
    (trace.metrics[index].none != Some(value)).then_some(value as f64)
}

/// What `seat` spent on `categories` in each game turn up to `last`,
/// cumulated: the entry at index `t - 1` is what it spent in turns 1 to `t`.
fn cumulative_spent<P: Profiling>(
    trace: &Trace,
    seat: usize,
    categories: &[&str],
    last: u32,
) -> Vec<i64> {
    let mut per_turn = vec![0i64; last as usize];
    for (ply, p) in trace.plies.iter().enumerate() {
        let turn = p.game_turn as usize;
        if p.to_act != seat || turn == 0 || turn > per_turn.len() {
            continue;
        }
        per_turn[turn - 1] += P::spending(trace, ply)
            .into_iter()
            .filter(|(name, _)| categories.contains(&name.as_str()))
            .map(|(_, amount)| amount)
            .sum::<i64>();
    }
    let mut total = 0;
    per_turn
        .into_iter()
        .map(|s| {
            total += s;
            total
        })
        .collect()
}

/// The efficiency of `seat` at each game turn from 1 to `last`: what it
/// gained so far over what it spent so far; `None` before it has spent
/// anything or when it has no ply at the turn.
fn efficiency_values<P: Profiling>(
    trace: &Trace,
    seat: usize,
    efficiency: &Efficiency,
    last: u32,
) -> Vec<Option<f64>> {
    let indexes: Option<Vec<usize>> = efficiency.gained.iter().map(|k| trace.metric(k)).collect();
    let Some(indexes) = indexes else {
        return vec![None; last as usize];
    };
    let gained: Vec<Vec<Option<i64>>> = indexes
        .iter()
        .map(|&i| values(trace, seat, i, last))
        .collect();
    let spent = cumulative_spent::<P>(trace, seat, &efficiency.spent, last);
    (0..last as usize)
        .map(|t| {
            let total: Option<i64> = gained.iter().map(|g| g[t]).sum();
            let total = total?;
            (spent[t] > 0).then(|| total as f64 / spent[t] as f64)
        })
        .collect()
}

/// The upkeep burden of `seat` at each game turn from 1 to `last`, in
/// percent of the gross income; `None` when there is no gross income.
fn burden_values(trace: &Trace, seat: usize, hooks: &EconomyHooks, last: u32) -> Vec<Option<f64>> {
    let (Some(up), Some(gross)) = (trace.metric(hooks.upkeep), trace.metric(hooks.gross_income))
    else {
        return vec![None; last as usize];
    };
    let up = values(trace, seat, up, last);
    let gross = values(trace, seat, gross, last);
    up.into_iter()
        .zip(gross)
        .map(|(u, g)| match (u, g) {
            (Some(u), Some(g)) if g > 0 => Some(u as f64 * 100.0 / g as f64),
            _ => None,
        })
        .collect()
}

/// The first game turn at which `seat`'s `key` is above zero (read right
/// after its own ply).
fn first_above_zero(trace: &Trace, seat: usize, key: &str) -> Option<u32> {
    let index = trace.metric(key)?;
    values(trace, seat, index, last_own_turn(trace, seat))
        .into_iter()
        .position(|v| v.is_some_and(|v| v > 0))
        .map(|t| t as u32 + 1)
}

/// The game turn of the first moment of one of `kinds` that favours `seat`.
fn first_decisive(trace: &Trace, seat: usize, kinds: &[&str]) -> Option<u32> {
    trace
        .moments
        .iter()
        .filter(|m| m.favours == Some(seat) && kinds.contains(&m.kind.as_str()))
        .map(|m| turn_of(trace, m.ply))
        .min()
}

fn exchange_rows(label: &str, sides: &[Side], hooks: &EconomyHooks) -> Vec<ExchangeRow> {
    let mut rows = Vec::new();
    for (phase, name) in PHASES.iter().enumerate().map(|(i, p)| (i, p.name())) {
        let read = |key: &str| -> Vec<f64> {
            sides
                .iter()
                .filter_map(|(trace, seat)| end_of_phase(trace, *seat, trace.metric(key)?, phase))
                .collect()
        };
        let games = sides
            .iter()
            .filter(|(trace, seat)| {
                (0..trace.plies.len())
                    .any(|i| trace.plies[i].to_act == *seat && phase_index(trace.phase(i)) == phase)
            })
            .count();
        rows.push(ExchangeRow {
            group: label.to_string(),
            phase: name.to_string(),
            games,
            killed: mean(&read(hooks.killed)),
            lost: mean(&read(hooks.lost)),
            balance: mean(&read(hooks.balance)),
        });
    }
    rows
}

fn efficiency_rows<P: Profiling>(
    label: &str,
    sides: &[Side],
    hooks: &EconomyHooks,
) -> Vec<EfficiencyRow> {
    let turns = P::turns();
    let last = turns.iter().copied().max().unwrap_or(0);
    let mut rows = Vec::new();
    for eff in &hooks.efficiency {
        let series: Vec<Vec<Option<f64>>> = sides
            .iter()
            .map(|(trace, seat)| efficiency_values::<P>(trace, *seat, eff, last))
            .collect();
        for &turn in &turns {
            let at: Vec<f64> = series
                .iter()
                .filter_map(|s| s.get(turn as usize - 1).copied().flatten())
                .collect();
            rows.push(EfficiencyRow {
                group: label.to_string(),
                name: eff.name.to_string(),
                turn,
                games: at.len(),
                median: median(&at),
            });
        }
    }
    rows
}

fn mix_rows<P: Profiling>(label: &str, sides: &[Side], hooks: &EconomyHooks) -> Vec<MixRow> {
    let mut rows = Vec::new();
    for (phase, name) in PHASES.iter().enumerate().map(|(i, p)| (i, p.name())) {
        let mut plies = 0;
        let mut by_role: Vec<i64> = vec![0; hooks.roles.len() + 1];
        for (trace, seat) in sides {
            for (ply, p) in trace.plies.iter().enumerate() {
                if p.to_act != *seat || phase_index(trace.phase(ply)) != phase {
                    continue;
                }
                plies += 1;
                for (category, amount) in P::spending(trace, ply) {
                    let role = hooks
                        .roles
                        .iter()
                        .position(|r| r.categories.contains(&category.as_str()))
                        .unwrap_or(hooks.roles.len());
                    by_role[role] += amount;
                }
            }
        }
        let total: i64 = by_role.iter().sum();
        let names = hooks
            .roles
            .iter()
            .map(|r| r.name)
            .chain(std::iter::once(OTHER));
        for (i, role) in names.enumerate() {
            // "other" only when something was spent on it.
            if i == hooks.roles.len() && by_role[i] == 0 {
                continue;
            }
            rows.push(MixRow {
                group: label.to_string(),
                phase: name.to_string(),
                plies,
                role: role.to_string(),
                spent_per_ply: if plies == 0 {
                    0.0
                } else {
                    by_role[i] as f64 / plies as f64
                },
                share: (total > 0).then(|| by_role[i] as f64 * 100.0 / total as f64),
            });
        }
    }
    rows
}

fn upkeep_rows(label: &str, sides: &[Side], hooks: &EconomyHooks) -> Vec<UpkeepRow> {
    let mut rows = Vec::new();
    for (phase, name) in PHASES.iter().enumerate().map(|(i, p)| (i, p.name())) {
        let (mut plies, mut burdens) = (0, Vec::new());
        for (trace, seat) in sides {
            let (Some(up), Some(gross)) =
                (trace.metric(hooks.upkeep), trace.metric(hooks.gross_income))
            else {
                continue;
            };
            for (ply, p) in trace.plies.iter().enumerate() {
                if p.to_act != *seat || phase_index(trace.phase(ply)) != phase {
                    continue;
                }
                plies += 1;
                let (u, g) = (p.metrics[*seat][up], p.metrics[*seat][gross]);
                if g > 0 {
                    burdens.push(u as f64 * 100.0 / g as f64);
                }
            }
        }
        rows.push(UpkeepRow {
            group: label.to_string(),
            phase: name.to_string(),
            plies,
            burden: mean(&burdens),
        });
    }
    rows
}

fn tempo_row(label: &str, sides: &[Side], hooks: &EconomyHooks) -> TempoRow {
    let (mut contacts, mut decisives, mut gaps) = (Vec::new(), Vec::new(), Vec::new());
    for (trace, seat) in sides {
        let contact = first_above_zero(trace, *seat, hooks.contact);
        let decisive = first_decisive(trace, *seat, &hooks.decisive);
        contacts.extend(contact.map(f64::from));
        decisives.extend(decisive.map(f64::from));
        if let (Some(c), Some(d)) = (contact, decisive) {
            gaps.push(f64::from(d) - f64::from(c));
        }
    }
    TempoRow {
        group: label.to_string(),
        games: sides.len(),
        contact_games: contacts.len(),
        contact_turn: median(&contacts),
        decisive_games: decisives.len(),
        decisive_turn: median(&decisives),
        both_games: gaps.len(),
        to_decisive: median(&gaps),
    }
}

/// How our games went from each size of lead at each turn, for the lead
/// metric of the traces and the income. The sizes are relative to the
/// median size of the leads at the turn: ahead (or behind) by up to that
/// median, or by more, and level.
fn lead_rows<P: Profiling>(games: &[&(GameMeta, Trace)], hooks: &EconomyHooks) -> Vec<LeadRow> {
    let mut keys: Vec<String> = Vec::new();
    for (_, trace) in games {
        for key in [trace.lead.as_str(), hooks.income] {
            if !keys.iter().any(|k| k == key) {
                keys.push(key.to_string());
            }
        }
    }
    let mut rows = Vec::new();
    for key in &keys {
        for turn in P::turns() {
            // Our lead (an advantage if positive) and whether we won.
            let mut leads: Vec<(i64, bool)> = Vec::new();
            for (meta, trace) in games {
                let (Some((us, _)), Some(index)) = (versus(meta, trace), trace.metric(key)) else {
                    continue;
                };
                let (Some(ours), Some(theirs)) = (
                    value_at(trace, us, turn, index),
                    value_at(trace, 1 - us, turn, index),
                ) else {
                    continue;
                };
                let lead = if P::lower_is_better(key) {
                    theirs - ours
                } else {
                    ours - theirs
                };
                leads.push((lead, trace.winner == Some(us)));
            }
            rows.extend(lead_bins(key, turn, &leads));
        }
    }
    rows
}

/// The bins of `leads` (a lead and whether it was a win) at one turn.
fn lead_bins(key: &str, turn: u32, leads: &[(i64, bool)]) -> Vec<LeadRow> {
    let sizes: Vec<f64> = leads
        .iter()
        .filter(|(l, _)| *l != 0)
        .map(|(l, _)| l.unsigned_abs() as f64)
        .collect();
    let edge = median(&sizes).map(|m| m.ceil() as i64);
    let bin = |lead: i64| -> String {
        match edge {
            _ if lead == 0 => "level".to_string(),
            Some(e) if lead < -e => format!("behind by more than {e}"),
            Some(e) if lead < 0 => format!("behind by up to {e}"),
            Some(e) if lead <= e => format!("ahead by up to {e}"),
            Some(e) => format!("ahead by more than {e}"),
            None => "level".to_string(),
        }
    };
    let mut order: Vec<String> = Vec::new();
    if let Some(e) = edge {
        order.extend([
            format!("behind by more than {e}"),
            format!("behind by up to {e}"),
        ]);
    }
    order.push("level".to_string());
    if let Some(e) = edge {
        order.extend([
            format!("ahead by up to {e}"),
            format!("ahead by more than {e}"),
        ]);
    }
    order
        .into_iter()
        .filter_map(|name| {
            let here: Vec<&(i64, bool)> = leads.iter().filter(|(l, _)| bin(*l) == name).collect();
            (!here.is_empty()).then(|| LeadRow {
                metric: key.to_string(),
                turn,
                games: here.len(),
                wins: here.iter().filter(|(_, won)| *won).count(),
                bin: name,
            })
        })
        .collect()
}

/// The curves of the economy for two groups of entries: the exchange (its
/// balance and its two parts), the efficiencies and the upkeep burden.
fn economy_set<P: Profiling>(
    hooks: &EconomyHooks,
    slug: &str,
    labels: [&str; 2],
    either: bool,
    groups: [Vec<Entry>; 2],
) -> CurveSet {
    let keys: Vec<String> = [hooks.balance, hooks.killed, hooks.lost]
        .iter()
        .map(|k| (*k).to_string())
        .collect();
    let mut set = curve_set::<P>(slug, labels, either, groups.clone(), Some(&keys), false);
    let last = set.last_turn;
    if last == 0 {
        return set;
    }
    let mut derived: Vec<(String, String, bool, Columns)> = Vec::new();
    for eff in &hooks.efficiency {
        let columns = groups
            .iter()
            .map(|g| {
                g.iter()
                    .map(|e| efficiency_values::<P>(e.trace, e.seat, eff, last))
                    .collect()
            })
            .collect();
        derived.push((
            eff.name.to_string(),
            format!(
                "{} so far, for each unit of resource spent so far on {}.",
                eff.gained.join(" + "),
                eff.spent.join(", ")
            ),
            false,
            columns,
        ));
    }
    let columns = groups
        .iter()
        .map(|g| {
            g.iter()
                .map(|e| burden_values(e.trace, e.seat, hooks, last))
                .collect()
        })
        .collect();
    derived.push((
        BURDEN.to_string(),
        format!(
            "{} as a percentage of {}: how much of the gross income the army eats.",
            hooks.upkeep, hooks.gross_income
        ),
        true,
        columns,
    ));
    for (key, meaning, lower, columns) in derived {
        set.curves.push(build_curve(
            &key, meaning, lower, false, &columns, last, either,
        ));
    }
    set
}

/// A signed number: "+3", "-2", "0".
fn signed(value: f64) -> String {
    if value > 0.0 {
        format!("+{}", num(value))
    } else {
        num(value)
    }
}

/// The medians of `key` at `turn` for the two groups of the set, if the
/// curve exists.
fn pair(set: &CurveSet, key: &str, turn: u32) -> Option<(Option<f64>, Option<f64>)> {
    let curve = set.curves.iter().find(|c| c.key == key)?;
    let i = turn as usize - 1;
    Some((curve.lines[0].get(i)?.median, curve.lines[1].get(i)?.median))
}

/// The sentence that puts the two explanations side by side at the last
/// compared turn the set reaches: the exchange (fighting), then what each
/// side gets for what it spends and the upkeep burden (economy).
fn verdict<P: Profiling>(hooks: &EconomyHooks, set: &CurveSet) -> Option<Verdict> {
    let turn = P::compare_turns()
        .into_iter()
        .rev()
        .find(|&t| t >= 1 && t <= set.last_turn)?;
    let [first, second] = &set.labels;
    let both = |(a, b): (Option<f64>, Option<f64>), f: fn(f64) -> String, unit: &str| {
        let text = |v: Option<f64>| v.map_or("-".to_string(), |v| format!("{}{unit}", f(v)));
        format!("{first} {}, {second} {}", text(a), text(b))
    };
    let mut parts = Vec::new();
    if let Some(medians) = pair(set, hooks.balance, turn) {
        parts.push(format!("exchange {}", both(medians, signed, "")));
    }
    for eff in &hooks.efficiency {
        if let Some(medians) = pair(set, eff.name, turn) {
            parts.push(format!("{} {}", eff.name, both(medians, num, "")));
        }
    }
    if let Some(medians) = pair(set, BURDEN, turn) {
        parts.push(format!("{BURDEN} {}", both(medians, num, "%")));
    }
    if parts.is_empty() {
        return None;
    }
    let games = format!("n = {} and {}", set.games[0], set.games[1]);
    Some(Verdict {
        slug: set.slug.clone(),
        text: format!(
            "{first} and {second}, at turn {turn} ({games}): {}.",
            parts.join("; ")
        ),
    })
}

/// The economy analysis of the games, for the groups of games `groups` (see
/// [`Group`]) and the focus players.
pub(super) fn economy<P: Profiling>(
    games: &[&(GameMeta, Trace)],
    focus: &[String],
    groups: &[Group],
) -> Economy {
    let Some(hooks) = P::economy() else {
        return Economy::default();
    };
    let sides = side_groups(games, focus);
    let mut out = Economy {
        decisive: hooks.decisive.iter().map(|k| (*k).to_string()).collect(),
        ..Economy::default()
    };
    for (label, group) in sides.labelled() {
        out.exchange.extend(exchange_rows(label, group, &hooks));
        out.efficiency
            .extend(efficiency_rows::<P>(label, group, &hooks));
        out.mix.extend(mix_rows::<P>(label, group, &hooks));
        out.upkeep.extend(upkeep_rows(label, group, &hooks));
        out.tempo.push(tempo_row(label, group, &hooks));
    }
    out.lead = lead_rows::<P>(games, &hooks);
    fn side<'a>(g: &Group<'a>, i: usize) -> Vec<Entry<'a>> {
        g.sides.get(i).map(|s| s.1.clone()).unwrap_or_default()
    }
    let kind = |k: Kind| groups.iter().find(|g| g.kind == k);
    let all = kind(Kind::All);
    out.sets.push(economy_set::<P>(
        &hooks,
        "economy-top-vs-us",
        ["us", "top players"],
        true,
        [
            all.map(|g| side(g, 0)).unwrap_or_default(),
            all.map(|g| side(g, 1)).unwrap_or_default(),
        ],
    ));
    out.sets.push(economy_set::<P>(
        &hooks,
        "economy-wins-vs-losses",
        ["our wins", "our losses"],
        false,
        [
            kind(Kind::Wins).map(|g| side(g, 0)).unwrap_or_default(),
            kind(Kind::Losses).map(|g| side(g, 0)).unwrap_or_default(),
        ],
    ));
    for g in groups.iter().filter(|g| g.kind == Kind::Player) {
        out.players.push(economy_set::<P>(
            &hooks,
            &format!("economy-player-{}", chart::slug(&g.label)),
            ["us", &g.label],
            true,
            [side(g, 0), side(g, 1)],
        ));
    }
    out.verdicts = out
        .sets
        .iter()
        .chain(&out.players)
        .filter_map(|set| verdict::<P>(&hooks, set))
        .collect();
    out
}

#[cfg(test)]
mod tests;
