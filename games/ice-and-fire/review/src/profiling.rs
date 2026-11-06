//! A Code of Ice and Fire's hooks for the player profiles of `cg_review`
//! (`codingame-report`): the metrics a profile shows, the events of a ply
//! and the tokens of an opening.

use std::collections::BTreeMap;

use cg_review::profile::Profiling;
use cg_review::trace::Trace;
use ice_and_fire_engine::state::BUILD_COST;
use ice_and_fire_engine::{Command, SIZE};

use crate::trace::parse;

/// The game's hooks.
pub struct IceAndFire;

/// The kind of the moment that [`IceAndFire::events`] turns into the event
/// "large cut" (`trace::trace` makes it with `large_drops`).
const LARGE_CUT: &str = "large cut";

/// The metrics for which less is better: the exposure to a cut, the gap to
/// the enemy HQ, the upkeep, the cells cut off, the units and cells lost, and
/// the weak points of the territory (joints, the cells a single cell would cut
/// off, thin line ends).
const LOWER_IS_BETTER: [&str; 14] = [
    "exposed30",
    "exposed50",
    "hq_gap",
    "upkeep",
    "cut_off",
    "units_lost",
    "lost",
    "joints",
    "max_cut",
    "total_cut",
    "line_ends",
    "frontier_joints",
    "empty_joints",
    "reach",
];

/// `(x, y)` as the player in `seat` sees it, so that both seats' openings
/// compare: seat 0 as is, seat 1 mirrored by the half-turn
/// `(x, y) -> (11 - x, 11 - y)`. The maps are symmetric under that
/// half-turn, with the HQs at (0, 0) and (11, 11) (`RULES.md`, "The map";
/// `MAP_GENERATOR.md`, "The mirror" and the note that the map is half-turn
/// symmetric).
pub fn mirrored(seat: usize, x: u8, y: u8) -> (u8, u8) {
    if seat == 0 {
        (x, y)
    } else {
        let last = (SIZE - 1) as u8;
        (last - x, last - y)
    }
}

/// The token of one command, in `seat`'s view: `T1(1,0)` trains a level 1
/// unit at (1, 0), `M(2,1)` moves a unit to (2, 1), `BM(4,5)` builds a mine
/// and `BT(3,3)` a tower.
fn token(seat: usize, command: Command) -> Option<String> {
    Some(match command {
        Command::Wait => return None,
        Command::Train { level, x, y } => {
            let (x, y) = mirrored(seat, x, y);
            format!("T{level}({x},{y})")
        }
        Command::Move { x, y, .. } => {
            let (x, y) = mirrored(seat, x, y);
            format!("M({x},{y})")
        }
        Command::BuildMine { x, y } => {
            let (x, y) = mirrored(seat, x, y);
            format!("BM({x},{y})")
        }
        Command::BuildTower { x, y } => {
            let (x, y) = mirrored(seat, x, y);
            format!("BT({x},{y})")
        }
    })
}

fn commands(line: &str) -> impl Iterator<Item = Command> + '_ {
    line.split(';').filter_map(|part| parse(part.trim()))
}

impl Profiling for IceAndFire {
    /// Games last about 20 turns (at most 100).
    fn turns() -> Vec<u32> {
        vec![5, 10, 15, 20, 30]
    }

    /// The first three own plies: the trainings and first moves.
    fn opening_plies() -> usize {
        3
    }

    /// Gold in hand, income, active cells, army, mines, towers, the
    /// exposure to a 30-gold cut, the gap to a chain to the HQ, the
    /// material (the bots' evaluation) and the Voronoi land.
    fn key_metrics() -> Vec<&'static str> {
        vec![
            "gold",
            "income",
            "cells",
            "army",
            "mines",
            "towers",
            "exposed30",
            "hq_gap",
            "material",
            "voronoi",
        ]
    }

    /// "train level 1" to "train level 3", "mine", "tower", one for each
    /// command (the traces hold only the commands carried out, see
    /// `profiles`), and "large cut" once when this ply took at least 30% of the
    /// enemy's active cells (a `large cut` moment favouring the player).
    fn events(trace: &Trace, ply: usize) -> Vec<String> {
        let Some(p) = trace.plies.get(ply) else {
            return Vec::new();
        };
        let mut events: Vec<String> = commands(&p.commands)
            .filter_map(|command| match command {
                Command::Train { level, .. } => Some(format!("train level {level}")),
                Command::BuildMine { .. } => Some("mine".to_string()),
                Command::BuildTower { .. } => Some("tower".to_string()),
                Command::Move { .. } | Command::Wait => None,
            })
            .collect();
        if trace
            .moments
            .iter()
            .any(|m| m.ply == ply && m.kind == LARGE_CUT && m.favours == Some(p.to_act))
        {
            events.push("large cut".to_string());
        }
        events
    }

    /// A gap, an exposure, a cost or a loss: the less the better. Gold,
    /// income, cells, army and the others count more as better.
    fn lower_is_better(key: &str) -> bool {
        LOWER_IS_BETTER.contains(&key)
    }

    /// Turns 10 and 15: the middle of a game of about 20 turns, where the
    /// games' sides have taken their shape.
    fn compare_turns() -> Vec<u32> {
        vec![10, 15]
    }

    fn resource() -> &'static str {
        "gold"
    }

    /// The gold of the commands carried out, from their costs in `RULES.md`:
    /// "train level 1" to "train level 3" at 10, 20 and 30 ("TRAIN"); "mine"
    /// and "tower" at 15 each ("BUILD MINE x y and BUILD TOWER x y": a mine
    /// costs the same however many mines the player owns, since the cost
    /// increment is 0). The traces hold only the commands carried out (see
    /// `profiles`), so each is paid.
    fn spending(trace: &Trace, ply: usize) -> Vec<(String, i64)> {
        let Some(p) = trace.plies.get(ply) else {
            return Vec::new();
        };
        let mut spent: BTreeMap<String, i64> = BTreeMap::new();
        for command in commands(&p.commands) {
            let (name, cost) = match command {
                Command::Train { level, .. } => {
                    (format!("train level {level}"), 10 * i64::from(level))
                }
                Command::BuildMine { .. } => ("mine".to_string(), BUILD_COST),
                Command::BuildTower { .. } => ("tower".to_string(), BUILD_COST),
                Command::Move { .. } | Command::Wait => continue,
            };
            *spent.entry(name).or_default() += cost;
        }
        spent.into_iter().collect()
    }

    /// The commands of each of the first own plies, tokens joined by a
    /// space, in the order played, in the player's own view ([`mirrored`]).
    fn opening(trace: &Trace, seat: usize, plies: usize) -> Vec<String> {
        trace
            .plies
            .iter()
            .filter(|p| p.to_act == seat)
            .take(plies)
            .map(|p| {
                let tokens: Vec<String> = commands(&p.commands)
                    .filter_map(|command| token(seat, command))
                    .collect();
                if tokens.is_empty() {
                    "-".to_string()
                } else {
                    tokens.join(" ")
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests;
