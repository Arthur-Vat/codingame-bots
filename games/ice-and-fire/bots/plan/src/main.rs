//! Plan bot for A Code of Ice and Fire (experiment E002): a search after the
//! contest's winner (`games/ice-and-fire/research/README.md`, "What strong
//! bots do").
//!
//! A turn is a list of commands, the plan. The bot first looks for a lethal
//! move or chain of trainings and plays it. Else it climbs: from the start
//! of the turn it keeps adding the single action (a move, a training, a
//! tower, a mine, one of the best cuts, a lethal chain) that most raises the
//! score of the plan, until none does or the time is up. The score of a plan
//! is the worst evaluation of the state after it, our turn ended, over the
//! replies of the opponent in a memory (its lethal chain, its best cuts and
//! bronze's greedy turn, each answered by the plan). After a climb the
//! opponent's replies to the plan are added to the memory and the plan is
//! searched again against them; the plan that scores best against the whole
//! memory is played.
//!
//! The input never names the league, so the bot plays with all the rules
//! (Bronze). Each command sent was accepted by the engine, in order, on a
//! scratch copy of the state. A turn whose text does not read as a state is
//! answered with `WAIT`; a bad count line or input that ends in the middle of
//! a turn ends the bot with exit code 1. The time budget of a turn is 40 ms
//! times `CG_TIME_SCALE` on every turn, the first included. Debug output goes
//! to stderr only.

mod actions;
mod eval;
mod search;
mod session;

use std::io;
use std::time::{Duration, Instant};

use crate::search::Limit;

/// The time of a turn at CodinGame's limits, milliseconds: 50 ms, less a
/// margin for the input, the output and a busy machine. The first turn's
/// 1,000 ms is not used.
const BUDGET_MS: f64 = 40.0;

/// The share of the budget after which no new climb or reply search starts;
/// the rest is for choosing the plan to play.
const SOFT_SHARE: f64 = 0.6;

/// The limits of a turn that started at `start`.
fn limit(start: Instant) -> Limit {
    // A scale beyond 100 is no use and could overflow the clock.
    let scale = cg_core::time::time_scale().min(100.0);
    let budget = Duration::from_secs_f64(BUDGET_MS / 1000.0 * scale);
    Limit {
        soft: Some(start + budget.mul_f64(SOFT_SHARE)),
        hard: Some(start + budget),
        max_evals: u64::MAX,
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    if let Err(err) = session::play(&limit, stdin.lock(), stdout.lock()) {
        eprintln!("plan: {err}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests;
