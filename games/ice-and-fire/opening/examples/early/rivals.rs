//! E003's other openings, as opponents of the harness: `fill` and `rush`,
//! ported from the screening commit 277d510 (`IAF_OPENING=fill` and `rush`
//! with their default settings). They play their rules whatever the gap.

use ice_and_fire_engine::{Bits, Command, State, CELLS, HQ, SIZE};
use ice_and_fire_opening::econ::distances;

/// Cost of a level 1 unit.
const UNIT_COST: i64 = 10;
/// `fill` trains while its side has this many free cells per unit.
const PER_UNIT: i64 = 3;

fn xy(cell: usize) -> (u8, u8) {
    ((cell % SIZE) as u8, (cell / SIZE) as u8)
}

/// What one turn knows about the map, as in `econ`.
struct Turn {
    me: usize,
    theirs: [u8; CELLS],
    to_hq: [u8; CELLS],
    free: Bits,
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

    fn value(&self, cell: usize) -> i64 {
        let open = i64::from((Bits::cell(cell).neighbours() & self.free).count());
        let contest = 12 - i64::from(self.theirs[cell].min(12));
        3 * open + 2 * contest - i64::from(self.to_hq[cell].min(30)) / 4
    }

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

fn play(state: &mut State, command: Command, out: &mut Vec<Command>) -> bool {
    let accepted = state.apply(command);
    if accepted {
        out.push(command);
    }
    accepted
}

fn step(state: &mut State, from: usize, to: usize, out: &mut Vec<Command>) -> bool {
    let Some((id, _)) = state.unit_at(from) else {
        return false;
    };
    let (x, y) = xy(to);
    play(state, Command::Move { id, x, y }, out)
}

fn train(state: &mut State, cell: usize, out: &mut Vec<Command>) -> bool {
    let (x, y) = xy(cell);
    play(state, Command::Train { level: 1, x, y }, out)
}

/// `fill`: free cells first (the moves of `econ`), then trainings on the best
/// free cell next to the territory while the side has [`PER_UNIT`] free
/// cells per unit; no mines.
pub fn fill(state: &State) -> Vec<Command> {
    let mut scratch = state.clone();
    let mut turn = Turn::new(state);
    let mut out = Vec::new();
    let me = turn.me;
    let mut idle: Vec<usize> = Vec::new();
    loop {
        let enter = scratch.enterable(me, 1);
        let mut pick: Option<(u32, usize)> = None;
        for from in scratch.movable_units().iter() {
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
            Some(to) if step(&mut scratch, from, to, &mut out) => turn.claim(to),
            _ => idle.push(from),
        }
    }
    for from in idle {
        if !scratch.movable_units().test(from) {
            continue;
        }
        let target = [turn.side, turn.free].into_iter().find_map(|cells| {
            cells
                .iter()
                .min_by_key(|&cell| (scratch.map().distance(from, cell), cell))
        });
        if let Some(target) = target {
            step(&mut scratch, from, target, &mut out);
        }
    }
    while scratch.gold(me) >= UNIT_COST {
        let units = scratch.unit_count(me) as i64;
        if i64::from(turn.side.count()) < PER_UNIT * (units + 1) {
            break;
        }
        let cells = scratch.near_active(me) & turn.free & scratch.enterable(me, 1);
        let Some(cell) = turn.best(cells) else {
            break;
        };
        if !train(&mut scratch, cell, &mut out) {
            break;
        }
        turn.claim(cell);
    }
    out
}

/// `rush`, the gold boss's rule: each unit, nearest the enemy HQ first, to
/// the neighbour nearest the enemy HQ (a free cell on a tie), then trainings
/// on the free cells next to the territory nearest the enemy HQ.
pub fn rush(state: &State) -> Vec<Command> {
    let mut scratch = state.clone();
    let mut turn = Turn::new(state);
    let mut out = Vec::new();
    let me = turn.me;
    let mut units: Vec<usize> = scratch.movable_units().iter().collect();
    units.sort_by_key(|&cell| (turn.to_hq[cell], cell));
    for from in units {
        let enter = scratch.enterable(me, 1);
        let best = (Bits::cell(from).neighbours() & enter)
            .iter()
            .min_by_key(|&cell| (turn.to_hq[cell], !turn.free.test(cell), cell));
        if let Some(to) = best {
            if (turn.to_hq[to] < turn.to_hq[from] || turn.free.test(to))
                && step(&mut scratch, from, to, &mut out)
            {
                turn.claim(to);
            }
        }
    }
    while scratch.gold(me) >= UNIT_COST {
        let cells = scratch.near_active(me) & turn.free & scratch.enterable(me, 1);
        let Some(cell) = cells.iter().min_by_key(|&cell| (turn.to_hq[cell], cell)) else {
            break;
        };
        if !train(&mut scratch, cell, &mut out) {
            break;
        }
        turn.claim(cell);
    }
    out
}
