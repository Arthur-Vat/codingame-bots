//! `uttt-arena`: plays Ultimate Tic-Tac-Toe bots against each other.
//! Run with `--help` for the commands.

use std::process::ExitCode;

use cg_arena::referee::{GameSetup, Referee};
use uttt_referee::UtttReferee;

fn new_referee(setup: &GameSetup) -> Box<dyn Referee> {
    Box::new(UtttReferee::with_opening(setup.seed, setup.opening_plies))
}

fn main() -> ExitCode {
    cg_arena::cli::main("uttt-arena", new_referee)
}
