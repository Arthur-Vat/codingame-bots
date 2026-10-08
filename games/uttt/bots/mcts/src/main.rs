//! Monte Carlo tree search Ultimate Tic-Tac-Toe bot.
//!
//! Every turn it searches the current position with UCT and playouts that
//! take a game-winning move whenever there is one and otherwise draw their
//! first moves from a playout policy learned from self-play, then random
//! moves (`cg-search`, `uttt-engine`), for most of the turn's time limit,
//! then plays the move it tried most often. The same policy orders each
//! node's children, so that the most promising are tried first. The search keeps the
//! part of its tree under the moves played since its previous search, and
//! proves wins and losses near the end of the game: it plays a proven win,
//! avoids proven losses, and stops searching once the position is proven.
//! A move that wins the game at once is played without searching, and so is
//! the only action offered.
//!
//! It tracks the position from the opponent's moves but only plays actions
//! from the list it receives. Each turn it prints one line to stderr:
//! iterations, visits kept from earlier searches, the expected score of its
//! move (or the proven result), and the time it took.
//!
//! The seed comes from `CG_SEED` when the arena sets it, otherwise from the
//! clock. The arena may scale time limits with `CG_TIME_SCALE`, and
//! `CG_FIXED_ITERS` replaces the time budget by a number of iterations.

use std::io::{self, BufRead, Write};
use std::time::{Duration, Instant};

use cg_core::input::{Input, InputError};
use cg_core::rng::{seed_from_env_or_clock, Rng};
use cg_search::{Budget, Mcts};
use uttt_engine::search::PolicyBoard;
use uttt_engine::{Board, Move, PlayoutPolicy};

/// CodinGame's time limit for the first answer.
const FIRST_LIMIT: Duration = Duration::from_millis(1000);
/// CodinGame's time limit for every later answer.
const LIMIT: Duration = Duration::from_millis(100);
/// Share of the limit the search may use.
const SHARE: f64 = 0.9;
/// Time kept for reading, writing and timing noise beyond the share, at
/// CodinGame's limits; scaled with the arena's time factor, like the limit.
/// None since E007: the search gets 90 ms of 100. Up to E006 it kept 8 ms
/// (82 ms of search) so that no game was lost on time; ADR 0015 tolerates
/// rare timeouts, which cost far less than the search time they buy. In
/// 100 local games at full time, 99.9% of answers took at most 94.1 ms.
const RESERVE: Duration = Duration::ZERO;
/// The exploration constant of UCB1, from matches on 2026-10-07 (time
/// scale 0.2, 200 pairs each): 0.5 beat 1.0 by 164 Elo, 0.7 beat 1.0 by
/// 116, 1.4 lost to 1.0 by 108, and 0.4 and 0.3 lost to 0.5 by 28 and 116.
/// At full time, 0.5 beat 1.0 by 162.
const EXPLORATION: f64 = 0.5;

/// The playout policy: weights by move class, learned from self-play
/// (ADR 0016), for the first 16 moves of each playout; decisive random
/// moves after that, which are cheaper. Screened on 2026-10-08 against
/// v007 at 20 ms (200 pairs each): the whole playout +1 Elo, 16 moves +18,
/// 8 moves +30, 4 moves +22; sharper weights (temperature 0.5) +53 for 16
/// moves and +48 for 8, flatter ones (2) -22 for 8.
static POLICY: PlayoutPolicy = PlayoutPolicy::new(weights::PLAYOUT_WEIGHTS).for_plies(16);

/// Whether the search also uses the playout policy's weights as priors in
/// its tree: a node's children are tried in order of their weights, most
/// promising first (E014).
const PRIORS: bool = true;
/// The weight of a bias toward the heavier children in selection, which
/// fades as they are visited. Screened on 2026-10-08 against v008 at 20 ms
/// (100 pairs each): no bias +50.7 and +50.7 (two seeds), 0.3 +29.6, 1.0
/// +34.9; so none.
const PRIOR_WEIGHT: f64 = 0.0;

mod weights;

fn main() {
    let seed = seed_from_env_or_clock();
    eprintln!("mcts: seed {seed}");
    let mut rng = Rng::new(seed);
    let mut mcts = Mcts::new(EXPLORATION, rng.next_u64());
    mcts.priors = PRIORS;
    mcts.prior_weight = PRIOR_WEIGHT;
    let stdin = io::stdin();
    let result = play(
        &mut mcts,
        &mut rng,
        Input::new(stdin.lock()),
        io::stdout().lock(),
        &mut io::stderr(),
        |start, limit| Budget::for_turn(start, limit, SHARE, RESERVE),
    );
    match result {
        Ok(()) | Err(InputError::Eof) => {}
        Err(err) => {
            eprintln!("mcts: {err}");
            std::process::exit(1);
        }
    }
}

/// Plays turns until the input ends. `budget` gives the search budget of a
/// turn from its start and CodinGame's limit for it.
fn play(
    mcts: &mut Mcts<PolicyBoard>,
    rng: &mut Rng,
    mut input: Input<impl BufRead>,
    mut out: impl Write,
    log: &mut impl Write,
    budget: impl Fn(Instant, Duration) -> Budget,
) -> Result<(), InputError> {
    let io_error = |err: io::Error| InputError::Io(err.to_string());
    // `None` once the position is lost track of; then the bot plays at random.
    let mut board = Some(Board::new());
    let mut actions = Vec::new();
    let mut limit = FIRST_LIMIT;
    loop {
        let opponent: (i32, i32) = input.two()?;
        let start = Instant::now();
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
                let mv = if moves.len() == 1 {
                    moves[0]
                } else {
                    let root = PolicyBoard {
                        board: position,
                        policy: &POLICY,
                    };
                    let result = mcts.search(&root, &moves, budget(start, limit));
                    let outlook = match result.proven {
                        Some(true) => "proven win".to_string(),
                        Some(false) => "proven loss".to_string(),
                        None => format!("expected score {:.3}", result.expected_score),
                    };
                    writeln!(
                        log,
                        "mcts: {} iterations, {} kept, {outlook}, {} ms",
                        result.iterations,
                        result.reused,
                        start.elapsed().as_millis()
                    )
                    .map_err(io_error)?;
                    result.best
                };
                board = Some(apply_move(position, mv));
                actions[moves.iter().position(|&m| m == mv).expect("a candidate")]
            }
            None => {
                if tracked {
                    writeln!(log, "mcts: lost track of the position; playing at random")
                        .map_err(io_error)?;
                }
                board = None;
                *rng.pick(&actions).expect("the list is not empty")
            }
        };
        writeln!(out, "{} {}", choice.0, choice.1)
            .and_then(|()| out.flush())
            .map_err(io_error)?;
        limit = LIMIT;
    }
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
