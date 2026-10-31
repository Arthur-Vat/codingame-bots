//! Timing probe for A Code of Ice and Fire on CodinGame: a bot to paste in
//! the IDE and play against itself, never to submit, that measures what our
//! arena cannot.
//!
//! - **CodinGame's time limit, as the bot's clock sees it.** A bot times its
//!   turn from the moment the turn's first line arrives; CodinGame times it
//!   from when it sends the input, and how far apart the two are is unknown.
//!   From turn [`RAMP_FROM`] on, the bot holds each answer back until
//!   [`target`]: [`RAMP_START_MS`] ms on that turn, one more each turn after.
//!   The turn on which CodinGame reports a timeout bounds the limit: the last
//!   answer accepted was held to one millisecond less.
//! - **CodinGame's overhead between turns.** Each turn records the gap from
//!   its previous answer to the next turn's first line. Against itself, the
//!   opponent's part of that gap is its own hold, known from its line; the
//!   rest is CodinGame's.
//! - **CodinGame's speed against ours, and how steady it is.** On the first
//!   turn (1,000 ms) it runs a fixed workload ([`unit`], on a position
//!   written here) for [`WORKLOAD_MS`] ms; on later turns it runs the same
//!   units while it holds its answer. It records the units done a
//!   millisecond; the same file run locally gives our own figure.
//!
//! Each turn it plays the greedy policy's turn (`ice_and_fire_policy`, the
//! `bronze` bot's) less any command that would take the enemy HQ
//! ([`peaceful`]), so that a game against itself lasts until the hold
//! reaches CodinGame's limit. Each turn, before holding, it prints one line
//! to stderr with all it has measured in the game ([`Game::line`]), so that
//! the last line of each side is enough to read afterwards. Debug output
//! goes to stderr only. A bad count line or input that ends in the middle of
//! a turn ends the bot with exit code 1.
//!
//! Off CodinGame, the same workload measures a machine: with
//! [`WORKLOAD_ENV`] set, the bot plays no game and prints its speed
//! ([`calibrate`]). `scripts/calibrate.sh` runs it to set the evaluations'
//! time scale to CodinGame's speed (ADR 0048).

use std::fmt::Write as _;
use std::hint::{black_box, spin_loop};
use std::io;
use std::time::{Duration, Instant};

use ice_and_fire_engine::{answer_line, Command, League, Outcome, State, HQ, SIZE};
use ice_and_fire_policy::greedy_turn;
use ice_and_fire_policy::io::turns;
use ice_and_fire_tactics::chain::chain_to;
use ice_and_fire_tactics::cut::cuts;

/// The version of the line's format, first in it.
pub const FORMAT: u32 = 1;
/// The first turn whose answer is held back.
pub const RAMP_FROM: u32 = 2;
/// How long the answer of turn [`RAMP_FROM`] is held, in ms from the turn's
/// first line; each later turn adds one.
pub const RAMP_START_MS: u64 = 42;
/// The longest an answer is held, in ms.
pub const RAMP_MAX_MS: u64 = 120;
/// How long the first turn's workload runs, in ms.
pub const WORKLOAD_MS: u64 = 500;

/// The workload's position: the initial input and one turn's input of a
/// middle-game position of a v011 self-play game (the plan bot's speed
/// bank, `ice-and-fire-9100-p0-0`, ply 14).
const WORK_INITIAL: &str = "\
8\n\
1 0\n\
2 2\n\
3 4\n\
5 5\n\
6 6\n\
8 7\n\
9 9\n\
10 11\n\
";
const WORK_TURN: &str = "\
25\n\
20\n\
8\n\
19\n\
OOO##....###\n\
.OO#.....###\n\
.OO#.....###\n\
.OOOO....###\n\
.OOO.....###\n\
#OOOXXXXX.##\n\
##OOO.XXXX.#\n\
###...XXXXX.\n\
###.....XXX.\n\
###.....#XX.\n\
###.....#XXX\n\
###....##XXX\n\
3\n\
0 0 0 0\n\
0 1 1 0\n\
1 0 11 11\n\
10\n\
0 1 1 4 3\n\
0 2 1 2 6\n\
0 5 1 3 6\n\
0 8 1 4 6\n\
1 3 1 8 8\n\
1 4 1 6 7\n\
1 6 1 6 6\n\
1 7 1 6 5\n\
1 9 1 5 5\n\
1 10 1 4 5\n\
";

/// How long the answer of `turn` (1 first) is held, from the turn's first
/// line; `None` before [`RAMP_FROM`].
pub fn target(turn: u32) -> Option<Duration> {
    let steps = u64::from(turn.checked_sub(RAMP_FROM)?);
    Some(Duration::from_millis(
        (RAMP_START_MS + steps).min(RAMP_MAX_MS),
    ))
}

/// One unit of the workload on `state`: the greedy turn, the cheapest chain
/// to the enemy HQ and the 4 best cuts of up to 5 trainings, all with the
/// player's gold. Returns a number that depends on all of them, so that
/// none is optimized away.
pub fn unit(state: &State) -> usize {
    let me = state.to_act();
    let gold = state.gold(me);
    let mut commands = Vec::new();
    greedy_turn(state, &mut commands);
    let chain = chain_to(state, HQ[1 - me], gold).map_or(0, |c| c.commands.len());
    let cut = cuts(state, 5, gold, 4).len();
    commands.len() + chain + cut
}

/// The workload's state.
pub fn work_state() -> Result<State, String> {
    State::read(WORK_INITIAL, WORK_TURN, League::Bronze, 8)
}

/// Units of the workload done on `state` until `until`, as far as whole
/// units fit, then a busy wait to `until`. Returns the units done and the
/// time they took.
pub fn work_until(state: &State, until: Instant) -> (u64, Duration) {
    let start = Instant::now();
    let mut done = 0u64;
    let mut slowest = Duration::ZERO;
    loop {
        let before = Instant::now();
        // Stop while one more unit, at twice the slowest so far, still fits.
        if before + slowest * 2 >= until {
            break;
        }
        black_box(unit(black_box(state)));
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
/// ms and prints each stretch's units a millisecond on its own line of
/// stdout, then exits.
pub const WORKLOAD_ENV: &str = "CG_WORKLOAD_MS";
/// How many stretches [`WORKLOAD_ENV`] runs.
pub const CALIBRATION_STRETCHES: usize = 10;

/// The units a millisecond of the workload in each of `stretches` stretches
/// of `ms` ms, as the turns of a game measure them.
pub fn calibrate(ms: u64, stretches: usize) -> Result<Vec<f64>, String> {
    let work = work_state()?;
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
            eprintln!("timing: cannot read the workload's position: {err}");
            1
        }
    }
}

/// Units a millisecond, if any time passed.
fn per_ms(done: u64, took: Duration) -> Option<f64> {
    let ms = took.as_secs_f64() * 1e3;
    (ms > 0.0).then(|| done as f64 / ms)
}

/// The commands of `commands` that the rules accept in order on `state`,
/// less any that would take the enemy HQ (and those that then fail).
pub fn peaceful(state: &State, commands: &[Command]) -> Vec<Command> {
    let me = state.to_act();
    let mut scratch = state.clone();
    let mut kept = Vec::new();
    for &command in commands {
        let mut next = scratch.clone();
        if next.apply(command) && next.outcome() != Some(Outcome::Win(me)) {
            scratch = next;
            kept.push(command);
        }
    }
    kept
}

/// The map as `<144 characters, row by row> <mine spots as x,y separated by
/// spaces>`, `#` a void cell and `.` any other (the `rules-check` bot's
/// MAP line without its seat).
pub fn map_text(state: &State) -> String {
    let map = state.map();
    let mut text: String = (0..SIZE * SIZE)
        .map(|cell| if map.valid.test(cell) { '.' } else { '#' })
        .collect();
    for cell in map.spots.iter() {
        let _ = write!(text, " {},{}", cell % SIZE, cell / SIZE);
    }
    text
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
    /// The policy's time.
    pub policy: f64,
    /// When the answer left; none until it has.
    pub answer: Option<f64>,
    /// Units of the workload a millisecond during the hold (the first turn:
    /// the workload's own run).
    pub speed: Option<f64>,
}

/// Everything the bot has measured in the game.
#[derive(Default)]
pub struct Game {
    /// `red` or `blue`.
    pub seat: &'static str,
    /// The map ([`map_text`]).
    pub map: String,
    pub turns: Vec<Turn>,
}

fn ms(value: Option<f64>, digits: usize) -> String {
    value.map_or("-".to_string(), |v| format!("{v:.digits$}"))
}

impl Game {
    /// The line printed before a turn's hold: the format, the seat, the map,
    /// then every turn as `turn:target/read/gap/policy/answer/speed` (ms, ms,
    /// ms, ms, ms, units a ms; `-` when none), the last one, the turn in
    /// progress, without its answer and speed.
    pub fn line(&self) -> String {
        let mut line = format!(
            "timing v{FORMAT} {} | map {} | turn:target/read/gap/policy/answer/speed |",
            self.seat, self.map
        );
        for t in &self.turns {
            let target = t.target.map_or("-".to_string(), |v| v.to_string());
            let _ = write!(
                line,
                " {}:{target}/{}/{}/{}/{}/{}",
                t.turn,
                ms(Some(t.read), 2),
                ms(t.gap, 1),
                ms(Some(t.policy), 1),
                ms(t.answer, 2),
                ms(t.speed, 1),
            );
        }
        line
    }
}

/// The answer to one turn's input, `state` being read from it: the greedy
/// turn without a win ([`peaceful`]).
pub fn answer(state: &State) -> String {
    let mut commands = Vec::new();
    greedy_turn(state, &mut commands);
    answer_line(&peaceful(state, &commands))
}

fn main() {
    if let Ok(text) = std::env::var(WORKLOAD_ENV) {
        std::process::exit(measure(&text));
    }
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut game = Game::default();
    let mut sent: Option<Instant> = None;
    let work = work_state();
    let played = turns(stdin.lock(), stdout.lock(), |initial, text, turn, start| {
        let read = start.elapsed();
        let gap = sent.map(|at| start.saturating_duration_since(at));
        let state = match State::read(initial, text, League::Bronze, turn) {
            Ok(state) => state,
            Err(err) => {
                eprintln!("timing: cannot read the input: {err}");
                sent = Some(Instant::now());
                return "WAIT".to_string();
            }
        };
        if game.map.is_empty() {
            game.seat = if state.to_act() == 0 { "red" } else { "blue" };
            game.map = map_text(&state);
        }
        let mut speed = None;
        if turn == 1 {
            if let Ok(work) = &work {
                let (done, took) =
                    work_until(work, Instant::now() + Duration::from_millis(WORKLOAD_MS));
                speed = per_ms(done, took);
            }
        }
        let thinking = Instant::now();
        let answer = answer(&state);
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
        // Printed before the hold, so that the line of a turn CodinGame cuts
        // off can still show.
        eprintln!("{}", game.line());
        if let (Some(wait), Ok(work)) = (held, &work) {
            let (done, took) = work_until(work, start + wait);
            if let Some(last) = game.turns.last_mut() {
                last.speed = per_ms(done, took);
            }
        }
        if let Some(last) = game.turns.last_mut() {
            last.answer = Some(start.elapsed().as_secs_f64() * 1e3);
        }
        sent = Some(Instant::now());
        answer
    });
    if let Err(err) = played {
        eprintln!("timing: {err}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests;
