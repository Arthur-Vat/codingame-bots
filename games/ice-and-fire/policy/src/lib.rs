//! A simple greedy turn policy for A Code of Ice and Fire, shared by the bots
//! of the Wood leagues and Bronze. It is a means of promotion, not a strong
//! player: it grows the territory, pushes its units toward the enemy HQ,
//! kills what it can, and takes the HQ as soon as gold allows a chain of
//! trainings (or one move) that reaches it.
//!
//! A turn is planned on a scratch copy of the engine's state, command by
//! command, keeping only the commands the rules accept, so the bot never sends
//! a command that the state it sees refuses. There is no search: plain loops
//! over a few dozen cells.
//!
//! The league cannot be read from the input, so each bot fixes it and builds
//! its states with it ([`answer`], [`io::play`]).

pub mod io;
mod plan;

use ice_and_fire_engine::{answer_line, Command, League, State};

/// The commands of one turn for the player to act in `state`, in `out`
/// (cleared first). Each one succeeds when applied in order to `state`. Empty
/// when there is nothing worth doing.
pub fn greedy_turn(state: &State, out: &mut Vec<Command>) {
    out.clear();
    plan::plan(state, out);
}

/// The answer line to one turn's input, as CodinGame sends it: the initial
/// input, the turn's input, the league of the game and the game turn from 1.
/// Input that does not read as a state gives `WAIT`, with the reason on
/// stderr.
pub fn answer(initial: &str, turn: &str, league: League, game_turn: u32) -> String {
    match State::read(initial, turn, league, game_turn) {
        Ok(state) => {
            let mut commands = Vec::new();
            greedy_turn(&state, &mut commands);
            answer_line(&commands)
        }
        Err(err) => {
            eprintln!("policy: cannot read the input: {err}");
            "WAIT".to_string()
        }
    }
}
