//! `cg-bundler`: writes a bot and the workspace crates it uses as one Rust
//! file to paste into CodinGame.

use std::path::PathBuf;
use std::process::ExitCode;

use cg_bundler::{bundle_package, Options, SIZE_LIMIT};
use clap::Parser;

/// Bundles a bot package and the workspace crates it uses into one Rust file
/// that CodinGame accepts.
#[derive(Parser)]
#[command(name = "cg-bundler")]
struct Args {
    /// The bot's package name, for example `uttt-bot-random`.
    package: String,

    /// Write the bundle to this file instead of stdout.
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Cargo.toml of the workspace [default: the one of the current directory].
    #[arg(long, value_name = "PATH")]
    manifest_path: Option<PathBuf>,

    /// Skip formatting with rustfmt.
    #[arg(long)]
    no_format: bool,

    /// Keep comments, indentation and blank lines, for a bundle that reads
    /// like the sources.
    #[arg(long)]
    keep_comments: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let options = Options {
        format: !args.no_format,
        strip: !args.keep_comments,
    };
    let bundle = match bundle_package(&args.package, args.manifest_path.as_deref(), options) {
        Ok(bundle) => bundle,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::FAILURE;
        }
    };
    if bundle.len() > SIZE_LIMIT {
        eprintln!(
            "warning: the bundle is {} bytes, over CodinGame's limit of {SIZE_LIMIT}",
            bundle.len()
        );
    }
    match &args.output {
        Some(path) => {
            if let Err(err) = std::fs::write(path, &bundle) {
                eprintln!("error: cannot write {}: {err}", path.display());
                return ExitCode::FAILURE;
            }
            eprintln!("wrote {} ({} bytes)", path.display(), bundle.len());
        }
        None => print!("{bundle}"),
    }
    ExitCode::SUCCESS
}
