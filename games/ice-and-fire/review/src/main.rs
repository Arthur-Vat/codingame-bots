//! `ice-and-fire-review`: turns recorded games into review files, and
//! summarizes a reviewed batch (`cg_review`).
//!
//! ```sh
//! ice-and-fire-review games --out DIR [--text DIR] GAME.json...
//! ice-and-fire-review summary DIR > DIR/summary-data.md
//! ice-and-fire-review report DIR [--curves FILE] > report.md
//! ice-and-fire-review towers [--csv DIR] [--jobs N] GAME.json... > towers.md
//! ice-and-fire-review bank --out FILE [--count N] [--seed S] [--jobs N] GAME.json...
//! ice-and-fire-review screening [--jobs N] GAME.json... > screening.md
//! ice-and-fire-review codingame --out DIR [--kind ranking] [--strict] LOG.json... > check.md
//! ice-and-fire-review codingame-report --data DIR --me NAME [--kinds ranking,test] [--release NAME] [--top N] --out DIR
//! ice-and-fire-review codingame-moments --data DIR --me NAME [--kinds ranking,test] [--release NAME] [--top N] [--examples K] [--out DIR] > moments.md
//! ice-and-fire-review codingame-judge --data DIR --me NAME --judge BIN [--release NAME] [--players A,B | --top N] [--tries N] [--limit PLIES] [--jobs N] --out DIR
//! ```

mod bank;
mod codingame;
mod judge;
mod profiles;
mod profiling;
mod replay;
mod screening;
mod study;
#[cfg(test)]
mod test_util;
mod trace;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use cg_arena::record::Record;
use cg_review::report;
use cg_review::summary::{load, summarize, SUMMARY_SUFFIX};
use cg_review::text::{game_text, metrics_csv};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "ice-and-fire-review",
    about = "Review files of recorded A Code of Ice and Fire games"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Writes, for each record, NAME.md (the game file a reviewer reads),
    /// NAME.csv (the metrics) and NAME.summary.json (the result, phases and
    /// decisive moments), NAME being the record's file name without
    /// `.json`. It refuses to write over any of the records.
    Games {
        /// The directory to write to, created if missing.
        #[arg(long)]
        out: PathBuf,
        /// Where to write the game files instead, created if missing: they
        /// are large and the records rebuild them.
        #[arg(long)]
        text: Option<PathBuf>,
        /// Game records written by the arena (`--records`).
        #[arg(required = true)]
        records: Vec<PathBuf>,
    },
    /// Prints the summary of a batch: the files `games` wrote in DIR, and the
    /// reviewers' notes in NAME.review.md when present.
    Summary {
        /// The batch's directory.
        dir: PathBuf,
    },
    /// Prints the metrics report of the games `games` wrote in DIR (ADR
    /// 0044): from which turn each metric tells the winner, the metrics that
    /// tell it best at a few turns, the two bots compared, and the same by
    /// slice (length, seats, winner, lead at turn 10), from the start and
    /// before the end.
    Report {
        /// The directory of the games' NAME.csv and NAME.summary.json.
        dir: PathBuf,
        /// Also write every metric's points, turn by turn from the start and
        /// before the end, for every slice of the batch, as JSON here, for
        /// charts.
        #[arg(long)]
        curves: Option<PathBuf>,
    },
    /// Prints the tower and threat study of the records: every tower built
    /// and what became of it, every ply's threat to the enemy HQ against
    /// whether it was carried out, and every ply's cuts against what was
    /// taken.
    Towers {
        /// Also write the study's rows as towers.csv, threats.csv and
        /// cuts.csv in this directory, created if missing.
        #[arg(long)]
        csv: Option<PathBuf>,
        /// Records replayed at the same time.
        #[arg(long, default_value_t = 2)]
        jobs: usize,
        /// Game records written by the arena (`--records`).
        #[arg(required = true)]
        records: Vec<PathBuf>,
    },
    /// Writes a bank of middle-game positions drawn from the records, for
    /// the plan bot's analysis mode (`bank` in `ice-and-fire-bot-plan`,
    /// built with `--features analysis`).
    Bank {
        /// The file to write; it may not exist yet.
        #[arg(long)]
        out: PathBuf,
        /// Positions to draw (all of them if there are fewer).
        #[arg(long, default_value_t = 5000)]
        count: usize,
        /// The seed of the draw.
        #[arg(long, default_value_t = 22022)]
        seed: u64,
        /// Records replayed at the same time.
        #[arg(long, default_value_t = 2)]
        jobs: usize,
        /// Game records written by the arena (`--records`).
        #[arg(required = true)]
        records: Vec<PathBuf>,
    },
    /// Prints the screening report of a match's records: per bot, its score
    /// by seat, towers, mines and units by level a game, the gold it kept,
    /// its faults and its think times.
    Screening {
        /// Records replayed at the same time.
        #[arg(long, default_value_t = 2)]
        jobs: usize,
        /// Game records written by the arena (`--records`).
        #[arg(required = true)]
        records: Vec<PathBuf>,
    },
    /// Converts CodinGame logs (the API's JSON logs) into records, replays
    /// each with our engine and prints, as Markdown, where the engine
    /// disagrees with CodinGame: the winner, a command carried out or
    /// refused, a command that is not read.
    Codingame {
        /// Where to write the records, as GAME_ID.json, created if missing.
        #[arg(long)]
        out: PathBuf,
        /// How the games were played: ranking, test or replay.
        #[arg(long, default_value = "ranking", value_parser = ["ranking", "test", "replay"])]
        kind: String,
        /// Exit with 1 when a game disagrees or does not convert.
        #[arg(long)]
        strict: bool,
        /// The logs, `<game_id>.json`.
        #[arg(required = true)]
        logs: Vec<PathBuf>,
    },
    /// Writes the profiles of the players met on CodinGame and our
    /// weaknesses, from a data directory of `scripts/codingame.sh` (logs,
    /// index and leaderboard snapshots): results and ends by opponent, then
    /// for each of the top players its key metrics, events, decisive moments
    /// and openings, then the metrics at which our losses fall behind. Games
    /// that disagree with our engine are left out and listed.
    CodingameReport {
        /// The game's data directory, for example `data/ice-and-fire`.
        #[arg(long)]
        data: PathBuf,
        /// Our nickname on CodinGame, for games whose index has no seat.
        #[arg(long)]
        me: String,
        /// The kinds of games to use, separated by commas.
        #[arg(long, value_delimiter = ',', default_values_t = ["ranking".to_string(), "test".to_string(), "replay".to_string()])]
        kinds: Vec<String>,
        /// Only the games of this batch.
        #[arg(long)]
        batch: Option<String>,
        /// Only our games played by this release (the index's `release`,
        /// which test games carry and ranking games get from
        /// `scripts/codingame.sh fetch --backfill-releases`).
        #[arg(long)]
        release: Option<String>,
        /// Profiles for the top N players of the snapshot that have a game.
        #[arg(long, default_value_t = 10)]
        top: usize,
        /// Profiles for these players instead, separated by commas.
        #[arg(long, value_delimiter = ',')]
        players: Vec<String>,
        /// The leaderboard snapshot, instead of the newest in the data directory.
        #[arg(long)]
        leaderboard: Option<PathBuf>,
        /// Also write the numbers as JSON here.
        #[arg(long)]
        json: Option<PathBuf>,
        /// The folder to write the report's files to, created if missing:
        /// tables as CSV, one SVG chart a metric or event, the data of every
        /// turn, `README.md` and `signals.md`.
        #[arg(long)]
        out: PathBuf,
        /// Games replayed at the same time.
        #[arg(long, default_value_t = 4)]
        jobs: usize,
    },
    /// Prints, for each of our losses on CodinGame, its decisive moment (the
    /// first large cut, HQ threat or debt against us) with the board before
    /// and after it, how exposed we were before it, and what each side
    /// held; a summary by opponent group first, then examples against each
    /// of the top players. Reads the data directory as `codingame-report`.
    CodingameMoments {
        /// The game's data directory, for example `data/ice-and-fire`.
        #[arg(long)]
        data: PathBuf,
        /// Our nickname on CodinGame, for games whose index has no seat.
        #[arg(long)]
        me: String,
        /// The kinds of games to use, separated by commas.
        #[arg(long, value_delimiter = ',', default_values_t = ["ranking".to_string(), "test".to_string(), "replay".to_string()])]
        kinds: Vec<String>,
        /// Only the games of this batch.
        #[arg(long)]
        batch: Option<String>,
        /// Only our games played by this release (see `codingame-report`).
        #[arg(long)]
        release: Option<String>,
        /// The focus players are the top N of the snapshot that we met.
        #[arg(long, default_value_t = 10)]
        top: usize,
        /// Example losses against each focus player.
        #[arg(long, default_value_t = 3)]
        examples: usize,
        /// The leaderboard snapshot, instead of the newest in the data directory.
        #[arg(long)]
        leaderboard: Option<PathBuf>,
        /// Games replayed at the same time.
        #[arg(long, default_value_t = 4)]
        jobs: usize,
        /// Write moments.csv, summary.csv, examples.md and README.md into
        /// this folder, instead of printing the Markdown report.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Writes, to a folder, for every ply of chosen CodinGame players, what the plan
    /// bot's search would play there and how it scores its own plan and the
    /// one played, with the agreement and the score gap by player and phase
    /// and the plies with the largest gaps. Games are read as
    /// `codingame-report` reads them.
    CodingameJudge {
        /// The game's data directory, for example `data/ice-and-fire`.
        #[arg(long)]
        data: PathBuf,
        /// Our nickname on CodinGame, for games whose index has no seat.
        #[arg(long)]
        me: String,
        /// The plan bot built with `--features analysis`
        /// (`target/release/ice-and-fire-bot-plan`).
        #[arg(long)]
        judge: PathBuf,
        /// Only our games played by this release (see `codingame-report`).
        #[arg(long)]
        release: Option<String>,
        /// The top N players of the newest leaderboard snapshot that have a game.
        #[arg(long, default_value_t = 10)]
        top: usize,
        /// These players instead, separated by commas.
        #[arg(long, value_delimiter = ',')]
        players: Vec<String>,
        /// Tries of each search (the plan bot's `--tries`).
        #[arg(long, default_value_t = 600)]
        tries: u64,
        /// Judge at most this many plies, spread over all of them.
        #[arg(long)]
        limit: Option<usize>,
        /// The leaderboard snapshot, instead of the newest in the data directory.
        #[arg(long)]
        leaderboard: Option<PathBuf>,
        /// Judge processes run at the same time.
        #[arg(long, default_value_t = 4)]
        jobs: usize,
        /// The folder to write to, created if missing: judgements.csv,
        /// summary.csv, largest-gaps.md and README.md.
        #[arg(long)]
        out: PathBuf,
    },
}

/// The records in file name order, so that a batch gives the same output
/// whatever order the shell lists them in.
fn sorted(mut records: Vec<PathBuf>) -> Vec<PathBuf> {
    records.sort();
    records.dedup();
    records
}

/// The three files of the record `path`: the game file in `text_dir`, the
/// others in `out`, with what each holds.
fn outputs(path: &Path, out: &Path, text_dir: &Path) -> Result<[PathBuf; 3], String> {
    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format!("{}: no file name", path.display()))?;
    Ok([
        text_dir.join(format!("{name}.md")),
        out.join(format!("{name}.csv")),
        out.join(format!("{name}{SUMMARY_SUFFIX}")),
    ])
}

/// The path to compare files by: canonical when the file exists.
fn same_file_key(path: &Path) -> PathBuf {
    fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

/// Writes the three files of one record.
fn write_game(path: &Path, out: &Path, text_dir: &Path) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let record: Record =
        serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    if record.game != "ice-and-fire" {
        return Err(format!(
            "{}: a record of {}, not of ice-and-fire",
            path.display(),
            record.game
        ));
    }
    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format!("{}: no file name", path.display()))?;
    let trace = trace::trace(name, &record);
    let summary =
        serde_json::to_string_pretty(&trace.summary()).map_err(|e| format!("{name}: {e}"))?;
    let files = outputs(path, out, text_dir)?;
    for (file, body) in files
        .iter()
        .zip([game_text(&trace), metrics_csv(&trace), summary + "\n"])
    {
        fs::write(file, body).map_err(|e| format!("{}: {e}", file.display()))?;
    }
    Ok(())
}

fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Command::Games { out, text, records } => {
            let text = text.unwrap_or_else(|| out.clone());
            for dir in [&out, &text] {
                fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
            }
            // No output may be one of the records.
            let inputs: Vec<PathBuf> = records.iter().map(|p| same_file_key(p)).collect();
            for path in &records {
                for file in outputs(path, &out, &text)? {
                    if inputs.contains(&same_file_key(&file)) {
                        return Err(format!(
                            "{} would be overwritten: write elsewhere with --out or --text",
                            file.display()
                        ));
                    }
                }
            }
            for path in &records {
                write_game(path, &out, &text)?;
            }
            eprintln!("wrote {} games to {}", records.len(), out.display());
            Ok(())
        }
        Command::Report { dir, curves } => {
            let (games, _, problems) = load(&dir)?;
            if games.is_empty() {
                return Err(format!("{}: no game files", dir.display()));
            }
            for problem in &problems {
                eprintln!("left out: {problem}");
            }
            let meanings = trace::metrics()
                .into_iter()
                .map(|m| (m.key, m.meaning))
                .collect();
            print!("{}", report::report(&games, &meanings));
            if let Some(file) = curves {
                let json = serde_json::to_string(&report::sliced(&games))
                    .map_err(|e| format!("curves: {e}"))?;
                fs::write(&file, json).map_err(|e| format!("{}: {e}", file.display()))?;
            }
            Ok(())
        }
        Command::Towers { csv, jobs, records } => {
            let games = replay::map_records(&sorted(records), jobs, study::study)?;
            if let Some(dir) = csv {
                fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
                for (name, text) in study::csv(&games) {
                    let file = dir.join(name);
                    fs::write(&file, text).map_err(|e| format!("{}: {e}", file.display()))?;
                }
            }
            print!("{}", study::report(&games));
            Ok(())
        }
        Command::Bank {
            out,
            count,
            seed,
            jobs,
            records,
        } => {
            if out.exists() {
                return Err(format!("{} exists: write elsewhere", out.display()));
            }
            let blocks = replay::map_records(&sorted(records), jobs, |game| {
                bank::middle_plies(game)
                    .map(|ply| bank::block(game, ply))
                    .collect::<Vec<_>>()
            })?;
            let candidates: Vec<(usize, usize)> = blocks
                .iter()
                .enumerate()
                .flat_map(|(g, b)| (0..b.len()).map(move |i| (g, i)))
                .collect();
            let total = candidates.len();
            let drawn = bank::draw(candidates, count, seed);
            let text: String = drawn.iter().map(|&(g, i)| blocks[g][i].as_str()).collect();
            fs::write(&out, text).map_err(|e| format!("{}: {e}", out.display()))?;
            eprintln!(
                "wrote {} of {total} middle-game positions of {} games to {}",
                drawn.len(),
                blocks.len(),
                out.display()
            );
            Ok(())
        }
        Command::Screening { jobs, records } => {
            let games = replay::map_records(&sorted(records), jobs, screening::sides)?;
            print!("{}", screening::report(&games));
            Ok(())
        }
        Command::Codingame {
            out,
            kind,
            strict,
            logs,
        } => {
            let (text, clean) = codingame::run(&sorted(logs), &out, &kind)?;
            print!("{text}");
            if strict && !clean {
                return Err("some games disagree or did not convert".to_string());
            }
            Ok(())
        }
        Command::CodingameReport {
            data,
            me,
            kinds,
            batch,
            release,
            top,
            players,
            leaderboard,
            json,
            out,
            jobs,
        } => {
            let (numbers, files) = profiles::run(&profiles::Request {
                data,
                me,
                kinds,
                batch,
                release,
                top,
                players,
                leaderboard,
                jobs,
            })?;
            fs::create_dir_all(&out).map_err(|e| format!("{}: {e}", out.display()))?;
            eprintln!("writing {} files to {}", files.len(), out.display());
            for (name, body) in files {
                let file = out.join(name);
                fs::write(&file, body).map_err(|e| format!("{}: {e}", file.display()))?;
            }
            if let Some(file) = json {
                fs::write(&file, numbers).map_err(|e| format!("{}: {e}", file.display()))?;
            }
            Ok(())
        }
        Command::CodingameMoments {
            data,
            me,
            kinds,
            batch,
            release,
            top,
            examples,
            leaderboard,
            jobs,
            out,
        } => {
            let text = profiles::run_moments(
                &profiles::Request {
                    data,
                    me,
                    kinds,
                    batch,
                    release,
                    top,
                    players: Vec::new(),
                    leaderboard,
                    jobs,
                },
                examples,
                out.as_deref(),
            )?;
            print!("{text}");
            Ok(())
        }
        Command::CodingameJudge {
            data,
            me,
            judge,
            release,
            top,
            players,
            tries,
            limit,
            leaderboard,
            jobs,
            out,
        } => {
            judge::run(&judge::Request {
                data,
                me,
                judge,
                players,
                release,
                top,
                tries,
                limit,
                jobs,
                leaderboard,
                out,
            })?;
            Ok(())
        }
        Command::Summary { dir } => {
            let (games, notes, problems) = load(&dir)?;
            if games.is_empty() {
                return Err(format!("{}: no game files", dir.display()));
            }
            print!("{}", summarize(&games, &notes, &problems));
            Ok(())
        }
    }
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("ice-and-fire-review: {err}");
            ExitCode::FAILURE
        }
    }
}
