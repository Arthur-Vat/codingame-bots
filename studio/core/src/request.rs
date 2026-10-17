//! The request to start a game, and its checking
//! (`POST /api/sessions` of the server, the `new` request of the browser's
//! program).

use std::ops::RangeInclusive;

use cg_arena::referee::SEATS;
use serde::Deserialize;

use crate::session::{Mode, SeatConfig};

/// The smallest and largest think time a computer seat accepts.
pub const THINK_MS: RangeInclusive<u64> = 10..=10_000;

/// The largest number of opening plies a request may ask for.
pub const MAX_OPENING_PLIES: u32 = 81;

/// The body of a request to start a game.
#[derive(Deserialize)]
pub struct CreateRequest {
    pub game: String,
    pub seed: Option<u64>,
    pub opening_plies: Option<u32>,
    pub seats: Vec<SeatRequest>,
}

/// A seat as requested, with its optional parts.
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SeatRequest {
    Human {
        name: Option<String>,
    },
    Bot {
        release: String,
        think_ms: Option<u64>,
        mode: Option<Mode>,
    },
}

/// A checked request: what a new game needs.
#[derive(Debug)]
pub struct NewGame {
    pub game: String,
    pub seed: u64,
    pub opening_plies: u32,
    pub seats: [SeatConfig; SEATS],
}

/// The message of a think time out of range.
pub fn think_ms_message() -> String {
    format!(
        "think_ms must be a number from {} to {}",
        THINK_MS.start(),
        THINK_MS.end()
    )
}

/// The message of a human seat in a game without human play.
pub const NO_HUMAN_PLAY: &str = "this game has no human play: both seats must be bots";

/// Checks `request`. `game_known` says whether its game is registered;
/// `human_play` says whether the game lets a human take a seat (only read
/// when the game is known); `known_releases` lists the game's releases, and
/// is only called once the rest of the request is right. The error is the
/// message of the 400 the server answers.
pub fn validate(
    request: CreateRequest,
    game_known: bool,
    human_play: bool,
    known_releases: impl FnOnce() -> Vec<String>,
) -> Result<NewGame, String> {
    if !game_known {
        return Err(format!("unknown game {:?}", request.game));
    }
    let Some(seed) = request.seed else {
        return Err("seed is required".to_string());
    };
    let opening_plies = request.opening_plies.unwrap_or(0);
    if opening_plies > MAX_OPENING_PLIES {
        return Err("opening_plies is too large".to_string());
    }
    if opening_plies > 0 {
        return Err(
            "openings are not supported in live games yet: opening_plies must be 0".to_string(),
        );
    }
    if request.seats.len() != SEATS {
        return Err(format!("seats must hold exactly {SEATS} seats"));
    }
    if !human_play
        && request
            .seats
            .iter()
            .any(|seat| matches!(seat, SeatRequest::Human { .. }))
    {
        return Err(NO_HUMAN_PLAY.to_string());
    }
    let known = known_releases();
    let mut seats = Vec::new();
    for (index, seat) in request.seats.into_iter().enumerate() {
        match seat {
            SeatRequest::Human { name } => {
                let name = name
                    .map(|name| name.trim().chars().take(40).collect::<String>())
                    .filter(|name| !name.is_empty())
                    .unwrap_or_else(|| format!("Player {}", index + 1));
                seats.push(SeatConfig::Human { name });
            }
            SeatRequest::Bot {
                release,
                think_ms,
                mode,
            } => {
                if !known.contains(&release) {
                    return Err(format!("unknown release {release:?}"));
                }
                let Some(think_ms) = think_ms.filter(|ms| THINK_MS.contains(ms)) else {
                    return Err(think_ms_message());
                };
                let Some(mode) = mode else {
                    return Err("mode must be \"fixed\" or \"realtime\"".to_string());
                };
                seats.push(SeatConfig::Bot {
                    release,
                    think_ms,
                    mode,
                });
            }
        }
    }
    let seats: [SeatConfig; SEATS] = seats
        .try_into()
        .map_err(|_| format!("seats must hold exactly {SEATS} seats"))?;
    Ok(NewGame {
        game: request.game,
        seed,
        opening_plies,
        seats,
    })
}
