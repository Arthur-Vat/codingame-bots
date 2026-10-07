//! Measures the engine's speed with random playouts from the start position,
//! the core of Monte Carlo tree search.
//!
//! ```sh
//! cargo run --release -p uttt-engine --example speed -- [SECONDS]
//! ```
//!
//! Prints a Markdown table, so CI can add it to the job summary.

use std::time::{Duration, Instant};

use cg_core::rng::Rng;
use uttt_engine::{Board, MoveList, Status};

fn main() {
    let seconds: f64 = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(2.0);
    let budget = Duration::from_secs_f64(seconds);
    let mut rng = Rng::new(1);
    let mut moves = MoveList::new();

    // Warm up caches and CPU frequency before measuring.
    measure(&mut rng, &mut moves, Duration::from_millis(200));
    let (playouts, played, elapsed) = measure(&mut rng, &mut moves, budget);

    let per_second = playouts as f64 / elapsed.as_secs_f64();
    println!("| Engine benchmark | Result |");
    println!("| --- | --- |");
    println!(
        "| Random playouts from the start | {} per second |",
        thousands(per_second)
    );
    println!(
        "| Moves per playout | {:.1} |",
        played as f64 / playouts as f64
    );
    println!(
        "| Moves per second | {} |",
        thousands(played as f64 / elapsed.as_secs_f64())
    );
    println!(
        "| Measured for | {:.1} s, one thread |",
        elapsed.as_secs_f64()
    );
}

/// Plays random games for `budget`; returns games, moves and time taken.
fn measure(rng: &mut Rng, moves: &mut MoveList, budget: Duration) -> (u64, u64, Duration) {
    let start = Instant::now();
    let (mut playouts, mut played) = (0u64, 0u64);
    while start.elapsed() < budget {
        for _ in 0..1000 {
            let mut board = Board::new();
            while board.status() == Status::Ongoing {
                board.legal_moves(moves);
                board.play(moves[rng.below(moves.len() as u64) as usize]);
                played += 1;
            }
            playouts += 1;
        }
    }
    (playouts, played, start.elapsed())
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
