//! Measures the engine's speed with random playouts from the start position,
//! then with the decisive playouts that the search uses (a move that wins
//! the game when there is one), then the speed of that search (`cg-search`)
//! from the same position.
//!
//! ```sh
//! cargo run --release -p uttt-engine --example speed -- [SECONDS]
//! ```
//!
//! Prints a Markdown table, so CI can add it to the job summary.

use std::time::{Duration, Instant};

use cg_core::rng::Rng;
use cg_search::{Budget, Mcts};
use uttt_engine::{Board, Move, MoveList, Status};

/// The exploration constant of the MCTS bot.
const EXPLORATION: f64 = 0.5;
/// The length of each measured search: CodinGame's limit after the first turn.
const SEARCH: Duration = Duration::from_millis(100);

fn main() {
    let seconds: f64 = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(2.0);
    let budget = Duration::from_secs_f64(seconds);
    let mut rng = Rng::new(1);

    // Warm up caches and CPU frequency before measuring.
    measure(&mut rng, Duration::from_millis(200), Board::random_playout);
    let (playouts, elapsed) = measure(&mut rng, budget, Board::random_playout);
    let moves = moves_per_playout(&mut rng, Board::random_move);
    let (decisive, decisive_time) = measure(&mut rng, budget, Board::decisive_playout);
    let decisive_moves = moves_per_playout(&mut rng, Board::decisive_move);

    let per_second = playouts as f64 / elapsed.as_secs_f64();
    println!("| Engine benchmark | Result |");
    println!("| --- | --- |");
    println!(
        "| Random playouts from the start | {} per second |",
        thousands(per_second)
    );
    println!("| Moves per playout | {moves:.1} |");
    println!("| Moves per second | {} |", thousands(per_second * moves));
    println!(
        "| Decisive playouts from the start | {} per second, {decisive_moves:.1} moves each |",
        thousands(decisive as f64 / decisive_time.as_secs_f64()),
    );
    let (searches, iterations, search_time) = measure_search(budget);
    println!(
        "| MCTS iterations from the start, {} ms searches | {} per second |",
        SEARCH.as_millis(),
        thousands(iterations as f64 / search_time.as_secs_f64())
    );
    println!(
        "| Measured for | {:.1} s of playouts and {searches} searches, one thread |",
        (elapsed + decisive_time).as_secs_f64()
    );
}

/// Runs searches of `SEARCH` from the start for about `budget`; returns
/// searches, iterations and time taken.
fn measure_search(budget: Duration) -> (u64, u64, Duration) {
    let mut mcts = Mcts::new(EXPLORATION, 1);
    let board = Board::new();
    let mut moves = MoveList::new();
    board.legal_moves(&mut moves);
    let start = Instant::now();
    let (mut searches, mut iterations) = (0u64, 0u64);
    while searches == 0 || start.elapsed() < budget {
        let result = mcts.search(&board, &moves, Budget::Until(Instant::now() + SEARCH));
        searches += 1;
        iterations += result.iterations;
    }
    (searches, iterations, start.elapsed())
}

/// Runs `playout` from the start for `budget`, as the search does; returns
/// playouts and time taken.
fn measure(
    rng: &mut Rng,
    budget: Duration,
    playout: fn(&mut Board, &mut Rng) -> Status,
) -> (u64, Duration) {
    let start = Instant::now();
    let mut playouts = 0u64;
    while start.elapsed() < budget {
        for _ in 0..1000 {
            let mut board = Board::new();
            playout(&mut board, rng);
            playouts += 1;
        }
    }
    (playouts, start.elapsed())
}

/// The average length of 10,000 games from the start, played one `policy`
/// move at a time.
fn moves_per_playout(rng: &mut Rng, policy: fn(&Board, &mut Rng) -> Move) -> f64 {
    let mut played = 0u64;
    for _ in 0..10_000 {
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            board.play(policy(&board, rng));
            played += 1;
        }
    }
    played as f64 / 10_000.0
}

/// Formats a count with thousands separators: 1234567.8 -> "1,234,568".
fn thousands(value: f64) -> String {
    let digits = format!("{:.0}", value);
    let mut out = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            out.push(',');
        }
        out.push(digit);
    }
    out
}
