//! A game's `LEADERBOARD.md`, generated from the league's results file
//! ([ADR 0038](../../../docs/adr/0038-incremental-league.md), decision 5).
//!
//! [`render`] is a pure function of the stored pairings, the ratings, the
//! journal's text and the set of release files, so the same inputs give the
//! same bytes and CI can check that the committed file matches. The
//! `leaderboard` command in [`crate::cli`] reads those inputs and writes or
//! checks the file.

use std::collections::{BTreeMap, BTreeSet};

use crate::league::{PairingResult, Rating};

/// The decision record the generated file links to, relative to the game's
/// folder (`games/<game>/`).
const ADR_LINK: &str = "../../docs/adr/0038-incremental-league.md";

/// The experiment that promoted a release, from a row of the journal's table.
#[derive(Clone, Debug, PartialEq)]
pub struct JournalEntry {
    /// The text of the row's first cell, a link such as
    /// `[E032](E032-puct.md)`: `E032`.
    pub experiment: String,
    /// The link's target, `E032-puct.md`, relative to the journal's folder.
    pub target: String,
    /// The row's second cell, the date.
    pub date: String,
}

/// The experiments of a journal's table rows that name a release in their
/// last cell as ``Promoted: `GAME-vNNN` ``, keyed by `vNNN`. Other rows and
/// malformed rows are ignored; two rows promoting the same release are an
/// error.
pub fn parse_journal(text: &str, game: &str) -> Result<BTreeMap<String, JournalEntry>, String> {
    let prefix = format!("{game}-v");
    let mut entries: BTreeMap<String, JournalEntry> = BTreeMap::new();
    for line in text.lines() {
        let Some((release, entry)) = parse_row(line, &prefix) else {
            continue;
        };
        if let Some(first) = entries.get(&release) {
            return Err(format!(
                "the journal promotes {game}-{release} twice: {} and {}",
                first.experiment, entry.experiment
            ));
        }
        entries.insert(release, entry);
    }
    Ok(entries)
}

/// The release a table row promotes (`vNNN`) and its experiment, or `None`
/// for a row that is not a promotion or is malformed.
fn parse_row(line: &str, prefix: &str) -> Option<(String, JournalEntry)> {
    let inner = line.trim().strip_prefix('|')?.strip_suffix('|')?;
    let cells: Vec<&str> = inner.split('|').map(str::trim).collect();
    if cells.len() < 3 {
        return None;
    }
    let (_, after) = cells.last()?.split_once("Promoted: `")?;
    let (token, _) = after.split_once('`')?;
    let digits = token.strip_prefix(prefix)?;
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let (experiment, target) = parse_link(cells[0])?;
    if !is_date(cells[1]) {
        return None;
    }
    Some((
        format!("v{digits}"),
        JournalEntry {
            experiment,
            target,
            date: cells[1].to_string(),
        },
    ))
}

/// The text and the target of a Markdown link that fills the whole cell.
fn parse_link(cell: &str) -> Option<(String, String)> {
    let inner = cell.strip_prefix('[')?.strip_suffix(')')?;
    let (text, target) = inner.split_once("](")?;
    let valid = |part: &str| !part.is_empty() && !part.contains(char::is_whitespace);
    (valid(text) && valid(target) && !target.contains(['(', ')']))
        .then(|| (text.to_string(), target.to_string()))
}

/// Whether `text` looks like `YYYY-MM-DD`.
fn is_date(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.len() == 10
        && bytes.iter().enumerate().all(|(at, byte)| {
            if at == 4 || at == 7 {
                *byte == b'-'
            } else {
                byte.is_ascii_digit()
            }
        })
}

/// A generated leaderboard and what to tell the user about it.
#[derive(Clone, Debug, PartialEq)]
pub struct Leaderboard {
    /// The Markdown file, every line ending with `\n`.
    pub text: String,
    /// Problems that do not stop the generation, one message each.
    pub warnings: Vec<String>,
}

/// `number` with a comma between thousands: `30400` is `30,400`.
pub fn thousands(number: u64) -> String {
    let digits = number.to_string();
    let mut text = String::with_capacity(digits.len() + digits.len() / 3);
    for (at, digit) in digits.chars().enumerate() {
        if at > 0 && (digits.len() - at).is_multiple_of(3) {
            text.push(',');
        }
        text.push(digit);
    }
    text
}

/// The leaderboard of `game` as the text of `LEADERBOARD.md`.
///
/// `pairings` are the stored pairings and `ratings` their ratings with
/// `anchor` at 0 ([`crate::league::rate`]); `ratings` is empty when there are
/// no pairings. `journal` is the text of the game's journal index and
/// `releases` holds the names of the bots that have a release file.
///
/// Rows are sorted by rating, best first, ties by name; bots not linked to
/// the anchor come last, by name, without a rank.
pub fn render(
    game: &str,
    pairings: &[PairingResult],
    ratings: &[Rating],
    anchor: &str,
    journal: &str,
    releases: &BTreeSet<String>,
) -> Result<Leaderboard, String> {
    let promoted = parse_journal(journal, game)?;
    let mut warnings = Vec::new();

    let mut order: Vec<&Rating> = ratings.iter().collect();
    order.sort_by(|x, y| {
        y.is_linked()
            .cmp(&x.is_linked())
            .then_with(|| {
                if x.is_linked() {
                    y.rating.total_cmp(&x.rating)
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .then_with(|| x.name.cmp(&y.name))
    });

    let mut text = String::new();
    text.push_str("# Leaderboard\n\n");
    text.push_str(&format!(
        "Generated by `{game}-arena leaderboard` from [league.jsonl](league.jsonl) \
         ([ADR 0038]({ADR_LINK})); do not edit by hand. {}\n\n",
        summary_sentences(pairings, anchor)
    ));
    text.push_str("| Rank | Bot | Elo | 95% interval | Games | Experiment | Date |\n");
    text.push_str("| ---: | --- | ---: | ---: | ---: | --- | --- |\n");
    let mut rank = 0;
    for rating in order {
        let name = &rating.name;
        let (rank_cell, elo, interval) = if !rating.is_linked() {
            ("—".to_string(), "n/a".to_string(), "n/a".to_string())
        } else {
            rank += 1;
            if *name == anchor {
                (rank.to_string(), "0.0".to_string(), "anchor".to_string())
            } else {
                (
                    rank.to_string(),
                    format!("{:+.1}", rating.rating),
                    format!("±{:.1}", rating.margin),
                )
            }
        };
        let (bot, experiment, date) = if releases.contains(name) {
            let bot = format!("[{name}](releases/{game}-{name}.rs)");
            match promoted.get(name) {
                Some(entry) => (
                    bot,
                    format!("[{}](journal/{})", entry.experiment, entry.target),
                    entry.date.clone(),
                ),
                None => {
                    warnings.push(format!(
                        "release {name} has no row in the journal that promotes {game}-{name}"
                    ));
                    (bot, "—".to_string(), "—".to_string())
                }
            }
        } else {
            (name.clone(), "baseline".to_string(), String::new())
        };
        let date = if date.is_empty() {
            String::new()
        } else {
            format!(" {date}")
        };
        text.push_str(&format!(
            "| {rank_cell} | {bot} | {elo} | {interval} | {} | {experiment} |{date} |\n",
            thousands(u64::from(rating.games))
        ));
    }
    Ok(Leaderboard { text, warnings })
}

/// The sentences after "do not edit by hand": the ratings and the time
/// limits, or that there are no pairings yet.
fn summary_sentences(pairings: &[PairingResult], anchor: &str) -> String {
    let Some(first) = pairings.first() else {
        return "No pairings yet.".to_string();
    };
    let games: u64 = pairings
        .iter()
        .map(|pairing| u64::from(pairing.games()))
        .sum();
    let same = pairings.iter().all(|pairing| {
        pairing.time_scale == first.time_scale
            && pairing.time_tolerance_ms == first.time_tolerance_ms
            && pairing.opening_plies == first.opening_plies
    });
    let limits = if same {
        format!(
            "Time limits: time scale {}, tolerance {} ms, {} opening plies.",
            first.time_scale, first.time_tolerance_ms, first.opening_plies
        )
    } else {
        "Time limits: mixed (see league.jsonl).".to_string()
    };
    format!(
        "Bradley-Terry ratings over {} {} and {} games, with `{anchor}` at 0 and 95% intervals. \
         {limits}",
        thousands(pairings.len() as u64),
        if pairings.len() == 1 {
            "pairing"
        } else {
            "pairings"
        },
        thousands(games)
    )
}

#[cfg(test)]
mod tests;
