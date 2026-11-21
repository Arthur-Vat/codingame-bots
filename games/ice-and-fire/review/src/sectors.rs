//! `codingame-sectors`: who takes which part of the map in the early game
//! (the owner, 2026-11-21: "identify goals from the start of the game and
//! assign them afterwards"; he saw us leave the bottom-left corner to
//! zasmu).
//!
//! The map is cut into three lanes across the line even between the HQs, by
//! a cell's `x − y` (both HQs have 0): the flank of the bottom-left corner
//! (`x − y ≤ −4`), the centre, and the flank of the top-right corner
//! (`x − y ≥ 4`). The map is symmetric by a half turn, so a lane is the same
//! for both seats. For each game, each side and each game turn up to
//! `turns` (at the start of red's ply), a CSV row per lane: the valid cells
//! of the lane, the side's active cells in it, the lane's valid cells
//! strictly nearer the side's active land than the enemy's (its Voronoi
//! share there), and the first game turn the side held an active cell of
//! the lane's middle (ring at most 1, `solo::rings`).

use std::fmt::Write as _;

use ice_and_fire_engine::{Bits, State, CELLS, SIZE};
use ice_and_fire_opening::econ::distances;
use ice_and_fire_opening::solo::rings;

use crate::profiles::{prepare, read_index, Request};
use crate::replay::{replay, Game};
use crate::style::releases;

/// The lanes, by name.
pub const LANES: [&str; 3] = ["bottom_left", "middle", "top_right"];

/// The lane of `cell`: 0, 1 or 2 as in [`LANES`].
pub fn lane(cell: usize) -> usize {
    let (x, y) = ((cell % SIZE) as i64, (cell / SIZE) as i64);
    match x - y {
        d if d <= -4 => 0,
        d if d >= 4 => 2,
        _ => 1,
    }
}

/// The fewest contested cells of a front ([`fronts`]).
const FRONT_MIN: u32 = 4;

/// The fronts of the map of `state` (E080, the owner: some maps have only
/// two flanks, others only a middle): the cells within one ring of the line
/// even between the HQs, cut by their place along that line into the
/// bottom-left flank, the middle and the top-right flank (the lanes of
/// [`lane`]); a place is a front only when it has at least [`FRONT_MIN`]
/// such valid cells. Every valid cell then goes to its nearest front (steps
/// over valid cells; on a tie, the front in the cell's own place, then the
/// middle). Returns, per front, its name, its region and its contested
/// cells.
pub fn fronts(state: &State) -> Vec<(String, Bits, Bits)> {
    let ring = rings(state);
    let valid = state.map().valid;
    let mut parts = [Bits::EMPTY; 3];
    for c in valid.iter() {
        if ring[c] <= 1 {
            parts[lane(c)].set(c);
        }
    }
    let kept: Vec<usize> = (0..3).filter(|&l| parts[l].count() >= FRONT_MIN).collect();
    if kept.is_empty() {
        return vec![(LANES[1].to_string(), valid, parts[0] | parts[1] | parts[2])];
    }
    let d: Vec<[u8; CELLS]> = kept.iter().map(|&l| distances(state, parts[l])).collect();
    let mut regions = vec![Bits::EMPTY; kept.len()];
    for c in valid.iter() {
        // On a tie, the front in the cell's own place, then the middle.
        let own = |i: usize| match kept[i] {
            l if l == lane(c) => 0,
            1 => 1,
            _ => 2,
        };
        let best = (0..kept.len())
            .min_by_key(|&i| (d[i][c], own(i)))
            .unwrap_or(0);
        regions[best].set(c);
    }
    kept.iter()
        .zip(regions)
        .map(|(&l, region)| (LANES[l].to_string(), region, parts[l]))
        .collect()
}

/// The CSV rows of one game (see the module's docs), with `prefix` (the
/// game's own columns) at the start of each; with `by_fronts`, by the map's
/// [`fronts`] rather than the three lanes.
fn rows(
    game: &Game,
    turns: u32,
    prefix: &str,
    names: [&str; 2],
    by_fronts: bool,
    out: &mut String,
) {
    let Some(first) = game.steps.first() else {
        return;
    };
    let ring = rings(&first.state);
    let valid = first.state.map().valid;
    let (lane_names, lanes, middle): (Vec<String>, Vec<Bits>, Vec<Bits>) = if by_fronts {
        let f = fronts(&first.state);
        (
            f.iter().map(|x| x.0.clone()).collect(),
            f.iter().map(|x| x.1).collect(),
            f.iter().map(|x| x.2).collect(),
        )
    } else {
        let mut lanes = vec![Bits::EMPTY; 3];
        let mut middle = vec![Bits::EMPTY; 3];
        for c in valid.iter() {
            lanes[lane(c)].set(c);
            if ring[c] <= 1 {
                middle[lane(c)].set(c);
            }
        }
        (LANES.iter().map(|s| s.to_string()).collect(), lanes, middle)
    };
    let k = lanes.len();
    let mut front: Vec<Vec<Option<u32>>> = vec![vec![None; k]; 2];
    let mut by_turn: Vec<(u32, State)> = Vec::new();
    for step in &game.steps {
        let s = &step.state;
        let t = s.game_turn();
        if t > turns {
            break;
        }
        for (side, row) in front.iter_mut().enumerate() {
            for (l, first) in row.iter_mut().enumerate() {
                if first.is_none() && !(s.active(side) & middle[l]).is_empty() {
                    *first = Some(t);
                }
            }
        }
        if s.to_act() == 0 {
            by_turn.push((t, s.clone()));
        }
    }
    for (t, s) in &by_turn {
        let d = [distances(s, s.active(0)), distances(s, s.active(1))];
        for side in 0..2 {
            for l in 0..k {
                let held = (s.active(side) & lanes[l]).count();
                let nearer = lanes[l]
                    .iter()
                    .filter(|&c| d[side][c] != u8::MAX && d[side][c] < d[1 - side][c])
                    .count();
                let _ = writeln!(
                    out,
                    "{prefix},{t},{side},{},{},{},{held},{nearer},{}",
                    names[side],
                    lane_names[l],
                    lanes[l].count(),
                    front[side][l].map_or(String::new(), |f| f.to_string())
                );
            }
        }
    }
}

/// The board of `state` from red's side, as the plan bot's tests write it:
/// `#` void, `.` neutral, `O`/`o` red's active and inactive cells, `X`/`x`
/// blue's; then the units (seat, level, x, y) and buildings (seat, kind, x,
/// y), as JSON.
fn board_json(state: &State) -> String {
    let map = state.map();
    let mut rows = Vec::new();
    for y in 0..SIZE {
        let mut row = String::new();
        for x in 0..SIZE {
            let c = y * SIZE + x;
            row.push(if !map.valid.test(c) {
                '#'
            } else if state.active(0).test(c) {
                'O'
            } else if state.owned(0).test(c) {
                'o'
            } else if state.active(1).test(c) {
                'X'
            } else if state.owned(1).test(c) {
                'x'
            } else {
                '.'
            });
        }
        rows.push(format!("\"{row}\""));
    }
    let mut units = Vec::new();
    for seat in 0..2 {
        for c in state.units(seat).iter() {
            let level = state.unit_level(c).unwrap_or(1);
            units.push(format!("[{seat},{level},{},{}]", c % SIZE, c / SIZE));
        }
    }
    let mut buildings = Vec::new();
    for seat in 0..2 {
        for c in (state.towers() & state.owned(seat)).iter() {
            buildings.push(format!("[{seat},\"tower\",{},{}]", c % SIZE, c / SIZE));
        }
        for c in (state.mines() & state.owned(seat)).iter() {
            buildings.push(format!("[{seat},\"mine\",{},{}]", c % SIZE, c / SIZE));
        }
    }
    format!(
        "{{\"turn\":{},\"rows\":[{}],\"units\":[{}],\"buildings\":[{}]}}",
        state.game_turn(),
        rows.join(","),
        units.join(","),
        buildings.join(",")
    )
}

/// Runs `codingame-sectors --boards`: for each game of `ids`, a JSON line
/// with its players and its boards at the start of red's ply of each game
/// turn of `at`.
pub fn boards(request: &Request, ids: &[u64], at: &[u32]) -> Result<String, String> {
    let (entries, _) = read_index(&request.data.join("index.jsonl"), request)?;
    let mut out = String::new();
    for entry in entries.iter().filter(|e| ids.contains(&e.meta.game_id)) {
        let Ok((meta, record)) = prepare(&request.data, entry, &request.me) else {
            continue;
        };
        let names = [
            record.players[0].name.clone(),
            record.players[1].name.clone(),
        ];
        let winner = record.winner.map_or(-1, |w| w as i64);
        let game = replay(&meta.game_id.to_string(), record);
        let mut shown = Vec::new();
        for step in &game.steps {
            let s = &step.state;
            if s.to_act() == 0 && at.contains(&s.game_turn()) {
                shown.push(board_json(s));
            }
        }
        let _ = writeln!(
            out,
            "{{\"game_id\":{},\"red\":\"{}\",\"blue\":\"{}\",\"winner\":{winner},\"boards\":[{}]}}",
            meta.game_id,
            names[0],
            names[1],
            shown.join(",")
        );
    }
    Ok(out)
}

/// Runs `codingame-sectors`: the CSV of every game of `request`, by lanes
/// or, with `by_fronts`, by the map's [`fronts`].
pub fn run(request: &Request, turns: u32, by_fronts: bool) -> Result<String, String> {
    let (entries, _) = read_index(&request.data.join("index.jsonl"), request)?;
    let release_of = releases(&request.data)?;
    let jobs = request.jobs.clamp(1, 64);
    let size = entries.len().div_ceil(jobs).max(1);
    let mut parts: Vec<String> = vec![String::new(); entries.len().div_ceil(size)];
    std::thread::scope(|scope| {
        for (chunk, out) in entries.chunks(size).zip(parts.iter_mut()) {
            let release_of = &release_of;
            scope.spawn(move || {
                for entry in chunk {
                    let Ok((meta, record)) = prepare(&request.data, entry, &request.me) else {
                        continue;
                    };
                    let Some(us) = meta.our_seat else { continue };
                    let names = [
                        record.players[0].name.clone(),
                        record.players[1].name.clone(),
                    ];
                    let winner = record.winner.map_or(String::new(), |w| w.to_string());
                    let release = release_of.get(&meta.game_id).cloned().unwrap_or_default();
                    let prefix = format!("{},{},{},{winner}", meta.game_id, meta.kind, release);
                    let _ = us;
                    let game = replay(&meta.game_id.to_string(), record);
                    rows(
                        &game,
                        turns,
                        &prefix,
                        [&names[0], &names[1]],
                        by_fronts,
                        out,
                    );
                }
            });
        }
    });
    let mut text = String::from(
        "game_id,kind,release,winner,turn,seat,player,lane,lane_cells,held,nearer,middle_first_turn\n",
    );
    for part in parts {
        text.push_str(&part);
    }
    Ok(text)
}
