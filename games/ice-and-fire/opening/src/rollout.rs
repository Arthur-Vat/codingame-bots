//! Rollouts to the middle game (E004): our side plays a policy, the opponent
//! is simulated as `ice-and-fire-v003`'s `econ` opening, until the middle game
//! starts, and the end is scored for us. The planners of the first turn
//! compare their plans this way.

use std::time::Instant;

use cg_core::rng::Rng;

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

/// Changes one or two of `params`' weights by a small step, within their
/// ranges.
pub fn mutate(params: &Params, rng: &mut Rng) -> Params {
    let mut p = *params;
    let changes = 1 + rng.below(2);
    for _ in 0..changes {
        let up = rng.below(2) == 0;
        let step = |v: i64, low: i64, high: i64| {
            if up {
                (v + 1).min(high)
            } else {
                (v - 1).max(low)
            }
        };
        match rng.below(10) {
            0 => p.open = step(p.open, 0, 8),
            1 => p.contest = step(p.contest, 0, 8),
            2 => p.contest_cap = step(i64::from(p.contest_cap), 4, 22) as u8,
            3 => p.hq_div = [0, 1, 2, 4, 8][rng.below(5) as usize],
            4 => p.horizon = step(p.horizon, 8, 26),
            5 => p.min_turns_left = step(p.min_turns_left, 1, 12),
            6 => p.safe_spot = step(i64::from(p.safe_spot), 0, 10) as u8,
            7 => p.contact_offset = step(p.contact_offset, 0, 5),
            8 => p.contact_div = step(p.contact_div, 1, 4),
            _ => p.after_contact = step(p.after_contact, 0, 8),
        }
    }
    p
}

/// The weights that score best for the player to act within the time: the
/// best of `sets` by [`value_of`], then, until `deadline` (or `max_tries`
/// more rollouts), hill climbing from it on [`mutate`]d weights, a change
/// kept when its rollout scores higher. The first set if none was scored.
pub fn search(
    state: &State,
    sets: &[Params],
    model: &Params,
    settings: Settings,
    deadline: Option<Instant>,
    max_tries: usize,
    seed: u64,
) -> Params {
    let index = choose(state, sets, model, settings, deadline);
    let mut best = sets[index];
    let mut best_value = value_of(state, &best, model, settings);
    let mut rng = Rng::new(seed);
    for _ in 0..max_tries {
        if deadline.is_some_and(|t| Instant::now() >= t) {
            break;
        }
        let trial = mutate(&best, &mut rng);
        if trial == best {
            continue;
        }
        let value = value_of(state, &trial, model, settings);
        if value > best_value {
            best = trial;
            best_value = value;
        }
    }
    best
}

#[cfg(test)]
mod tests;
