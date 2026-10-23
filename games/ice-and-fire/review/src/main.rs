//! `ice-and-fire-review`: turns recorded games into review files, and
//! summarizes a reviewed batch (`cg_review`).
//!
//! ```sh
//! ice-and-fire-review games --out DIR [--text DIR] GAME.json...
//! ice-and-fire-review summary DIR > DIR/summary-data.md
//! ```

mod trace;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use cg_arena::record::Record;
use cg_review::summary::{load, summarize};
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
    /// NAME.csv (the metrics) and NAME.json (the result, phases and decisive
    /// moments), NAME being the record's file name without `.json`.
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
}

/// Writes the three files of one record: the game file in `text`, the
/// others in `out`.
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
    for (dir, ext, body) in [
        (text_dir, "md", game_text(&trace)),
        (out, "csv", metrics_csv(&trace)),
        (out, "json", summary + "\n"),
    ] {
        let file = dir.join(format!("{name}.{ext}"));
        fs::write(&file, body).map_err(|e| format!("{}: {e}", file.display()))?;
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
            for path in &records {
                write_game(path, &out, &text)?;
            }
            eprintln!("wrote {} games to {}", records.len(), out.display());
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
