//! Reading and checking a request of the analysis protocol
//! (`studio/README.md`, "The analysis program"). Every malformed request
//! gives an error message; nothing here panics.

use serde_json::{Map, Value};
use uttt_engine::{Board, Move, Status};

/// The most moves a request may list.
pub const MAX_MOVES: usize = 200;
const DEFAULT_ITERATIONS: u64 = 1_000_000;
const ITERATIONS: (u64, u64) = (1, 100_000_000);
const DEFAULT_REPORT_EVERY: u64 = 20_000;
const REPORT_EVERY: (u64, u64) = (1_000, 10_000_000);
const DEFAULT_DEPTH: u64 = 10;
const DEPTH: (u64, u64) = (1, 40);
const DEFAULT_SEED: u64 = 1;

/// A checked request, with the position its moves lead to.
#[derive(Debug)]
pub struct Request {
    pub board: Board,
    pub iterations: u64,
    pub report_every: u64,
    pub depth: usize,
    pub seed: u64,
}

/// How much of a text an error message quotes.
const QUOTED: usize = 40;

/// `text` as a JSON string, cut short when long, for an error message.
fn quoted(text: &str) -> String {
    let short: String = text.chars().take(QUOTED).collect();
    let cut = short.len() < text.len();
    let mut json = Value::String(short).to_string();
    if cut {
        json.insert_str(json.len() - 1, "...");
    }
    json
}

impl Request {
    /// Reads `text`, checks every field and plays the moves.
    pub fn parse(text: &str) -> Result<Request, String> {
        let value: Value =
            serde_json::from_str(text).map_err(|error| format!("invalid JSON: {error}"))?;
        let Value::Object(fields) = value else {
            return Err("the request must be a JSON object".to_string());
        };
        for name in fields.keys() {
            if !["moves", "iterations", "report_every", "depth", "seed"].contains(&name.as_str()) {
                return Err(format!("unknown field {}", quoted(name)));
            }
        }
        let moves = fields
            .get("moves")
            .ok_or_else(|| "missing field \"moves\"".to_string())?;
        let board = play(moves)?;
        let iterations = number(&fields, "iterations", DEFAULT_ITERATIONS, ITERATIONS)?;
        let report_every = number(&fields, "report_every", DEFAULT_REPORT_EVERY, REPORT_EVERY)?;
        let depth = number(&fields, "depth", DEFAULT_DEPTH, DEPTH)?;
        let seed = seed(&fields)?;
        Ok(Request {
            board,
            iterations,
            report_every,
            depth: depth as usize,
            seed,
        })
    }
}

/// An integer field within `(least, most)`, or `default` when absent.
fn number(
    fields: &Map<String, Value>,
    name: &str,
    default: u64,
    (least, most): (u64, u64),
) -> Result<u64, String> {
    let Some(value) = fields.get(name) else {
        return Ok(default);
    };
    let Some(number) = value.as_u64() else {
        return Err(format!(
            "\"{name}\" must be an integer from {least} to {most}"
        ));
    };
    if !(least..=most).contains(&number) {
        return Err(format!("\"{name}\" must be from {least} to {most}"));
    }
    Ok(number)
}

/// The seed: a decimal string of an unsigned 64-bit number.
fn seed(fields: &Map<String, Value>) -> Result<u64, String> {
    let Some(value) = fields.get("seed") else {
        return Ok(DEFAULT_SEED);
    };
    let text = value
        .as_str()
        .ok_or_else(|| "\"seed\" must be a string of decimal digits".to_string())?;
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("\"seed\" must be a string of decimal digits".to_string());
    }
    text.parse()
        .map_err(|_| "\"seed\" must be at most 18446744073709551615".to_string())
}

/// The position after the listed moves, each checked legal in turn.
fn play(moves: &Value) -> Result<Board, String> {
    let Value::Array(moves) = moves else {
        return Err("\"moves\" must be a list of strings".to_string());
    };
    if moves.len() > MAX_MOVES {
        return Err(format!(
            "\"moves\" has {} entries; at most {MAX_MOVES}",
            moves.len()
        ));
    }
    let mut board = Board::new();
    for (index, entry) in moves.iter().enumerate() {
        let number = index + 1;
        let Some(text) = entry.as_str() else {
            return Err(format!("move {number} must be a string"));
        };
        if board.status() != Status::Ongoing {
            return Err(format!(
                "move {number} ({}) comes after the game is over",
                quoted(text)
            ));
        }
        match parse_move(text) {
            Some(mv) if board.is_legal(mv) => board.play(mv),
            _ => return Err(format!("move {number} ({}) is not legal", quoted(text))),
        }
    }
    Ok(board)
}

/// "row col": two numbers from 0 to 8 separated by one space.
fn parse_move(text: &str) -> Option<Move> {
    let (row, col) = text.split_once(' ')?;
    let digit = |part: &str| {
        (part.len() == 1 && part.bytes().all(|byte| byte.is_ascii_digit()))
            .then(|| part.parse::<usize>().ok())
            .flatten()
    };
    Move::from_row_col(digit(row)?, digit(col)?)
}
