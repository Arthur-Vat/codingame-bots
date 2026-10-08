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
use uttt_engine::value::ValueNetwork;
use uttt_engine::PlayoutPolicy;
use value_fit::Examples as _;

mod data;
mod duel;
mod fit;
mod selfplay;
mod value_fit;

#[derive(Parser)]
#[command(about = "Self-play data and training for Ultimate Tic-Tac-Toe bots")]
struct Cli {
    #[command(subcommand)]
    command: Command,
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
        /// Pairs of games between a search with the network and one with
        /// the playouts, at equal iterations: the gate of ADR 0018. 0 for
        /// none.
        #[arg(long, default_value_t = 400)]
        duel_pairs: u32,
        /// Iterations per move in those games.
        #[arg(long, default_value_t = 3_000)]
        duel_iterations: u64,
        /// The exploration constant of the search with the network in those
        /// games; the one with playouts keeps the bot's 0.5.
        #[arg(long, default_value_t = 0.5)]
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
            policy_plies,
            no_visits,
            seed,
            out,
        } => match policy
            .map(|path| load_policy(&path, policy_plies))
            .transpose()
        {
            Ok(policy) => selfplay(
                &selfplay::SelfPlay {
                    iterations,
                    opening_plies,
                    exploration,
                    policy,
                    record_visits: !no_visits,
                },
                games,
                seed,
                &out,
            ),
            Err(message) => Err(message),
        },
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
            duel_pairs,
            duel_iterations,
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
            duel_pairs,
            duel_iterations,
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
    duel_pairs: u32,
    duel_iterations: u64,
    duel_exploration: f64,
    weights_out: PathBuf,
    report_out: PathBuf,
    origin: String,
}

fn fit_value(options: &ValueOptions) -> Result<(), String> {
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

    let initial = value_fit::initial_parameters(settings.seed);
    let initial_metrics = value_fit::measure(&initial, held_out, threads);
    let mut epochs = Vec::new();
    let parameters = value_fit::train(initial, fitted, held_out, settings, |epoch| {
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
    let (rounded, text) = value_fit::quantize(&parameters);
    std::fs::write(
        &options.weights_out,
        value_fit::weights_source(&text, &options.origin),
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
    let network = value_fit::measure(&parameters, &compared[..], threads);
    let network_rounded = value_fit::measure(&rounded, &compared[..], threads);
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
    let held_out_rounded = value_fit::measure(&rounded, held_out, threads);
    let duel = duel::Duel {
        iterations: options.duel_iterations,
        opening_plies: 6,
        exploration: 0.5,
        network_exploration: options.duel_exploration,
    };
    let duel_results = (options.duel_pairs > 0).then(|| {
        eprintln!(
            "duel: {} pairs at {} iterations per move",
            options.duel_pairs, options.duel_iterations
        );
        let network: &'static ValueNetwork = Box::leak(Box::new(
            ValueNetwork::from_parameters(&rounded).expect("the trainer's layout"),
        ));
        duel.play(network, policy, options.duel_pairs, settings.seed, threads)
    });

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
        "- Network: 217-64-16-1, {} parameters; weights rounded to 8 bits, {} characters of base64",
        value_fit::PARAMETERS,
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
    match &duel_results {
        Some(results) => {
            let (verdict, why) = if results.not_clearly_weaker() {
                ("passes", "not clearly weaker")
            } else {
                ("fails", "clearly weaker")
            };
            let _ = writeln!(
                report,
                "\n**Gate of ADR 0018: {verdict}.** At {} iterations per move, the network's search scored {} against the playouts' over {} pairs of games: {why}.",
                duel.iterations,
                elo(results),
                options.duel_pairs
            );
        }
        None => {
            let _ = writeln!(
                report,
                "\n**Gate of ADR 0018: not measured**, no games played."
            );
        }
    }
    if let Some(results) = &duel_results {
        let _ = writeln!(
            report,
            "\n## Against the playouts, at equal iterations\n\nA search with the network (weights rounded, exploration {}) against one with the playouts above (exploration {}), {} iterations per move each, {} pairs of games from {} random opening moves: {} wins, {} draws, {} losses for the network, {}. Pairs by the network's points (0, ½, 1, 1½, 2): {:?}.",
            duel.network_exploration,
            duel.exploration,
            duel.iterations,
            options.duel_pairs,
            duel.opening_plies,
            results.wins,
            results.draws,
            results.losses,
            elo(results),
            results.pairs
        );
    }
    let _ = writeln!(
        report,
        "\n## Predictions on held-out positions\n\nOn {} positions of held-out games, against the game's result for the side to move. Measured this way, the network with rounded weights is worth {worth}. A single playout's error is mostly its own noise, which a search averages away, so only the games above judge the network (ADR 0018).\n",
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
