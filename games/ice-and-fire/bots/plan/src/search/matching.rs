//! Whole turns built by a matching that keeps units apart (E107, stages 4
//! and 5 of `games/ice-and-fire/research/codingame/2026-11-28-type-planner.md`):
//! each unit draws a kind of action from the table's prior for its type, the
//! kind becomes the cell that crowds our units least, and the turns so built
//! start climbs of their own.

use cg_core::rng::Rng;
use ice_and_fire_engine::{Bits, Command, State};

use crate::actions::xy;
use crate::eval::reach;
use crate::types::{prior, Kind, Typing, KINDS};

/// The price of each own unit next to a cell, of each cell chosen by another
/// unit within 2 steps, and the gain of each free neutral cell next to it.
const NEXT_OWN: i64 = 3;
const NEAR_CHOSEN: i64 = 2;
const FREE_NEUTRAL: i64 = 1;

/// What the sampled turns of one root share.
pub struct Matcher {
    typing: Typing,
    /// The movable units: cell, id and the prior of the unit's key.
    units: Vec<(usize, u32, [u8; KINDS])>,
    /// Cells of ours that the enemy can enter at its next turn.
    danger: Bits,
    /// Our units that stay where they are.
    fixed: Bits,
}

impl Matcher {
    /// The matcher for the turn of the side to act in `root`.
    pub fn new(root: &State) -> Matcher {
        let me = root.to_act();
        let typing = Typing::new(root);
        let units = root
            .movable_units()
            .iter()
            .filter_map(|cell| {
                let (id, _) = root.unit_at(cell)?;
                let key = typing.key(root, cell)?;
                Some((cell, id, prior(&key)))
            })
            .collect();
        Matcher {
            typing,
            units,
            danger: reach(root, 1 - me),
            fixed: root.units(me).and_not(root.movable_units()),
        }
    }

    /// A seed from the state: the game turn and the ids of the units, with
    /// the number of the sample.
    pub fn seed(&self, root: &State, sample: u64) -> u64 {
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325 ^ u64::from(root.game_turn());
        for &(_, id, _) in &self.units {
            hash = (hash ^ u64::from(id)).wrapping_mul(0x0100_0000_01b3);
        }
        (hash ^ sample).wrapping_mul(0x0100_0000_01b3)
    }

    /// A whole turn of moves of `root`'s movable units, in a random order
    /// fixed by `seed`, and the state after it.
    pub fn sample(&self, root: &State, seed: u64) -> (Vec<Command>, State) {
        let me = root.to_act();
        let mut rng = Rng::new(seed);
        let mut order: Vec<usize> = (0..self.units.len()).collect();
        rng.shuffle(&mut order);
        let valid = root.map().valid;
        let mut cur = root.clone();
        let mut commands = Vec::new();
        let mut settled = self.fixed;
        let mut chosen = Bits::EMPTY;
        for index in order {
            let (from, id, p) = self.units[index];
            let level = cur.unit_level(from).unwrap_or(1).clamp(1, 3);
            let enemy_land = cur.owned(1 - me);
            let danger = self.danger.test(from);
            let steps = (Bits::cell(from).neighbours() & valid & cur.enterable(me, level))
                .iter()
                .filter(|&to| !danger || enemy_land.test(to))
                .collect::<Vec<_>>();
            let kinds: Vec<Kind> = steps
                .iter()
                .map(|&to| self.typing.kind(&cur, from, to))
                .collect();
            let there = |kind: Kind| kind == Kind::Stay || kinds.contains(&kind);
            let mut weight = [0u64; KINDS];
            for kind in Kind::ALL {
                weight[kind as usize] = u64::from(there(kind)) * u64::from(p[kind as usize]);
            }
            if weight.iter().all(|&w| w == 0) {
                for kind in Kind::ALL {
                    weight[kind as usize] = u64::from(there(kind));
                }
            }
            let mut ticket = rng.below(weight.iter().sum());
            let mut kind = Kind::Stay;
            for k in Kind::ALL {
                let w = weight[k as usize];
                if ticket < w {
                    kind = k;
                    break;
                }
                ticket -= w;
            }
            let mut to = from;
            if kind != Kind::Stay {
                let cost = |cell: usize| {
                    let one = Bits::cell(cell).neighbours();
                    let near = (one | one.neighbours()) & chosen;
                    let occupied = cur.units(0) | cur.units(1);
                    let free = (one & valid)
                        .and_not(cur.owned(0))
                        .and_not(cur.owned(1))
                        .and_not(occupied);
                    NEXT_OWN * i64::from((one & settled).count())
                        + NEAR_CHOSEN * i64::from(near.count())
                        - FREE_NEUTRAL * i64::from(free.count())
                };
                let best = steps
                    .iter()
                    .zip(&kinds)
                    .filter(|(_, &k)| k == kind)
                    .map(|(&cell, _)| cell)
                    .min_by_key(|&cell| (cost(cell), self.typing.distance(cell), cell));
                if let Some(cell) = best {
                    let (x, y) = xy(cell);
                    let command = Command::Move { id, x, y };
                    if cur.apply(command) {
                        commands.push(command);
                        to = cell;
                    }
                }
            }
            settled.set(to);
            chosen.set(to);
        }
        (commands, cur)
    }
}

#[cfg(test)]
mod tests;
