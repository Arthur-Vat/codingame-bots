//! The loop of a bot: read CodinGame's input, think, answer, flush, with the
//! policy crate's loop (`ice_and_fire_policy::io::turns`).

use std::io::{BufRead, Write};
use std::time::Instant;

use ice_and_fire_engine::{answer_line, League, State};
use ice_and_fire_opening::planned::Planned;
use ice_and_fire_policy::io::turns;

use crate::search::{think, Limit};

/// Plays turns of a Bronze game until the input ends. `limit` gives the
/// search's limits from the instant the turn's first line arrived and the
/// number of the turn (1 first); `opening` plays the opening and keeps its
/// plan between turns. Prints a debug line per turn to stderr (debug output
/// goes there only).
pub fn play(
    limit: &dyn Fn(Instant, u32) -> Limit,
    mut opening: Planned,
    input: impl BufRead,
    output: impl Write,
) -> Result<(), String> {
    turns(
        input,
        output,
        |initial, text, turn, start| match State::read(initial, text, League::Bronze, turn) {
            Ok(state) => {
                let thought = think(&state, limit(start, turn), &mut opening);
                eprintln!(
                    "turn {turn}: climbs {} tries {} memory {} {:?}",
                    thought.climbs,
                    thought.evals,
                    thought.memory,
                    start.elapsed()
                );
                answer_line(&thought.commands)
            }
            Err(err) => {
                eprintln!("plan: cannot read the input: {err}");
                "WAIT".to_string()
            }
        },
    )
}
