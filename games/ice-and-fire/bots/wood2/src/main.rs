//! Wood 2 bot for A Code of Ice and Fire: the greedy policy
//! (`games/ice-and-fire/policy`) with the Wood 2 rules, where units have three
//! levels and a level 1 unit beats none. A means of promotion, not a strong
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
    if let Err(err) = ice_and_fire_policy::io::play(League::Wood2, stdin.lock(), stdout.lock()) {
        eprintln!("wood2: {err}");
        std::process::exit(1);
    }
}
