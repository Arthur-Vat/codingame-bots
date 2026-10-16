//! Trains the NNUE of `uttt_engine::nnue` on self-play games: 189 inputs per
//! perspective, an accumulator of `N` units shared by both seats, and 30
//! output buckets (a target and a game phase), then quantizes it as the
//! engine's integer arithmetic expects.
//!
//! The parameters are one flat vector of `f32`: the first layer's weights
//! `W` (`FEATURES * N`, feature-major), its biases `b` (`N`), the output
//! weights `V` (`BUCKETS * 2N`, bucket-major, the side to move's half first)
//! and the output biases `c` (`BUCKETS`). For a position, the accumulator of
//! a seat is `b` plus the rows of `W` of the seat's active inputs, clipped
//! to 0..1 and squared; the logit is the bucket's `V` against the squares of
//! both accumulators, plus `c`. Training minimises the cross-entropy
//! against the target with Adam on mini-batches spread over threads, each
//! position shown in one of the board's 8 symmetries, drawn at random.

use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::{Condvar, Mutex, RwLock};
use std::time::Instant;

use cg_core::rng::Rng;
use uttt_engine::nnue::{
    self, Network, BUCKETS, FEATURES, MAX_ACTIVE, MAX_BIAS, OUR_BOARDS, QA, QB, TARGETS,
    THEIR_CELLS,
};
use uttt_engine::{Board, MoveList, Status};

use crate::value_fit::{
    self, cross_entropy, shares, sigmoid, symmetric_cell, Examples, Metrics, Positions, Training,
};
use crate::{read_file, WidthArg};

/// Where each parameter group starts in the flat vector.
pub struct Shape<const N: usize>;

impl<const N: usize> Shape<N> {
    pub const WEIGHTS: usize = 0;
    pub const BIAS: usize = FEATURES * N;
    pub const OUTPUT: usize = Self::BIAS + N;
    pub const OUTPUT_BIAS: usize = Self::OUTPUT + BUCKETS * 2 * N;
    /// The number of parameters.
    pub const PARAMETERS: usize = Self::OUTPUT_BIAS + BUCKETS;
}

/// One of the board's 8 symmetries, as the input and the bucket that each
/// input and bucket becomes when the same symmetry is applied to the big
/// grid and to every small board: the game stays the same, since a move's
/// cell names the next small board.
pub struct Symmetry {
    pub features: [u16; FEATURES],
    pub buckets: [u8; BUCKETS],
}

impl Symmetry {
    /// Symmetry `index` of `value_fit::symmetric_cell`; 0 is the identity.
    pub fn new(index: usize) -> Symmetry {
        let map = |cell: usize| symmetric_cell(index, cell);
        let features = std::array::from_fn(|feature| {
            let image = if feature < OUR_BOARDS {
                let offset = if feature < THEIR_CELLS {
                    0
                } else {
                    THEIR_CELLS
                };
                let cell = feature - offset;
                offset + 9 * map(cell / 9) + map(cell % 9)
            } else {
                let start = OUR_BOARDS + (feature - OUR_BOARDS) / 9 * 9;
                start + map(feature - start)
            };
            image as u16
        });
        // A symmetry moves the target, not the game's phase.
        let buckets = std::array::from_fn(|bucket| {
            let (phase, target) = (bucket / TARGETS, bucket % TARGETS);
            let image = if target == 9 { 9 } else { map(target) };
            (phase * TARGETS + image) as u8
        });
        Symmetry { features, buckets }
    }
}

/// The 8 symmetries.
pub fn symmetries() -> Vec<Symmetry> {
    (0..8).map(Symmetry::new).collect()
}

/// A position as the network sees it, through one symmetry: the active
/// inputs of the side to move (list 0) and of the other seat (list 1), and
/// the output bucket.
pub struct View {
    lists: [[u16; MAX_ACTIVE]; 2],
    lens: [usize; 2],
    bucket: usize,
}

impl View {
    pub fn new(board: &Board, symmetry: &Symmetry) -> View {
        let stm = board.to_move();
        let mut lists = [[0u16; MAX_ACTIVE]; 2];
        let mut lens = [0; 2];
        for (side, seat) in [stm, 1 - stm].into_iter().enumerate() {
            lens[side] = nnue::active_features(board, seat, &mut lists[side]);
            for feature in &mut lists[side][..lens[side]] {
                *feature = symmetry.features[usize::from(*feature)];
            }
        }
        View {
            lists,
            lens,
            bucket: usize::from(symmetry.buckets[nnue::bucket(board)]),
        }
    }

    fn active(&self, side: usize) -> &[u16] {
        &self.lists[side][..self.lens[side]]
    }
}

/// The sum of `clamp(acc, 0, 1)^2 * weight`, in 8 lanes.
fn squared_dot(acc: &[f32], weights: &[f32]) -> f32 {
    let square = |x: f32| {
        let a = x.clamp(0.0, 1.0);
        a * a
    };
    let mut lanes = [0.0f32; 8];
    for (x, w) in acc.chunks_exact(8).zip(weights.chunks_exact(8)) {
        for (lane, (&x, &w)) in lanes.iter_mut().zip(x.iter().zip(w)) {
            *lane += square(x) * w;
        }
    }
    let tail: f32 = acc
        .chunks_exact(8)
        .remainder()
        .iter()
        .zip(weights.chunks_exact(8).remainder())
        .map(|(&x, &w)| square(x) * w)
        .sum();
    lanes.iter().sum::<f32>() + tail
}

/// Values of the network for one position.
struct Forward<const N: usize> {
    /// The accumulators before clipping: the side to move's, then the
    /// other seat's.
    acc: [[f32; N]; 2],
    /// The output before the sigmoid.
    logit: f32,
}

fn forward<const N: usize>(parameters: &[f32], view: &View) -> Forward<N> {
    let bias = &parameters[Shape::<N>::BIAS..][..N];
    let output = &parameters[Shape::<N>::OUTPUT + view.bucket * 2 * N..][..2 * N];
    let mut acc = [[0.0f32; N]; 2];
    let mut logit = parameters[Shape::<N>::OUTPUT_BIAS + view.bucket];
    for (side, values) in acc.iter_mut().enumerate() {
        values.copy_from_slice(bias);
        for &feature in view.active(side) {
            let row = &parameters[Shape::<N>::WEIGHTS + usize::from(feature) * N..][..N];
            for (value, weight) in values.iter_mut().zip(row) {
                *value += weight;
            }
        }
        logit += squared_dot(values, &output[side * N..][..N]);
    }
    Forward { acc, logit }
}

/// Adds the gradient of the cross-entropy of one position to `gradient`,
/// and returns the cross-entropy.
fn accumulate<const N: usize>(
    parameters: &[f32],
    view: &View,
    target: f32,
    gradient: &mut [f32],
) -> f64 {
    let values = forward::<N>(parameters, view);
    let d_logit = sigmoid(values.logit) - target;
    gradient[Shape::<N>::OUTPUT_BIAS + view.bucket] += d_logit;
    let output = Shape::<N>::OUTPUT + view.bucket * 2 * N;
    for (side, acc) in values.acc.iter().enumerate() {
        let weights = &parameters[output + side * N..][..N];
        let output_gradient = &mut gradient[output + side * N..][..N];
        let mut d_acc = [0.0f32; N];
        for (((x, w), g), d) in acc
            .iter()
            .zip(weights)
            .zip(output_gradient.iter_mut())
            .zip(d_acc.iter_mut())
        {
            let a = x.clamp(0.0, 1.0);
            *g += d_logit * a * a;
            *d = if *x > 0.0 && *x < 1.0 {
                d_logit * w * 2.0 * a
            } else {
                0.0
            };
        }
        for &feature in view.active(side) {
            let row = &mut gradient[Shape::<N>::WEIGHTS + usize::from(feature) * N..][..N];
            for (g, d) in row.iter_mut().zip(&d_acc) {
                *g += d;
            }
        }
        for (g, d) in gradient[Shape::<N>::BIAS..][..N].iter_mut().zip(&d_acc) {
            *g += d;
        }
    }
    cross_entropy(values.logit, target)
}

/// Starting parameters: small random first-layer weights, biases that put
/// the accumulators in the middle of their range, small random output
/// weights.
pub fn initial_parameters<const N: usize>(seed: u64) -> Vec<f32> {
    let mut rng = Rng::new(seed);
    let mut uniform = |spread: f64| ((rng.unit() * 2.0 - 1.0) * spread) as f32;
    let mut parameters = Vec::with_capacity(Shape::<N>::PARAMETERS);
    parameters.extend((0..FEATURES * N).map(|_| uniform(0.1)));
    parameters.extend([0.5; N]);
    let spread = 1.0 / ((2 * N) as f64).sqrt();
    parameters.extend((0..BUCKETS * 2 * N).map(|_| uniform(spread)));
    parameters.extend([0.0; BUCKETS]);
    parameters
}

/// Clips the parameters to what the quantization can hold: first-layer
/// weights to -1..=1 (127 at scale `QA`), biases to what keeps the
/// accumulators within `i16`, output weights to 127 at scale `QB`, output
/// biases to a magnitude of 1,000. `values` holds the parameters from index
/// `start` on: all of them from 0, or a thread's share.
pub fn clip<const N: usize>(start: usize, values: &mut [f32]) {
    let groups = [
        (Shape::<N>::BIAS, 127.0 / QA as f32),
        (Shape::<N>::OUTPUT, f32::from(MAX_BIAS) / QA as f32),
        (Shape::<N>::OUTPUT_BIAS, 127.0 / QB as f32),
        (Shape::<N>::PARAMETERS, 1000.0),
    ];
    let mut index = start;
    let mut rest = values;
    for (end, limit) in groups {
        let (group, after) = rest.split_at_mut(end.saturating_sub(index).min(rest.len()));
        for value in group.iter_mut() {
            *value = value.clamp(-limit, limit);
        }
        index += group.len();
        rest = after;
    }
}

/// The parameters as the bot stores them: the engine's network, with the
/// quantization of `uttt_engine::nnue`'s documentation.
pub fn quantize<const N: usize>(parameters: &[f32]) -> Result<Network<N>, String> {
    let (weights, rest) = parameters.split_at(Shape::<N>::BIAS);
    let (bias, rest) = rest.split_at(N);
    let (output, output_bias) = rest.split_at(BUCKETS * 2 * N);
    let byte = |value: f32, scale: i32| (scale as f32 * value).round().clamp(-127.0, 127.0) as i8;
    let weights: Vec<i8> = weights.iter().map(|&w| byte(w, QA)).collect();
    let max_bias = f32::from(MAX_BIAS);
    let bias: Vec<i16> = bias
        .iter()
        .map(|&b| (QA as f32 * b).round().clamp(-max_bias, max_bias) as i16)
        .collect();
    let output: Vec<i8> = output.iter().map(|&v| byte(v, QB)).collect();
    let mut biases = [0i32; BUCKETS];
    for (bias, &c) in biases.iter_mut().zip(output_bias) {
        *bias = ((QA * QB) as f32 * c).round() as i32;
    }
    Network::<N>::from_parts(&weights, &bias, &output, &biases)
}

/// The metrics of the logits that `logit` gives on `examples`, against the
/// results, over `threads` threads.
fn measure_logits<E: Examples + ?Sized>(
    examples: &E,
    threads: usize,
    logit: impl Fn(&Board) -> f32 + Sync,
) -> Metrics {
    let logit = &logit;
    let sums: Vec<(f64, f64)> = std::thread::scope(|scope| {
        let handles: Vec<_> = shares(examples.len(), threads)
            .into_iter()
            .map(|range| {
                scope.spawn(move || {
                    let (mut entropy, mut squared) = (0.0, 0.0);
                    for index in range {
                        let example = examples.get(index);
                        let z = logit(&example.board);
                        entropy += cross_entropy(z, example.result);
                        squared += f64::from(sigmoid(z) - example.result).powi(2);
                    }
                    (entropy, squared)
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().expect("a measuring thread panicked"))
            .collect()
    });
    let count = examples.len().max(1) as f64;
    Metrics {
        cross_entropy: sums.iter().map(|sum| sum.0).sum::<f64>() / count,
        squared_error: sums.iter().map(|sum| sum.1).sum::<f64>() / count,
    }
}

/// The metrics of the float network `parameters` on `examples`, without
/// symmetries.
pub fn measure<const N: usize>(
    parameters: &[f32],
    examples: &(impl Examples + ?Sized),
    threads: usize,
) -> Metrics {
    let identity = Symmetry::new(0);
    measure_logits(examples, threads, |board| {
        forward::<N>(parameters, &View::new(board, &identity)).logit
    })
}

/// The metrics of the quantized `network` on `examples`, by the engine's
/// own arithmetic.
pub fn measure_quantized<const N: usize>(
    network: &Network<N>,
    examples: &(impl Examples + ?Sized),
    threads: usize,
) -> Metrics {
    measure_logits(examples, threads, |board| {
        network.logit(&network.refresh(board), board)
    })
}

/// What an epoch of training gave.
#[derive(Clone, Copy, Debug)]
pub struct Epoch {
    pub number: u32,
    /// Average cross-entropy over the epoch's fitted positions, in their
    /// symmetries, as the parameters changed.
    pub fitted: f64,
    pub held_out: Metrics,
    /// Seconds spent training, without measuring, up to the end of this
    /// epoch.
    pub training_seconds: f64,
}

/// A barrier for the threads of `train` that a panicking thread breaks, so
/// that the others panic too instead of waiting for it forever.
struct Gate {
    threads: usize,
    /// The threads arrived in this round, the round's number, and whether
    /// a thread panicked.
    state: Mutex<(usize, u64, bool)>,
    turned: Condvar,
}

impl Gate {
    fn new(threads: usize) -> Gate {
        Gate {
            threads,
            state: Mutex::new((0, 0, false)),
            turned: Condvar::new(),
        }
    }

    /// Waits until all the threads have called it in this round.
    fn wait(&self) {
        let broken = "another training thread panicked";
        let mut state = self.state.lock().expect(broken);
        assert!(!state.2, "{broken}");
        state.0 += 1;
        if state.0 == self.threads {
            state.0 = 0;
            state.1 += 1;
            self.turned.notify_all();
            return;
        }
        let round = state.1;
        let state = self
            .turned
            .wait_while(state, |state| state.1 == round && !state.2)
            .expect(broken);
        // A round that turned lets its threads go, even if a thread has
        // panicked since: their next round fails at its start.
        assert!(state.1 != round, "{broken}");
    }

    /// Breaks the gate if the calling thread is panicking.
    fn guard(&self) -> impl Drop + '_ {
        struct Guard<'a>(&'a Gate);
        impl Drop for Guard<'_> {
            fn drop(&mut self) {
                if std::thread::panicking() {
                    if let Ok(mut state) = self.0.state.lock() {
                        state.2 = true;
                    }
                    self.0.turned.notify_all();
                }
            }
        }
        Guard(self)
    }
}

/// One gradient step, as the main thread of `train` hands it to the
/// workers.
#[derive(Default)]
struct Step {
    /// The batch's positions, each with its symmetry.
    views: Vec<(u32, u8)>,
    /// Positions per worker: worker `t` takes the `t`-th chunk, and the last
    /// workers none when the batch is short.
    part: usize,
    rate: f32,
    /// Adam's bias corrections.
    corrections: (f32, f32),
    /// Training is over: the workers return.
    done: bool,
}

/// Trains from `parameters` on `fitted`, measuring on `held_out` after each
/// epoch, and calls `report` then. `settings.penalty` weighs an L2 penalty
/// as in `value_fit`; `fit-nnue` sets it to 0.
///
/// `settings.threads` workers live as long as the training. In each step,
/// worker `t` sums the gradient of the batch's `t`-th part, then, once all
/// are done, sums the parts' gradients over its own share of the
/// parameters, in the parts' order, and takes Adam's step on that share;
/// the main thread then copies the shares into the parameters. The result
/// depends on the number of threads (the order of the sums), not on their
/// timing.
pub fn train<const N: usize>(
    parameters: Vec<f32>,
    fitted: &(impl Examples + ?Sized),
    held_out: &(impl Examples + ?Sized),
    settings: &Training,
    mut report: impl FnMut(&Epoch),
) -> Vec<f32> {
    let symmetries = symmetries();
    let threads = settings.threads.max(1);
    let mut rng = Rng::new(settings.seed);
    let count = Shape::<N>::PARAMETERS;
    let (beta1, beta2, epsilon) = (0.9f32, 0.999f32, 1e-8f32);
    let steps_per_epoch = fitted.len().div_ceil(settings.batch);
    let total_steps = (steps_per_epoch * settings.epochs as usize).max(1);
    let fitted_count = u32::try_from(fitted.len()).expect("fewer than 2^32 positions to fit");
    let mut order: Vec<u32> = (0..fitted_count).collect();

    // Each worker's share of the parameters for Adam's step.
    let mut shares = shares(count, threads);
    shares.resize(threads, count..count);
    let parameters = RwLock::new(parameters);
    let step = RwLock::new(Step::default());
    let gradients: Vec<RwLock<Vec<f32>>> = (0..threads)
        .map(|_| RwLock::new(vec![0.0f32; count]))
        .collect();
    let losses: Vec<Mutex<f64>> = (0..threads).map(|_| Mutex::new(0.0)).collect();
    let updated: Vec<Mutex<Vec<f32>>> = shares
        .iter()
        .map(|share| Mutex::new(vec![0.0f32; share.len()]))
        .collect();
    // Three rounds per step: the step is ready, the gradients are summed,
    // the shares are updated.
    let gate = Gate::new(threads + 1);

    std::thread::scope(|scope| {
        for (worker, share) in shares.iter().cloned().enumerate() {
            let (parameters, step, gradients, symmetries) =
                (&parameters, &step, &gradients, &symmetries);
            let (loss, updated, gate) = (&losses[worker], &updated[worker], &gate);
            scope.spawn(move || {
                let _guard = gate.guard();
                let (mut first, mut second) =
                    (vec![0.0f32; share.len()], vec![0.0f32; share.len()]);
                let mut sum = vec![0.0f32; share.len()];
                loop {
                    gate.wait();
                    let step = step.read().expect("the step");
                    if step.done {
                        return;
                    }
                    if let Some(chunk) = step.views.chunks(step.part).nth(worker) {
                        let parameters = parameters.read().expect("the parameters");
                        let mut gradient = gradients[worker].write().expect("a gradient");
                        gradient.fill(0.0);
                        let mut total = 0.0;
                        for &(index, symmetry) in chunk {
                            let example = fitted.get(index as usize);
                            let view =
                                View::new(&example.board, &symmetries[usize::from(symmetry)]);
                            total += accumulate::<N>(
                                &parameters,
                                &view,
                                settings.target.of(&example),
                                &mut gradient,
                            );
                        }
                        *loss.lock().expect("a loss") = total;
                    }
                    gate.wait();
                    {
                        let used = step.views.len().div_ceil(step.part);
                        let parameters = parameters.read().expect("the parameters");
                        for (part, gradient) in gradients[..used].iter().enumerate() {
                            let gradient = gradient.read().expect("a gradient");
                            let gradient = &gradient[share.clone()];
                            if part == 0 {
                                sum.copy_from_slice(gradient);
                            } else {
                                for (sum, value) in sum.iter_mut().zip(gradient) {
                                    *sum += value;
                                }
                            }
                        }
                        let scale = 1.0 / step.views.len() as f32;
                        let (correction1, correction2) = step.corrections;
                        let mut updated = updated.lock().expect("a share");
                        for ((new, (&value, &gradient)), (m, v)) in updated
                            .iter_mut()
                            .zip(parameters[share.clone()].iter().zip(&sum))
                            .zip(first.iter_mut().zip(second.iter_mut()))
                        {
                            let g = gradient * scale + settings.penalty * value;
                            *m = beta1 * *m + (1.0 - beta1) * g;
                            *v = beta2 * *v + (1.0 - beta2) * g * g;
                            *new = value
                                - step.rate * (*m / correction1)
                                    / ((*v / correction2).sqrt() + epsilon);
                        }
                        clip::<N>(share.start, &mut updated);
                    }
                    drop(step);
                    gate.wait();
                }
            });
        }

        let _guard = gate.guard();
        let mut step_number: u32 = 0;
        let mut training_seconds = 0.0;
        for number in 1..=settings.epochs {
            let start = Instant::now();
            rng.shuffle(&mut order);
            let mut epoch_loss = 0.0;
            for batch in order.chunks(settings.batch) {
                step_number += 1;
                let used = {
                    let mut step = step.write().expect("the step");
                    step.views.clear();
                    step.views
                        .extend(batch.iter().map(|&index| (index, rng.below(8) as u8)));
                    step.part = batch.len().div_ceil(threads);
                    let progress = step_number as f32 / total_steps as f32;
                    step.rate = settings.rate
                        * (0.05 + 0.95 * 0.5 * (1.0 + (std::f32::consts::PI * progress).cos()));
                    step.corrections = (
                        1.0 - beta1.powi(step_number as i32),
                        1.0 - beta2.powi(step_number as i32),
                    );
                    batch.len().div_ceil(step.part)
                };
                gate.wait();
                gate.wait();
                epoch_loss += losses[..used]
                    .iter()
                    .map(|loss| *loss.lock().expect("a loss"))
                    .sum::<f64>();
                gate.wait();
                let mut parameters = parameters.write().expect("the parameters");
                for (share, updated) in shares.iter().zip(&updated) {
                    parameters[share.clone()].copy_from_slice(&updated.lock().expect("a share"));
                }
            }
            training_seconds += start.elapsed().as_secs_f64();
            report(&Epoch {
                number,
                fitted: epoch_loss / fitted.len().max(1) as f64,
                held_out: measure::<N>(
                    &parameters.read().expect("the parameters"),
                    held_out,
                    threads,
                ),
                training_seconds,
            });
        }
        step.write().expect("the step").done = true;
        gate.wait();
    });
    parameters.into_inner().expect("the parameters")
}

/// The accumulator's width, from the command line.
fn width(arg: WidthArg) -> usize {
    match arg {
        WidthArg::W128 => 128,
        WidthArg::W256 => 256,
        WidthArg::W512 => 512,
    }
}

/// What `fit-nnue` was asked.
pub struct Options {
    pub data: Vec<PathBuf>,
    pub width: WidthArg,
    pub training: Training,
    pub compared_positions: usize,
    /// A network to measure on the same held-out positions, such as the
    /// bot's, with the file it came from.
    pub compare: Option<(String, Network<256>)>,
    pub weights_out: Option<PathBuf>,
    pub report_out: Option<PathBuf>,
    pub origin: String,
}

pub fn fit(options: &Options) -> Result<(), String> {
    match width(options.width) {
        128 => fit_sized::<128>(options),
        256 => fit_sized::<256>(options),
        _ => fit_sized::<512>(options),
    }
}

/// The constant predictor `average`'s metrics on `results`.
fn constant_metrics(average: f64, results: impl Iterator<Item = f32>) -> Metrics {
    let p = average.clamp(1e-9, 1.0 - 1e-9);
    let (mut entropy, mut squared, mut count) = (0.0, 0.0, 0usize);
    for result in results {
        let result = f64::from(result);
        entropy -= result * p.ln() + (1.0 - result) * (1.0 - p).ln();
        squared += (p - result).powi(2);
        count += 1;
    }
    let count = count.max(1) as f64;
    Metrics {
        cross_entropy: entropy / count,
        squared_error: squared / count,
    }
}

/// The plies of each row of the data's table, from the first to the last.
const PLY_ROWS: [(u8, u8); 7] = [
    (0, 3),
    (4, 7),
    (8, 11),
    (12, 15),
    (16, 23),
    (24, 31),
    (32, 81),
];

/// The plies at which the data's table counts distinct positions.
const DISTINCT_PLIES: [u8; 3] = [4, 8, 12];

/// What the positions of `positions` look like, ply by ply: how many, how
/// far the root score is from even on average, the shares whose root score
/// leans to one side (beyond 0.3 or 0.7) or says a side is nearly sure to
/// win (beyond 0.1 or 0.9); then the first player's results over the
/// games, and how many distinct positions there are at a few plies, which
/// says how varied the openings are.
pub fn data_table(positions: &Positions) -> String {
    let mut rows = [(0usize, 0.0f64, 0usize, 0usize); PLY_ROWS.len()];
    let mut distinct: Vec<(usize, std::collections::HashSet<Board>)> = DISTINCT_PLIES
        .iter()
        .map(|_| (0, std::collections::HashSet::new()))
        .collect();
    // The first player's wins, draws and losses, in half points.
    let mut first_player = [0usize; 3];
    let mut last_game = None;
    for index in 0..positions.len() {
        let ply = positions.ply(index);
        let score = positions.score(index);
        if let Some(row) = PLY_ROWS
            .iter()
            .position(|&(first, last)| (first..=last).contains(&ply))
        {
            let row = &mut rows[row];
            row.0 += 1;
            row.1 += f64::from((score - 0.5).abs());
            row.2 += usize::from(!(0.3..=0.7).contains(&score));
            row.3 += usize::from(!(0.1..=0.9).contains(&score));
        }
        if let Some(at) = DISTINCT_PLIES.iter().position(|&p| p == ply) {
            distinct[at].0 += 1;
            distinct[at].1.insert(positions.get(index).board);
        }
        let game = positions.game(index);
        if last_game != Some(game) {
            last_game = Some(game);
            let result = positions.result(index);
            let result = if ply.is_multiple_of(2) {
                result
            } else {
                1.0 - result
            };
            first_player[(2.0 - 2.0 * result) as usize] += 1;
        }
    }
    let mut table = String::new();
    let _ = writeln!(
        table,
        "| Plies | Positions | Root score's average distance from 0.5 | Root score beyond 0.3 or 0.7 | Beyond 0.1 or 0.9 |"
    );
    let _ = writeln!(table, "| --- | --- | --- | --- | --- |");
    for (&(first, last), &(count, distance, leaning, decided)) in PLY_ROWS.iter().zip(&rows) {
        if count == 0 {
            continue;
        }
        let count_f = count as f64;
        let plies = if last == 81 {
            format!("{first}+")
        } else {
            format!("{first}-{last}")
        };
        let _ = writeln!(
            table,
            "| {plies} | {count} | {:.3} | {:.1}% | {:.1}% |",
            distance / count_f,
            100.0 * leaning as f64 / count_f,
            100.0 * decided as f64 / count_f
        );
    }
    let games = first_player.iter().sum::<usize>();
    if games > 0 {
        let share = |count: usize| 100.0 * count as f64 / games as f64;
        let _ = writeln!(
            table,
            "\nThe first player won {:.1}% of the {games} games, drew {:.1}% and lost {:.1}%.",
            share(first_player[0]),
            share(first_player[1]),
            share(first_player[2])
        );
    }
    let counts: Vec<String> = DISTINCT_PLIES
        .iter()
        .zip(&distinct)
        .filter(|(_, (count, _))| *count > 0)
        .map(|(ply, (count, set))| format!("at ply {ply}, {} of {count}", set.len()))
        .collect();
    if !counts.is_empty() {
        let _ = writeln!(table, "\nDistinct positions: {}.", counts.join("; "));
    }
    table
}

/// What a quantized network makes of held-out positions, beyond its
/// error: its error and its agreement with the move played, by ply; its
/// calibration; and its first layer's units that never vary.
pub struct Assessment {
    /// For each row of `PLY_ROWS`: positions, the sum of squared errors,
    /// and how many of the moves played the network would choose.
    pub phases: [(usize, f64, usize); PLY_ROWS.len()],
    /// For each tenth of the predicted score: positions, the sum of the
    /// predictions and the sum of the results.
    pub bins: [(usize, f64, f64); 10],
    /// First-layer units at 0 or below on every position, from both seats:
    /// they add nothing and learn nothing.
    pub dead: usize,
    /// Units at full scale or above on every position: a constant.
    pub constant: usize,
    pub width: usize,
}

/// The move `network` would choose from `board`: the child best for the
/// side to move, a move that wins the game first.
fn network_move<const M: usize>(network: &Network<M>, board: &Board) -> Option<uttt_engine::Move> {
    let mut moves = MoveList::new();
    board.legal_moves(&mut moves);
    let mover = board.to_move();
    let value = |mv: uttt_engine::Move| {
        let mut child = *board;
        child.play(mv);
        match child.status() {
            Status::Win(seat) if seat == mover => 1.0,
            Status::Win(_) => 0.0,
            Status::Draw => 0.5,
            Status::Ongoing => 1.0 - network.evaluate_board(&child),
        }
    };
    let mut best = None;
    for &mv in moves.iter() {
        let score = value(mv);
        if best.is_none_or(|(_, best_score)| score > best_score) {
            best = Some((mv, score));
        }
    }
    best.map(|(mv, _)| mv)
}

/// Assesses `network` on the positions `indices` of `positions`.
pub fn assess<const M: usize>(
    network: &Network<M>,
    positions: &Positions,
    indices: &[usize],
) -> Assessment {
    let mut phases = [(0, 0.0, 0); PLY_ROWS.len()];
    let mut bins = [(0, 0.0, 0.0); 10];
    // Each unit's highest and lowest value over the positions and seats.
    let mut highest = [i16::MIN; M];
    let mut lowest = [i16::MAX; M];
    for &index in indices {
        let example = positions.get(index);
        let board = &example.board;
        let accumulator = network.refresh(board);
        for values in &accumulator.values {
            for ((high, low), &value) in highest.iter_mut().zip(&mut lowest).zip(values) {
                *high = (*high).max(value);
                *low = (*low).min(value);
            }
        }
        let predicted = network.evaluate(&accumulator, board);
        let ply = positions.ply(index);
        if let Some(row) = PLY_ROWS
            .iter()
            .position(|&(first, last)| (first..=last).contains(&ply))
        {
            let phase = &mut phases[row];
            phase.0 += 1;
            phase.1 += f64::from(predicted - example.result).powi(2);
            let played = positions.next_move(index);
            phase.2 += usize::from(played.is_some() && network_move(network, board) == played);
        }
        let bin = &mut bins[((predicted * 10.0) as usize).min(9)];
        bin.0 += 1;
        bin.1 += f64::from(predicted);
        bin.2 += f64::from(example.result);
    }
    let any = !indices.is_empty();
    Assessment {
        phases,
        bins,
        dead: highest.iter().filter(|&&high| any && high <= 0).count(),
        constant: lowest
            .iter()
            .filter(|&&low| any && i32::from(low) >= QA)
            .count(),
        width: M,
    }
}

impl Assessment {
    /// The average gap between the predictions and the results over the
    /// tenths of the predicted score, weighted by their positions.
    pub fn calibration_error(&self) -> f64 {
        let total: usize = self.bins.iter().map(|bin| bin.0).sum();
        let gaps: f64 = self.bins.iter().map(|bin| (bin.1 - bin.2).abs()).sum();
        gaps / total.max(1) as f64
    }

    /// The share of moves played that the network would choose.
    pub fn agreement(&self) -> f64 {
        let (count, agreed) = self.phases.iter().fold((0, 0), |(count, agreed), phase| {
            (count + phase.0, agreed + phase.2)
        });
        agreed as f64 / count.max(1) as f64
    }
}

/// The report's sections on `assessments`, named networks assessed on the
/// same positions.
pub fn assessment_tables(assessments: &[(&str, &Assessment)]) -> String {
    let mut table = String::new();
    let names: Vec<&str> = assessments.iter().map(|(name, _)| *name).collect();
    let _ = writeln!(
        table,
        "By ply (moves played before the position): squared error against the result, and the share of the moves played that the network would choose, the child it scores best for the side to move. Openings' moves were drawn, not chosen, when self-play sampled them.\n"
    );
    let mut header = "| Plies | Positions |".to_string();
    let mut rule = "| --- | --- |".to_string();
    for name in &names {
        let _ = write!(header, " {name}: squared error | {name}: agreement |");
        rule.push_str(" --- | --- |");
    }
    let _ = writeln!(table, "{header}\n{rule}");
    for (row, &(first, last)) in PLY_ROWS.iter().enumerate() {
        let count = assessments.first().map_or(0, |(_, a)| a.phases[row].0);
        if count == 0 {
            continue;
        }
        let plies = if last == 81 {
            format!("{first}+")
        } else {
            format!("{first}-{last}")
        };
        let _ = write!(table, "| {plies} | {count} |");
        for (_, assessment) in assessments {
            let (count, squared, agreed) = assessment.phases[row];
            let count = count.max(1) as f64;
            let _ = write!(
                table,
                " {:.4} | {:.1}% |",
                squared / count,
                100.0 * agreed as f64 / count
            );
        }
        let _ = writeln!(table);
    }
    let _ = write!(table, "| All | |");
    for (_, assessment) in assessments {
        let (count, squared) = assessment
            .phases
            .iter()
            .fold((0, 0.0), |(count, squared), phase| {
                (count + phase.0, squared + phase.1)
            });
        let _ = write!(
            table,
            " {:.4} | {:.1}% |",
            squared / count.max(1) as f64,
            100.0 * assessment.agreement()
        );
    }
    let _ = writeln!(table);

    let _ = writeln!(
        table,
        "\nCalibration: the positions by predicted score, with the average prediction and the average result.\n"
    );
    let mut header = "| Predicted |".to_string();
    let mut rule = "| --- |".to_string();
    for name in &names {
        let _ = write!(header, " {name}: positions | predicted | result |");
        rule.push_str(" --- | --- | --- |");
    }
    let _ = writeln!(table, "{header}\n{rule}");
    for bin in 0..10 {
        let _ = write!(
            table,
            "| {:.1}-{:.1} |",
            bin as f64 / 10.0,
            (bin + 1) as f64 / 10.0
        );
        for (_, assessment) in assessments {
            let (count, predicted, result) = assessment.bins[bin];
            if count == 0 {
                let _ = write!(table, " 0 | | |");
            } else {
                let _ = write!(
                    table,
                    " {count} | {:.3} | {:.3} |",
                    predicted / count as f64,
                    result / count as f64
                );
            }
        }
        let _ = writeln!(table);
    }
    let gaps: Vec<String> = assessments
        .iter()
        .map(|(name, assessment)| format!("{name} {:.4}", assessment.calibration_error()))
        .collect();
    let _ = writeln!(
        table,
        "\nAverage gap between prediction and result, weighted by positions: {}.",
        gaps.join("; ")
    );
    let units: Vec<String> = assessments
        .iter()
        .map(|(name, assessment)| {
            format!(
                "{name}, {} dead (never above 0) and {} constant (always at full scale) of {}",
                assessment.dead, assessment.constant, assessment.width
            )
        })
        .collect();
    let _ = writeln!(
        table,
        "\nFirst-layer units over these positions, from both seats: {}.",
        units.join("; ")
    );
    table
}

/// The two rows of a table of metrics.
fn row(report: &mut String, name: &str, metrics: &Metrics) {
    let _ = writeln!(
        report,
        "| {name} | {:.4} | {:.4} |",
        metrics.squared_error, metrics.cross_entropy
    );
}

fn fit_sized<const N: usize>(options: &Options) -> Result<(), String> {
    let start = Instant::now();
    // File by file: only the compact positions stay in memory.
    let mut split = value_fit::Split::distinct();
    for path in &options.data {
        split.add(&read_file(path)?);
    }
    let game_count = split.games;
    let (fitted, held_out) = (&split.fitted, &split.held_out);
    if fitted.len() == 0 || held_out.len() == 0 {
        return Err(format!(
            "{} positions to fit and {} held out: not enough games",
            fitted.len(),
            held_out.len()
        ));
    }
    let settings = &options.training;
    let threads = settings.threads;
    eprintln!(
        "{game_count} games, {} dropped as copies of an earlier one: {} positions to fit, {} held out",
        split.duplicates,
        fitted.len(),
        held_out.len()
    );

    let initial = initial_parameters::<N>(settings.seed);
    let initial_metrics = measure::<N>(&initial, held_out, threads);
    let mut epochs = Vec::new();
    let parameters = train::<N>(initial, fitted, held_out, settings, |epoch| {
        eprintln!(
            "epoch {}: fitted {:.4}, held out {:.4} nats, squared error {:.4}, {:.0} s",
            epoch.number,
            epoch.fitted,
            epoch.held_out.cross_entropy,
            epoch.held_out.squared_error,
            start.elapsed().as_secs_f64()
        );
        epochs.push(*epoch);
    });
    if parameters.iter().any(|value| !value.is_finite()) {
        return Err("training diverged: a parameter is not finite".to_string());
    }
    let network = quantize::<N>(&parameters)?;
    let bytes = network.to_bytes();
    let source = format!(
        "{}, accumulator width {N}, `uttt-trainer fit-nnue`",
        options.origin
    );
    let module = cg_packer::rust_module(&bytes, "NNUE_WEIGHTS", &source)?;
    let characters = cg_packer::pack(&bytes).chars().count();
    if let Some(path) = &options.weights_out {
        std::fs::write(path, &module).map_err(|err| format!("{}: {err}", path.display()))?;
    }

    let compared = value_fit::spread(held_out, options.compared_positions);
    let average = (0..fitted.len())
        .map(|index| f64::from(fitted.result(index)))
        .sum::<f64>()
        / fitted.len() as f64;
    let constant = constant_metrics(average, (0..held_out.len()).map(|i| held_out.result(i)));
    let float = measure::<N>(&parameters, held_out, threads);
    let quantized = measure_quantized(&network, held_out, threads);
    let constant_compared = constant_metrics(average, compared.iter().map(|e| e.result));
    let float_compared = measure::<N>(&parameters, &compared[..], threads);
    let quantized_compared = measure_quantized(&network, &compared[..], threads);
    let other = options.compare.as_ref().map(|(_, other)| {
        (
            measure_quantized(other, held_out, threads),
            measure_quantized(other, &compared[..], threads),
        )
    });
    // The same positions as `compared`.
    let step = held_out
        .len()
        .div_ceil(options.compared_positions.max(1))
        .max(1);
    let indices: Vec<usize> = (0..held_out.len()).step_by(step).collect();
    let assessment = assess(&network, held_out, &indices);
    let other_assessment = options
        .compare
        .as_ref()
        .map(|(_, other)| assess(other, held_out, &indices));

    let trained = epochs.last().map_or(0.0, |epoch| epoch.training_seconds);
    let shown = fitted.len() as f64 * f64::from(settings.epochs);
    let mut report = String::new();
    let _ = writeln!(report, "# NNUE fit\n");
    let _ = writeln!(report, "- Data: {}", options.origin);
    let _ = writeln!(
        report,
        "- Games: {game_count}, of which {} dropped as copies of an earlier game (the same moves); positions: {} fitted, {} held out (every 20th game kept, whole)",
        split.duplicates,
        fitted.len(),
        held_out.len()
    );
    let _ = writeln!(
        report,
        "- Network: ({FEATURES} -> {N}) x 2 -> {BUCKETS} buckets, {} parameters; quantized as in uttt_engine::nnue, {} bytes packed in {characters} characters",
        Shape::<N>::PARAMETERS,
        bytes.len()
    );
    let _ = writeln!(
        report,
        "- Training: target {:?}, {} epochs, batches of {}, Adam from {} falling to a twentieth, seed {}, {} threads, {:.0} s in all",
        settings.target,
        settings.epochs,
        settings.batch,
        settings.rate,
        settings.seed,
        threads,
        start.elapsed().as_secs_f64()
    );
    if let Some((path, _)) = &options.compare {
        let _ = writeln!(report, "- Compared network: `{path}`, width 256");
    }
    if trained > 0.0 {
        let _ = writeln!(
            report,
            "- Throughput: {:.0} positions per second of training ({shown:.0} positions in {trained:.0} s, without the measurements)",
            shown / trained
        );
    }
    let _ = writeln!(report, "\n## Held out, all {} positions\n", held_out.len());
    let _ = writeln!(
        report,
        "| Predictor | Squared error | Cross-entropy, nats |"
    );
    let _ = writeln!(report, "| --- | --- | --- |");
    row(
        &mut report,
        &format!("The average result of fitted games, {average:.3}"),
        &constant,
    );
    row(&mut report, "Network, float", &float);
    row(&mut report, "Network, quantized", &quantized);
    if let Some((metrics, _)) = &other {
        row(&mut report, "Compared network, quantized", metrics);
    }
    let _ = writeln!(
        report,
        "\n## Held out, {} positions spread evenly\n",
        compared.len()
    );
    let _ = writeln!(
        report,
        "| Predictor | Squared error | Cross-entropy, nats |"
    );
    let _ = writeln!(report, "| --- | --- | --- |");
    row(
        &mut report,
        &format!("The average result of fitted games, {average:.3}"),
        &constant_compared,
    );
    row(&mut report, "Network, float", &float_compared);
    row(&mut report, "Network, quantized", &quantized_compared);
    if let Some((_, metrics)) = &other {
        row(&mut report, "Compared network, quantized", metrics);
    }
    let _ = writeln!(
        report,
        "\n## Held out, {} positions spread evenly, in detail\n\nThe quantized networks, as the bot plays them.\n",
        compared.len()
    );
    let mut assessments = vec![("Network", &assessment)];
    if let Some(other) = &other_assessment {
        assessments.push(("Compared", other));
    }
    let _ = write!(report, "{}", assessment_tables(&assessments));
    let _ = writeln!(report, "\n## Training\n");
    let _ = writeln!(
        report,
        "| Epoch | Fitted positions, cross-entropy | Held out, cross-entropy | Held out, squared error |"
    );
    let _ = writeln!(report, "| --- | --- | --- | --- |");
    let _ = writeln!(
        report,
        "| Start | | {:.4} | {:.4} |",
        initial_metrics.cross_entropy, initial_metrics.squared_error
    );
    for epoch in &epochs {
        let _ = writeln!(
            report,
            "| {} | {:.4} | {:.4} | {:.4} |",
            epoch.number, epoch.fitted, epoch.held_out.cross_entropy, epoch.held_out.squared_error
        );
    }
    let _ = writeln!(
        report,
        "| Quantized | | {:.4} | {:.4} |",
        quantized.cross_entropy, quantized.squared_error
    );
    let _ = writeln!(
        report,
        "\n## Data\n\nThe fitted positions by ply (moves played before them), with the search's root score for the side to move.\n"
    );
    let _ = write!(report, "{}", data_table(fitted));
    if let Some(path) = &options.report_out {
        std::fs::write(path, &report).map_err(|err| format!("{}: {err}", path.display()))?;
    }
    print!("{report}");
    Ok(())
}

#[cfg(test)]
mod tests;
