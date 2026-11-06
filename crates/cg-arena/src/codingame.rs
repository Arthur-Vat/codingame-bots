//! CodinGame game logs converted into game records
//! ([ADR 0056](../../../docs/adr/0056-codingame-game-analysis.md), decision
//! 4). The log is the API's JSON log of a game
//! (`GET /games/<id>?format=json`); the conversion is generic, and a game
//! supplies what is specific to it through [`FromCodinGame`]: how its seed
//! is written in the game options and how its frames hold the players'
//! answers.

use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use crate::record::{Origin, Player, PlayerKind, Record, RecordedAnswer, RECORD_FORMAT};
use crate::referee::SEATS;
use crate::runner::EndReason;

/// A CodinGame JSON log. Members the conversion does not read are ignored;
/// members without a value are absent.
#[derive(Clone, Debug, Deserialize)]
pub struct Log {
    pub game_id: u64,
    /// Lines of `name=value`, for example `seed=12`.
    #[serde(default)]
    pub game_options: Option<String>,
    #[serde(default)]
    pub agents: Vec<Agent>,
    #[serde(default)]
    pub frames: Vec<Frame>,
    /// The final scores by agent index.
    #[serde(default)]
    pub scores: Vec<f64>,
    /// The agent indexes, the winner first.
    #[serde(default)]
    pub ranking: Vec<usize>,
    /// Why the game stopped early, for example `TIMEOUT`.
    #[serde(default)]
    pub fail_cause: Option<String>,
}

/// A player of a log.
#[derive(Clone, Debug, Deserialize)]
pub struct Agent {
    pub index: usize,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub agent_id: Option<i64>,
}

/// One frame of a log.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Frame {
    #[serde(default)]
    pub frame: Option<u64>,
    /// The agent the frame is about. Not reliable for the effects of an
    /// answer: a game reads the agent named in the text.
    #[serde(default)]
    pub agent: Option<usize>,
    #[serde(default)]
    pub referee: Option<String>,
    /// The effects of an answer, as text; `$0` and `$1` stand for the agents.
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub stderr: Option<String>,
    #[serde(default)]
    pub error: Option<FrameError>,
    /// An answer, joined as the game writes it.
    #[serde(default)]
    pub stdout: Option<String>,
}

/// The error a frame reports.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct FrameError {
    #[serde(default)]
    pub line: Option<i64>,
    #[serde(default)]
    pub message: Option<String>,
}

/// What a game provides to be converted.
pub trait FromCodinGame {
    /// The game's folder name, for example `ice-and-fire`.
    const GAME: &'static str;

    /// The seed in the game options (`game_options`, or an empty text).
    fn seed(options: &str) -> Result<u64, String>;

    /// The answers of every turn, in order, read from the frames.
    fn answers(log: &Log) -> Result<Vec<Vec<RecordedAnswer>>, String>;
}

/// Reads the log at `path`.
pub fn read_log(path: &Path) -> Result<Log, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

/// The seat named in a summary `$N timeout!`, if there is one.
fn timed_out_seat(log: &Log) -> Option<usize> {
    log.frames
        .iter()
        .filter_map(|frame| frame.summary.as_deref())
        .flat_map(str::lines)
        .find_map(|line| {
            let digits = line.trim().strip_prefix('$')?;
            let (seat, rest) = digits.split_at(digits.find(|c: char| !c.is_ascii_digit())?);
            (rest.trim() == "timeout!").then(|| seat.parse().ok())?
        })
}

/// How the game ended, from the log.
fn end_reason(log: &Log) -> Result<EndReason, String> {
    let id = log.game_id;
    let summary_seat = timed_out_seat(log);
    let cause = log.fail_cause.as_deref().filter(|c| !c.is_empty());
    if cause.is_none() && summary_seat.is_none() {
        return Ok(EndReason::Finished);
    }
    let error_seat = || {
        log.frames
            .iter()
            .find(|frame| frame.error.is_some())
            .and_then(|frame| frame.agent)
    };
    let seat = summary_seat.or_else(error_seat).ok_or_else(|| {
        format!("game {id}: it failed ({cause:?}) but the log does not say which agent")
    })?;
    if seat >= SEATS {
        return Err(format!("game {id}: the failing agent {seat} is not a seat"));
    }
    match cause {
        None | Some("TIMEOUT") => Ok(EndReason::Timeout {
            seat,
            limit_ms: 0.0,
        }),
        Some(other) => Ok(EndReason::Crash {
            seat,
            detail: other.to_string(),
        }),
    }
}

/// Converts `log` into a record of game `G`. `kind` is `ranking`, `test` or
/// `replay`. Times are not in the logs: every answer's `ms` is 0 and a
/// timeout's `limit_ms` is 0.
pub fn to_record<G: FromCodinGame>(log: &Log, kind: &str) -> Result<Record, String> {
    let id = log.game_id;
    let mut agents: Vec<&Agent> = log.agents.iter().collect();
    agents.sort_by_key(|agent| agent.index);
    if agents.len() != SEATS || agents.iter().enumerate().any(|(i, a)| a.index != i) {
        return Err(format!(
            "game {id}: expected {SEATS} agents with indexes 0 and 1, found {}",
            log.agents.len()
        ));
    }
    let mut agent_ids = [0_i64; SEATS];
    for (seat, agent) in agents.iter().enumerate() {
        agent_ids[seat] = agent
            .agent_id
            .ok_or_else(|| format!("game {id}: agent {seat} has no agent_id"))?;
    }
    let seed = G::seed(log.game_options.as_deref().unwrap_or(""))
        .map_err(|e| format!("game {id}: {e}"))?;
    let turns = G::answers(log).map_err(|e| format!("game {id}: {e}"))?;
    let [first, second] = match log.scores[..] {
        [first, second] => [first, second],
        _ => {
            return Err(format!(
                "game {id}: expected 2 scores, found {}",
                log.scores.len()
            ))
        }
    };
    let winner = if first == second {
        None
    } else {
        match log.ranking.first() {
            Some(&seat) if seat < SEATS => Some(seat),
            other => return Err(format!("game {id}: the ranking's first agent is {other:?}")),
        }
    };
    let end = end_reason(log)?;
    let unix_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());
    Ok(Record {
        format: RECORD_FORMAT,
        game: G::GAME.to_string(),
        seed,
        opening_plies: 0,
        unix_time,
        source: format!("codingame {kind} game {id}"),
        players: std::array::from_fn(|seat| Player {
            name: agents[seat].name.clone(),
            kind: PlayerKind::Bot,
            bot_seed: None,
            command: None,
            time_scale: 1.0,
            fixed_iters: None,
        }),
        turns,
        end,
        winner,
        origin: Some(Origin {
            site: "codingame".to_string(),
            game_id: id,
            kind: kind.to_string(),
            agent_ids,
        }),
    })
}

#[cfg(test)]
mod tests;
