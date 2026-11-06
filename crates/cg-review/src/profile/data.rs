//! The groups of sides that the curves and the data files compare, and the
//! data files: every metric and event of every group at every turn, as CSV.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use serde::Serialize;

use super::curves::{event_turns, last_own_turn, stats, values, Entry};
use super::{chart, result, versus, GameMeta, Profiling};
use crate::trace::Trace;

/// Rows of the data files need at least this many games at the turn.
pub const MIN_GAMES: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind {
    /// One focus player and us, in the games between them.
    Player,
    /// All focus players and us, in the games between them.
    All,
    Wins,
    Losses,
}

/// Sides of games that are compared, or described, together.
pub(super) struct Group<'a> {
    pub kind: Kind,
    /// The data file's name part.
    pub slug: String,
    pub label: String,
    /// The sides: a name and one entry for each game. Pairs list us first.
    pub sides: Vec<(String, Vec<Entry<'a>>)>,
}

/// A data file, for the report to link to.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DataGroup {
    pub slug: String,
    pub label: String,
    /// The games behind it (the first side's entries).
    pub games: usize,
}

impl Group<'_> {
    pub(super) fn summary(&self) -> DataGroup {
        DataGroup {
            slug: self.slug.clone(),
            label: self.label.clone(),
            games: self.sides.first().map_or(0, |s| s.1.len()),
        }
    }
}

/// The data groups: each focus player against us, all of them against us,
/// our wins and our losses.
pub(super) fn groups<'a>(games: &[&'a (GameMeta, Trace)], focus: &[String]) -> Vec<Group<'a>> {
    let mut per_player: Vec<(Vec<Entry>, Vec<Entry>)> =
        focus.iter().map(|_| Default::default()).collect();
    let (mut us, mut top) = (Vec::new(), Vec::new());
    let (mut wins, mut losses) = (Vec::new(), Vec::new());
    for game in games {
        let game: &'a (GameMeta, Trace) = game;
        let (meta, trace) = (&game.0, &game.1);
        let Some((seat, opponent)) = versus(meta, trace) else {
            continue;
        };
        let ours = Entry { trace, seat };
        let theirs = Entry {
            trace,
            seat: 1 - seat,
        };
        if let Some(i) = focus.iter().position(|f| *f == opponent) {
            us.push(ours);
            top.push(theirs);
            per_player[i].0.push(ours);
            per_player[i].1.push(theirs);
        }
        match result(trace, seat) {
            Some(true) => wins.push(ours),
            Some(false) => losses.push(ours),
            None => {}
        }
    }
    let mut out: Vec<Group> = focus
        .iter()
        .zip(per_player)
        .map(|(name, (ours, theirs))| Group {
            kind: Kind::Player,
            slug: chart::slug(name),
            label: name.clone(),
            sides: vec![("us".to_string(), ours), (name.clone(), theirs)],
        })
        .collect();
    out.push(Group {
        kind: Kind::All,
        slug: "all-top-players".to_string(),
        label: "all top players".to_string(),
        sides: vec![("us".to_string(), us), ("top players".to_string(), top)],
    });
    out.push(Group {
        kind: Kind::Wins,
        slug: "our-wins".to_string(),
        label: "our wins".to_string(),
        sides: vec![("us".to_string(), wins)],
    });
    out.push(Group {
        kind: Kind::Losses,
        slug: "our-losses".to_string(),
        label: "our losses".to_string(),
        sides: vec![("us".to_string(), losses)],
    });
    out
}

/// A CSV field, quoted if it has a comma or a quote.
pub(super) fn field(text: &str) -> String {
    if text.contains([',', '"', '\n']) {
        format!("\"{}\"", text.replace('"', "\"\""))
    } else {
        text.to_string()
    }
}

/// A number with at most three decimals, no trailing zeros.
pub(super) fn num(value: f64) -> String {
    let text = format!("{value:.3}");
    let text = text.trim_end_matches('0').trim_end_matches('.');
    match text {
        "" | "-" | "-0" => "0".to_string(),
        text => text.to_string(),
    }
}

/// Appends the rows of one series: its values by turn, for the turns that
/// at least `MIN_GAMES` games have.
fn rows(out: &mut String, head: &str, kind: &str, name: &str, columns: &[Vec<Option<f64>>]) {
    let turns = columns.iter().map(Vec::len).max().unwrap_or(0);
    for turn in 1..=turns {
        let at: Vec<f64> = columns
            .iter()
            .filter_map(|c| c.get(turn - 1).copied().flatten())
            .collect();
        if at.len() < MIN_GAMES {
            continue;
        }
        if let Some(s) = stats(&at) {
            let _ = writeln!(
                out,
                "{head},{kind},{name},{turn},{},{},{},{},{}",
                s.n,
                num(s.median),
                num(s.p25),
                num(s.p75),
                num(s.mean)
            );
        }
    }
}

/// The CSV text of one group: for each side, each metric of the traces and
/// each event, one row a turn that `MIN_GAMES` games reach, with the number
/// of games, the median, the quartiles and the mean. An event's value is 1
/// in a game where the side does it at that turn and 0 where it does not (so
/// its mean is the share).
fn csv<P: Profiling>(group: &Group) -> String {
    let mut out = String::from("group,side,kind,name,turn,n,median,p25,p75,mean\n");
    for (side, entries) in &group.sides {
        let head = format!("{},{}", field(&group.slug), field(side));
        let last = entries
            .iter()
            .map(|e| last_own_turn(e.trace, e.seat))
            .max()
            .unwrap_or(0);
        let mut keys: Vec<&str> = Vec::new();
        for e in entries {
            for m in &e.trace.metrics {
                if !keys.contains(&m.key.as_str()) {
                    keys.push(&m.key);
                }
            }
        }
        for key in keys {
            let columns: Vec<Vec<Option<f64>>> = entries
                .iter()
                .map(|e| match e.trace.metric(key) {
                    Some(i) => values(e.trace, e.seat, i, last)
                        .into_iter()
                        .map(|v| v.map(|v| v as f64))
                        .collect(),
                    None => Vec::new(),
                })
                .collect();
            rows(&mut out, &head, "metric", key, &columns);
        }
        let by_entry: Vec<Vec<Option<Vec<String>>>> =
            entries.iter().map(|e| event_turns::<P>(e, last)).collect();
        let names: BTreeSet<&String> = by_entry.iter().flatten().flatten().flatten().collect();
        for name in names {
            let columns: Vec<Vec<Option<f64>>> = by_entry
                .iter()
                .map(|turns| {
                    turns
                        .iter()
                        .map(|t| t.as_ref().map(|v| f64::from(u8::from(v.contains(name)))))
                        .collect()
                })
                .collect();
            rows(&mut out, &head, "event", name, &columns);
        }
    }
    out
}

/// The data files of every group: file name and CSV.
pub(super) fn files<P: Profiling>(groups: &[Group]) -> Vec<(String, String)> {
    groups
        .iter()
        .map(|g| (format!("turns-{}.csv", g.slug), csv::<P>(g)))
        .collect()
}

#[cfg(test)]
mod tests;
