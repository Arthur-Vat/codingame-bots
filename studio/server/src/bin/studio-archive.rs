//! `studio-archive`: builds an archive of arena games from folders of game
//! records ([ADR 0031](../../../docs/adr/0031-arena-games-archive.md)). Run
//! with `--help` for the options.

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use studio::archive::{build, Options};

/// Checks game records by replaying them with the game's referee, keeps each
/// once, and writes `<OUT>/records/<id>.json` and `<OUT>/index.json`.
///
/// Every file of an input folder (searched recursively) whose name ends in
/// `.json` is read, except `index.json` and names starting with a dot. If any
/// file is not a valid record, nothing is written and every such file is named.
#[derive(Parser)]
#[command(name = "studio-archive", version)]
struct Args {
    /// The archive's folder: it must not exist, or be empty.
    #[arg(long, value_name = "DIR")]
    out: PathBuf,
    /// Accept only records of this game.
    #[arg(long, value_name = "GAME")]
    game: Option<String>,
    /// Folders of records, read in this order; the first copy of a game wins.
    #[arg(required = true, value_name = "INPUT")]
    inputs: Vec<PathBuf>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let out = args.out.clone();
    let options = Options {
        out: args.out,
        game: args.game,
        inputs: args.inputs,
    };
    match build(&options) {
        Ok(summary) => {
            for input in &summary.inputs {
                println!(
                    "{}: {} records, {} new",
                    input.input.display(),
                    input.records,
                    input.new
                );
            }
            println!(
                "{} records written to {} ({} duplicates skipped)",
                summary.written,
                out.display(),
                summary.duplicates
            );
            ExitCode::SUCCESS
        }
        Err(messages) => {
            for message in messages {
                eprintln!("{message}");
            }
            ExitCode::FAILURE
        }
    }
}
