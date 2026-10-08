//! Measures what a value network would cost the search, before training
//! one (stage 2 of ADR 0016, proposal in ADR 0017).
//!
//! Untrained networks of several sizes, with random weights, replace the
//! playout at each new leaf of the search. The example measures how many
//! positions each evaluates per second, against the playouts of
//! `uttt-v008` (learned policy for 16 moves, then decisive moves) from the
//! same positions, then how many iterations a 90 ms search runs with each,
//! from positions at several stages of the game.
//!
//! ```sh
//! cargo run --release -p uttt-engine --example value_speed -- [SECONDS]
//! ```
//!
//! The network reads the position from the side to move's view: its marks
//! and the opponent's in each open small board, who won each closed board,
//! the board it is sent to, and each side's threats (open boards where it
//! has a cell that wins the board). Hidden layers use clipped ReLU, the
//! output a sigmoid: the side to move's expected score. Arithmetic is in
//! `f32`, in loops the compiler vectorizes for the plain x86-64 target.

use std::time::{Duration, Instant};

use cg_core::rng::Rng;
use cg_search::{Budget, Game, Mcts};
use uttt_engine::search::PolicyBoard;
use uttt_engine::{Board, Move, PlayoutPolicy, Status};

/// `uttt-v008`'s playout policy (`games/uttt/bots/mcts/src/weights.rs`).
static POLICY: PlayoutPolicy = PlayoutPolicy::new([
    800, 10000, 7504, 2816, 60, 1026, 618, 2201, 313, 1891, 1815, 1727, 1070, 1070, 1070, 1070,
    460, 3859, 2582, 2999, 165, 1083, 841, 1668, 113, 756, 800, 828, 1070, 1070, 1070, 1070,
])
.for_plies(16);

/// The exploration constant of the MCTS bot.
const EXPLORATION: f64 = 0.5;
/// The bot's search time per turn at CodinGame's limits.
const SEARCH: Duration = Duration::from_millis(90);

/// Inputs: 81 cells for the side to move's marks, 81 for the opponent's
/// (open boards only), 3 per small board for a closed board (won by the
/// side to move, won by the opponent, full), 10 for the board the side to
/// move is sent to (9 for any), 9 + 9 for each side's threats.
const INPUTS: usize = 81 + 81 + 27 + 10 + 18;
const STATUS: usize = 162;
const TARGET: usize = STATUS + 27;
const THREATS: usize = TARGET + 10;
/// At most 81 cells, 9 boards, 1 target and 18 threats are active.
const MAX_ACTIVE: usize = 128;

/// The inputs that are 1 in `board`, from the side to move's view.
fn active_inputs(board: &Board, list: &mut [u16; MAX_ACTIVE]) -> usize {
    let me = board.to_move();
    let them = 1 - me;
    let mut count = 0;
    let closed = board.closed_boards();
    for small in 0..9 {
        if closed & (1 << small) == 0 {
            for (seat, offset) in [(me, 0), (them, 81)] {
                let mut cells = board.cells(seat, small);
                while cells != 0 {
                    list[count] = (offset + small * 9 + cells.trailing_zeros() as usize) as u16;
                    count += 1;
                    cells &= cells - 1;
                }
            }
        } else {
            let status = if board.won_boards(me) & (1 << small) != 0 {
                0
            } else if board.won_boards(them) & (1 << small) != 0 {
                1
            } else {
                2
            };
            list[count] = (STATUS + small * 3 + status) as u16;
            count += 1;
        }
    }
    list[count] = (TARGET + board.target().unwrap_or(9)) as u16;
    count += 1;
    for (seat, offset) in [(me, 0), (them, 9)] {
        let mut threats = board.threat_boards(seat);
        while threats != 0 {
            list[count] = (THREATS + offset + threats.trailing_zeros() as usize) as u16;
            count += 1;
            threats &= threats - 1;
        }
    }
    count
}

/// Evaluates positions for the side to move.
trait Evaluator: Sync {
    fn name(&self) -> String;
    fn parameters(&self) -> usize;
    /// The side to move's expected score, from 0 to 1.
    fn evaluate(&self, board: &Board) -> f32;
}

/// `INPUTS -> H -> H2 -> 1`, or `INPUTS -> H -> 1` when `H2` is 0.
struct Network<const H: usize, const H2: usize> {
    input: Vec<[f32; H]>,
    input_bias: [f32; H],
    /// `hidden[k][j]` links first-layer unit `j` to second-layer unit `k`.
    hidden: Vec<[f32; H]>,
    hidden_bias: [f32; H2],
    /// The output weights of the last hidden layer (`H2`, or `H` when `H2`
    /// is 0).
    output: Vec<f32>,
    output_bias: f32,
}

impl<const H: usize, const H2: usize> Network<H, H2> {
    /// Random weights, scaled so that units neither all saturate nor all
    /// stay at 0.
    fn random(rng: &mut Rng) -> Self {
        let mut uniform = |scale: f64| ((rng.unit() * 2.0 - 1.0) * scale) as f32;
        let input = (0..INPUTS)
            .map(|_| std::array::from_fn(|_| uniform(0.15)))
            .collect();
        let input_bias = std::array::from_fn(|_| uniform(0.5));
        let hidden = (0..H2)
            .map(|_| std::array::from_fn(|_| uniform(1.0 / (H as f64).sqrt())))
            .collect();
        let hidden_bias = std::array::from_fn(|_| uniform(0.5));
        let last = if H2 == 0 { H } else { H2 };
        let output = (0..last).map(|_| uniform(1.0)).collect();
        Network {
            input,
            input_bias,
            hidden,
            hidden_bias,
            output,
            output_bias: uniform(0.1),
        }
    }
}

/// The dot product of two equally long slices, in 8 lanes so that it
/// vectorizes without reordering a single sum.
fn dot(a: &[f32], b: &[f32]) -> f32 {
    let mut lanes = [0.0f32; 8];
    let chunks = a.chunks_exact(8).zip(b.chunks_exact(8));
    for (x, y) in chunks {
        for (lane, (x, y)) in lanes.iter_mut().zip(x.iter().zip(y)) {
            *lane += x * y;
        }
    }
    let tail: f32 = a
        .chunks_exact(8)
        .remainder()
        .iter()
        .zip(b.chunks_exact(8).remainder())
        .map(|(x, y)| x * y)
        .sum();
    lanes.iter().sum::<f32>() + tail
}

impl<const H: usize, const H2: usize> Evaluator for Network<H, H2> {
    fn name(&self) -> String {
        if H2 == 0 {
            format!("{INPUTS}-{H}-1")
        } else {
            format!("{INPUTS}-{H}-{H2}-1")
        }
    }

    fn parameters(&self) -> usize {
        let hidden = if H2 == 0 { 0 } else { H * H2 + H2 };
        INPUTS * H + H + hidden + self.output.len() + 1
    }

    fn evaluate(&self, board: &Board) -> f32 {
        let mut list = [0u16; MAX_ACTIVE];
        let count = active_inputs(board, &mut list);
        let mut first = self.input_bias;
        for &input in &list[..count] {
            for (value, weight) in first.iter_mut().zip(&self.input[usize::from(input)]) {
                *value += weight;
            }
        }
        for value in &mut first {
            *value = value.clamp(0.0, 1.0);
        }
        let z = self.output_bias
            + if H2 == 0 {
                dot(&first, &self.output)
            } else {
                let mut second = self.hidden_bias;
                for (value, row) in second.iter_mut().zip(&self.hidden) {
                    *value = (*value + dot(&first, row)).clamp(0.0, 1.0);
                }
                dot(&second, &self.output)
            };
        1.0 / (1.0 + (-z).exp())
    }
}

/// A position searched with an evaluator at new leaves instead of a
/// playout. A side to move with a game-winning move wins: exact, and
/// cheap with the threats the board keeps.
#[derive(Clone, Copy)]
struct ValueBoard {
    board: Board,
    evaluator: &'static dyn Evaluator,
}

impl PartialEq for ValueBoard {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl Game for ValueBoard {
    type Move = Move;

    fn to_move(&self) -> usize {
        self.board.to_move()
    }

    fn legal_moves(&self, moves: &mut Vec<Move>) {
        Game::legal_moves(&self.board, moves);
    }

    fn play(&mut self, mv: Move) {
        self.board.play(mv);
    }

    fn score(&self) -> Option<f64> {
        uttt_engine::search::score(self.board.status())
    }

    fn playout(&mut self, _rng: &mut Rng) -> f64 {
        let value = if self.board.game_winning_move().is_some() {
            1.0
        } else {
            f64::from(self.evaluator.evaluate(&self.board))
        };
        if self.board.to_move() == 0 {
            value
        } else {
            1.0 - value
        }
    }
}

fn main() {
    let seconds: f64 = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(1.0);
    let budget = Duration::from_secs_f64(seconds);
    let mut rng = Rng::new(1);
    let positions = sample_positions(&mut rng, 1_000);

    let evaluators: Vec<&'static dyn Evaluator> = vec![
        Box::leak(Box::new(Network::<32, 0>::random(&mut rng))),
        Box::leak(Box::new(Network::<64, 16>::random(&mut rng))),
        Box::leak(Box::new(Network::<128, 32>::random(&mut rng))),
        Box::leak(Box::new(Network::<256, 32>::random(&mut rng))),
    ];

    // Warm up caches and CPU frequency before measuring.
    playouts_per_second(&positions, &mut rng, Duration::from_millis(300));
    let playouts = playouts_per_second(&positions, &mut rng, budget);
    println!(
        "Positions: {} from 1,000 decisive self-play games, every position of each.\n",
        thousands(positions.len() as f64)
    );
    println!("| Leaf estimate | Parameters | Per second | Time each |");
    println!("| --- | --- | --- | --- |");
    println!(
        "| v008 playout (16 policy moves, then decisive) | 32 | {} | {:.2} µs |",
        thousands(playouts),
        1e6 / playouts
    );
    for evaluator in &evaluators {
        let rate = evaluations_per_second(*evaluator, &positions, budget);
        println!(
            "| Network {} | {} | {} | {:.2} µs |",
            evaluator.name(),
            thousands(evaluator.parameters() as f64),
            thousands(rate),
            1e6 / rate
        );
    }

    println!();
    let plies = [0, 10, 20, 30, 40];
    let per_ply = 8;
    let searched: Vec<Vec<Board>> = plies
        .iter()
        .map(|&ply| search_positions(&mut rng, ply, per_ply))
        .collect();
    print!(
        "| Iterations per {} ms search, average of {per_ply} positions | ",
        SEARCH.as_millis()
    );
    for ply in plies {
        print!("Move {} | ", ply + 1);
    }
    println!();
    println!("| --- |{}", " --- |".repeat(plies.len()));
    print!("| v008 playouts | ");
    for boards in &searched {
        let iterations = boards.iter().map(|&board| {
            iterations(PolicyBoard {
                board,
                policy: &POLICY,
            })
        });
        print!("{} | ", thousands(average(iterations)));
    }
    println!();
    for evaluator in &evaluators {
        print!("| Network {} | ", evaluator.name());
        for boards in &searched {
            let iterations = boards.iter().map(|&board| {
                iterations(ValueBoard {
                    board,
                    evaluator: *evaluator,
                })
            });
            print!("{} | ", thousands(average(iterations)));
        }
        println!();
    }
}

/// Every ongoing position of `games` games of decisive moves from the start.
fn sample_positions(rng: &mut Rng, games: usize) -> Vec<Board> {
    let mut positions = Vec::new();
    for _ in 0..games {
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            positions.push(board);
            board.play(board.decisive_move(rng));
        }
    }
    positions
}

/// `count` positions after `ply` decisive moves from the start, still going
/// on and without a game-winning move, where a search would return at once.
fn search_positions(rng: &mut Rng, ply: usize, count: usize) -> Vec<Board> {
    let mut found = Vec::new();
    while found.len() < count {
        let mut board = Board::new();
        for _ in 0..ply {
            if board.status() != Status::Ongoing {
                break;
            }
            board.play(board.decisive_move(rng));
        }
        if board.status() == Status::Ongoing && board.game_winning_move().is_none() {
            found.push(board);
        }
    }
    found
}

/// Iterations of one search of `SEARCH` from `root`, with a fresh tree.
fn iterations<G: Game<Move = Move>>(root: G) -> u64 {
    let mut mcts = Mcts::new(EXPLORATION, 7);
    let mut moves = Vec::new();
    root.legal_moves(&mut moves);
    let result = mcts.search(&root, &moves, Budget::Until(Instant::now() + SEARCH));
    result.iterations
}

/// `uttt-v008` playouts per second from `positions`, in turn.
fn playouts_per_second(positions: &[Board], rng: &mut Rng, budget: Duration) -> f64 {
    let start = Instant::now();
    let mut count = 0u64;
    let mut sink = 0u32;
    while start.elapsed() < budget {
        for position in positions.iter().take(10_000) {
            let mut board = *position;
            if board.policy_playout(&POLICY, rng) == Status::Draw {
                sink += 1;
            }
            count += 1;
        }
    }
    std::hint::black_box(sink);
    count as f64 / start.elapsed().as_secs_f64()
}

/// Evaluations per second of `evaluator` on `positions`, in turn.
fn evaluations_per_second(evaluator: &dyn Evaluator, positions: &[Board], budget: Duration) -> f64 {
    let start = Instant::now();
    let mut count = 0u64;
    let mut sink = 0.0f32;
    while start.elapsed() < budget {
        for position in positions.iter().take(10_000) {
            sink += evaluator.evaluate(std::hint::black_box(position));
            count += 1;
        }
    }
    std::hint::black_box(sink);
    count as f64 / start.elapsed().as_secs_f64()
}

fn average(values: impl Iterator<Item = u64>) -> f64 {
    let (sum, count) = values.fold((0, 0), |(sum, count), value| (sum + value, count + 1));
    sum as f64 / f64::from(count.max(1))
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
