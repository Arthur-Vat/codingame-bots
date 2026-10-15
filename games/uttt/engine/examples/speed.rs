//! Measures the engine's speed with random playouts from the start position,
//! then with the decisive playouts that the search uses (a move that wins
//! the game when there is one), then the speed of that search (`cg-search`)
//! from the same position, then the cost of an NNUE of width 256 (random
//! weights): incremental updates, evaluations and accumulators from scratch.
//! Last, it measures where the time of the MCTS bot's search goes: searches
//! with the bot's network and settings, from positions of games it played,
//! and each part's share of their time (see [`measure_shares`]).
//!
//! ```sh
//! cargo run --release -p uttt-engine --example speed -- [SECONDS]
//! ```
//!
//! Prints a Markdown table, so CI can add it to the job summary.

use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use cg_core::rng::Rng;
use cg_search::{Budget, Game, Mcts};
use uttt_engine::nnue::{Accumulator, Network, BUCKETS, FEATURES};
use uttt_engine::search::{self, NnueBoard};
use uttt_engine::{Board, Move, MoveList, Status};

// The MCTS bot's network. Packed characters include ones that clippy takes
// for invisible.
#[allow(clippy::invisible_characters)]
#[path = "../../bots/mcts/src/nnue_weights.rs"]
mod nnue_weights;

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
    let nnue = measure_nnue(&mut rng, budget);
    println!(
        "| NNUE 256: incremental updates (one move) | {} per second |",
        thousands(nnue.updates)
    );
    println!(
        "| NNUE 256: evaluations | {} per second |",
        thousands(nnue.evaluations)
    );
    println!(
        "| NNUE 256: accumulators from scratch | {} per second |",
        thousands(nnue.refreshes)
    );
    let shares = measure_shares();
    println!(
        "| NNUE search: the MCTS bot's network and settings, {} searches of {} iterations | {} iterations per second |",
        shares.searches,
        thousands(SHARE_ITERATIONS as f64),
        thousands(shares.iterations_per_second)
    );
    println!(
        "| NNUE search: per iteration | {:.1} moves walking down, {:.2} nodes expanded, {:.1} children, {:.1} judged by the network |",
        shares.depth, shares.expansions, shares.children, shares.evaluations
    );
    for (part, share) in [
        (
            "updating the accumulators with each move walking down",
            shares.walk,
        ),
        ("copying the accumulators for each child", shares.copy),
        ("adding each child's move rows", shares.rows),
        ("evaluating each child", shares.evaluate),
        ("the rest: tree, moves, rules", shares.rest()),
    ] {
        println!(
            "| NNUE search: share of the time, {part} | {:.1}% |",
            100.0 * share
        );
    }
    println!(
        "| Measured for | {:.1} s of playouts and {searches} searches, one thread |",
        (elapsed + decisive_time).as_secs_f64()
    );
}

/// Width of the network measured.
const WIDTH: usize = 256;

/// Rates of the NNUE's operations, per second.
struct NnueRates {
    updates: f64,
    evaluations: f64,
    refreshes: f64,
}

/// A network of width `WIDTH` with random weights, from a seeded generator.
fn random_network(rng: &mut Rng) -> Network<WIDTH> {
    let mut weight = |range: u64| (rng.below(2 * range + 1) as i64 - range as i64) as i8;
    let feature_weights: Vec<i8> = (0..FEATURES * WIDTH).map(|_| weight(30)).collect();
    let feature_bias: Vec<i16> = (0..WIDTH).map(|_| i16::from(weight(60))).collect();
    let output_weights: Vec<i8> = (0..BUCKETS * 2 * WIDTH).map(|_| weight(60)).collect();
    Network::from_parts(
        &feature_weights,
        &feature_bias,
        &output_weights,
        &[0; BUCKETS],
    )
    .expect("random weights are in range")
}

/// Measures the NNUE for about `budget` per operation: updates of random
/// games from the start, evaluations of the positions of such games, and
/// accumulators built from scratch for the same positions.
fn measure_nnue(rng: &mut Rng, budget: Duration) -> NnueRates {
    let net = random_network(rng);
    let mut moves = MoveList::new();

    // 200 games from the start, as moves, and the positions they went through.
    let mut games: Vec<Vec<Move>> = Vec::new();
    let mut positions: Vec<Board> = Vec::new();
    for _ in 0..200 {
        let mut board = Board::new();
        let mut game = Vec::new();
        while board.status() == Status::Ongoing {
            board.legal_moves(&mut moves);
            let mv = *rng.pick(&moves).expect("the game goes on");
            positions.push(board);
            game.push(mv);
            board.play(mv);
        }
        games.push(game);
    }

    // Warm up, then count the updates of the games, replayed.
    let mut sink = 0.0f32;
    let mut updates = 0.0;
    for limit in [Duration::from_millis(200), budget] {
        let mut played = 0u64;
        let start = Instant::now();
        while start.elapsed() < limit {
            for game in &games {
                let mut board = Board::new();
                let mut accumulator = net.refresh(&board);
                for &mv in game {
                    net.play(&mut accumulator, &mut board, mv);
                }
                played += game.len() as u64;
                sink += f32::from(accumulator.values[0][0]);
            }
        }
        updates = played as f64 / start.elapsed().as_secs_f64();
    }

    let accumulators: Vec<(Accumulator<WIDTH>, Board)> = positions
        .iter()
        .map(|board| (net.refresh(board), *board))
        .collect();
    let mut evaluations = 0u64;
    let start = Instant::now();
    while start.elapsed() < budget {
        for (accumulator, board) in &accumulators {
            sink += net.evaluate(accumulator, board);
            evaluations += 1;
        }
    }
    let evaluations = evaluations as f64 / start.elapsed().as_secs_f64();

    let mut refreshes = 0u64;
    let start = Instant::now();
    while start.elapsed() < budget {
        for board in &positions {
            sink += f32::from(net.refresh(board).values[1][0]);
            refreshes += 1;
        }
    }
    let refreshes = refreshes as f64 / start.elapsed().as_secs_f64();

    // Keep the work from being optimized away.
    assert!(sink.is_finite());
    NnueRates {
        updates,
        evaluations,
        refreshes,
    }
}

/// The MCTS bot's exploration constant and minimax weight.
const BOT_EXPLORATION: f64 = 0.2;
const BOT_MINIMAX_WEIGHT: f64 = 0.5;
/// Iterations of each search whose time is shared out: about one turn of
/// the bot at CodinGame's limits (E018 measured a median of 21,000).
const SHARE_ITERATIONS: u64 = 20_000;
/// Iterations per move of the games that give the positions searched.
const GAME_ITERATIONS: u64 = 1_000;

/// The MCTS bot's network, decoded from its packed characters.
fn bot_network() -> &'static Network<256> {
    static NETWORK: std::sync::OnceLock<Network<256>> = std::sync::OnceLock::new();
    NETWORK.get_or_init(|| {
        let bytes = cg_pack::unpack(nnue_weights::NNUE_WEIGHTS).expect("the NNUE weights unpack");
        Network::from_bytes(&bytes).expect("the NNUE weights decode")
    })
}

/// A search of the MCTS bot's kind: expanding all children, with its
/// exploration constant and minimax weight.
fn bot_search<G: Game>() -> Mcts<G> {
    let mut mcts = Mcts::new(BOT_EXPLORATION, 1);
    mcts.expand_all = true;
    mcts.minimax_weight = BOT_MINIMAX_WEIGHT;
    mcts
}

/// How many more times a [`Probe`] does its part, for nothing: several, so
/// that the time it adds stands well above the machine's noise (about 10%).
const EXTRA: u32 = 4;

/// The part of [`NnueBoard`]'s work that a [`Probe`] does `EXTRA` more
/// times, or `Count` to count the work instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Part {
    None,
    Count,
    /// Updating a copy of the accumulators with each move played, most of
    /// them by iterations walking down the tree.
    Walk,
    /// Copying the accumulators for each child.
    Copy,
    /// Copying them and adding the child's move rows.
    Update,
    /// Evaluating each child the network judges.
    Evaluate,
}

/// Counts of the `Count` pass: expansions, children, network evaluations,
/// and moves played on positions (mostly by iterations walking down the
/// tree).
static COUNTS: [AtomicU64; 4] = [
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
    AtomicU64::new(0),
];

fn count(index: usize, amount: u64) {
    COUNTS[index].fetch_add(amount, Ordering::Relaxed);
}

/// [`NnueBoard`] with its `child_scores` written out, so that one part of
/// its work can be done more times: the search, its tree and its results
/// are those of `NnueBoard` (checked by [`measure_shares`]), and the time
/// the search takes more, divided by `EXTRA`, is that part's cost.
#[derive(Clone, Copy)]
struct Probe {
    inner: NnueBoard<256>,
    part: Part,
}

impl PartialEq for Probe {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Game for Probe {
    type Move = Move;

    fn to_move(&self) -> usize {
        self.inner.to_move()
    }

    fn legal_moves(&self, moves: &mut Vec<Move>) {
        self.inner.legal_moves(moves);
    }

    fn play(&mut self, mv: Move) {
        match self.part {
            Part::Count => count(3, 1),
            Part::Walk => {
                for _ in 0..EXTRA {
                    let mut extra = black_box(*self.inner.accumulator());
                    let mut board = *self.inner.board();
                    self.inner.network().play(&mut extra, &mut board, mv);
                    black_box(&extra);
                }
            }
            _ => {}
        }
        self.inner.play(mv);
    }

    fn score(&self) -> Option<f64> {
        self.inner.score()
    }

    fn playout(&mut self, rng: &mut Rng) -> f64 {
        self.inner.playout(rng)
    }

    fn child_scores(&self, moves: &[Move], _rng: &mut Rng, scores: &mut Vec<(f64, bool)>) {
        let network = self.inner.network();
        let board = self.inner.board();
        scores.clear();
        let accumulator = *self.inner.accumulator();
        if self.part == Part::Count {
            count(0, 1);
            count(1, moves.len() as u64);
        }
        for &mv in moves {
            let mut child = accumulator;
            let mut next = *board;
            network.play(&mut child, &mut next, mv);
            match self.part {
                Part::Copy => {
                    for _ in 0..EXTRA {
                        black_box(&black_box(accumulator));
                    }
                }
                Part::Update => {
                    for _ in 0..EXTRA {
                        let mut extra = black_box(accumulator);
                        let mut extra_board = *board;
                        network.play(&mut extra, &mut extra_board, mv);
                        black_box(&extra);
                    }
                }
                _ => {}
            }
            scores.push(match search::score(next.status()) {
                Some(score) => (score, true),
                None => {
                    let value = if next.game_winning_move().is_some() {
                        1.0
                    } else {
                        if self.part == Part::Evaluate {
                            for _ in 0..EXTRA {
                                black_box(network.evaluate(black_box(&child), &next));
                            }
                        }
                        if self.part == Part::Count {
                            count(2, 1);
                        }
                        f64::from(network.evaluate(&child, &next))
                    };
                    let seat_0 = if next.to_move() == 0 {
                        value
                    } else {
                        1.0 - value
                    };
                    (seat_0, false)
                }
            });
        }
    }
}

/// Where the time of the MCTS bot's search goes.
struct Shares {
    searches: usize,
    iterations_per_second: f64,
    /// Per iteration.
    expansions: f64,
    children: f64,
    evaluations: f64,
    /// Moves played on positions per iteration: the depth of its walk.
    depth: f64,
    /// Shares of the search's time.
    walk: f64,
    copy: f64,
    rows: f64,
    evaluate: f64,
}

impl Shares {
    fn rest(&self) -> f64 {
        1.0 - self.walk - self.copy - self.rows - self.evaluate
    }
}

/// Positions the MCTS bot meets: every eighth position from the fourth ply
/// of three games of its search against itself, at `GAME_ITERATIONS` per
/// move after four random plies, leaving out positions with a move that
/// wins the game (the search plays it without searching).
fn bot_positions() -> Vec<Board> {
    let network = bot_network();
    let mut positions = Vec::new();
    let mut moves = MoveList::new();
    for seed in 1..=3 {
        let mut rng = Rng::new(seed);
        let mut mcts = bot_search::<NnueBoard>();
        let mut board = Board::new();
        let mut ply = 0;
        while board.status() == Status::Ongoing {
            board.legal_moves(&mut moves);
            if ply >= 4 && ply % 8 == 4 && board.game_winning_move().is_none() && moves.len() > 1 {
                positions.push(board);
            }
            let mv = if ply < 4 {
                *rng.pick(&moves).expect("the game goes on")
            } else {
                let root = NnueBoard::new(board, network);
                mcts.search(&root, &moves, Budget::Iterations(GAME_ITERATIONS))
                    .best
            };
            board.play(mv);
            ply += 1;
        }
    }
    positions
}

/// Searches positions of the MCTS bot's games (see [`bot_positions`]) for
/// `SHARE_ITERATIONS` each, with the bot's network and settings, once with
/// [`NnueBoard`] and once with a [`Probe`] for each [`Part`]. Every search
/// of a position builds the same tree, so the time a probe takes more than
/// `Part::None`, divided by `EXTRA`, is the cost of its part; what is left of the
/// search's time is the tree's own work, the moves and the rules. Each
/// position is searched by every probe in turn, so that the machine's drift
/// weighs on all alike.
///
/// # Panics
///
/// If a probe's search ends differently from `NnueBoard`'s.
fn measure_shares() -> Shares {
    let network = bot_network();
    let positions = bot_positions();
    let parts = [
        Part::Count,
        Part::None,
        Part::Walk,
        Part::Copy,
        Part::Update,
        Part::Evaluate,
    ];
    // One search per part, each fed the same positions in the same order.
    let mut real = bot_search::<NnueBoard>();
    let mut probes: Vec<Mcts<Probe>> = parts.iter().map(|_| bot_search()).collect();
    let mut times = [Duration::ZERO; 6];
    let mut real_time = Duration::ZERO;
    let mut moves = MoveList::new();
    let budget = Budget::Iterations(SHARE_ITERATIONS);
    for &board in &positions {
        board.legal_moves(&mut moves);
        let inner = NnueBoard::new(board, network);
        let start = Instant::now();
        let expected = real.search(&inner, &moves, budget);
        real_time += start.elapsed();
        for ((&part, mcts), time) in parts.iter().zip(&mut probes).zip(&mut times) {
            let start = Instant::now();
            let result = mcts.search(&Probe { inner, part }, &moves, budget);
            *time += start.elapsed();
            assert_eq!(result, expected, "the probe {part:?} searched differently");
        }
    }

    let iterations = (positions.len() as u64 * SHARE_ITERATIONS) as f64;
    let per_iteration = |index: usize| COUNTS[index].load(Ordering::Relaxed) as f64 / iterations;
    let base = times[1].as_secs_f64();
    let extra = |index: usize| (times[index].as_secs_f64() - base) / base / f64::from(EXTRA);
    let copy = extra(3);
    // The walk's probe also copies the accumulators once per move.
    let copy_per_move = copy / per_iteration(1) * per_iteration(3);
    Shares {
        searches: positions.len(),
        iterations_per_second: iterations / real_time.as_secs_f64(),
        expansions: per_iteration(0),
        children: per_iteration(1),
        evaluations: per_iteration(2),
        depth: per_iteration(3),
        walk: extra(2) - copy_per_move,
        copy,
        rows: extra(4) - copy,
        evaluate: extra(5),
    }
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
