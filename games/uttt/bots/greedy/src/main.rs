//! Greedy Ultimate Tic-Tac-Toe bot: looks one move ahead with simple rules
//! (win the game, win small boards, do not hand the opponent a board or a
//! free choice) and breaks ties at random.
//!
//! It is a fixed, fast baseline: clearly stronger than random, far weaker
//! than search. The arena uses it to check that its statistics tell bots
//! apart, and the league uses it as a reference point.
//!
//! It tracks the position with `uttt-engine` but only plays actions from
//! the list it receives, so it also follows openings imposed by the arena.

mod policy;

use std::io::{self, BufRead, Write};

use cg_core::input::{Input, InputError};
use cg_core::rng::{seed_from_env_or_clock, Rng};
use uttt_engine::{Board, Move};

fn main() {
    let seed = seed_from_env_or_clock();
    eprintln!("greedy: seed {seed}");
    let mut rng = Rng::new(seed);
    let stdin = io::stdin();
    let result = play(
        &mut rng,
        Input::new(stdin.lock()),
        io::stdout().lock(),
        &mut io::stderr(),
    );
    match result {
        Ok(()) | Err(InputError::Eof) => {}
        Err(err) => {
            eprintln!("greedy: {err}");
            std::process::exit(1);
        }
    }
}

/// Plays turns until the input ends.
fn play(
    rng: &mut Rng,
    mut input: Input<impl BufRead>,
    mut out: impl Write,
    log: &mut impl Write,
) -> Result<(), InputError> {
    let io_error = |err: io::Error| InputError::Io(err.to_string());
    // `None` once the position is lost track of; then the bot plays at random.
    let mut board = Some(Board::new());
    let mut actions = Vec::new();
    loop {
        let opponent: (i32, i32) = input.two()?;
        let count: usize = input.one()?;
        actions.clear();
        for _ in 0..count {
            actions.push(input.two::<i32, i32>()?);
        }
        if actions.is_empty() {
            return Err(InputError::Parse {
                line: count.to_string(),
                expected: "at least one valid action".to_string(),
            });
        }

        let tracked = board.is_some();
        if opponent != (-1, -1) {
            board = board.and_then(|position| apply(position, opponent));
        }
        let moves: Option<Vec<Move>> = actions.iter().map(|&action| to_move(action)).collect();
        let position = match (board, moves) {
            (Some(position), Some(moves)) if moves.iter().all(|&mv| position.is_legal(mv)) => {
                Some((position, moves))
            }
            _ => None,
        };
        let choice = match position {
            Some((position, moves)) => {
                let index = best(&position, &moves, rng);
                board = Some(apply_move(position, moves[index]));
                actions[index]
            }
            None => {
                if tracked {
                    writeln!(log, "greedy: lost track of the position; playing at random")
                        .map_err(io_error)?;
                }
                board = None;
                *rng.pick(&actions).expect("the list is not empty")
            }
        };
        writeln!(out, "{} {}", choice.0, choice.1)
            .and_then(|()| out.flush())
            .map_err(io_error)?;
    }
}

/// The index of the best-scoring move, ties broken at random.
fn best(board: &Board, moves: &[Move], rng: &mut Rng) -> usize {
    let mut best = Vec::new();
    let mut best_score = i32::MIN;
    for (index, &mv) in moves.iter().enumerate() {
        let score = policy::score(board, mv);
        if score > best_score {
            best_score = score;
            best.clear();
        }
        if score == best_score {
            best.push(index);
        }
    }
    *rng.pick(&best).expect("there is at least one move")
}

fn to_move((row, col): (i32, i32)) -> Option<Move> {
    Move::from_row_col(usize::try_from(row).ok()?, usize::try_from(col).ok()?)
}

/// The position after `action`, or `None` if it is not legal there.
fn apply(board: Board, action: (i32, i32)) -> Option<Board> {
    let mv = to_move(action)?;
    board.is_legal(mv).then(|| apply_move(board, mv))
}

fn apply_move(mut board: Board, mv: Move) -> Board {
    board.play(mv);
    board
}

#[cfg(test)]
mod tests;
