//! The opening study of the CodinGame games (`ice-and-fire-review
//! codingame-opening`, stage S0 of the goal-directed opening): how each
//! group of players (our releases, the top players, the rest) uses its units
//! and its land in the first turns, seat by seat.
//!
//! - **Unit turns.** Each turn of a unit that may move (not trained this
//!   turn) is one of: it took a neutral cell, it took an enemy cell, or it
//!   took nothing. A unit that took nothing is *beaten* when it had a
//!   neutral neighbour at the start of the turn but our other commands of
//!   the same turn took them all (two units after the same land), *free*
//!   when a neutral neighbour was still there (it chose not to take one;
//!   *free by the enemy* when every such neighbour touches an enemy cell),
//!   *enclosed near* when it had no neutral neighbour but one 2 steps away,
//!   and *enclosed far* otherwise.
//! - **Trainings.** For each unit trained up to the last turn studied, the
//!   cells (neutral or enemy) it took in its next [`WINDOW`] turns, when the
//!   game lasted that long.
//! - **Zones.** Each cell is in a side's *home* when it is at least 3 steps
//!   nearer that side's HQ than the enemy HQ, in the *middle* within 2 steps
//!   of even, and *far* otherwise (steps over non-void cells, the map alone).
//!   At the start of a side's turn: the cells it holds in each zone, and the
//!   cells of its home and of the middle the enemy holds.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use ice_and_fire_engine::{Bits, Command, State, HQ};
use ice_and_fire_opening::econ::distances;
use serde_json::Value;

use crate::profiles::{newest_snapshot, prepare, read_index, read_ranks, Request};
use crate::replay::{replay, Game};

/// Turns after a training in which its unit's captures are counted.
pub const WINDOW: u32 = 5;
/// A cell is in a side's home when it is at least this many steps nearer
/// that side's HQ than the enemy's; in the middle when the difference is
/// smaller.
pub const HOME: i32 = 3;

/// What became of one unit turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnitTurn {
    Neutral,
    Enemy,
    Beaten,
    Free,
    FreeByEnemy,
    EnclosedNear,
    EnclosedFar,
}

/// The counts of one side's ply.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Ply {
    pub game_turn: u32,
    /// Unit turns by [`UnitTurn`], in its order.
    pub units: [u32; 7],
    /// Active cells at the start of the ply.
    pub cells: u32,
    /// Cells held at the start of the ply: home, middle, far.
    pub held: [u32; 3],
    /// Cells of our home and of the middle the enemy holds.
    pub lost: [u32; 2],
    /// The sizes of our home and of the middle.
    pub sizes: [u32; 2],
    pub trainings: u32,
    pub mines: u32,
}

/// One training: its game turn, and the cells its unit took in its next
/// [`WINDOW`] turns (`None` when the game ended sooner).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Training {
    pub game_turn: u32,
    pub taken: Option<u32>,
}

/// One side of one game.
#[derive(Clone, Debug, Default)]
pub struct Side {
    pub plies: Vec<Ply>,
    pub trainings: Vec<Training>,
}

/// The cell of our unit `id`, if it is alive.
fn cell_of(state: &State, seat: usize, id: u32) -> Option<usize> {
    state
        .units(seat)
        .iter()
        .find(|&cell| state.unit_id(cell) == Some(id))
}

fn neutral(state: &State) -> Bits {
    state.map().valid.and_not(state.owned(0) | state.owned(1))
}

/// The zone of each cell for `seat`: 0 home, 1 middle, 2 far; 3 void.
fn zones(state: &State, seat: usize) -> [u8; ice_and_fire_engine::CELLS] {
    let ours = distances(state, Bits::cell(HQ[seat]));
    let theirs = distances(state, Bits::cell(HQ[1 - seat]));
    let mut out = [3; ice_and_fire_engine::CELLS];
    for cell in state.map().valid.iter() {
        let diff = i32::from(ours[cell]) - i32::from(theirs[cell]);
        out[cell] = if diff <= -HOME {
            0
        } else if diff < HOME {
            1
        } else {
            2
        };
    }
    out
}

/// What a unit with no capture this turn had: [`UnitTurn::Beaten`],
/// [`UnitTurn::Free`] or enclosed, from the state at the start of the ply
/// and after it.
fn idle(start: &State, end: &State, seat: usize, cell: usize) -> UnitTurn {
    let free = neutral(start);
    let next = Bits::cell(cell).neighbours() & free;
    if !next.is_empty() {
        if (next & neutral(end)).is_empty() && (next & end.owned(seat)) == next {
            return UnitTurn::Beaten;
        }
        let quiet = next
            .iter()
            .any(|c| (Bits::cell(c).neighbours() & start.owned(1 - seat)).is_empty());
        return if quiet {
            UnitTurn::Free
        } else {
            UnitTurn::FreeByEnemy
        };
    }
    let ring = Bits::cell(cell).neighbours().neighbours() & free;
    if ring.is_empty() {
        UnitTurn::EnclosedFar
    } else {
        UnitTurn::EnclosedNear
    }
}

/// Both sides of `game` up to game turn `turns`.
pub fn study(game: &Game, turns: u32) -> [Side; 2] {
    let mut sides = [Side::default(), Side::default()];
    let Some(first) = game.steps.first() else {
        return sides;
    };
    let zone = [zones(&first.state, 0), zones(&first.state, 1)];
    // Per unit id: its seat, training turn, and the turns it took cells.
    let mut trained: HashMap<u32, (usize, u32)> = HashMap::new();
    let mut captures: HashMap<u32, Vec<u32>> = HashMap::new();
    let last_turn = game.steps.last().map_or(0, |step| step.state.game_turn());
    for step in &game.steps {
        let start = &step.state;
        let seat = start.to_act();
        let turn = start.game_turn();
        if turn > turns + WINDOW {
            break;
        }
        let study = turn <= turns;
        let mut sim = start.clone();
        let mut outcome: HashMap<u32, UnitTurn> = HashMap::new();
        let movable = start.movable_units() & start.units(seat);
        let mut ply = Ply {
            game_turn: turn,
            cells: start.active(seat).count(),
            ..Ply::default()
        };
        for cell in start.owned(seat).iter() {
            if zone[seat][cell] < 3 {
                ply.held[usize::from(zone[seat][cell])] += 1;
            }
        }
        for cell in start.owned(1 - seat).iter() {
            if zone[seat][cell] < 2 {
                ply.lost[usize::from(zone[seat][cell])] += 1;
            }
        }
        for cell in start.map().valid.iter() {
            if zone[seat][cell] < 2 {
                ply.sizes[usize::from(zone[seat][cell])] += 1;
            }
        }
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
                    if before.owned(seat).test(to) {
                        continue;
                    }
                    let taken = if before.owned(1 - seat).test(to) {
                        UnitTurn::Enemy
                    } else {
                        UnitTurn::Neutral
                    };
                    outcome.insert(id, taken);
                    captures.entry(id).or_default().push(turn);
                }
                Command::Train { x, y, .. } => {
                    if sim.apply(command) {
                        let cell = usize::from(y) * ice_and_fire_engine::SIZE + usize::from(x);
                        if let Some(id) = sim.unit_id(cell) {
                            trained.insert(id, (seat, turn));
                        }
                        ply.trainings += 1;
                    }
                }
                Command::BuildMine { .. } => {
                    if sim.apply(command) {
                        ply.mines += 1;
                    }
                }
                _ => {
                    sim.apply(command);
                }
            }
        }
        if !study {
            continue;
        }
        for cell in movable.iter() {
            let Some(id) = start.unit_id(cell) else {
                continue;
            };
            let kind = outcome
                .get(&id)
                .copied()
                .unwrap_or_else(|| idle(start, &sim, seat, cell));
            ply.units[kind as usize] += 1;
        }
        sides[seat].plies.push(ply);
    }
    for (&id, &(seat, turn)) in &trained {
        if turn > turns {
            continue;
        }
        let taken = (last_turn >= turn + WINDOW).then(|| {
            captures.get(&id).map_or(0, |at| {
                at.iter()
                    .filter(|&&t| t > turn && t <= turn + WINDOW)
                    .count() as u32
            })
        });
        sides[seat].trainings.push(Training {
            game_turn: turn,
            taken,
        });
    }
    sides
}

/// Our release of each game, from the index (`release`).
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

/// The sums of a group's plies at one turn and seat.
#[derive(Clone, Debug, Default)]
struct Sum {
    plies: u32,
    units: [u64; 7],
    cells: u64,
    held: [u64; 3],
    lost: [u64; 2],
    sizes: [u64; 2],
    trainings: u64,
    mines: u64,
    /// Plies with a mine built so far in the game.
    mined: u32,
}

/// The study's report, as Markdown: per seat, per group, per turn.
pub fn run(request: &Request, turns: u32) -> Result<String, String> {
    let index = request.data.join("index.jsonl");
    let (entries, mut left_out) = read_index(&index, request)?;
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
    let jobs = request.jobs.clamp(1, 64);
    let size = entries.len().div_ceil(jobs).max(1);
    type Done = Result<(Vec<(String, usize, Side)>, u64), (Option<u64>, String)>;
    let mut slots: Vec<Option<Done>> = Vec::new();
    slots.resize_with(entries.len(), || None);
    std::thread::scope(|scope| {
        for (chunk, out) in entries.chunks(size).zip(slots.chunks_mut(size)) {
            let (ranks, release_of) = (&ranks, &release_of);
            scope.spawn(move || {
                for (entry, slot) in chunk.iter().zip(out.iter_mut()) {
                    *slot = Some(match prepare(&request.data, entry, &request.me) {
                        Err(left) => Err((left.game_id, left.why)),
                        Ok((meta, record)) => {
                            let names = [
                                record.players[0].name.clone(),
                                record.players[1].name.clone(),
                            ];
                            let game = replay(&meta.game_id.to_string(), record);
                            let sides = study(&game, turns);
                            let mut out = Vec::new();
                            for (seat, side) in sides.into_iter().enumerate() {
                                let group = if meta.our_seat == Some(seat) {
                                    match release_of.get(&meta.game_id) {
                                        Some(release) => release.clone(),
                                        None => continue,
                                    }
                                } else if meta.our_seat.is_none() {
                                    continue;
                                } else if ranks.get(&names[seat]).is_some_and(|&r| r <= top) {
                                    format!("top {top}")
                                } else {
                                    "others".to_string()
                                };
                                out.push((group, seat, side));
                            }
                            Ok((out, meta.game_id))
                        }
                    });
                }
            });
        }
    });
    let mut sums: BTreeMap<(usize, String, u32), Sum> = BTreeMap::new();
    let mut games: BTreeMap<(usize, String), u32> = BTreeMap::new();
    let mut trainings: BTreeMap<(usize, String, u32), (u32, u32, u32)> = BTreeMap::new();
    let mut wasted: BTreeMap<(usize, String), (u32, u64, u64)> = BTreeMap::new();
    for slot in slots.into_iter().flatten() {
        let (sides, _) = match slot {
            Ok(done) => done,
            Err((id, why)) => {
                left_out.push(cg_review::profile::LeftOut { game_id: id, why });
                continue;
            }
        };
        for (group, seat, side) in sides {
            *games.entry((seat, group.clone())).or_default() += 1;
            let mut mined = false;
            let mut idle_units = 0u64;
            let mut all_units = 0u64;
            for ply in &side.plies {
                mined |= ply.mines > 0;
                let sum = sums
                    .entry((seat, group.clone(), ply.game_turn))
                    .or_default();
                sum.plies += 1;
                for (total, n) in sum.units.iter_mut().zip(ply.units) {
                    *total += u64::from(n);
                }
                sum.cells += u64::from(ply.cells);
                for (total, n) in sum.held.iter_mut().zip(ply.held) {
                    *total += u64::from(n);
                }
                for (total, n) in sum.lost.iter_mut().zip(ply.lost) {
                    *total += u64::from(n);
                }
                for (total, n) in sum.sizes.iter_mut().zip(ply.sizes) {
                    *total += u64::from(n);
                }
                sum.trainings += u64::from(ply.trainings);
                sum.mines += u64::from(ply.mines);
                sum.mined += u32::from(mined);
                if ply.game_turn <= 10 {
                    all_units += ply.units.iter().map(|&n| u64::from(n)).sum::<u64>();
                    idle_units += ply.units[2..].iter().map(|&n| u64::from(n)).sum::<u64>();
                }
            }
            let w = wasted.entry((seat, group.clone())).or_default();
            if side.plies.iter().any(|p| p.game_turn == 10) {
                w.0 += 1;
                w.1 += idle_units;
                w.2 += all_units;
            }
            for t in side.trainings {
                let bucket = match t.game_turn {
                    0..=4 => 4,
                    5..=7 => 7,
                    8..=10 => 10,
                    _ => turns,
                };
                let Some(taken) = t.taken else { continue };
                let e = trainings.entry((seat, group.clone(), bucket)).or_default();
                e.0 += 1;
                e.1 += taken;
                e.2 += u32::from(taken == 0);
            }
        }
    }
    let mut md = String::new();
    let _ = writeln!(
        md,
        "# The opening: unit turns, trainings and zones\n\nGame turns 1 to {turns} of the CodinGame games in `{}`, by the seat of the side studied (0 red, 1 blue). Groups: our releases (from the index), the top {top} of the leaderboard snapshot, and the others we met. Commands are those carried out.\n",
        request.data.display()
    );
    let _ = writeln!(md, "## Games\n\n| Seat | Group | Games | Unit turns idle by turn 10 (games reaching it) | Share idle |\n| ---: | --- | ---: | ---: | ---: |");
    for ((seat, group), n) in &games {
        let (g, idle, all) = wasted
            .get(&(*seat, group.clone()))
            .cloned()
            .unwrap_or_default();
        let _ = writeln!(
            md,
            "| {seat} | {group} | {n} | {:.1} ({g}) | {} |",
            if g == 0 {
                0.0
            } else {
                idle as f64 / f64::from(g)
            },
            pct(idle, all)
        );
    }
    let _ = writeln!(md, "\n## Unit turns\n\nShares of the turns of units that could move. *Beaten*: a neutral neighbour at the start, all taken by our other commands of the turn. *Free*: a neutral neighbour left, not taken; *by the enemy* when each such neighbour touches an enemy cell. *Enclosed*: no neutral neighbour; *near* when one is 2 steps away.\n\n| Seat | Group | Turn | Plies | Units a ply | Neutral | Enemy | Beaten | Free | Free by the enemy | Enclosed near | Enclosed far |\n| ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");
    for ((seat, group, turn), s) in &sums {
        let all: u64 = s.units.iter().sum();
        let _ = writeln!(
            md,
            "| {seat} | {group} | {turn} | {} | {:.1} | {} | {} | {} | {} | {} | {} | {} |",
            s.plies,
            all as f64 / f64::from(s.plies),
            pct(s.units[0], all),
            pct(s.units[1], all),
            pct(s.units[2], all),
            pct(s.units[3], all),
            pct(s.units[4], all),
            pct(s.units[5], all),
            pct(s.units[6], all)
        );
    }
    let _ = writeln!(md, "\n## Zones\n\nMeans at the start of the side's turn. Home: cells at least {HOME} steps nearer our HQ than the enemy's; middle: within {} steps of even. *Lost*: held by the enemy.\n\n| Seat | Group | Turn | Plies | Active cells | Home held | Home lost | Home size | Middle held | Middle lost | Middle size | Far held | Trainings | Mine built so far |\n| ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |", HOME - 1);
    for ((seat, group, turn), s) in &sums {
        let n = f64::from(s.plies);
        let m = |x: u64| x as f64 / n;
        let _ = writeln!(
            md,
            "| {seat} | {group} | {turn} | {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.2} | {} |",
            s.plies,
            m(s.cells),
            m(s.held[0]),
            m(s.lost[0]),
            m(s.sizes[0]),
            m(s.held[1]),
            m(s.lost[1]),
            m(s.sizes[1]),
            m(s.held[2]),
            m(s.trainings),
            pct(u64::from(s.mined), u64::from(s.plies))
        );
    }
    let _ = writeln!(md, "\n## Trainings\n\nUnits trained by game turn {turns}, by when (up to the turn named), with the cells each took (neutral or enemy) in its next {WINDOW} turns; trainings in games that ended sooner are left out.\n\n| Seat | Group | Trained by turn | Trainings | Cells taken a training | None taken |\n| ---: | --- | ---: | ---: | ---: | ---: |");
    for ((seat, group, bucket), (n, taken, none)) in &trainings {
        let _ = writeln!(
            md,
            "| {seat} | {group} | {bucket} | {n} | {:.2} | {} |",
            f64::from(*taken) / f64::from(*n),
            pct(u64::from(*none), u64::from(*n))
        );
    }
    if !left_out.is_empty() {
        let _ = writeln!(md, "\n## Left out\n\n{} games:\n", left_out.len());
        for left in left_out.iter().take(20) {
            let _ = writeln!(md, "- {:?}: {}", left.game_id, left.why);
        }
    }
    Ok(md)
}

fn pct(part: u64, all: u64) -> String {
    if all == 0 {
        "-".to_string()
    } else {
        format!("{:.0}%", 100.0 * part as f64 / all as f64)
    }
}

#[cfg(test)]
mod tests;
