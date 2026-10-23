//! The files a review batch keeps for each game: the game file, which a
//! reviewer reads, and the metrics as CSV, which the summary reads.

use std::fmt::Write as _;

use crate::trace::{Trace, SIDES};

/// A side's name with a capital, for sentences.
fn side_title(side: usize) -> &'static str {
    ["Red", "Blue"][side]
}

/// The heading that names `ply` in the game file and in the reviewers'
/// notes.
pub fn ply_title(trace: &Trace, ply: usize) -> String {
    let p = &trace.plies[ply];
    format!(
        "Ply {ply} · turn {} · {} to play · {}",
        p.game_turn,
        SIDES[p.to_act],
        trace.phase(ply).name()
    )
}

/// The plies `from..to` as "plies A to B (turns C to D)".
fn span(trace: &Trace, from: usize, to: usize) -> String {
    if from >= to {
        return "none".to_string();
    }
    format!(
        "plies {from} to {} (turns {} to {})",
        to - 1,
        trace.plies[from].game_turn,
        trace.plies[to - 1].game_turn
    )
}

/// The game file: a Markdown page with the result, how to read it, the
/// phases, the decisive moments the rules found, then every ply's board,
/// metrics and commands.
pub fn game_text(trace: &Trace) -> String {
    let mut out = String::new();
    let n = trace.plies.len();
    let _ = writeln!(
        out,
        "# Game {}: red {} against blue {}\n",
        trace.name, trace.players[0], trace.players[1]
    );
    let turns = trace.plies.last().map_or(0, |p| p.game_turn);
    let result = match trace.winner {
        Some(w) => format!("{} ({}) won", trace.players[w], SIDES[w]),
        None => "Draw".to_string(),
    };
    let _ = writeln!(
        out,
        "**Result:** {result}; {}; {turns} turns, {n} plies.\n",
        trace.end
    );

    out.push_str("## How to read this file\n\n");
    for line in &trace.primer {
        let _ = writeln!(out, "- {line}");
    }
    out.push_str(
        "- Each ply is one player's turn: the position at its start, both sides' metrics, \
         then the commands the player sent and the gold they cost (spent).\n\n",
    );
    out.push_str("### Metrics\n\n| Metric | Meaning |\n| --- | --- |\n");
    for metric in &trace.metrics {
        let _ = writeln!(out, "| {} | {} |", metric.key, metric.meaning);
    }
    let _ = writeln!(
        out,
        "\nThe lead is the difference of `{}` between the sides.\n",
        trace.lead
    );

    out.push_str("## Phases\n\n");
    let middle = trace.middle.unwrap_or(n);
    let endgame = trace.endgame.unwrap_or(n).max(middle);
    let _ = writeln!(out, "- Early game: {}.", span(trace, 0, middle));
    let _ = writeln!(out, "- Middle game: {}.", span(trace, middle, endgame));
    let _ = writeln!(out, "- Endgame: {}.\n", span(trace, endgame, n));

    out.push_str("## Decisive moments found by rules\n\n");
    if trace.moments.is_empty() {
        out.push_str("None.\n");
    }
    for moment in &trace.moments {
        let favours = moment
            .favours
            .map_or(String::new(), |s| format!(", favours {}", SIDES[s]));
        let _ = writeln!(
            out,
            "- **{}** ({}{favours}): {}",
            ply_title(trace, moment.ply),
            moment.kind,
            moment.text
        );
    }

    out.push_str("\n## Plies\n");
    for (index, ply) in trace.plies.iter().enumerate() {
        let _ = writeln!(out, "\n### {}\n\n```text", ply_title(trace, index));
        for line in &ply.board {
            let _ = writeln!(out, "{line}");
        }
        out.push_str("```\n\n| side |");
        for metric in &trace.metrics {
            let _ = write!(out, " {} |", metric.key);
        }
        out.push_str("\n| --- |");
        for _ in &trace.metrics {
            out.push_str(" ---: |");
        }
        out.push('\n');
        for (name, values) in SIDES.iter().zip(&ply.metrics) {
            let _ = write!(out, "| {name} |");
            for value in values {
                let _ = write!(out, " {value} |");
            }
            out.push('\n');
        }
        let commands = if ply.commands.trim().is_empty() {
            "(nothing)".to_string()
        } else {
            format!("`{}`", ply.commands.trim())
        };
        let _ = writeln!(
            out,
            "\n{} played (spent {}): {commands}",
            side_title(ply.to_act),
            ply.spent
        );
    }
    out
}

/// The metrics as CSV: one row a ply, `ply,turn,to_act,phase,spent`, then
/// each metric for red (`red_KEY`) and for blue (`blue_KEY`).
pub fn metrics_csv(trace: &Trace) -> String {
    let mut out = String::from("ply,turn,to_act,phase,spent");
    for side in SIDES {
        for metric in &trace.metrics {
            let _ = write!(out, ",{side}_{}", metric.key);
        }
    }
    out.push('\n');
    for (index, ply) in trace.plies.iter().enumerate() {
        let phase = match trace.phase(index) {
            crate::trace::Phase::Early => "early",
            crate::trace::Phase::Middle => "middle",
            crate::trace::Phase::End => "end",
        };
        let _ = write!(
            out,
            "{index},{},{},{phase},{}",
            ply.game_turn, SIDES[ply.to_act], ply.spent
        );
        for side in 0..2 {
            for value in &ply.metrics[side] {
                let _ = write!(out, ",{value}");
            }
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests;
