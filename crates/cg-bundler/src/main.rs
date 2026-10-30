//! `cg-bundler`: writes a bot and the workspace crates it uses as one Rust
//! file to paste into CodinGame.

use std::path::PathBuf;
use std::process::ExitCode;

use cg_bundler::{bundle_package_report, cg_size, Minify, Options, SIZE_LIMIT};
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

    /// Write the readable bundle, with comments and rustfmt's layout, instead
    /// of the minified one (ADR 0045).
    #[arg(long)]
    keep_comments: bool,

    /// How far to minify (ADR 0045): 1 writes only the tokens Rust needs; 2
    /// also removes the items rustc reports as never used; 3 also gives the
    /// names the bundle defines the shortest names not otherwise used. 2 and
    /// 3 need rustc 1.90.0 (found with rustup, or named by CG_RUSTC).
    /// Ignored with --keep-comments.
    #[arg(
        long,
        value_name = "N",
        default_value_t = 3,
        value_parser = clap::value_parser!(u8).range(1..=3)
    )]
    minify_level: u8,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let options = Options {
        format: !args.no_format,
        minify: match (args.keep_comments, args.minify_level) {
            (true, _) => Minify::Off,
            (false, 1) => Minify::Level1,
            (false, 2) => Minify::Level2,
            (false, _) => Minify::Level3,
        },
    };
    let bundled = match bundle_package_report(&args.package, args.manifest_path.as_deref(), options)
    {
        Ok(bundled) => bundled,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::FAILURE;
        }
    };
    if let Some(report) = &bundled.pruned {
        eprintln!("level 2: {report}");
    }
    if let Some(report) = &bundled.renamed {
        eprintln!("level 3: {report}");
    }
    let bundle = bundled.code;
    if cg_size(&bundle) > SIZE_LIMIT {
        eprintln!(
            "warning: the bundle is {} characters (UTF-16 units), over CodinGame's limit of {SIZE_LIMIT}",
            cg_size(&bundle)
        );
    }
    match &args.output {
        Some(path) => {
            if let Err(err) = std::fs::write(path, &bundle) {
                eprintln!("error: cannot write {}: {err}", path.display());
                return ExitCode::FAILURE;
            }
            eprintln!(
                "wrote {} ({} characters, {} bytes)",
                path.display(),
                cg_size(&bundle),
                bundle.len()
            );
        }
        None => print!("{bundle}"),
    }
    ExitCode::SUCCESS
}
