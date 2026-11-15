//! A scan of unit turns in the CodinGame games (`codingame-units`): what
//! each unit that may move did in a turn, and, when it took nothing, what
//! it could have done, for our releases and the top players, by seat and
//! phase.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use ice_and_fire_engine::{Bits, Command, State};
use serde_json::Value;

use crate::profiles::{newest_snapshot, prepare, read_index, read_ranks, Request};
use crate::replay::{replay, Game};

/// The kinds of unit turn, in the order of the counts.
pub const KINDS: [&str; 9] = [
    "took_neutral",
    "took_enemy",
    "killed",
    "moved_inside",
    "stayed_can_neutral",
    "stayed_can_enemy",
    "stayed_blocked",
    "stayed_inside2",
    "stayed_deep",
];

#[derive(Clone, Debug, Default)]
pub struct Sum {
    pub plies: u64,
    pub kinds: [u64; 9],
    /// Of the units that stayed and could take a cell, those with an enemy
    /// unit within 2 steps.
    pub guard: [u64; 2],
    /// Of the units that moved inside, those that ended nearer a cell not
    /// ours, and those that had a cell to take from where they stood.
    pub closer: u64,
    pub could: u64,
    pub trained: u64,
    /// Of the blocked units, those next to an enemy unit, and those of level 1.
    pub by_unit: u64,
    pub level1: u64,
}

fn phase(turn: u32) -> &'static str {
    match turn {
        0..=4 => "01-04",
        5..=7 => "05-07",
        8..=10 => "08-10",
        11..=13 => "11-13",
        14..=17 => "14-17",
        _ => "18+",
    }
}

fn cell_of(state: &State, seat: usize, id: u32) -> Option<usize> {
    state
        .units(seat)
        .iter()
        .find(|&c| state.unit_id(c) == Some(id))
}

/// Steps from `cell` to the nearest valid cell that is not active for
/// `seat` (1: a neighbour), up to 9.
fn steps_out(state: &State, seat: usize, cell: usize) -> u32 {
    let valid = state.map().valid;
    let out = valid.and_not(state.active(seat));
    let mut reach = Bits::cell(cell);
    for d in 1..9 {
        reach = (reach | reach.neighbours()) & valid;
        if !(reach & out).is_empty() {
            return d;
        }
    }
    9
}

/// The cells next to `cell`, not active for `seat`, that its unit may enter.
fn takes(state: &State, seat: usize, cell: usize) -> Bits {
    let Some(level) = state.unit_level(cell) else {
        return Bits::EMPTY;
    };
    let valid = state.map().valid;
    Bits::cell(cell).neighbours() & valid.and_not(state.active(seat)) & state.enterable(seat, level)
}

fn scan(game: &Game, sums: &mut BTreeMap<(usize, &'static str), Sum>) {
    for step in &game.steps {
        let start = &step.state;
        let seat = start.to_act();
        let sum = sums.entry((seat, phase(start.game_turn()))).or_default();
        sum.plies += 1;
        let movable = start.movable_units() & start.units(seat);
        let mut sim = start.clone();
        let mut did: HashMap<u32, usize> = HashMap::new();
        for &command in &step.commands {
            if sim.outcome().is_some() {
                break;
            }
            match command {
                Command::Move { id, .. } => {
                    let before = sim.clone();
                    if !sim.apply(command) {
                        continue;
                    }
                    let Some(to) = cell_of(&sim, seat, id) else {
                        continue;
                    };
                    let kind = if before.active(seat).test(to) {
                        3
                    } else if before.units(1 - seat).test(to) {
                        2
                    } else if before.owned(1 - seat).test(to) {
                        1
                    } else {
                        0
                    };
                    did.insert(id, kind);
                }
                Command::Train { .. } => {
                    if sim.apply(command) {
                        sum.trained += 1;
                    }
                }
                _ => {
                    sim.apply(command);
                }
            }
        }
        for cell in movable.iter() {
            let Some(id) = start.unit_id(cell) else {
                continue;
            };
            match did.get(&id) {
                Some(&3) => {
                    sum.kinds[3] += 1;
                    if !takes(start, seat, cell).is_empty() {
                        sum.could += 1;
                    }
                    if let Some(end) = cell_of(&sim, seat, id) {
                        if steps_out(&sim, seat, end) < steps_out(start, seat, cell) {
                            sum.closer += 1;
                        }
                    }
                }
                Some(&kind) => sum.kinds[kind] += 1,
                None => {
                    // It stayed: what it could still do at the end of the turn.
                    let Some(end) = cell_of(&sim, seat, id) else {
                        continue;
                    };
                    let can = takes(&sim, seat, end);
                    let near = Bits::cell(end).neighbours();
                    let near2 = (near | near.neighbours()) & sim.units(1 - seat);
                    if !can.is_empty() {
                        let neutral = can.and_not(sim.owned(1 - seat));
                        let k = if neutral.is_empty() { 5 } else { 4 };
                        sum.kinds[k] += 1;
                        if !near2.is_empty() {
                            sum.guard[k - 4] += 1;
                        }
                    } else {
                        let k = match steps_out(&sim, seat, end) {
                            1 => 6,
                            2 => 7,
                            _ => 8,
                        };
                        sum.kinds[k] += 1;
                        if k == 6 {
                            if !(near & sim.units(1 - seat)).is_empty() {
                                sum.by_unit += 1;
                            }
                            if sim.unit_level(end) == Some(1) {
                                sum.level1 += 1;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn releases(path: &Path) -> Result<HashMap<u64, String>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut out = HashMap::new();
    for line in text.lines() {
        let Ok(value) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if let (Some(id), Some(release)) = (value["game_id"].as_u64(), value["release"].as_str()) {
            out.insert(id, release.trim_start_matches("ice-and-fire-").to_string());
        }
    }
    Ok(out)
}

/// The scan as CSV: a row a group, seat and phase.
pub fn run(request: &Request) -> Result<String, String> {
    let index = request.data.join("index.jsonl");
    let (entries, _) = read_index(&index, request)?;
    let release_of = releases(&index)?;
    let ranks = match request
        .leaderboard
        .clone()
        .or_else(|| newest_snapshot(&request.data))
    {
        Some(path) => read_ranks(&path)?,
        None => BTreeMap::new(),
    };
    let top = request.top as u32;
    let mut all: BTreeMap<(String, usize, &'static str), Sum> = BTreeMap::new();
    let mut games: BTreeMap<(String, usize), u32> = BTreeMap::new();
    for entry in &entries {
        let Ok((meta, record)) = prepare(&request.data, entry, &request.me) else {
            continue;
        };
        let Some(ours) = meta.our_seat else { continue };
        let Some(release) = release_of.get(&meta.game_id) else {
            continue;
        };
        let names = [
            record.players[0].name.clone(),
            record.players[1].name.clone(),
        ];
        let against = if ranks.get(&names[1 - ours]).is_some_and(|&r| r <= top) {
            "top"
        } else {
            "others"
        };
        let game = replay(&meta.game_id.to_string(), record);
        let mut sums = BTreeMap::new();
        scan(&game, &mut sums);
        for ((seat, phase), sum) in sums {
            let group = if seat == ours {
                format!("{release} vs {against}")
            } else {
                format!("{against} vs {release}")
            };
            let into = all.entry((group, seat, phase)).or_default();
            into.plies += sum.plies;
            for k in 0..9 {
                into.kinds[k] += sum.kinds[k];
            }
            into.guard[0] += sum.guard[0];
            into.guard[1] += sum.guard[1];
            into.closer += sum.closer;
            into.could += sum.could;
            into.trained += sum.trained;
            into.by_unit += sum.by_unit;
            into.level1 += sum.level1;
        }
        for seat in 0..2 {
            let group = if seat == ours {
                format!("{release} vs {against}")
            } else {
                format!("{against} vs {release}")
            };
            *games.entry((group, seat)).or_default() += 1;
        }
    }
    let mut out = String::from("group,seat,phase,games,plies");
    for kind in KINDS {
        let _ = write!(out, ",{kind}");
    }
    out.push_str(",guard_neutral,guard_enemy,inside_closer,inside_could,trained,by_unit,level1\n");
    for ((group, seat, phase), sum) in &all {
        let n = games.get(&(group.clone(), *seat)).copied().unwrap_or(0);
        let _ = write!(out, "{group},{seat},{phase},{n},{}", sum.plies);
        for k in sum.kinds {
            let _ = write!(out, ",{k}");
        }
        let _ = writeln!(
            out,
            ",{},{},{},{},{},{},{}",
            sum.guard[0], sum.guard[1], sum.closer, sum.could, sum.trained, sum.by_unit, sum.level1
        );
    }
    Ok(out)
}
