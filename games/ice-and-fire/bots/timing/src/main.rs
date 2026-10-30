//! Timing probe for A Code of Ice and Fire on CodinGame: a bot to paste in
//! the IDE, never to submit, that measures two things our arena cannot.
//!
//! - **CodinGame's time limit, as the bot's clock sees it.** A bot times its
//!   turn from the moment the turn's first line arrives; CodinGame times it
//!   from when it sends the input, and how far apart the two are is unknown.
//!   From turn [`RAMP_FROM`] on, the bot holds each answer back until
//!   [`target`]: [`RAMP_START_MS`] ms on that turn, one more each turn after.
//!   The turn on which CodinGame reports a timeout bounds the limit: the last
//!   answer accepted was held to one millisecond less.
//! - **CodinGame's speed against ours.** On the first turn (1,000 ms) it runs
//!   a fixed workload, on a position written here, for [`WORKLOAD_MS`] ms and
//!   counts the units done. The same count printed by the same file run
//!   locally gives the ratio of the two machines' speeds.
//!
//! Each turn it plays the greedy policy's answer (`ice_and_fire_policy`,
//! the `bronze` bot's) and prints one line to stderr, which the IDE shows:
//! the turn, the time the policy took, when the answer left by the bot's
//! clock and the target. Debug output goes to stderr only. A bad count line
//! or input that ends in the middle of a turn ends the bot with exit code 1.

use std::hint::{black_box, spin_loop};
use std::io;
use std::time::{Duration, Instant};

use ice_and_fire_engine::{League, State, HQ};
use ice_and_fire_policy::greedy_turn;
use ice_and_fire_policy::io::turns;
use ice_and_fire_tactics::chain::chain_to;
use ice_and_fire_tactics::cut::cuts;

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

/// Units of the workload done in `budget`, and the time they took.
pub fn workload(budget: Duration) -> Result<(u64, Duration), String> {
    let state = work_state()?;
    let start = Instant::now();
    let mut done = 0u64;
    while start.elapsed() < budget {
        black_box(unit(black_box(&state)));
        done += 1;
    }
    Ok((done, start.elapsed()))
}

/// Waits, busy, until `until`.
fn hold(until: Instant) {
    while Instant::now() < until {
        spin_loop();
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let played = turns(stdin.lock(), stdout.lock(), |initial, text, turn, start| {
        if turn == 1 {
            match workload(Duration::from_millis(WORKLOAD_MS)) {
                Ok((done, took)) => eprintln!(
                    "timing: workload {done} units in {:.1} ms, {:.2} units a ms",
                    took.as_secs_f64() * 1e3,
                    done as f64 / (took.as_secs_f64() * 1e3)
                ),
                Err(err) => eprintln!("timing: no workload: {err}"),
            }
        }
        let thinking = Instant::now();
        let answer = ice_and_fire_policy::answer(initial, text, League::Bronze, turn);
        let thought = thinking.elapsed();
        let held = target(turn);
        if let Some(wait) = held {
            hold(start + wait);
        }
        eprintln!(
            "turn {turn}: policy {:.1} ms, answer at {:.2} ms, target {}",
            thought.as_secs_f64() * 1e3,
            start.elapsed().as_secs_f64() * 1e3,
            held.map_or("none".to_string(), |d| format!("{} ms", d.as_millis()))
        );
        answer
    });
    if let Err(err) = played {
        eprintln!("timing: {err}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests;
