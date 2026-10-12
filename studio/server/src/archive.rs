//! Builds an archive of arena games ([ADR 0031](../../../docs/adr/0031-arena-games-archive.md))
//! from folders of game records.
//!
//! Every record is read, replayed with the game's referee, and kept once
//! under its id. Nothing is written unless every file passes. The layout:
//!
//! ```text
//! <out>/index.json         {"format":1,"rows":[<Row>,...]}, newest first
//! <out>/records/<id>.json  one record, as the studio's history saves it
//! ```

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use cg_arena::record::Record;
use serde::Serialize;
use studio_core::games::{find, registered};
use studio_core::history::{checked, parse_record, record_id, Filter, Row};

use crate::history::valid_id;

/// The version of the archive's layout.
pub const ARCHIVE_FORMAT: u32 = 1;

/// What to build.
pub struct Options {
    /// The archive's folder: missing, or empty.
    pub out: PathBuf,
    /// When set, only records of this game are accepted.
    pub game: Option<String>,
    /// The folders of records, in the order they are read.
    pub inputs: Vec<PathBuf>,
}

/// What one input contributed.
#[derive(Debug, PartialEq, Eq)]
pub struct InputSummary {
    pub input: PathBuf,
    /// The records that passed, duplicates included.
    pub records: usize,
    /// The records not seen in an earlier input or earlier in this one.
    pub new: usize,
}

/// What was written.
#[derive(Debug, PartialEq, Eq)]
pub struct Summary {
    pub inputs: Vec<InputSummary>,
    pub written: usize,
    pub duplicates: usize,
}

/// The file `index.json`.
#[derive(Serialize)]
struct Index<'a> {
    format: u32,
    rows: &'a [Row],
}

/// Whether the file called `name` can hold a record.
fn is_record_name(name: &str) -> bool {
    name.ends_with(".json") && name != "index.json" && !name.starts_with('.')
}

/// Adds to `files` the candidate files under `dir`, folders walked in sorted
/// order. Symbolic links and other kinds of files are skipped.
fn walk(dir: &Path, files: &mut Vec<PathBuf>, errors: &mut Vec<String>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) => {
            errors.push(format!("{}: cannot read folder: {error}", dir.display()));
            return;
        }
    };
    let mut paths = Vec::new();
    for entry in entries {
        match entry {
            Ok(entry) => paths.push(entry.path()),
            Err(error) => errors.push(format!("{}: cannot read folder: {error}", dir.display())),
        }
    }
    paths.sort();
    for path in paths {
        let metadata = match fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) => {
                errors.push(format!("{}: cannot read: {error}", path.display()));
                continue;
            }
        };
        if metadata.is_dir() {
            walk(&path, files, errors);
        } else if metadata.is_file() {
            let name = path.file_name().map(|name| name.to_string_lossy());
            if name.is_some_and(|name| is_record_name(&name)) {
                files.push(path);
            }
        }
    }
}

/// The record in `path`, checked, with its id.
fn read_record(path: &Path, game: Option<&str>) -> Result<(String, Record), String> {
    let bytes = fs::read(path).map_err(|error| format!("cannot read: {error}"))?;
    let record = parse_record(&bytes)?;
    if let Some(game) = game {
        if record.game != game {
            return Err(format!(
                "the game is {:?}, not {game:?} (--game)",
                record.game
            ));
        }
    }
    let games = registered();
    checked(find(&games, &record.game), &record)?;
    let id = record_id(&record)?;
    if !valid_id(&id) {
        return Err(format!("{id:?} cannot be the id of a game"));
    }
    Ok((id, record))
}

/// Whether `out` can be written to: missing, or an empty folder.
fn check_out(out: &Path) -> Result<(), String> {
    match fs::metadata(out) {
        Ok(metadata) if metadata.is_dir() => {
            let mut entries = fs::read_dir(out)
                .map_err(|error| format!("{}: cannot read: {error}", out.display()))?;
            if entries.next().is_some() {
                Err(format!("{}: the output folder is not empty", out.display()))
            } else {
                Ok(())
            }
        }
        Ok(_) => Err(format!("{}: the output is not a folder", out.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("{}: cannot read: {error}", out.display())),
    }
}

/// Builds the archive. On failure, nothing is written and the messages say
/// every problem found, one per entry (`<path>: <reason>` for a file).
pub fn build(options: &Options) -> Result<Summary, Vec<String>> {
    check_out(&options.out).map_err(|message| vec![message])?;
    let mut errors = Vec::new();
    let mut seen = HashSet::new();
    let mut kept: Vec<(String, Record)> = Vec::new();
    let mut summaries = Vec::new();
    let mut duplicates = 0;
    for input in &options.inputs {
        match fs::metadata(input) {
            Ok(metadata) if metadata.is_dir() => {}
            Ok(_) => {
                errors.push(format!("{}: not a folder", input.display()));
                continue;
            }
            Err(error) => {
                errors.push(format!("{}: cannot read folder: {error}", input.display()));
                continue;
            }
        }
        let mut files = Vec::new();
        walk(input, &mut files, &mut errors);
        let mut summary = InputSummary {
            input: input.clone(),
            records: 0,
            new: 0,
        };
        for path in files {
            match read_record(&path, options.game.as_deref()) {
                Ok((id, record)) => {
                    summary.records += 1;
                    if seen.insert(id.clone()) {
                        summary.new += 1;
                        kept.push((id, record));
                    } else {
                        duplicates += 1;
                    }
                }
                Err(reason) => errors.push(format!("{}: {reason}", path.display())),
            }
        }
        summaries.push(summary);
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    write(&options.out, &kept).map_err(|message| vec![message])?;
    Ok(Summary {
        inputs: summaries,
        written: kept.len(),
        duplicates,
    })
}

/// Writes the records and the index into `out`.
fn write(out: &Path, kept: &[(String, Record)]) -> Result<(), String> {
    let records = out.join("records");
    fs::create_dir_all(&records)
        .map_err(|error| format!("{}: cannot create: {error}", records.display()))?;
    let mut rows = Vec::with_capacity(kept.len());
    for (id, record) in kept {
        let bytes = serde_json::to_vec(record).map_err(|error| error.to_string())?;
        let path = records.join(format!("{id}.json"));
        fs::write(&path, bytes)
            .map_err(|error| format!("{}: cannot write: {error}", path.display()))?;
        rows.push(Row::new(id, record));
    }
    let rows = Filter::default().select(rows);
    let index = Index {
        format: ARCHIVE_FORMAT,
        rows: &rows,
    };
    let bytes = serde_json::to_vec(&index).map_err(|error| error.to_string())?;
    let path = out.join("index.json");
    fs::write(&path, bytes).map_err(|error| format!("{}: cannot write: {error}", path.display()))
}
