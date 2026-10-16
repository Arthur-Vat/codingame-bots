//! The command line shared by every game's arena binary, with four
//! commands: `match`, `sprt`, `league` and `leaderboard`.

use std::collections::{BTreeSet, HashMap};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::thread;
use std::time::Duration;

use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};

use crate::leaderboard;
use crate::league::{self, PairingResult, Rating};

use crate::ratings::{elo_margins, elo_ratings, MatchupResult};
use crate::record::{Record, Sampler};
use crate::referee::RefereeFactory;
use crate::runner::{BotSpec, MatchOptions};
use crate::sprt::{SequentialTest, SprtSettings, Verdict};
use crate::summary::{allowed_timeouts, Faults, Summary};
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
    /// Plays every pair of bots and computes Elo ratings; with `--results`,
    /// plays only a new release's pairings not yet stored.
    League(LeagueArgs),
    /// Writes the game's LEADERBOARD.md from a league results file, or
    /// checks that a committed one matches it.
    Leaderboard(LeaderboardArgs),
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

    /// Write each kept game, with every answer, as a JSON file in this
    /// directory, created if missing. Files are named
    /// GAME-SEED-pPAIR-SWAPPED.json (SWAPPED is 1 for the second game of a
    /// pair). A league adds the number of the bots' matchup, as in
    /// GAME-SEED-m0v1-p3-1.json. Existing files are never overwritten: use a
    /// new or empty directory.
    #[arg(long, value_name = "DIR")]
    pub records: Option<PathBuf>,

    /// Keep about N games per pair of bots (each matchup of a league), a
    /// mix of wins, draws and losses of the first bot, plus every game lost
    /// by a fault. Without it, --records keeps every game.
    #[arg(long, value_name = "N", requires = "records")]
    pub records_sample: Option<usize>,

    /// Multiply the game's time limits. Bots receive the factor in
    /// CG_TIME_SCALE to scale their own budget.
    #[arg(long, default_value_t = 1.0)]
    pub time_scale: f64,

    /// Milliseconds added to every scaled time limit, without telling the
    /// bots, to absorb the delays of a busy machine. 0 keeps CodinGame's
    /// limits.
    #[arg(long, value_name = "MS", default_value_t = 0)]
    pub time_tolerance_ms: u64,

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

    /// Exit with status 1 if this bot times out, crashes or answers
    /// invalidly; the other bots' faults only lose them their games.
    #[arg(long, value_name = "NAME")]
    pub expect_no_faults_from: Option<String>,

    /// Timeouts that `--expect-no-faults` and `--expect-no-faults-from`
    /// tolerate, as a fraction of each bot's games, rounded down: 0.01
    /// allows 10 in 1,000 games. A timeout still loses its game. Crashes
    /// and invalid answers are never tolerated.
    #[arg(
        long,
        value_name = "RATE",
        default_value_t = 0.0,
        allow_negative_numbers = true
    )]
    pub max_timeout_rate: f64,
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

    /// Exit with status 1 if the first bot is clearly weaker: the 95%
    /// interval of its Elo advantage lies entirely below 0.
    #[arg(long)]
    pub expect_not_worse: bool,

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

    /// The bot rated 0 Elo [default: the first one]. With --results it may be
    /// a bot that appears only in the results file.
    #[arg(long, value_name = "NAME")]
    pub anchor: Option<String>,

    /// Plays incrementally: reads the pairings stored in this JSON-lines
    /// file (see docs/adr/0038-incremental-league.md), plays only the
    /// pairings of the release --new not yet stored, and rates every bot of
    /// the file and of the --bot options over stored and new pairings. The
    /// file must exist; an empty one is fine. Every bot of the file other
    /// than --new needs a --bot, since any of them may be chosen.
    #[arg(long, value_name = "FILE", requires_all = ["new", "new_results"])]
    pub results: Option<PathBuf>,

    /// The release being rated, one of the --bot options. It plays its
    /// opponents as the first bot of each pairing; if the results file
    /// already holds a pairing of it, no new pairing is played.
    #[arg(long, value_name = "NAME", requires = "results")]
    pub new: Option<String>,

    /// Creates (truncates) this file and appends each new pairing to it as
    /// one line of the results file's format, flushed at once, so that a
    /// crash keeps the pairings already finished.
    #[arg(long, value_name = "FILE", requires = "results")]
    pub new_results: Option<PathBuf>,

    /// The previous release, played first. Without one (the first release),
    /// the new release's rating is taken to be the anchor's, 0, when the
    /// nearest opponents are chosen.
    #[arg(long, value_name = "NAME", requires = "results")]
    pub previous: Option<String>,

    /// Opponents played whatever their rating, in this order, separated by
    /// commas (the workflow gives `greedy,v001`).
    #[arg(
        long,
        value_name = "NAME,NAME",
        value_delimiter = ',',
        requires = "results"
    )]
    pub fixed_opponents: Vec<String>,

    /// How many more opponents the new release plays: the known bots whose
    /// rating is closest to its own, among those linked to the anchor. When
    /// the previous release, the fixed opponents and these cover every known
    /// bot, all of them are played.
    #[arg(long, value_name = "N", default_value_t = 5, requires = "results")]
    pub nearest: usize,

    /// Replays one stored pairing, as A,B in either order, after the new
    /// pairings, and reports whether its score is within the interval of the
    /// stored one: a sign that the machine's speed has changed. It is never
    /// written to --new-results, and a result outside the interval prints a
    /// line starting `warning: canary` without changing the exit status.
    #[arg(long, value_name = "A,B", requires = "results")]
    pub canary: Option<String>,

    /// Written to each new pairing's line as `commit`.
    #[arg(long, value_name = "SHA", requires = "results")]
    pub commit: Option<String>,

    /// Prints the opponents chosen (estimated from the previous release's
    /// rating) and the canary, and exits without playing.
    #[arg(long, requires = "results")]
    pub plan: bool,

    #[command(flatten)]
    pub common: CommonArgs,
}

#[derive(Debug, clap::Args)]
pub struct LeaderboardArgs {
    /// The league results file (see docs/adr/0038-incremental-league.md),
    /// read with the checks `league --results` makes. Bots with a file
    /// `releases/GAME-NAME.rs` next to it are releases.
    #[arg(long, value_name = "FILE")]
    pub results: PathBuf,

    /// The bot rated 0 Elo. It must appear in the results file unless the
    /// file is empty.
    #[arg(long, value_name = "NAME")]
    pub anchor: String,

    /// The game's journal index: the table rows whose last column says
    /// Promoted: `GAME-vNNN` give a release's experiment and date.
    #[arg(long, value_name = "FILE")]
    pub journal: PathBuf,

    /// Write the Markdown to this file instead of standard output.
    #[arg(long, value_name = "FILE", conflicts_with = "check")]
    pub out: Option<PathBuf>,

    /// Write nothing; exit with status 1 unless this file holds exactly the
    /// generated Markdown.
    #[arg(long, value_name = "FILE")]
    pub check: Option<PathBuf>,
}

/// Runs the arena for one game. `name` is the binary's name for `--help`;
/// the game's id in game records is `name` without a trailing `-arena`
/// (`uttt-arena` records the game `uttt`). `new_referee` builds each game.
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
    let game = game_id(name);
    let result = match cli.command {
        Command::Match(args) => run_match(args, game, &new_referee),
        Command::Sprt(args) => run_sprt(args, game, &new_referee),
        Command::League(args) => run_league(args, game, &new_referee),
        Command::Leaderboard(args) => run_leaderboard(&args, game, name),
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

/// The game's id in game records: the binary's name without `-arena`.
fn game_id(binary: &str) -> &str {
    binary.strip_suffix("-arena").unwrap_or(binary)
}

fn run_match<F: RefereeFactory>(
    args: MatchArgs,
    game: &str,
    new_referee: &F,
) -> Result<bool, String> {
    let [first, second] = two_bots(&args.bots)?;
    check_fault_name(&args.common, &[first.name.clone(), second.name.clone()])?;
    let tournament = tournament([first, second], args.pairs, &args.common)?;
    let mut output = Output::create(args.common.out.as_deref())?;
    let mut records = Records::create(&args.common, game, "arena match")?;
    let mut summary = Summary::new(tournament.bots.clone().map(|bot| bot.name));
    play(
        &tournament,
        new_referee,
        &mut output,
        &mut records,
        |game| {
            summary.add(game);
            Flow::Continue
        },
    )?;
    output.finish()?;
    let records_result = records.finish();

    println!("{summary}");
    let mut passed = faults_ok(&args.common, &summary);
    if let (Some(minimum), Some(score)) = (args.min_score, summary.score()) {
        if score < minimum {
            eprintln!("error: score {score:.3} is below the minimum {minimum}");
            passed = false;
        }
    }
    if args.expect_not_worse && summary.is_clearly_worse() {
        let [first, second] = &summary.names;
        eprintln!("error: {first} is clearly weaker than {second}");
        passed = false;
    }
    records_result.map(|()| passed)
}

fn run_sprt<F: RefereeFactory>(
    args: SprtArgs,
    game: &str,
    new_referee: &F,
) -> Result<bool, String> {
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
    check_fault_name(
        &args.common,
        &[candidate.name.clone(), baseline.name.clone()],
    )?;
    let tournament = tournament([candidate, baseline], args.max_pairs, &args.common)?;
    let mut output = Output::create(args.common.out.as_deref())?;
    let mut records = Records::create(&args.common, game, "arena sprt")?;
    let mut summary = Summary::new(tournament.bots.clone().map(|bot| bot.name));
    let mut test = SequentialTest::new(settings);
    play(
        &tournament,
        new_referee,
        &mut output,
        &mut records,
        |game| {
            summary.add(game);
            match test.add(game) {
                Verdict::Continue => Flow::Continue,
                _ => Flow::Stop,
            }
        },
    )?;
    output.finish()?;
    let records_result = records.finish();

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
    let passed = faults_ok(&args.common, &summary) && verdict == Verdict::Accepted;
    records_result.map(|()| passed)
}

fn run_league<F: RefereeFactory>(
    args: LeagueArgs,
    game: &str,
    new_referee: &F,
) -> Result<bool, String> {
    let bots = args
        .bots
        .iter()
        .map(|spec| BotSpec::parse(spec))
        .collect::<Result<Vec<_>, _>>()?;
    if args.results.is_some() {
        return incremental::run_incremental_league(&args, &bots, game, new_referee);
    }
    if bots.len() < 2 {
        return Err("a league needs at least two --bot options".to_string());
    }
    let names: Vec<String> = bots.iter().map(|bot| bot.name.clone()).collect();
    check_unique(&names)?;
    check_fault_name(&args.common, &names)?;
    let anchor = match &args.anchor {
        Some(name) => names
            .iter()
            .position(|other| other == name)
            .ok_or_else(|| format!("--anchor {name:?} is not one of the bots"))?,
        None => 0,
    };

    let mut output = Output::create(args.common.out.as_deref())?;
    let mut records = Records::create(&args.common, game, "arena league")?;
    let mut results = Vec::new();
    let mut faults = vec![Faults::default(); bots.len()];
    let mut games = vec![0u32; bots.len()];
    for a in 0..bots.len() {
        for b in a + 1..bots.len() {
            let tournament =
                tournament([bots[a].clone(), bots[b].clone()], args.pairs, &args.common)?;
            let mut summary = Summary::new([names[a].clone(), names[b].clone()]);
            records.start_matchup(&format!("m{a}v{b}"));
            play(
                &tournament,
                new_referee,
                &mut output,
                &mut records,
                |game| {
                    summary.add(game);
                    Flow::Continue
                },
            )?;
            print_pairing(&summary);
            for (bot, found) in [a, b].into_iter().zip(summary.faults) {
                faults[bot].timeouts += found.timeouts;
                faults[bot].crashes += found.crashes;
                faults[bot].invalid += found.invalid;
            }
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
    let records_result = records.finish();

    let ratings = elo_ratings(bots.len(), &results, anchor);
    let margins = elo_margins(bots.len(), &results, anchor, &ratings);
    let table: Vec<Rating> = (0..bots.len())
        .map(|bot| Rating {
            name: names[bot].clone(),
            rating: ratings[bot],
            margin: margins[bot],
            games: games[bot],
        })
        .collect();
    print_ratings(&table, &names[anchor]);
    let passed = faults_pass(&args.common, &names, &faults, &games);
    records_result.map(|()| passed)
}

/// The `leaderboard` command: generates the game's `LEADERBOARD.md` from the
/// results file and prints it, writes it to `--out`, or compares it with the
/// file `--check` names. `binary` is the arena's name, for the messages.
fn run_leaderboard(args: &LeaderboardArgs, game: &str, binary: &str) -> Result<bool, String> {
    let pairings = league::read_results(&args.results)?;
    let journal = fs::read_to_string(&args.journal)
        .map_err(|err| format!("cannot read {}: {err}", args.journal.display()))?;
    let ratings = if pairings.is_empty() {
        Vec::new()
    } else {
        if !pairings
            .iter()
            .any(|pairing| pairing.involves(&args.anchor))
        {
            return Err(format!(
                "--anchor {:?} is not in {}",
                args.anchor,
                args.results.display()
            ));
        }
        let names = league::known_bots(&pairings, [args.anchor.as_str()]);
        league::rate(&names, &pairings, &args.anchor)?
    };
    let folder = args.results.parent().unwrap_or(Path::new(""));
    let releases: BTreeSet<String> = ratings
        .iter()
        .map(|rating| &rating.name)
        .filter(|name| !name.contains(['/', '\\']))
        .filter(|name| {
            folder
                .join("releases")
                .join(format!("{game}-{name}.rs"))
                .is_file()
        })
        .cloned()
        .collect();
    let board = leaderboard::render(game, &pairings, &ratings, &args.anchor, &journal, &releases)?;
    for warning in &board.warnings {
        eprintln!("warning: {warning}");
    }
    if let Some(path) = &args.check {
        if fs::read(path).is_ok_and(|bytes| bytes == board.text.as_bytes()) {
            return Ok(true);
        }
        eprintln!(
            "error: {} is not the leaderboard of {}; regenerate it with: {binary} leaderboard \
             --results {} --anchor {} --journal {} --out {}",
            path.display(),
            args.results.display(),
            args.results.display(),
            args.anchor,
            args.journal.display(),
            path.display()
        );
        return Ok(false);
    }
    match &args.out {
        Some(path) => fs::write(path, board.text.as_bytes())
            .map_err(|err| format!("cannot write {}: {err}", path.display()))?,
        None => io::stdout()
            .write_all(board.text.as_bytes())
            .map_err(|err| format!("cannot write to standard output: {err}"))?,
    }
    Ok(true)
}

/// Errors when two names are the same.
fn check_unique(names: &[String]) -> Result<(), String> {
    for (index, name) in names.iter().enumerate() {
        if names[..index].contains(name) {
            return Err(format!("two bots are named {name:?}"));
        }
    }
    Ok(())
}

/// Prints the result of one pairing of a league.
fn print_pairing(summary: &Summary) {
    println!(
        "{} vs {}: {:.1}% of {} games ({} wins, {} draws, {} losses)",
        summary.names[0],
        summary.names[1],
        100.0 * summary.score().unwrap_or(0.5),
        summary.games(),
        summary.wins,
        summary.draws,
        summary.losses
    );
}

/// Prints the ratings table, best first; bots with no chain of pairings to
/// the anchor come last, without a rating.
fn print_ratings(table: &[Rating], anchor: &str) {
    let mut order: Vec<&Rating> = table.iter().collect();
    order.sort_by(|x, y| {
        y.is_linked()
            .cmp(&x.is_linked())
            .then_with(|| y.rating.total_cmp(&x.rating))
    });
    println!("\nElo ratings, {anchor} at 0, with 95% intervals:");
    for (rank, rating) in order.iter().enumerate() {
        if rating.is_linked() {
            println!(
                "{:>3}. {:<24} {:>+8.1} ± {:<6.1} ({} games)",
                rank + 1,
                rating.name,
                rating.rating,
                rating.margin,
                rating.games
            );
        } else {
            println!(
                "{:>3}. {:<24} {:>8} ± {:<6} ({} games)",
                rank + 1,
                rating.name,
                "n/a",
                "inf",
                rating.games
            );
        }
    }
}

/// Reports the faults of a league's bots and whether the checked ones are
/// acceptable. `games` are the games each bot played in this run.
fn faults_pass(common: &CommonArgs, names: &[String], faults: &[Faults], games: &[u32]) -> bool {
    let mut passed = true;
    for ((name, found), &played) in names.iter().zip(faults).zip(games) {
        if found.total() > 0 {
            println!("faults: {name} lost {} games by a fault", found.total());
            let checked =
                common.expect_no_faults || common.expect_no_faults_from.as_ref() == Some(name);
            if checked && !found.within(played, common.max_timeout_rate) {
                passed = false;
            }
        }
    }
    passed
}

/// Checks that `--expect-no-faults-from` names one of `names`.
fn check_fault_name(common: &CommonArgs, names: &[String]) -> Result<(), String> {
    match &common.expect_no_faults_from {
        Some(name) if !names.contains(name) => Err(format!(
            "--expect-no-faults-from {name:?} is not one of the bots"
        )),
        _ => Ok(()),
    }
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

/// Checks the values of the shared options.
fn check_common(common: &CommonArgs) -> Result<(), String> {
    if common.time_scale.is_nan() || common.time_scale <= 0.0 {
        return Err("--time-scale must be positive".to_string());
    }
    if !(0.0..1.0).contains(&common.max_timeout_rate) {
        return Err("--max-timeout-rate must be at least 0 and below 1".to_string());
    }
    Ok(())
}

/// A tournament between `bots` with the shared options.
fn tournament(bots: [BotSpec; 2], pairs: u32, common: &CommonArgs) -> Result<Tournament, String> {
    check_common(common)?;
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
            time_tolerance: Duration::from_millis(common.time_tolerance_ms),
            show_bot_stderr: common.show_bot_stderr,
            ..MatchOptions::default()
        },
    })
}

/// Plays a tournament, writing each game to `output` and the kept ones to
/// `records`, and reporting progress.
fn play<F: RefereeFactory>(
    tournament: &Tournament,
    new_referee: &F,
    output: &mut Output,
    records: &mut Records,
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
        records.write(tournament, game);
        played += 1;
        if played.is_multiple_of(200) && played < total {
            eprintln!("  {played} games played");
        }
        on_game(game)
    })
    .map_err(|err| err.to_string())
}

/// Whether the checked bots' faults are acceptable: none, except timeouts
/// within `--max-timeout-rate`. Reports them otherwise.
fn faults_ok(common: &CommonArgs, summary: &Summary) -> bool {
    let games = summary.games();
    let allowed = allowed_timeouts(games, common.max_timeout_rate);
    let mut ok = true;
    for (name, faults) in summary.names.iter().zip(&summary.faults) {
        let checked =
            common.expect_no_faults || common.expect_no_faults_from.as_ref() == Some(name);
        if checked && !faults.within(games, common.max_timeout_rate) {
            eprintln!(
                "error: {name} lost {} games by a fault ({} timeouts, {} crashes, {} invalid \
                 answers; {allowed} timeouts allowed in {games} games)",
                faults.total(),
                faults.timeouts,
                faults.crashes,
                faults.invalid
            );
            ok = false;
        }
    }
    ok
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

/// The optional directory of game records: one JSON file per kept game.
struct Records {
    dir: Option<PathBuf>,
    game: String,
    source: &'static str,
    limit: Option<usize>,
    sampler: Sampler,
    /// The start of every file name in the current tournament.
    stem: String,
    seed: u64,
    error: Option<String>,
}

impl Records {
    /// Prepares the directory `--records` names, if any.
    fn create(common: &CommonArgs, game: &str, source: &'static str) -> Result<Records, String> {
        if common.records_sample.is_some() && common.records.is_none() {
            return Err("--records-sample needs --records".to_string());
        }
        if let Some(dir) = &common.records {
            fs::create_dir_all(dir)
                .map_err(|err| format!("cannot create {}: {err}", dir.display()))?;
        }
        Ok(Records {
            dir: common.records.clone(),
            game: game.to_string(),
            source,
            limit: common.records_sample,
            sampler: Sampler::new(common.records_sample),
            stem: format!("{game}-{}", common.seed),
            seed: common.seed,
            error: None,
        })
    }

    /// Starts the games of one pair of bots in a league: each pair gets its
    /// own sample and its own file names.
    fn start_matchup(&mut self, tag: &str) {
        self.sampler = Sampler::new(self.limit);
        self.stem = format!("{}-{}-{tag}", self.game, self.seed);
    }

    fn write(&mut self, tournament: &Tournament, game: &GameRecord) {
        let Some(dir) = &self.dir else { return };
        if !self.sampler.keep(game) {
            return;
        }
        let [first, second] = &tournament.bots;
        let seat_bots = if game.swapped {
            [second, first]
        } else {
            [first, second]
        };
        let record = Record::from_match(
            &self.game,
            self.source,
            tournament.opening_plies,
            &game.game,
            &tournament.options,
            seat_bots,
        );
        let path = dir.join(record_file_name(&self.stem, game));
        let written = serde_json::to_vec(&record)
            .map_err(|err| err.to_string())
            .and_then(|json| write_new_file(&path, &json));
        if let Err(err) = written {
            self.error
                .get_or_insert(format!("{}: {err}", path.display()));
        }
    }

    fn finish(self) -> Result<(), String> {
        match self.error {
            Some(err) => Err(format!("cannot write game records: {err}")),
            None => Ok(()),
        }
    }
}

/// Writes `bytes` to a file that must not exist yet: a record is never
/// overwritten.
fn write_new_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|err| {
            if err.kind() == io::ErrorKind::AlreadyExists {
                "the file exists; use a new or empty directory for --records".to_string()
            } else {
                err.to_string()
            }
        })?;
    file.write_all(bytes).map_err(|err| err.to_string())
}

/// `STEM-pPAIR-0.json`, or `-1` for the swapped game of the pair.
fn record_file_name(stem: &str, game: &GameRecord) -> String {
    format!("{stem}-p{}-{}.json", game.pair, u8::from(game.swapped))
}

mod incremental;

#[cfg(test)]
mod tests;
