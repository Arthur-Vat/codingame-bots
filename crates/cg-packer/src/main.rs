//! `cg-packer`: packs a file's bytes into a Rust module that a bot decodes
//! with `cg_pack::unpack` (docs/research/utf16-packing.md).

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

/// Packs a file's bytes into characters outside ASCII and writes them as a
/// Rust module with one string constant, for a bot to decode with
/// `cg_pack::unpack`.
#[derive(Parser)]
#[command(name = "cg-packer")]
struct Args {
    /// The file whose bytes are packed.
    input: PathBuf,

    /// The Rust file to write, for example `src/weights.rs`.
    #[arg(short, long, value_name = "FILE")]
    output: PathBuf,

    /// The constant's name.
    #[arg(long, default_value = "DATA")]
    name: String,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let data = match std::fs::read(&args.input) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("cg-packer: cannot read {}: {err}", args.input.display());
            return ExitCode::FAILURE;
        }
    };
    let file_name = args.input.file_name().map_or_else(
        || args.input.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );
    let module = match cg_packer::rust_module(&data, &args.name, &format!("`{file_name}`")) {
        Ok(module) => module,
        Err(err) => {
            eprintln!("cg-packer: {err}");
            return ExitCode::FAILURE;
        }
    };
    if let Err(err) = std::fs::write(&args.output, &module) {
        eprintln!("cg-packer: cannot write {}: {err}", args.output.display());
        return ExitCode::FAILURE;
    }
    eprintln!(
        "wrote {} ({} bytes, a module of {} characters)",
        args.output.display(),
        data.len(),
        module.encode_utf16().count()
    );
    ExitCode::SUCCESS
}
