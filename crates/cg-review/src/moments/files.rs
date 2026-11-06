//! The decisive moments as files for a folder: the numbers as CSV, the
//! examples as Markdown, and a README that says what is in each.

use std::fmt::Write as _;

use super::render::{examples, median_of_turns, with_moment};
use super::{median_turn, Findings, Loss, EXPOSURE_PLIES};
use crate::profile::median;
use crate::profile::render::num;
use crate::trace::SIDES;

/// A CSV field, quoted when it needs it.
fn field(text: &str) -> String {
    if text.contains([',', '"', '\n']) {
        format!("\"{}\"", text.replace('"', "\"\""))
    } else {
        text.to_string()
    }
}

fn line(out: &mut String, fields: &[String]) {
    let fields: Vec<String> = fields.iter().map(|f| field(f)).collect();
    let _ = writeln!(out, "{}", fields.join(","));
}

/// An empty field for a missing value.
fn blank(v: Option<i64>) -> String {
    v.map_or(String::new(), |v| v.to_string())
}

/// The files of the folder, as (name, content).
pub fn files(f: &Findings) -> Vec<(String, String)> {
    let mut examples_md = String::from("# Decisive moments: examples\n\n");
    examples(&mut examples_md, f);
    vec![
        ("moments.csv".to_string(), moments_csv(f)),
        ("summary.csv".to_string(), summary_csv(f)),
        ("examples.md".to_string(), examples_md),
        ("README.md".to_string(), readme(f)),
    ]
}

fn moments_csv(f: &Findings) -> String {
    let mut out = String::new();
    let mut header: Vec<String> = [
        "group", "game_id", "opponent", "rank", "our_seat", "kind", "ply", "turn",
    ]
    .map(String::from)
    .to_vec();
    for key in &f.metrics {
        header.push(format!("theirs_{key}"));
    }
    for key in &f.metrics {
        header.push(format!("ours_{key}"));
    }
    for k in 1..=EXPOSURE_PLIES {
        header.push(format!("exposure_{k}"));
    }
    line(&mut out, &header);
    for loss in &f.losses {
        let mut row = vec![
            "loss".to_string(),
            loss.game_id.to_string(),
            loss.opponent.clone(),
            loss.rank.map_or(String::new(), |r| r.to_string()),
            SIDES[loss.our_seat].to_string(),
        ];
        match &loss.found {
            Some(found) => {
                row.extend([
                    found.kind.clone(),
                    found.ply.to_string(),
                    found.turn.to_string(),
                ]);
                row.extend(found.theirs.iter().map(|v| blank(*v)));
                row.extend(found.ours.iter().map(|v| blank(*v)));
                row.extend(found.exposure.iter().map(|v| blank(*v)));
            }
            None => {
                row.extend(["none".to_string(), String::new(), String::new()]);
                row.resize(header.len(), String::new());
            }
        }
        line(&mut out, &row);
    }
    for control in &f.controls {
        for case in &control.cases {
            let mut row = vec![
                "win_control".to_string(),
                case.game_id.to_string(),
                case.opponent.clone(),
                case.rank.map_or(String::new(), |r| r.to_string()),
                SIDES[case.our_seat].to_string(),
                control.kind.clone(),
                case.ply.to_string(),
                case.turn.to_string(),
            ];
            row.resize(header.len() - EXPOSURE_PLIES, String::new());
            row.extend(case.exposure.iter().map(|v| blank(*v)));
            line(&mut out, &row);
        }
    }
    out
}

/// One row of summary.csv.
struct Row<'a> {
    table: &'a str,
    group: &'a str,
    kind: &'a str,
    threshold: String,
    ply: String,
    measure: &'a str,
    value: String,
}

fn add(out: &mut String, row: Row) {
    line(
        out,
        &[
            row.table.to_string(),
            row.group.to_string(),
            row.kind.to_string(),
            row.threshold,
            row.ply,
            row.measure.to_string(),
            row.value,
        ],
    );
}

fn by_kind(out: &mut String, group: &str, kind: &str, measure: &str, value: String) {
    add(
        out,
        Row {
            table: "losses_by_kind",
            group,
            kind,
            threshold: String::new(),
            ply: String::new(),
            measure,
            value,
        },
    );
}

fn summary_csv(f: &Findings) -> String {
    let mut out = String::new();
    line(
        &mut out,
        &[
            "table",
            "group",
            "kind",
            "threshold",
            "ply",
            "measure",
            "value",
        ]
        .map(String::from),
    );
    let focus: Vec<&Loss> = f.losses.iter().filter(|l| l.focus).collect();
    let others: Vec<&Loss> = f.losses.iter().filter(|l| !l.focus).collect();
    for (group, losses) in [("focus", &focus), ("others", &others)] {
        for kind in f.kinds_found() {
            let found = with_moment(losses, kind);
            by_kind(&mut out, group, kind, "count", found.len().to_string());
            by_kind(
                &mut out,
                group,
                kind,
                "median_turn",
                num(median_turn(found.iter())),
            );
        }
        let none = losses.iter().filter(|l| l.found.is_none()).count();
        by_kind(&mut out, group, "none", "count", none.to_string());
        by_kind(&mut out, group, "all", "count", losses.len().to_string());
        by_kind(
            &mut out,
            group,
            "all",
            "median_turn",
            num(median_of_turns(losses)),
        );
    }
    if f.exposure_metric.is_some() {
        exposure_rows(&mut out, f);
    }
    out
}

fn exposure_rows(out: &mut String, f: &Findings) {
    let all: Vec<&Loss> = f.losses.iter().collect();
    for kind in f.kinds_found() {
        let losses: Vec<Vec<Option<i64>>> = with_moment(&all, kind)
            .into_iter()
            .map(|x| x.exposure)
            .collect();
        let mut groups = vec![("losses".to_string(), losses)];
        if let Some(c) = f.controls.iter().find(|c| c.kind == kind) {
            groups.push((format!("wins_at_turn_{}", c.turn), c.exposures()));
        }
        for (group, cases) in &groups {
            for k in 0..EXPOSURE_PLIES {
                let ply = (k + 1).to_string();
                let values: Vec<i64> = cases.iter().filter_map(|x| x[k]).collect();
                let floats: Vec<f64> = values.iter().map(|&v| v as f64).collect();
                let mut put = |threshold: String, measure: &str, value: String| {
                    add(
                        out,
                        Row {
                            table: "exposure",
                            group,
                            kind,
                            threshold,
                            ply: ply.clone(),
                            measure,
                            value,
                        },
                    );
                };
                put(String::new(), "n", values.len().to_string());
                put(String::new(), "median", num(median(&floats)));
                for &t in &f.thresholds {
                    let high = values.iter().filter(|&&v| v >= t).count();
                    put(t.to_string(), "at_or_above", high.to_string());
                    let pct = if values.is_empty() {
                        String::new()
                    } else {
                        format!("{:.1}", high as f64 * 100.0 / values.len() as f64)
                    };
                    put(t.to_string(), "share_pct", pct);
                }
            }
        }
    }
}

fn readme(f: &Findings) -> String {
    let mut out = String::from("# Decisive moments of our losses: data\n\n");
    let _ = writeln!(
        out,
        "{} games used, {} of them our losses. Made by `codingame-moments --out`; the report that links this folder reads these numbers.\n",
        f.games,
        f.losses.len()
    );
    out.push_str("## Files\n\n");
    out.push_str("- `moments.csv`: one row per loss (group `loss`), and one row per win of the control (group `win_control`).\n");
    out.push_str("- `summary.csv`: the summary tables in long format.\n");
    out.push_str("- `examples.md`: for the top players, the first losses with the boards before and after the decisive moment, in code blocks.\n\n");
    out.push_str("## Columns of moments.csv\n\n");
    out.push_str("- `group`: `loss`, or `win_control` (one of our wins, read at the median turn of the decisive moments of the row's kind: at the first opponent ply of that turn; wins that ended earlier are left out).\n");
    out.push_str("- `game_id`, `opponent`, `rank` (in the leaderboard snapshot, empty if unranked), `our_seat` (side name).\n");
    let _ = writeln!(
        out,
        "- `kind`: the decisive kind ({}), or `none` for a loss without one; for a win, the kind whose median turn was used.",
        f.kinds.join(", ")
    );
    out.push_str(
        "- `ply`, `turn`: the ply of the moment (the control ply for a win) and its game turn.\n",
    );
    let keys: Vec<String> = f.metrics.iter().map(|k| format!("`{k}`")).collect();
    let _ = writeln!(
        out,
        "- `theirs_<metric>`, `ours_<metric>` for {}: each side's value right after its own last ply before the moment; empty if the side had no ply yet or the value means none. Empty for wins.",
        keys.join(", ")
    );
    if let Some(metric) = &f.exposure_metric {
        let _ = writeln!(
            out,
            "- `exposure_1` to `exposure_{EXPOSURE_PLIES}`: our `{metric}` right after our last, second to last, ... own ply before the moment (before the control ply for a win); empty if there was no such ply.\n"
        );
    }
    out.push_str("## Columns of summary.csv\n\n");
    out.push_str("- `table`: `losses_by_kind` (by `group` focus or others, and `kind`, with `none` and `all`) or `exposure` (by `group` losses or wins at the control turn, `kind`, `ply` 1 to 3).\n");
    out.push_str("- `threshold`: the exposure threshold, for `at_or_above` and `share_pct`; empty otherwise.\n");
    out.push_str("- `measure`: `count`, `median_turn`, `n` (cases with a value at that ply), `median` (of the exposure), `at_or_above` (cases at or above the threshold), `share_pct` (that share of `n`).\n");
    out.push_str("- `value`: the number.\n");
    out
}
