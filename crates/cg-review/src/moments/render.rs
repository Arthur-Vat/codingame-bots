//! The decisive moments as Markdown for a phone: tables of at most six
//! columns, and the boards one after the other in fenced code blocks.

use std::fmt::Write as _;

use super::{median_turn, Findings, Found, Loss, EXPOSURE_PLIES};
use crate::profile::median;
use crate::profile::render::{num, share, sources, table};
use crate::trace::SIDES;

fn value(v: Option<i64>) -> String {
    v.map_or("-".to_string(), |v| v.to_string())
}

/// `text` ending with a period.
fn sentence(text: &str) -> String {
    let text = text.trim_end();
    if text.ends_with('.') {
        text.to_string()
    } else {
        format!("{text}.")
    }
}

/// The report.
pub fn markdown(f: &Findings) -> String {
    let mut out = String::from("# Decisive moments of our losses\n\n");
    sources(&mut out, &f.sources);
    summary(&mut out, f);
    exposure(&mut out, f);
    examples(&mut out, f);
    out
}

fn with_moment(losses: &[&Loss], kind: &str) -> Vec<Found> {
    losses
        .iter()
        .filter_map(|l| l.found.clone())
        .filter(|found| found.kind == kind)
        .collect()
}

fn summary(out: &mut String, f: &Findings) {
    out.push_str("## Summary\n\n");
    let all: Vec<&Loss> = f.losses.iter().collect();
    let found = all.iter().filter(|l| l.found.is_some()).count();
    let _ = writeln!(
        out,
        "We lost {} of {} games (draws are left out). The decisive kinds, in order of interest: {}. The decisive moment of a loss is the first moment against us of one of them; {} losses have one and {} have none.\n",
        all.len(),
        f.games,
        f.kinds.join(", "),
        found,
        all.len() - found
    );
    let focus: Vec<&Loss> = f.losses.iter().filter(|l| l.focus).collect();
    let others: Vec<&Loss> = f.losses.iter().filter(|l| !l.focus).collect();
    let mut rows = Vec::new();
    for kind in f.kinds_found() {
        let (a, b) = (with_moment(&focus, kind), with_moment(&others, kind));
        rows.push(vec![
            kind.to_string(),
            a.len().to_string(),
            num(median_turn(a.iter())),
            b.len().to_string(),
            num(median_turn(b.iter())),
        ]);
    }
    let none = |losses: &[&Loss]| losses.iter().filter(|l| l.found.is_none()).count();
    rows.push(vec![
        "no decisive moment".to_string(),
        none(&focus).to_string(),
        "-".to_string(),
        none(&others).to_string(),
        "-".to_string(),
    ]);
    rows.push(vec![
        "all losses".to_string(),
        focus.len().to_string(),
        num(median_of_turns(&focus)),
        others.len().to_string(),
        num(median_of_turns(&others)),
    ]);
    let _ = writeln!(
        out,
        "Losses by decisive kind. Focus players are the top {} of the snapshot that we met. The turn is the game turn of the play that made the moment (for \"all losses\", the turn the game ended).\n",
        f.focus.len()
    );
    table(
        out,
        &["kind", "focus", "median turn", "others", "median turn"],
        &rows,
    );
}

fn median_of_turns(losses: &[&Loss]) -> Option<f64> {
    let turns: Vec<f64> = losses.iter().map(|l| f64::from(l.turns)).collect();
    median(&turns)
}

fn exposure(out: &mut String, f: &Findings) {
    let Some(metric) = &f.exposure_metric else {
        return;
    };
    let _ = writeln!(
        out,
        "## Our exposure before the moment\n\nOur `{metric}` right after each of our last {EXPOSURE_PLIES} own plies before the decisive moment (1 is the last). \"> 0\" is the share of the losses with a value at that ply where it was above zero.\n"
    );
    let all: Vec<&Loss> = f.losses.iter().collect();
    let mut rows = Vec::new();
    for kind in f.kinds_found() {
        let founds = with_moment(&all, kind);
        let at = |k: usize| -> Vec<i64> { founds.iter().filter_map(|x| x.exposure[k]).collect() };
        let last: Vec<f64> = at(0).iter().map(|&v| v as f64).collect();
        let positive = |k: usize| {
            let values = at(k);
            share(values.iter().filter(|&&v| v > 0).count(), values.len())
        };
        rows.push(vec![
            kind.to_string(),
            founds.len().to_string(),
            num(median(&last)),
            positive(0),
            positive(1),
            positive(2),
        ]);
    }
    table(
        out,
        &[
            "kind",
            "losses",
            "median at 1",
            "> 0 at 1",
            "> 0 at 2",
            "> 0 at 3",
        ],
        &rows,
    );
}

fn board(out: &mut String, title: &str, lines: &[String]) {
    let _ = writeln!(out, "{title}\n\n```");
    for line in lines {
        let _ = writeln!(out, "{}", line.trim_end());
    }
    out.push_str("```\n\n");
}

fn examples(out: &mut String, f: &Findings) {
    out.push_str("## Examples\n\n");
    if f.focus.is_empty() || f.examples == 0 {
        out.push_str("No example requested or no focus player.\n\n");
        return;
    }
    if !f.primer.is_empty() {
        out.push_str("How to read the boards:\n\n");
        for line in &f.primer {
            let _ = writeln!(out, "- {line}");
        }
        out.push('\n');
    }
    for name in &f.focus {
        let losses: Vec<&Loss> = f.losses.iter().filter(|l| &l.opponent == name).collect();
        if losses.is_empty() {
            continue;
        }
        let rank = losses[0]
            .rank
            .map_or(String::new(), |r| format!("rank {r}, "));
        let found = losses.iter().filter(|l| l.found.is_some()).count();
        let _ = writeln!(
            out,
            "### {name} ({rank}{} losses, {found} with a decisive moment)\n",
            losses.len()
        );
        for loss in losses.iter().filter(|l| l.found.is_some()).take(f.examples) {
            example(out, f, loss);
        }
    }
}

fn example(out: &mut String, f: &Findings, loss: &Loss) {
    let Some(found) = &loss.found else { return };
    let _ = writeln!(
        out,
        "#### Game {}: we were {}, lost at turn {} ({})\n",
        loss.game_id, SIDES[loss.our_seat], loss.turns, loss.end
    );
    let _ = writeln!(
        out,
        "Decisive moment: `{}` at turn {} (ply {}), played by {}. {}\n",
        found.kind,
        found.turn,
        found.ply,
        SIDES[1 - loss.our_seat],
        sentence(&found.text)
    );
    let _ = writeln!(out, "Their answer: `{}`\n", found.commands);
    if let Some(metric) = &f.exposure_metric {
        // Oldest first, so that a growth reads left to right.
        let values: Vec<String> = found.exposure.iter().rev().map(|v| value(*v)).collect();
        let _ = writeln!(
            out,
            "Our `{metric}` over our last {EXPOSURE_PLIES} own plies before it, oldest first: {}.\n",
            values.join(" -> ")
        );
    }
    board(out, "Board before (the opponent to play):", &found.before);
    match &found.after {
        Some(after) => board(out, "Board after:", after),
        None => out.push_str("No board after: the game ended on this ply.\n\n"),
    }
    out.push_str("Metrics right after each side's last own ply before the moment:\n\n");
    let rows: Vec<Vec<String>> = f
        .metrics
        .iter()
        .enumerate()
        .map(|(i, key)| vec![key.clone(), value(found.theirs[i]), value(found.ours[i])])
        .collect();
    table(out, &["metric", "theirs", "ours"], &rows);
}
