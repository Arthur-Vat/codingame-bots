//! `uttt-trainer`: self-play data and training for Ultimate Tic-Tac-Toe
//! bots (ADR 0016). Run with `--help` for the commands.

use std::fmt::Write as _;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use cg_core::rng::Rng;
use clap::{Parser, Subcommand};
use uttt_engine::board::CLASSES;
use uttt_engine::value::Network;
use uttt_engine::{PatternPolicy, PlayoutPolicy};
use value_fit::Examples as _;

mod data;
mod duel;
mod fit;
mod head_start;
mod patterns_fit;
mod selfplay;
mod value_fit;

#[derive(Parser)]
#[command(about = "Self-play data and training for Ultimate Tic-Tac-Toe bots")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// The value network's hidden layers.
#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum SizeArg {
    /// 64 then 16 units, about 15,000 parameters.
    #[value(name = "64-16")]
    Small,
    /// 128 then 32 units, about 32,000 parameters.
    #[value(name = "128-32")]
    Large,
}

/// What the value network learns.
#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum TargetArg {
    Result,
    Score,
    Mix,
}

#[derive(Subcommand)]
enum Command {
    /// Plays self-play games and writes each searched position's root
    /// visits.
    Selfplay {
        /// Games to play.
        #[arg(long, default_value_t = 100)]
        games: u32,
        /// Search iterations per move.
        #[arg(long, default_value_t = 10_000)]
        iterations: u64,
        /// Random moves at the start of each game.
        #[arg(long, default_value_t = 6)]
        opening_plies: u32,
        /// The search's exploration constant.
        #[arg(long, default_value_t = 0.5)]
        exploration: f64,
        /// Rust source holding the bot's playout weights (`PLAYOUT_WEIGHTS`),
        /// such as `games/uttt/bots/mcts/src/weights.rs`, for the bot's
        /// playouts; decisive playouts without it.
        #[arg(long)]
        policy: Option<PathBuf>,
        /// Rust source holding a pattern policy's text (`PATTERN_TEXT`), such
        /// as `games/uttt/bots/mcts/src/pattern_weights.rs`, for the bot's
        /// playouts and order of children since E015; replaces `--policy`.
        #[arg(long)]
        patterns: Option<PathBuf>,
        /// Moves of each playout drawn from the policy, as in the bot.
        #[arg(long, default_value_t = 16)]
        policy_plies: u32,
        /// Records only each searched position's root score, not the visits
        /// of each move, which only the playout policy's fit needs.
        #[arg(long)]
        no_visits: bool,
        /// Seed of the first game; each game derives its own.
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// The data file to write.
        #[arg(long)]
        out: PathBuf,
    },
    /// Fits the playout policy's class weights to self-play data.
    FitPolicy {
        /// Data files written by `selfplay`.
        #[arg(long = "data", required = true)]
        data: Vec<PathBuf>,
        /// Above 1, flattens the weights the bot gets.
        #[arg(long, default_value_t = 1.0)]
        temperature: f64,
        /// Weight of the penalty on the parameters' size.
        #[arg(long, default_value_t = 0.001)]
        penalty: f64,
        /// Gradient steps.
        #[arg(long, default_value_t = 2_000)]
        steps: u32,
        /// Where to write the weights, as Rust source.
        #[arg(long)]
        weights_out: PathBuf,
        /// Where to write the report, in Markdown.
        #[arg(long)]
        report_out: PathBuf,
        /// Where the data came from, for the generated files.
        #[arg(long, default_value = "local run")]
        origin: String,
    },
    /// Fits move models to self-play data with visits and compares them:
    /// the playout policy's 32 classes, E015's pattern policy, and larger
    /// models (E016).
    FitPatterns {
        /// Data files written by `selfplay`, with visits.
        #[arg(long = "data", required = true)]
        data: Vec<PathBuf>,
        /// Passes over the fitted positions.
        #[arg(long, default_value_t = 4)]
        epochs: u32,
        /// Positions per step.
        #[arg(long, default_value_t = 256)]
        batch: usize,
        /// Adam's step size.
        #[arg(long, default_value_t = 0.01)]
        rate: f32,
        /// Weight of the penalty on the log-weights' squares.
        #[arg(long, default_value_t = 1e-5)]
        penalty: f32,
        /// Below 1, sharpens the weights written for bots.
        #[arg(long, default_value_t = 1.0)]
        temperature: f32,
        /// Seed of the order of positions.
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// The models to fit, by name, or `all`: classes, patterns,
        /// destinations-7, phases-2, destination-patterns, rich, large,
        /// destination-roles, phases-destinations.
        #[arg(long, value_delimiter = ',', default_value = "classes,patterns")]
        models: Vec<String>,
        /// Where to write each model's weights, as Rust source named after
        /// the model (`patterns.rs` is the bot's format); the classes' are
        /// not written.
        #[arg(long)]
        weights_dir: PathBuf,
        /// Where to write the report, in Markdown.
        #[arg(long)]
        report_out: PathBuf,
        /// Where the data came from, for the generated files.
        #[arg(long, default_value = "local run")]
        origin: String,
    },
    /// Plays the bot's search against itself, one side searching longer
    /// on its first moves: the most an opening book could bring (E013).
    HeadStart {
        /// Rust source holding the bot's playout weights.
        #[arg(long)]
        policy: PathBuf,
        /// Moves of each playout drawn from the policy, as in the bot.
        #[arg(long, default_value_t = 16)]
        policy_plies: u32,
        /// Pairs of games; each side plays first once per pair.
        #[arg(long, default_value_t = 100)]
        pairs: u32,
        /// Iterations per move after the first, for both sides; about what
        /// the bot runs in 90 ms.
        #[arg(long, default_value_t = 60_000)]
        iterations: u64,
        /// Iterations of each side's first move, the first turn's longer
        /// search.
        #[arg(long, default_value_t = 600_000)]
        first_iterations: u64,
        /// How many times the usual iterations the head start gives.
        #[arg(long, default_value_t = 20)]
        boost: u64,
        /// How many first moves get the head start.
        #[arg(long, default_value_t = 4)]
        moves: u32,
        /// Seed of the games.
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Threads [default: one per core].
        #[arg(long)]
        threads: Option<usize>,
    },
    /// Trains the value network on self-play games and compares it with
    /// the bot's playouts (ADR 0017).
    FitValue {
        /// Data files written by `selfplay`.
        #[arg(long = "data", required = true)]
        data: Vec<PathBuf>,
        /// Passes over the fitted positions.
        #[arg(long, default_value_t = 10)]
        epochs: u32,
        /// Positions per gradient step.
        #[arg(long, default_value_t = 1024)]
        batch: usize,
        /// Adam's step size at the start; it falls to a twentieth of it.
        #[arg(long, default_value_t = 0.001)]
        rate: f32,
        /// Weight of the penalty on the parameters' squares.
        #[arg(long, default_value_t = 0.0)]
        penalty: f32,
        /// What the network learns: the game's result, the search's root
        /// score, or their average.
        #[arg(long, value_enum, default_value_t = TargetArg::Result)]
        target: TargetArg,
        /// Threads [default: one per core].
        #[arg(long)]
        threads: Option<usize>,
        /// Seed of the starting weights, the order of positions and the
        /// playouts.
        #[arg(long, default_value_t = 1)]
        seed: u64,
        /// Rust source holding the bot's playout weights, whose playouts the
        /// network must beat.
        #[arg(long)]
        policy: PathBuf,
        /// Moves of each playout drawn from the policy, as in the bot.
        #[arg(long, default_value_t = 16)]
        policy_plies: u32,
        /// Held-out positions on which the network's predictions are
        /// compared with playouts'.
        #[arg(long, default_value_t = 50_000)]
        compared_positions: usize,
        /// The network's hidden layers.
        #[arg(long, value_enum, default_value_t = SizeArg::Small)]
        size: SizeArg,
        /// Pairs of games, for each share below, between a search with the
        /// network and one with the playouts: the gate of ADRs 0018 and
        /// 0019. 0 for none.
        #[arg(long, default_value_t = 300)]
        duel_pairs: u32,
        /// Milliseconds of search per move in those games, the bot's at
        /// CodinGame's limits.
        #[arg(long, default_value_t = 90)]
        duel_ms: u64,
        /// Milliseconds of search on each side's first move, the bot's
        /// first turn.
        #[arg(long, default_value_t = 900)]
        duel_first_ms: u64,
        /// Iterations per move instead of time, on every move.
        #[arg(long)]
        duel_iterations: Option<u64>,
        /// The network's shares of a leaf's estimate to play, the rest
        /// coming from a playout: 1 for the network alone.
        #[arg(long, value_delimiter = ',', default_value = "1,0.5")]
        duel_shares: Vec<f64>,
        /// The exploration constant of the search with the network in those
        /// games; the one with playouts keeps the bot's 0.5.
        #[arg(long, default_value_t = 0.3)]
        duel_exploration: f64,
        /// Where to write the weights, as Rust source.
        #[arg(long)]
        weights_out: PathBuf,
        /// Where to write the report, in Markdown.
        #[arg(long)]
        report_out: PathBuf,
        /// Where the data came from, for the generated files.
        #[arg(long, default_value = "local run")]
        origin: String,
    },
}

fn main() -> ExitCode {
    let result = match Cli::parse().command {
        Command::Selfplay {
            games,
            iterations,
            opening_plies,
            exploration,
            policy,
            patterns,
            policy_plies,
            no_visits,
            seed,
            out,
        } => policy
            .map(|path| load_policy(&path, policy_plies))
            .transpose()
            .and_then(|policy| {
                let patterns = patterns
                    .map(|path| load_patterns(&path, policy_plies))
                    .transpose()?;
                selfplay(
                    &selfplay::SelfPlay {
                        iterations,
                        opening_plies,
                        exploration,
                        policy,
                        patterns,
                        record_visits: !no_visits,
                    },
                    games,
                    seed,
                    &out,
                )
            }),
        Command::FitPolicy {
            data,
            temperature,
            penalty,
            steps,
            weights_out,
            report_out,
            origin,
        } => fit_policy(&FitOptions {
            data,
            temperature,
            penalty,
            steps,
            weights_out,
            report_out,
            origin,
        }),
        Command::FitPatterns {
            data,
            epochs,
            batch,
            rate,
            penalty,
            temperature,
            seed,
            models,
            weights_dir,
            report_out,
            origin,
        } => pattern_models(&models).and_then(|models| {
            fit_patterns(
                &data,
                &patterns_fit::Fitting {
                    epochs,
                    batch,
                    rate,
                    penalty,
                    seed,
                },
                temperature,
                &models,
                &weights_dir,
                &report_out,
                &origin,
            )
        }),
        Command::HeadStart {
            policy,
            policy_plies,
            pairs,
            iterations,
            first_iterations,
            boost,
            moves,
            seed,
            threads,
        } => load_policy(&policy, policy_plies).map(|policy| {
            let head_start = head_start::HeadStart {
                iterations,
                first_iterations,
                boost,
                moves,
                exploration: 0.5,
            };
            let threads = threads
                .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, usize::from));
            let start = Instant::now();
            let results = head_start.play(policy, pairs, seed, threads);
            let elo = match results.elo() {
                Some(elo) => format!("{:+.1} Elo [{:+.1}, {:+.1}]", elo.elo, elo.low, elo.high),
                None => "no Elo estimate".to_string(),
            };
            println!(
                "head start of {boost}x on the first {moves} moves, {iterations} iterations per move, {first_iterations} on the first, {pairs} pairs, seed {seed}: {} wins, {} draws, {} losses, {elo}; pairs by points (0, 1/2, 1, 3/2, 2): {:?}; {:.0} s",
                results.wins,
                results.draws,
                results.losses,
                results.pairs,
                start.elapsed().as_secs_f64()
            );
        }),
        Command::FitValue {
            data,
            epochs,
            batch,
            rate,
            penalty,
            target,
            threads,
            seed,
            policy,
            policy_plies,
            compared_positions,
            size,
            duel_pairs,
            duel_ms,
            duel_first_ms,
            duel_iterations,
            duel_shares,
            duel_exploration,
            weights_out,
            report_out,
            origin,
        } => fit_value(&ValueOptions {
            data,
            training: value_fit::Training {
                epochs,
                batch: batch.max(1),
                rate,
                penalty,
                target: match target {
                    TargetArg::Result => value_fit::Target::Result,
                    TargetArg::Score => value_fit::Target::Score,
                    TargetArg::Mix => value_fit::Target::Mix,
                },
                threads: threads
                    .unwrap_or_else(|| std::thread::available_parallelism().map_or(1, usize::from)),
                seed,
            },
            policy,
            policy_plies,
            compared_positions,
            size,
            duel_pairs,
            duel: match duel_iterations {
                Some(count) => (
                    duel::Limit::Iterations(count),
                    duel::Limit::Iterations(count),
                ),
                None => (
                    duel::Limit::Time(std::time::Duration::from_millis(duel_first_ms)),
                    duel::Limit::Time(std::time::Duration::from_millis(duel_ms)),
                ),
            },
            duel_shares,
            duel_exploration,
            weights_out,
            report_out,
            origin,
        }),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn selfplay(
    settings: &selfplay::SelfPlay,
    games: u32,
    seed: u64,
    out: &PathBuf,
) -> Result<(), String> {
    let file = File::create(out).map_err(|err| format!("{}: {err}", out.display()))?;
    let mut out_file = BufWriter::new(file);
    data::write_header(&mut out_file).map_err(|err| err.to_string())?;
    let mut seeds = Rng::new(seed);
    let start = Instant::now();
    let mut positions = 0;
    for game in 0..games {
        let record = settings.play(seeds.next_u64());
        positions += record.searched.len();
        data::write_game(&mut out_file, &record).map_err(|err| err.to_string())?;
        if (game + 1) % 50 == 0 {
            eprintln!(
                "{} games, {positions} positions, {:.0} s",
                game + 1,
                start.elapsed().as_secs_f64()
            );
        }
    }
    out_file.flush().map_err(|err| err.to_string())?;
    eprintln!(
        "wrote {games} games, {positions} positions to {} in {:.0} s",
        out.display(),
        start.elapsed().as_secs_f64()
    );
    Ok(())
}

struct FitOptions {
    data: Vec<PathBuf>,
    temperature: f64,
    penalty: f64,
    steps: u32,
    weights_out: PathBuf,
    report_out: PathBuf,
    origin: String,
}

/// The games of one data file.
fn read_file(path: &Path) -> Result<Vec<data::GameRecord>, String> {
    let file = File::open(path).map_err(|err| format!("{}: {err}", path.display()))?;
    data::read_games(&mut BufReader::new(file)).map_err(|err| format!("{}: {err}", path.display()))
}

/// The games of every data file.
fn read_data(paths: &[PathBuf]) -> Result<Vec<data::GameRecord>, String> {
    let mut games = Vec::new();
    for path in paths {
        games.append(&mut read_file(path)?);
    }
    Ok(games)
}

/// The playout policy whose weights `path`'s Rust source holds, drawing
/// `plies` moves of each playout.
fn load_policy(path: &Path, plies: u32) -> Result<&'static PlayoutPolicy, String> {
    let source =
        std::fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
    let weights = selfplay::read_policy_weights(&source)
        .map_err(|err| format!("{}: {err}", path.display()))?;
    if let Some(weight) = weights.iter().find(|&&weight| weight >= 1 << 20) {
        return Err(format!(
            "{}: weight {weight} is 2^20 or more",
            path.display()
        ));
    }
    Ok(Box::leak(Box::new(
        PlayoutPolicy::new(weights).for_plies(plies),
    )))
}

/// The pattern policy whose text `path`'s Rust source holds, drawing
/// `plies` moves of each playout.
fn load_patterns(path: &Path, plies: u32) -> Result<&'static PatternPolicy, String> {
    let source =
        std::fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
    let policy = selfplay::read_pattern_text(&source)
        .and_then(|text| PatternPolicy::decode(&text))
        .map_err(|err| format!("{}: {err}", path.display()))?;
    Ok(Box::leak(Box::new(policy.for_plies(plies))))
}

fn fit_policy(options: &FitOptions) -> Result<(), String> {
    let games = read_data(&options.data)?;
    let examples = fit::examples(&games);
    if examples.is_empty() {
        return Err("no searched positions in the data".to_string());
    }
    // A quarter of the positions, every fourth one, is kept out of the fit
    // to check that the policy generalises.
    let (train, held_out): (Vec<_>, Vec<_>) = examples
        .iter()
        .cloned()
        .enumerate()
        .partition(|(index, _)| index % 4 != 3);
    let train: Vec<_> = train.into_iter().map(|(_, example)| example).collect();
    let held_out: Vec<_> = held_out.into_iter().map(|(_, example)| example).collect();

    let (theta, train_loss) = fit::fit(&train, options.steps, options.penalty);
    let uniform = [0.0; CLASSES];
    let (uniform_loss, _) = fit::loss_and_gradient(&uniform, &held_out);
    let (held_out_loss, _) = fit::loss_and_gradient(&theta, &held_out);
    let weights = fit::weights(&theta, options.temperature, 10_000);

    std::fs::write(
        &options.weights_out,
        fit::weights_source(&weights, &options.origin),
    )
    .map_err(|err| format!("{}: {err}", options.weights_out.display()))?;

    let mut moves = [0u64; CLASSES];
    let mut visits = [0.0; CLASSES];
    for example in &examples {
        for k in 0..CLASSES {
            moves[k] += u64::from(example.moves[k]);
            visits[k] += example.visits[k];
        }
    }
    let mut report = String::new();
    let _ = writeln!(report, "# Playout policy fit\n");
    let _ = writeln!(report, "- Data: {}", options.origin);
    let _ = writeln!(
        report,
        "- Games: {}; searched positions: {} ({} fitted, {} held out)",
        games.len(),
        examples.len(),
        train.len(),
        held_out.len()
    );
    let _ = writeln!(
        report,
        "- Settings: {} steps, penalty {}, temperature {}",
        options.steps, options.penalty, options.temperature
    );
    let _ = writeln!(
        report,
        "- Cross-entropy on held-out positions, nats per position: uniform {:.4}, fitted {:.4} (fitted on its own positions: {:.4})",
        uniform_loss, held_out_loss, train_loss
    );
    let _ = writeln!(
        report,
        "- Probability of the search's preferred move, held out: uniform {:.3}, fitted {:.3}\n",
        fit::best_move_probability(&uniform, &held_out),
        fit::best_move_probability(&theta, &held_out)
    );
    let _ = writeln!(
        report,
        "| Class | Features | Moves seen | Share of visits | θ | Weight |"
    );
    let _ = writeln!(report, "| --- | --- | --- | --- | --- | --- |");
    let total_positions = examples.len() as f64;
    for k in 0..CLASSES {
        if moves[k] == 0 {
            continue;
        }
        let _ = writeln!(
            report,
            "| {k} | {} | {} | {:.4} | {:+.3} | {} |",
            fit::class_name(k),
            moves[k],
            visits[k] / total_positions,
            theta[k],
            weights[k]
        );
    }
    std::fs::write(&options.report_out, &report)
        .map_err(|err| format!("{}: {err}", options.report_out.display()))?;
    print!("{report}");
    Ok(())
}

struct ValueOptions {
    data: Vec<PathBuf>,
    training: value_fit::Training,
    policy: PathBuf,
    policy_plies: u32,
    compared_positions: usize,
    size: SizeArg,
    duel_pairs: u32,
    /// The budgets of each side's first move and of its later moves.
    duel: (duel::Limit, duel::Limit),
    duel_shares: Vec<f64>,
    duel_exploration: f64,
    weights_out: PathBuf,
    report_out: PathBuf,
    origin: String,
}

fn fit_value(options: &ValueOptions) -> Result<(), String> {
    match options.size {
        SizeArg::Small => fit_value_sized::<64, 16>(options),
        SizeArg::Large => fit_value_sized::<128, 32>(options),
    }
}

/// How a duel's budgets read in the report.
fn describe(limits: (duel::Limit, duel::Limit)) -> String {
    let one = |limit: duel::Limit| match limit {
        duel::Limit::Iterations(count) => format!("{count} iterations"),
        duel::Limit::Time(time) => format!("{} ms", time.as_millis()),
    };
    if limits.0 == limits.1 {
        format!("{} per move", one(limits.1))
    } else {
        format!(
            "{} on each side's first move, then {} per move",
            one(limits.0),
            one(limits.1)
        )
    }
}

fn fit_value_sized<const H: usize, const H2: usize>(options: &ValueOptions) -> Result<(), String> {
    let start = Instant::now();
    let policy = load_policy(&options.policy, options.policy_plies)?;
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

    let initial = value_fit::initial_parameters::<H, H2>(settings.seed);
    let initial_metrics = value_fit::measure::<H, H2>(&initial, held_out, threads);
    let mut epochs = Vec::new();
    let parameters = value_fit::train::<H, H2>(initial, fitted, held_out, settings, |epoch| {
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
    let (rounded, text) = value_fit::quantize::<H, H2>(&parameters);
    std::fs::write(
        &options.weights_out,
        value_fit::weights_source(&text, &options.origin, (H, H2)),
    )
    .map_err(|err| format!("{}: {err}", options.weights_out.display()))?;

    // The same held-out positions for every predictor.
    let compared = value_fit::spread(held_out, options.compared_positions);
    let counts = [1, 2, 4, 8, 16, 32];
    eprintln!(
        "predictions: {} positions, up to {} playouts each",
        compared.len(),
        counts[5]
    );
    let errors = value_fit::playout_errors(&compared, policy, &counts, settings.seed, threads);
    let network = value_fit::measure::<H, H2>(&parameters, &compared[..], threads);
    let network_rounded = value_fit::measure::<H, H2>(&rounded, &compared[..], threads);
    let average = (0..fitted.len())
        .map(|index| f64::from(fitted.result(index)))
        .sum::<f64>()
        / fitted.len() as f64;
    let constant = compared
        .iter()
        .map(|e| (average - f64::from(e.result)).powi(2))
        .sum::<f64>()
        / compared.len() as f64;
    let worth = value_fit::worth_in_playouts(network_rounded.squared_error, &counts, &errors);
    let held_out_rounded = value_fit::measure::<H, H2>(&rounded, held_out, threads);
    let network_rounded_weights: &'static Network<H, H2> = Box::leak(Box::new(
        Network::<H, H2>::from_parameters(&rounded).expect("the trainer's layout"),
    ));
    let budgets = describe(options.duel);
    let mut duels = Vec::new();
    if options.duel_pairs > 0 {
        for &share in &options.duel_shares {
            let duel = duel::Duel {
                first: options.duel.0,
                later: options.duel.1,
                opening_plies: 6,
                exploration: 0.5,
                network_exploration: options.duel_exploration,
                network_share: share,
            };
            eprintln!(
                "duel: network share {share}, {} pairs, {budgets}, {:.0} s",
                options.duel_pairs,
                start.elapsed().as_secs_f64()
            );
            let results = duel.play(
                network_rounded_weights,
                policy,
                options.duel_pairs,
                settings.seed,
                threads,
            );
            duels.push((share, results));
        }
    }

    let mut report = String::new();
    let _ = writeln!(report, "# Value network fit\n");
    let _ = writeln!(report, "- Data: {}", options.origin);
    let _ = writeln!(
        report,
        "- Games: {game_count}; positions: {} fitted, {} held out (every 20th game, whole)",
        fitted.len(),
        held_out.len()
    );
    let _ = writeln!(
        report,
        "- Network: 217-{H}-{H2}-1, {} parameters; weights rounded to 8 bits, {} characters of base64",
        value_fit::Shape::<H, H2>::PARAMETERS,
        text.len()
    );
    let _ = writeln!(
        report,
        "- Training: target {:?}, {} epochs, batches of {}, Adam from {} falling to a twentieth, penalty {}, seed {}, {} threads, {:.0} s in all",
        settings.target,
        settings.epochs,
        settings.batch,
        settings.rate,
        settings.penalty,
        settings.seed,
        threads,
        start.elapsed().as_secs_f64()
    );
    let _ = writeln!(
        report,
        "- Playouts: `{}`, {} policy moves each",
        options.policy.display(),
        options.policy_plies
    );
    let elo = |results: &duel::Results| match results.elo() {
        Some(elo) => format!("{:+.1} Elo [{:+.1}, {:+.1}]", elo.elo, elo.low, elo.high),
        None => "no Elo estimate".to_string(),
    };
    let variant = |share: f64| {
        if share >= 1.0 {
            "Network alone".to_string()
        } else {
            format!(
                "{:.0}% network, {:.0}% playout",
                100.0 * share,
                100.0 * (1.0 - share)
            )
        }
    };
    if duels.is_empty() {
        let _ = writeln!(
            report,
            "\n**Gate of ADR 0019: not measured**, no games played."
        );
    } else {
        let passing: Vec<String> = duels
            .iter()
            .filter(|(_, results)| results.not_clearly_weaker())
            .map(|(share, _)| variant(*share).to_lowercase())
            .collect();
        let verdict = if passing.is_empty() {
            "fails: every variant is clearly weaker".to_string()
        } else {
            format!("passes with {}", passing.join(" and "))
        };
        let _ = writeln!(report, "\n**Gate of ADR 0019: {verdict}.**");
        let _ = writeln!(
            report,
            "\n## Against the playouts, at the bot's budgets\n\nA search with the network (weights rounded, exploration {}) against one with the playouts above (exploration 0.5), {budgets}, {} pairs of games from 6 random opening moves, {threads} games at a time:\n",
            options.duel_exploration, options.duel_pairs
        );
        let _ = writeln!(
            report,
            "| Leaf estimate | Wins, draws, losses | Elo | Pairs by the network's points (0, ½, 1, 1½, 2) |"
        );
        let _ = writeln!(report, "| --- | --- | --- | --- |");
        for (share, results) in &duels {
            let _ = writeln!(
                report,
                "| {} | {}, {}, {} | {} | {:?} |",
                variant(*share),
                results.wins,
                results.draws,
                results.losses,
                elo(results),
                results.pairs
            );
        }
    }
    let _ = writeln!(
        report,
        "\n## Predictions on held-out positions\n\nOn {} positions of held-out games, against the game's result for the side to move. Measured this way, the network with rounded weights is worth {worth}. A single playout's error is mostly its own noise, which a search averages away, so only the games above judge the network (ADRs 0018 and 0019).\n",
        compared.len()
    );
    let _ = writeln!(
        report,
        "| Predictor | Squared error | Cross-entropy, nats |"
    );
    let _ = writeln!(report, "| --- | --- | --- |");
    let _ = writeln!(
        report,
        "| The average result of fitted games, {average:.3} | {constant:.4} | |"
    );
    for (count, error) in counts.iter().zip(&errors) {
        let _ = writeln!(report, "| Average of {count} playouts | {error:.4} | |");
    }
    let _ = writeln!(
        report,
        "| Network | {:.4} | {:.4} |",
        network.squared_error, network.cross_entropy
    );
    let _ = writeln!(
        report,
        "| Network, weights rounded | {:.4} | {:.4} |",
        network_rounded.squared_error, network_rounded.cross_entropy
    );
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
        "| Rounded | | {:.4} | {:.4} |",
        held_out_rounded.cross_entropy, held_out_rounded.squared_error
    );
    std::fs::write(&options.report_out, &report)
        .map_err(|err| format!("{}: {err}", options.report_out.display()))?;
    print!("{report}");
    Ok(())
}

/// The models named in `names`, `all` standing for every one.
fn pattern_models(names: &[String]) -> Result<Vec<patterns_fit::Model>, String> {
    let mut models = Vec::new();
    for name in names {
        if name == "all" {
            models.extend(patterns_fit::ALL);
        } else {
            models.push(
                patterns_fit::Model::from_name(name)
                    .ok_or_else(|| format!("no model called {name:?}"))?,
            );
        }
    }
    models.dedup();
    Ok(models)
}

/// One fitted model's results, for the report.
struct PatternRow {
    model: patterns_fit::Model,
    seen: usize,
    held_out: patterns_fit::Metrics,
    rounded: patterns_fit::Metrics,
    epochs: Vec<f64>,
    characters: Option<(usize, usize)>,
}

fn fit_patterns(
    data: &[PathBuf],
    settings: &patterns_fit::Fitting,
    temperature: f32,
    models: &[patterns_fit::Model],
    weights_dir: &Path,
    report_out: &Path,
    origin: &str,
) -> Result<(), String> {
    use patterns_fit::{Model, Split};
    let start = Instant::now();
    let games = read_data(data)?;
    eprintln!(
        "{} games read in {:.0} s",
        games.len(),
        start.elapsed().as_secs_f64()
    );
    std::fs::create_dir_all(weights_dir)
        .map_err(|err| format!("{}: {err}", weights_dir.display()))?;
    let mut rows = Vec::new();
    let mut counts = None;
    let mut uniform = None;
    for &model in models {
        let mut split = Split::new(model);
        split.add(&games);
        let (fitted, held_out) = (&split.fitted, &split.held_out);
        if fitted.positions() == 0 || held_out.positions() == 0 {
            return Err("no searched positions with visits in the data".to_string());
        }
        eprintln!(
            "{}: {} positions to fit ({} moves), {} held out, {:.0} s",
            model.name(),
            fitted.positions(),
            fitted.moves(),
            held_out.positions(),
            start.elapsed().as_secs_f64()
        );
        let weights = model.weights();
        uniform.get_or_insert_with(|| patterns_fit::measure(&vec![0.0; weights], held_out));
        counts.get_or_insert((
            fitted.games + held_out.games,
            fitted.positions(),
            fitted.moves(),
            held_out.positions(),
        ));
        let mut epochs = Vec::new();
        let theta = patterns_fit::fit(weights, fitted, settings, |epoch, theta| {
            let metrics = patterns_fit::measure(theta, held_out);
            eprintln!(
                "{} epoch {epoch}: held out {:.4} nats, favourite move {:.3}, {:.0} s",
                model.name(),
                metrics.cross_entropy,
                metrics.best_move,
                start.elapsed().as_secs_f64()
            );
            epochs.push(metrics.cross_entropy);
        });
        let rounded: Vec<f32> = theta
            .iter()
            .map(|&value| ((value * 4.0).round() / 4.0).clamp(-8.0, 7.75))
            .collect();
        let mut seen = vec![false; weights];
        for &feature in &fitted.features {
            seen[feature as usize] = true;
        }
        let characters = (model != Model::Classes).then(|| {
            let sharpened: Vec<f32> = theta.iter().map(|&value| value / temperature).collect();
            let text = uttt_engine::board::encode_pattern_weights(&sharpened);
            let source = patterns_fit::weights_source(model, &text, origin);
            let path = weights_dir.join(format!("{}.rs", model.name()));
            std::fs::write(&path, source)
                .map(|()| (text.len(), patterns_fit::entropy_characters(&text)))
                .map_err(|err| format!("{}: {err}", path.display()))
        });
        rows.push(PatternRow {
            model,
            seen: seen.iter().filter(|&&seen| seen).count(),
            held_out: patterns_fit::measure(&theta, held_out),
            rounded: patterns_fit::measure(&rounded, held_out),
            epochs,
            characters: characters.transpose()?,
        });
    }
    let (games, fitted, moves, held_out) = counts.unwrap_or_default();
    let uniform = uniform.unwrap_or_default();

    let mut report = String::new();
    let _ = writeln!(report, "# Move models fit\n");
    let _ = writeln!(report, "- Data: {origin}");
    let _ = writeln!(
        report,
        "- Games: {games}; positions: {fitted} fitted ({moves} moves), {held_out} held out (every 20th game, whole)"
    );
    let _ = writeln!(
        report,
        "- Settings: {} epochs, batches of {} positions, Adam at {}, penalty {}, seed {}; weights written divided by temperature {}; {:.0} s in all\n",
        settings.epochs,
        settings.batch,
        settings.rate,
        settings.penalty,
        settings.seed,
        temperature,
        start.elapsed().as_secs_f64()
    );
    let _ = writeln!(
        report,
        "| Model | Weights | Seen in fitted positions | Held-out cross-entropy, nats | Probability of the search's favourite move | Cross-entropy, weights rounded to quarters | Base64 characters | Entropy-coded, about |"
    );
    let _ = writeln!(report, "| --- | --- | --- | --- | --- | --- | --- | --- |");
    let _ = writeln!(
        report,
        "| Uniform | 0 | | {:.4} | {:.3} | | | |",
        uniform.cross_entropy, uniform.best_move
    );
    for row in &rows {
        let (characters, coded) = row
            .characters
            .map_or((String::new(), String::new()), |(plain, coded)| {
                (plain.to_string(), coded.to_string())
            });
        let _ = writeln!(
            report,
            "| {} | {} | {} | {:.4} | {:.3} | {:.4} | {characters} | {coded} |",
            row.model.name(),
            row.model.weights(),
            row.seen,
            row.held_out.cross_entropy,
            row.held_out.best_move,
            row.rounded.cross_entropy,
        );
    }
    let _ = writeln!(report, "\n## Held-out cross-entropy by epoch\n");
    let _ = writeln!(report, "| Model | By epoch |");
    let _ = writeln!(report, "| --- | --- |");
    for row in &rows {
        let curve: Vec<String> = row
            .epochs
            .iter()
            .map(|value| format!("{value:.4}"))
            .collect();
        let _ = writeln!(report, "| {} | {} |", row.model.name(), curve.join(", "));
    }
    std::fs::write(report_out, &report)
        .map_err(|err| format!("{}: {err}", report_out.display()))?;
    print!("{report}");
    Ok(())
}
