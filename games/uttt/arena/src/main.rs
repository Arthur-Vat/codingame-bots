//! `uttt-arena`: plays two Ultimate Tic-Tac-Toe bots against each other.
//! Run with `--help` for the options.

use std::process::ExitCode;

use uttt_referee::UtttReferee;

fn main() -> ExitCode {
    cg_arena::cli::main("uttt-arena", |seed| Box::new(UtttReferee::new(seed)))
}
