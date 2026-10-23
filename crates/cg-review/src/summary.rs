//! A batch's summary, from the files of each game (`NAME.json`, the
//! [`GameSummary`]; `NAME.csv`, the metrics) and, when present, the
//! reviewer's notes (`NAME.review.md`, whose last fenced `json` block is a
//! list of [`Note`]s).
//!
//! It says which metrics go with the winner at three points of the game,
//! how often the side a kind of decisive moment favoured went on to win,
//! and whether the plays reviewers judged good or bad moved the lead the
//! way they said.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::trace::{GameSummary, SIDES};

/// One ply's metrics, read back from the CSV.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub to_act: usize,
    /// Each side's metrics, in the order of [`GameSummary::metrics`].
    pub values: [Vec<i64>; 2],
}

/// A game of the batch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    pub summary: GameSummary,
    pub rows: Vec<Row>,
}

/// A play a reviewer judged.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    /// The ply, as the game file numbers them.
    pub ply: usize,
    /// `red` or `blue`.
    pub side: String,
    /// `good` or `bad`.
    pub verdict: String,
    /// What was played and why it was good or bad.
    #[serde(default)]
    pub note: String,
}

/// Reads the metrics CSV written by [`crate::text::metrics_csv`] for a game
/// with `keys`.
pub fn parse_csv(text: &str, keys: &[String]) -> Result<Vec<Row>, String> {
    let mut lines = text.lines();
    let header = lines.next().ok_or("empty CSV")?;
    let columns: Vec<&str> = header.split(',').collect();
    let expected = 5 + 2 * keys.len();
    if columns.len() != expected {
        return Err(format!(
            "the header has {} columns, expected {expected}",
            columns.len()
        ));
    }
    let mut rows = Vec::new();
    for (number, line) in lines.enumerate() {
        let cells: Vec<&str> = line.split(',').collect();
        if cells.len() != expected {
            return Err(format!("row {}: {} columns", number + 1, cells.len()));
        }
        let to_act = SIDES
            .iter()
            .position(|s| *s == cells[2])
            .ok_or_else(|| format!("row {}: unknown side {}", number + 1, cells[2]))?;
        let mut values = [Vec::new(), Vec::new()];
        for (side, side_values) in values.iter_mut().enumerate() {
            for cell in &cells[5 + side * keys.len()..5 + (side + 1) * keys.len()] {
                side_values.push(
                    cell.parse::<i64>()
                        .map_err(|e| format!("row {}: {cell}: {e}", number + 1))?,
                );
            }
        }
        rows.push(Row { to_act, values });
    }
    Ok(rows)
}

/// The notes in a review: its last fenced `json` block.
pub fn parse_notes(review: &str) -> Result<Vec<Note>, String> {
    let start = review
        .rfind("```json")
        .ok_or("no fenced json block in the review")?;
    let body = &review[start + "```json".len()..];
    let end = body.find("```").ok_or("the json block is not closed")?;
    serde_json::from_str(&body[..end]).map_err(|e| format!("the json block: {e}"))
}

/// What [`load`] reads: the games, the reviewers' notes by game name, and
/// the reviews whose notes could not be read.
pub type Batch = (Vec<Game>, BTreeMap<String, Vec<Note>>, Vec<String>);

/// The games of the batch in `dir` (every `NAME.json` with its `NAME.csv`),
/// by name, and the notes of those that have a `NAME.review.md`. A review
/// whose notes cannot be read is reported in the second list.
pub fn load(dir: &Path) -> Result<Batch, String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            name.strip_suffix(".json").map(str::to_string)
        })
        .collect();
    names.sort();
    let mut games = Vec::new();
    let mut notes = BTreeMap::new();
    let mut problems = Vec::new();
    for name in names {
        let read = |ext: &str| fs::read_to_string(dir.join(format!("{name}.{ext}")));
        let summary: GameSummary = serde_json::from_str(&read("json").map_err(|e| e.to_string())?)
            .map_err(|e| format!("{name}.json: {e}"))?;
        let rows = parse_csv(
            &read("csv").map_err(|e| format!("{name}.csv: {e}"))?,
            &summary.metrics,
        )
        .map_err(|e| format!("{name}.csv: {e}"))?;
        if let Ok(review) = read("review.md") {
            match parse_notes(&review) {
                Ok(list) => {
                    notes.insert(name.clone(), list);
                }
                Err(e) => problems.push(format!("{name}.review.md: {e}")),
            }
        }
        games.push(Game { summary, rows });
    }
    Ok((games, notes, problems))
}

/// The lead of `side` at `row`: its value of the lead metric less the
/// other side's.
fn lead_of(game: &Game, row: usize, side: usize) -> Option<i64> {
    let index = game
        .summary
        .metrics
        .iter()
        .position(|k| *k == game.summary.lead)?;
    let values = &game.rows.get(row)?.values;
    Some(values[side][index] - values[1 - side][index])
}

/// The three points of the game the metrics are compared at, with their
/// ply in `game`, if the game has them.
fn checkpoints(game: &Game) -> [(&'static str, Option<usize>); 3] {
    let n = game.rows.len();
    let middle = game.summary.middle.filter(|&m| m < n);
    [
        ("At the start of the middle game", middle),
        (
            "Five turns into the middle game",
            middle.map(|m| m + 10).filter(|&p| p < n),
        ),
        (
            "At the start of the endgame",
            game.summary.endgame.filter(|&e| e < n),
        ),
    ]
}

/// A percentage, or a dash with no cases.
fn percent(part: usize, whole: usize) -> String {
    if whole == 0 {
        "–".to_string()
    } else {
        format!("{:.0}%", 100.0 * part as f64 / whole as f64)
    }
}

/// The summary of `games`, with the reviewers' `notes` by game name and the
/// `problems` met while reading them.
pub fn summarize(
    games: &[Game],
    notes: &BTreeMap<String, Vec<Note>>,
    problems: &[String],
) -> String {
    let mut out = String::from("# Summary of the batch\n\n");
    let decided: Vec<&Game> = games
        .iter()
        .filter(|g| g.summary.winner.is_some())
        .collect();

    // Results by player.
    let mut players: BTreeMap<String, [usize; 3]> = BTreeMap::new();
    for game in games {
        for side in 0..2 {
            let entry = players
                .entry(game.summary.players[side].clone())
                .or_default();
            match game.summary.winner {
                Some(w) if w == side => entry[side] += 1,
                None => entry[2] += 1,
                _ => {}
            }
        }
    }
    let turns: u32 = games.iter().map(|g| g.summary.last_turn).sum();
    let _ = writeln!(
        out,
        "{} games, {} decided; {:.1} turns on average.\n",
        games.len(),
        decided.len(),
        f64::from(turns) / games.len().max(1) as f64
    );
    out.push_str("| Player | Wins as red | Wins as blue | Draws |\n| --- | ---: | ---: | ---: |\n");
    for (name, [red, blue, draws]) in &players {
        let _ = writeln!(out, "| {name} | {red} | {blue} | {draws} |");
    }

    // Metrics at the checkpoints.
    out.push_str(
        "\n## Metrics that go with the winner\n\nFor each point of the game, in the decided games \
         that reach it: how often the winner's value of the metric was higher than the loser's, \
         lower or equal, and the mean of the winner's value less the loser's. Some metrics are \
         better low (see their meanings in a game file).\n",
    );
    let keys = games
        .first()
        .map(|g| g.summary.metrics.clone())
        .unwrap_or_default();
    for point in 0..3 {
        let label = decided.first().map_or("", |g| checkpoints(g)[point].0);
        let at: Vec<(&Game, usize)> = decided
            .iter()
            .filter_map(|g| checkpoints(g)[point].1.map(|ply| (*g, ply)))
            .collect();
        let _ = writeln!(out, "\n### {label} ({} games)\n", at.len());
        if at.is_empty() {
            out.push_str("No decided game reached it.\n");
            continue;
        }
        out.push_str("| Metric | Winner's higher | Lower | Equal | Mean difference |\n| --- | ---: | ---: | ---: | ---: |\n");
        for (index, key) in keys.iter().enumerate() {
            let (mut higher, mut lower, mut equal, mut sum) = (0, 0, 0, 0i64);
            for (game, ply) in &at {
                let w = game.summary.winner.unwrap_or(0);
                let values = &game.rows[*ply].values;
                let diff = values[w][index] - values[1 - w][index];
                sum += diff;
                match diff.signum() {
                    1 => higher += 1,
                    -1 => lower += 1,
                    _ => equal += 1,
                }
            }
            let _ = writeln!(
                out,
                "| {key} | {} ({higher}) | {lower} | {equal} | {:+.1} |",
                percent(higher, higher + lower),
                sum as f64 / at.len() as f64
            );
        }
    }

    // Decisive moments.
    out.push_str(
        "\n## Decisive moments\n\nFor each kind, in the decided games: how many games had one, how \
         many there were, and how often the side the first one favoured won.\n\n\
         | Kind | Games | Moments | First one's side won |\n| --- | ---: | ---: | ---: |\n",
    );
    let mut kinds: BTreeMap<String, (usize, usize, usize, usize)> = BTreeMap::new();
    for game in &decided {
        let w = game.summary.winner.unwrap_or(0);
        let mut seen: Vec<&str> = Vec::new();
        for moment in &game.summary.moments {
            let entry = kinds.entry(moment.kind.clone()).or_default();
            entry.1 += 1;
            if !seen.contains(&moment.kind.as_str()) {
                seen.push(&moment.kind);
                entry.0 += 1;
                if let Some(side) = moment.favours {
                    entry.3 += 1;
                    if side == w {
                        entry.2 += 1;
                    }
                }
            }
        }
    }
    for (kind, (games_with, count, won, judged)) in &kinds {
        let _ = writeln!(
            out,
            "| {kind} | {games_with} | {count} | {} ({won} of {judged}) |",
            percent(*won, *judged)
        );
    }

    // Reviewers' notes against the lead.
    let lead = games
        .first()
        .map_or(String::new(), |g| g.summary.lead.clone());
    let _ = writeln!(
        out,
        "\n## Reviewers' judgments against the metrics\n\n{} of {} games reviewed. For each play \
         a reviewer judged, the change of the player's lead in `{lead}` from the start of that \
         turn to the start of its next one; a judgment agrees when a good play raised the lead \
         and a bad one lowered it.\n",
        notes.len(),
        games.len()
    );
    let mut groups: BTreeMap<(String, String), (usize, i64, usize)> = BTreeMap::new();
    let mut unknown = 0;
    for game in games {
        let Some(list) = notes.get(&game.summary.name) else {
            continue;
        };
        for note in list {
            let side = SIDES.iter().position(|s| *s == note.side);
            let verdict = note.verdict.to_lowercase();
            let (Some(side), true) = (side, verdict == "good" || verdict == "bad") else {
                unknown += 1;
                continue;
            };
            let next = (note.ply + 2).min(game.rows.len().saturating_sub(1));
            let (Some(before), Some(after)) =
                (lead_of(game, note.ply, side), lead_of(game, next, side))
            else {
                unknown += 1;
                continue;
            };
            let phase = phase_of(game, note.ply);
            let delta = after - before;
            let agrees = if verdict == "good" {
                delta > 0
            } else {
                delta < 0
            };
            for key in [
                (verdict.clone(), phase.to_string()),
                (verdict.clone(), "all".to_string()),
            ] {
                let entry = groups.entry(key).or_default();
                entry.0 += 1;
                entry.1 += delta;
                if agrees {
                    entry.2 += 1;
                }
            }
        }
    }
    if groups.is_empty() {
        out.push_str("No judgments to compare.\n");
    } else {
        out.push_str("| Verdict | Phase | Plays | Mean change of the lead | Agrees |\n| --- | --- | ---: | ---: | ---: |\n");
        for ((verdict, phase), (count, sum, agree)) in &groups {
            let _ = writeln!(
                out,
                "| {verdict} | {phase} | {count} | {:+.1} | {} |",
                *sum as f64 / *count as f64,
                percent(*agree, *count)
            );
        }
    }
    if unknown > 0 {
        let _ = writeln!(
            out,
            "\n{unknown} judgments named no known side, verdict or ply and were left out."
        );
    }
    if !problems.is_empty() {
        out.push_str("\nReviews that could not be read:\n\n");
        for problem in problems {
            let _ = writeln!(out, "- {problem}");
        }
    }
    out
}

/// The phase of `ply` in `game`, as the CSV's phase column names it.
fn phase_of(game: &Game, ply: usize) -> &'static str {
    if game.summary.endgame.is_some_and(|e| ply >= e) {
        "end"
    } else if game.summary.middle.is_some_and(|m| ply >= m) {
        "middle"
    } else {
        "early"
    }
}

#[cfg(test)]
mod tests;
