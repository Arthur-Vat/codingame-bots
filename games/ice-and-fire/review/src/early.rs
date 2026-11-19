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
//!   [`CHOKE_SHARE`](ice_and_fire_early::zones::CHOKE_SHARE) of the shortest
//!   paths between the two HQs go through;
//! - `voronoi`: its share of the cells nearer its land than the enemy's, over
//!   the cells nearer one side;
//! - `mid_chokes`: its share of the choke cells of the middle band
//!   ([`MIDDLE`](ice_and_fire_early::zones::MIDDLE)), the contested ones; 0 too on a map whose band has none,
//!   which `map_mid_chokes` (their number) tells apart.
//!
//! - `units`, `l2`, `l3`: its units, and those of level 2 and 3;
//! - `gold`, `income`: its gold and income as the turn's input shows them;
//! - `owned`, `mines`, `towers`: its cells (active or not), its mines and
//!   its towers;
//! - `map_cells`: the map's valid cells.
//!
//! The row also names the player, its group (its release for our games), the
//! opponent and whether the side won the game.

use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use ice_and_fire_early::zones::{share, to_centre, voronoi, Zones};
use ice_and_fire_engine::{Outcome, State};
use serde_json::Value;

use crate::profiles::{prepare, read_index, Request};
use crate::replay::{map_records, replay, Game};

/// The measures of `seat` in `state`: active, centre, to_centre, chokes,
/// voronoi.
pub fn measures(state: &State, seat: usize, zones: &Zones) -> (u32, u32, u8, f64, f64) {
    (
        state.active(seat).count(),
        (state.owned(seat) & zones.centre).count(),
        to_centre(state, seat, zones),
        share(state, seat, zones.chokes),
        voronoi(state, seat),
    )
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

/// The CSV header of `codingame-early` and `early`.
pub const HEADER: &str = "game_id,kind,seat,player,group,opponent,won,turn,active,centre,to_centre,chokes,voronoi,map_chokes,mid_chokes,map_mid_chokes,units,l2,l3,gold,income,owned,mines,towers,map_cells\n";

/// Who played a game, for its rows.
pub struct Sides<'a> {
    pub game_id: &'a str,
    pub kind: &'a str,
    pub names: [String; 2],
    pub groups: [String; 2],
}

/// Appends the rows of `game` up to game turn `turns`.
fn game_rows(
    out: &mut String,
    sides: &Sides,
    game: &Game,
    recorded_winner: Option<usize>,
    turns: u32,
) {
    let winner = match game.end.outcome() {
        Some(Outcome::Win(w)) => Some(w),
        Some(Outcome::Draw) => None,
        None => recorded_winner,
    };
    let Some(first) = game.steps.first() else {
        return;
    };
    let zones = Zones::of(&first.state);
    let mid = zones.middle_chokes();
    for step in &game.steps {
        let state = &step.state;
        if state.game_turn() > turns {
            break;
        }
        let seat = state.to_act();
        let (active, held, to_centre, choke, vor) = measures(state, seat, &zones);
        let mid_share = share(state, seat, mid);
        let units = state.units(seat);
        let level = |l: u8| {
            units
                .iter()
                .filter(|&c| state.unit_level(c) == Some(l))
                .count()
        };
        let owned = state.owned(seat);
        let won = match winner {
            Some(w) if w == seat => "1",
            Some(_) => "0",
            None => "",
        };
        let _ = writeln!(
            out,
            "{},{},{seat},{},{},{},{won},{},{active},{held},{to_centre},{choke:.3},{vor:.3},{},{mid_share:.3},{},{},{},{},{},{},{},{},{},{}",
            sides.game_id,
            sides.kind,
            sides.names[seat],
            sides.groups[seat],
            sides.names[1 - seat],
            state.game_turn(),
            zones.chokes.count(),
            mid.count(),
            units.count(),
            level(2),
            level(3),
            state.gold(seat),
            state.income(seat),
            owned.count(),
            (owned & state.mines()).count(),
            (owned & state.towers()).count(),
            state.map().valid.count()
        );
    }
}

/// The CSV of `codingame-early`: one row a side and game turn up to
/// `turns`.
pub fn run(request: &Request, turns: u32) -> Result<String, String> {
    let index = request.data.join("index.jsonl");
    let (entries, _) = read_index(&index, request)?;
    let release_of = releases(&index)?;
    let mut out = String::from(HEADER);
    for entry in &entries {
        let Ok((meta, record)) = prepare(&request.data, entry, &request.me) else {
            continue;
        };
        let names = [
            record.players[0].name.clone(),
            record.players[1].name.clone(),
        ];
        let groups = [0, 1].map(|seat| {
            if meta.our_seat == Some(seat) {
                release_of
                    .get(&meta.game_id)
                    .cloned()
                    .unwrap_or_else(|| "us".to_string())
            } else {
                names[seat].clone()
            }
        });
        let winner = record.winner;
        let game_id = meta.game_id.to_string();
        let game = replay(&game_id, record);
        let sides = Sides {
            game_id: &game_id,
            kind: &meta.kind,
            names,
            groups,
        };
        game_rows(&mut out, &sides, &game, winner, turns);
    }
    Ok(out)
}

/// The CSV of `early`: the same rows for arena records, named by the
/// record's file name, of kind `arena`, each side's group its bot's name.
pub fn run_records(paths: &[PathBuf], jobs: usize, turns: u32) -> Result<String, String> {
    let parts = map_records(paths, jobs, |game| {
        let names = [
            game.record.players[0].name.clone(),
            game.record.players[1].name.clone(),
        ];
        let sides = Sides {
            game_id: &game.name,
            kind: "arena",
            groups: names.clone(),
            names,
        };
        let mut out = String::new();
        game_rows(&mut out, &sides, game, game.record.winner, turns);
        out
    })?;
    let mut out = String::from(HEADER);
    for part in parts {
        out.push_str(&part);
    }
    Ok(out)
}

pub mod fit;

#[cfg(test)]
mod tests;
