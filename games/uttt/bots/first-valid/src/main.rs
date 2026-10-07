//! Hello-world Ultimate Tic-Tac-Toe bot for CodinGame.
//!
//! Every turn, CodinGame sends:
//!
//! 1. `opponentRow opponentCol`: the opponent's last move, or `-1 -1` when
//!    this bot moves first;
//! 2. `validActionCount`: how many moves are legal;
//! 3. that many lines of `row col`, rows and columns numbered 0 to 8.
//!
//! The bot answers `row col` with the first valid action. It exists to prove
//! the repository-to-CodinGame path end to end (Phase 0), not to win.
//!
//! The file uses the standard library only, so it can be pasted into the
//! CodinGame editor as is.

use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    if let Err(err) = play(stdin.lock(), stdout.lock()) {
        eprintln!("first-valid: {err}");
        std::process::exit(1);
    }
}

/// A board cell in CodinGame coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Action {
    row: i32,
    col: i32,
}

/// Plays turns until the referee closes the input.
fn play(input: impl BufRead, mut output: impl Write) -> Result<(), String> {
    let mut lines = input.lines();
    let mut turn: u32 = 0;
    loop {
        // End of input before a new turn starts means the game is over.
        let Some(opponent) = next_line(&mut lines)? else {
            return Ok(());
        };
        turn += 1;
        parse_action(&opponent).map_err(|err| format!("turn {turn}: {err}"))?;

        let count_line = required_line(&mut lines, turn)?;
        let count: usize = count_line
            .trim()
            .parse()
            .map_err(|_| format!("turn {turn}: bad action count {count_line:?}"))?;

        let mut actions = Vec::with_capacity(count);
        for _ in 0..count {
            let line = required_line(&mut lines, turn)?;
            actions.push(parse_action(&line).map_err(|err| format!("turn {turn}: {err}"))?);
        }

        let action = choose(&actions).ok_or_else(|| format!("turn {turn}: no valid action"))?;
        writeln!(output, "{} {}", action.row, action.col)
            .and_then(|()| output.flush())
            .map_err(|err| format!("turn {turn}: cannot write move: {err}"))?;
    }
}

/// The whole strategy of this bot.
fn choose(actions: &[Action]) -> Option<Action> {
    actions.first().copied()
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

/// Parses `row col`.
fn parse_action(line: &str) -> Result<Action, String> {
    let mut parts = line.split_whitespace().map(str::parse::<i32>);
    match (parts.next(), parts.next(), parts.next()) {
        (Some(Ok(row)), Some(Ok(col)), None) => Ok(Action { row, col }),
        _ => Err(format!("expected `row col`, got {line:?}")),
    }
}

#[cfg(test)]
mod tests;
