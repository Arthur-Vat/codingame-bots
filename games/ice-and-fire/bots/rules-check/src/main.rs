//! Rules check on CodinGame. Plays random turns and, every turn, compares
//! what CodinGame sends with what our engine says: the received state alone
//! (active cells, incomes, units on inactive cells), and the state against
//! the one predicted after the bot's own commands. It also logs the game's
//! map once, so our map generator can be compared with CodinGame's, and says
//! when the engine thinks its own commands end the game.
//!
//! Use it in the IDE from Bronze league upward, where the full rules apply.
//! Every turn it prints one line to stderr ending with running totals
//! (`rules-check: turn N, X checks, M differences`), so the last line of a
//! game sums it up. Differences are printed on lines starting with
//! `rules-check: DIFFERENCE`. A turn's input that does not parse is a
//! difference too, and the answer is `WAIT`.
//!
//! When `CG_RULES_CHECK_STRICT` is set (CI does this), the first difference
//! makes it exit with an error instead.
//!
//! Once, CodinGame sends `numberMineSpots`, then that many lines `x y`.
//! Every turn it sends four lines (gold, income, the opponent's gold and
//! income), 12 map rows, `buildingCount` and that many building lines, then
//! `unitCount` and that many unit lines.

mod checker;
mod map_log;
mod session;

use std::io::{self, BufRead, Write};

use cg_core::rng::seed_from_env_or_clock;

use crate::session::Session;

/// Environment variable that turns differences into errors.
const STRICT_ENV: &str = "CG_RULES_CHECK_STRICT";

/// Lines of a turn before the building count: four numbers, then the map.
const HEADER_LINES: usize = 4 + 12;

/// The most building or unit lines read after a count: one per cell. A larger
/// count is a difference, but must not make the bot wait for lines that
/// CodinGame will never send.
const MAX_LINES: usize = 144;

fn main() {
    let seed = seed_from_env_or_clock();
    eprintln!("rules-check: seed {seed}");
    let strict = std::env::var_os(STRICT_ENV).is_some();
    let stdin = io::stdin();
    let result = play(
        seed,
        strict,
        stdin.lock(),
        io::stdout().lock(),
        &mut io::stderr(),
    );
    if let Err(err) = result {
        eprintln!("rules-check: {err}");
        std::process::exit(1);
    }
}

/// Plays and checks turns until the input ends.
fn play(
    seed: u64,
    strict: bool,
    input: impl BufRead,
    mut out: impl Write,
    log: &mut impl Write,
) -> Result<(), String> {
    let io_error = |err: io::Error| format!("cannot write: {err}");
    let mut lines = input.lines();

    // The initial input. End of input before it means there is no game.
    let Some(first) = next_line(&mut lines)? else {
        return Ok(());
    };
    let mut initial = format!("{first}\n");
    for _ in 0..count_of(&first) {
        match next_line(&mut lines)? {
            Some(line) => initial.push_str(&format!("{line}\n")),
            None => break,
        }
    }
    let mut session = Session::new(initial, seed);

    loop {
        let text = match read_turn(&mut lines) {
            Ok(Some(text)) => text,
            Ok(None) => return Ok(()),
            Err(err) => {
                let line = session.note_difference(&err);
                writeln!(log, "{line}").map_err(io_error)?;
                return if strict { Err(err) } else { Ok(()) };
            }
        };
        let started = std::time::Instant::now();
        let turn = session.play_turn(&text);
        for line in &turn.log {
            writeln!(log, "{line}").map_err(io_error)?;
        }
        if strict && session.differences() > 0 {
            return Err(format!(
                "turn {}: the first difference stops the bot (strict mode)",
                session.turn()
            ));
        }
        writeln!(out, "{}", turn.answer)
            .and_then(|()| out.flush())
            .map_err(io_error)?;
        let elapsed = started.elapsed();
        if elapsed.as_millis() >= 10 {
            writeln!(
                log,
                "rules-check: turn {} took {} ms",
                session.turn(),
                elapsed.as_millis()
            )
            .map_err(io_error)?;
        }
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

/// A count line as a number of lines to read; anything else reads none.
fn count_of(line: &str) -> usize {
    line.trim().parse::<usize>().unwrap_or(0).min(MAX_LINES)
}

/// One turn's input as text. `Ok(None)` when the game is over; an error when
/// the input stops in the middle of a turn.
fn read_turn(
    lines: &mut impl Iterator<Item = io::Result<String>>,
) -> Result<Option<String>, String> {
    let mut text = String::new();
    let Some(first) = next_line(lines)? else {
        return Ok(None);
    };
    text.push_str(&first);
    text.push('\n');
    let cut = || "the input ends in the middle of a turn".to_string();
    for _ in 1..HEADER_LINES {
        let line = next_line(lines)?.ok_or_else(cut)?;
        text.push_str(&line);
        text.push('\n');
    }
    // Buildings, then units: each a count and that many lines.
    for _ in 0..2 {
        let count_line = next_line(lines)?.ok_or_else(cut)?;
        text.push_str(&count_line);
        text.push('\n');
        for _ in 0..count_of(&count_line) {
            let line = next_line(lines)?.ok_or_else(cut)?;
            text.push_str(&line);
            text.push('\n');
        }
    }
    Ok(Some(text))
}

#[cfg(test)]
mod tests;
