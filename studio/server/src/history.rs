//! The saved games ([ADR 0027](../../../docs/adr/0027-game-records.md)): one
//! JSON file per game record in a folder that is never committed, and the
//! routes to save, list, view, export and delete them.
//!
//! A file is named `<game>-<unix time>-<12 hex digits of a hash of the
//! record>.json`; its stem is the game's id. Listing reads every file each
//! time, which is plenty for a few thousand games.

use std::fs;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use std::sync::Mutex;

use cg_arena::record::{Record, RECORD_FORMAT};
use cg_arena::referee::{GameSetup, SEATS};
use cg_arena::runner::EndReason;
use serde_json::{json, Value};
use studio_game::StudioGame;

use crate::api::Response;
use crate::state::{lock, State};

const SECONDS_PER_DAY: u64 = 86_400;

/// The folder of saved games.
pub struct History {
    dir: PathBuf,
    /// Held while saving, so that two saves of one record cannot both win.
    writing: Mutex<()>,
}

/// Whether `id` can be a file stem of the history: no path can hide in it.
pub fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

/// FNV-1a over `bytes`: a hash that never changes between Rust versions.
fn content_hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// The id a record is saved under.
fn record_id(record: &Record) -> Result<String, String> {
    let bytes = serde_json::to_vec(record).map_err(|error| error.to_string())?;
    let hash = format!("{:016x}", content_hash(&bytes));
    Ok(format!(
        "{}-{}-{}",
        record.game,
        record.unix_time,
        &hash[..12]
    ))
}

impl History {
    pub fn new(dir: PathBuf) -> History {
        History {
            dir,
            writing: Mutex::new(()),
        }
    }

    fn path(&self, id: &str) -> PathBuf {
        self.dir.join(format!("{id}.json"))
    }

    /// Saves `record` (already validated). The id, and whether the same
    /// record was already there.
    fn save(&self, record: &Record) -> Result<(String, bool), String> {
        let id = record_id(record)?;
        let bytes = serde_json::to_vec(record).map_err(|error| error.to_string())?;
        let _writing = lock(&self.writing);
        fs::create_dir_all(&self.dir)
            .map_err(|error| format!("cannot create {}: {error}", self.dir.display()))?;
        let path = self.path(&id);
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut file) => {
                if let Err(error) = file.write_all(&bytes) {
                    drop(file);
                    let _ = fs::remove_file(&path);
                    return Err(format!("cannot write {}: {error}", path.display()));
                }
                Ok((id, false))
            }
            Err(error) if error.kind() == ErrorKind::AlreadyExists => Ok((id, true)),
            Err(error) => Err(format!("cannot write {}: {error}", path.display())),
        }
    }

    /// The record `id`: `Ok(None)` if there is no such file.
    fn read(&self, id: &str) -> Result<Option<Record>, String> {
        let path = self.path(id);
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(format!("cannot read {}: {error}", path.display())),
        };
        serde_json::from_slice(&bytes)
            .map(Some)
            .map_err(|error| format!("{} is not a game record: {error}", path.display()))
    }

    fn delete(&self, id: &str) -> bool {
        let _writing = lock(&self.writing);
        fs::remove_file(self.path(id)).is_ok()
    }

    /// Every readable record with its id. Files that cannot be read are
    /// skipped and named on stderr.
    fn all(&self) -> Vec<(String, Record)> {
        let Ok(entries) = fs::read_dir(&self.dir) else {
            return Vec::new();
        };
        let mut records = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            let id = match (path.extension(), path.file_stem().and_then(|s| s.to_str())) {
                (Some(extension), Some(id)) if extension == "json" && valid_id(id) => id,
                _ => continue,
            };
            match self.read(id) {
                Ok(Some(record)) => records.push((id.to_string(), record)),
                Ok(None) => {}
                Err(message) => eprintln!("studio: history: skipping: {message}"),
            }
        }
        records
    }
}

/// How a game ended, as the listing names it.
fn end_name(end: &EndReason) -> &'static str {
    match end {
        EndReason::Finished => "finished",
        EndReason::Timeout { .. } => "timeout",
        EndReason::Crash { .. } => "crash",
        EndReason::Invalid { .. } => "invalid",
        EndReason::Aborted { .. } => "aborted",
        EndReason::Resigned { .. } => "resigned",
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ResultFilter {
    X,
    O,
    Draw,
    Fault,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SourceFilter {
    Studio,
    Arena,
}

/// The filters of the listing; each one that is set must hold.
#[derive(Default)]
struct Filter {
    game: Option<String>,
    release: Option<String>,
    result: Option<ResultFilter>,
    source: Option<SourceFilter>,
    /// Inclusive bounds on the record's unix time, in seconds.
    from: Option<u64>,
    to: Option<u64>,
}

impl Filter {
    fn parse(query: &str) -> Result<Filter, String> {
        let mut filter = Filter::default();
        for pair in query.split('&').filter(|pair| !pair.is_empty()) {
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            if value.is_empty() {
                continue;
            }
            match key {
                "game" => filter.game = Some(value.to_string()),
                "release" => filter.release = Some(value.to_string()),
                "result" => {
                    filter.result = Some(match value {
                        "x" => ResultFilter::X,
                        "o" => ResultFilter::O,
                        "draw" => ResultFilter::Draw,
                        "fault" => ResultFilter::Fault,
                        _ => return Err("result must be x, o, draw or fault".to_string()),
                    })
                }
                "source" => {
                    filter.source = Some(match value {
                        "studio" => SourceFilter::Studio,
                        "arena" => SourceFilter::Arena,
                        _ => return Err("source must be studio or arena".to_string()),
                    })
                }
                "from" => filter.from = Some(day_start(value)?),
                "to" => filter.to = Some(day_start(value)? + SECONDS_PER_DAY - 1),
                _ => {}
            }
        }
        Ok(filter)
    }

    fn matches(&self, record: &Record) -> bool {
        if self.game.as_ref().is_some_and(|game| *game != record.game) {
            return false;
        }
        if self
            .release
            .as_ref()
            .is_some_and(|release| record.players.iter().all(|p| p.name != *release))
        {
            return false;
        }
        if let Some(result) = self.result {
            let fault = matches!(
                record.end,
                EndReason::Timeout { .. } | EndReason::Crash { .. } | EndReason::Invalid { .. }
            );
            let drawn = record.winner.is_none()
                && matches!(record.end, EndReason::Finished | EndReason::Aborted { .. });
            let holds = match result {
                ResultFilter::X => record.winner == Some(0),
                ResultFilter::O => record.winner == Some(1),
                ResultFilter::Draw => drawn,
                ResultFilter::Fault => fault,
            };
            if !holds {
                return false;
            }
        }
        if let Some(source) = self.source {
            let holds = match source {
                SourceFilter::Studio => record.source == "studio",
                SourceFilter::Arena => record.source.starts_with("arena"),
            };
            if !holds {
                return false;
            }
        }
        self.from.is_none_or(|from| record.unix_time >= from)
            && self.to.is_none_or(|to| record.unix_time <= to)
    }
}

/// The unix time of 00:00 UTC on `date`, written `YYYY-MM-DD`.
fn day_start(date: &str) -> Result<u64, String> {
    let bad = || format!("{date:?} is not a date written YYYY-MM-DD");
    let mut parts = date.split('-');
    let (Some(year), Some(month), Some(day), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Err(bad());
    };
    if year.len() != 4 || month.len() != 2 || day.len() != 2 {
        return Err(bad());
    }
    let year: i64 = year.parse().map_err(|_| bad())?;
    let month: i64 = month.parse().map_err(|_| bad())?;
    let day: i64 = day.parse().map_err(|_| bad())?;
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return Err(bad()),
    };
    if !(1..=days_in_month).contains(&day) {
        return Err(bad());
    }
    // Days since 1970-01-01 (the civil calendar algorithm, years from March).
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let month_from_march = (month + 9) % 12;
    let day_of_year = (153 * month_from_march + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = era * 146_097 + day_of_era - 719_468;
    u64::try_from(days)
        .map(|days| days * SECONDS_PER_DAY)
        .map_err(|_| bad())
}

/// A record replayed with its game.
struct Replay {
    frames: Vec<Value>,
    opening_turns: usize,
    /// The turns that replay: all, or all but a final invalid one.
    shown_turns: usize,
}

/// Checks `record` and replays it. Fails with a message naming the turn
/// that breaks the rules. The last turn may break them when the game ended
/// by an invalid answer: the arena records the answer that lost the game.
fn replay(game: &dyn StudioGame, record: &Record) -> Result<Replay, String> {
    if record.format != RECORD_FORMAT {
        return Err(format!(
            "unsupported record format {} (this studio reads format {RECORD_FORMAT})",
            record.format
        ));
    }
    if record.players.len() != SEATS || record.winner.is_some_and(|seat| seat >= SEATS) {
        return Err("the record's players or winner are not valid".to_string());
    }
    let setup = GameSetup {
        seed: record.seed,
        opening_plies: record.opening_plies,
    };
    let turns = &record.turns;
    let failed = |error| format!("the record does not replay: {error}");
    let (shown_turns, frames) = match game.frames(&setup, turns) {
        Ok(frames) => (turns.len(), frames),
        Err(error)
            if matches!(record.end, EndReason::Invalid { .. }) && error.turn + 1 == turns.len() =>
        {
            let frames = game.frames(&setup, &turns[..error.turn]).map_err(failed)?;
            (error.turn, frames)
        }
        Err(error) => return Err(failed(error)),
    };
    Ok(Replay {
        frames,
        opening_turns: game.opening_turns(&setup),
        shown_turns,
    })
}

/// The game of `record` and its replay, or the response saying why not.
fn checked<'a>(
    state: &'a State,
    record: &Record,
) -> Result<(&'a dyn StudioGame, Replay), Response> {
    let Some(game) = state.game(&record.game) else {
        return Err(Response::error(
            400,
            &format!("unknown game {:?}", record.game),
        ));
    };
    replay(game, record)
        .map(|replay| (game, replay))
        .map_err(|message| Response::error(400, &message))
}

fn parse_record(body: &[u8]) -> Result<Record, Response> {
    serde_json::from_slice(body)
        .map_err(|error| Response::error(400, &format!("invalid game record: {error}")))
}

fn save_checked(state: &State, record: &Record) -> Response {
    if let Err(response) = checked(state, record) {
        return response;
    }
    match state.history().save(record) {
        Ok((id, duplicate)) => Response::json(200, &json!({ "id": id, "duplicate": duplicate })),
        Err(message) => Response::error(500, &message),
    }
}

/// `POST /api/history`: saves the record in the body.
pub fn save(state: &State, body: &[u8]) -> Response {
    match parse_record(body) {
        Ok(record) => save_checked(state, &record),
        Err(response) => response,
    }
}

/// `POST /api/sessions/{id}/save`: saves the session's record.
pub fn save_session(state: &State, id: &str) -> Response {
    let Some(shared) = state.session(id) else {
        return Response::error(404, "unknown session");
    };
    let record = lock(&shared).record();
    save_checked(state, &record)
}

/// `GET /api/history?...`: the summaries of the saved games, newest first.
pub fn list(state: &State, query: &str) -> Response {
    let filter = match Filter::parse(query) {
        Ok(filter) => filter,
        Err(message) => return Response::error(400, &message),
    };
    let mut records: Vec<(String, Record)> = state
        .history()
        .all()
        .into_iter()
        .filter(|(_, record)| filter.matches(record))
        .collect();
    records.sort_by(|a, b| (b.1.unix_time, &b.0).cmp(&(a.1.unix_time, &a.0)));
    let summaries: Vec<Value> = records
        .iter()
        .map(|(id, record)| {
            json!({
                "id": id,
                "game": record.game,
                "unix_time": record.unix_time,
                "players": [record.players[0].name, record.players[1].name],
                "winner": record.winner,
                "end": end_name(&record.end),
                "turns": record.turns.len(),
                "source": record.source,
            })
        })
        .collect();
    Response::json(200, &Value::Array(summaries))
}

/// The record `id`, or the response saying why there is none.
fn stored(state: &State, id: &str) -> Result<Record, Response> {
    if !valid_id(id) {
        return Err(Response::error(400, "invalid game id"));
    }
    match state.history().read(id) {
        Ok(Some(record)) => Ok(record),
        Ok(None) => Err(Response::error(404, "unknown game")),
        Err(message) => Err(Response::error(500, &message)),
    }
}

/// `GET /api/history/{id}`: the record, as exported.
pub fn get(state: &State, id: &str) -> Response {
    match stored(state, id) {
        Ok(record) => Response::json(200, &serde_json::to_value(record).unwrap_or(Value::Null)),
        Err(response) => response,
    }
}

/// `DELETE /api/history/{id}`.
pub fn delete(state: &State, id: &str) -> Response {
    if !valid_id(id) {
        return Response::error(400, "invalid game id");
    }
    if state.history().delete(id) {
        Response::new(204, "application/json", Vec::new())
    } else {
        Response::error(404, "unknown game")
    }
}

fn view_of(state: &State, record: Record) -> Response {
    match checked(state, &record) {
        Ok((_, replay)) => Response::json(
            200,
            &json!({
                "record": record,
                "frames": replay.frames,
                "opening_turns": replay.opening_turns,
                "shown_turns": replay.shown_turns,
            }),
        ),
        Err(response) => response,
    }
}

/// `GET /api/history/{id}/view`: the record with its frames.
pub fn view(state: &State, id: &str) -> Response {
    match stored(state, id) {
        Ok(record) => view_of(state, record),
        Err(response) => response,
    }
}

/// `POST /api/view`: the same for a record that is not saved.
pub fn view_body(state: &State, body: &[u8]) -> Response {
    match parse_record(body) {
        Ok(record) => view_of(state, record),
        Err(response) => response,
    }
}
