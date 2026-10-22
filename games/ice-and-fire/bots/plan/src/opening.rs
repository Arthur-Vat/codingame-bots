//! The opening: the turns before the two territories meet (experiment E003,
//! `games/ice-and-fire/research/early-game.md`).
//!
//! While the territories are more than [`GAP`] steps apart, a turn is played
//! by rules instead of the plan search:
//!
//! 1. **Moves.** Every unit that has a free (neutral) neighbour takes one, a
//!    different cell for each unit; the unit with the fewest such cells moves
//!    first, and each takes the free cell worth most (`Turn::value`:
//!    contested cells near the enemy first, then cells that open more free
//!    cells). The other units walk toward the nearest free cell of our side
//!    of the map, else the nearest free cell.
//! 2. **Purchases**, while the gold allows: a level 1 training on the free
//!    cell next to the territory worth most, or a mine on an own mine spot of
//!    our side, whichever returns more gold by game turn [`HORIZON`] (at least
//!    [`MIN_TURNS_LEFT`] turns away). A unit
//!    is worth the cells it can take before the territories meet; a mine 4 a
//!    turn. A purchase that does not pay back by the horizon is not made.
//!
//! Every command is applied to a scratch copy of the state as it is chosen,
//! so the turn holds only commands the rules accept.

use ice_and_fire_engine::state::BUILD_COST;
use ice_and_fire_engine::{Bits, Command, State, CELLS, HQ, SIZE};

/// The opening is played while the territories are more than this many steps
/// apart; at this gap or less the plan search plays.
pub const GAP: u8 = 3;

/// The game turn by which a purchase of the opening must pay back...
pub const HORIZON: i64 = 15;
/// ...but never fewer than this many turns from now, since a game between
/// bots that survive the opening lasts about 20 turns.
pub const MIN_TURNS_LEFT: i64 = 6;

/// Cost of a level 1 unit.
const UNIT_COST: i64 = 10;

/// A mine spot is used only when the enemy territory is more than this many
/// steps from it.
const SAFE_SPOT: u8 = 3;

fn xy(cell: usize) -> (u8, u8) {
    ((cell % SIZE) as u8, (cell / SIZE) as u8)
}

/// Steps over non-void cells from the cells of `from`; `u8::MAX` where there
/// is no path.
pub fn distances(state: &State, from: Bits) -> [u8; CELLS] {
    let mut dist = [u8::MAX; CELLS];
    let valid = state.map().valid;
    let mut frontier = from & valid;
    let mut seen = frontier;
    let mut d = 0u8;
    while !frontier.is_empty() {
        for cell in frontier.iter() {
            dist[cell] = d;
        }
        let next = (frontier.neighbours() & valid).and_not(seen);
        seen |= next;
        frontier = next;
        d = d.saturating_add(1);
    }
    dist
}

/// The fewest steps from a cell of one territory to a cell of the other,
/// `u8::MAX` if there is no path.
pub fn gap(state: &State) -> u8 {
    let dist = distances(state, state.owned(0));
    state
        .owned(1)
        .iter()
        .map(|cell| dist[cell])
        .min()
        .unwrap_or(u8::MAX)
}

/// What one turn of the opening knows about the map.
struct Turn {
    me: usize,
    /// Steps from the enemy's territory.
    theirs: [u8; CELLS],
    /// Steps from the enemy HQ.
    to_hq: [u8; CELLS],
    /// Neutral cells not yet claimed by a command of this turn.
    free: Bits,
    /// The free cells of our side: at least as near our territory as the
    /// enemy's.
    side: Bits,
}

impl Turn {
    fn new(state: &State) -> Turn {
        let me = state.to_act();
        let mine = distances(state, state.owned(me));
        let theirs = distances(state, state.owned(1 - me));
        let to_hq = distances(state, Bits::cell(HQ[1 - me]));
        let free = state.map().valid.and_not(state.owned(0) | state.owned(1));
        let mut side = Bits::EMPTY;
        for cell in free.iter() {
            if mine[cell] <= theirs[cell] {
                side.set(cell);
            }
        }
        Turn {
            me,
            theirs,
            to_hq,
            free,
            side,
        }
    }

    /// How much a free cell is worth taking now: 3 for each free cell next to
    /// it, 2 for each step it is nearer the enemy's territory than 12, less a
    /// quarter of its steps to the enemy HQ (a tie-break toward the middle).
    fn value(&self, cell: usize) -> i64 {
        let open = i64::from((Bits::cell(cell).neighbours() & self.free).count());
        let contest = 12 - i64::from(self.theirs[cell].min(12));
        3 * open + 2 * contest - i64::from(self.to_hq[cell].min(30)) / 4
    }

    /// The cell of `cells` worth most, the lowest index on a tie.
    fn best(&self, cells: Bits) -> Option<usize> {
        cells
            .iter()
            .max_by_key(|&cell| (self.value(cell), std::cmp::Reverse(cell)))
    }

    fn claim(&mut self, cell: usize) {
        self.free.clear(cell);
        self.side.clear(cell);
    }
}

/// Applies `command` to `state` and keeps it in `out` if the rules accept it;
/// whether they did.
fn play(state: &mut State, command: Command, out: &mut Vec<Command>) -> bool {
    let accepted = state.apply(command);
    if accepted {
        out.push(command);
    }
    accepted
}

/// Moves the unit on `from` toward `to`; whether the rules accepted it.
fn step(state: &mut State, from: usize, to: usize, out: &mut Vec<Command>) -> bool {
    let Some((id, _)) = state.unit_at(from) else {
        return false;
    };
    let (x, y) = xy(to);
    play(state, Command::Move { id, x, y }, out)
}

/// Gold collected by the horizon, `h` turns away, from a unit trained now
/// that takes a new cell on each of its next `k` turns (its own cell pays its
/// upkeep), less its cost.
pub fn unit_return(h: i64, k: i64) -> i64 {
    let k = k.clamp(0, (h - 1).max(0));
    k * h - k * (k + 1) / 2 - UNIT_COST
}

/// The opening's commands for the player to act, or `None` once the
/// territories are [`GAP`] steps apart or less, or the game is over.
pub fn turn(state: &State) -> Option<Vec<Command>> {
    if state.outcome().is_some() || gap(state) <= GAP {
        return None;
    }
    let mut scratch = state.clone();
    let mut turn = Turn::new(state);
    let mut out = Vec::new();
    moves(&mut scratch, &mut turn, &mut out);
    purchases(&mut scratch, &mut turn, &mut out);
    Some(out)
}

/// Units with a free neighbour, fewest choices first, each take the free
/// neighbour worth most that no other unit took; the others walk toward the
/// nearest free cell, of our side first.
fn moves(state: &mut State, turn: &mut Turn, out: &mut Vec<Command>) {
    let me = turn.me;
    let mut idle: Vec<usize> = Vec::new();
    loop {
        let enter = state.enterable(me, 1);
        let mut pick: Option<(u32, usize)> = None;
        for from in state.movable_units().iter() {
            if idle.contains(&from) {
                continue;
            }
            let options = (Bits::cell(from).neighbours() & turn.free & enter).count();
            if options == 0 {
                idle.push(from);
            } else if pick.is_none_or(|(n, _)| options < n) {
                pick = Some((options, from));
            }
        }
        let Some((_, from)) = pick else {
            break;
        };
        match turn.best(Bits::cell(from).neighbours() & turn.free & enter) {
            Some(to) if step(state, from, to, out) => turn.claim(to),
            _ => idle.push(from),
        }
    }
    for from in idle {
        if !state.movable_units().test(from) {
            continue;
        }
        let target = [turn.side, turn.free].into_iter().find_map(|cells| {
            cells
                .iter()
                .min_by_key(|&cell| (state.map().distance(from, cell), cell))
        });
        if let Some(target) = target {
            step(state, from, target, out);
        }
    }
}

/// Trainings and mines, the one that returns more by the horizon first, while
/// the gold allows and the purchase pays back.
fn purchases(state: &mut State, turn: &mut Turn, out: &mut Vec<Command>) {
    let me = turn.me;
    let h = (HORIZON - i64::from(state.game_turn())).max(MIN_TURNS_LEFT);
    // Both sides grow toward each other, so the gap closes by about two steps
    // a turn; a unit takes a cell a turn until then.
    let to_contact = (i64::from(gap(state)) - 2).max(0) / 2;
    loop {
        let gold = state.gold(me);
        let units = state.unit_count(me) as i64;
        let room = i64::from(turn.side.count());
        let cell = turn.best(state.near_active(me) & turn.free & state.enterable(me, 1));
        let unit_gain = unit_return(h, (room / (units + 1)).min(to_contact));
        let can_unit = cell.is_some() && gold >= UNIT_COST && unit_gain > 0;
        let mine_gain = 4 * h - BUILD_COST;
        let spot = (state.map().spots & state.active(me) & state.empty_cells())
            .iter()
            .filter(|&spot| turn.theirs[spot] > SAFE_SPOT)
            .min_by_key(|&spot| (state.map().distance(HQ[me], spot), spot));
        let can_mine = spot.is_some() && gold >= BUILD_COST && mine_gain > 0;
        if can_mine && (!can_unit || mine_gain >= unit_gain) {
            let (x, y) = xy(spot.unwrap_or_default());
            if play(state, Command::BuildMine { x, y }, out) {
                continue;
            }
            break;
        }
        let Some(cell) = cell.filter(|_| can_unit) else {
            break;
        };
        let (x, y) = xy(cell);
        if !play(state, Command::Train { level: 1, x, y }, out) {
            break;
        }
        turn.claim(cell);
    }
}

#[cfg(test)]
mod tests;
