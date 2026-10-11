//! Game records ([ADR 0027](../../../docs/adr/0027-game-records.md)): the
//! checking that a record replays and says how it ended truly, its id, and
//! the rows and filters of the list of saved games. Where the records are
//! kept is the caller's business.
//!
//! A saved game's id is `<game>-<unix time>-<12 hex digits of a hash of the
//! record>`; the server names its file with it.

use cg_arena::record::{Record, RECORD_FORMAT};
use cg_arena::referee::{GameSetup, Outcome, SEATS};
use cg_arena::runner::EndReason;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use studio_game::{live_game, StudioGame};

const SECONDS_PER_DAY: u64 = 86_400;

/// FNV-1a over `bytes`: a hash that never changes between Rust versions.
fn content_hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// The id a record is saved under.
pub fn record_id(record: &Record) -> Result<String, String> {
    let bytes = serde_json::to_vec(record).map_err(|error| error.to_string())?;
    let hash = format!("{:016x}", content_hash(&bytes));
    Ok(format!(
        "{}-{}-{}",
        record.game,
        record.unix_time,
        &hash[..12]
    ))
}

/// How a game ended, as the listing names it.
pub fn end_name(end: &EndReason) -> &'static str {
    match end {
        EndReason::Finished => "finished",
        EndReason::Timeout { .. } => "timeout",
        EndReason::Crash { .. } => "crash",
        EndReason::Invalid { .. } => "invalid",
        EndReason::Aborted { .. } => "aborted",
        EndReason::Resigned { .. } => "resigned",
    }
}

/// One line of the list of saved games.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Row {
    pub id: String,
    pub game: String,
    pub unix_time: u64,
    pub players: [String; SEATS],
    pub winner: Option<usize>,
    /// The name of the end, see [`end_name`].
    pub end: String,
    pub turns: usize,
    pub source: String,
}

impl Row {
    /// The row of the record saved as `id`.
    pub fn new(id: &str, record: &Record) -> Row {
        Row {
            id: id.to_string(),
            game: record.game.clone(),
            unix_time: record.unix_time,
            players: [
                record.players[0].name.clone(),
                record.players[1].name.clone(),
            ],
            winner: record.winner,
            end: end_name(&record.end).to_string(),
            turns: record.turns.len(),
            source: record.source.clone(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ResultFilter {
    X,
    O,
    Draw,
    Fault,
    Unfinished,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SourceFilter {
    Studio,
    Arena,
}

/// The filters of the listing; each one that is set must hold.
#[derive(Default)]
pub struct Filter {
    game: Option<String>,
    release: Option<String>,
    result: Option<ResultFilter>,
    source: Option<SourceFilter>,
    /// Inclusive bounds on the record's unix time, in seconds.
    from: Option<u64>,
    to: Option<u64>,
}

impl Filter {
    /// Reads a query string (`a=b&c=d`, without the `?`).
    pub fn parse(query: &str) -> Result<Filter, String> {
        let mut filter = Filter::default();
        for pair in query.split('&').filter(|pair| !pair.is_empty()) {
            let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
            let value = percent_decode(value)?;
            let value = value.as_str();
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
                        "unfinished" => ResultFilter::Unfinished,
                        _ => {
                            return Err("result must be x, o, draw, fault or unfinished".to_string())
                        }
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
                "to" => {
                    filter.to = Some(day_start(value)?.saturating_add(SECONDS_PER_DAY - 1));
                }
                _ => {}
            }
        }
        Ok(filter)
    }

    /// Whether `row` passes every filter that is set.
    pub fn matches(&self, row: &Row) -> bool {
        if self.game.as_ref().is_some_and(|game| *game != row.game) {
            return false;
        }
        if self
            .release
            .as_ref()
            .is_some_and(|release| row.players.iter().all(|name| name != release))
        {
            return false;
        }
        if let Some(result) = self.result {
            let end = row.end.as_str();
            let fault = matches!(end, "timeout" | "crash" | "invalid");
            let drawn = row.winner.is_none() && end == "finished";
            let holds = match result {
                ResultFilter::X => row.winner == Some(0),
                ResultFilter::O => row.winner == Some(1),
                ResultFilter::Draw => drawn,
                ResultFilter::Fault => fault,
                ResultFilter::Unfinished => end == "aborted",
            };
            if !holds {
                return false;
            }
        }
        if let Some(source) = self.source {
            let holds = match source {
                SourceFilter::Studio => row.source == "studio",
                SourceFilter::Arena => row.source.starts_with("arena"),
            };
            if !holds {
                return false;
            }
        }
        self.from.is_none_or(|from| row.unix_time >= from)
            && self.to.is_none_or(|to| row.unix_time <= to)
    }

    /// The rows that pass, newest first, ties broken by the id, last first.
    pub fn select(&self, rows: Vec<Row>) -> Vec<Row> {
        let mut rows: Vec<Row> = rows.into_iter().filter(|row| self.matches(row)).collect();
        rows.sort_by(|a, b| (b.unix_time, &b.id).cmp(&(a.unix_time, &a.id)));
        rows
    }
}

/// A query value with its `%XX` escapes and `+` (a space) decoded.
pub fn percent_decode(value: &str) -> Result<String, String> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut at = 0;
    while at < bytes.len() {
        match bytes[at] {
            b'+' => decoded.push(b' '),
            b'%' => {
                let byte = bytes
                    .get(at + 1..at + 3)
                    .and_then(|hex| std::str::from_utf8(hex).ok())
                    .filter(|hex| hex.bytes().all(|byte| byte.is_ascii_hexdigit()))
                    .and_then(|hex| u8::from_str_radix(hex, 16).ok())
                    .ok_or_else(|| format!("bad %-escape in {value:?}"))?;
                decoded.push(byte);
                at += 2;
            }
            byte => decoded.push(byte),
        }
        at += 1;
    }
    String::from_utf8(decoded).map_err(|_| format!("{value:?} is not valid UTF-8"))
}

/// The unix time of 00:00 UTC on `date`, written `YYYY-MM-DD`.
pub fn day_start(date: &str) -> Result<u64, String> {
    let bad = || format!("{date:?} is not a date written YYYY-MM-DD");
    let mut parts = date.split('-');
    let (Some(year), Some(month), Some(day), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Err(bad());
    };
    let digits = |text: &str, length: usize| {
        text.len() == length && text.bytes().all(|byte| byte.is_ascii_digit())
    };
    if !digits(year, 4) || !digits(month, 2) || !digits(day, 2) {
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
pub struct Replay {
    pub frames: Vec<Value>,
    pub opening_turns: usize,
    /// The turns that replay: all, or all but a final invalid one.
    pub shown_turns: usize,
}

/// Checks `record` and replays it. Fails with a message naming the turn
/// that breaks the rules. The last turn may break them when the game ended
/// by an invalid answer: the arena records the answer that lost the game.
pub fn replay(game: &dyn StudioGame, record: &Record) -> Result<Replay, String> {
    if record.format != RECORD_FORMAT {
        return Err(format!(
            "unsupported record format {} (this studio reads format {RECORD_FORMAT})",
            record.format
        ));
    }
    if record.winner.is_some_and(|seat| seat >= SEATS) {
        return Err("the record's winner is not a seat".to_string());
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
    check_result(game, record, setup, shown_turns)?;
    Ok(Replay {
        frames,
        opening_turns: game.opening_turns(&setup),
        shown_turns,
    })
}

/// Checks that the record's end and winner agree with each other and with
/// the referee: a finished game ends as the referee says, a fault or a
/// resignation is won by the other seat, an aborted game has no winner.
pub fn check_result(
    game: &dyn StudioGame,
    record: &Record,
    setup: GameSetup,
    shown_turns: usize,
) -> Result<(), String> {
    let lost_by = |seat: usize, what: &str| {
        if seat >= SEATS {
            Err(format!("the seat that {what} is not a seat"))
        } else if record.winner != Some(SEATS - 1 - seat) {
            Err(format!(
                "seat {seat} {what}, so seat {} must be the winner, not {:?}",
                SEATS - 1 - seat,
                record.winner
            ))
        } else {
            Ok(())
        }
    };
    match &record.end {
        EndReason::Finished => {
            let live = live_game(game, setup, &record.turns[..shown_turns])
                .map_err(|error| format!("the record does not replay: {error}"))?;
            let expected = match live.outcome() {
                Some(Outcome::Win(seat)) => Some(seat),
                Some(Outcome::Draw) => None,
                None => {
                    return Err(
                        "the record says the game finished, but it is not over after its turns"
                            .to_string(),
                    )
                }
            };
            if record.winner == expected {
                Ok(())
            } else {
                Err(format!(
                    "the referee ends the game with winner {expected:?}, the record says {:?}",
                    record.winner
                ))
            }
        }
        EndReason::Resigned { seat } => lost_by(*seat, "resigned"),
        EndReason::Timeout { seat, .. } => lost_by(*seat, "timed out"),
        EndReason::Crash { seat, .. } => lost_by(*seat, "crashed"),
        EndReason::Invalid { seat, .. } => lost_by(*seat, "answered invalidly"),
        EndReason::Aborted { .. } => {
            if record.winner.is_none() {
                Ok(())
            } else {
                Err("an aborted game has no winner".to_string())
            }
        }
    }
}

/// Checks `record` against its game (`None` when the game is not
/// registered) and replays it.
pub fn checked(game: Option<&dyn StudioGame>, record: &Record) -> Result<Replay, String> {
    let Some(game) = game else {
        return Err(format!("unknown game {:?}", record.game));
    };
    replay(game, record)
}

/// The record read from the bytes of a JSON document.
pub fn parse_record(bytes: &[u8]) -> Result<Record, String> {
    serde_json::from_slice(bytes).map_err(|error| format!("invalid game record: {error}"))
}

/// The record with its frames, as `GET /api/history/{id}/view` answers.
pub fn view(game: Option<&dyn StudioGame>, record: Record) -> Result<Value, String> {
    let replay = checked(game, &record)?;
    Ok(json!({
        "record": record,
        "frames": replay.frames,
        "opening_turns": replay.opening_turns,
        "shown_turns": replay.shown_turns,
    }))
}
