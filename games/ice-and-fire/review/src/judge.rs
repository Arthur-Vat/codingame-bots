//! `codingame-judge`: at each ply of chosen CodinGame players, what the plan
//! bot's search would play, and how it scores its own plan and the one the
//! player played (ADR 0056, decision 8).
//!
//! The games are those of a data directory of `scripts/codingame.sh`, read
//! as `codingame-report` reads them (index, logs, conversion, check, the
//! commands carried out). Every ply of the chosen players is written as a
//! bank block with a `played` line (`bank::block_played`), the blocks are
//! split over `--jobs` runs of the plan bot's analysis build
//! (`judge --positions FILE --tries N`), and the judgements it prints
//! are rendered by `cg_review::judge`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as Process;

use cg_review::judge::{
    judgements_csv, largest_gaps, readme, summary_csv, Judgement, Judging, MoveMatch,
};
use cg_review::profile::{focus_players, GameMeta, Options};
use cg_review::trace::Trace;
use ice_and_fire_engine::Command;
use ice_and_fire_tactics::eval::WIN;

use crate::bank::block_played;
use crate::profiles::{newest_snapshot, prepare, read_index, read_ranks, Entry, Request as Base};
use crate::profiling::IceAndFire as Hooks;
use crate::replay::replay;
use crate::trace::{parse, trace};

/// What `codingame-judge` is asked.
pub struct Request {
    pub data: PathBuf,
    /// Our nickname, to find our seat when the index does not say.
    pub me: String,
    /// The plan bot's analysis build.
    pub judge: PathBuf,
    /// Judge these players instead of the top of the snapshot.
    pub players: Vec<String>,
    /// Only our games played by this release (the index's `release`).
    pub release: Option<String>,
    pub top: usize,
    /// Tries of each search.
    pub tries: u64,
    /// At most this many plies, spread over the whole list.
    pub limit: Option<usize>,
    pub jobs: usize,
    pub leaderboard: Option<PathBuf>,
    /// The folder to write the files to, created if missing.
    pub out: PathBuf,
}

/// A score gap (ours less the played plan's) above which the played plan
/// counts as scoring much lower. The evaluation counts gold 1, income 3, an
/// active cell 4, a unit 10 per level (a level-1 unit costs 10 gold) and a
/// tower 10, so 20 is about two level-1 units, or five active cells, or 20
/// gold: a clear material difference, more than the noise of a search.
pub const GAP_THRESHOLD: i64 = 20;

/// Scores at or beyond this, in absolute value, are forced results: a won or
/// lost game is `WIN`, and a plan that lets the opponent take our HQ next
/// turn is penalised `WIN / 2`, so a quarter of `WIN` separates them from
/// every board evaluation.
pub const DECIDED_BOUND: i64 = WIN / 4;

/// The commands of an answer line, without `WAIT`.
fn commands(line: &str) -> Vec<Command> {
    line.split(';')
        .filter_map(|part| parse(part.trim()))
        .collect()
}

impl Judging for Hooks {
    fn compare(played: &str, ours: &str) -> MoveMatch {
        let (played, ours) = (commands(played), commands(ours));
        let sorted = |cs: &[Command]| {
            let mut words: Vec<String> = cs.iter().map(Command::to_text).collect();
            words.sort();
            words
        };
        if sorted(&played) == sorted(&ours) {
            MoveMatch::Same
        } else if matches!((played.first(), ours.first()), (Some(a), Some(b)) if a == b) {
            MoveMatch::First
        } else {
            MoveMatch::Different
        }
    }

    fn gap_threshold() -> i64 {
        GAP_THRESHOLD
    }

    fn decided_bound() -> i64 {
        DECIDED_BOUND
    }
}

/// One ply to judge.
struct Item {
    player: String,
    game_id: u64,
    ply: usize,
    turn: u32,
    phase: cg_review::trace::Phase,
    played: String,
    board: Vec<String>,
    block: String,
}

/// What the plan bot printed for a position.
#[derive(Debug, PartialEq, Eq)]
pub struct Output {
    pub name: String,
    pub ply: usize,
    pub ours: String,
    pub ours_score: i64,
    pub played_score: i64,
}

/// Parses one line of the plan bot's `judge` output.
pub fn parse_output(line: &str) -> Result<Output, String> {
    let fields: Vec<&str> = line.split('\t').collect();
    let bad = |what: &str| format!("{what} in {line:?}");
    if fields.len() != 6 {
        return Err(bad("not 6 fields"));
    }
    let field = |i: usize, key: &str| {
        fields[i]
            .strip_prefix(key)
            .ok_or_else(|| bad(&format!("no {key}")))
    };
    let number = |i: usize, key: &str| {
        field(i, key)?
            .parse::<i64>()
            .map_err(|_| bad(&format!("bad {key}")))
    };
    Ok(Output {
        name: fields[0].to_string(),
        ply: fields[1].parse().map_err(|_| bad("bad ply"))?,
        ours: field(2, "ours=")?.to_string(),
        ours_score: number(3, "ours_score=")?,
        played_score: number(4, "played_score=")?,
    })
}

/// What one judge process gave: its lines, and the lines it printed that are
/// not judgements.
type Chunk = Result<(Vec<Output>, Vec<String>), String>;

/// Runs the plan bot's `judge` on `blocks` (written to `file`) and parses
/// its lines; the bad lines come back as messages.
fn run_chunk(judge: &Path, file: &Path, count: usize, tries: u64) -> Chunk {
    let result = Process::new(judge)
        .arg("judge")
        .arg("--positions")
        .arg(file)
        .arg("--tries")
        .arg(tries.to_string())
        .output()
        .map_err(|e| format!("{}: {e}", judge.display()))?;
    if !result.status.success() {
        let err = String::from_utf8_lossy(&result.stderr);
        let tail: String = err
            .chars()
            .rev()
            .take(300)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        return Err(format!(
            "the judge failed on the bank chunk {} ({count} positions): {}: {}",
            file.display(),
            result.status,
            tail.trim()
        ));
    }
    // What the judge says on stderr (the plies it left out, and why) is passed on.
    eprint!("{}", String::from_utf8_lossy(&result.stderr));
    let (mut outputs, mut bad) = (Vec::new(), Vec::new());
    for line in String::from_utf8_lossy(&result.stdout).lines() {
        if line.trim().is_empty() {
            continue;
        }
        match parse_output(line) {
            Ok(output) => outputs.push(output),
            Err(e) => bad.push(e),
        }
    }
    Ok((outputs, bad))
}

/// Every `limit`-th ply of `items`, evenly spread, all when `limit` is none
/// or large enough.
fn spread<T>(items: Vec<T>, limit: Option<usize>) -> Vec<T> {
    let Some(limit) = limit.filter(|&l| l < items.len()) else {
        return items;
    };
    let total = items.len();
    let keep: BTreeSet<usize> = (0..limit).map(|i| i * total / limit).collect();
    items
        .into_iter()
        .enumerate()
        .filter(|(i, _)| keep.contains(i))
        .map(|(_, item)| item)
        .collect()
}

/// The plies of `players` in the games.
fn items(games: &[(GameMeta, Trace, cg_arena::record::Record)], players: &[String]) -> Vec<Item> {
    let mut out = Vec::new();
    for (meta, trace, record) in games {
        if !trace.players.iter().any(|p| players.contains(p)) {
            continue;
        }
        let game = replay(&meta.game_id.to_string(), record.clone());
        for (ply, p) in trace.plies.iter().enumerate().take(game.steps.len()) {
            let player = &trace.players[p.to_act];
            if !players.contains(player) {
                continue;
            }
            out.push(Item {
                player: player.clone(),
                game_id: meta.game_id,
                ply,
                turn: p.game_turn,
                phase: trace.phase(ply),
                played: ice_and_fire_engine::answer_line(&game.steps[ply].commands),
                board: p.board.clone(),
                block: block_played(&game, ply),
            });
        }
    }
    out
}

/// Judges the plies and writes the files of `request.out`:
/// `judgements.csv`, `summary.csv`, `largest-gaps.md` and `README.md`.
pub fn run(request: &Request) -> Result<(), String> {
    let base = Base {
        data: request.data.clone(),
        me: request.me.clone(),
        kinds: vec![],
        batch: None,
        release: request.release.clone(),
        our_seat: None,
        top: request.top,
        players: request.players.clone(),
        leaderboard: request.leaderboard.clone(),
        jobs: request.jobs,
    };
    let (entries, left_out) = read_index(&request.data.join("index.jsonl"), &base)?;
    let mut games = Vec::new();
    let mut unusable = left_out.len();
    for entry in &entries {
        match prepare_one(&request.data, entry, &request.me) {
            Ok(game) => games.push(game),
            Err(_) => unusable += 1,
        }
    }
    let snapshot = request
        .leaderboard
        .clone()
        .or_else(|| newest_snapshot(&request.data));
    let ranks = match &snapshot {
        Some(path) => read_ranks(path)?,
        None => BTreeMap::new(),
    };
    let pairs: Vec<(GameMeta, Trace)> = games
        .iter()
        .map(|(meta, trace, _)| (meta.clone(), trace.clone()))
        .collect();
    let refs: Vec<&(GameMeta, Trace)> = pairs.iter().collect();
    let options = Options {
        top: request.top,
        players: request.players.clone(),
    };
    let players = focus_players(&refs, &ranks, &options);
    if players.is_empty() {
        return Err("no player to judge: give --players, or a data directory with a leaderboard snapshot and games of its top players".to_string());
    }
    let all = items(&games, &players);
    if all.is_empty() {
        return Err(format!(
            "no ply to judge: none of {} has a ply in the games",
            players.join(", ")
        ));
    }
    let total = all.len();
    let all = spread(all, request.limit);
    eprintln!(
        "judging {} of {total} plies of {} players",
        all.len(),
        players.len()
    );

    // The blocks, dealt out over the jobs.
    let jobs = request.jobs.clamp(1, 64).min(all.len().max(1));
    static RUNS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let run = RUNS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("ice-and-fire-judge-{}-{run}", std::process::id()));
    fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut chunks: Vec<(PathBuf, usize)> = Vec::new();
    for job in 0..jobs {
        let text: String = all
            .iter()
            .skip(job)
            .step_by(jobs)
            .map(|item| item.block.as_str())
            .collect();
        let count = all.iter().skip(job).step_by(jobs).count();
        let file = dir.join(format!("chunk-{job}.bank"));
        fs::write(&file, text).map_err(|e| format!("{}: {e}", file.display()))?;
        chunks.push((file, count));
    }
    let results: Vec<Chunk> = std::thread::scope(|scope| {
        let handles: Vec<_> = chunks
            .iter()
            .map(|(file, count)| {
                scope.spawn(move || run_chunk(&request.judge, file, *count, request.tries))
            })
            .collect();
        handles
            .into_iter()
            .map(|h| {
                h.join()
                    .unwrap_or_else(|_| Err("a judge thread panicked".to_string()))
            })
            .collect()
    });
    if results.iter().all(Result::is_ok) {
        let _ = fs::remove_dir_all(&dir);
    }
    let mut outputs = BTreeMap::new();
    let mut malformed = Vec::new();
    for result in results {
        let (found, bad) = result?;
        malformed.extend(bad);
        for output in found {
            outputs.insert((output.name.clone(), output.ply), output);
        }
    }

    if outputs.is_empty() {
        return Err(format!(
            "the judge gave no judgement for {} positions (is {} the plan bot built with --features analysis?){}",
            all.len(),
            request.judge.display(),
            malformed
                .first()
                .map_or(String::new(), |m| format!("; first line it printed that is not one: {m}"))
        ));
    }
    let mut judgements = Vec::new();
    for item in &all {
        let Some(output) = outputs.get(&(item.game_id.to_string(), item.ply)) else {
            continue;
        };
        judgements.push(Judgement {
            player: item.player.clone(),
            rank: ranks.get(&item.player).copied(),
            game_id: item.game_id,
            ply: item.ply,
            turn: item.turn,
            phase: item.phase,
            ours: output.ours.clone(),
            played: item.played.clone(),
            ours_score: output.ours_score,
            played_score: output.played_score,
            board: item.board.clone(),
        });
    }
    let mut notes = vec![
        "One unit of score is one gold in the plan bot's evaluation (income counts 3, an active cell 4, a unit 10 per level, a tower 10); the gap threshold of 20 is about two level-1 units or five active cells, a clear material difference. Forced results (a win, or the opponent taking our HQ next turn) score 500,000 or more and are decided.".to_string(),
        format!(
            "{} games used ({} left out), players judged: {}.",
            games.len(),
            unusable,
            players.join(", ")
        ),
        format!(
            "{} of {total} plies of these players sent to the judge, {} judged, {} tries a search.",
            all.len(),
            judgements.len(),
            request.tries
        ),
        "The commands judged are those our engine carried out. The opening is treated as over, as `bank` does.".to_string(),
    ];
    if judgements.len() < all.len() {
        notes.push(format!(
            "{} plies were left out by the judge (positions it could not read).",
            all.len() - judgements.len()
        ));
    }
    if !malformed.is_empty() {
        notes.push(format!(
            "{} malformed judge lines were skipped, the first: {}",
            malformed.len(),
            malformed[0]
        ));
    }
    fs::create_dir_all(&request.out).map_err(|e| format!("{}: {e}", request.out.display()))?;
    let files = [
        ("judgements.csv", judgements_csv::<Hooks>(&judgements)),
        ("summary.csv", summary_csv::<Hooks>(&judgements)),
        ("largest-gaps.md", largest_gaps::<Hooks>(&judgements)),
        ("README.md", readme::<Hooks>(&notes)),
    ];
    for (name, text) in files {
        let file = request.out.join(name);
        fs::write(&file, text).map_err(|e| format!("{}: {e}", file.display()))?;
    }
    Ok(())
}

/// A game's meta, trace and record, or why it is left out.
fn prepare_one(
    data: &Path,
    entry: &Entry,
    me: &str,
) -> Result<(GameMeta, Trace, cg_arena::record::Record), String> {
    let (meta, record) = prepare(data, entry, me).map_err(|left| left.why)?;
    let name = meta.game_id.to_string();
    let t = trace(&name, &record);
    Ok((meta, t, record))
}

#[cfg(test)]
mod tests;
