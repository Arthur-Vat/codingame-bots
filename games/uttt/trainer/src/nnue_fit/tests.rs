use uttt_engine::{Move, Status};

use super::*;
use crate::data::{GameRecord, Searched};
use crate::value_fit::{Split, Target};

/// Every position of `games` random games, from seed `seed`.
fn random_positions(games: u64, seed: u64) -> Vec<Board> {
    let mut rng = Rng::new(seed);
    let mut positions = Vec::new();
    for _ in 0..games {
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            positions.push(board);
            board.play(board.random_move(&mut rng));
        }
    }
    positions
}

/// A random game, with every position searched.
fn random_record(seed: u64) -> GameRecord {
    let mut rng = Rng::new(seed);
    let mut board = Board::new();
    let mut record = GameRecord::default();
    while board.status() == Status::Ongoing {
        record.searched.push(Searched {
            ply: record.moves.len() as u8,
            score: Some(rng.unit() as f32),
            visits: Vec::new(),
        });
        let mv = board.random_move(&mut rng);
        record.moves.push(mv);
        board.play(mv);
    }
    record
}

/// Parameters of a width-`N` network, each group uniform in its own range.
fn random_parameters<const N: usize>(seed: u64, weight: f64, output: f64) -> Vec<f32> {
    let mut rng = Rng::new(seed);
    let mut uniform = |spread: f64| ((rng.unit() * 2.0 - 1.0) * spread) as f32;
    let mut parameters = Vec::new();
    parameters.extend((0..FEATURES * N).map(|_| uniform(weight)));
    parameters.extend((0..N).map(|_| 0.5 + uniform(0.1)));
    parameters.extend((0..BUCKETS * 2 * N).map(|_| uniform(output)));
    parameters.extend((0..BUCKETS).map(|_| uniform(0.2)));
    assert_eq!(parameters.len(), Shape::<N>::PARAMETERS);
    parameters
}

/// The cross-entropy of the model in `f64`, and how close an accumulator
/// came to a clipping point (0 or 1).
fn reference<const N: usize>(parameters: &[f64], view: &View, target: f64) -> (f64, f64) {
    let bias = &parameters[Shape::<N>::BIAS..][..N];
    let output = Shape::<N>::OUTPUT + view.bucket * 2 * N;
    let mut logit = parameters[Shape::<N>::OUTPUT_BIAS + view.bucket];
    let mut margin = f64::MAX;
    for side in 0..2 {
        for (unit, &b) in bias.iter().enumerate() {
            let mut acc = b;
            for &feature in view.active(side) {
                acc += parameters[usize::from(feature) * N + unit];
            }
            margin = margin.min(acc.abs()).min((acc - 1.0).abs());
            let a = acc.clamp(0.0, 1.0);
            logit += a * a * parameters[output + side * N + unit];
        }
    }
    let loss = logit.max(0.0) - logit * target + (-logit.abs()).exp().ln_1p();
    (loss, margin)
}

#[test]
fn the_layout_adds_up() {
    assert_eq!(
        Shape::<8>::PARAMETERS,
        FEATURES * 8 + 8 + BUCKETS * 16 + BUCKETS
    );
    assert_eq!(initial_parameters::<8>(1).len(), Shape::<8>::PARAMETERS);
    assert_eq!(initial_parameters::<256>(1).len(), Shape::<256>::PARAMETERS);
    // A quantized network of the same width has as many values as the
    // trainer has parameters.
    assert_eq!(
        Network::<8>::BYTES,
        8 + 4 + FEATURES * 8 + 2 * 8 + BUCKETS * 16 + 4 * BUCKETS
    );
}

#[test]
fn gradients_match_finite_differences() {
    const N: usize = 8;
    let parameters = random_parameters::<N>(3, 0.05, 0.6);
    let wide: Vec<f64> = parameters.iter().map(|&p| f64::from(p)).collect();
    let symmetries = symmetries();
    let boards = random_positions(8, 4);
    let mut views = Vec::new();
    for (index, board) in boards.iter().enumerate().step_by(5) {
        let view = View::new(board, &symmetries[index % 8]);
        // Accumulators away from the clipping points, where the loss bends.
        if reference::<N>(&wide, &view, 0.0).1 > 1e-3 {
            views.push((view, [0.0, 0.5, 1.0, 0.3][index % 4]));
        }
    }
    assert!(views.len() > 20, "only {} positions kept", views.len());

    let mut gradient = vec![0.0f32; Shape::<N>::PARAMETERS];
    let mut loss = 0.0;
    for (view, target) in &views {
        loss += accumulate::<N>(&parameters, view, *target, &mut gradient);
    }
    let total = |moved: &[f64]| {
        views
            .iter()
            .map(|(view, target)| reference::<N>(moved, view, f64::from(*target)).0)
            .sum::<f64>()
    };
    assert!(
        (loss - total(&wide)).abs() < 1e-3,
        "{loss} against {}",
        total(&wide)
    );

    // Parameters of every group: all biases, output weights and output
    // biases, and every 7th first-layer weight.
    let mut probes: Vec<usize> = (Shape::<N>::BIAS..Shape::<N>::PARAMETERS).collect();
    probes.extend((0..Shape::<N>::BIAS).step_by(7));
    let step = 1e-6;
    let mut nonzero = [0usize; 4];
    for probe in probes {
        let mut moved = wide.clone();
        moved[probe] += step;
        let up = total(&moved);
        moved[probe] -= 2.0 * step;
        let down = total(&moved);
        let numeric = (up - down) / (2.0 * step);
        let analytic = f64::from(gradient[probe]);
        assert!(
            (numeric - analytic).abs() < 1e-4 + 1e-3 * numeric.abs(),
            "parameter {probe}: {analytic} against {numeric}"
        );
        if analytic != 0.0 {
            let group = match probe {
                p if p < Shape::<N>::BIAS => 0,
                p if p < Shape::<N>::OUTPUT => 1,
                p if p < Shape::<N>::OUTPUT_BIAS => 2,
                _ => 3,
            };
            nonzero[group] += 1;
        }
    }
    // Each group was checked where its gradient is not zero.
    assert!(nonzero.iter().all(|&count| count >= 8), "{nonzero:?}");
}

#[test]
fn gradients_stop_at_clipped_accumulators() {
    const N: usize = 8;
    // All accumulators above 1: nothing flows back to the first layer.
    let mut parameters = random_parameters::<N>(5, 0.05, 0.6);
    for value in &mut parameters[Shape::<N>::BIAS..][..N] {
        *value = 5.0;
    }
    let board = Board::new();
    let view = View::new(&board, &Symmetry::new(0));
    let mut gradient = vec![0.0f32; Shape::<N>::PARAMETERS];
    accumulate::<N>(&parameters, &view, 1.0, &mut gradient);
    assert!(gradient[..Shape::<N>::OUTPUT].iter().all(|&g| g == 0.0));
    assert!(gradient[Shape::<N>::OUTPUT..].iter().any(|&g| g != 0.0));
}

#[test]
fn symmetries_map_features_and_buckets_as_in_the_symmetric_game() {
    let symmetries = symmetries();
    assert_eq!(symmetries.len(), 8);
    let identity = &symmetries[0];
    assert!(identity
        .features
        .iter()
        .enumerate()
        .all(|(index, &image)| usize::from(image) == index));
    assert!(identity
        .buckets
        .iter()
        .enumerate()
        .all(|(index, &image)| usize::from(image) == index));
    for symmetry in &symmetries {
        let mut images = symmetry.features.to_vec();
        images.sort_unstable();
        assert!(images
            .iter()
            .enumerate()
            .all(|(index, &image)| usize::from(image) == index));
        let mut buckets = symmetry.buckets.to_vec();
        buckets.sort_unstable();
        assert!(buckets
            .iter()
            .enumerate()
            .all(|(index, &image)| usize::from(image) == index));
    }
    let mut rng = Rng::new(6);
    let mut checked = 0;
    for game in 0..40 {
        let index = game % 8;
        let symmetry = &symmetries[index];
        let mut board = Board::new();
        let mut image = Board::new();
        while board.status() == Status::Ongoing {
            assert_eq!(image.to_move(), board.to_move());
            let view = View::new(&board, symmetry);
            let mapped = View::new(&image, identity);
            // The mapped features of the original, in each perspective.
            for side in 0..2 {
                let mut ours = view.active(side).to_vec();
                ours.sort_unstable();
                let mut theirs = mapped.active(side).to_vec();
                theirs.sort_unstable();
                assert_eq!(ours, theirs, "symmetry {index}, side {side}");
            }
            // The unmapped features, mapped here.
            let mut raw = [0u16; MAX_ACTIVE];
            let len = nnue::active_features(&board, board.to_move(), &mut raw);
            let mut by_hand: Vec<u16> = raw[..len]
                .iter()
                .map(|&f| symmetry.features[usize::from(f)])
                .collect();
            by_hand.sort_unstable();
            let mut ours = view.active(0).to_vec();
            ours.sort_unstable();
            assert_eq!(by_hand, ours);
            assert_eq!(view.bucket, mapped.bucket, "symmetry {index}");
            let mv = board.random_move(&mut rng);
            let mapped_move = Move::new(
                symmetric_cell(index, mv.board()),
                symmetric_cell(index, mv.cell()),
            );
            let mut legal = uttt_engine::moves::MoveList::new();
            image.legal_moves(&mut legal);
            assert!(legal.contains(&mapped_move));
            board.play(mv);
            image.play(mapped_move);
            assert_eq!(
                board.status() == Status::Ongoing,
                image.status() == Status::Ongoing
            );
            checked += 1;
        }
    }
    assert!(checked > 1000);
}

/// Asserts that the float network `parameters` and its quantization give
/// logits within `tolerance` of each other on random positions.
fn assert_quantization_error_below<const N: usize>(parameters: &[f32], tolerance: f32) {
    let network = quantize::<N>(parameters).unwrap();
    let identity = Symmetry::new(0);
    let mut worst = 0.0f32;
    for board in random_positions(10, 8).iter().step_by(3) {
        let float = forward::<N>(parameters, &View::new(board, &identity)).logit;
        let engine = network.logit(&network.refresh(board), board);
        worst = worst.max((float - engine).abs());
    }
    assert!(worst < tolerance, "width {N}: logits differ by {worst}");
}

/// `parameters` rounded to the quantization's grid, as floats.
fn on_the_grid<const N: usize>(parameters: &[f32]) -> Vec<f32> {
    let round = |value: f32, scale: f32| (value * scale).round() / scale;
    let (qa, qb) = (QA as f32, QB as f32);
    parameters
        .iter()
        .enumerate()
        .map(|(index, &value)| match index {
            i if i < Shape::<N>::OUTPUT => round(value, qa),
            i if i < Shape::<N>::OUTPUT_BIAS => round(value, qb),
            _ => round(value, qa * qb),
        })
        .collect()
}

#[test]
fn the_quantized_network_computes_what_the_float_network_does() {
    // Parameters already on the grid are held exactly: the logits differ
    // only by the engine's truncation (at most 1 / (QA * QB)) and f32 sums.
    assert_quantization_error_below::<16>(&on_the_grid::<16>(&initial_parameters::<16>(1)), 1e-3);
    assert_quantization_error_below::<128>(
        &on_the_grid::<128>(&initial_parameters::<128>(2)),
        1e-3,
    );
    assert_quantization_error_below::<32>(
        &on_the_grid::<32>(&random_parameters::<32>(3, 0.3, 0.3)),
        1e-3,
    );
}

#[test]
fn rounding_to_the_grid_moves_logits_by_a_small_stated_amount() {
    // At the starting scale, the output weights (about 0.06) are a few
    // steps of 1 / QB, which is the main source of error: below 0.15 in
    // logit, about 4% in probability.
    assert_quantization_error_below::<16>(&initial_parameters::<16>(1), 0.15);
    assert_quantization_error_below::<128>(&initial_parameters::<128>(2), 0.15);
    // Larger first-layer weights, some accumulators clipped.
    assert_quantization_error_below::<32>(&random_parameters::<32>(3, 0.3, 0.3), 0.15);
}

#[test]
fn quantizing_clamps_and_rounds_as_the_engine_expects() {
    const N: usize = 8;
    let mut parameters = vec![0.0f32; Shape::<N>::PARAMETERS];
    parameters[0] = 0.5;
    parameters[1] = -0.5;
    parameters[2] = 3.0;
    parameters[Shape::<N>::BIAS] = 1000.0;
    parameters[Shape::<N>::BIAS + 1] = -0.25;
    parameters[Shape::<N>::OUTPUT] = 0.5;
    parameters[Shape::<N>::OUTPUT + 1] = -9.0;
    parameters[Shape::<N>::OUTPUT_BIAS + 3] = 0.01;
    parameters[Shape::<N>::OUTPUT_BIAS + 4] = -2000.0;
    let bytes = quantize::<N>(&parameters).unwrap().to_bytes();
    let weights = &bytes[12..];
    assert_eq!(weights[0] as i8, 64); // round(127 * 0.5)
    assert_eq!(weights[1] as i8, -64);
    assert_eq!(weights[2] as i8, 127);
    let biases = &bytes[12 + FEATURES * N..];
    assert_eq!(i16::from_le_bytes([biases[0], biases[1]]), MAX_BIAS);
    assert_eq!(i16::from_le_bytes([biases[2], biases[3]]), -32); // round(127 * -0.25)
    let output = &bytes[12 + FEATURES * N + 2 * N..];
    assert_eq!(output[0] as i8, 32); // 64 * 0.5
    assert_eq!(output[1] as i8, -127);
    let at = 12 + FEATURES * N + 2 * N + BUCKETS * 2 * N;
    let output_bias = |k: usize| {
        let b = &bytes[at + 4 * k..];
        i32::from_le_bytes([b[0], b[1], b[2], b[3]])
    };
    assert_eq!(output_bias(3), 81); // round(127 * 64 * 0.01)
    assert_eq!(output_bias(4), -127 * 64 * 2000);
    // The clip keeps parameters where quantization is exact enough.
    clip::<N>(0, &mut parameters);
    assert_eq!(parameters[2], 1.0);
    assert_eq!(parameters[Shape::<N>::OUTPUT_BIAS + 4], -1000.0);
    assert_eq!(parameters[Shape::<N>::OUTPUT + 1], -127.0 / 64.0);
    assert_eq!(
        parameters[Shape::<N>::BIAS],
        f32::from(MAX_BIAS) / QA as f32
    );
}

#[test]
fn the_written_module_decodes_to_the_networks_bytes() {
    const N: usize = 16;
    let parameters = random_parameters::<N>(9, 0.2, 0.5);
    let network = quantize::<N>(&parameters).unwrap();
    let bytes = network.to_bytes();
    let module = cg_packer::rust_module(&bytes, "NNUE_WEIGHTS", "a test").unwrap();
    let start = module.find("NNUE_WEIGHTS: &str = \"").unwrap() + "NNUE_WEIGHTS: &str = \"".len();
    let end = module.rfind("\";").unwrap();
    let decoded = cg_pack::unpack(&module[start..end]).expect("the module's text unpacks");
    assert_eq!(decoded, bytes);
    assert_eq!(Network::<N>::from_bytes(&decoded).unwrap(), network);
}

#[test]
fn clipping_keeps_every_quantization_valid() {
    const N: usize = 8;
    let mut parameters = random_parameters::<N>(11, 50.0, 50.0);
    for value in &mut parameters[Shape::<N>::BIAS..][..N] {
        *value *= 1000.0;
    }
    clip::<N>(0, &mut parameters);
    assert!(quantize::<N>(&parameters).is_ok());
}

#[test]
fn training_beats_the_constant_predictor_and_stays_finite() {
    const N: usize = 16;
    let games: Vec<GameRecord> = (0..400).map(random_record).collect();
    let mut split = Split::default();
    split.add(&games);
    let (fitted, held_out) = (&split.fitted, &split.held_out);
    assert!(fitted.len() > 5000 && held_out.len() > 100);
    let average = (0..fitted.len())
        .map(|index| f64::from(fitted.result(index)))
        .sum::<f64>()
        / fitted.len() as f64;
    let constant = constant_metrics(average, (0..held_out.len()).map(|i| held_out.result(i)));
    let start = initial_parameters::<N>(12);
    let settings = Training {
        epochs: 6,
        batch: 64,
        rate: 0.003,
        penalty: 0.0,
        target: Target::Result,
        threads: 2,
        seed: 13,
    };
    let mut epochs = Vec::new();
    let trained = train::<N>(start, fitted, held_out, &settings, |epoch| {
        epochs.push(*epoch)
    });
    assert_eq!(epochs.len(), 6);
    assert!(trained.iter().all(|value| value.is_finite()));
    assert!(epochs.iter().all(|e| e.fitted.is_finite()));
    let float = measure::<N>(&trained, held_out, 2);
    assert_eq!(epochs[5].held_out, float);
    assert!(
        float.cross_entropy < constant.cross_entropy - 0.02,
        "{float:?} against {constant:?}"
    );
    assert!(float.squared_error < constant.squared_error);
    // The quantized network, in the engine's arithmetic, is as good.
    let network = quantize::<N>(&trained).unwrap();
    let quantized = measure_quantized(&network, held_out, 2);
    assert!(
        (quantized.cross_entropy - float.cross_entropy).abs() < 0.03,
        "{quantized:?} against {float:?}"
    );
    // The same measurement from the held-out positions built one by one,
    // over other threads (sums in another order).
    let built: Vec<_> = (0..held_out.len()).map(|i| held_out.get(i)).collect();
    let again = measure::<N>(&trained, &built[..], 3);
    assert!((again.cross_entropy - float.cross_entropy).abs() < 1e-9);
}

/// `train` as it was before its threads lived through the training: threads
/// spawned for each batch, then the sum of their gradients and Adam's step
/// on one thread.
fn train_spawning_threads<const N: usize>(
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
    let fitted_count = u32::try_from(fitted.len()).expect("fewer than 2^32 positions to fit");
    let mut order: Vec<u32> = (0..fitted_count).collect();
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
            clip::<N>(0, &mut parameters);
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

#[test]
fn training_matches_the_threads_spawned_for_each_batch_bit_for_bit() {
    const N: usize = 16;
    let games: Vec<GameRecord> = (0..60).map(random_record).collect();
    let mut split = Split::default();
    split.add(&games);
    let (fitted, held_out) = (&split.fitted, &split.held_out);
    // Batches of 100 leave a short last batch; with 3 or 4 threads, it
    // can need fewer threads than there are. Batches of 2 for 9 threads
    // leave most of them without positions.
    for (epochs, threads, batch, target) in [
        (3, 1, 100, Target::Result),
        (3, 3, 100, Target::Score),
        (3, 4, 100, Target::Mix),
        (3, 4, 7, Target::Result),
        (3, 5, 3000, Target::Result),
        (1, 9, 2, Target::Result),
        (0, 4, 100, Target::Result),
    ] {
        let settings = Training {
            epochs,
            batch,
            rate: 0.003,
            penalty: 1e-4,
            target,
            threads,
            seed: 21,
        };
        let start = initial_parameters::<N>(22);
        let (mut epochs, mut expected_epochs) = (Vec::new(), Vec::new());
        let trained = train::<N>(start.clone(), fitted, held_out, &settings, |epoch| {
            epochs.push((epoch.fitted, epoch.held_out))
        });
        let expected = train_spawning_threads::<N>(start, fitted, held_out, &settings, |epoch| {
            expected_epochs.push((epoch.fitted, epoch.held_out))
        });
        let bits = |values: &[f32]| {
            values
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        };
        assert!(
            bits(&trained) == bits(&expected),
            "{threads} threads, batches of {batch}"
        );
        assert_eq!(
            epochs, expected_epochs,
            "{threads} threads, batches of {batch}"
        );
    }
}

#[test]
fn clipping_a_share_clips_as_the_whole() {
    const N: usize = 8;
    let parameters = random_parameters::<N>(5, 3.0, 3.0);
    let mut whole = parameters.clone();
    clip::<N>(0, &mut whole);
    for threads in [1, 2, 3, 7, 64] {
        let mut shared = parameters.clone();
        for share in shares(shared.len(), threads) {
            let start = share.start;
            clip::<N>(start, &mut shared[share]);
        }
        assert_eq!(shared, whole, "{threads} shares");
    }
}

#[test]
fn a_panicking_thread_breaks_the_gate() {
    let gate = Gate::new(2);
    let mut waited = None;
    // The scope itself panics, as one of its threads did.
    let scope = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        std::thread::scope(|scope| {
            scope.spawn(|| {
                let _guard = gate.guard();
                gate.wait();
                panic!("a test panic");
            });
            gate.wait();
            waited = Some(std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                || gate.wait(),
            )));
        })
    }));
    assert!(scope.is_err());
    assert!(waited.is_some_and(|waited| waited.is_err()));
}

#[test]
fn the_constant_predictor_is_measured_against_results() {
    let metrics = constant_metrics(0.5, [0.0f32, 1.0, 0.5, 0.5].into_iter());
    assert!((metrics.cross_entropy - std::f64::consts::LN_2).abs() < 1e-12);
    assert!((metrics.squared_error - (0.25 + 0.25) / 4.0).abs() < 1e-12);
    // Degenerate averages do not give infinities.
    assert!(constant_metrics(1.0, [0.0f32].into_iter())
        .cross_entropy
        .is_finite());
}

/// A scratch directory of its own, removed on drop.
struct Scratch(std::path::PathBuf);

impl Scratch {
    fn new(name: &str) -> Scratch {
        let path = std::env::temp_dir().join(format!("nnue-fit-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&path).unwrap();
        Scratch(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Writes `games` random games as a data file.
fn write_data(path: &std::path::Path, games: u64) {
    let mut bytes = Vec::new();
    crate::data::write_header(&mut bytes).unwrap();
    for seed in 0..games {
        crate::data::write_game(&mut bytes, &random_record(seed)).unwrap();
    }
    std::fs::write(path, bytes).unwrap();
}

fn options(data: PathBuf, scratch: &Scratch) -> Options {
    Options {
        data: vec![data],
        width: WidthArg::W128,
        training: Training {
            epochs: 2,
            batch: 256,
            rate: 0.001,
            penalty: 0.0,
            target: Target::Mix,
            threads: 2,
            seed: 1,
        },
        compared_positions: 100,
        compare: None,
        weights_out: Some(scratch.0.join("nnue_weights.rs")),
        report_out: Some(scratch.0.join("report.md")),
        origin: "a test".to_string(),
    }
}

#[test]
fn fit_writes_the_weights_and_the_report() {
    let scratch = Scratch::new("fit");
    let data = scratch.0.join("games.bin");
    write_data(&data, 60);
    let options = options(data, &scratch);
    fit(&options).unwrap();

    let module = std::fs::read_to_string(scratch.0.join("nnue_weights.rs")).unwrap();
    assert!(module.contains("pub const NNUE_WEIGHTS: &str = \""));
    assert!(module.contains("a test, accumulator width 128"));
    let start = module.find("NNUE_WEIGHTS: &str = \"").unwrap() + "NNUE_WEIGHTS: &str = \"".len();
    let end = module.rfind("\";").unwrap();
    let bytes = cg_pack::unpack(&module[start..end]).unwrap();
    assert!(Network::<128>::from_bytes(&bytes).is_ok());

    let report = std::fs::read_to_string(scratch.0.join("report.md")).unwrap();
    let tables: Vec<&str> = report.lines().filter(|l| l.starts_with("| ")).collect();
    // The held-out table comes first, as the workflow's annotation reads
    // the first lines.
    assert!(tables[0].starts_with("| Predictor"));
    assert!(tables[2].starts_with("| The average result"));
    assert!(tables[3].starts_with("| Network, float"));
    assert!(tables[4].starts_with("| Network, quantized"));
    assert!(report.contains("| Start |"));
    assert!(report.contains("| Quantized |"));
    assert!(report.contains("(189 -> 128) x 2 -> 30 buckets, 32030 parameters"));
    assert!(report.contains("of which 0 dropped as copies"));
    assert!(report.contains("every 20th game kept, whole"));
    assert!(report.contains("in detail"));
    assert!(!report.contains("Compared"));
    assert!(report.contains("## Data"));
    assert!(report.contains("| Plies | Positions |"));
    assert!(!report.contains("NaN"));
}

#[test]
fn the_data_table_counts_positions_by_ply_and_the_first_players_results() {
    let games: Vec<GameRecord> = (0..40).map(random_record).collect();
    let mut split = Split::default();
    split.add(&games);
    let fitted = &split.fitted;
    let table = data_table(fitted);
    let rows: Vec<Vec<&str>> = table
        .lines()
        .filter(|line| {
            line.starts_with("| ") && !line.starts_with("| Plies") && !line.starts_with("| ---")
        })
        .map(|line| line.split('|').map(str::trim).collect())
        .collect();
    // Every position is in one row, the rows in order of plies.
    let counted: usize = rows
        .iter()
        .map(|row| row[2].parse::<usize>().unwrap())
        .sum();
    assert_eq!(counted, fitted.len());
    assert_eq!(rows[0][1], "0-3");
    assert_eq!(rows.last().unwrap()[1], "32+");
    // Random games' positions at ply 0 are all the start; at ply 4, nearly
    // all different.
    let at_4 = (0..fitted.len()).filter(|&i| fitted.ply(i) == 4).count();
    assert!(table.contains("at ply 4, "));
    assert!(table.contains(&format!(" of {at_4}")));
    // The first player's results, from each game's result and who moves.
    let mut wins = 0;
    let mut losses = 0;
    let fitted_games: Vec<&GameRecord> = games
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 20 != 19)
        .map(|(_, game)| game)
        .collect();
    for game in &fitted_games {
        match game.status() {
            Status::Win(0) => wins += 1,
            Status::Win(_) => losses += 1,
            _ => {}
        }
    }
    let share = |count: usize| 100.0 * count as f64 / fitted_games.len() as f64;
    let expected = format!(
        "The first player won {:.1}% of the {} games, drew {:.1}% and lost {:.1}%.",
        share(wins),
        fitted_games.len(),
        share(fitted_games.len() - wins - losses),
        share(losses)
    );
    assert!(table.contains(&expected), "{table}\nexpected: {expected}");
    // No positions, no rows.
    assert!(!data_table(&Positions::default()).contains("| 0-3"));
}

#[test]
fn fit_reports_too_little_data_as_errors() {
    let scratch = Scratch::new("small");
    let data = scratch.0.join("games.bin");
    // No game.
    write_data(&data, 0);
    let error = fit(&options(data.clone(), &scratch)).unwrap_err();
    assert!(error.contains("not enough games"), "{error}");
    // Fewer than 20 games: nothing is held out.
    write_data(&data, 10);
    let error = fit(&options(data, &scratch)).unwrap_err();
    assert!(error.contains("not enough games"), "{error}");
    // A missing file and a file that is not data.
    let error = fit(&options(scratch.0.join("missing.bin"), &scratch)).unwrap_err();
    assert!(error.contains("missing.bin"), "{error}");
    let junk = scratch.0.join("junk.bin");
    std::fs::write(&junk, b"not a data file").unwrap();
    assert!(fit(&options(junk, &scratch)).is_err());
}

/// The bot's network, as the Train workflow reads it.
fn bot_network() -> Network<256> {
    crate::selfplay::read_network(include_str!("../../../bots/mcts/src/nnue_weights.rs")).unwrap()
}

#[test]
fn fit_drops_copies_and_compares_another_network() {
    let scratch = Scratch::new("fit-compare");
    let data = scratch.0.join("games.bin");
    let copies = scratch.0.join("copies.bin");
    write_data(&data, 60);
    // Copies of 30 of them, in another file.
    write_data(&copies, 30);
    let mut options = options(data, &scratch);
    options.data.push(copies);
    options.compare = Some(("the bot's".to_string(), bot_network()));
    fit(&options).unwrap();

    let report = std::fs::read_to_string(scratch.0.join("report.md")).unwrap();
    assert!(report.contains("- Games: 90, of which 30 dropped as copies"));
    assert!(report.contains("- Compared network: `the bot's`, width 256"));
    let tables: Vec<&str> = report.lines().filter(|l| l.starts_with("| ")).collect();
    // Both held-out tables, within the workflow annotation's 14 lines.
    assert!(tables[5].starts_with("| Compared network, quantized"));
    assert!(tables[11].starts_with("| Compared network, quantized"));
    assert!(report.contains("| Plies | Positions | Network: squared error | Network: agreement | Compared: squared error | Compared: agreement |"));
    assert!(report.contains("| All | |"));
    assert!(report.contains("Average gap between prediction and result"));
    assert!(report.contains("of 128; Compared,"));
    assert!(!report.contains("NaN"));
}

#[test]
fn a_distinct_split_drops_copies_from_any_file() {
    let games: Vec<GameRecord> = (0..100).map(random_record).collect();
    let mut all = Split::default();
    all.add(&games);
    all.add(&games[..40]);
    assert_eq!((all.games, all.duplicates), (140, 0));

    let mut distinct = Split::distinct();
    distinct.add(&games[..60]);
    distinct.add(&games);
    assert_eq!((distinct.games, distinct.duplicates), (160, 60));
    // Every 20th game kept is held out: copies do not shift the split.
    let mut plain = Split::default();
    plain.add(&games);
    assert_eq!(distinct.fitted.len(), plain.fitted.len());
    assert_eq!(distinct.held_out.len(), plain.held_out.len());
    for index in 0..plain.held_out.len() {
        assert_eq!(
            distinct.held_out.get(index).board,
            plain.held_out.get(index).board
        );
    }
}

/// A game in which every move is the one `network` would choose, every
/// position searched.
fn network_game(network: &Network<256>, opening: &[Move]) -> GameRecord {
    let mut board = Board::new();
    let mut record = GameRecord::default();
    while board.status() == Status::Ongoing {
        record.searched.push(Searched {
            ply: record.moves.len() as u8,
            score: Some(0.5),
            visits: Vec::new(),
        });
        let mv = match opening.get(record.moves.len()) {
            Some(&mv) => mv,
            None => network_move(network, &board).unwrap(),
        };
        record.moves.push(mv);
        board.play(mv);
    }
    record
}

#[test]
fn the_assessment_counts_agreement_calibration_and_units() {
    let network = bot_network();
    // Two random moves, then the network's choices.
    let games: Vec<GameRecord> = (0..20)
        .map(|seed| {
            let random = random_record(seed);
            network_game(&network, &random.moves[..2])
        })
        .collect();
    let mut split = Split::default();
    split.add(&games);
    let positions = &split.fitted;
    let indices: Vec<usize> = (0..positions.len()).collect();
    let assessment = assess(&network, positions, &indices);
    let counted: usize = assessment.phases.iter().map(|phase| phase.0).sum();
    assert_eq!(counted, positions.len());
    assert_eq!(
        assessment.bins.iter().map(|bin| bin.0).sum::<usize>(),
        positions.len()
    );
    // After the two random moves, every move is the network's.
    let (count, _, agreed) = assessment.phases[0];
    assert!(agreed >= count / 2, "{agreed} of {count}");
    for phase in &assessment.phases[1..] {
        assert_eq!(phase.2, phase.0);
    }
    assert!(assessment.agreement() > 0.9);
    assert!(assessment.calibration_error() <= 1.0);
    assert_eq!(assessment.width, 256);
    assert!(assessment.dead + assessment.constant <= 256);

    // Units that never move are counted, and none without positions.
    let dead = assess(&network_at(-0.5), positions, &indices);
    assert_eq!((dead.dead, dead.constant), (128, 0));
    let constant = assess(&network_at(1.5), positions, &indices);
    assert_eq!((constant.dead, constant.constant), (0, 128));
    let none = assess(&network, positions, &[]);
    assert_eq!((none.dead, none.constant), (0, 0));
}

/// A network of width 128 whose units all sit at `bias`, whatever the
/// position: its first layer's weights are 0.
fn network_at(bias: f32) -> Network<128> {
    let mut parameters = random_parameters::<128>(1, 0.0, 0.1);
    for value in &mut parameters[Shape::<128>::BIAS..Shape::<128>::OUTPUT] {
        *value = bias;
    }
    quantize::<128>(&parameters).unwrap()
}

#[test]
fn next_move_is_the_move_played() {
    let games: Vec<GameRecord> = (0..3).map(random_record).collect();
    let mut split = Split::default();
    split.add(&games);
    let positions = &split.fitted;
    for index in 0..positions.len() {
        let game = &games[positions.game(index) as usize];
        let ply = usize::from(positions.ply(index));
        assert_eq!(positions.next_move(index), game.moves.get(ply).copied());
    }
}
