//! The command line shared by every game's arena binary.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::thread;

use clap::{CommandFactory, FromArgMatches, Parser};

use crate::referee::Referee;
use crate::runner::{BotSpec, MatchOptions};
use crate::summary::Summary;
use crate::tournament::{self, Tournament};

/// Plays two bots against each other in seat-swapped pairs and prints a
/// summary from the first bot's point of view.
#[derive(Debug, Parser)]
pub struct Args {
    /// A bot as NAME=COMMAND. Give exactly two. The command is split on
    /// spaces, for example `v2=target/release/uttt-bot-random`.
    #[arg(long = "bot", value_name = "NAME=COMMAND", required = true)]
    pub bots: Vec<String>,

    /// Number of seat-swapped pairs; twice as many games are played.
    #[arg(long, default_value_t = 100)]
    pub pairs: u32,

    /// Seed from which all game and bot seeds are derived.
    #[arg(long, default_value_t = 1)]
    pub seed: u64,

    /// Games played at the same time [default: number of CPUs].
    #[arg(long)]
    pub jobs: Option<usize>,

    /// Write one JSON line per game to this file.
    #[arg(long, value_name = "FILE")]
    pub out: Option<PathBuf>,

    /// Multiply the game's time limits, to tolerate slow machines.
    #[arg(long, default_value_t = 1.0)]
    pub time_scale: f64,

    /// Show what bots print to stderr instead of discarding it.
    #[arg(long)]
    pub show_bot_stderr: bool,

    /// Exit with status 1 if any bot times out, crashes or answers invalidly.
    #[arg(long)]
    pub expect_no_faults: bool,
}

/// Runs the arena for one game. `name` is the binary's name for `--help`;
/// `new_referee` builds a game from its seed.
///
/// Exit status: 0 on success, 1 when `--expect-no-faults` saw a fault,
/// 2 when the arena could not run.
pub fn main<F>(name: &'static str, new_referee: F) -> ExitCode
where
    F: Fn(u64) -> Box<dyn Referee> + Sync,
{
    let matches = Args::command().name(name).get_matches();
    let args = match Args::from_arg_matches(&matches) {
        Ok(args) => args,
        Err(err) => err.exit(),
    };
    match run(args, &new_referee) {
        Ok(code) => code,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::from(2)
        }
    }
}

fn run<F>(args: Args, new_referee: &F) -> Result<ExitCode, String>
where
    F: Fn(u64) -> Box<dyn Referee> + Sync,
{
    let bots = parse_bots(&args.bots)?;
    if args.time_scale.is_nan() || args.time_scale <= 0.0 {
        return Err("--time-scale must be positive".to_string());
    }
    let jobs = args
        .jobs
        .unwrap_or_else(|| thread::available_parallelism().map_or(1, usize::from));
    let tournament = Tournament {
        bots,
        pairs: args.pairs,
        seed: args.seed,
        jobs,
        options: MatchOptions {
            time_scale: args.time_scale,
            show_bot_stderr: args.show_bot_stderr,
            ..MatchOptions::default()
        },
    };

    let mut out = match &args.out {
        Some(path) => {
            Some(BufWriter::new(File::create(path).map_err(|err| {
                format!("cannot create {}: {err}", path.display())
            })?))
        }
        None => None,
    };
    let mut summary = Summary::new(tournament.bots.clone().map(|bot| bot.name));
    let total = args.pairs * 2;
    let mut write_error = None;
    eprintln!(
        "playing {total} games ({} pairs) with {jobs} jobs, seed {}",
        args.pairs, args.seed
    );
    tournament::run(&tournament, new_referee, |game| {
        summary.add(game);
        if let Some(out) = out.as_mut() {
            let written = serde_json::to_writer(&mut *out, game)
                .map_err(|err| err.to_string())
                .and_then(|()| out.write_all(b"\n").map_err(|err| err.to_string()));
            if let Err(err) = written {
                write_error.get_or_insert(err);
            }
        }
        let done = summary.games();
        if done.is_multiple_of(100) && done < total {
            eprintln!("{done}/{total} games");
        }
    })
    .map_err(|err| err.to_string())?;
    if let Some(out) = out.as_mut() {
        out.flush().map_err(|err| err.to_string())?;
    }
    if let Some(err) = write_error {
        return Err(format!("cannot write results: {err}"));
    }

    println!("{summary}");
    if args.expect_no_faults && summary.total_faults() > 0 {
        eprintln!(
            "error: {} games ended by a bot fault",
            summary.total_faults()
        );
        return Ok(ExitCode::from(1));
    }
    Ok(ExitCode::SUCCESS)
}

fn parse_bots(specs: &[String]) -> Result<[BotSpec; 2], String> {
    let bots: Vec<BotSpec> = specs
        .iter()
        .map(|spec| BotSpec::parse(spec))
        .collect::<Result<_, _>>()?;
    let [first, second]: [BotSpec; 2] = bots.try_into().map_err(|bots: Vec<BotSpec>| {
        format!("give exactly two --bot options, not {}", bots.len())
    })?;
    if first.name == second.name {
        return Err(format!(
            "both bots are named {:?}; give them different names",
            first.name
        ));
    }
    Ok([first, second])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_command_line_is_well_formed() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }

    #[test]
    fn needs_two_differently_named_bots() {
        let one = vec!["a=bot".to_string()];
        assert!(parse_bots(&one).unwrap_err().contains("exactly two"));
        let same = vec!["a=bot".to_string(), "a=other".to_string()];
        assert!(parse_bots(&same).unwrap_err().contains("different names"));
        let ok = vec!["a=bot".to_string(), "b=bot --x".to_string()];
        assert_eq!(parse_bots(&ok).unwrap()[1].args, vec!["--x".to_string()]);
    }
}
