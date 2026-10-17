//! Measures the engine's speed on Bronze games from the referee's maps:
//! random turns generated and played, whole random games, and the text
//! protocol (`turn_input`, `read`, and `State::new` with a map built).
//!
//! ```sh
//! cargo +1.90.0 run --release -p ice-and-fire-engine --example speed -- [SECONDS]
//! ```
//!
//! Prints a Markdown table, so CI can add it to the job summary.

use std::hint::black_box;
use std::time::{Duration, Instant};

use cg_core::rng::Rng;
use ice_and_fire_engine::{League, State};
use ice_and_fire_referee::map::{generate, Map};
use ice_and_fire_referee::League as RefereeLeague;

/// Maps the measures cycle through.
const MAPS: u64 = 64;

fn main() {
    let seconds: f64 = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(2.0);
    let budget = Duration::from_secs_f64(seconds);
    let maps: Vec<Map> = (0..MAPS)
        .map(|seed| generate(seed, RefereeLeague::Bronze))
        .collect();
    let new_state = |map: &Map| State::new(&map.void, &map.mine_spots, League::Bronze);
    let mut rng = Rng::new(1);

    // Warm up caches and CPU frequency before measuring.
    measure_games(&maps, &mut rng, Duration::from_millis(300));
    let games = measure_games(&maps, &mut rng, budget);
    let turns_per_second = games.turns as f64 / games.seconds;

    // Turns generated alone, on positions from the middle of games. They have
    // their own generator: the timed loops above draw a number of random
    // turns that depends on the machine's speed. A game that ended before
    // turn 40 is left out, since no player receives an input after the end
    // and `read` rightly refuses a captured HQ.
    let mut position_rng = Rng::new(2);
    let positions: Vec<State> = maps
        .iter()
        .filter_map(|map| {
            let mut state = new_state(map);
            let mut turn = Vec::new();
            for _ in 0..40 {
                state.random_turn(&mut position_rng, &mut turn);
                state.play_turn(&turn);
                if state.outcome().is_some() {
                    return None;
                }
            }
            Some(state)
        })
        .collect();
    assert!(!positions.is_empty(), "every game ended before turn 40");
    let generated = count(budget, |round| {
        let mut turn = Vec::new();
        positions[round % positions.len()].random_turn(&mut rng, &mut turn);
        black_box(turn.len());
    });
    let inputs = count(budget, |round| {
        black_box(positions[round % positions.len()].turn_input(round % 2));
    });
    let texts: Vec<(String, String)> = positions
        .iter()
        .map(|state| (state.initial_input(0), state.turn_input(state.to_act())))
        .collect();
    let reads = count(budget, |round| {
        let (initial, turn) = &texts[round % texts.len()];
        black_box(State::read(initial, turn, League::Bronze, 40).expect("valid text"));
    });
    // The same map every time: read hands out the map it remembers.
    let (initial, turn) = &texts[0];
    let reads_same_map = count(budget, |_| {
        black_box(State::read(initial, turn, League::Bronze, 40).expect("valid text"));
    });
    let starts = count(budget, |round| {
        black_box(new_state(&maps[round % maps.len()]));
    });

    println!("| Engine benchmark | Result |");
    println!("| --- | --- |");
    println!(
        "| Random turns generated and played | {} per second |",
        thousands(turns_per_second)
    );
    println!(
        "| Random turns generated alone (from move 40 of a game) | {} per second |",
        thousands(generated)
    );
    println!(
        "| Whole random games (Bronze) | {} per second |",
        thousands(games.games as f64 / games.seconds)
    );
    println!(
        "| Turns per random game | {:.1} |",
        games.turns as f64 / games.games as f64
    );
    println!("| `turn_input` | {} per second |", thousands(inputs));
    println!(
        "| `read` (a new map every time) | {} per second |",
        thousands(reads)
    );
    println!(
        "| `read` (the map of the last read) | {} per second |",
        thousands(reads_same_map)
    );
    println!(
        "| `State::new` (builds the distance table) | {} per second |",
        thousands(starts)
    );
}

struct Games {
    games: u64,
    turns: u64,
    seconds: f64,
}

/// Plays random games, one per map in turn, for `budget`.
fn measure_games(maps: &[Map], rng: &mut Rng, budget: Duration) -> Games {
    let mut games = Games {
        games: 0,
        turns: 0,
        seconds: 0.0,
    };
    let mut turn = Vec::new();
    let start = Instant::now();
    while start.elapsed() < budget {
        let map = &maps[games.games as usize % maps.len()];
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        while state.outcome().is_none() {
            state.random_turn(rng, &mut turn);
            state.play_turn(&turn);
            games.turns += 1;
        }
        black_box(state.outcome());
        games.games += 1;
    }
    games.seconds = start.elapsed().as_secs_f64();
    games
}

/// Runs `work` (given the round number) for `budget`; returns rounds per second.
fn count(budget: Duration, mut work: impl FnMut(usize)) -> f64 {
    let start = Instant::now();
    let mut rounds = 0usize;
    while start.elapsed() < budget {
        // Check the clock every 64 rounds: it costs about as much as a round.
        for _ in 0..64 {
            work(rounds);
            rounds += 1;
        }
    }
    rounds as f64 / start.elapsed().as_secs_f64()
}

/// A number with a space every three digits.
fn thousands(value: f64) -> String {
    let digits = format!("{:.0}", value);
    let mut text = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            text.push(',');
        }
        text.push(digit);
    }
    text
}
