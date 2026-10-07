//! The command line shared by every game's arena binary, with three
//! commands: `match`, `sprt` and `league`.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::thread;

use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};

use crate::ratings::{elo_margins, elo_ratings, MatchupResult};
use crate::referee::RefereeFactory;
use crate::runner::{BotSpec, MatchOptions};
use crate::sprt::{SequentialTest, SprtSettings, Verdict};
use crate::summary::Summary;
use crate::tournament::{self, Flow, GameRecord, Tournament};

/// Plays bots against each other as separate processes, the way CodinGame
/// does, in seat-swapped pairs.
#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Plays two bots and summarizes the results from the first one's side.
    Match(MatchArgs),
    /// Plays a candidate against a baseline until a sequential probability
    /// ratio test decides whether the candidate is stronger.
    Sprt(SprtArgs),
    /// Plays every pair of bots and computes Elo ratings.
    League(LeagueArgs),
}

/// Options every command shares.
#[derive(Debug, clap::Args)]
pub struct CommonArgs {
    /// Seed from which all game and bot seeds are derived.
    #[arg(long, default_value_t = 1)]
    pub seed: u64,

    /// Games played at the same time [default: number of CPUs].
    #[arg(long)]
    pub jobs: Option<usize>,

    /// Write one JSON line per game to this file.
    #[arg(long, value_name = "FILE")]
    pub out: Option<PathBuf>,

    /// Multiply the game's time limits. Bots receive the factor in
    /// CG_TIME_SCALE to scale their own budget.
    #[arg(long, default_value_t = 1.0)]
    pub time_scale: f64,

    /// Moves the referee imposes at the start of each game, to vary the
    /// positions. 0 plays exactly as on CodinGame.
    #[arg(long, default_value_t = 0)]
    pub opening_plies: u32,

    /// Show what bots print to stderr instead of discarding it.
    #[arg(long)]
    pub show_bot_stderr: bool,

    /// Exit with status 1 if any bot times out, crashes or answers invalidly.
    #[arg(long)]
    pub expect_no_faults: bool,
}

#[derive(Debug, clap::Args)]
pub struct MatchArgs {
    /// A bot as NAME=COMMAND. Give exactly two. The command is split on
    /// spaces, for example `v2=target/release/uttt-bot-random`.
    #[arg(long = "bot", value_name = "NAME=COMMAND", required = true)]
    pub bots: Vec<String>,

    /// Number of seat-swapped pairs; twice as many games are played.
    #[arg(long, default_value_t = 100)]
    pub pairs: u32,

    /// Exit with status 1 if the first bot's score is below this, from 0 to 1.
    #[arg(long)]
    pub min_score: Option<f64>,

    #[command(flatten)]
    pub common: CommonArgs,
}

#[derive(Debug, clap::Args)]
pub struct SprtArgs {
    /// The bot under test, as NAME=COMMAND.
    #[arg(long, value_name = "NAME=COMMAND")]
    pub candidate: String,

    /// The bot to beat, as NAME=COMMAND.
    #[arg(long, value_name = "NAME=COMMAND")]
    pub baseline: String,

    /// H0: the candidate is at most this many Elo stronger.
    #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
    pub elo0: f64,

    /// H1: the candidate is at least this many Elo stronger.
    #[arg(long, default_value_t = 10.0, allow_negative_numbers = true)]
    pub elo1: f64,

    /// Probability of accepting a candidate that is not stronger.
    #[arg(long, default_value_t = 0.05)]
    pub alpha: f64,

    /// Probability of rejecting a candidate that is stronger.
    #[arg(long, default_value_t = 0.05)]
    pub beta: f64,

    /// Pairs after which an undecided test stops, as inconclusive.
    #[arg(long, default_value_t = 10_000)]
    pub max_pairs: u32,

    #[command(flatten)]
    pub common: CommonArgs,
}

#[derive(Debug, clap::Args)]
pub struct LeagueArgs {
    /// A bot as NAME=COMMAND. Give at least two.
    #[arg(long = "bot", value_name = "NAME=COMMAND", required = true)]
    pub bots: Vec<String>,

    /// Seat-swapped pairs played by each pair of bots.
    #[arg(long, default_value_t = 100)]
    pub pairs: u32,

    /// The bot rated 0 Elo [default: the first one].
    #[arg(long, value_name = "NAME")]
    pub anchor: Option<String>,

    #[command(flatten)]
    pub common: CommonArgs,
}

/// Runs the arena for one game. `name` is the binary's name for `--help`;
/// `new_referee` builds each game.
///
/// Exit status: 0 on success (for `sprt`: the candidate is accepted), 1 when
/// a check failed (a fault with `--expect-no-faults`, a score below
/// `--min-score`, a candidate rejected or undecided), 2 when the arena could
/// not run.
pub fn main<F: RefereeFactory>(name: &'static str, new_referee: F) -> ExitCode {
    let matches = Cli::command().name(name).get_matches();
    let cli = match Cli::from_arg_matches(&matches) {
        Ok(cli) => cli,
        Err(err) => err.exit(),
    };
    let result = match cli.command {
        Command::Match(args) => run_match(args, &new_referee),
        Command::Sprt(args) => run_sprt(args, &new_referee),
        Command::League(args) => run_league(args, &new_referee),
    };
    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::from(2)
        }
    }
}

fn run_match<F: RefereeFactory>(args: MatchArgs, new_referee: &F) -> Result<bool, String> {
    let [first, second] = two_bots(&args.bots)?;
    let tournament = tournament([first, second], args.pairs, &args.common)?;
    let mut output = Output::create(args.common.out.as_deref())?;
    let mut summary = Summary::new(tournament.bots.clone().map(|bot| bot.name));
    play(&tournament, new_referee, &mut output, |game| {
        summary.add(game);
        Flow::Continue
    })?;
    output.finish()?;

    println!("{summary}");
    let mut passed = faults_ok(&args.common, &summary);
    if let (Some(minimum), Some(score)) = (args.min_score, summary.score()) {
        if score < minimum {
            eprintln!("error: score {score:.3} is below the minimum {minimum}");
            passed = false;
        }
    }
    Ok(passed)
}

fn run_sprt<F: RefereeFactory>(args: SprtArgs, new_referee: &F) -> Result<bool, String> {
    let settings = SprtSettings {
        elo0: args.elo0,
        elo1: args.elo1,
        alpha: args.alpha,
        beta: args.beta,
    };
    settings.validate()?;
    let candidate = BotSpec::parse(&args.candidate)?;
    let baseline = BotSpec::parse(&args.baseline)?;
    if candidate.name == baseline.name {
        return Err("the candidate and the baseline need different names".to_string());
    }
    let tournament = tournament([candidate, baseline], args.max_pairs, &args.common)?;
    let mut output = Output::create(args.common.out.as_deref())?;
    let mut summary = Summary::new(tournament.bots.clone().map(|bot| bot.name));
    let mut test = SequentialTest::new(settings);
    play(&tournament, new_referee, &mut output, |game| {
        summary.add(game);
        match test.add(game) {
            Verdict::Continue => Flow::Continue,
            _ => Flow::Stop,
        }
    })?;
    output.finish()?;

    println!("{summary}");
    let (lower, upper) = settings.bounds();
    println!(
        "SPRT elo0={} elo1={} alpha={} beta={}: LLR {:.2} (bounds {:.2}, {:.2}) after {} pairs",
        settings.elo0,
        settings.elo1,
        settings.alpha,
        settings.beta,
        test.llr(),
        lower,
        upper,
        test.counted_pairs()
    );
    let verdict = test.verdict();
    let [candidate, baseline] = &summary.names;
    println!(
        "Verdict: {}",
        match verdict {
            Verdict::Accepted => format!("ACCEPTED: {candidate} is stronger than {baseline}"),
            Verdict::Rejected => format!("REJECTED: {candidate} is not stronger than {baseline}"),
            Verdict::Continue => format!("INCONCLUSIVE after {} pairs", args.max_pairs),
        }
    );
    Ok(faults_ok(&args.common, &summary) && verdict == Verdict::Accepted)
}

fn run_league<F: RefereeFactory>(args: LeagueArgs, new_referee: &F) -> Result<bool, String> {
    let bots = args
        .bots
        .iter()
        .map(|spec| BotSpec::parse(spec))
        .collect::<Result<Vec<_>, _>>()?;
    if bots.len() < 2 {
        return Err("a league needs at least two --bot options".to_string());
    }
    let names: Vec<String> = bots.iter().map(|bot| bot.name.clone()).collect();
    for (index, name) in names.iter().enumerate() {
        if names[..index].contains(name) {
            return Err(format!("two bots are named {name:?}"));
        }
    }
    let anchor = match &args.anchor {
        Some(name) => names
            .iter()
            .position(|other| other == name)
            .ok_or_else(|| format!("--anchor {name:?} is not one of the bots"))?,
        None => 0,
    };

    let mut output = Output::create(args.common.out.as_deref())?;
    let mut results = Vec::new();
    let mut faults = 0;
    let mut games = vec![0u32; bots.len()];
    for a in 0..bots.len() {
        for b in a + 1..bots.len() {
            let tournament =
                tournament([bots[a].clone(), bots[b].clone()], args.pairs, &args.common)?;
            let mut summary = Summary::new([names[a].clone(), names[b].clone()]);
            play(&tournament, new_referee, &mut output, |game| {
                summary.add(game);
                Flow::Continue
            })?;
            let score = summary.score().unwrap_or(0.5);
            println!(
                "{} vs {}: {:.1}% of {} games ({} wins, {} draws, {} losses)",
                names[a],
                names[b],
                100.0 * score,
                summary.games(),
                summary.wins,
                summary.draws,
                summary.losses
            );
            faults += summary.total_faults();
            games[a] += summary.games();
            games[b] += summary.games();
            results.push(MatchupResult {
                a,
                b,
                score_a: f64::from(summary.wins) + 0.5 * f64::from(summary.draws),
                games: summary.games(),
            });
        }
    }
    output.finish()?;

    let ratings = elo_ratings(bots.len(), &results, anchor);
    let margins = elo_margins(bots.len(), &results, anchor, &ratings);
    let mut order: Vec<usize> = (0..bots.len()).collect();
    order.sort_by(|&x, &y| ratings[y].total_cmp(&ratings[x]));
    println!("\nElo ratings, {} at 0, with 95% intervals:", names[anchor]);
    for (rank, &bot) in order.iter().enumerate() {
        println!(
            "{:>3}. {:<24} {:>+8.1} ± {:<6.1} ({} games)",
            rank + 1,
            names[bot],
            ratings[bot],
            margins[bot],
            games[bot]
        );
    }
    if faults > 0 {
        println!("faults: {faults} games ended by a bot fault");
    }
    Ok(!(args.common.expect_no_faults && faults > 0))
}

/// Parses exactly two differently named bots.
fn two_bots(specs: &[String]) -> Result<[BotSpec; 2], String> {
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

/// A tournament between `bots` with the shared options.
fn tournament(bots: [BotSpec; 2], pairs: u32, common: &CommonArgs) -> Result<Tournament, String> {
    if common.time_scale.is_nan() || common.time_scale <= 0.0 {
        return Err("--time-scale must be positive".to_string());
    }
    Ok(Tournament {
        bots,
        pairs,
        seed: common.seed,
        opening_plies: common.opening_plies,
        jobs: common
            .jobs
            .unwrap_or_else(|| thread::available_parallelism().map_or(1, usize::from)),
        options: MatchOptions {
            time_scale: common.time_scale,
            show_bot_stderr: common.show_bot_stderr,
            ..MatchOptions::default()
        },
    })
}

/// Plays a tournament, writing each game to `output` and reporting progress.
fn play<F: RefereeFactory>(
    tournament: &Tournament,
    new_referee: &F,
    output: &mut Output,
    mut on_game: impl FnMut(&GameRecord) -> Flow,
) -> Result<(), String> {
    let total = tournament.pairs * 2;
    let [first, second] = &tournament.bots;
    eprintln!(
        "{} vs {}: up to {total} games with {} jobs, seed {}",
        first.name, second.name, tournament.jobs, tournament.seed
    );
    let mut played = 0u32;
    tournament::run(tournament, new_referee, |game| {
        output.write(game);
        played += 1;
        if played.is_multiple_of(200) && played < total {
            eprintln!("  {played} games played");
        }
        on_game(game)
    })
    .map_err(|err| err.to_string())
}

/// Whether the faults are acceptable; reports them otherwise.
fn faults_ok(common: &CommonArgs, summary: &Summary) -> bool {
    if common.expect_no_faults && summary.total_faults() > 0 {
        eprintln!(
            "error: {} games ended by a bot fault",
            summary.total_faults()
        );
        return false;
    }
    true
}

/// The optional JSON-lines file of game records.
struct Output {
    file: Option<BufWriter<File>>,
    error: Option<String>,
}

impl Output {
    fn create(path: Option<&Path>) -> Result<Output, String> {
        let file = match path {
            Some(path) => {
                Some(BufWriter::new(File::create(path).map_err(|err| {
                    format!("cannot create {}: {err}", path.display())
                })?))
            }
            None => None,
        };
        Ok(Output { file, error: None })
    }

    fn write(&mut self, game: &GameRecord) {
        if let Some(file) = self.file.as_mut() {
            let written = serde_json::to_writer(&mut *file, game)
                .map_err(|err| err.to_string())
                .and_then(|()| file.write_all(b"\n").map_err(|err| err.to_string()));
            if let Err(err) = written {
                self.error.get_or_insert(err);
            }
        }
    }

    fn finish(mut self) -> Result<(), String> {
        if let Some(file) = self.file.as_mut() {
            file.flush().map_err(|err| err.to_string())?;
        }
        match self.error {
            Some(err) => Err(format!("cannot write results: {err}")),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests;
