//! Random Ultimate Tic-Tac-Toe bot: plays a uniformly random valid action.
//!
//! It is the baseline every rating is anchored to, and the first bot built
//! from shared crates, so it also tests the bundler.
//!
//! The seed comes from `CG_SEED` when the arena sets it, otherwise from the
//! clock. It is printed to stderr so a game can be replayed.

use std::io::{self, BufRead, Write};

use cg_core::input::{Input, InputError};
use cg_core::rng::{seed_from_env_or_clock, Rng};

fn main() {
    let seed = seed_from_env_or_clock();
    eprintln!("random: seed {seed}");
    let mut rng = Rng::new(seed);
    let stdin = io::stdin();
    match play(&mut rng, Input::new(stdin.lock()), io::stdout().lock()) {
        Ok(()) | Err(InputError::Eof) => {}
        Err(err) => {
            eprintln!("random: {err}");
            std::process::exit(1);
        }
    }
}

/// Plays turns until the input ends.
fn play(
    rng: &mut Rng,
    mut input: Input<impl BufRead>,
    mut out: impl Write,
) -> Result<(), InputError> {
    let mut actions = Vec::new();
    loop {
        let _opponent: (i32, i32) = input.two()?;
        let count: usize = input.one()?;
        actions.clear();
        for _ in 0..count {
            actions.push(input.two::<i32, i32>()?);
        }
        let (row, col) = *rng.pick(&actions).ok_or(InputError::Parse {
            line: count.to_string(),
            expected: "at least one valid action".to_string(),
        })?;
        writeln!(out, "{row} {col}")
            .and_then(|()| out.flush())
            .map_err(|err| InputError::Io(err.to_string()))?;
    }
}

#[cfg(test)]
mod tests;
