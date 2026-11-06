//! `codingame-report`: the profiles of the players met on CodinGame and our
//! weaknesses, from a data directory written by `scripts/codingame.sh`
//! (`logs/<game_id>.json`, `index.jsonl`, `leaderboard/<time>.json`).
//! Each log is converted, replayed and checked against CodinGame as
//! `codingame` does; a game that disagrees, or whose log is missing or
//! malformed, is left out and listed in the report's sources.
//! `codingame-moments` loads the same games and shows the decisive moment
//! of each of our losses.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use cg_arena::codingame::{read_log, to_record};
use cg_arena::record::Record;
use cg_review::moments;
use cg_review::profile::{analyze, markdown, GameMeta, Input, LeftOut, Options};
use cg_review::trace::Trace;
use serde_json::Value;

use crate::codingame::{check, parts, IceAndFire};
use crate::profiling::IceAndFire as Hooks;
use crate::replay::replay;
use crate::trace::{parse, trace};

/// What `codingame-report` is asked.
pub struct Request {
    pub data: PathBuf,
    /// Our nickname, to find our seat when the index does not say.
    pub me: String,
    /// The kinds of games to use, empty for all.
    pub kinds: Vec<String>,
    pub batch: Option<String>,
    pub top: usize,
    pub players: Vec<String>,
    pub leaderboard: Option<PathBuf>,
    pub jobs: usize,
}

/// The kinds of games the index holds.
const KINDS: [&str; 3] = ["ranking", "test", "replay"];

/// One line of the index that passes the filters.
struct Entry {
    meta: GameMeta,
    /// The line's players by seat, if it lists them.
    names: Vec<String>,
}

/// The lines of the index: the games to use, and the lines that are not
/// games (left out).
fn read_index(path: &Path, request: &Request) -> Result<(Vec<Entry>, Vec<LeftOut>), String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let (mut entries, mut left_out) = (Vec::new(), Vec::new());
    let mut seen = std::collections::HashSet::new();
    for (number, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let value: Value = match serde_json::from_str(line) {
            Ok(value) => value,
            Err(e) => {
                left_out.push(LeftOut {
                    game_id: None,
                    why: format!("index line {}: {e}", number + 1),
                });
                continue;
            }
        };
        let Some(game_id) = value["game_id"].as_u64() else {
            left_out.push(LeftOut {
                game_id: None,
                why: format!("index line {}: no game_id", number + 1),
            });
            continue;
        };
        let kind = value["kind"].as_str().unwrap_or("").to_string();
        if !KINDS.contains(&kind.as_str()) {
            left_out.push(LeftOut {
                game_id: Some(game_id),
                why: format!("unknown kind {kind:?} in the index"),
            });
            continue;
        }
        let batch = value["batch"].as_str().map(str::to_string);
        if !request.kinds.is_empty() && !request.kinds.contains(&kind) {
            continue;
        }
        if request.batch.is_some() && request.batch != batch {
            continue;
        }
        if !seen.insert(game_id) {
            left_out.push(LeftOut {
                game_id: Some(game_id),
                why: "listed twice in the index; the first line is used".to_string(),
            });
            continue;
        }
        let names = value["players"]
            .as_array()
            .map(|players| {
                let mut named: Vec<(u64, String)> = players
                    .iter()
                    .filter_map(|p| Some((p["seat"].as_u64()?, p["name"].as_str()?.to_string())))
                    .collect();
                named.sort();
                named.into_iter().map(|(_, n)| n).collect()
            })
            .unwrap_or_default();
        let our_seat = value["our_seat"]
            .as_u64()
            .and_then(|s| usize::try_from(s).ok());
        entries.push(Entry {
            meta: GameMeta {
                game_id,
                kind,
                batch,
                our_seat,
            },
            names,
        });
    }
    Ok((entries, left_out))
}

/// `record` with, in each answer, only the commands our engine carries out
/// (the loop of the check: each applied in order to a copy of the state,
/// stopping when the game ends), joined by `;`, or `WAIT` if none. A refused
/// command has no effect, so the game replays the same; the events and
/// openings then count what happened, not what was sent.
pub fn carried_out_record(record: &Record) -> Record {
    let game = replay("carried-out", record.clone());
    let mut out = record.clone();
    for (step, answers) in game.steps.iter().zip(out.turns.iter_mut()) {
        let seat = step.state.to_act();
        let Some(line) = answers
            .iter_mut()
            .find(|a| a.seat == seat)
            .and_then(|a| a.lines.first_mut())
        else {
            continue;
        };
        let mut state = step.state.clone();
        let mut kept = Vec::new();
        for part in parts(line) {
            if state.outcome().is_some() {
                break;
            }
            if parse(part).is_some_and(|command| state.apply(command)) {
                kept.push(part.to_string());
            }
        }
        *line = if kept.is_empty() {
            "WAIT".to_string()
        } else {
            kept.join(";")
        };
    }
    out
}

/// The trace of one game, or why it is left out.
fn load(data: &Path, entry: &Entry, me: &str) -> Result<(GameMeta, Trace), LeftOut> {
    let id = entry.meta.game_id;
    let left = |why: String| LeftOut {
        game_id: Some(id),
        why,
    };
    let path = data.join("logs").join(format!("{id}.json"));
    if !path.is_file() {
        return Err(left(format!("no log at {}", path.display())));
    }
    let log = read_log(&path).map_err(left)?;
    let record = to_record::<IceAndFire>(&log, &entry.meta.kind).map_err(left)?;
    let checked = check(&log, &record);
    if let Some(finding) = checked.findings.first() {
        return Err(left(format!(
            "our engine disagrees with CodinGame ({} findings, first: check {}, {})",
            checked.findings.len(),
            finding.check,
            finding.detail
        )));
    }
    let mut meta = entry.meta.clone();
    if meta.our_seat.is_none() {
        meta.our_seat = record.players.iter().position(|p| p.name == me);
    }
    let logged = [&record.players[0].name, &record.players[1].name];
    if entry.names.len() == 2 && entry.names.iter().zip(logged).any(|(a, b)| a != b) {
        return Err(left(format!(
            "the index names {:?} but the log {logged:?}",
            entry.names
        )));
    }
    Ok((meta, trace(&id.to_string(), &carried_out_record(&record))))
}

/// Runs `load` on every entry on `jobs` threads, in the order of `entries`.
fn load_all(
    data: &Path,
    entries: &[Entry],
    me: &str,
    jobs: usize,
) -> Vec<Result<(GameMeta, Trace), LeftOut>> {
    let jobs = jobs.clamp(1, 64);
    let size = entries.len().div_ceil(jobs).max(1);
    let mut slots: Vec<Option<Result<(GameMeta, Trace), LeftOut>>> = Vec::new();
    slots.resize_with(entries.len(), || None);
    std::thread::scope(|scope| {
        for (chunk, slot_chunk) in entries.chunks(size).zip(slots.chunks_mut(size)) {
            scope.spawn(move || {
                for (entry, slot) in chunk.iter().zip(slot_chunk.iter_mut()) {
                    *slot = Some(load(data, entry, me));
                }
            });
        }
    });
    slots
        .into_iter()
        .zip(entries)
        .map(|(slot, entry)| {
            slot.unwrap_or_else(|| {
                Err(LeftOut {
                    game_id: Some(entry.meta.game_id),
                    why: "not loaded".to_string(),
                })
            })
        })
        .collect()
}

/// The ranks of a leaderboard snapshot, by nickname, and its name.
fn read_ranks(path: &Path) -> Result<BTreeMap<String, u32>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let value: Value =
        serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let rows = value["rows"]
        .as_array()
        .ok_or_else(|| format!("{}: no rows", path.display()))?;
    let mut ranks = BTreeMap::new();
    for row in rows {
        if let (Some(name), Some(rank)) = (row["nickname"].as_str(), row["rank"].as_u64()) {
            ranks
                .entry(name.to_string())
                .or_insert(u32::try_from(rank).unwrap_or(u32::MAX));
        }
    }
    Ok(ranks)
}

/// The newest snapshot in `data/leaderboard` (the names are times).
fn newest_snapshot(data: &Path) -> Option<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(data.join("leaderboard"))
        .ok()?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort();
    files.pop()
}

/// The games of the request, loaded and checked, with the leaderboard.
fn load_input(request: &Request) -> Result<Input, String> {
    let (entries, mut left_out) = read_index(&request.data.join("index.jsonl"), request)?;
    let mut games = Vec::new();
    for result in load_all(&request.data, &entries, &request.me, request.jobs) {
        match result {
            Ok(game) => games.push(game),
            Err(left) => left_out.push(left),
        }
    }
    let snapshot = request
        .leaderboard
        .clone()
        .or_else(|| newest_snapshot(&request.data));
    let (ranks, name) = match &snapshot {
        Some(path) => (
            read_ranks(path)?,
            path.file_name().map(|n| n.to_string_lossy().into_owned()),
        ),
        None => (BTreeMap::new(), None),
    };
    let mut warnings = Vec::new();
    let mismatched: Vec<&(GameMeta, Trace)> = games
        .iter()
        .filter(|(meta, trace)| {
            meta.our_seat
                .and_then(|s| trace.players.get(s))
                .is_some_and(|name| *name != request.me)
        })
        .collect();
    if let Some((meta, trace)) = mismatched.first() {
        let seat = meta.our_seat.unwrap_or(0);
        warnings.push(format!(
            "--me {:?} differs from the player at our seat in {} games (for example game {}: {:?}); those games count that player as ours.",
            request.me,
            mismatched.len(),
            meta.game_id,
            trace.players[seat]
        ));
    }
    Ok(Input {
        warnings,
        notes: vec!["Events and openings count the commands carried out by the rules, not the ones refused (a refused command has no effect).".to_string()],
        games,
        ranks,
        snapshot: name,
        left_out,
    })
}

fn options(request: &Request) -> Options {
    Options {
        top: request.top,
        players: request.players.clone(),
    }
}

/// The report (Markdown) and its numbers (JSON).
pub fn run(request: &Request) -> Result<(String, String), String> {
    let input = load_input(request)?;
    let options = options(request);
    let analysis = analyze::<Hooks>(&input, &options);
    let json = serde_json::to_string_pretty(&analysis).map_err(|e| e.to_string())?;
    Ok((markdown(&analysis), json + "\n"))
}

/// The decisive moments of our losses, `examples` a focus player: the
/// Markdown report, or, with `out`, the files of that folder (and a line
/// that says so).
pub fn run_moments(
    request: &Request,
    examples: usize,
    out: Option<&Path>,
) -> Result<String, String> {
    let input = load_input(request)?;
    let findings = moments::analyze::<Hooks>(&input, &options(request), examples);
    let Some(dir) = out else {
        return Ok(moments::markdown(&findings));
    };
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let files = moments::files(&findings);
    for (name, content) in &files {
        let path = dir.join(name);
        std::fs::write(&path, content).map_err(|e| format!("{}: {e}", path.display()))?;
    }
    Ok(format!(
        "wrote {} files in {}\n",
        files.len(),
        dir.display()
    ))
}

#[cfg(test)]
mod tests;
