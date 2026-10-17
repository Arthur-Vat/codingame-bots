//! Bronze bot for A Code of Ice and Fire: the greedy policy
//! (`games/ice-and-fire/policy`) with all the rules, towers included. It also
//! serves in the leagues above Bronze until real bots exist. A means of
//! promotion, not a strong bot.
//!
//! The input never names the league, so this bot fixes it. Input that cannot
//! be read is answered with `WAIT`; debug output goes to stderr only.

use std::io;

use ice_and_fire_engine::League;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    if let Err(err) = ice_and_fire_policy::io::play(League::Bronze, stdin.lock(), stdout.lock()) {
        eprintln!("bronze: {err}");
        std::process::exit(1);
    }
}
