//! `ice-and-fire-review`: turns recorded games into review files, and
//! summarizes a reviewed batch (`cg_review`).
//!
//! ```sh
//! ice-and-fire-review games --out DIR [--text DIR] GAME.json...
//! ice-and-fire-review summary DIR > DIR/summary-data.md
//! ice-and-fire-review excerpts --out DIR [--text DIR] [--moves 3] GAME.json...
//! ice-and-fire-review excerpt-summary DIR > DIR/summary-data.md
//! ```

mod trace;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use cg_arena::record::Record;
use cg_review::excerpt::{excerpt_summary, excerpt_text, games_row, primer_text, GAMES_HEADER};
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
    /// Writes, for each record, NAME.excerpt.md: each side's last moves up
    /// to the game's decisive moment (ADR 0044), with primer.md, the rules
    /// the excerpts share, beside them; and games.csv, a row per game (its
    /// bots, winner and decisive moment), in OUT. It refuses to write over
    /// any of the records.
    Excerpts {
        /// The directory of games.csv, created if missing.
        #[arg(long)]
        out: PathBuf,
        /// Where to write the excerpts and primer.md instead, created if
        /// missing: the records rebuild them.
        #[arg(long)]
        text: Option<PathBuf>,
        /// Each side's turns that an excerpt shows.
        #[arg(long, default_value_t = 3)]
        moves: usize,
        /// Game records written by the arena (`--records`).
        #[arg(required = true)]
        records: Vec<PathBuf>,
    },
    /// Prints the counts of an excerpt batch: its games.csv and the
    /// reviewers' notes, every notes-*.md, in DIR.
    ExcerptSummary {
        /// The batch's directory.
        dir: PathBuf,
    },
}

/// Reads and replays the record `path`.
fn read_trace(path: &Path) -> Result<cg_review::trace::Trace, String> {
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
    Ok(trace::trace(name, &record))
}

/// Writes the excerpts of `records` and primer.md into `text_dir`, and
/// games.csv into `out`.
fn write_excerpts(
    records: &[PathBuf],
    out: &Path,
    text_dir: &Path,
    moves: usize,
) -> Result<(), String> {
    let mut traces = Vec::new();
    for path in records {
        traces.push(read_trace(path)?);
    }
    let inputs: Vec<PathBuf> = records.iter().map(|p| same_file_key(p)).collect();
    let mut files: Vec<(PathBuf, String)> = Vec::new();
    let mut rows = String::from(GAMES_HEADER);
    rows.push('\n');
    for trace in &traces {
        files.push((
            text_dir.join(format!("{}.excerpt.md", trace.name)),
            excerpt_text(trace, moves),
        ));
        rows.push_str(&games_row(trace, moves));
        rows.push('\n');
    }
    if let Some(first) = traces.first() {
        files.push((text_dir.join("primer.md"), primer_text(first)));
    }
    files.push((out.join("games.csv"), rows));
    for (file, _) in &files {
        if inputs.contains(&same_file_key(file)) {
            return Err(format!(
                "{} would be overwritten: write elsewhere with --out or --text",
                file.display()
            ));
        }
    }
    for (file, body) in files {
        fs::write(&file, body).map_err(|e| format!("{}: {e}", file.display()))?;
    }
    Ok(())
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
    let trace = read_trace(path)?;
    let name = &trace.name;
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
        Command::Excerpts {
            out,
            text,
            moves,
            records,
        } => {
            let text = text.unwrap_or_else(|| out.clone());
            for dir in [&out, &text] {
                fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
            }
            write_excerpts(&records, &out, &text, moves)?;
            eprintln!(
                "wrote {} excerpts to {}, games.csv to {}",
                records.len(),
                text.display(),
                out.display()
            );
            Ok(())
        }
        Command::ExcerptSummary { dir } => {
            print!("{}", excerpt_summary(&dir)?);
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
