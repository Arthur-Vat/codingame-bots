//! Markdown helpers for the decisive moments: numbers, shares, tables and the
//! sources section.

use std::fmt::Write as _;

use crate::profile::{Sources, SMALL};

/// A number with at most one decimal.
pub(super) fn num(value: Option<f64>) -> String {
    match value {
        None => "-".to_string(),
        Some(v) if (v - v.round()).abs() < 0.05 => format!("{:.0}", v.round()),
        Some(v) => format!("{v:.1}"),
    }
}

/// " [n = 2]" for a small sample.
fn flag(n: usize) -> String {
    if n > 0 && n < SMALL {
        format!(" [n = {n}]")
    } else {
        String::new()
    }
}

/// "60% (3/5)", flagged when `n` is small.
pub(super) fn share(k: usize, n: usize) -> String {
    if n == 0 {
        return "- (0)".to_string();
    }
    format!("{}% ({k}/{n}){}", (k * 100 + n / 2) / n, flag(n))
}

fn cell(text: &str) -> String {
    text.replace('|', "\\|")
}

pub(super) fn table(out: &mut String, headers: &[&str], rows: &[Vec<String>]) {
    if rows.is_empty() {
        out.push_str("(none)\n\n");
        return;
    }
    let _ = writeln!(out, "| {} |", headers.join(" | "));
    let _ = writeln!(
        out,
        "|{}|",
        headers
            .iter()
            .enumerate()
            .map(|(i, _)| if i == 0 { "---" } else { "---:" })
            .collect::<Vec<_>>()
            .join("|")
    );
    for row in rows {
        let _ = writeln!(
            out,
            "| {} |",
            row.iter().map(|c| cell(c)).collect::<Vec<_>>().join(" | ")
        );
    }
    out.push('\n');
}

pub(super) fn sources(out: &mut String, sources: &Sources) {
    out.push_str("## Sources\n\n");
    for w in &sources.warnings {
        let _ = writeln!(out, "**Warning:** {w}\n");
    }
    for n in &sources.notes {
        let _ = writeln!(out, "{n}\n");
    }
    let rows: Vec<Vec<String>> = sources
        .by_kind
        .iter()
        .map(|k| {
            vec![
                k.kind.clone(),
                k.batch.clone().unwrap_or_else(|| "-".to_string()),
                k.games.to_string(),
            ]
        })
        .collect();
    table(out, &["kind", "batch", "games"], &rows);
    let total: usize = sources.by_kind.iter().map(|k| k.games).sum();
    let _ = writeln!(out, "{total} games used.");
    match &sources.snapshot {
        Some(s) => {
            let _ = writeln!(out, "Ranks from the leaderboard snapshot {s}.");
        }
        None => out.push_str("No leaderboard snapshot: players are unranked.\n"),
    }
    if sources.left_out.is_empty() {
        out.push_str("No game left out.\n\n");
    } else {
        let _ = writeln!(out, "\n{} games left out:\n", sources.left_out.len());
        for l in &sources.left_out {
            let id = l.game_id.map_or("-".to_string(), |g| g.to_string());
            let _ = writeln!(out, "- {id}: {}", l.why);
        }
        out.push('\n');
    }
}
