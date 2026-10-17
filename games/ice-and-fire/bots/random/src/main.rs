//! Random A Code of Ice and Fire bot: plays the engine's random turn (moves,
//! trainings and buildings the rules accept, `State::random_turn`).
//!
//! It is the baseline every rating is anchored to. The input never names the
//! league, so the bot plays with all the rules (Bronze); the commands of a
//! turn are chosen on the state the bot reads, so each one is valid there.
//!
//! The seed comes from `CG_SEED` when the arena sets it, otherwise from the
//! clock. It is printed to stderr so a game can be replayed. A turn whose
//! text does not read as a state is answered with `WAIT`; a bad count line or
//! input that ends in the middle of a turn ends the bot with exit code 1.
//! Debug output goes to stderr only.

use std::io::{self, BufRead, Write};

use cg_core::rng::{seed_from_env_or_clock, Rng};
use ice_and_fire_engine::{answer_line, League, State};

/// Lines of a turn before the building count: four numbers, then the map.
const HEADER_LINES: usize = 4 + 12;

fn main() {
    let seed = seed_from_env_or_clock();
    eprintln!("random: seed {seed}");
    let stdin = io::stdin();
    let stdout = io::stdout();
    if let Err(err) = play(&mut Rng::new(seed), stdin.lock(), stdout.lock()) {
        eprintln!("random: {err}");
        std::process::exit(1);
    }
}

/// Plays turns until the referee closes the input.
fn play(rng: &mut Rng, input: impl BufRead, mut output: impl Write) -> Result<(), String> {
    let mut lines = Lines { reader: input };

    // The initial input. End of input before it means there is no game.
    let Some(first) = lines.line()? else {
        return Ok(());
    };
    let mut initial = format!("{first}\n");
    lines.take(count(&first, "mine spot count")?, &mut initial)?;

    let mut commands = Vec::new();
    let mut turn = 0;
    loop {
        // End of input before a new turn starts means the game is over.
        let Some(first) = lines.line()? else {
            return Ok(());
        };
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

        let answer = match State::read(&initial, &text, League::Bronze, turn) {
            Ok(state) => {
                state.random_turn(rng, &mut commands);
                answer_line(&commands)
            }
            Err(err) => {
                eprintln!("random: turn {turn}: cannot read the input: {err}");
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

#[cfg(test)]
mod tests;
