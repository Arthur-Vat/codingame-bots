//! Rollouts to the middle game (E004): our side plays a policy, the opponent
//! is simulated as `ice-and-fire-v003`'s `econ` opening, until the middle game
//! starts, and the end is scored for us. The planners of the first turn
//! compare their plans this way.

use std::time::Instant;

use ice_and_fire_engine::{Command, State};

use crate::econ::{self, Params};
use crate::middle;
use crate::score::{self, Settings};

/// Most game turns a rollout plays: the middle game comes at turn 7 to 9.
pub const CAP: u32 = 20;

/// Plays from `state` (`me` to act) until the middle game, the cap or the
/// end of the game: `ours` gives our turns (from the number of our turns
/// played so far in the rollout, 0 first), the opponent plays `econ` with
/// `model`'s weights. Returns the state reached.
pub fn play(
    state: &State,
    model: &Params,
    mut ours: impl FnMut(usize, &State) -> Vec<Command>,
) -> State {
    let me = state.to_act();
    let mut sim = state.clone();
    let mut turns = 0;
    loop {
        if sim.outcome().is_some() || sim.game_turn() > CAP {
            return sim;
        }
        // The root is our turn now, whatever it holds.
        if (turns > 0 || sim.to_act() != me) && middle::reached(&sim) {
            return sim;
        }
        let commands = if sim.to_act() == me {
            turns += 1;
            ours(turns - 1, &sim)
        } else {
            econ::rules_turn_with(&sim, model)
        };
        sim.play_turn(&commands);
    }
}

/// The score for `me` of the middle game reached by playing `econ` with
/// `params` from `state` against the model.
pub fn value_of(state: &State, params: &Params, model: &Params, settings: Settings) -> i64 {
    let me = state.to_act();
    let end = play(state, model, |_, s| econ::rules_turn_with(s, params));
    score::score(&end, me, settings)
}

/// The index of the weights of `sets` whose rollout scores best for the
/// player to act, the first on a tie; the first set if none was scored
/// before `deadline`. `sets` must not be empty.
pub fn choose(
    state: &State,
    sets: &[Params],
    model: &Params,
    settings: Settings,
    deadline: Option<Instant>,
) -> usize {
    let mut best: Option<(i64, usize)> = None;
    for (index, params) in sets.iter().enumerate() {
        if best.is_some() && deadline.is_some_and(|t| Instant::now() >= t) {
            break;
        }
        let value = value_of(state, params, model, settings);
        if best.is_none_or(|(top, _)| value > top) {
            best = Some((value, index));
        }
    }
    best.map_or(0, |(_, index)| index)
}

#[cfg(test)]
mod tests;
