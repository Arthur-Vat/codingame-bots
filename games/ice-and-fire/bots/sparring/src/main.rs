//! Sparring bot for A Code of Ice and Fire, in the style of zasmu, the top
//! player of the 2026-11-18 leaderboard, as the study
//! `games/ice-and-fire/research/codingame/2026-11-18-zasmu-strategy.md`
//! measured it: a tower wall along the front, mines on safe spots, level 1
//! units for the land, few fights, and gold saved for one strike on the enemy
//! HQ (stage A, piece 2, of that note's curriculum). It is an opponent for
//! our bots and a reference for the capability tests, not a release.
//!
//! Its turn is the banker's (`games/ice-and-fire/ladder`), copied and
//! changed: see [`plan`]. The input never names the league, so the bot fixes
//! it (Bronze, all the rules). A turn whose text does not read as a state is
//! answered with `WAIT`; a bad count line or input that ends in the middle of
//! a turn ends the bot with exit code 1. Debug output goes to stderr only.

use std::io::{self, BufRead, Write};
use std::time::Instant;

use ice_and_fire_engine::{answer_line, League, State};

mod plan;

/// Lines of a turn before the building count: four numbers, then the map.
const HEADER_LINES: usize = 4 + 12;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    if let Err(err) = play(stdin.lock(), stdout.lock()) {
        eprintln!("sparring: {err}");
        std::process::exit(1);
    }
}

/// The answer line to one turn's input: the initial input, the turn's input
/// and the game turn from 1. Input that does not read as a state gives
/// `WAIT`, with the reason on stderr.
fn answer(initial: &str, turn: &str, game_turn: u32) -> String {
    match State::read(initial, turn, League::Bronze, game_turn) {
        Ok(state) => {
            let mut commands = Vec::new();
            plan::plan(&state, &mut commands);
            answer_line(&commands)
        }
        Err(err) => {
            eprintln!("sparring: cannot read the input: {err}");
            "WAIT".to_string()
        }
    }
}

/// Plays turns until the input ends, flushing each answer.
fn play(input: impl BufRead, mut output: impl Write) -> Result<(), String> {
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

        let start = Instant::now();
        let answer = answer(&initial, &text, turn);
        eprintln!("turn {turn} (sparring): {:?}", start.elapsed());
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

fn count(line: &str, what: &str) -> Result<usize, String> {
    line.trim()
        .parse()
        .map_err(|_| format!("bad {what} {line:?}"))
}

#[cfg(test)]
mod tests;
