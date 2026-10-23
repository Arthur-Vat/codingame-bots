//! A record of A Code of Ice and Fire replayed into a [`Trace`]: the board
//! and both sides' metrics at the start of every player's turn, the phases
//! and the decisive moments.

use cg_arena::record::Record;
use cg_arena::runner::EndReason;
use cg_review::detect::{large_drops, lead_changes};
use cg_review::trace::{Metric, Moment, Ply, Trace, SIDES};
use ice_and_fire_engine::{Command, League, State, HQ, SIZE};
use ice_and_fire_opening::{middle, score};
use ice_and_fire_referee::map::generate;
use ice_and_fire_tactics::chain::chain_to;
use ice_and_fire_tactics::cut::cuts;

/// Upkeep of a unit by level (RULES.md, "Income").
const UPKEEP: [i64; 3] = [1, 4, 20];
/// `hq_gap` when no chain of trainings reaches the enemy HQ.
pub const NO_CHAIN: i64 = 999;
/// A large cut: a side loses at least this share of its active cells, in
/// percent, and at least [`CUT_MIN`] of them, in one ply.
pub const CUT_SHARE: i64 = 30;
pub const CUT_MIN: i64 = 6;
/// A lead change: the side ahead in `material` by at least this much
/// changes. A swing: the lead moves by at least [`SWING`] in one turn.
pub const LEAD_MARGIN: i64 = 20;
pub const SWING: i64 = 60;

/// What the reviewer must know to read the game.
const PRIMER: &[&str] = &[
    "A Code of Ice and Fire, Bronze rules: a 12 x 12 map; red's HQ is at (0, 0), blue's at (11, 11). Red plays first in each turn. Taking the enemy HQ wins; after 100 turns the side with more gold plus training cost of its units wins.",
    "Each turn a player may train units (level 1, 2 or 3 for 10, 20 or 30 gold) on its own cells or next to them, move each unit one cell, and build mines (15 gold, on mine spots, +4 income) and towers (15 gold). Upkeep each turn: 1, 4 or 20 by level.",
    "A unit beats a lower level only (level 3 beats all); a cell next to an enemy tower, or holding one, can only be entered by level 3. Entering an enemy cell takes it.",
    "A cell is active when a path of its owner's cells joins it to the owner's HQ. Inactive (cut off) cells earn nothing and the units on them die at their owner's next turn: cutting a corridor removes everything beyond it at once. A side whose gold would go below 0 at its turn loses all its units (debt).",
    "Income each turn: active cells, plus 4 for each active mine, less upkeep.",
    "Board: two characters a cell. The first is the owner: R red, B blue, upper case for an active cell, lower case (r, b) for a cut-off one, a space for nobody. The second is the content: . empty, 1 2 3 a unit of that level, M mine, T tower, H HQ, * an empty mine spot. ## is void. Columns are x (0 to 11), rows are y (0 to 11), as in the commands.",
    "Commands: TRAIN level x y; MOVE unit-id x y (one step towards x y); BUILD MINE x y; BUILD TOWER x y; WAIT.",
];

/// The metrics, in the order of [`side_metrics`].
pub fn metrics() -> Vec<Metric> {
    [
        ("gold", "Gold in hand at this point."),
        ("income", "Gold earned each turn: active cells, plus 4 a mine on an active cell, less upkeep."),
        ("upkeep", "Upkeep of the units on active cells: 1, 4 and 20 for levels 1, 2 and 3."),
        ("cells", "Active cells: owned and joined to the HQ."),
        ("cut_off", "Owned cells cut off from the HQ: they earn nothing, and units on them die."),
        ("units", "Units on active cells."),
        ("army", "Training cost of those units: 10, 20 or 30 by level."),
        ("mines", "Mines on active cells."),
        ("towers", "Towers on active cells."),
        ("voronoi", "Non-void cells nearer this side's active cells than the enemy's (a tie counts half, rounded down): the land it is likely to get."),
        ("exposed30", "Active cells the enemy could take or cut off with 30 gold of trainings (its best cut of up to 5 trainings)."),
        ("exposed50", "The same with 50 gold."),
        ("can_cut", "Enemy active cells this side could take or cut off now with the gold it has."),
        ("hq_gap", "Gold this side still lacks to pay its cheapest chain of trainings to the enemy HQ: 0 when it can pay now, 999 when no chain reaches it."),
        ("material", "Gold + 3 x income + 4 x cells + army + 10 x towers: a side's holdings as the bots' shared evaluation counts them. The lead is its difference."),
    ]
    .iter()
    .map(|(key, meaning)| Metric {
        key: (*key).to_string(),
        meaning: (*meaning).to_string(),
    })
    .collect()
}

/// One command of an answer, as the referee reads it; anything else is
/// skipped (the rules drop it or the game would have ended).
pub fn parse(part: &str) -> Option<Command> {
    let words: Vec<&str> = part.split_whitespace().collect();
    let num = |i: usize| words.get(i).and_then(|w| w.parse::<u32>().ok());
    let small = |i: usize| num(i).and_then(|n| u8::try_from(n).ok());
    match words.first().copied()? {
        "MOVE" => Some(Command::Move {
            id: num(1)?,
            x: small(2)?,
            y: small(3)?,
        }),
        "TRAIN" => Some(Command::Train {
            level: small(1)?,
            x: small(2)?,
            y: small(3)?,
        }),
        "BUILD" => {
            let (x, y) = (small(2)?, small(3)?);
            match words.get(1).copied()? {
                "MINE" => Some(Command::BuildMine { x, y }),
                "TOWER" => Some(Command::BuildTower { x, y }),
                _ => None,
            }
        }
        _ => None,
    }
}

/// The board: a header of columns, then one line a row (see the primer).
pub fn board(state: &State) -> Vec<String> {
    let mut lines = vec![format!(
        "   {}",
        (0..SIZE)
            .map(|x| format!("{x:>2}"))
            .collect::<Vec<_>>()
            .join(" ")
    )];
    for y in 0..SIZE {
        let cells: Vec<String> = (0..SIZE).map(|x| cell_text(state, y * SIZE + x)).collect();
        lines.push(format!("{y:>2} {}", cells.join(" ")));
    }
    lines
}

/// One cell's two characters.
fn cell_text(state: &State, cell: usize) -> String {
    if !state.map().valid.test(cell) {
        return "##".to_string();
    }
    let owner = (0..2).find(|&s| state.owned(s).test(cell));
    let first = match owner {
        Some(s) => {
            let letter = ['R', 'B'][s];
            if state.active(s).test(cell) {
                letter
            } else {
                letter.to_ascii_lowercase()
            }
        }
        None => ' ',
    };
    let second = if HQ.contains(&cell) {
        'H'
    } else if let Some(level) = state.unit_level(cell) {
        char::from(b'0' + level)
    } else if state.mines().test(cell) {
        'M'
    } else if state.towers().test(cell) {
        'T'
    } else if state.map().spots.test(cell) {
        '*'
    } else {
        '.'
    };
    format!("{first}{second}")
}

/// The state as `seat` would see it at its turn, with `gold`: `seat` to
/// play, all its units free to move. `None` if the text does not read back.
fn view(state: &State, seat: usize, gold: i64) -> Option<State> {
    let text = state.turn_input(seat);
    let rest = text.split_once('\n').map_or("", |(_, r)| r);
    State::read(
        &state.initial_input(seat),
        &format!("{gold}\n{rest}"),
        League::Bronze,
        state.game_turn(),
    )
    .ok()
}

/// The enemy active cells `seat` could take or cut off with `budget` gold.
fn cut_cells(state: &State, seat: usize, budget: i64) -> i64 {
    let Some(view) = view(state, seat, budget.max(0)) else {
        return 0;
    };
    cuts(&view, 5, budget.max(0), 8)
        .iter()
        .map(|c| c.cells)
        .max()
        .unwrap_or(0)
}

/// Gold enough for any chain: the chain finder plays its trainings, which
/// must be paid for.
const AMPLE: i64 = 10_000;

/// The cost of `seat`'s cheapest chain of trainings to the enemy HQ.
fn hq_cost(state: &State, seat: usize) -> Option<i64> {
    let view = view(state, seat, AMPLE)?;
    chain_to(&view, HQ[1 - seat], AMPLE).map(|c| c.cost)
}

/// `seat`'s metrics, in the order of [`metrics`].
pub fn side_metrics(state: &State, seat: usize) -> Vec<i64> {
    let active = state.active(seat);
    let units = state.units(seat) & active;
    let (mut upkeep, mut army) = (0, 0);
    for cell in units.iter() {
        if let Some(level) = state.unit_level(cell) {
            upkeep += UPKEEP[usize::from(level) - 1];
            army += 10 * i64::from(level);
        }
    }
    let cells = i64::from(active.count());
    let towers = i64::from((state.towers() & active).count());
    let gold = state.gold(seat);
    let income = state.income(seat);
    let gap = hq_cost(state, seat).map_or(NO_CHAIN, |cost| (cost - gold).max(0));
    vec![
        gold,
        income,
        upkeep,
        cells,
        i64::from(state.owned(seat).and_not(active).count()),
        i64::from(units.count()),
        army,
        i64::from((state.mines() & active).count()),
        towers,
        score::voronoi_halves(state, seat) / 2,
        cut_cells(state, 1 - seat, 30),
        cut_cells(state, 1 - seat, 50),
        cut_cells(state, seat, gold),
        gap,
        gold + 3 * income + 4 * cells + army + 10 * towers,
    ]
}

/// How the game ended, in a few words.
fn end_text(record: &Record) -> String {
    match &record.end {
        EndReason::Finished => "the rules ended the game".to_string(),
        EndReason::Timeout { seat, .. } => format!("{} timed out", SIDES[*seat]),
        EndReason::Crash { seat, .. } => format!("{} crashed", SIDES[*seat]),
        EndReason::Invalid { seat, .. } => format!("{} sent an invalid answer", SIDES[*seat]),
        EndReason::Aborted { .. } => "the arena stopped the game".to_string(),
        EndReason::Resigned { seat } => format!("{} resigned", SIDES[*seat]),
    }
}

/// Replays `record` (a Bronze game) into a trace named `name`.
pub fn trace(name: &str, record: &Record) -> Trace {
    let map = generate(record.seed, ice_and_fire_referee::League::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let keys = metrics();
    let mut plies = Vec::new();
    let mut moments = Vec::new();
    let mut middle_ply = None;
    let mut threats = [false; 2];
    for answers in &record.turns {
        if state.outcome().is_some() {
            break;
        }
        let seat = state.to_act();
        let other = 1 - seat;
        let ply = plies.len();
        if middle_ply.is_none() && middle::reached(&state) {
            middle_ply = Some(ply);
        }
        let line = answers
            .iter()
            .find(|a| a.seat == seat)
            .and_then(|a| a.lines.first())
            .cloned()
            .unwrap_or_default();
        let commands: Vec<Command> = line.split(';').filter_map(|p| parse(p.trim())).collect();
        let snapshot = Ply {
            game_turn: state.game_turn(),
            to_act: seat,
            board: board(&state),
            metrics: [side_metrics(&state, 0), side_metrics(&state, 1)],
            commands: line.trim().to_string(),
            spent: 0,
        };
        if !threats[seat] && snapshot.metrics[seat][13] == 0 {
            threats[seat] = true;
            moments.push(Moment {
                ply,
                kind: "HQ threat".to_string(),
                favours: Some(seat),
                text: format!(
                    "{} can pay a chain of trainings to {}'s HQ this turn, for the first time",
                    SIDES[seat], SIDES[other]
                ),
            });
        }
        let gold = state.gold(seat);
        let (other_gold, other_units) = (state.gold(other), state.unit_count(other));
        state.play_turn(&commands);
        let mut snapshot = snapshot;
        snapshot.spent = (gold - state.gold(seat)).max(0);
        plies.push(snapshot);
        // The other side's start of turn: a debt wipes its army.
        if state.outcome().is_none() && other_units > 0 && other_gold + state.income(other) < 0 {
            moments.push(Moment {
                ply,
                kind: "debt".to_string(),
                favours: Some(seat),
                text: format!(
                    "{} could not pay its upkeep at its next turn and lost all {other_units} of its units",
                    SIDES[other]
                ),
            });
        }
    }
    let mut trace = Trace {
        name: name.to_string(),
        players: [
            record.players[0].name.clone(),
            record.players[1].name.clone(),
        ],
        winner: record.winner,
        end: end_text(record),
        primer: PRIMER.iter().map(|s| (*s).to_string()).collect(),
        metrics: keys,
        lead: "material".to_string(),
        plies,
        middle: middle_ply,
        endgame: None,
        moments,
    };
    trace.endgame = endgame(&trace);
    trace.moments.extend(large_drops(
        &trace,
        "cells",
        CUT_SHARE,
        CUT_MIN,
        "large cut",
    ));
    trace
        .moments
        .extend(lead_changes(&trace, "material", LEAD_MARGIN, SWING));
    trace.moments.sort_by_key(|m| m.ply);
    trace
}

/// The first ply, from the middle game on, at which one side has twice the
/// other's active cells or can pay a chain to the enemy HQ with its gold
/// plus one turn of income: the winner is converting.
fn endgame(trace: &Trace) -> Option<usize> {
    let from = trace.middle?;
    (from..trace.plies.len()).find(|&ply| {
        (0..2).any(|side| {
            let get = |s: usize, key: &str| trace.value(ply, s, key).unwrap_or(0);
            let twice = get(side, "cells") >= 2 * get(1 - side, "cells");
            let gap = get(side, "hq_gap");
            twice || (gap != NO_CHAIN && gap <= get(side, "income").max(0))
        })
    })
}

#[cfg(test)]
mod tests;
