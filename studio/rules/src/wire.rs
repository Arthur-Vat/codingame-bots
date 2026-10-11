//! The shapes of the protocol, and their checking.
//!
//! Everything here comes from the browser, or from a file it imported, so
//! nothing is trusted: a shape that breaks a rule the studio relies on is an
//! error, not a panic later.

use cg_arena::record::RecordedAnswer;
use cg_arena::referee::SEATS;
use cg_arena::runner::EndReason;
use serde::Deserialize;
use studio_core::request::{think_ms_message, MAX_OPENING_PLIES, THINK_MS};
use studio_core::session::{GameResult, GameState, Mode, SeatConfig};
use studio_game::StudioGame;

/// A seat. A bot seat in an output also has a `name`, equal to its
/// release; it is ignored in an input.
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SeatWire {
    Human {
        name: String,
    },
    Bot {
        release: String,
        think_ms: u64,
        mode: Mode,
        #[serde(default)]
        fixed_iters: Option<u64>,
    },
}

/// A game in progress, as the browser holds it.
#[derive(Deserialize)]
pub struct StateWire {
    pub game: String,
    pub seed: u64,
    pub opening_plies: u32,
    pub seats: [SeatWire; SEATS],
    pub turns: Vec<Vec<RecordedAnswer>>,
    #[serde(default)]
    pub result: Option<GameResult>,
}

fn seat_of_end(end: &EndReason) -> Option<usize> {
    match end {
        EndReason::Timeout { seat, .. }
        | EndReason::Crash { seat, .. }
        | EndReason::Invalid { seat, .. }
        | EndReason::Resigned { seat } => Some(*seat),
        EndReason::Finished | EndReason::Aborted { .. } => None,
    }
}

/// The checked state.
pub fn to_state(wire: StateWire, game: &dyn StudioGame) -> Result<GameState, String> {
    if wire.opening_plies > MAX_OPENING_PLIES {
        return Err("opening_plies is too large".to_string());
    }
    let mut fixed_iters = [None; SEATS];
    let mut seats = Vec::new();
    for (index, seat) in wire.seats.into_iter().enumerate() {
        seats.push(match seat {
            SeatWire::Human { name } => SeatConfig::Human { name },
            SeatWire::Bot {
                release,
                think_ms,
                mode,
                fixed_iters: iters,
            } => {
                if !THINK_MS.contains(&think_ms) {
                    return Err(think_ms_message());
                }
                fixed_iters[index] = iters;
                SeatConfig::Bot {
                    release,
                    think_ms,
                    mode,
                }
            }
        });
    }
    let seats: [SeatConfig; SEATS] = seats
        .try_into()
        .map_err(|_| format!("seats must hold exactly {SEATS} seats"))?;
    for (index, turn) in wire.turns.iter().enumerate() {
        if let Some(answer) = turn.iter().find(|answer| answer.seat >= SEATS) {
            return Err(format!("turn {index}: seat {} is not a seat", answer.seat));
        }
    }
    if let Some(result) = &wire.result {
        if result.winner.is_some_and(|seat| seat >= SEATS)
            || seat_of_end(&result.end).is_some_and(|seat| seat >= SEATS)
        {
            return Err("the result names a seat that does not exist".to_string());
        }
    }
    let mut state = GameState::new(game, wire.seed, wire.opening_plies, seats);
    state.turns = wire.turns;
    state.result = wire.result;
    state.fixed_iters = fixed_iters;
    Ok(state)
}
