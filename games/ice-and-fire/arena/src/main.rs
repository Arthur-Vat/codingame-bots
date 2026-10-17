//! `ice-and-fire-arena`: plays A Code of Ice and Fire bots against each other.
//! Run with `--help` for the commands.

use std::process::ExitCode;

use cg_arena::referee::{GameSetup, Referee};
use ice_and_fire_referee::IceAndFireReferee;

/// A Bronze game, which has all the rules, on the map of the setup's seed.
/// The referee ignores the opening plies: each seed's map varies the games.
fn new_referee(setup: &GameSetup) -> Box<dyn Referee> {
    Box::new(IceAndFireReferee::new(setup))
}

fn main() -> ExitCode {
    cg_arena::cli::main("ice-and-fire-arena", new_referee)
}
