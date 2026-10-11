//! `studio`: the studio's local server. Run with `--help` for the options.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use studio::server::serve;
use studio::State;

/// Serves the studio on 127.0.0.1.
#[derive(Parser)]
#[command(name = "studio", version)]
struct Args {
    /// The port to listen on.
    #[arg(long, default_value_t = 8411)]
    port: u16,
    /// The repository root (default: the nearest folder above the current
    /// one that holds `games/` and `Cargo.toml`).
    #[arg(long)]
    repo: Option<PathBuf>,
    /// The built front end (default: `<repo>/studio/web/dist`).
    #[arg(long)]
    web: Option<PathBuf>,
    /// Where compiled bots and speed measurements go (default:
    /// `<repo>/studio/data`).
    #[arg(long)]
    data: Option<PathBuf>,
}

/// The nearest ancestor of `start` (itself included) holding `games/` and
/// `Cargo.toml`.
fn find_repo(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|dir| dir.join("games").is_dir() && dir.join("Cargo.toml").is_file())
        .map(Path::to_path_buf)
}

fn main() -> ExitCode {
    let args = Args::parse();
    let repo = match args.repo {
        Some(repo) => repo,
        None => {
            let found = std::env::current_dir().ok().and_then(|dir| find_repo(&dir));
            match found {
                Some(repo) => repo,
                None => {
                    eprintln!("studio: no repository found above the current folder; pass --repo");
                    return ExitCode::FAILURE;
                }
            }
        }
    };
    // Bots get their seed, time scale and iterations from the studio only.
    for name in ["CG_SEED", "CG_TIME_SCALE", "CG_FIXED_ITERS"] {
        std::env::remove_var(name);
    }
    let web = args.web.unwrap_or_else(|| repo.join("studio/web/dist"));
    let data = args.data.unwrap_or_else(|| repo.join("studio/data"));
    let state = State::new(repo, web, data);
    match serve(&state, args.port) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("studio: {message}");
            ExitCode::FAILURE
        }
    }
}
