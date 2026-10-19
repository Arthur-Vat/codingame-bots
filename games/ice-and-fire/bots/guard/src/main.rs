//! Guard bot for A Code of Ice and Fire, the second step of the ladder
//! (`games/ice-and-fire/ladder`): the chain bot, plus a defence of its HQ when
//! the enemy could take it next turn.
//!
//! The input never names the league, so the ladder fixes it (Bronze, all the
//! rules). A turn whose text does not read as a state is answered with
//! `WAIT`; a bad count line or input that ends in the middle of a turn ends
//! the bot with exit code 1. Debug output goes to stderr only.

use std::io;

use ice_and_fire_ladder::Steps;

const STEPS: Steps = Steps {
    tower_chain: true,
    defence: true,
    ..Steps::NONE
};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    if let Err(err) = ice_and_fire_ladder::io::play("guard", STEPS, stdin.lock(), stdout.lock()) {
        eprintln!("guard: {err}");
        std::process::exit(1);
    }
}
