//! Plan bot for A Code of Ice and Fire (experiments E002 to E004, E010): a search
//! after the contest's winner (`games/ice-and-fire/research/README.md`,
//! "What strong bots do"), with a planned opening
//! (`games/ice-and-fire/research/early-game.md`).
//!
//! A turn is a list of commands, the plan. The bot first looks for a lethal
//! move or chain of trainings and plays it. Until the middle game (it can pay
//! a chain of trainings to an enemy cell) it plays the opening
//! (`ice_and_fire_opening::planned` with the solo planner, E010): v003's
//! opening by rules, with weights for a compact shape and a pull toward the
//! enemy HQ chosen turn by turn by a beam search over our next 7 turns as if
//! the opponent passed (`ice_and_fire_opening::solo`), which rates the
//! territory, the income and the gold, less the cells one capture would cut
//! off, the joints and the ends of lines, plus the land nearer us than the
//! enemy and the progress toward its HQ. The plan is made on the first turn,
//! kept while our side of the board is as predicted, and made again within
//! the turn's time otherwise. Else it climbs: from
//! the start of the turn it keeps adding the single action (a move, a
//! training, a tower, a mine, one of the best cuts, a lethal chain) that most
//! raises the score of the plan, until none does or the time is up. The score
//! of a plan is the worst evaluation of the state after it, our turn ended,
//! over the replies of the opponent in a memory (its lethal chain, its best
//! cuts, bronze's greedy turn and its own plan, the same climb from its seat
//! with up to 300 tries (E019), each answered by the plan). After a climb
//! the opponent's replies to the plan are added to the memory and the plan is
//! searched again against them; when that teaches nothing new, the next climb
//! may not start with the first action of a plan found so far (E014), until
//! the time is up. The plan that scores best against the whole memory and
//! its own replies (E027) is played; a plan whose replies the time cut is
//! checked against them first, before the hard limit. Cuts are chains of up to 5 trainings (E014). The evaluation
//! counts a side that cannot pay its upkeep as having lost its units, and a
//! level-2 unit as worth 5 rather than its training cost of 20 (E020).
//!
//! The input never names the league, so the bot plays with all the rules
//! (Bronze). Each command sent was accepted by the engine, in order, on a
//! scratch copy of the state. A turn whose text does not read as a state is
//! answered with `WAIT`; a bad count line or input that ends in the middle of
//! a turn ends the bot with exit code 1. The time budget of a turn is 40 ms
//! times `CG_TIME_SCALE`, 800 ms on the first turn (of CodinGame's 1,000
//! ms), of which the opening's plan takes about 610 ms. With `CG_FIXED_ITERS`
//! set to a positive number (tests only, never on CodinGame) a turn has no
//! deadline: the opening searches all its widths and the middle game stops
//! after that many tries, so that a game replays exactly (ADR 0025). Debug
//! output goes to stderr only.
//!
//! Built with `--features analysis`, the binary also measures a bank of
//! positions (`bank`) and times the search on one (`speed`), see the
//! `analysis` module; bots never carry that code.

mod actions;
#[cfg(feature = "analysis")]
mod analysis;
mod eval;
mod search;
mod session;

use std::io;
use std::time::{Duration, Instant};

use ice_and_fire_opening::econ::Params;
use ice_and_fire_opening::planned::{Planned, Planner};
use ice_and_fire_opening::solo::{self, Objective};

use crate::search::Limit;

/// The time of a turn at CodinGame's limits, milliseconds: the answer
/// leaves by it, a hard limit by the bot's clock, CodinGame's 50 ms less 2 ms
/// for the input and the output (E031; 40 ms before). No work starts that
/// could not end before it (`search::think`).
const BUDGET_MS: f64 = 48.0;
/// The first turn's time, of CodinGame's 1,000 ms (E010); the opening's
/// plan stops at 76% of it, about 610 ms.
const FIRST_BUDGET_MS: f64 = 800.0;

/// The share of the budget that the soft limit marks: the opening's choice
/// stops 40% of the way from it to the hard limit, and the middle game's
/// climbs 79% of the way, about 44 ms (`search::CLIMB_SHARE`, E027, E031),
/// the rest being for choosing the plan to play.
const SOFT_SHARE: f64 = 0.6;

/// The limits of the turn `turn` (1 first) that started at `start`.
fn limit(start: Instant, turn: u32) -> Limit {
    // With CG_FIXED_ITERS (set by tests, never by CodinGame), a number of
    // tries and no deadline, so that a game replays exactly (ADR 0025). It
    // is read as the arena and the other searches read it.
    if let Some(max_evals) = std::env::var("CG_FIXED_ITERS")
        .ok()
        .and_then(|n| n.trim().parse().ok())
        .filter(|&n| n > 0)
    {
        return Limit {
            soft: None,
            hard: None,
            max_evals,
        };
    }
    // A scale beyond 100 is no use and could overflow the clock.
    let scale = cg_core::time::time_scale().min(100.0);
    let ms = if turn == 1 {
        FIRST_BUDGET_MS
    } else {
        BUDGET_MS
    };
    let budget = Duration::from_secs_f64(ms / 1000.0 * scale);
    Limit {
        soft: Some(start + budget.mul_f64(SOFT_SHARE)),
        hard: Some(start + budget),
        max_evals: u64::MAX,
    }
}

/// The number of our turns an opening plan covers: the middle game starts
/// at our eighth turn in the median game (E010).
const OPENING_TURNS: usize = 7;

/// What the opening's plans maximize: the solo planner's weights, written
/// out (E010).
const OBJECTIVE: Objective = Objective {
    cells: 4,
    income: 3,
    gold: 1,
    cut: 2,
    joints: 2,
    tips: 2,
    reach: 6,
    voronoi: 1,
};

/// The opening: v003's opening with the solo planner's weight sets, planned
/// over [`OPENING_TURNS`] of our turns within the turn's time.
fn opening() -> Planned {
    opening_of(64)
}

/// The opening with beams up to `width` when a turn has no deadline.
fn opening_of(width: usize) -> Planned {
    let planner = Planner::Solo {
        turns: OPENING_TURNS,
        width,
        objective: OBJECTIVE,
    };
    Planned::new(planner, solo::SETS.to_vec(), Params::V003)
}

fn main() {
    #[cfg(feature = "analysis")]
    analysis::main_if_asked();
    let stdin = io::stdin();
    let stdout = io::stdout();
    if let Err(err) = session::play(&limit, opening(), stdin.lock(), stdout.lock()) {
        eprintln!("plan: {err}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests;
