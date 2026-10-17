//! Wood 3 bot for A Code of Ice and Fire: the greedy policy
//! (`games/ice-and-fire/policy`) with the Wood 3 rules, where a level 1 unit
//! beats another and only level 1 exists. A means of promotion, not a strong
//! bot.
//!
//! The input never names the league, so this bot fixes it. A turn whose
//! text does not read as a state is answered with `WAIT`; a bad count line or
//! input that ends in the middle of a turn ends the bot with exit code 1.
//! Debug output goes to stderr only.

use std::io;

use ice_and_fire_engine::League;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    if let Err(err) = ice_and_fire_policy::io::play(League::Wood3, stdin.lock(), stdout.lock()) {
        eprintln!("wood3: {err}");
        std::process::exit(1);
    }
}
