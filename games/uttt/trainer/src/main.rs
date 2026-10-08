//! `uttt-trainer`: self-play data and training for Ultimate Tic-Tac-Toe
//! bots (ADR 0016). Run with `--help` for the commands.

use std::fmt::Write as _;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use cg_core::rng::Rng;
use clap::{Parser, Subcommand};
use uttt_engine::board::CLASSES;

mod data;
mod fit;
mod selfplay;

#[derive(Parser)]
#[command(about = "Self-play data and training for Ultimate Tic-Tac-Toe bots")]
struct Cli {
    #[command(subcommand)]
    command: Command,
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
}

fn main() -> ExitCode {
    let result = match Cli::parse().command {
        Command::Selfplay {
            games,
            iterations,
            opening_plies,
            exploration,
            seed,
            out,
        } => selfplay(
            &selfplay::SelfPlay {
                iterations,
                opening_plies,
                exploration,
            },
            games,
            seed,
            &out,
        ),
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

fn fit_policy(options: &FitOptions) -> Result<(), String> {
    let mut games = Vec::new();
    for path in &options.data {
        let file = File::open(path).map_err(|err| format!("{}: {err}", path.display()))?;
        let mut read = data::read_games(&mut BufReader::new(file))
            .map_err(|err| format!("{}: {err}", path.display()))?;
        games.append(&mut read);
    }
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
