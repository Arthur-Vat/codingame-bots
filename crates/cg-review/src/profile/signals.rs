//! `signals.md`: the strongest findings, picked by code from the numbers
//! alone, each pointing to its chart and data file. No interpretation.

use std::fmt::Write as _;

use super::data::num;
use super::{chart, Analysis, Curve, CurveSet, SMALL};

/// The metrics listed for each set.
const BEST: usize = 4;
/// An event's rate counts as different when the two groups are this many
/// percentage points apart.
pub const EVENT_GAP: f64 = 20.0;

/// A median with at most one decimal, or "-".
fn med(value: Option<f64>) -> String {
    value.map_or("-".to_string(), num)
}

/// The sentence for `curve` at game turn `turn`.
fn sentence(set: &CurveSet, curve: &Curve, turn: u32) -> String {
    let [first, second] = &set.labels;
    let mut text = format!("**{}**: ", curve.key);
    match curve.separates_from {
        Some(t) => {
            let _ = write!(text, "{first} and {second} separate from turn {t}");
        }
        None => {
            let _ = write!(text, "{first} and {second} do not stay apart to the end");
        }
    }
    let i = turn as usize - 1;
    let _ = write!(
        text,
        "; at turn {turn} the median is {} for {first} and {} for {second} ({}AUC {}, n = {} and {}).",
        med(curve.lines[0][i].median),
        med(curve.lines[1][i].median),
        if curve.lower_is_better {
            "less is better; "
        } else {
            ""
        },
        curve.auc[i].map_or("-".to_string(), |v| format!("{v:.2}")),
        curve.lines[0][i].games,
        curve.lines[1][i].games,
    );
    text
}

/// "9 to 14, 17": the turns as ranges of consecutive turns.
fn ranges(turns: &[u32]) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < turns.len() {
        let mut j = i;
        while j + 1 < turns.len() && turns[j + 1] == turns[j] + 1 {
            j += 1;
        }
        out.push(if j > i {
            format!("{} to {}", turns[i], turns[j])
        } else {
            turns[i].to_string()
        });
        i = j + 1;
    }
    out.join(", ")
}

/// The best metrics of `set` by the ranking, with a sentence each. Where
/// the groups separate, `at_separation` reads the turn from there, else the
/// last compared turn that has an AUC.
fn best(out: &mut String, a: &Analysis, set: &CurveSet, at_separation: bool, data: &[&str]) {
    let comparison = set.compare(&a.curves.compare_turns);
    let mut shown = 0;
    for row in comparison.rows.iter().filter(|r| r.score.is_some()) {
        let Some(curve) = set.curves.iter().find(|c| c.key == row.key) else {
            continue;
        };
        let last = row
            .at
            .iter()
            .rev()
            .find(|c| c.auc.is_some())
            .map(|c| c.turn);
        let turn = if at_separation {
            curve.separates_from.or(last)
        } else {
            last
        };
        let Some(turn) = turn else { continue };
        let _ = writeln!(
            out,
            "- {} Chart: `{}`. Data: {}.",
            sentence(set, curve, turn),
            chart::file_name(set, &curve.key, false),
            data.join(", ")
        );
        shown += 1;
        if shown == BEST {
            break;
        }
    }
    if shown == 0 {
        out.push_str("- Nothing to compare: too few games.\n");
    }
    out.push('\n');
}

/// The turns where the two groups' rates of an event differ by `EVENT_GAP`
/// points or more, with the larger side.
fn event_gaps(out: &mut String, set: &CurveSet, data: &str) {
    let [first, second] = &set.labels;
    let mut any = false;
    for curve in &set.events {
        let mut by_side: [Vec<u32>; 2] = [Vec::new(), Vec::new()];
        let mut widest: Option<(f64, u32, f64, f64)> = None;
        for (a, b) in curve.lines[0].iter().zip(&curve.lines[1]) {
            let (Some(x), Some(y)) = (a.mean, b.mean) else {
                continue;
            };
            if a.games < SMALL || b.games < SMALL {
                continue;
            }
            let gap = (x - y) * 100.0;
            if gap.abs() >= EVENT_GAP {
                by_side[usize::from(gap < 0.0)].push(a.turn);
                if widest.is_none_or(|w| gap.abs() > w.0) {
                    widest = Some((gap.abs(), a.turn, x * 100.0, y * 100.0));
                }
            }
        }
        let Some((gap, turn, x, y)) = widest else {
            continue;
        };
        any = true;
        let _ = write!(out, "- **{}**:", curve.key);
        for (side, label) in [first, second].into_iter().enumerate() {
            if !by_side[side].is_empty() {
                let _ = write!(
                    out,
                    " the rate of {label} is {EVENT_GAP:.0} points higher or more at {} {};",
                    if by_side[side].len() > 1 {
                        "turns"
                    } else {
                        "turn"
                    },
                    ranges(&by_side[side])
                );
            }
        }
        let _ = writeln!(
            out,
            " widest at turn {turn}: {:.0}% of {first} against {:.0}% of {second} ({gap:.0} points). Chart: `{}`. Data: {data}.",
            x,
            y,
            chart::file_name(set, &curve.key, true)
        );
    }
    if !any {
        let _ = writeln!(
            out,
            "- No event differs by {EVENT_GAP:.0} points at a turn with {SMALL} games on each side."
        );
    }
    out.push('\n');
}

/// The text of `signals.md`.
pub(super) fn signals(a: &Analysis) -> String {
    let c = &a.curves;
    let mut out = String::from("# Signals\n\nPicked by code from the numbers in this folder; no interpretation. Each line names its chart and the data file it comes from (`README.md` lists the files and their columns).\n\n");
    out.push_str("The AUC of a metric at a turn is the share of pairs (one value of the first group, one of the second) where the first group's value is the better one (higher, or lower where less is better), a tie counting half: 0.5 is no difference. Two groups \"separate\" from the first turn after which the AUC stays at 0.70 or more (for the top players against us, 0.70 or more or 0.30 or less) to the last turn with at least 5 games on each side. Medians are read for each side right after its own ply of the turn.\n\n");
    out.push_str("## Our wins against our losses\n\nThe four metrics with the highest mean AUC at the compared turns, read at the last of them.\n\n");
    best(
        &mut out,
        a,
        &c.wins_vs_losses,
        false,
        &[
            "`separation.csv` (set `wins-vs-losses`)",
            "`turns-our-wins.csv`",
            "`turns-our-losses.csv`",
        ],
    );
    out.push_str("## The top players against us\n\nThe four metrics where the two groups differ most at the compared turns, read at the turn they separate.\n\n");
    best(
        &mut out,
        a,
        &c.top_vs_us,
        true,
        &[
            "`separation.csv` (set `top-vs-us`)",
            "`turns-all-top-players.csv`",
        ],
    );
    out.push_str("## Each top player against us\n\nFor each player, among the metrics that tell our wins from our losses best, the one where it and we differ most at the compared turns, read at the last of them.\n\n");
    let mut any = false;
    for set in &c.players {
        let comparison = set.compare(&c.compare_turns);
        let Some(row) = comparison.rows.iter().find(|r| r.score.is_some()) else {
            continue;
        };
        let Some(curve) = set.curves.iter().find(|x| x.key == row.key) else {
            continue;
        };
        let Some(turn) = row
            .at
            .iter()
            .rev()
            .find(|x| x.auc.is_some())
            .map(|x| x.turn)
        else {
            continue;
        };
        any = true;
        let _ = writeln!(
            out,
            "- {}, {} Chart: `{}`. Data: `separation.csv` (set `{}`), `turns-{}.csv`.",
            set.labels[1],
            sentence(set, curve, turn),
            chart::file_name(set, &curve.key, false),
            set.slug,
            set.slug.trim_start_matches("player-"),
        );
    }
    if !any {
        out.push_str("- Too few games against each player.\n");
    }
    out.push('\n');
    out.push_str("## Events by turn\n\nTurns where the share of games in which a side does something differs by at least 20 points between the groups, with 5 games or more on each side.\n\n");
    out.push_str("The top players against us:\n\n");
    event_gaps(&mut out, &c.top_vs_us, "`turns-all-top-players.csv`");
    out.push_str("Our wins against our losses:\n\n");
    event_gaps(
        &mut out,
        &c.wins_vs_losses,
        "`turns-our-wins.csv`, `turns-our-losses.csv`",
    );
    out
}
