//! The ladder of reference bots of A Code of Ice and Fire (experiment E001).
//! Each bot plays the turn of the greedy bronze policy
//! (`games/ice-and-fire/policy`) plus the steps of [`Steps`] it switches on,
//! each bot adding one step to the one before: `chain` (lethal chains aware of
//! towers), `guard` (a defence of the HQ), `cutter` (cuts of the enemy's
//! territory) and `banker` (gold kept for our winning chain), which is the
//! game's first release. With every step off, [`plan`] plays exactly the turn
//! of bronze.
//!
//! The reference bots are built from this crate, so its play must not change
//! once merged: only bug fixes, each recorded in the journal. Later bots copy
//! what they need and change their copy. The tactics they use
//! (`games/ice-and-fire/tactics`) follow the same rule.
//!
//! A turn is planned on a scratch copy of the engine's state, command by
//! command, keeping only the commands the rules accept, so the bot never
//! sends a command that the state it sees refuses. The league cannot be read
//! from the input, so the bots fix it: Bronze, all the rules. Standard library
//! only, so the bundler can copy it into bots.

pub mod io;
mod plan;

use ice_and_fire_engine::{answer_line, Command, League, State};

/// Which of the ladder's steps a bot plays, on top of bronze's turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Steps {
    /// Lethal with the cheapest chain of trainings that knows the towers
    /// (`tactics::chain::chain_to`), instead of bronze's static chain.
    pub tower_chain: bool,
    /// A defence while the enemy could take our HQ next turn, and a last
    /// check that undoes spending that exposed the HQ.
    pub defence: bool,
    /// Up to 2 cuts of the enemy's territory that leave the HQ no more
    /// exposed.
    pub cuts: bool,
    /// Gold is kept when our chain to the enemy HQ is affordable next turn.
    pub banking: bool,
}

impl Steps {
    /// No step: bronze's turn.
    pub const NONE: Steps = Steps {
        tower_chain: false,
        defence: false,
        cuts: false,
        banking: false,
    };

    /// Every step.
    pub const ALL: Steps = Steps {
        tower_chain: true,
        defence: true,
        cuts: true,
        banking: true,
    };
}

/// The commands of one turn for the player to act in `state`, in `out`
/// (cleared first). Each one succeeds when applied in order to `state`. Empty
/// when there is nothing worth doing.
pub fn plan(state: &State, steps: Steps, out: &mut Vec<Command>) {
    out.clear();
    plan::plan(state, steps, out);
}

/// The answer line to one turn's input, as CodinGame sends it: the initial
/// input, the turn's input, the league of the game and the game turn from 1.
/// Input that does not read as a state gives `WAIT`, with the reason on
/// stderr.
pub fn answer(initial: &str, turn: &str, league: League, game_turn: u32, steps: Steps) -> String {
    match State::read(initial, turn, league, game_turn) {
        Ok(state) => {
            let mut commands = Vec::new();
            plan(&state, steps, &mut commands);
            answer_line(&commands)
        }
        Err(err) => {
            eprintln!("ladder: cannot read the input: {err}");
            "WAIT".to_string()
        }
    }
}

#[cfg(test)]
mod tests;
