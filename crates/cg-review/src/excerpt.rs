//! Excerpt reviews (ADR 0044): instead of a whole game, a reviewer reads each
//! side's last few moves up to the game's decisive moment, so that one cheap
//! reviewer covers many games. The excerpts share one primer; a row per game
//! and the reviewers' notes give the batch's counts.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::text::{ply_title, write_ply};
use crate::trace::{Moment, Trace, SIDES};

/// The kinds of moment that make a game's decisive moment, in order of
/// preference: the first of the first kind found.
pub const DECISIVE_KINDS: [&str; 2] = ["large cut", "lead change"];

/// The plies an excerpt shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Window {
    /// The first ply shown.
    pub from: usize,
    /// The last ply shown, the play that caused the moment (or the game's
    /// last ply).
    pub last: usize,
    /// The decisive moment, if the rules found one.
    pub moment: Option<Moment>,
}

/// The game's decisive moment: the first moment of the first kind of
/// [`DECISIVE_KINDS`] that the game has.
pub fn decisive(trace: &Trace) -> Option<&Moment> {
    DECISIVE_KINDS
        .iter()
        .find_map(|kind| trace.moments.iter().find(|m| m.kind == *kind))
}

/// The plies of each side's last `moves` turns up to the play that caused
/// the decisive moment: a large cut's own ply, or the ply before the red ply
/// at which a lead change shows. Without a decisive moment, the game's last
/// plies. None for a game without plies.
pub fn window(trace: &Trace, moves: usize) -> Option<Window> {
    let n = trace.plies.len();
    if n == 0 {
        return None;
    }
    let moment = decisive(trace).cloned();
    let last = match &moment {
        Some(m) if m.kind == "lead change" => m.ply.saturating_sub(1),
        Some(m) => m.ply,
        None => n - 1,
    }
    .min(n - 1);
    let from = (last + 1).saturating_sub(2 * moves.max(1));
    Some(Window { from, last, moment })
}

/// The bots' names and the result, in one line.
fn result_line(trace: &Trace) -> String {
    match trace.winner {
        Some(w) => format!("{} ({}) won; {}", trace.players[w], SIDES[w], trace.end),
        None => format!("Draw; {}", trace.end),
    }
}

/// The excerpt of `trace` for a reviewer: the result, the decisive moment,
/// then each side's last `moves` turns up to it (board, metrics, commands)
/// and the position after the last of them. Without plies, a line that says
/// so.
pub fn excerpt_text(trace: &Trace, moves: usize) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "# Excerpt {}: red {} against blue {}\n",
        trace.name, trace.players[0], trace.players[1]
    );
    let _ = writeln!(
        out,
        "**Result:** {}; {} plies.\n",
        result_line(trace),
        trace.plies.len()
    );
    let Some(w) = window(trace, moves) else {
        out.push_str("The game has no plies.\n");
        return out;
    };
    match &w.moment {
        Some(m) => {
            let favours = m
                .favours
                .map_or(String::new(), |s| format!(", favours {}", SIDES[s]));
            let _ = writeln!(
                out,
                "**Decisive moment:** {} ({}{favours}): {}.\n",
                ply_title(trace, m.ply),
                m.kind,
                m.text
            );
        }
        None => out.push_str(
            "**Decisive moment:** none found by the rules (no large cut, no lead change); \
             the excerpt shows the game's last plies.\n\n",
        ),
    }
    let _ = writeln!(
        out,
        "**Shown:** plies {} to {}, each side's last {moves} turns up to the play that caused \
         it, then the position after it.",
        w.from, w.last
    );
    for index in w.from..=w.last {
        write_ply(&mut out, trace, index, true);
    }
    if w.last + 1 < trace.plies.len() {
        out.push_str("\n## The position after it\n");
        write_ply(&mut out, trace, w.last + 1, false);
    }
    out
}

/// What every excerpt of the game assumes: the rules in short, the board's
/// legend and each metric's meaning, written once for the batch.
pub fn primer_text(trace: &Trace) -> String {
    let mut out = String::from("# How to read the excerpts\n\n");
    for line in &trace.primer {
        let _ = writeln!(out, "- {line}");
    }
    out.push_str(
        "- Each ply is one player's turn: the position at its start, both sides' metrics, \
         then the commands the player sent and the gold they cost (spent).\n\n",
    );
    out.push_str("## Metrics\n\n| Metric | Meaning |\n| --- | --- |\n");
    for metric in &trace.metrics {
        let _ = writeln!(out, "| {} | {} |", metric.key, metric.meaning);
    }
    let _ = writeln!(
        out,
        "\nThe lead is the difference of `{}` between the sides.",
        trace.lead
    );
    out
}

/// The header of the batch's `games.csv`.
pub const GAMES_HEADER: &str = "game,red,blue,winner,plies,moment,moment_ply,favours,from,last";

/// The row of `trace` in the batch's `games.csv`: its name, the bots by
/// seat, the winning seat (`draw`), its plies, the decisive moment's kind,
/// ply and favoured side (empty without one), and the plies shown.
pub fn games_row(trace: &Trace, moves: usize) -> String {
    let winner = trace.winner.map_or("draw", |w| SIDES[w]);
    let w = window(trace, moves);
    let (kind, ply, favours) = match w.as_ref().and_then(|w| w.moment.as_ref()) {
        Some(m) => (
            m.kind.clone(),
            m.ply.to_string(),
            m.favours.map_or("", |s| SIDES[s]).to_string(),
        ),
        None => (String::new(), String::new(), String::new()),
    };
    let (from, last) = w.map_or((String::new(), String::new()), |w| {
        (w.from.to_string(), w.last.to_string())
    });
    format!(
        "{},{},{},{winner},{},{kind},{ply},{favours},{from},{last}",
        trace.name,
        trace.players[0],
        trace.players[1],
        trace.plies.len()
    )
}

/// A row of `games.csv`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameRow {
    pub game: String,
    pub players: [String; 2],
    /// The winning seat, `None` for a draw.
    pub winner: Option<usize>,
    pub moment: String,
    /// The side the moment favours, if any.
    pub favours: Option<usize>,
}

fn seat(name: &str) -> Option<usize> {
    SIDES.iter().position(|s| *s == name)
}

/// Reads `games.csv`.
pub fn parse_games(text: &str) -> Result<Vec<GameRow>, String> {
    let mut lines = text.lines();
    if lines.next() != Some(GAMES_HEADER) {
        return Err(format!("games.csv must start with `{GAMES_HEADER}`"));
    }
    lines
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let f: Vec<&str> = line.split(',').collect();
            if f.len() != 10 {
                return Err(format!(
                    "games.csv: `{line}` has {} fields, not 10",
                    f.len()
                ));
            }
            Ok(GameRow {
                game: f[0].to_string(),
                players: [f[1].to_string(), f[2].to_string()],
                winner: seat(f[3]),
                moment: f[5].to_string(),
                favours: seat(f[7]),
            })
        })
        .collect()
}

/// One judgment of an excerpt reviewer.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ExcerptNote {
    pub game: String,
    pub side: String,
    pub verdict: String,
    pub ply: usize,
    pub category: String,
    pub note: String,
}

/// The notes of one reviewer's reply: its last fenced `json` block.
pub fn parse_excerpt_notes(text: &str) -> Result<Vec<ExcerptNote>, String> {
    let start = text
        .rfind("```json")
        .ok_or("no fenced json block in the notes")?;
    let body = &text[start + "```json".len()..];
    let end = body.find("```").ok_or("the json block is not closed")?;
    serde_json::from_str(&body[..end]).map_err(|e| format!("the json block: {e}"))
}

/// The share `part` of `whole` as a percentage, or a dash.
fn percent(part: usize, whole: usize) -> String {
    if whole == 0 {
        "–".to_string()
    } else {
        format!("{}%", (part * 100 + whole / 2) / whole)
    }
}

/// The counts of an excerpt batch in `dir`: its `games.csv` and the
/// reviewers' notes, every `notes-*.md`. Markdown, ending with the notes
/// that could not be read or that name an unknown game or side.
pub fn excerpt_summary(dir: &Path) -> Result<String, String> {
    let games_file = dir.join("games.csv");
    let text =
        fs::read_to_string(&games_file).map_err(|e| format!("{}: {e}", games_file.display()))?;
    let games = parse_games(&text)?;
    if games.is_empty() {
        return Err(format!("{}: no games", games_file.display()));
    }
    let by_name: BTreeMap<&str, &GameRow> = games.iter().map(|g| (g.game.as_str(), g)).collect();

    let mut problems: Vec<String> = Vec::new();
    let mut notes: Vec<ExcerptNote> = Vec::new();
    let mut files: Vec<_> = fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("notes-") && n.ends_with(".md"))
        })
        .collect();
    files.sort();
    for file in &files {
        let name = file.file_name().and_then(|n| n.to_str()).unwrap_or("?");
        match fs::read_to_string(file)
            .map_err(|e| e.to_string())
            .and_then(|t| parse_excerpt_notes(&t))
        {
            Ok(list) => notes.extend(list),
            Err(e) => problems.push(format!("{name}: {e}")),
        }
    }

    let mut out = String::from("# Counts of the excerpt batch\n\n");
    // Results by bot and seat.
    let mut bots: Vec<&str> = games
        .iter()
        .flat_map(|g| g.players.iter().map(String::as_str))
        .collect();
    bots.sort_unstable();
    bots.dedup();
    let _ = writeln!(
        out,
        "{} games, {} notes from {} files.\n",
        games.len(),
        notes.len(),
        files.len()
    );
    out.push_str("| Bot | Games | Wins | Wins as red | Wins as blue |\n| --- | ---: | ---: | ---: | ---: |\n");
    for bot in &bots {
        let played = games
            .iter()
            .filter(|g| g.players.contains(&bot.to_string()));
        let (mut n, mut wins, mut red, mut blue) = (0, 0, 0, 0);
        for g in played {
            n += 1;
            if let Some(w) = g.winner {
                if g.players[w] == *bot {
                    wins += 1;
                    if w == 0 {
                        red += 1;
                    } else {
                        blue += 1;
                    }
                }
            }
        }
        let _ = writeln!(out, "| {bot} | {n} | {wins} | {red} | {blue} |");
    }

    // The decisive moments.
    out.push_str(
        "\n## Decisive moments\n\n| Kind | Games | Its side won |\n| --- | ---: | ---: |\n",
    );
    let mut kinds: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for g in &games {
        let kind = if g.moment.is_empty() {
            "none (last plies)".to_string()
        } else {
            g.moment.clone()
        };
        let entry = kinds.entry(kind).or_default();
        entry.0 += 1;
        if g.favours.is_some() && g.favours == g.winner {
            entry.1 += 1;
        }
    }
    for (kind, (n, won)) in &kinds {
        let _ = writeln!(out, "| {kind} | {n} | {} |", percent(*won, *n));
    }

    // Judgments by bot, verdict and category.
    let mut counts: BTreeMap<(String, String, String), usize> = BTreeMap::new();
    for note in &notes {
        let Some(game) = by_name.get(note.game.as_str()) else {
            problems.push(format!("a note names an unknown game `{}`", note.game));
            continue;
        };
        let Some(side) = seat(&note.side) else {
            problems.push(format!("{}: unknown side `{}`", note.game, note.side));
            continue;
        };
        let bot = game.players[side].clone();
        *counts
            .entry((bot, note.verdict.clone(), note.category.clone()))
            .or_default() += 1;
    }
    out.push_str("\n## Judgments by bot\n\n| Bot | Verdict | Category | Count |\n| --- | --- | --- | ---: |\n");
    let mut rows: Vec<_> = counts.into_iter().collect();
    rows.sort_by(|a, b| (&a.0 .0, &a.0 .1, b.1).cmp(&(&b.0 .0, &b.0 .1, a.1)));
    for ((bot, verdict, category), n) in rows {
        let _ = writeln!(out, "| {bot} | {verdict} | {category} | {n} |");
    }
    let reviewed: std::collections::BTreeSet<&str> =
        notes.iter().map(|n| n.game.as_str()).collect();
    let missing: Vec<&str> = games
        .iter()
        .map(|g| g.game.as_str())
        .filter(|g| !reviewed.contains(g))
        .collect();
    if !missing.is_empty() {
        problems.push(format!(
            "{} games have no note: {}",
            missing.len(),
            missing.join(", ")
        ));
    }
    if !problems.is_empty() {
        out.push_str("\n## Problems\n\n");
        for p in &problems {
            let _ = writeln!(out, "- {p}");
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests;
