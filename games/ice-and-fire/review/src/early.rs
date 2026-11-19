//! The early game's territory measures of each side of the CodinGame games
//! (`codingame-early`), as CSV: one row a side and game turn, at the start of
//! that side's ply, for the first turns of the game. Asked by the owner on
//! 2026-11-19: can zasmu's early game be modelled with the number of active
//! cells, the centre and the chokepoints, and the Voronoi share alone?
//!
//! The measures, for the side `s` to act:
//!
//! - `active`: its active cells;
//! - `centre`: its owned cells within one step of even between the two HQs;
//! - `to_centre`: the fewest steps from its owned cells to such a cell (0
//!   when it holds one);
//! - `chokes`: its share of the map's choke cells, the cells that at least
//!   [`CHOKE_SHARE`] of the shortest paths between the two HQs go through;
//! - `voronoi`: its share of the cells nearer its land than the enemy's, over
//!   the cells nearer one side.
//!
//! The row also names the player, its group (its release for our games), the
//! opponent and whether the side won the game.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use ice_and_fire_engine::{Bits, Outcome, State, CELLS, HQ};
use serde_json::Value;

use crate::profiles::{prepare, read_index, Request};
use crate::replay::replay;

/// The share of the shortest HQ-to-HQ paths a cell must carry to be a choke
/// cell (as in the zasmu study: 40%).
pub const CHOKE_SHARE: f64 = 0.4;

/// Steps from `sources` to each valid cell, `u8::MAX` where none reaches.
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

/// The number of shortest paths from `from` to each cell (as `f64`, which
/// does not overflow), and the steps.
fn paths_from(state: &State, from: usize) -> ([f64; CELLS], [u8; CELLS]) {
    let dist = steps_from(state, Bits::cell(from));
    let mut order: Vec<usize> = state
        .map()
        .valid
        .iter()
        .filter(|&c| dist[c] != u8::MAX)
        .collect();
    order.sort_by_key(|&c| dist[c]);
    let mut count = [0.0; CELLS];
    count[from] = 1.0;
    for cell in order {
        if cell == from {
            continue;
        }
        let around = Bits::cell(cell).neighbours() & state.map().valid;
        count[cell] = around
            .iter()
            .filter(|&n| dist[n] != u8::MAX && dist[n] + 1 == dist[cell])
            .map(|n| count[n])
            .sum();
    }
    (count, dist)
}

/// The map's choke cells (see [`CHOKE_SHARE`]) and its centre cells.
pub fn map_zones(state: &State) -> (Bits, Bits) {
    let (a, da) = paths_from(state, HQ[0]);
    let (b, db) = paths_from(state, HQ[1]);
    let total = a[HQ[1]];
    let length = da[HQ[1]];
    let mut chokes = Bits::EMPTY;
    let mut centre = Bits::EMPTY;
    for cell in state.map().valid.iter() {
        if da[cell] == u8::MAX || db[cell] == u8::MAX {
            continue;
        }
        if da[cell].abs_diff(db[cell]) <= 1 {
            centre.set(cell);
        }
        let on_path = u16::from(da[cell]) + u16::from(db[cell]) == u16::from(length);
        if total > 0.0
            && on_path
            && cell != HQ[0]
            && cell != HQ[1]
            && a[cell] * b[cell] / total >= CHOKE_SHARE
        {
            chokes.set(cell);
        }
    }
    (chokes, centre)
}

/// The share of the cells nearer `seat`'s land than the enemy's.
pub fn voronoi(state: &State, seat: usize) -> f64 {
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

/// The measures of `seat` in `state`: active, centre, to_centre, chokes,
/// voronoi.
pub fn measures(
    state: &State,
    seat: usize,
    chokes: Bits,
    centre: Bits,
) -> (u32, u32, u8, f64, f64) {
    let owned = state.owned(seat);
    let active = state.active(seat).count();
    let held = (owned & centre).count();
    let to_centre = {
        let d = steps_from(state, owned);
        centre.iter().map(|c| d[c]).min().unwrap_or(u8::MAX)
    };
    let choke_share = if chokes.is_empty() {
        0.0
    } else {
        f64::from((owned & chokes).count()) / f64::from(chokes.count())
    };
    (active, held, to_centre, choke_share, voronoi(state, seat))
}

fn releases(path: &Path) -> Result<HashMap<u64, String>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut out = HashMap::new();
    for line in text.lines() {
        let Ok(value) = serde_json::from_str::<Value>(line) else {
            continue;
        };
        if let (Some(id), Some(release)) = (value["game_id"].as_u64(), value["release"].as_str()) {
            out.insert(id, release.to_string());
        }
    }
    Ok(out)
}

/// The CSV of `codingame-early`: one row a side and game turn up to
/// `turns`.
pub fn run(request: &Request, turns: u32) -> Result<String, String> {
    let index = request.data.join("index.jsonl");
    let (entries, _) = read_index(&index, request)?;
    let release_of = releases(&index)?;
    let mut out = String::from(
        "game_id,kind,seat,player,group,opponent,won,turn,active,centre,to_centre,chokes,voronoi,map_chokes\n",
    );
    for entry in &entries {
        let Ok((meta, record)) = prepare(&request.data, entry, &request.me) else {
            continue;
        };
        let names = [
            record.players[0].name.clone(),
            record.players[1].name.clone(),
        ];
        let winner = record.winner;
        let game = replay(&meta.game_id.to_string(), record);
        let winner = match game.end.outcome() {
            Some(Outcome::Win(w)) => Some(w),
            Some(Outcome::Draw) => None,
            None => winner,
        };
        let Some(first) = game.steps.first() else {
            continue;
        };
        let (chokes, centre) = map_zones(&first.state);
        for step in &game.steps {
            let state = &step.state;
            if state.game_turn() > turns {
                break;
            }
            let seat = state.to_act();
            let group = if meta.our_seat == Some(seat) {
                release_of
                    .get(&meta.game_id)
                    .cloned()
                    .unwrap_or_else(|| "us".to_string())
            } else {
                names[seat].clone()
            };
            let (active, held, to_centre, choke, vor) = measures(state, seat, chokes, centre);
            let won = match winner {
                Some(w) if w == seat => "1",
                Some(_) => "0",
                None => "",
            };
            let _ = writeln!(
                out,
                "{},{},{seat},{},{group},{},{won},{},{active},{held},{to_centre},{choke:.3},{vor:.3},{}",
                meta.game_id,
                meta.kind,
                names[seat],
                names[1 - seat],
                state.game_turn(),
                chokes.count()
            );
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests;
