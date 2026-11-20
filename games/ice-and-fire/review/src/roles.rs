//! The three goals of the early game, measured in the CodinGame games
//! (`codingame-roles`, plan `research/codingame/2026-11-20-frontline-plan.md`,
//! Step 1): objectives and who holds them, frontline units, trainings at the
//! tip of the land, pockets, idle back-line units, and how local the
//! captures are. All measures are per side at the start of its ply, for
//! game turns 1 to `turns`, as Markdown by group and seat.
//!
//! Definitions (the plan's, made exact):
//! - The *objectives* of a map are its middle chokes and its band cells
//!   (valid cells at most 2 rings from the line between the HQs), taken at
//!   game turn 1. The *value* of an objective for a side is the cells
//!   strictly nearer (in steps over valid cells) to the side's active land
//!   plus the objective than to the enemy's active land, less the same for
//!   the enemy with the objective added to its land. Those of value at
//!   least `min_value`, at the start of the ply, are *counted*.
//! - An objective is *held* when it is an active cell of the side.
//! - A unit is a *frontline unit* when it moved in each of the side's last
//!   two plies and each move brought it nearer, by map distance, to the
//!   counted objective not held that is nearest to it now.
//! - A *tip training* is a TRAIN on a cell next to the destination of an
//!   earlier MOVE of the same ply.
//! - A *pocket* is a neutral valid cell that the enemy's active land cannot
//!   reach over valid cells the side does not own.
//! - A unit is *idle* when it did not move in the ply and no valid cell
//!   within 2 steps is missing from the side's land.
//! - *Locality* (game turns 8 on): of the cells the side captured in a ply,
//!   the share within 2 steps of a counted objective (the nearest), the
//!   plies whose captures lie nearest to two objectives more than 2 steps
//!   apart, and the plies after which enemy cells were cut off (active
//!   before, inactive after, still owned) more than 4 steps from every
//!   captured cell. Plies with no counted objective are not read.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use ice_and_fire_engine::{Bits, Command, State, SIZE};
use ice_and_fire_opening::econ::distances;
use ice_and_fire_opening::solo::rings;

use crate::profiles::{newest_snapshot, prepare, read_index, read_ranks, Request};
use crate::replay::{replay, Game};
use crate::style::releases;

/// The game turns from which locality is read.
const LOCAL_FROM: u32 = 8;
/// The game turns of the idle total.
const IDLE_FROM: u32 = 11;
const IDLE_TO: u32 = 14;

/// The objectives of the map of `state`: its middle chokes and its band cells.
pub fn objectives(state: &State) -> Bits {
    let mut out = ice_and_fire_early::zones::Zones::of(state).middle_chokes();
    for (cell, ring) in rings(state).into_iter().enumerate() {
        if ring <= 2 && state.map().valid.test(cell) {
            out.set(cell);
        }
    }
    out
}

/// The valid cells strictly nearer to `a` than to `b`.
fn nearer(state: &State, a: Bits, b: Bits) -> i32 {
    let (da, db) = (distances(state, a), distances(state, b));
    let mut n = 0;
    for c in state.map().valid.iter() {
        if da[c] != u8::MAX && da[c] < db[c] {
            n += 1;
        }
    }
    n
}

/// The value of objective `g` for the side whose land is `mine` against
/// `theirs` (both active land).
pub fn value_of(state: &State, mine: Bits, theirs: Bits, g: usize) -> i32 {
    let goal = Bits::cell(g);
    nearer(state, mine | goal, theirs) - nearer(state, theirs | goal, mine)
}

/// The objectives of `set` of value at least `min` for `seat` in `state`.
fn counted(state: &State, seat: usize, set: Bits, min: i32) -> Bits {
    let (mine, theirs) = (state.active(seat), state.active(1 - seat));
    let mut out = Bits::EMPTY;
    for g in set.iter() {
        if value_of(state, mine, theirs, g) >= min {
            out.set(g);
        }
    }
    out
}

/// The cell nearest to `from` among `set`, by map distance (the lowest cell
/// on a tie).
fn nearest(state: &State, from: usize, set: Bits) -> Option<(usize, u8)> {
    let mut best: Option<(usize, u8)> = None;
    for g in set.iter() {
        let d = state.map().distance(from, g);
        if d != u8::MAX && best.is_none_or(|(_, b)| d < b) {
            best = Some((g, d));
        }
    }
    best
}

/// The objective a unit that stood on `cells[0]`, `cells[1]` and now
/// `cells[2]` is the frontline unit of: the nearest of `open` from the
/// last cell, when each of the two moves brought it nearer.
pub fn frontline_of(state: &State, cells: [usize; 3], open: Bits) -> Option<usize> {
    let (g, _) = nearest(state, cells[2], open)?;
    let d = |c: usize| state.map().distance(c, g);
    (d(cells[0]) > d(cells[1]) && d(cells[1]) > d(cells[2])).then_some(g)
}

/// The value of `g` to the side alone: the cells nearer to its land plus
/// `g` than to the enemy's, less those nearer without `g` (the enemy's land
/// unchanged).
pub fn marginal_value(state: &State, mine: Bits, theirs: Bits, g: usize) -> i32 {
    nearer(state, mine | Bits::cell(g), theirs) - nearer(state, mine, theirs)
}

/// Whether the unit of `seat` nearest to `g` (by map distance, the lowest
/// cell on a tie) is nearer to it than it was at `before`, its cell in the
/// previous ply (`None` if it was not there).
pub fn nearest_approaching(
    state: &State,
    seat: usize,
    g: usize,
    before: impl Fn(u32) -> Option<usize>,
) -> bool {
    let Some((cell, d)) = nearest(state, g, state.units(seat)) else {
        return false;
    };
    let Some(id) = state.unit_id(cell) else {
        return false;
    };
    before(id).is_some_and(|b| state.map().distance(b, g) > d)
}

/// Whether the active cell of `seat` nearest to `g` holds a unit of `seat`.
pub fn tip_unit(state: &State, seat: usize, g: usize) -> bool {
    nearest(state, g, state.active(seat)).is_some_and(|(c, _)| state.units(seat).test(c))
}

/// The neutral valid cells the enemy's active land cannot reach over valid
/// cells that `mine` does not hold.
pub fn pockets(valid: Bits, mine: Bits, owned_theirs: Bits, active_theirs: Bits) -> Bits {
    let open = valid.and_not(mine);
    let mut reach = active_theirs & open;
    loop {
        let next = (reach | reach.neighbours()) & open;
        if next == reach {
            break;
        }
        reach = next;
    }
    valid.and_not(mine).and_not(owned_theirs).and_not(reach)
}

/// Whether a unit on `cell` has no valid cell outside `mine` within 2 steps.
pub fn is_boxed_in(state: &State, mine: Bits, cell: usize) -> bool {
    !state.map().valid.iter().any(|c| {
        let d = state.map().distance(cell, c);
        d != u8::MAX && d <= 2 && !mine.test(c)
    })
}

/// The trainings of a ply, and those at the tip: next to the destination of
/// an earlier MOVE of the ply.
pub fn tip_trainings(commands: &[Command]) -> (u32, u32) {
    let (mut trainings, mut tips) = (0, 0);
    let mut moved = Bits::EMPTY;
    for &command in commands {
        match command {
            Command::Move { x, y, .. } => {
                if usize::from(x) < SIZE && usize::from(y) < SIZE {
                    moved.set(usize::from(y) * SIZE + usize::from(x));
                }
            }
            Command::Train { x, y, .. } => {
                trainings += 1;
                if usize::from(x) < SIZE
                    && usize::from(y) < SIZE
                    && moved
                        .neighbours()
                        .test(usize::from(y) * SIZE + usize::from(x))
                {
                    tips += 1;
                }
            }
            _ => {}
        }
    }
    (trainings, tips)
}

/// What one side did at one game turn.
#[derive(Clone, Debug, Default)]
pub struct Turn {
    pub counted: u32,
    pub held: u32,
    /// Counted objectives not held, and those with a frontline unit.
    pub open: u32,
    pub fronted: u32,
    pub trainings: u32,
    pub tips: u32,
    pub pockets: u32,
    pub units: u32,
    pub idle: u32,
    /// Units that stood still, and those next to a valid cell not ours.
    pub still: u32,
    pub still_edge: u32,
    /// Counted objectives not held with the nearest unit approaching, and
    /// with a unit on the nearest active cell.
    pub approaching: u32,
    pub tip_unit: u32,
    /// Every objective, and those held.
    pub all: u32,
    pub all_held: u32,
    /// Objectives of marginal value at least the threshold, and those held.
    pub marginal: u32,
    pub marginal_held: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Locality {
    pub captured: u32,
    pub near: u32,
    pub capture_plies: u32,
    pub multi: u32,
    pub cut_plies: u32,
    pub cut_far: u32,
}

/// The measures of one side of one game.
#[derive(Clone, Debug, Default)]
pub struct Side {
    pub turns: BTreeMap<u32, Turn>,
    pub locality: Locality,
    /// Of the objectives counted at some turn, the first turn each is held.
    pub first_held: Vec<u32>,
    pub ever_counted: u32,
}

fn cell_of(state: &State, seat: usize, id: u32) -> Option<usize> {
    state
        .units(seat)
        .iter()
        .find(|&c| state.unit_id(c) == Some(id))
}

/// The measures of both sides of `game`.
pub fn sides(game: &Game, turns: u32, min_value: i32) -> [Side; 2] {
    let mut out = [Side::default(), Side::default()];
    let Some(first) = game.steps.first() else {
        return out;
    };
    let set = objectives(&first.state);
    for (seat, side) in out.iter_mut().enumerate() {
        let enemy = 1 - seat;
        let plies: Vec<usize> = (0..game.steps.len())
            .filter(|&i| {
                game.steps[i].state.to_act() == seat && game.steps[i].state.game_turn() <= turns
            })
            .collect();
        let mut first_hold: BTreeMap<usize, Option<u32>> = BTreeMap::new();
        for (k, &i) in plies.iter().enumerate() {
            let start = &game.steps[i].state;
            let next = game.steps.get(i + 1).map_or(&game.end, |s| &s.state);
            let turn = start.game_turn();
            let mine = start.active(seat);
            let count = counted(start, seat, set, min_value);
            let open = count.and_not(mine);
            let t = side.turns.entry(turn).or_default();
            t.counted = count.count();
            t.held = (count & mine).count();
            t.open = open.count();
            t.all = set.count();
            t.all_held = (set & mine).count();
            let mut marginal = Bits::EMPTY;
            for g in set.iter() {
                if marginal_value(start, mine, start.active(enemy), g) >= min_value {
                    marginal.set(g);
                }
            }
            t.marginal = marginal.count();
            t.marginal_held = (marginal & mine).count();
            let before = |id: u32| {
                k.checked_sub(1)
                    .and_then(|j| cell_of(&game.steps[plies[j]].state, seat, id))
            };
            for g in open.iter() {
                if nearest_approaching(start, seat, g, before) {
                    t.approaching += 1;
                }
                if tip_unit(start, seat, g) {
                    t.tip_unit += 1;
                }
            }
            for g in count.iter() {
                let slot = first_hold.entry(g).or_insert(None);
                if slot.is_none() && mine.test(g) {
                    *slot = Some(turn);
                }
            }
            // Frontline units: the positions at the starts of the two
            // previous plies of the side.
            if k >= 2 {
                let mut fronted = Bits::EMPTY;
                for cell in start.units(seat).iter() {
                    let Some(id) = start.unit_id(cell) else {
                        continue;
                    };
                    let (a, b) = (
                        cell_of(&game.steps[plies[k - 2]].state, seat, id),
                        cell_of(&game.steps[plies[k - 1]].state, seat, id),
                    );
                    if let (Some(a), Some(b)) = (a, b) {
                        if let Some(g) = frontline_of(start, [a, b, cell], open) {
                            fronted.set(g);
                        }
                    }
                }
                t.fronted = fronted.count();
            }
            let (trainings, tips) = tip_trainings(&game.steps[i].commands);
            t.trainings = trainings;
            t.tips = tips;
            let valid = start.map().valid;
            t.pockets = pockets(
                valid,
                start.owned(seat),
                start.owned(enemy),
                start.active(enemy),
            )
            .count();
            for cell in start.units(seat).iter() {
                let Some(id) = start.unit_id(cell) else {
                    continue;
                };
                t.units += 1;
                let stayed = cell_of(next, seat, id) == Some(cell);
                if stayed && is_boxed_in(start, start.owned(seat), cell) {
                    t.idle += 1;
                }
                if stayed {
                    t.still += 1;
                    if !(Bits::cell(cell).neighbours() & valid.and_not(start.owned(seat)))
                        .is_empty()
                    {
                        t.still_edge += 1;
                    }
                }
            }
            if turn >= LOCAL_FROM && !count.is_empty() {
                let taken = start.owned(enemy) & next.owned(seat) & valid;
                if !taken.is_empty() {
                    let loc = &mut side.locality;
                    loc.capture_plies += 1;
                    let mut regions = Bits::EMPTY;
                    for c in taken.iter() {
                        loc.captured += 1;
                        if let Some((g, d)) = nearest(start, c, count) {
                            regions.set(g);
                            if d <= 2 {
                                loc.near += 1;
                            }
                        }
                    }
                    let spread = regions.iter().any(|a| {
                        regions.iter().any(|b| {
                            start.map().distance(a, b) > 2 && start.map().distance(a, b) != u8::MAX
                        })
                    });
                    if spread {
                        loc.multi += 1;
                    }
                    let cut = start.active(enemy).and_not(next.active(enemy)) & next.owned(enemy);
                    if !cut.is_empty() {
                        loc.cut_plies += 1;
                        let far = cut.iter().any(|c| {
                            taken.iter().all(|t| {
                                let d = start.map().distance(c, t);
                                d == u8::MAX || d > 4
                            })
                        });
                        if far {
                            loc.cut_far += 1;
                        }
                    }
                }
            }
        }
        side.ever_counted = first_hold.len() as u32;
        side.first_held = first_hold.values().flatten().copied().collect();
    }
    out
}

/// A cell of a table: `num / den` pooled over the sides, and the sides
/// behind it.
fn pooled(sides: &[Side], turns: &[u32], f: impl Fn(&Turn) -> (u32, u32)) -> String {
    let (mut num, mut den, mut n) = (0u64, 0u64, 0u32);
    for side in sides {
        let (mut sn, mut sd) = (0u64, 0u64);
        for turn in turns {
            if let Some(t) = side.turns.get(turn) {
                let (a, b) = f(t);
                sn += u64::from(a);
                sd += u64::from(b);
            }
        }
        if sd > 0 {
            n += 1;
            num += sn;
            den += sd;
        }
    }
    if den == 0 {
        "-".to_string()
    } else {
        format!("{:.2} ({n})", num as f64 / den as f64)
    }
}

type Groups = BTreeMap<(String, usize), Vec<Side>>;

fn seat_name(seat: usize) -> &'static str {
    if seat == 0 {
        "red"
    } else {
        "blue"
    }
}

fn table(
    text: &mut String,
    title: &str,
    groups: &Groups,
    from: u32,
    last: u32,
    total: Option<(u32, u32)>,
    f: impl Fn(&Turn) -> (u32, u32),
) {
    let first = from.min(last);
    let _ = writeln!(text, "\n## {title}\n");
    let mut head = String::from("| Group |");
    for t in first..=last {
        let _ = write!(head, " {t} |");
    }
    if let Some((a, b)) = total {
        let _ = write!(head, " {a}-{b} |");
    }
    let _ = writeln!(text, "{head}");
    let columns = (last + 1 - first) as usize + usize::from(total.is_some());
    let _ = writeln!(text, "| --- |{}", " --- |".repeat(columns));
    for ((group, seat), sides) in groups {
        let mut row = format!("| {group} {} ({}) |", seat_name(*seat), sides.len());
        for t in first..=last {
            let _ = write!(row, " {} |", pooled(sides, &[t], &f));
        }
        if let Some((a, b)) = total {
            let turns: Vec<u32> = (a..=b.min(last)).collect();
            let _ = write!(row, " {} |", pooled(sides, &turns, &f));
        }
        let _ = writeln!(text, "{row}");
    }
}

fn share(a: u32, b: u32) -> String {
    if b == 0 {
        "-".to_string()
    } else {
        format!("{:.2}", f64::from(a) / f64::from(b))
    }
}

/// The report's text from the sides of each group and seat.
pub fn report(groups: &Groups, turns: u32, from: u32, min_value: i32, left_out: usize) -> String {
    let mut text = String::new();
    let _ = writeln!(text, "# Roles in the early game by group\n");
    let _ = writeln!(
        text,
        "Measures of games/ice-and-fire/research/codingame/2026-11-20-frontline-plan.md (Step 1), per side at the start of its ply, game turns 1 to {turns}, objectives of value at least {min_value}. Each figure is pooled over the sides of its group and seat still playing at that turn; in brackets, the sides behind it. Red is seat 0, blue seat 1. Games left out: {left_out}."
    );
    table(
        &mut text,
        "Counted objectives per side",
        groups,
        from,
        turns,
        None,
        |t| (t.counted, 1),
    );
    table(
        &mut text,
        "Counted objectives held per side",
        groups,
        from,
        turns,
        None,
        |t| (t.held, 1),
    );
    table(
        &mut text,
        "Share of counted objectives not held that have a frontline unit",
        groups,
        from,
        turns,
        None,
        |t| (t.fronted, t.open),
    );
    table(
        &mut text,
        "Share of trainings at the tip (after a move to a neighbour)",
        groups,
        from,
        turns,
        None,
        |t| (t.tips, t.trainings),
    );
    table(
        &mut text,
        "Pockets per side (neutral cells the enemy cannot reach)",
        groups,
        from,
        turns,
        None,
        |t| (t.pockets, 1),
    );
    table(
        &mut text,
        "Share of unit turns idle with no cell to take within 2 steps",
        groups,
        from,
        turns,
        Some((IDLE_FROM, IDLE_TO)),
        |t| (t.idle, t.units),
    );
    table(&mut text, "Variant: share of counted objectives not held whose nearest unit approached in the previous ply", groups, from, turns, None, |t| {
        (t.approaching, t.open)
    });
    table(&mut text, "Variant: share of counted objectives not held with a unit on our nearest active cell (tip unit)", groups, from, turns, None, |t| {
        (t.tip_unit, t.open)
    });
    table(
        &mut text,
        "Variant: share of unit turns standing still, any reason",
        groups,
        from,
        turns,
        Some((IDLE_FROM, IDLE_TO)),
        |t| (t.still, t.units),
    );
    table(
        &mut text,
        "Variant: share of unit turns standing still next to a cell not ours",
        groups,
        from,
        turns,
        Some((IDLE_FROM, IDLE_TO)),
        |t| (t.still_edge, t.units),
    );
    table(
        &mut text,
        "Variant: every objective, no threshold, per side",
        groups,
        from,
        turns,
        None,
        |t| (t.all, 1),
    );
    table(
        &mut text,
        "Variant: every objective held per side",
        groups,
        from,
        turns,
        None,
        |t| (t.all_held, 1),
    );
    table(
        &mut text,
        "Variant: objectives of marginal value at least the threshold, per side",
        groups,
        from,
        turns,
        None,
        |t| (t.marginal, 1),
    );
    table(
        &mut text,
        "Variant: those of marginal value held per side",
        groups,
        from,
        turns,
        None,
        |t| (t.marginal_held, 1),
    );
    let _ = writeln!(text, "\n## First game turn an objective is held\n");
    let _ = writeln!(
        text,
        "| Group | Sides | Objectives counted | Held by game turn {turns} | Median first turn held |"
    );
    let _ = writeln!(text, "| --- | --- | --- | --- | --- |");
    for ((group, seat), sides) in groups {
        let counted: u32 = sides.iter().map(|s| s.ever_counted).sum();
        let mut firsts: Vec<u32> = sides.iter().flat_map(|s| s.first_held.clone()).collect();
        firsts.sort_unstable();
        let median = if firsts.is_empty() {
            "-".to_string()
        } else {
            let n = firsts.len();
            format!("{:.1}", (firsts[(n - 1) / 2] + firsts[n / 2]) as f64 / 2.0)
        };
        let _ = writeln!(
            text,
            "| {group} {} | {} | {counted} | {} | {median} |",
            seat_name(*seat),
            sides.len(),
            firsts.len()
        );
    }
    let _ = writeln!(text, "\n## Locality, game turns {LOCAL_FROM} to {turns}\n");
    let _ = writeln!(
        text,
        "| Group | Sides | Captured cells | Within 2 steps of an objective | Plies with captures | Captures near two objectives more than 2 apart | Plies that cut enemy land | Cut more than 4 steps from every capture |"
    );
    let _ = writeln!(text, "| --- | --- | --- | --- | --- | --- | --- | --- |");
    for ((group, seat), sides) in groups {
        let mut l = Locality::default();
        for s in sides {
            l.captured += s.locality.captured;
            l.near += s.locality.near;
            l.capture_plies += s.locality.capture_plies;
            l.multi += s.locality.multi;
            l.cut_plies += s.locality.cut_plies;
            l.cut_far += s.locality.cut_far;
        }
        let _ = writeln!(
            text,
            "| {group} {} | {} | {} | {} | {} | {} ({}) | {} | {} ({}) |",
            seat_name(*seat),
            sides.len(),
            l.captured,
            share(l.near, l.captured),
            l.capture_plies,
            share(l.multi, l.capture_plies),
            l.multi,
            l.cut_plies,
            share(l.cut_far, l.capture_plies),
            l.cut_far
        );
    }
    text
}

/// The report, as Markdown: one row per group and seat. The groups are
/// the players of `request.players` (each its own), the rest of the top
/// `request.top` of the leaderboard snapshot, and our releases in games
/// against them.
pub fn run(request: &Request, turns: u32, from: u32, min_value: i32) -> Result<String, String> {
    let (entries, left_out) = read_index(&request.data.join("index.jsonl"), request)?;
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
    type Done = Vec<(String, usize, Side)>;
    let jobs = request.jobs.clamp(1, 64);
    let size = entries.len().div_ceil(jobs).max(1);
    let mut slots: Vec<Option<Done>> = Vec::new();
    slots.resize_with(entries.len(), || None);
    std::thread::scope(|scope| {
        for (chunk, out) in entries.chunks(size).zip(slots.chunks_mut(size)) {
            let (group_of, release_of) = (&group_of, &release_of);
            scope.spawn(move || {
                for (entry, slot) in chunk.iter().zip(out.iter_mut()) {
                    let Ok((meta, record)) = prepare(&request.data, entry, &request.me) else {
                        continue;
                    };
                    let Some(us) = meta.our_seat else { continue };
                    let theirs = record.players[1 - us].name.clone();
                    let Some(theirs) = group_of(&theirs) else {
                        continue;
                    };
                    let game = replay(&meta.game_id.to_string(), record);
                    let [a, b] = sides(&game, turns, min_value);
                    let mut both = [Some(a), Some(b)];
                    let mut done = vec![(theirs, 1 - us, both[1 - us].take().unwrap_or_default())];
                    if let Some(release) = release_of.get(&meta.game_id) {
                        done.push((release.clone(), us, both[us].take().unwrap_or_default()));
                    }
                    *slot = Some(done);
                }
            });
        }
    });
    let mut groups: Groups = BTreeMap::new();
    for (group, seat, side) in slots.into_iter().flatten().flatten() {
        groups.entry((group, seat)).or_default().push(side);
    }
    Ok(report(&groups, turns, from, min_value, left_out.len()))
}

#[cfg(test)]
mod tests;
