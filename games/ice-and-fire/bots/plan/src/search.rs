//! The bot's turn: a lethal move or chain first, then the opening until the
//! middle game (`ice_and_fire_opening::planned`), then the plan search
//! (`ice_and_fire_search::plan`).

use ice_and_fire_engine::State;
use ice_and_fire_opening::planned::Planned;
use ice_and_fire_search::plan::{lethal, plans};
pub use ice_and_fire_search::plan::{Limit, Thought};

/// The commands to play in `state` for the player to act, within `limit`;
/// `opening` plays until the middle game (`ice_and_fire_opening::middle`).
pub fn think(state: &State, limit: Limit, opening: &mut Planned) -> Thought {
    let thought = |commands| Thought {
        commands,
        climbs: 0,
        evals: 0,
        memory: 0,
    };
    if state.outcome().is_some() {
        return thought(Vec::new());
    }
    if let Some(commands) = lethal(state) {
        return thought(commands);
    }
    if opening.playing(state) {
        // The opening's choice stops at 76% of the turn: the hard limit is
        // the whole budget.
        let at = |share: f64| match (limit.soft, limit.hard) {
            (Some(soft), Some(hard)) => Some(soft + (hard - soft).mul_f64(share)),
            _ => None,
        };
        return thought(opening.turn(state, at(0.4)));
    }
    plans(state, limit)
}
