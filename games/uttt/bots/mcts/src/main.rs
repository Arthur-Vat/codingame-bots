//! Monte Carlo tree search Ultimate Tic-Tac-Toe bot.
//!
//! Every turn it searches the current position with UCT for most of the
//! turn's time limit (`cg-search`, `uttt-engine`), then plays the move it
//! tried most often. It runs no playouts: the first time the search reaches
//! a node, it creates all its children and gives each the estimate of an
//! NNUE (`uttt_engine::nnue`), from the node's accumulators updated for
//! each child, except that a side to move with a move that wins the game is
//! scored a win ("jacekmax", E018). The accumulators are built once per turn
//! at the root and updated with each move as the search walks down the tree
//! (E022). Each node keeps the minimax
//! value of its children, which selection uses. The search
//! keeps the part of its tree under the moves played since its previous
//! search, and proves wins and losses near the end of the game: it plays a
//! proven win, avoids proven losses, and stops searching once the position
//! is proven.
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
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use cg_core::input::{Input, InputError};
use cg_core::rng::{seed_from_env_or_clock, Rng};
use cg_search::{Budget, Mcts};
use uttt_engine::nnue::Network;
use uttt_engine::search::NnueBoard;
use uttt_engine::{Board, Move};

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
/// The exploration constant of UCB1. With playouts (up to `uttt-v010`) it
/// was 0.5, from matches on 2026-10-07 (time scale 0.2, 200 pairs each):
/// 0.5 beat 1.0 by 164 Elo, 0.7 beat 1.0 by 116, 1.4 lost to 1.0 by 108,
/// and 0.4 and 0.3 lost to 0.5 by 28 and 116. With the NNUE at the leaves
/// (E017), screened on 2026-10-14 against `uttt-v010` at 20 ms (time scale
/// 0.2, 5 ms of tolerance, 4-ply openings, 100 pairs each): 0.5 +12 Elo
/// [-33, +58], 0.3 +78 [+31, +128], 0.2 +56 [+18, +96]. At full time, 50
/// pairs, 0.3 +70 [+19, +126]. With E020's search (E021), screened on
/// 2026-10-15 against `uttt-v013` (4-ply openings): at 20 ms, 15 ms of
/// tolerance, 100 pairs each, 0.15 +56.1, 0.2 +31.4, 0.25 +57.9, 0.4 -70.4;
/// at full time, 50 pairs each, 0.15 +3.5, 0.25 -17.4, 0.2 +24.4 and +45.4
/// (two seeds).
const EXPLORATION: f64 = 0.2;

/// The network that judges the leaves (E017): the NNUE of Train run
/// 38799715918 (E024), (189 → 256) × 2 → 10, quantized and decoded once
/// from its packed characters (see `cg-pack`).
fn network() -> &'static Network<256> {
    static NETWORK: OnceLock<Network<256>> = OnceLock::new();
    NETWORK.get_or_init(|| {
        let bytes = cg_pack::unpack(nnue_weights::NNUE_WEIGHTS).expect("the NNUE weights unpack");
        Network::from_bytes(&bytes).expect("the NNUE weights decode")
    })
}

// Packed characters include ones that clippy takes for invisible.
#[allow(clippy::invisible_characters)]
mod nnue_weights;

/// Whether the search expands all the children of a node at once and
/// estimates them with the network, backing up their minimax value (E018).
const EXPAND_ALL: bool = true;
/// The weight of a child's minimax value against its average score in
/// selection. The minimax value only in E018; half and half since E020,
/// screened on 2026-10-14 against `uttt-v012` (4-ply openings): at 20 ms
/// (100 pairs each) 0 -77.7 Elo, 0.25 +5.2, 0.5 +43.7, 0.75 +59.6, 0.9
/// +34.9; at full time (two seeds, 50 pairs each) 0.5 +74.1 and +77.7, 0.75
/// +10.4 and +38.4. The search uses no priors:
/// from E014 to E017 the pattern policy ordered each node's children, but
/// since E018 every child gets the network's estimate as soon as it is
/// created, so the order no longer matters, and its weights left the bot.
const MINIMAX_WEIGHT: f64 = 0.5;

/// PUCT (E030, tuned by E032): the temperature of the priors that a node's
/// children get from their own estimates, `softmax(estimate / T)`, and the
/// weight of the bias they give, `PRIOR_WEIGHT · prior · sqrt(parent visits)
/// / (visits + 1)`. A temperature of 0 selects by UCB1 alone.
const VALUE_TEMPERATURE: f64 = 0.05;
const PRIOR_WEIGHT: f64 = 0.5;

fn main() {
    let seed = seed_from_env_or_clock();
    eprintln!("mcts: seed {seed}");
    let mut rng = Rng::new(seed);
    // Decoded before the first turn's clock starts, if the input waits.
    network();
    let mut mcts = Mcts::new(EXPLORATION, rng.next_u64());
    mcts.expand_all = EXPAND_ALL;
    mcts.minimax_weight = MINIMAX_WEIGHT;
    mcts.value_temperature = VALUE_TEMPERATURE;
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
    mcts: &mut Mcts<NnueBoard>,
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
                    let root = NnueBoard::new(position, network());
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
