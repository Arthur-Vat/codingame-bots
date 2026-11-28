//! Which towers our climbs keep (E112): near the enemy's land, or where the
//! enemy can strike.

use ice_and_fire_engine::{Bits, Command, State, HQ, SIZE};
use ice_and_fire_tactics::chain::chain_to;
use ice_and_fire_tactics::cut::cuts;

use crate::actions::{Action, CUT_LENGTH};

/// The enemy's affordable cuts to keep in mind (the best few).
const THREAT_CUTS: usize = 4;

/// The cells the enemy's affordable strikes at its next turn go through,
/// from `root` (our turn's start, `me` to act): its best cuts with its gold
/// and income, and its cheapest chain of trainings to our HQ if it can pay
/// it.
pub fn threat_cells(root: &State, me: usize) -> Bits {
    let mut next = root.clone();
    next.end_turn();
    let mut out = Bits::EMPTY;
    if next.outcome().is_some() {
        return out;
    }
    let gold = next.gold(1 - me);
    let mut add = |commands: &[Command]| {
        for c in commands {
            match *c {
                Command::Train { x, y, .. } | Command::Move { x, y, .. } => {
                    out.set(usize::from(y) * SIZE + usize::from(x));
                }
                _ => {}
            }
        }
    };
    for cut in cuts(&next, CUT_LENGTH, gold, THREAT_CUTS) {
        add(&cut.commands);
    }
    if let Some(chain) = chain_to(&next, HQ[me], gold) {
        add(&chain.commands);
    }
    out
}

/// The cells within `k` steps of the land of `me`'s enemy.
pub fn near_enemy(state: &State, me: usize, k: u8) -> Bits {
    let mut near = state.owned(1 - me);
    for _ in 0..k {
        near = near | near.neighbours();
    }
    near & state.map().valid
}

/// The cells of every tower that `action` builds.
fn towers(action: &Action) -> impl Iterator<Item = usize> + '_ {
    action.commands().iter().filter_map(|c| match *c {
        Command::BuildTower { x, y } => Some(usize::from(y) * SIZE + usize::from(x)),
        _ => None,
    })
}

/// Keeps in `actions` only those whose towers stand within `k` steps of the
/// enemy's land or protect a cell of `threat` (the tower's cell or a
/// neighbour); actions without a tower are kept.
pub fn keep_towers(state: &State, me: usize, k: u8, threat: Bits, actions: &mut Vec<Action>) {
    let near = near_enemy(state, me, k);
    actions.retain(|action| {
        towers(action).all(|t| {
            let zone = Bits::cell(t) | Bits::cell(t).neighbours();
            near.test(t) || !(zone & threat).is_empty()
        })
    });
}

/// For the order of [`super::TOWER_NEAR_FIRST`]: 0 for an action without a
/// tower, else the steps from its nearest tower to the enemy's land (at most
/// 15).
pub fn rank(state: &State, me: usize, action: &Action) -> u8 {
    let enemy = state.owned(1 - me);
    let map = state.map();
    towers(action)
        .map(|t| {
            enemy
                .iter()
                .map(|e| map.distance(t, e))
                .min()
                .unwrap_or(15)
                .min(15)
        })
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests;
