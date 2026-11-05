//! Timing probe for Ultimate Tic-Tac-Toe on CodinGame: a bot to play
//! against itself in test games, never to submit, that measures what our
//! arena cannot.
//!
//! - **CodinGame's time limit, as the bot's clock sees it.** A bot times its
//!   turn from the moment the turn's first line arrives; CodinGame times it
//!   from when it sends the input, and how far apart the two are is unknown.
//!   From turn [`RAMP_FROM`] on, the bot holds each answer back until
//!   [`target`]: [`RAMP_START_MS`] ms on that turn, one more each turn after.
//!   The turn on which CodinGame reports a timeout bounds the limit: the last
//!   answer accepted was held to one millisecond less. [`RAMP_START_MS`] can
//!   be raised for a later game when the first ones end before a cut.
//! - **CodinGame's overhead between turns.** Each turn records the gap from
//!   its previous answer to the next turn's first line. Against itself, the
//!   opponent's part of that gap is its own hold, known from its line; the
//!   rest is CodinGame's.
//! - **CodinGame's speed against ours, and how steady it is.** On the first
//!   turn (1,000 ms) it runs a fixed workload ([`unit`]: `uttt-v018`'s search,
//!   its network and settings, on a middle-game position written here) for
//!   [`WORKLOAD_MS`] ms; on later turns it runs the same units while it holds
//!   its answer. It records the units done a millisecond; the same file run
//!   locally gives our own figure. Speeds are in the workload's search
//!   iterations a millisecond, [`UNIT_ITERATIONS`] to a unit.
//!
//! Each turn it plays a random valid action that wins no small board
//! ([`peaceful`]), or any valid action when all of them do, so that a game
//! against itself lasts as long as the board allows: up to 41 answers a
//! side. Each turn, before holding, it prints one line to stderr with all it
//! has measured in the game ([`Game::line`]), so that the last line of each
//! side is enough to read afterwards. Debug output goes to stderr only. Input
//! that does not read ends the bot with exit code 1.
//!
//! Off CodinGame, the same workload measures a machine: with
//! [`WORKLOAD_ENV`] set, the bot plays no game and prints its speed
//! ([`calibrate`]). `scripts/calibrate.sh` runs it to set the evaluations'
//! time scale to CodinGame's speed (ADR 0048, ADR 0049 decision 8).
//!
//! The workload must not change once CodinGame's speed has been measured
//! with it: its network is a copy of `uttt-v018`'s, kept here when the bot's
//! own changes, and a change to the workload, the engine or the search that
//! changes its speed needs a new probe on CodinGame.

use std::fmt::Write as _;
use std::hint::{black_box, spin_loop};
use std::io::{self, BufRead, Write};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use cg_core::input::{Input, InputError};
use cg_core::rng::{seed_from_env_or_clock, Rng};
use cg_search::{Budget, Mcts};
use uttt_engine::nnue::Network;
use uttt_engine::search::NnueBoard;
use uttt_engine::{Board, Move, MoveList};

/// The version of the line's format, first in it.
pub const FORMAT: u32 = 1;
/// The first turn whose answer is held back.
pub const RAMP_FROM: u32 = 2;
/// How long the answer of turn [`RAMP_FROM`] is held, in ms from the turn's
/// first line; each later turn adds one. CodinGame gives 100 ms a turn.
pub const RAMP_START_MS: u64 = 95;
/// The longest an answer is held, in ms.
pub const RAMP_MAX_MS: u64 = 250;
/// How long the first turn's workload runs, in ms.
pub const WORKLOAD_MS: u64 = 500;
/// The iterations of one unit of the workload: about 6 ms on a session's
/// machine (about 330 iterations a millisecond).
pub const UNIT_ITERATIONS: u64 = 2_000;
/// The seed of each unit's search.
const UNIT_SEED: u64 = 18;

/// `uttt-v018`'s search settings (`bots/mcts`): exploration, minimax weight,
/// PUCT's temperature and weight; every child expanded at once.
const EXPLORATION: f64 = 0.2;
const MINIMAX_WEIGHT: f64 = 0.5;
const VALUE_TEMPERATURE: f64 = 0.05;
const PRIOR_WEIGHT: f64 = 0.5;

/// The workload's position: the 24 plies of a game of [`peaceful`] actions
/// (seed 18 of `the_workload_position_is_a_peaceful_game`), as (row, col).
const WORK_PLIES: [(usize, usize); 24] = [
    (4, 5),
    (4, 6),
    (3, 1),
    (1, 4),
    (3, 3),
    (0, 2),
    (0, 8),
    (0, 6),
    (0, 1),
    (0, 4),
    (1, 5),
    (5, 6),
    (7, 2),
    (3, 7),
    (0, 5),
    (1, 8),
    (4, 8),
    (3, 8),
    (1, 6),
    (3, 2),
    (0, 7),
    (0, 3),
    (2, 2),
    (8, 8),
];

/// `uttt-v018`'s network (E024), decoded once from its packed characters.
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

/// How long the answer of `turn` (1 first) is held, from the turn's first
/// line; `None` before [`RAMP_FROM`].
pub fn target(turn: u32) -> Option<Duration> {
    let steps = u64::from(turn.checked_sub(RAMP_FROM)?);
    Some(Duration::from_millis(
        (RAMP_START_MS + steps).min(RAMP_MAX_MS),
    ))
}

/// The workload: its position and that position's legal moves.
pub struct Work {
    root: NnueBoard,
    moves: Vec<Move>,
}

/// The workload's position, `None` if [`WORK_PLIES`] do not play.
pub fn work() -> Option<Work> {
    let mut board = Board::new();
    for &(row, col) in &WORK_PLIES {
        let mv = Move::from_row_col(row, col)?;
        if !board.is_legal(mv) {
            return None;
        }
        board.play(mv);
    }
    let mut list = MoveList::new();
    board.legal_moves(&mut list);
    Some(Work {
        root: NnueBoard::new(board, network()),
        moves: list.to_vec(),
    })
}

/// One unit of the workload: a new search of `uttt-v018`'s with
/// [`UNIT_ITERATIONS`] iterations from the workload's position. Returns a
/// number that depends on the search, so that it is not optimized away.
pub fn unit(work: &Work) -> u64 {
    let mut mcts = Mcts::new(EXPLORATION, UNIT_SEED);
    mcts.expand_all = true;
    mcts.minimax_weight = MINIMAX_WEIGHT;
    mcts.value_temperature = VALUE_TEMPERATURE;
    mcts.prior_weight = PRIOR_WEIGHT;
    let result = mcts.search(&work.root, &work.moves, Budget::Iterations(UNIT_ITERATIONS));
    result.iterations + result.nodes as u64
}

/// Units of the workload done until `until`, as far as whole units fit,
/// then a busy wait to `until`. Returns the units done and the time they
/// took.
pub fn work_until(work: &Work, until: Instant) -> (u64, Duration) {
    let start = Instant::now();
    let mut done = 0u64;
    let mut slowest = Duration::ZERO;
    loop {
        let before = Instant::now();
        // Stop while one more unit, at twice the slowest so far, still fits.
        if before + slowest * 2 >= until {
            break;
        }
        black_box(unit(black_box(work)));
        slowest = slowest.max(before.elapsed());
        done += 1;
    }
    let took = start.elapsed();
    while Instant::now() < until {
        spin_loop();
    }
    (done, took)
}

/// When this variable holds a number of milliseconds `N`, the bot plays no
/// game: it runs the workload in [`CALIBRATION_STRETCHES`] stretches of `N`
/// ms and prints each stretch's speed (iterations a millisecond) on its own
/// line of stdout, then exits.
pub const WORKLOAD_ENV: &str = "CG_WORKLOAD_MS";
/// How many stretches [`WORKLOAD_ENV`] runs.
pub const CALIBRATION_STRETCHES: usize = 10;

/// The speed of the workload (iterations a millisecond) in each of
/// `stretches` stretches of `ms` ms, as the turns of a game measure them.
pub fn calibrate(ms: u64, stretches: usize) -> Result<Vec<f64>, String> {
    let work = work().ok_or("the workload's position does not play")?;
    Ok((0..stretches)
        .filter_map(|_| {
            let (done, took) = work_until(&work, Instant::now() + Duration::from_millis(ms));
            per_ms(done, took)
        })
        .collect())
}

/// Runs [`WORKLOAD_ENV`]'s measure for `text` ms and returns the exit code.
fn measure(text: &str) -> i32 {
    let Some(ms) = text.trim().parse::<u64>().ok().filter(|&ms| ms > 0) else {
        eprintln!("timing: {WORKLOAD_ENV} must be a positive number of milliseconds, not {text:?}");
        return 2;
    };
    match calibrate(ms, CALIBRATION_STRETCHES) {
        Ok(speeds) => {
            for speed in speeds {
                println!("{speed:.2}");
            }
            0
        }
        Err(err) => {
            eprintln!("timing: {err}");
            1
        }
    }
}

/// The workload's iterations a millisecond, when `done` units took `took`,
/// if any time passed. Speeds count iterations, not units, so that two
/// decimals keep them precise.
fn per_ms(done: u64, took: Duration) -> Option<f64> {
    let ms = took.as_secs_f64() * 1e3;
    (ms > 0.0).then(|| (done * UNIT_ITERATIONS) as f64 / ms)
}

/// The actions of `actions` that win no small board on `board`; all of them
/// when each one does, or when one is not legal there.
pub fn peaceful(board: &Board, actions: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let me = board.to_move();
    let won = board.won_boards(me);
    let kept: Option<Vec<(i32, i32)>> = actions
        .iter()
        .filter_map(|&action| match to_move(action) {
            Some(mv) if board.is_legal(mv) => {
                let mut next = *board;
                next.play(mv);
                (next.won_boards(me) == won).then_some(Some(action))
            }
            _ => Some(None),
        })
        .collect();
    match kept {
        Some(kept) if !kept.is_empty() => kept,
        _ => actions.to_vec(),
    }
}

fn to_move((row, col): (i32, i32)) -> Option<Move> {
    Move::from_row_col(usize::try_from(row).ok()?, usize::try_from(col).ok()?)
}

/// What one turn measured, in ms by the bot's clock from the turn's first
/// line.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Turn {
    pub turn: u32,
    /// How long the answer was held, if it was.
    pub target: Option<u64>,
    /// When the rest of the turn's input had been read.
    pub read: f64,
    /// From the bot's previous answer to this turn's first line; none on
    /// the first turn.
    pub gap: Option<f64>,
    /// The time to choose the action.
    pub policy: f64,
    /// When the answer left; none until it has.
    pub answer: Option<f64>,
    /// The workload's iterations a millisecond during the hold (the first
    /// turn: the workload's own run).
    pub speed: Option<f64>,
}

/// Everything the bot has measured in the game.
#[derive(Default)]
pub struct Game {
    /// `first` or `second`: whether the bot played the game's first move.
    pub seat: &'static str,
    pub turns: Vec<Turn>,
}

fn ms(value: Option<f64>, digits: usize) -> String {
    value.map_or("-".to_string(), |v| format!("{v:.digits$}"))
}

impl Game {
    /// The line printed before a turn's hold: the format, the seat, then
    /// every turn as `turn:target/read/gap/policy/answer/speed` (ms, ms, ms,
    /// ms, ms, iterations a ms; `-` when none), the last one, the turn in
    /// progress, without its answer and speed.
    pub fn line(&self) -> String {
        let mut line = format!(
            "timing v{FORMAT} {} | turn:target/read/gap/policy/answer/speed |",
            self.seat
        );
        for t in &self.turns {
            let target = t.target.map_or("-".to_string(), |v| v.to_string());
            let _ = write!(
                line,
                " {}:{target}/{}/{}/{}/{}/{}",
                t.turn,
                ms(Some(t.read), 2),
                ms(t.gap, 1),
                ms(Some(t.policy), 2),
                ms(t.answer, 2),
                ms(t.speed, 1),
            );
        }
        line
    }
}

fn main() {
    if let Ok(text) = std::env::var(WORKLOAD_ENV) {
        std::process::exit(measure(&text));
    }
    let mut rng = Rng::new(seed_from_env_or_clock());
    // Decoded before the first turn's clock starts, if the input waits.
    let work = work();
    let stdin = io::stdin();
    let result = play(
        &mut rng,
        work.as_ref(),
        Input::new(stdin.lock()),
        io::stdout().lock(),
        &mut io::stderr(),
    );
    match result {
        Ok(()) | Err(InputError::Eof) => {}
        Err(err) => {
            eprintln!("timing: {err}");
            std::process::exit(1);
        }
    }
}

/// Plays turns until the input ends, measuring each and holding its answer
/// as [`target`] says (only with a workload to run meanwhile).
fn play(
    rng: &mut Rng,
    work: Option<&Work>,
    mut input: Input<impl BufRead>,
    mut out: impl Write,
    log: &mut impl Write,
) -> Result<(), InputError> {
    let io_error = |err: io::Error| InputError::Io(err.to_string());
    let mut game = Game::default();
    let mut board = Board::new();
    let mut sent: Option<Instant> = None;
    let mut actions = Vec::new();
    for turn in 1u32.. {
        let opponent: (i32, i32) = input.two()?;
        let start = Instant::now();
        let gap = sent.map(|at| start.saturating_duration_since(at));
        let count: usize = input.one()?;
        actions.clear();
        for _ in 0..count {
            actions.push(input.two::<i32, i32>()?);
        }
        let read = start.elapsed();
        if turn == 1 {
            game.seat = if opponent == (-1, -1) {
                "first"
            } else {
                "second"
            };
        }
        if let Some(mv) = to_move(opponent).filter(|&mv| board.is_legal(mv)) {
            board.play(mv);
        }

        let mut speed = None;
        if let (1, Some(work)) = (turn, work) {
            let (done, took) =
                work_until(work, Instant::now() + Duration::from_millis(WORKLOAD_MS));
            speed = per_ms(done, took);
        }
        let thinking = Instant::now();
        let action = *rng
            .pick(&peaceful(&board, &actions))
            .ok_or(InputError::Parse {
                line: count.to_string(),
                expected: "at least one valid action".to_string(),
            })?;
        if let Some(mv) = to_move(action).filter(|&mv| board.is_legal(mv)) {
            board.play(mv);
        }
        let policy = thinking.elapsed();
        let held = target(turn);
        game.turns.push(Turn {
            turn,
            target: held.map(|d| d.as_millis() as u64),
            read: read.as_secs_f64() * 1e3,
            gap: gap.map(|d| d.as_secs_f64() * 1e3),
            policy: policy.as_secs_f64() * 1e3,
            answer: None,
            speed,
        });
        // Written before the hold, so that the line of a turn CodinGame cuts
        // off can still show.
        writeln!(log, "{}", game.line()).map_err(io_error)?;
        if let (Some(wait), Some(work)) = (held, work) {
            let (done, took) = work_until(work, start + wait);
            if let Some(last) = game.turns.last_mut() {
                last.speed = per_ms(done, took);
            }
        }
        if let Some(last) = game.turns.last_mut() {
            last.answer = Some(start.elapsed().as_secs_f64() * 1e3);
        }
        writeln!(out, "{} {}", action.0, action.1)
            .and_then(|()| out.flush())
            .map_err(io_error)?;
        sent = Some(Instant::now());
    }
    Ok(())
}

#[cfg(test)]
mod tests;
