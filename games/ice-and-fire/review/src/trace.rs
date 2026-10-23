//! A record of A Code of Ice and Fire replayed into a [`Trace`]: the board
//! and both sides' metrics at the start of every player's turn, the phases
//! and the decisive moments.

use cg_arena::record::Record;
use cg_arena::runner::EndReason;
use cg_review::detect::{large_drops, lead_changes};
use cg_review::trace::{Metric, Moment, Ply, Trace, SIDES};
use ice_and_fire_engine::{Bits, Command, League, State, HQ, SIZE};
use ice_and_fire_opening::{middle, score, solo};
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
    "A unit beats a lower level only (level 3 beats all). Only level 3 may enter a cell holding an enemy tower, or an enemy active cell next to an active enemy tower; a neutral cell next to a tower is not protected. Entering an enemy cell takes it.",
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
        ("exposed30", "Active cells the enemy could take or cut off with 30 gold of trainings: the most among its 8 best cuts of up to 5 trainings."),
        ("exposed50", "The same with 50 gold."),
        ("can_cut", "Enemy active cells this side could take or cut off with the gold it has, in the same way."),
        ("hq_gap", "Gold this side still lacks to pay its cheapest chain of trainings to the enemy HQ: 0 when it can pay now, 999 when no chain reaches it."),
        ("material", "Gold + 3 x income + 4 x cells + army + 10 x towers: a side's holdings as the bots' shared evaluation counts them. The lead is its difference."),
        ("gross_income", "Income before upkeep: active cells plus 4 a mine on an active cell."),
        ("free_spots", "Mine spots on active cells without a mine: mines this side could still build at home."),
        ("level1", "Level 1 units on active cells."),
        ("level2", "Level 2 units on active cells."),
        ("level3", "Level 3 units on active cells."),
        ("joints", "Active cells whose loss would cut other active cells off from the HQ."),
        ("max_cut", "The most active cells that the loss of a single cell would cut off."),
        ("total_cut", "The sum over the joints of the cells each would cut off: how much of the territory hangs on single cells."),
        ("line_ends", "Active cells other than the HQ with at most one own active neighbour: the tips of thin lines."),
        ("reach", "Steps from the nearest active cell to the enemy HQ."),
        ("frontier", "Active cells next to an enemy cell."),
        ("border", "Active cells next to a cell this side does not own (void aside): the territory's perimeter."),
        ("room", "Neutral cells next to this side's active cells: where it can still grow without a fight."),
        ("far_cells", "Active cells nearer the enemy HQ than this side's own (Manhattan distance)."),
        ("spread", "The mean Manhattan distance of the active cells from this side's HQ, times 10."),
        ("frontier_joints", "Joints next to an enemy cell: the links the enemy can reach first."),
        ("empty_joints", "Joints that hold no unit, building or HQ of this side."),
        ("guarded_joints", "Joints that hold a unit of this side."),
        ("frontier_units", "Units on frontier cells."),
        ("inner_units", "Units with no enemy cell within 2 steps: kept away from any fight."),
        ("units_lost", "Units of this side lost since the end of its last turn: taken by the enemy, cut off, or lost to debt."),
        ("contact", "Enemy units next to this side's active cells."),
        ("hq_guard", "Units of this side within 2 steps of its HQ."),
        ("frontier_towers", "Towers on frontier cells."),
        ("tower_cover", "Active cells a tower of this side protects (the tower's cell and its neighbours)."),
        ("spent_units", "Gold this side spent training units in its last turn."),
        ("spent_buildings", "Gold this side spent building mines and towers in its last turn."),
        ("kept_share", "The share of its gold this side kept unspent in its last turn, in percent."),
        ("moves", "Moves this side made in its last turn."),
        ("trains", "Units this side trained in its last turn."),
        ("builds", "Mines and towers this side built in its last turn."),
        ("captured", "Enemy cells this side took in its last turn."),
        ("lost", "Cells of this side the enemy took in its last turn."),
        ("cells_change", "The change of this side's active cells since the start of its previous turn."),
        ("material_change", "The change of this side's material since the start of its previous turn."),
    ]
    .iter()
    .map(|(key, meaning)| Metric {
        key: (*key).to_string(),
        meaning: (*meaning).to_string(),
        none: (*key == "hq_gap").then_some(NO_CHAIN),
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

/// What a side did in its last turn, and what it had at the start of the
/// turn before: the metrics that are not seen in one position.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Flow {
    pub spent_units: i64,
    pub spent_buildings: i64,
    pub kept_share: i64,
    pub moves: i64,
    pub trains: i64,
    pub builds: i64,
    pub captured: i64,
    /// Cells of this side the enemy took in the enemy's last turn.
    pub lost: i64,
    /// Units of this side lost since the end of its last turn.
    pub units_lost: i64,
    /// Active cells and material at the start of this side's previous
    /// turn, if it had one.
    pub before: Option<(i64, i64)>,
}

/// Manhattan distance between two cells.
fn manhattan(a: usize, b: usize) -> i64 {
    let (ax, ay) = ((a % SIZE) as i64, (a / SIZE) as i64);
    let (bx, by) = ((b % SIZE) as i64, (b / SIZE) as i64);
    (ax - bx).abs() + (ay - by).abs()
}

/// `seat`'s metrics, in the order of [`metrics`], with `flow`, what it did
/// in its last turn.
pub fn side_metrics(state: &State, seat: usize, flow: &Flow) -> Vec<i64> {
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
    let mines = i64::from((state.mines() & active).count());
    let towers = i64::from((state.towers() & active).count());
    let gold = state.gold(seat);
    // From its definition: after a debt the engine keeps the income it
    // showed, which no longer counts the lost units' upkeep.
    let income = cells + 4 * mines - upkeep;
    let gap = hq_cost(state, seat).map_or(NO_CHAIN, |cost| (cost - gold).max(0));
    let material = gold + 3 * income + 4 * cells + army + 10 * towers;
    let mut out = vec![
        gold,
        income,
        upkeep,
        cells,
        i64::from(state.owned(seat).and_not(active).count()),
        i64::from(units.count()),
        army,
        mines,
        towers,
        score::voronoi_halves(state, seat) / 2,
        cut_cells(state, 1 - seat, 30),
        cut_cells(state, 1 - seat, 50),
        cut_cells(state, seat, gold),
        gap,
        material,
    ];
    out.extend(shape_metrics(state, seat, material, flow));
    out
}

/// The metrics after `material` in [`metrics`]: the army by level, the
/// territory's shape, where the units stand, and `flow`.
fn shape_metrics(state: &State, seat: usize, material: i64, flow: &Flow) -> Vec<i64> {
    let other = 1 - seat;
    let valid = state.map().valid;
    let active = state.active(seat);
    let owned = state.owned(seat);
    let enemy = state.owned(other);
    let units = state.units(seat) & active;
    let near = |cell: usize| Bits::cell(cell).neighbours();
    let count = |bits: Bits| i64::from(bits.count());
    let mut levels = [0; 3];
    for cell in units.iter() {
        if let Some(level) = state.unit_level(cell) {
            levels[usize::from(level) - 1] += 1;
        }
    }
    let shape = solo::shape(state, seat);
    let cut = solo::severed(state, seat);
    let joints: Bits = active
        .iter()
        .filter(|&c| cut[c] > 0)
        .fold(Bits::EMPTY, |b, c| b | Bits::cell(c));
    let total_cut: i64 = joints.iter().map(|c| cut[c]).sum();
    let frontier: Bits = active
        .iter()
        .filter(|&c| !(near(c) & enemy).is_empty())
        .fold(Bits::EMPTY, |b, c| b | Bits::cell(c));
    let border = active
        .iter()
        .filter(|&c| !(near(c) & valid).and_not(owned).is_empty())
        .count() as i64;
    let neutral = valid.and_not(owned | enemy);
    let room = count(active.neighbours() & neutral);
    let far = active
        .iter()
        .filter(|&c| manhattan(c, HQ[other]) < manhattan(c, HQ[seat]))
        .count() as i64;
    let spread = if active.is_empty() {
        0
    } else {
        active.iter().map(|c| manhattan(c, HQ[seat])).sum::<i64>() * 10 / count(active)
    };
    let held = state.units(seat) | state.mines() | state.towers() | Bits::cell(HQ[seat]);
    let enemy_near2 = enemy.neighbours().neighbours() | enemy.neighbours() | enemy;
    let towers = state.towers() & active;
    let cover = towers
        .iter()
        .fold(Bits::EMPTY, |b, t| b | Bits::cell(t) | near(t))
        & active;
    let hq_area = near(HQ[seat]).neighbours() | near(HQ[seat]);
    let (cells_change, material_change) = flow
        .before
        .map_or((0, 0), |(c, m)| (count(active) - c, material - m));
    vec![
        count(active) + 4 * count(state.mines() & active),
        count((state.map().spots & active).and_not(state.mines())),
        levels[0],
        levels[1],
        levels[2],
        count(joints),
        shape.cut,
        total_cut,
        shape.tips,
        if shape.reach == i64::MAX {
            0
        } else {
            shape.reach
        },
        count(frontier),
        border,
        room,
        far,
        spread,
        count(joints & enemy.neighbours()),
        count(joints.and_not(held)),
        count(joints & state.units(seat)),
        count(units & frontier),
        count(units.and_not(enemy_near2)),
        flow.units_lost,
        count(state.units(other) & active.neighbours()),
        count(state.units(seat) & hq_area),
        count(towers & frontier),
        count(cover),
        flow.spent_units,
        flow.spent_buildings,
        flow.kept_share,
        flow.moves,
        flow.trains,
        flow.builds,
        flow.captured,
        flow.lost,
        cells_change,
        material_change,
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

/// What the player to act in `state` does with `commands`: the gold it
/// spends on units and on buildings, and its moves, trainings and builds
/// that the rules accept, applied one by one on a copy.
fn turn_flow(state: &State, commands: &[Command]) -> Flow {
    let mut sim = state.clone();
    let seat = state.to_act();
    let mut flow = Flow::default();
    for &command in commands {
        let gold = sim.gold(seat);
        if !sim.apply(command) {
            continue;
        }
        let cost = gold - sim.gold(seat);
        match command {
            Command::Train { .. } => {
                flow.spent_units += cost;
                flow.trains += 1;
            }
            Command::BuildMine { .. } | Command::BuildTower { .. } => {
                flow.spent_buildings += cost;
                flow.builds += 1;
            }
            Command::Move { .. } => flow.moves += 1,
            Command::Wait => {}
        }
    }
    flow
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
    // Before a side's first turn it has kept all its gold.
    let mut flows = [Flow {
        kept_share: 100,
        ..Flow::default()
    }; 2];
    // Each side's units at the end of its last turn.
    let mut units_after: [Option<i64>; 2] = [None; 2];
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
        for (side, after) in units_after.iter().enumerate() {
            if let Some(after) = after {
                flows[side].units_lost = (after - state.unit_count(side) as i64).max(0);
            }
        }
        let snapshot = Ply {
            game_turn: state.game_turn(),
            to_act: seat,
            board: board(&state),
            metrics: [
                side_metrics(&state, 0, &flows[0]),
                side_metrics(&state, 1, &flows[1]),
            ],
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
        let start = (snapshot.metrics[seat][3], snapshot.metrics[seat][14]);
        let mut flow = turn_flow(&state, &commands);
        let enemy_before = state.owned(other);
        state.play_turn(&commands);
        flow.captured = i64::from((state.owned(seat) & enemy_before).count());
        flow.kept_share = if gold > 0 {
            state.gold(seat).max(0) * 100 / gold
        } else {
            100
        };
        flow.lost = flows[seat].lost;
        flow.units_lost = flows[seat].units_lost;
        units_after[seat] = Some(state.unit_count(seat) as i64);
        flow.before = Some(start);
        flows[other].lost = flow.captured;
        flows[seat] = flow;
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
                    "{} could not pay its upkeep at its next turn and lost all its units",
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
