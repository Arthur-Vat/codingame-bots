//! Minimal A Code of Ice and Fire bot for CodinGame: answers `WAIT` every turn.
//!
//! Once, CodinGame sends `numberMineSpots`, then that many lines `x y`.
//! Every turn it sends:
//!
//! 1. four lines: own gold, own income, opponent's gold, opponent's income;
//! 2. 12 lines of 12 characters: the map;
//! 3. `buildingCount`, then that many lines `owner buildingType x y`;
//! 4. `unitCount`, then that many lines `owner unitId level x y`.
//!
//! The bot reads all of it, to keep the protocol in step, and answers `WAIT`.
//! It exists to give the arena and the CI a bot that is always valid.
//!
//! The file uses the standard library only, so it can be pasted into the
//! CodinGame editor as is.

use std::io::{self, BufRead, Write};

/// Lines of a turn before the building count: four numbers, then the map.
const HEADER_LINES: usize = 4 + 12;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    if let Err(err) = play(stdin.lock(), stdout.lock()) {
        eprintln!("wait: {err}");
        std::process::exit(1);
    }
}

/// Plays turns until the referee closes the input.
fn play(input: impl BufRead, mut output: impl Write) -> Result<(), String> {
    let mut lines = input.lines();

    // The initial input. End of input before it means there is no game.
    let Some(spots) = next_line(&mut lines)? else {
        return Ok(());
    };
    for _ in 0..parse_count(&spots, "mine spot count", 0)? {
        required_line(&mut lines, 0)?;
    }

    let mut turn: u32 = 0;
    loop {
        // End of input before a new turn starts means the game is over.
        if next_line(&mut lines)?.is_none() {
            return Ok(());
        }
        turn += 1;
        for _ in 1..HEADER_LINES {
            required_line(&mut lines, turn)?;
        }
        // Buildings, then units: each a count and that many lines.
        for what in ["building count", "unit count"] {
            let count_line = required_line(&mut lines, turn)?;
            for _ in 0..parse_count(&count_line, what, turn)? {
                required_line(&mut lines, turn)?;
            }
        }

        writeln!(output, "WAIT")
            .and_then(|()| output.flush())
            .map_err(|err| format!("turn {turn}: cannot write the answer: {err}"))?;
    }
}

fn next_line(
    lines: &mut impl Iterator<Item = io::Result<String>>,
) -> Result<Option<String>, String> {
    lines
        .next()
        .transpose()
        .map_err(|err| format!("cannot read input: {err}"))
}

fn required_line(
    lines: &mut impl Iterator<Item = io::Result<String>>,
    turn: u32,
) -> Result<String, String> {
    next_line(lines)?.ok_or_else(|| format!("turn {turn}: input ended in the middle of a turn"))
}

fn parse_count(line: &str, what: &str, turn: u32) -> Result<usize, String> {
    line.trim()
        .parse()
        .map_err(|_| format!("turn {turn}: bad {what} {line:?}"))
}

#[cfg(test)]
mod tests;
