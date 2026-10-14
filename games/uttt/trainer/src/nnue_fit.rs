//! Trains the NNUE of `uttt_engine::nnue` on self-play games: 189 inputs per
//! perspective, an accumulator of `N` units shared by both seats, and 10
//! output buckets, then quantizes it as the engine's integer arithmetic
//! expects.
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
use std::time::Instant;

use cg_core::rng::Rng;
use uttt_engine::nnue::{
    self, Network, BUCKETS, FEATURES, MAX_ACTIVE, MAX_BIAS, OUR_BOARDS, QA, QB, THEIR_CELLS,
};
use uttt_engine::Board;

use crate::value_fit::{
    self, cross_entropy, shares, sigmoid, symmetric_cell, Examples, Metrics, Training,
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
        let buckets = std::array::from_fn(|bucket| if bucket == 9 { 9 } else { map(bucket) as u8 });
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
/// biases to a magnitude of 1,000.
pub fn clip<const N: usize>(parameters: &mut [f32]) {
    let (weights, rest) = parameters.split_at_mut(Shape::<N>::BIAS);
    let (bias, rest) = rest.split_at_mut(N);
    let (output, output_bias) = rest.split_at_mut(BUCKETS * 2 * N);
    let weight_limit = 127.0 / QA as f32;
    let bias_limit = f32::from(MAX_BIAS) / QA as f32;
    let output_limit = 127.0 / QB as f32;
    for value in weights {
        *value = value.clamp(-weight_limit, weight_limit);
    }
    for value in bias {
        *value = value.clamp(-bias_limit, bias_limit);
    }
    for value in output {
        *value = value.clamp(-output_limit, output_limit);
    }
    for value in output_bias {
        *value = value.clamp(-1000.0, 1000.0);
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

/// Trains from `parameters` on `fitted`, measuring on `held_out` after each
/// epoch, and calls `report` then. `settings.penalty` weighs an L2 penalty
/// as in `value_fit`.
pub fn train<const N: usize>(
    mut parameters: Vec<f32>,
    fitted: &(impl Examples + ?Sized),
    held_out: &(impl Examples + ?Sized),
    settings: &Training,
    mut report: impl FnMut(&Epoch),
) -> Vec<f32> {
    let symmetries = symmetries();
    let threads = settings.threads.max(1);
    let mut rng = Rng::new(settings.seed);
    let count = Shape::<N>::PARAMETERS;
    let (mut first, mut second) = (vec![0.0f32; count], vec![0.0f32; count]);
    let (beta1, beta2, epsilon) = (0.9f32, 0.999f32, 1e-8f32);
    let steps_per_epoch = fitted.len().div_ceil(settings.batch);
    let total_steps = (steps_per_epoch * settings.epochs as usize).max(1);
    let mut step: u32 = 0;
    let mut gradients = vec![vec![0.0f32; count]; threads];
    let mut order: Vec<u32> = (0..fitted.len() as u32).collect();
    let mut training_seconds = 0.0;
    for number in 1..=settings.epochs {
        let start = Instant::now();
        rng.shuffle(&mut order);
        let mut epoch_loss = 0.0;
        for batch in order.chunks(settings.batch) {
            let views: Vec<(u32, u8)> = batch
                .iter()
                .map(|&index| (index, rng.below(8) as u8))
                .collect();
            let part = views.len().div_ceil(threads);
            let losses: Vec<f64> = std::thread::scope(|scope| {
                let parameters = &parameters;
                let symmetries = &symmetries;
                let handles: Vec<_> = gradients
                    .iter_mut()
                    .zip(views.chunks(part))
                    .map(|(gradient, chunk)| {
                        scope.spawn(move || {
                            gradient.fill(0.0);
                            let mut loss = 0.0;
                            for &(index, symmetry) in chunk {
                                let example = fitted.get(index as usize);
                                let view =
                                    View::new(&example.board, &symmetries[usize::from(symmetry)]);
                                loss += accumulate::<N>(
                                    parameters,
                                    &view,
                                    settings.target.of(&example),
                                    gradient,
                                );
                            }
                            loss
                        })
                    })
                    .collect();
                handles
                    .into_iter()
                    .map(|handle| handle.join().expect("a training thread panicked"))
                    .collect()
            });
            epoch_loss += losses.iter().sum::<f64>();
            let used = views.len().div_ceil(part);
            let (total, rest) = gradients.split_at_mut(1);
            for other in &rest[..used - 1] {
                for (sum, value) in total[0].iter_mut().zip(other) {
                    *sum += value;
                }
            }

            step += 1;
            let progress = step as f32 / total_steps as f32;
            let rate = settings.rate
                * (0.05 + 0.95 * 0.5 * (1.0 + (std::f32::consts::PI * progress).cos()));
            let scale = 1.0 / views.len() as f32;
            let (correction1, correction2) =
                (1.0 - beta1.powi(step as i32), 1.0 - beta2.powi(step as i32));
            for ((value, gradient), (m, v)) in parameters
                .iter_mut()
                .zip(&total[0])
                .zip(first.iter_mut().zip(second.iter_mut()))
            {
                let g = gradient * scale + settings.penalty * *value;
                *m = beta1 * *m + (1.0 - beta1) * g;
                *v = beta2 * *v + (1.0 - beta2) * g * g;
                *value -= rate * (*m / correction1) / ((*v / correction2).sqrt() + epsilon);
            }
            clip::<N>(&mut parameters);
        }
        training_seconds += start.elapsed().as_secs_f64();
        report(&Epoch {
            number,
            fitted: epoch_loss / fitted.len().max(1) as f64,
            held_out: measure::<N>(&parameters, held_out, threads),
            training_seconds,
        });
    }
    parameters
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
    let mut split = value_fit::Split::default();
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
        "{game_count} games: {} positions to fit, {} held out",
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

    let trained = epochs.last().map_or(0.0, |epoch| epoch.training_seconds);
    let shown = fitted.len() as f64 * f64::from(settings.epochs);
    let mut report = String::new();
    let _ = writeln!(report, "# NNUE fit\n");
    let _ = writeln!(report, "- Data: {}", options.origin);
    let _ = writeln!(
        report,
        "- Games: {game_count}; positions: {} fitted, {} held out (every 20th game, whole)",
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
    if let Some(path) = &options.report_out {
        std::fs::write(path, &report).map_err(|err| format!("{}: {err}", path.display()))?;
    }
    print!("{report}");
    Ok(())
}

#[cfg(test)]
mod tests;
