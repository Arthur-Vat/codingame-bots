//! The style of play of each group of players in the CodinGame games
//! (`ice-and-fire-review codingame-style`): the measures of
//! [research/codingame/2026-11-18-zasmu-strategy.md], computed for named
//! players, the rest of the top N and our releases, seat by seat, so that a
//! release can be compared with the top player on them (curriculum level 0).
//!
//! Each measure is taken per side of a game, then summarised over the
//! group's games by a mean or a median (the table says which). A measure at
//! a game turn is taken at the start of the side's ply of that turn, over
//! the games still running then.
//!
//! [research/codingame/2026-11-18-zasmu-strategy.md]: ../../research/codingame/2026-11-18-zasmu-strategy.md

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;

use ice_and_fire_engine::{Bits, Command, Outcome, State, CELLS, HQ};
use ice_and_fire_tactics::chain::{cost_to, UNREACHABLE};

use crate::profiles::{newest_snapshot, prepare, read_index, read_ranks, Request};
use crate::replay::{replay, Game};

/// How a measure is summarised over a group's games.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Summary {
    Mean,
    Median,
}

/// One measure of the report.
struct Measure {
    section: &'static str,
    name: &'static str,
    summary: Summary,
}

const fn m(section: &'static str, name: &'static str, summary: Summary) -> Measure {
    Measure {
        section,
        name,
        summary,
    }
}

const OPENING: &str = "Opening";
const FRONT: &str = "Holding the front";
const ECONOMY: &str = "Economy";
const STRIKE: &str = "The strike";

/// The measures, in the report's order; [`Side::values`] follows it.
const MEASURES: [Measure; 32] = [
    m(OPENING, "Voronoi share at turn 10", Summary::Mean),
    m(OPENING, "Centre cells held at turn 10", Summary::Mean),
    m(
        OPENING,
        "Trainings at turns 1-10 on the trainable cell nearest the enemy HQ",
        Summary::Mean,
    ),
    m(OPENING, "Moves a ply at turns 10-12", Summary::Mean),
    m(FRONT, "Towers built a game", Summary::Mean),
    m(FRONT, "Towers on the board at turn 15", Summary::Mean),
    m(FRONT, "Towers on the board at turn 20", Summary::Mean),
    m(FRONT, "Towers on the board at turn 25", Summary::Mean),
    m(FRONT, "Towers on the board at turn 30", Summary::Mean),
    m(
        FRONT,
        "Frontier cells protected by a tower at turn 15",
        Summary::Mean,
    ),
    m(
        FRONT,
        "Frontier cells protected by a tower at turn 20",
        Summary::Mean,
    ),
    m(
        FRONT,
        "Frontier cells protected by a tower at turn 25",
        Summary::Mean,
    ),
    m(FRONT, "Plies killing units (turn > 10)", Summary::Mean),
    m(
        FRONT,
        "Unit value lost per opponent reply, turns 11-20",
        Summary::Mean,
    ),
    m(
        ECONOMY,
        "Upkeep share of gross income at turn 20",
        Summary::Mean,
    ),
    m(
        ECONOMY,
        "Upkeep share of gross income at turn 30",
        Summary::Mean,
    ),
    m(
        ECONOMY,
        "Level 3 units on the board at turn 20",
        Summary::Mean,
    ),
    m(ECONOMY, "Mines built a game", Summary::Mean),
    m(
        ECONOMY,
        "First mine, game turn (games with one)",
        Summary::Median,
    ),
    m(ECONOMY, "Mines on the board at turn 12", Summary::Mean),
    m(ECONOMY, "Mines on the board at turn 20", Summary::Mean),
    m(ECONOMY, "Mines built then lost by the end", Summary::Mean),
    m(ECONOMY, "Gross income at turn 20", Summary::Mean),
    m(
        STRIKE,
        "Largest cut, share of the enemy's active cells",
        Summary::Median,
    ),
    m(STRIKE, "Gold before the largest cut", Summary::Median),
    m(
        STRIKE,
        "Gold 2 own turns before a cut of 15+ cells",
        Summary::Median,
    ),
    m(
        STRIKE,
        "Trainings and buildings on the turn before it",
        Summary::Mean,
    ),
    m(
        STRIKE,
        "Wins: distance from its land to the enemy HQ before the winning ply",
        Summary::Median,
    ),
    m(
        STRIKE,
        "Wins: the opponent's income at the winning ply",
        Summary::Median,
    ),
    m(STRIKE, "Wins: game turn", Summary::Median),
    m(
        STRIKE,
        "Wins: turns since gold + 2 incomes first paid the chain to the enemy HQ",
        Summary::Median,
    ),
    m(
        STRIKE,
        "Losses: turns since the opponent's gold + 2 incomes first paid its chain to our HQ",
        Summary::Median,
    ),
];

const VORONOI_10: usize = 0;
const CENTRE_10: usize = 1;
const ADVANCED_TRAINING: usize = 2;
const MOVES_10_12: usize = 3;
const TOWERS_BUILT: usize = 4;
const TOWERS_ON: usize = 5; // 15, 20, 25, 30
const PROTECTED: usize = 9; // 15, 20, 25
const FIGHT_SHARE: usize = 12;
const LOST_IN_REPLY: usize = 13;
const UPKEEP: usize = 14; // 20, 30
const LEVEL3_20: usize = 16;
const MINES_BUILT: usize = 17;
const FIRST_MINE: usize = 18;
const MINES_ON: usize = 19; // 12, 20
const MINES_LOST: usize = 21;
const GROSS_20: usize = 22;
const CUT_SHARE: usize = 23;
const CUT_GOLD: usize = 24;
const BANK_2: usize = 25;
const BANK_SPENDING: usize = 26;
const WIN_DISTANCE: usize = 27;
const WIN_OPPONENT_INCOME: usize = 28;
const WIN_TURN: usize = 29;
const WIN_LEAD: usize = 30;
const LOSS_WARNING: usize = 31;

/// The upkeep of a unit of level 1, 2 or 3 (RULES.md, "Income").
const UPKEEP_OF: [i64; 4] = [0, 1, 4, 20];

/// The turns of a side within which the chain to the enemy HQ must be paid
/// for the lead and warning measures: gold plus this many incomes.
const BANK_TURNS: i64 = 2;

/// The cut, in enemy active cells, from which the gold banked before it is
/// measured.
const BANK_CUT: u32 = 15;

/// The measures of one side of one game.
#[derive(Clone, Debug)]
pub struct Side {
    pub values: [Option<f64>; MEASURES.len()],
}

impl Default for Side {
    fn default() -> Self {
        Side {
            values: [None; MEASURES.len()],
        }
    }
}

/// Steps from `sources` to each cell over the map's non-void cells, `u8::MAX`
/// where none reaches.
fn steps_from(state: &State, sources: Bits) -> [u8; CELLS] {
    let valid = state.map().valid;
    let mut out = [u8::MAX; CELLS];
    let mut reached = sources & valid;
    let mut layer = reached;
    let mut step = 0u8;
    while !layer.is_empty() {
        for cell in layer.iter() {
            out[cell] = step;
        }
        let next = (layer.neighbours() & valid).and_not(reached);
        reached |= next;
        layer = next;
        step = step.saturating_add(1);
    }
    out
}

/// The share of the land nearer `seat`'s cells than the enemy's, over the
/// cells nearer one side.
fn voronoi(state: &State, seat: usize) -> f64 {
    let ours = steps_from(state, state.owned(seat));
    let theirs = steps_from(state, state.owned(1 - seat));
    let (mut near_us, mut near_them) = (0u32, 0u32);
    for cell in state.map().valid.iter() {
        match ours[cell].cmp(&theirs[cell]) {
            std::cmp::Ordering::Less => near_us += 1,
            std::cmp::Ordering::Greater => near_them += 1,
            std::cmp::Ordering::Equal => {}
        }
    }
    let all = near_us + near_them;
    if all == 0 {
        0.0
    } else {
        f64::from(near_us) / f64::from(all)
    }
}

/// The cells within one step of even between the two HQs (the map alone).
fn centre(state: &State) -> Bits {
    let map = state.map();
    let mut out = Bits::EMPTY;
    for cell in map.valid.iter() {
        let (a, b) = (map.distance(cell, HQ[0]), map.distance(cell, HQ[1]));
        if a != u8::MAX && b != u8::MAX && a.abs_diff(b) <= 1 {
            out.set(cell);
        }
    }
    out
}

/// `seat`'s active cells next to an enemy cell, and how many of them are a
/// tower of `seat`'s or next to one.
fn frontier(state: &State, seat: usize) -> (u32, u32) {
    let active = state.active(seat);
    let front = active & state.owned(1 - seat).neighbours();
    let towers = state.towers() & active;
    let guarded = towers | towers.neighbours();
    (front.count(), (front & guarded).count())
}

/// The upkeep `seat` pays: its units on active cells.
fn upkeep(state: &State, seat: usize) -> i64 {
    (state.units(seat) & state.active(seat))
        .iter()
        .filter_map(|cell| state.unit_level(cell))
        .map(|level| UPKEEP_OF[usize::from(level)])
        .sum()
}

/// The training cost of `seat`'s units.
fn unit_value(state: &State, seat: usize) -> i64 {
    state
        .units(seat)
        .iter()
        .filter_map(|cell| state.unit_level(cell))
        .map(|level| 10 * i64::from(level))
        .sum()
}

/// Whether a training of a level 1 unit on `cell` is the nearest to the
/// enemy HQ of the cells where `seat` could train one now.
fn nearest_training(state: &State, seat: usize, cell: usize) -> Option<bool> {
    let map = state.map();
    let target = HQ[1 - seat];
    let candidates = state.enterable(seat, 1) & state.near_active(seat);
    let best = candidates.iter().map(|c| map.distance(c, target)).min()?;
    Some(map.distance(cell, target) <= best)
}

fn mean(values: &[f64]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}

/// The measures of both sides of `game`.
pub fn sides(game: &Game) -> [Side; 2] {
    let mut out = [Side::default(), Side::default()];
    let Some(first) = game.steps.first() else {
        return out;
    };
    let centre = centre(&first.state);
    // The state at the start of each ply, and after the last one.
    let starts: Vec<&State> = game
        .steps
        .iter()
        .map(|s| &s.state)
        .chain(std::iter::once(&game.end))
        .collect();
    let winner = match game.end.outcome() {
        Some(Outcome::Win(w)) if game.hq_taken_by() == Some(w) => Some(w),
        _ => None,
    };
    for (seat, side) in out.iter_mut().enumerate() {
        let v = &mut side.values;
        let plies: Vec<usize> = (0..game.steps.len())
            .filter(|&i| game.steps[i].state.to_act() == seat)
            .collect();
        let (mut nearest, mut trainings) = (0u32, 0u32);
        let mut moves = 0u32;
        let (mut towers, mut mines) = (0u32, Bits::EMPTY);
        let (mut first_mine, mut fights, mut plies_after_10) = (None, 0u32, 0u32);
        let mut lost_in_reply = Vec::new();
        // The largest cut: (cells, share, ply index in `plies`).
        let mut largest: Option<(u32, f64, usize)> = None;
        let mut paid_since: Option<u32> = None;
        for (k, &i) in plies.iter().enumerate() {
            let start = starts[i];
            let next = starts[i + 1];
            let turn = start.game_turn();
            let enemy = 1 - seat;
            // Commands, replayed on a copy of the state.
            let mut sim = start.clone();
            for &command in &game.steps[i].commands {
                let before = sim.clone();
                if !sim.apply(command) {
                    continue;
                }
                match command {
                    Command::Move { .. } if (10..=12).contains(&turn) => {
                        moves += 1;
                    }
                    Command::Train { x, y, .. } if turn <= 10 => {
                        let cell = usize::from(y) * ice_and_fire_engine::SIZE + usize::from(x);
                        if let Some(is_nearest) = nearest_training(&before, seat, cell) {
                            trainings += 1;
                            nearest += u32::from(is_nearest);
                        }
                    }
                    Command::BuildTower { .. } => towers += 1,
                    Command::BuildMine { x, y } => {
                        mines.set(usize::from(y) * ice_and_fire_engine::SIZE + usize::from(x));
                        first_mine.get_or_insert(turn);
                    }
                    _ => {}
                }
            }
            let at = |t: u32| turn == t;
            if at(10) {
                v[VORONOI_10] = Some(voronoi(start, seat));
                v[CENTRE_10] = Some(f64::from((start.owned(seat) & centre).count()));
            }
            for (j, t) in [15, 20, 25, 30].into_iter().enumerate() {
                if at(t) {
                    v[TOWERS_ON + j] =
                        Some(f64::from((start.towers() & start.owned(seat)).count()));
                }
            }
            for (j, t) in [15, 20, 25].into_iter().enumerate() {
                if at(t) {
                    let (front, guarded) = frontier(start, seat);
                    if front > 0 {
                        v[PROTECTED + j] = Some(f64::from(guarded) / f64::from(front));
                    }
                }
            }
            for (j, t) in [20, 30].into_iter().enumerate() {
                if at(t) {
                    let paid = upkeep(start, seat);
                    let gross = start.income(seat) + paid;
                    if gross > 0 {
                        v[UPKEEP + j] = Some(paid as f64 / gross as f64);
                    }
                    if t == 20 {
                        v[GROSS_20] = Some(gross as f64);
                    }
                }
            }
            if at(20) {
                let level3 = start
                    .units(seat)
                    .iter()
                    .filter(|&c| start.unit_level(c) == Some(3))
                    .count();
                v[LEVEL3_20] = Some(level3 as f64);
            }
            for (j, t) in [12, 20].into_iter().enumerate() {
                if at(t) {
                    v[MINES_ON + j] = Some(f64::from((start.mines() & start.owned(seat)).count()));
                }
            }
            // What the ply did to the enemy, by the start of its next ply.
            let killed = unit_value(start, enemy) - unit_value(next, enemy);
            if turn > 10 {
                plies_after_10 += 1;
                fights += u32::from(killed > 0);
            }
            let before_cells = start.active(enemy).count();
            let cut = before_cells.saturating_sub(next.active(enemy).count());
            if before_cells > 0 && largest.is_none_or(|(c, _, _)| cut > c) {
                largest = Some((cut, f64::from(cut) / f64::from(before_cells), k));
            }
            // What the reply did to us.
            if (11..=20).contains(&turn) {
                if let Some(&after) = starts.get(i + 2) {
                    lost_in_reply.push((unit_value(next, seat) - unit_value(after, seat)) as f64);
                }
            }
            // Gold toward the chain to the enemy HQ.
            if paid_since.is_none() {
                let cost = cost_to(start, seat, false, HQ[enemy]);
                if cost != UNREACHABLE && start.gold(seat) + BANK_TURNS * start.income(seat) >= cost
                {
                    paid_since = Some(turn);
                }
            }
        }
        if trainings > 0 {
            v[ADVANCED_TRAINING] = Some(f64::from(nearest) / f64::from(trainings));
        }
        let ply_10_12 = plies
            .iter()
            .filter(|&&i| (10..=12).contains(&starts[i].game_turn()))
            .count();
        if ply_10_12 > 0 {
            v[MOVES_10_12] = Some(f64::from(moves) / ply_10_12 as f64);
        }
        v[TOWERS_BUILT] = Some(f64::from(towers));
        v[MINES_BUILT] = Some(f64::from(mines.count()));
        v[FIRST_MINE] = first_mine.map(f64::from);
        if !mines.is_empty() {
            let lost = mines.and_not(game.end.owned(seat)).count();
            v[MINES_LOST] = Some(f64::from(lost) / f64::from(mines.count()));
        }
        if plies_after_10 > 0 {
            v[FIGHT_SHARE] = Some(f64::from(fights) / f64::from(plies_after_10));
        }
        v[LOST_IN_REPLY] = mean(&lost_in_reply);
        if let Some((cells, share, k)) = largest {
            let start = starts[plies[k]];
            v[CUT_SHARE] = Some(share);
            v[CUT_GOLD] = Some(start.gold(seat) as f64);
            if cells >= BANK_CUT && k >= 2 {
                v[BANK_2] = Some(starts[plies[k - 2]].gold(seat) as f64);
            }
            if cells >= BANK_CUT && k >= 1 {
                let spending = game.steps[plies[k - 1]]
                    .commands
                    .iter()
                    .filter(|c| {
                        matches!(
                            c,
                            Command::Train { .. }
                                | Command::BuildMine { .. }
                                | Command::BuildTower { .. }
                        )
                    })
                    .count();
                v[BANK_SPENDING] = Some(spending as f64);
            }
        }
        if winner == Some(seat) {
            if let Some(&last) = plies.last() {
                let start = starts[last];
                let near = steps_from(start, start.active(seat))[HQ[1 - seat]];
                if near != u8::MAX {
                    v[WIN_DISTANCE] = Some(f64::from(near));
                }
                v[WIN_OPPONENT_INCOME] = Some(start.income(1 - seat) as f64);
                v[WIN_TURN] = Some(f64::from(start.game_turn()));
                if let Some(t) = paid_since {
                    v[WIN_LEAD] = Some(f64::from(start.game_turn() - t));
                }
            }
        }
    }
    // The warning a loser had: its opponent's lead.
    if let Some(w) = winner {
        out[1 - w].values[LOSS_WARNING] = out[w].values[WIN_LEAD];
    }
    out
}

/// The report's summary of one measure over a group's values.
fn summarise(summary: Summary, values: &mut [f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    match summary {
        Summary::Mean => mean(values),
        Summary::Median => {
            values.sort_by(f64::total_cmp);
            let n = values.len();
            Some(if n % 2 == 1 {
                values[n / 2]
            } else {
                (values[n / 2 - 1] + values[n / 2]) / 2.0
            })
        }
    }
}

/// Our releases from the index's lines (`release`, without the game's
/// prefix).
fn releases(data: &std::path::Path) -> Result<HashMap<u64, String>, String> {
    let path = data.join("index.jsonl");
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut out = HashMap::new();
    for line in text.lines() {
        let Ok(value) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if let (Some(id), Some(release)) = (value["game_id"].as_u64(), value["release"].as_str()) {
            out.insert(id, release.trim_start_matches("ice-and-fire-").to_string());
        }
    }
    Ok(out)
}

/// The report, as Markdown: one column per group and seat. The groups are
/// the players of `request.players` (each its own), the rest of the top
/// `request.top` of the leaderboard snapshot ("top N"), and our releases in
/// games against them (`request.release` keeps the games of one release).
pub fn run(request: &Request) -> Result<String, String> {
    let (entries, mut left_out) = read_index(&request.data.join("index.jsonl"), request)?;
    let release_of = releases(&request.data)?;
    let ranks = match request
        .leaderboard
        .clone()
        .or_else(|| newest_snapshot(&request.data))
    {
        Some(path) => read_ranks(&path)?,
        None => BTreeMap::new(),
    };
    let top = request.top as u32;
    let group_of = |name: &str| -> Option<String> {
        if request.players.iter().any(|p| p == name) {
            Some(name.to_string())
        } else if top > 0 && ranks.get(name).is_some_and(|&r| r <= top) {
            Some(format!("top {top}"))
        } else {
            None
        }
    };
    type Done = Result<Vec<(String, usize, Side)>, (Option<u64>, String)>;
    let jobs = request.jobs.clamp(1, 64);
    let size = entries.len().div_ceil(jobs).max(1);
    let mut slots: Vec<Option<Done>> = Vec::new();
    slots.resize_with(entries.len(), || None);
    std::thread::scope(|scope| {
        for (chunk, out) in entries.chunks(size).zip(slots.chunks_mut(size)) {
            let (group_of, release_of) = (&group_of, &release_of);
            scope.spawn(move || {
                for (entry, slot) in chunk.iter().zip(out.iter_mut()) {
                    *slot = Some(match prepare(&request.data, entry, &request.me) {
                        Err(left) => Err((left.game_id, left.why)),
                        Ok((meta, record)) => {
                            let Some(us) = meta.our_seat else {
                                *slot = Some(Ok(Vec::new()));
                                continue;
                            };
                            let names = [
                                record.players[0].name.clone(),
                                record.players[1].name.clone(),
                            ];
                            let Some(theirs) = group_of(&names[1 - us]) else {
                                *slot = Some(Ok(Vec::new()));
                                continue;
                            };
                            let game = replay(&meta.game_id.to_string(), record);
                            let [a, b] = sides(&game);
                            let mut both = [Some(a), Some(b)];
                            let mut out =
                                vec![(theirs, 1 - us, both[1 - us].take().unwrap_or_default())];
                            if let Some(release) = release_of.get(&meta.game_id) {
                                out.push((
                                    release.clone(),
                                    us,
                                    both[us].take().unwrap_or_default(),
                                ));
                            }
                            Ok(out)
                        }
                    });
                }
            });
        }
    });
    let mut groups: BTreeMap<(String, usize), Vec<Side>> = BTreeMap::new();
    for slot in slots.into_iter().flatten() {
        match slot {
            Ok(sides) => {
                for (group, seat, side) in sides {
                    groups.entry((group, seat)).or_default().push(side);
                }
            }
            Err((game_id, why)) => left_out.push(cg_review::profile::LeftOut { game_id, why }),
        }
    }
    Ok(report(&groups, left_out.len()))
}

/// The report's text from the sides of each group and seat.
fn report(groups: &BTreeMap<(String, usize), Vec<Side>>, left_out: usize) -> String {
    let mut text = String::new();
    let _ = writeln!(text, "# Style of play by group\n");
    let _ = writeln!(
        text,
        "Measures of games/ice-and-fire/research/codingame/2026-11-18-zasmu-strategy.md, per side of a game, summarised over each group's games (a measure at a turn: over the games still running then). Red is seat 0, blue seat 1. Games left out: {left_out}.\n"
    );
    let columns: Vec<String> = groups
        .iter()
        .map(|((group, seat), sides)| {
            format!(
                "{group} {} ({})",
                if *seat == 0 { "red" } else { "blue" },
                sides.len()
            )
        })
        .collect();
    let mut section = "";
    for (index, measure) in MEASURES.iter().enumerate() {
        if measure.section != section {
            section = measure.section;
            let _ = writeln!(text, "\n## {section}\n");
            let _ = writeln!(text, "| Measure | {} |", columns.join(" | "));
            let _ = writeln!(text, "| --- |{}", " --- |".repeat(columns.len()));
        }
        let cells: Vec<String> = groups
            .values()
            .map(|sides| {
                let mut values: Vec<f64> = sides.iter().filter_map(|s| s.values[index]).collect();
                match summarise(measure.summary, &mut values) {
                    Some(x) if x.abs() < 1.0 && x != 0.0 => format!("{x:.2}"),
                    Some(x) => format!("{x:.1}"),
                    None => "-".to_string(),
                }
            })
            .collect();
        let how = match measure.summary {
            Summary::Mean => "mean",
            Summary::Median => "median",
        };
        let _ = writeln!(text, "| {} ({how}) | {} |", measure.name, cells.join(" | "));
    }
    text
}

#[cfg(test)]
mod tests;
