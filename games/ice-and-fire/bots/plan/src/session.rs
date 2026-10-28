//! The loop of a bot: read CodinGame's input, think, answer, flush. The
//! protocol is the policy crate's (`ice_and_fire_policy::io`), whose loop
//! cannot take another turn function.

use std::io::{BufRead, Write};
use std::time::Instant;

use ice_and_fire_engine::{answer_line, League, State};
use ice_and_fire_opening::planned::Planned;

use crate::search::{think, Limit};

/// Lines of a turn before the building count: four numbers, then the map.
const HEADER_LINES: usize = 4 + 12;

/// Plays turns of a Bronze game until the input ends. `limit` gives the
/// search's limits from the instant the turn's first line arrived and the
/// number of the turn (1 first); `opening` plays the opening and keeps its
/// plan between turns. Prints a debug line per turn to stderr (debug output
/// goes there only).
pub fn play(
    limit: &dyn Fn(Instant, u32) -> Limit,
    mut opening: Planned,
    input: impl BufRead,
    mut output: impl Write,
) -> Result<(), String> {
    let league = League::Bronze;
    let mut lines = Lines { reader: input };

    // The initial input. End of input before it means there is no game.
    let Some(first) = lines.line()? else {
        return Ok(());
    };
    let mut initial = format!("{first}\n");
    lines.take(count(&first, "mine spot count")?, &mut initial)?;

    let mut turn = 0;
    loop {
        // End of input before a new turn starts means the game is over.
        let Some(first) = lines.line()? else {
            return Ok(());
        };
        let start = Instant::now();
        turn += 1;
        let mut text = format!("{first}\n");
        lines.take(HEADER_LINES - 1, &mut text)?;
        // Buildings, then units: each a count and that many lines.
        for what in ["building count", "unit count"] {
            let line = lines.required()?;
            let n = count(&line, what)?;
            text.push_str(&line);
            text.push('\n');
            lines.take(n, &mut text)?;
        }

        let answer = match State::read(&initial, &text, league, turn) {
            Ok(state) => {
                let thought = think(&state, limit(start, turn), &mut opening);
                eprintln!(
                    "turn {turn} (plan): climbs {}, actions tried {}, memory {}, deep {}, opening plans {}, {:?}",
                    thought.climbs,
                    thought.evals,
                    thought.memory,
                    thought.deep,
                    opening.plans,
                    start.elapsed()
                );
                answer_line(&thought.commands)
            }
            Err(err) => {
                eprintln!("plan: cannot read the input: {err}");
                "WAIT".to_string()
            }
        };
        writeln!(output, "{answer}")
            .and_then(|()| output.flush())
            .map_err(|err| format!("turn {turn}: cannot write the answer: {err}"))?;
    }
}

struct Lines<R> {
    reader: R,
}

impl<R: BufRead> Lines<R> {
    /// The next line without its line ending, or `None` at the end of input.
    fn line(&mut self) -> Result<Option<String>, String> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => Ok(None),
            Ok(_) => Ok(Some(line.trim_end_matches(['\r', '\n']).to_string())),
            Err(err) => Err(format!("cannot read input: {err}")),
        }
    }

    fn required(&mut self) -> Result<String, String> {
        self.line()?
            .ok_or_else(|| "input ended in the middle of a turn".to_string())
    }

    /// Appends the next `n` lines to `text`.
    fn take(&mut self, n: usize, text: &mut String) -> Result<(), String> {
        for _ in 0..n {
            text.push_str(&self.required()?);
            text.push('\n');
        }
        Ok(())
    }
}

/// A count line. A count over one line per cell is refused before the bot
/// waits for lines that CodinGame will never send.
fn count(line: &str, what: &str) -> Result<usize, String> {
    match line.trim().parse::<usize>() {
        Ok(n) if n <= MAX_LINES => Ok(n),
        _ => Err(format!("bad {what} {line:?}")),
    }
}

/// The most lines after a count: one per cell.
const MAX_LINES: usize = 144;
