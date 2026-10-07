//! Wood league bot. The lowest league of Ultimate Tic-Tac-Toe is plain 3×3
//! tic-tac-toe against a boss that plays randomly (according to the game's
//! source). This bot never loses at 3×3 and, among the moves that keep that
//! guarantee, picks the one that wins most often against random play.
//!
//! Its only purpose is promotion to Bronze, where the real game starts. If
//! the input shows the 9×9 board, it falls back to random valid actions.

mod solver;

use std::io::{self, BufRead, Write};

use cg_core::input::{Input, InputError};
use cg_core::rng::{seed_from_env_or_clock, Rng};
use uttt_engine::grid::FULL;

use crate::solver::Solver;

fn main() {
    let seed = seed_from_env_or_clock();
    eprintln!("wood: seed {seed}");
    let mut rng = Rng::new(seed);
    let stdin = io::stdin();
    match play(&mut rng, Input::new(stdin.lock()), io::stdout().lock()) {
        Ok(()) | Err(InputError::Eof) => {}
        Err(err) => {
            eprintln!("wood: {err}");
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
    let mut solver = Solver::new();
    // Wood league until a coordinate above 2 shows the 9×9 board.
    let mut wood = true;
    // Cells this bot has marked, as a 9-bit mask (Wood league only).
    let mut mine: u16 = 0;
    loop {
        let (opponent_row, opponent_col): (i32, i32) = input.two()?;
        let count: usize = input.one()?;
        let mut actions = Vec::with_capacity(count);
        for _ in 0..count {
            actions.push(input.two::<i32, i32>()?);
        }
        if opponent_row > 2
            || opponent_col > 2
            || actions.iter().any(|&(row, col)| row > 2 || col > 2)
        {
            wood = false;
        }

        let choice = if wood {
            wood_move(&mut solver, mine, &actions)
        } else {
            None
        };
        let (row, col) = match choice {
            Some(action) => action,
            None => *rng.pick(&actions).ok_or(InputError::Parse {
                line: count.to_string(),
                expected: "at least one valid action".to_string(),
            })?,
        };
        if wood {
            mine |= 1 << (3 * row + col);
        }
        writeln!(out, "{row} {col}")
            .and_then(|()| out.flush())
            .map_err(|err| InputError::Io(err.to_string()))?;
    }
}

/// The solver's move in Wood league. There, the valid actions are exactly
/// the empty cells, so every marked cell that is not ours is the opponent's.
fn wood_move(solver: &mut Solver, mine: u16, actions: &[(i32, i32)]) -> Option<(i32, i32)> {
    let empty = actions
        .iter()
        .fold(0u16, |mask, &(row, col)| mask | 1 << (3 * row + col));
    let theirs = !(empty | mine) & FULL;
    let cell = solver.best_move(mine, theirs)? as i32;
    Some((cell / 3, cell % 3))
}

#[cfg(test)]
mod tests;
