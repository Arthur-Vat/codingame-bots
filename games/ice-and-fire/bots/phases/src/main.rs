//! Phases bot for A Code of Ice and Fire (experiment E001): the greedy bronze
//! policy with a plan in phases. Lethal first (a step into the enemy HQ, or
//! the cheapest chain of trainings, aware of towers), then the moves, then a
//! defence when the enemy could take our HQ next turn, cuts of the enemy's
//! territory, banking gold for our own winning chain, then mines and trainings
//! as bronze does. See `plan.rs`.
//!
//! The input never names the league, so this bot fixes it (Bronze, all the
//! rules). A turn whose text does not read as a state is answered with `WAIT`;
//! a bad count line or input that ends in the middle of a turn ends the bot
//! with exit code 1. Debug output goes to stderr only.

mod io;
mod plan;

use ice_and_fire_engine::{answer_line, League, State};

/// The answer line to one turn's input, as CodinGame sends it: the initial
/// input, the turn's input, the league of the game and the game turn from 1.
/// Input that does not read as a state gives `WAIT`, with the reason on
/// stderr.
fn answer(initial: &str, turn: &str, league: League, game_turn: u32) -> String {
    match State::read(initial, turn, league, game_turn) {
        Ok(state) => {
            let mut commands = Vec::new();
            plan::plan(&state, &mut commands);
            answer_line(&commands)
        }
        Err(err) => {
            eprintln!("phases: cannot read the input: {err}");
            "WAIT".to_string()
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    if let Err(err) = io::play(League::Bronze, stdin.lock(), stdout.lock()) {
        eprintln!("phases: {err}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests;
