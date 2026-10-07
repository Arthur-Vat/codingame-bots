//! Rules check on CodinGame. Plays random valid actions and, every turn,
//! compares the valid actions CodinGame sends with the ones our engine
//! computes from the moves played so far. It also says when the engine
//! thinks its own move ends the game, and how.
//!
//! Use it in the IDE from Bronze league upward, where the 9×9 game is
//! played. Every turn it prints one line to stderr ending with running
//! totals, so the last line of a game sums it up. Differences are printed
//! on lines starting with `rules-check: DIFFERENCE`.
//!
//! When `CG_RULES_CHECK_STRICT` is set (CI does this), the first difference
//! makes it exit with an error instead, which the arena reports as a crash.

mod checker;

use std::io::{self, BufRead, Write};

use cg_core::input::{Input, InputError};
use cg_core::rng::{seed_from_env_or_clock, Rng};

use crate::checker::Checker;

/// Environment variable that turns differences into errors.
const STRICT_ENV: &str = "CG_RULES_CHECK_STRICT";

fn main() {
    let seed = seed_from_env_or_clock();
    eprintln!("rules-check: seed {seed}");
    let strict = std::env::var_os(STRICT_ENV).is_some();
    let mut rng = Rng::new(seed);
    let stdin = io::stdin();
    let result = play(
        &mut rng,
        strict,
        Input::new(stdin.lock()),
        io::stdout().lock(),
        &mut io::stderr(),
    );
    match result {
        Ok(()) | Err(InputError::Eof) => {}
        Err(err) => {
            eprintln!("rules-check: {err}");
            std::process::exit(1);
        }
    }
}

/// Plays and checks turns until the input ends.
fn play(
    rng: &mut Rng,
    strict: bool,
    mut input: Input<impl BufRead>,
    mut out: impl Write,
    log: &mut impl Write,
) -> Result<(), InputError> {
    let mut checker = Checker::new();
    let io_error = |err: io::Error| InputError::Io(err.to_string());
    loop {
        let opponent: (i32, i32) = input.two()?;
        let count: usize = input.one()?;
        let mut actions = Vec::with_capacity(count);
        for _ in 0..count {
            actions.push(input.two::<i32, i32>()?);
        }

        for line in checker.check_turn(opponent, &actions) {
            writeln!(log, "{line}").map_err(io_error)?;
        }
        if strict && checker.differences() > 0 {
            return Err(InputError::Parse {
                line: format!("turn {}", checker.turns()),
                expected: "the same valid actions as the engine (strict mode)".to_string(),
            });
        }

        let action = *rng.pick(&actions).ok_or(InputError::Parse {
            line: count.to_string(),
            expected: "at least one valid action".to_string(),
        })?;
        for line in checker.record_own_move(action) {
            writeln!(log, "{line}").map_err(io_error)?;
        }
        writeln!(out, "{} {}", action.0, action.1)
            .and_then(|()| out.flush())
            .map_err(io_error)?;
    }
}

#[cfg(test)]
mod tests;
