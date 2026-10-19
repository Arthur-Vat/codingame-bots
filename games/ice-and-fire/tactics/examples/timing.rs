//! Times the shared tactics on positions from random games, game turns 10
//! to 40: the mean and the worst time per call, in microseconds.
//!
//! ```sh
//! cargo +1.90.0 run --release -p ice-and-fire-tactics --example timing
//! ```
//!
//! A CodinGame turn has 50 ms. The positions come from `State::random_turn`
//! games on the referee's Bronze maps, so they hold fewer cells and buildings
//! than games between strong bots.

use std::hint::black_box;
use std::time::Instant;

use cg_core::rng::Rng;
use ice_and_fire_engine::{League, State, HQ};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;
use ice_and_fire_tactics::chain::{chain_to, Reach};
use ice_and_fire_tactics::cut::cuts;
use ice_and_fire_tactics::eval::evaluate;

/// Games played.
const GAMES: u64 = 60;

/// The game turns whose positions are timed.
const FIRST_TURN: u32 = 10;
const LAST_TURN: u32 = 40;

/// Each call is timed alone; the quick ones are repeated to be measurable.
struct Timing {
    name: &'static str,
    calls: u64,
    total: f64,
    worst: f64,
}

impl Timing {
    fn new(name: &'static str) -> Timing {
        Timing {
            name,
            calls: 0,
            total: 0.0,
            worst: 0.0,
        }
    }

    /// Times `call`, run `repeat` times, and records the time of one run.
    fn time<T>(&mut self, repeat: u32, mut call: impl FnMut() -> T) {
        let start = Instant::now();
        for _ in 0..repeat {
            black_box(call());
        }
        let micros = start.elapsed().as_secs_f64() * 1e6 / f64::from(repeat);
        self.calls += 1;
        self.total += micros;
        self.worst = self.worst.max(micros);
    }
}

fn main() {
    let mut reach = Timing::new("Reach::new");
    let mut reach_moves = Timing::new("Reach::new (moves)");
    let mut chain = Timing::new("chain_to(HQ, gold)");
    let mut cut = Timing::new("cuts(4, gold, 8)");
    let mut eval = Timing::new("evaluate");
    let mut cut_rich = Timing::new("cuts(4, 1000 gold, 8)");
    let mut chain_rich = Timing::new("chain_to(HQ, 1000 gold)");
    let mut positions = 0;
    for seed in 0..GAMES {
        let map = generate(seed, RefereeLeague::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let mut rng = Rng::new(seed ^ 0x71_3171);
        let mut commands = Vec::new();
        while state.outcome().is_none() && state.game_turn() <= LAST_TURN {
            if state.game_turn() >= FIRST_TURN {
                positions += 1;
                let me = state.to_act();
                let gold = state.gold(me);
                let target = HQ[1 - me];
                reach.time(200, || Reach::new(&state, me, false));
                reach_moves.time(200, || Reach::new(&state, me, true));
                chain.time(20, || chain_to(&state, target, gold));
                cut.time(1, || cuts(&state, 4, gold, 8));
                eval.time(100, || evaluate(&state, me));
                // The same position with 1000 gold, so that the gold does not
                // end the search early.
                let text = state.turn_input(me);
                let rich = format!("1000{}", &text[text.find('\n').unwrap_or(0)..]);
                if let Ok(rich) = State::read(&state.initial_input(me), &rich, League::Bronze, 1) {
                    cut_rich.time(1, || cuts(&rich, 4, 1000, 8));
                    chain_rich.time(5, || chain_to(&rich, target, 1000));
                }
            }
            state.random_turn(&mut rng, &mut commands);
            state.play_turn(&commands);
        }
    }
    // A heavy position: red holds rows 0 to 5 and blue rows 6 to 11, all
    // active, and red has 100,000 gold. The search ends at its node limit.
    let heavy = {
        let mut turn = String::from("100000\n72\n20\n72\n");
        for y in 0..12 {
            turn.push_str(if y < 6 {
                "OOOOOOOOOOOO\n"
            } else {
                "XXXXXXXXXXXX\n"
            });
        }
        turn.push_str("2\n0 0 0 0\n1 0 11 11\n0\n");
        State::read("0\n", &turn, League::Bronze, 10).expect("the heavy position")
    };
    let mut heavy_cuts = [
        (2, Timing::new("cuts(2, 100000 gold, 8), heavy")),
        (4, Timing::new("cuts(4, 100000 gold, 8), heavy")),
        (6, Timing::new("cuts(6, 100000 gold, 8), heavy")),
        (10, Timing::new("cuts(10, 100000 gold, 8), heavy")),
    ];
    for (len, timing) in &mut heavy_cuts {
        for _ in 0..20 {
            timing.time(1, || cuts(&heavy, *len, 100_000, 8));
        }
    }
    println!("{positions} positions, game turns {FIRST_TURN} to {LAST_TURN}");
    println!();
    println!("| Call | Calls | Mean (us) | Max (us) |");
    println!("| --- | ---: | ---: | ---: |");
    let mut timings = vec![
        &reach,
        &reach_moves,
        &chain,
        &cut,
        &eval,
        &chain_rich,
        &cut_rich,
    ];
    timings.extend(heavy_cuts.iter().map(|(_, timing)| timing));
    for timing in timings {
        println!(
            "| {} | {} | {:.1} | {:.1} |",
            timing.name,
            timing.calls,
            timing.total / timing.calls as f64,
            timing.worst
        );
    }
}
