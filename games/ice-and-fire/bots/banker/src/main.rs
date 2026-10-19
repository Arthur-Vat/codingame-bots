//! Banker bot for A Code of Ice and Fire, the last step of the ladder
//! (`games/ice-and-fire/ladder`): the cutter bot, plus gold kept for its
//! winning chain when that chain is affordable next turn. It is the game's
//! first release, `ice-and-fire-v001`.
//!
//! The input never names the league, so the ladder fixes it (Bronze, all the
//! rules). A turn whose text does not read as a state is answered with
//! `WAIT`; a bad count line or input that ends in the middle of a turn ends
//! the bot with exit code 1. Debug output goes to stderr only.

use std::io;

use ice_and_fire_ladder::Steps;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    if let Err(err) =
        ice_and_fire_ladder::io::play("banker", Steps::ALL, stdin.lock(), stdout.lock())
    {
        eprintln!("banker: {err}");
        std::process::exit(1);
    }
}
