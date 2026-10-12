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

use cg_arena::record::Record;
use serde_json::{json, Value};
use studio_core::history::{self, record_id, Filter, Row};

use crate::api::Response;
use crate::state::{lock, State};

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
    /// record was already there. A file of that name that is not a record
    /// (a write that was cut short) is replaced.
    fn save(&self, record: &Record) -> Result<(String, bool), String> {
        let id = record_id(record)?;
        let bytes = serde_json::to_vec(record).map_err(|error| error.to_string())?;
        let _writing = lock(&self.writing);
        fs::create_dir_all(&self.dir)
            .map_err(|error| format!("cannot create {}: {error}", self.dir.display()))?;
        if matches!(self.read(&id), Ok(Some(_))) {
            return Ok((id, true));
        }
        // Written next to its place, then renamed: a reader never sees half
        // a file. The listing ignores names without the `.json` extension.
        let temporary = self.dir.join(format!(".tmp-{id}-{}", std::process::id()));
        let path = self.path(&id);
        let written = fs::File::create(&temporary)
            .and_then(|mut file| file.write_all(&bytes).and_then(|()| file.sync_all()))
            .and_then(|()| fs::rename(&temporary, &path));
        if let Err(error) = written {
            let _ = fs::remove_file(&temporary);
            return Err(format!("cannot write {}: {error}", path.display()));
        }
        Ok((id, false))
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

    /// Deletes the file `id`: `Ok(false)` if there is none.
    fn delete(&self, id: &str) -> Result<bool, String> {
        let _writing = lock(&self.writing);
        let path = self.path(id);
        match fs::remove_file(&path) {
            Ok(()) => Ok(true),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(false),
            Err(error) => Err(format!("cannot delete {}: {error}", path.display())),
        }
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

/// Checks `record` against its game, or gives the response saying why not.
fn checked(state: &State, record: &Record) -> Result<(), Response> {
    history::checked(state.game(&record.game), record)
        .map(|_| ())
        .map_err(|message| Response::error(400, &message))
}

fn parse_record(body: &[u8]) -> Result<Record, Response> {
    history::parse_record(body).map_err(|message| Response::error(400, &message))
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
    let rows: Vec<Row> = state
        .history()
        .all()
        .iter()
        .map(|(id, record)| Row::new(id, record))
        .collect();
    let summaries: Vec<Value> = filter
        .select(rows)
        .iter()
        .map(|row| serde_json::to_value(row).unwrap_or(Value::Null))
        .collect();
    Response::json(200, &Value::Array(summaries))
}

/// `POST /api/filter?...`: the rows in the body (a JSON array of the rows of
/// `GET /api/history`) that pass the same filters, newest first. It reads no
/// saved game: the studio filters the arena games, which it fetched from the
/// published site, with the same rules as its own.
pub fn filter_rows(query: &str, body: &[u8]) -> Response {
    let filter = match Filter::parse(query) {
        Ok(filter) => filter,
        Err(message) => return Response::error(400, &message),
    };
    let rows: Vec<Row> = match serde_json::from_slice(body) {
        Ok(rows) => rows,
        Err(error) => {
            return Response::error(400, &format!("invalid request body: {error}"));
        }
    };
    let selected: Vec<Value> = filter
        .select(rows)
        .iter()
        .map(|row| serde_json::to_value(row).unwrap_or(Value::Null))
        .collect();
    Response::json(200, &Value::Array(selected))
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
    match state.history().delete(id) {
        Ok(true) => Response::new(204, "", Vec::new()),
        Ok(false) => Response::error(404, "unknown game"),
        Err(message) => Response::error(500, &message),
    }
}

fn view_of(state: &State, record: Record) -> Response {
    match history::view(state.game(&record.game), record) {
        Ok(view) => Response::json(200, &view),
        Err(message) => Response::error(400, &message),
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
