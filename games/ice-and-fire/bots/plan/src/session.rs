//! The loop of a bot: read CodinGame's input, think, answer, flush, with the
//! policy crate's loop (`ice_and_fire_policy::io::turns`).

use std::io::{BufRead, Write};
use std::time::Instant;

use ice_and_fire_engine::{answer_line, League, State};
use ice_and_fire_opening::planned::Planned;
use ice_and_fire_policy::io::turns;

use crate::search::{think, think_with, Limit};
use crate::strategy::{self, Guide, Memory};
use crate::sweep;

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
    // The roles of the strategic layer, kept beside the opening's plan; the
    // layer does nothing unless `strategy::STRATEGY` is set (E077).
    let mut roles = Memory::default();
    turns(
        input,
        output,
        |initial, text, turn, start| match State::read(initial, text, League::Bronze, turn) {
            Ok(state) => {
                let layer = strategy::guide(&state, &mut roles);
                // The sweeper's moves for the units behind the front (E097).
                let swept = sweep::sweep(&state);
                let thought = if !swept.is_empty() && sweep::MODE == 1 {
                    // Played first; the search plans the rest of the turn.
                    let mut after = state.clone();
                    for &command in &swept {
                        after.apply(command);
                    }
                    let mut thought = think(&after, limit(start, turn), &mut opening);
                    let mut commands = swept;
                    commands.append(&mut thought.commands);
                    thought.commands = commands;
                    thought
                } else if !swept.is_empty() && sweep::MODE == 2 && layer.is_none() {
                    // Offered to the search as one macro action.
                    let guide = Guide {
                        plan: swept,
                        offered: true,
                        ..Guide::default()
                    };
                    think_with(&state, limit(start, turn), &mut opening, Some(guide))
                } else {
                    match layer {
                        Some(guide) => {
                            think_with(&state, limit(start, turn), &mut opening, Some(guide))
                        }
                        None => think(&state, limit(start, turn), &mut opening),
                    }
                };
                eprintln!(
                    "turn {turn}: climbs {} tries {} memory {} checks {} forced {} strategic {} {} {:?}",
                    thought.climbs,
                    thought.evals,
                    thought.memory,
                    thought.checks,
                    thought.forced,
                    u8::from(thought.strategic),
                    thought.pieces,
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
