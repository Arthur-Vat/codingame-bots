//! How the sides take the middle chokes in the CodinGame games
//! (`codingame-middle`): phase 1 of the early-game plan
//! (`games/ice-and-fire/research/codingame/2026-11-19-early-game.md`).
//!
//! On maps with at least [`MIN_MIDDLE_CHOKES`] middle chokes (the choke
//! cells of the middle band, [`Zones::middle_chokes`]), for each side up to
//! the end of its ply of the last game turn studied:
//!
//! - when it first holds a middle choke, and its share of them at the start
//!   of its ply of the last turn;
//! - each middle choke it takes: by a unit that moved there (with the steps
//!   from the cell where that unit was trained, and the turns it took beyond
//!   those steps, its detour), by a training on the cell, or otherwise (a
//!   cell taken with no unit of the side on it afterwards); and each it
//!   loses to the enemy;
//! - its trainings: on a neutral cell or not, and the *gap*: the trained
//!   cell's steps to the nearest middle choke it does not hold, less the
//!   fewest such steps among the cells it could have trained on (0 is the
//!   cell nearest the middle);
//! - its unit moves: toward the nearest middle choke it does not hold,
//!   away, or level;
//! - its gold at the start of its plies of turns 8 and 10.
//!
//! On every map, turn by turn ([`flows`]): the cells each side gains during
//! its own plies, split by their owner just before the ply (neutral; the
//! enemy's, never the side's; or the side's own cells that the enemy had
//! taken, retaken), and the cells it loses during the enemy's plies. Lost
//! cells never turn neutral in this game: they pass to the enemy.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;

use ice_and_fire_early::score::MIN_MIDDLE_CHOKES;
use ice_and_fire_early::zones::{share, steps_from, Zones};
use ice_and_fire_engine::{Bits, State, CELLS, HQ};

use crate::profiles::{prepare, read_index, Request};
use crate::replay::{replay, Step};

/// What one side did up to the last turn studied, on a map with middle
/// chokes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SideStats {
    /// The first game turn after whose ply the side held a middle choke.
    pub first_mid: Option<u32>,
    /// Its share of the middle chokes at the start of its ply of the last
    /// turn.
    pub held: f64,
    pub by_move: u32,
    pub by_training: u32,
    pub other: u32,
    /// Middle chokes the enemy took from it.
    pub lost: u32,
    /// For each middle choke taken by a move: the steps from the cell the
    /// unit was trained on, and the turns it took beyond them.
    pub travels: Vec<(u32, i64)>,
    pub trainings: u32,
    pub on_neutral: u32,
    /// Trainings on a cell the side owned just before the ply.
    pub on_own: u32,
    /// Trainings on a cell the enemy owned just before the ply.
    pub on_enemy: u32,
    /// Each training's gap (see the module's doc), when the side held not
    /// every middle choke.
    pub gaps: Vec<u32>,
    pub moves: u32,
    pub toward: u32,
    pub away: u32,
    /// Gold at the start of its plies of turns 8 and 10.
    pub gold: [Option<i64>; 2],
}

/// The side's units, by id, with their cells.
fn units_of(state: &State, seat: usize) -> HashMap<u32, usize> {
    state
        .units(seat)
        .iter()
        .filter_map(|cell| state.unit_id(cell).map(|id| (id, cell)))
        .collect()
}

/// The cells `seat` could train a level-1 unit on, roughly: its active cells
/// and their neighbours, not the enemy's, with no unit or building.
fn trainable(state: &State, seat: usize) -> Bits {
    let active = state.active(seat);
    let occupied = state.units(0) | state.units(1) | state.mines() | state.towers();
    let mut out = ((active | active.neighbours()) & state.map().valid)
        .and_not(state.owned(1 - seat))
        .and_not(occupied);
    for hq in HQ {
        if out.test(hq) {
            out = out.and_not(Bits::cell(hq));
        }
    }
    out
}

/// Both sides' stats over `steps` (with `end` the state after the last),
/// up to game turn `turns`; `None` on a map without middle chokes.
pub fn sides(steps: &[Step], end: &State, turns: u32) -> Option<[SideStats; 2]> {
    let first = steps.first()?;
    let zones = Zones::of(&first.state);
    let mid = zones.middle_chokes();
    if mid.count() < MIN_MIDDLE_CHOKES {
        return None;
    }
    let mut out: [SideStats; 2] = Default::default();
    // Each unit's training turn and cell.
    let mut trained: HashMap<u32, (u32, usize)> = HashMap::new();
    for (i, step) in steps.iter().enumerate() {
        let before = &step.state;
        let turn = before.game_turn();
        if turn > turns {
            break;
        }
        let s = before.to_act();
        let after = steps.get(i + 1).map_or(end, |next| &next.state);
        let stats = &mut out[s];
        if turn == turns {
            stats.held = share(before, s, mid);
        }
        if turn == 8 {
            stats.gold[0] = Some(before.gold(s));
        }
        if turn == 10 {
            stats.gold[1] = Some(before.gold(s));
        }
        let target = mid.and_not(before.owned(s));
        let dist: [u8; CELLS] = steps_from(before, target);
        let units_before = units_of(before, s);
        let units_after = units_of(after, s);

        // Trainings.
        let candidates = trainable(before, s);
        let nearest = candidates.iter().map(|c| dist[c]).min();
        for (&id, &cell) in &units_after {
            if units_before.contains_key(&id) {
                continue;
            }
            trained.insert(id, (turn, cell));
            stats.trainings += 1;
            if before.owned(s).test(cell) {
                stats.on_own += 1;
            } else if before.owned(1 - s).test(cell) {
                stats.on_enemy += 1;
            } else {
                stats.on_neutral += 1;
            }
            if let Some(nearest) = nearest {
                if !target.is_empty() && dist[cell] != u8::MAX && nearest != u8::MAX {
                    stats
                        .gaps
                        .push(u32::from(dist[cell].saturating_sub(nearest)));
                }
            }
        }

        // Moves.
        if !target.is_empty() {
            for (id, &from) in &units_before {
                let Some(&to) = units_after.get(id) else {
                    continue;
                };
                if to == from {
                    continue;
                }
                stats.moves += 1;
                match dist[to].cmp(&dist[from]) {
                    std::cmp::Ordering::Less => stats.toward += 1,
                    std::cmp::Ordering::Greater => stats.away += 1,
                    std::cmp::Ordering::Equal => {}
                }
            }
        }

        // Middle chokes taken, and lost by the enemy.
        let taken = (mid & after.owned(s)).and_not(before.owned(s));
        for cell in taken.iter() {
            let unit = after
                .units(s)
                .test(cell)
                .then(|| after.unit_id(cell))
                .flatten();
            match unit {
                Some(id) if units_before.contains_key(&id) => {
                    stats.by_move += 1;
                    if let Some(&(at, from)) = trained.get(&id) {
                        let steps = steps_from(before, Bits::cell(from))[cell];
                        if steps != u8::MAX {
                            let took = i64::from(turn) - i64::from(at);
                            stats
                                .travels
                                .push((u32::from(steps), took - i64::from(steps)));
                        }
                    }
                }
                Some(_) => stats.by_training += 1,
                None => stats.other += 1,
            }
        }
        if stats.first_mid.is_none() && !(mid & after.owned(s)).is_empty() {
            stats.first_mid = Some(turn);
        }
        let lost = (mid & before.owned(1 - s)).and_not(after.owned(1 - s));
        out[1 - s].lost += lost.count();
    }
    Some(out)
}

/// Where a side's land came from, turn by turn: entry `t` of each vector
/// is game turn `t` (entry 0 unused).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Flows {
    /// Cells gained during its ply that were neutral just before it.
    pub neutral: Vec<u32>,
    /// Cells gained that the enemy held just before and the side never had.
    pub enemy: Vec<u32>,
    /// Cells gained that the enemy held just before and the side had owned
    /// earlier in the game.
    pub retaken: Vec<u32>,
    /// Cells lost during the enemy's ply.
    pub lost: Vec<u32>,
}

impl Flows {
    fn new(turns: u32) -> Flows {
        let n = turns as usize + 1;
        Flows {
            neutral: vec![0; n],
            enemy: vec![0; n],
            retaken: vec![0; n],
            lost: vec![0; n],
        }
    }
}

/// Both sides' land flows over `steps` (with `end` the state after the
/// last), up to the end of the plies of game turn `turns`; on every map.
pub fn flows(steps: &[Step], end: &State, turns: u32) -> [Flows; 2] {
    let mut out = [Flows::new(turns), Flows::new(turns)];
    let Some(first) = steps.first() else {
        return out;
    };
    let mut ever = [first.state.owned(0), first.state.owned(1)];
    for (i, step) in steps.iter().enumerate() {
        let before = &step.state;
        let turn = before.game_turn();
        if turn > turns {
            break;
        }
        let t = turn as usize;
        let s = before.to_act();
        let after = steps.get(i + 1).map_or(end, |next| &next.state);
        let gained = after.owned(s).and_not(before.owned(s));
        let neutral = gained.and_not(before.owned(0) | before.owned(1));
        let theirs = gained & before.owned(1 - s);
        let retaken = theirs & ever[s];
        out[s].neutral[t] += neutral.count();
        out[s].retaken[t] += retaken.count();
        out[s].enemy[t] += theirs.and_not(retaken).count();
        out[1 - s].lost[t] += before.owned(1 - s).and_not(after.owned(1 - s)).count();
        ever[s] |= after.owned(s);
        ever[1 - s] |= after.owned(1 - s);
    }
    out
}

/// The mean over `rows` of each turn's value of `pick`.
fn curve(rows: &[&Flows], turns: u32, pick: impl Fn(&Flows) -> &Vec<u32>) -> Vec<f64> {
    let n = rows.len().max(1) as f64;
    (0..=turns as usize)
        .map(|t| rows.iter().map(|r| f64::from(pick(r)[t])).sum::<f64>() / n)
        .collect()
}

/// The header of [`flow_row`]'s table, its first column named `first`.
pub fn flow_header(first: &str, turns: u32) -> String {
    format!(
        "| {first} | Seat | Games | Gained a turn: neutral / enemy's / retaken | Lost a turn | By turn 10: neutral / from the enemy / lost | By turn {turns}: neutral / from the enemy / lost | Gains from the enemy first exceed neutral gains (turn) |\n| --- | --- | ---: | --- | ---: | --- | --- | ---: |"
    )
}

/// One row of the land-flow table for `rows` (a group and seat): the mean
/// cells gained a turn of each kind and lost a turn over turns 1 to
/// `turns`; the cumulative means by turn 10 and `turns` ("from the enemy"
/// is the enemy's cells and the retaken ones); and the first turn whose
/// mean gains from the enemy exceed its mean neutral gains.
pub fn flow_row(name: &str, seat: usize, rows: &[&Flows], turns: u32) -> String {
    let neutral = curve(rows, turns, |f| &f.neutral);
    let enemy = curve(rows, turns, |f| &f.enemy);
    let retaken = curve(rows, turns, |f| &f.retaken);
    let lost = curve(rows, turns, |f| &f.lost);
    let per_turn = |c: &[f64]| c[1..].iter().sum::<f64>() / f64::from(turns.max(1));
    let by = |c: &[f64], t: u32| c[1..=(t.min(turns) as usize)].iter().sum::<f64>();
    let from_enemy: Vec<f64> = enemy.iter().zip(&retaken).map(|(a, b)| a + b).collect();
    let cross = (1..=turns as usize)
        .find(|&t| from_enemy[t] > neutral[t])
        .map_or_else(|| "-".to_string(), |t| t.to_string());
    format!(
        "| {name} | {} | {} | {:.2} / {:.2} / {:.2} | {:.2} | {:.1} / {:.1} / {:.1} | {:.1} / {:.1} / {:.1} | {cross} |",
        ["red", "blue"][seat],
        rows.len(),
        per_turn(&neutral),
        per_turn(&enemy),
        per_turn(&retaken),
        per_turn(&lost),
        by(&neutral, 10),
        by(&from_enemy, 10),
        by(&lost, 10),
        by(&neutral, turns),
        by(&from_enemy, turns),
        by(&lost, turns),
    )
}

/// The turn-by-turn table of [`flow_row`]'s means for turns 5 to `turns`:
/// a cell is "neutral / from the enemy / lost" for that turn.
pub fn flow_turns(groups: &[(String, usize, Vec<&Flows>)], turns: u32) -> String {
    let mut out = String::from("| Group | Seat |");
    for t in 5..=turns {
        let _ = write!(out, " Turn {t} |");
    }
    out.push_str("\n| --- | --- |");
    for _ in 5..=turns {
        out.push_str(" --- |");
    }
    out.push('\n');
    for (name, seat, rows) in groups {
        let neutral = curve(rows, turns, |f| &f.neutral);
        let enemy = curve(rows, turns, |f| &f.enemy);
        let retaken = curve(rows, turns, |f| &f.retaken);
        let lost = curve(rows, turns, |f| &f.lost);
        let _ = write!(out, "| {name} | {} |", ["red", "blue"][*seat]);
        for t in 5..=turns as usize {
            let _ = write!(
                out,
                " {:.1} / {:.1} / {:.1} |",
                neutral[t],
                enemy[t] + retaken[t],
                lost[t]
            );
        }
        out.push('\n');
    }
    out
}

/// A group of `--players`: `NAME`, or `LABEL=P1+P2+...` for several players
/// pooled under a label.
pub fn parse_group(spec: &str) -> (String, Vec<String>) {
    match spec.split_once('=') {
        Some((label, members)) => (
            label.to_string(),
            members.split('+').map(str::to_string).collect(),
        ),
        None => (spec.to_string(), vec![spec.to_string()]),
    }
}

/// A group's sides in one seat, with the map's number of middle chokes.
type Rows = Vec<(SideStats, u32)>;

fn mean(values: impl Iterator<Item = f64>) -> Option<f64> {
    let (mut sum, mut n) = (0.0, 0.0);
    for v in values {
        sum += v;
        n += 1.0;
    }
    (n > 0.0).then(|| sum / n)
}

fn show(v: Option<f64>, decimals: usize) -> String {
    v.map_or_else(|| "-".to_string(), |v| format!("{v:.decimals$}"))
}

fn pct(part: u32, all: u32) -> String {
    if all == 0 {
        "-".to_string()
    } else {
        format!("{:.0}%", 100.0 * f64::from(part) / f64::from(all))
    }
}

/// The report of `codingame-middle`, as Markdown: the players of `players`
/// one by one, and our releases of `releases` (as the index names them).
pub fn run(
    request: &Request,
    players: &[String],
    releases: &[String],
    turns: u32,
) -> Result<String, String> {
    let index = request.data.join("index.jsonl");
    let (entries, _) = read_index(&index, request)?;
    let release_of = super::releases(&index)?;
    let pools: Vec<(String, Vec<String>)> = players.iter().map(|p| parse_group(p)).collect();
    let mut groups: BTreeMap<(usize, String, usize), Rows> = BTreeMap::new();
    let mut land: BTreeMap<(usize, String, usize), Vec<Flows>> = BTreeMap::new();
    let (mut games, mut without) = (0u32, 0u32);
    for entry in &entries {
        let Ok((meta, record)) = prepare(&request.data, entry, &request.me) else {
            continue;
        };
        let names = [
            record.players[0].name.clone(),
            record.players[1].name.clone(),
        ];
        // Which side belongs to which group, ordered as asked.
        let group_of = |seat: usize| -> Option<(usize, String)> {
            if meta.our_seat == Some(seat) {
                let release = release_of.get(&meta.game_id)?;
                let k = releases.iter().position(|r| r == release)?;
                Some((pools.len() + k, release.clone()))
            } else {
                let k = pools
                    .iter()
                    .position(|(_, members)| members.contains(&names[seat]))?;
                Some((k, pools[k].0.clone()))
            }
        };
        let wanted = [group_of(0), group_of(1)];
        if wanted.iter().all(Option::is_none) {
            continue;
        }
        let game = replay(&meta.game_id.to_string(), record);
        let Some(first) = game.steps.first() else {
            continue;
        };
        let count = Zones::of(&first.state).middle_chokes().count();
        games += 1;
        let flow = flows(&game.steps, &game.end, turns);
        for (seat, group) in wanted.iter().enumerate() {
            if let Some((order, name)) = group {
                land.entry((*order, name.clone(), seat))
                    .or_default()
                    .push(flow[seat].clone());
            }
        }
        let Some(stats) = sides(&game.steps, &game.end, turns) else {
            without += 1;
            continue;
        };
        for (seat, group) in wanted.into_iter().enumerate() {
            if let Some((order, name)) = group {
                groups
                    .entry((order, name, seat))
                    .or_default()
                    .push((stats[seat].clone(), count));
            }
        }
    }

    let mut out = format!(
        "# How the sides take the middle chokes, up to game turn {turns}\n\n{games} games of the groups; {without} on maps with fewer than {MIN_MIDDLE_CHOKES} middle chokes, left out of every table but the land flows. Rows: a group and seat, over the games on maps with middle chokes unless said. Rates are per game unless said.\n"
    );
    let _ = writeln!(
        out,
        "\n## Holding the middle\n\n| Group | Seat | Games | First middle choke (mean turn) | Held one by turn 7 / 8 / 9 | Held at turn {turns} | Taken by a move / a training / otherwise | Lost to the enemy |\n| --- | --- | ---: | ---: | --- | ---: | --- | ---: |"
    );
    for ((_, name, seat), rows) in &groups {
        let n = rows.len() as u32;
        let by = |t: u32| {
            pct(
                rows.iter()
                    .filter(|(r, _)| r.first_mid.is_some_and(|f| f <= t))
                    .count() as u32,
                n,
            )
        };
        let per = |f: &dyn Fn(&SideStats) -> u32| {
            show(mean(rows.iter().map(|(r, _)| f64::from(f(r)))), 2)
        };
        let _ = writeln!(
            out,
            "| {name} | {} | {n} | {} | {} / {} / {} | {} | {} / {} / {} | {} |",
            ["red", "blue"][*seat],
            show(
                mean(rows.iter().filter_map(|(r, _)| r.first_mid.map(f64::from))),
                1
            ),
            by(7),
            by(8),
            by(9),
            show(mean(rows.iter().map(|(r, _)| r.held)), 2),
            per(&|r| r.by_move),
            per(&|r| r.by_training),
            per(&|r| r.other),
            per(&|r| r.lost),
        );
    }

    let _ = writeln!(
        out,
        "\n## The units that take them by moving\n\n| Group | Seat | Captures | Steps from the training cell (mean) | Turns beyond the steps (mean) | Without a detour |\n| --- | --- | ---: | ---: | ---: | ---: |"
    );
    for ((_, name, seat), rows) in &groups {
        let travels: Vec<(u32, i64)> = rows.iter().flat_map(|(r, _)| r.travels.clone()).collect();
        let n = travels.len() as u32;
        let _ = writeln!(
            out,
            "| {name} | {} | {n} | {} | {} | {} |",
            ["red", "blue"][*seat],
            show(mean(travels.iter().map(|t| f64::from(t.0))), 1),
            show(mean(travels.iter().map(|t| t.1 as f64)), 1),
            pct(travels.iter().filter(|t| t.1 <= 0).count() as u32, n),
        );
    }

    let _ = writeln!(
        out,
        "\n## Trainings and moves\n\n| Group | Seat | Trainings | On a neutral cell | Not neutral: on its own / the enemy's | Gap 0 (the cell nearest the middle) | Mean gap | Moves | Toward / away from the middle | Gold at turn 8 / 10 |\n| --- | --- | ---: | ---: | --- | ---: | ---: | ---: | --- | --- |"
    );
    for ((_, name, seat), rows) in &groups {
        let sum = |f: &dyn Fn(&SideStats) -> u32| rows.iter().map(|(r, _)| f(r)).sum::<u32>();
        let gaps: Vec<u32> = rows.iter().flat_map(|(r, _)| r.gaps.clone()).collect();
        let (moves, toward, away) = (sum(&|r| r.moves), sum(&|r| r.toward), sum(&|r| r.away));
        let gold = |k: usize| {
            show(
                mean(rows.iter().filter_map(|(r, _)| r.gold[k].map(|g| g as f64))),
                0,
            )
        };
        let _ = writeln!(
            out,
            "| {name} | {} | {} | {} | {} / {} | {} | {} | {} | {} / {} | {} / {} |",
            ["red", "blue"][*seat],
            show(mean(rows.iter().map(|(r, _)| f64::from(r.trainings))), 2),
            pct(sum(&|r| r.on_neutral), sum(&|r| r.trainings)),
            show(mean(rows.iter().map(|(r, _)| f64::from(r.on_own))), 2),
            show(mean(rows.iter().map(|(r, _)| f64::from(r.on_enemy))), 2),
            pct(
                gaps.iter().filter(|&&g| g == 0).count() as u32,
                gaps.len() as u32
            ),
            show(mean(gaps.iter().map(|&g| f64::from(g))), 2),
            show(mean(rows.iter().map(|(r, _)| f64::from(r.moves))), 2),
            pct(toward, moves),
            pct(away, moves),
            gold(0),
            gold(1),
        );
    }

    let _ = writeln!(
        out,
        "\n## Where the land comes from, on every map\n\nCells gained during the side's own plies, by their owner just before the ply (neutral; the enemy's; retaken: the side's own cells the enemy had taken), and cells lost during the enemy's plies, up to the end of the plies of turn {turns}; means over the group's games, all maps. \"From the enemy\" is the enemy's and the retaken cells together; the last column is the first turn whose mean gains from the enemy exceed its mean neutral gains.\n\n{}",
        flow_header("Group", turns)
    );
    for ((_, name, seat), rows) in &land {
        let rows: Vec<&Flows> = rows.iter().collect();
        let _ = writeln!(out, "{}", flow_row(name, *seat, &rows, turns));
    }
    let by_turn: Vec<(String, usize, Vec<&Flows>)> = land
        .iter()
        .map(|((_, name, seat), rows)| (name.clone(), *seat, rows.iter().collect()))
        .collect();
    let _ = writeln!(
        out,
        "\nTurn by turn (means; neutral / from the enemy / lost):\n\n{}",
        flow_turns(&by_turn, turns)
    );
    let _ = writeln!(
        out,
        "\n## By the map's number of middle chokes\n\n| Group | Seat | Chokes | Games | First middle choke (mean turn) | Held at turn {turns} | Lost to the enemy |\n| --- | --- | --- | ---: | ---: | ---: | ---: |"
    );
    for ((_, name, seat), rows) in &groups {
        for (label, low, high) in [("2 or 3", 2, 3), ("4 or more", 4, u32::MAX)] {
            let part: Vec<&SideStats> = rows
                .iter()
                .filter(|(_, c)| (low..=high).contains(c))
                .map(|(r, _)| r)
                .collect();
            if part.is_empty() {
                continue;
            }
            let _ = writeln!(
                out,
                "| {name} | {} | {label} | {} | {} | {} | {} |",
                ["red", "blue"][*seat],
                part.len(),
                show(
                    mean(part.iter().filter_map(|r| r.first_mid.map(f64::from))),
                    1
                ),
                show(mean(part.iter().map(|r| r.held)), 2),
                show(mean(part.iter().map(|r| f64::from(r.lost))), 2),
            );
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests;
