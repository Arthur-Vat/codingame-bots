//! The front of the map in the evaluation (E083), from the turn-1 planner
//! of E080 (the owner, 2026-11-26): the race to the contested cells, in
//! which units count, the goal cells of each front, and units that hold
//! the front line. Each weight at 0 is v032; the terms count until game
//! turn [`FRONT_UNTIL`].

use ice_and_fire_engine::{Bits, State, CELLS, HQ, SIZE};
use ice_and_fire_opening::econ::distances;

/// The last game turn at which the front's terms count (as the contested
/// band of E076).
pub const FRONT_UNTIL: u32 = 14;

/// What each contested band cell (E073's band) counts for the side that
/// arrives first ([`arrivals`]), less those the enemy arrives at first: 0
/// for nothing (v032).
pub const RACE: i64 = 2;

/// What a front counts for a side whose deepest goal cell held or reached
/// first is its entrance, centre or outpost ([`goal_cells`]): 0 for
/// nothing (v032).
pub const GOAL_E: i64 = 4;
/// See [`GOAL_E`].
pub const GOAL_C: i64 = 8;
/// See [`GOAL_E`].
pub const GOAL_O: i64 = 12;

/// What each unit of a side on a band cell it holds actively, which the
/// enemy cannot take at its next turn, counts: 0 for nothing (v032).
pub const HOLD: i64 = 4;

/// Whether E060's charge on idle units counts their steps to the front
/// (the enemy's cells and the band cells not the side's) rather than to the
/// nearest cell not the side's, until [`FRONT_UNTIL`]: false is v032.
pub const IDLE_TO_FRONT: bool = true;

/// The farthest arrival counted, in turns.
const CAP: usize = 8;

/// The cells `side` reaches within each number of turns, 0 to [`CAP`]:
/// its active land at once; a cell next to it in one turn (a training); the
/// steps of its nearest unit on its active land (a unit moves a cell a
/// turn, for free); one more than the steps from its land beyond (a chain
/// of trainings, slower: each cell of it is paid). Units, buildings and
/// owners on the way are ignored.
pub fn arrivals(state: &State, side: usize) -> [Bits; CAP + 1] {
    let valid = state.map().valid;
    let land = state.active(side);
    let mut floods = [land; CAP + 1];
    for j in 1..=CAP {
        floods[j] = (floods[j - 1] | floods[j - 1].neighbours()) & valid;
    }
    let mut units = state.units(side) & land;
    let mut out = [land; CAP + 1];
    for k in 1..=CAP {
        units = (units | units.neighbours()) & valid;
        out[k] = out[k - 1] | floods[(k - 1).max(1)] | units;
    }
    out
}

/// The first number of turns in which `layers` reach `cell`, past [`CAP`]
/// if never.
fn first(layers: &[Bits; CAP + 1], cell: usize) -> usize {
    layers.iter().position(|l| l.test(cell)).unwrap_or(CAP + 1)
}

/// The cells of `cells` that `a` reaches strictly before `b`, less those
/// `b` reaches strictly before `a`.
pub fn race_net(a: &[Bits; CAP + 1], b: &[Bits; CAP + 1], cells: Bits) -> i64 {
    let mut net = 0;
    for k in 0..=CAP {
        let (pa, pb) = if k == 0 {
            (Bits::EMPTY, Bits::EMPTY)
        } else {
            (a[k - 1], b[k - 1])
        };
        let new_a = a[k].and_not(pa) & cells;
        let new_b = b[k].and_not(pb) & cells;
        net += i64::from(new_a.and_not(b[k]).count()) - i64::from(new_b.and_not(a[k]).count());
    }
    net
}

/// The goal cells of each front of a map for red (seat 0): entrance,
/// centre, outpost.
struct Goals {
    valid: Bits,
    fronts: Vec<[usize; 3]>,
}

thread_local! {
    static GOALS: std::cell::RefCell<Option<Goals>> = const { std::cell::RefCell::new(None) };
}

fn xy(cell: usize) -> (i64, i64) {
    ((cell % SIZE) as i64, (cell / SIZE) as i64)
}

/// The front of a cell by its place along the middle: 0 bottom-left,
/// 1 middle, 2 top-right (red's view).
fn lane(cell: usize) -> usize {
    let (x, y) = xy(cell);
    match x - y {
        k if k <= -4 => 0,
        k if k >= 4 => 2,
        _ => 1,
    }
}

/// The most central shortest path from `start` to `goal` (each step to
/// the neighbour nearer `goal` with the least `|x - y|`, then the least x,
/// then the least y), `start` first.
fn central_path(state: &State, start: usize, goal: usize) -> Vec<usize> {
    let valid = state.map().valid;
    let d = distances(state, Bits::cell(goal));
    let mut out = vec![start];
    if d[start] == u8::MAX {
        return out;
    }
    let mut cur = start;
    while d[cur] > 0 {
        let next = (Bits::cell(cur).neighbours() & valid)
            .iter()
            .filter(|&n| d[n] != u8::MAX && d[n] + 1 == d[cur])
            .min_by_key(|&n| {
                let (x, y) = xy(n);
                ((x - y).abs(), x, y)
            });
        let Some(n) = next else { break };
        out.push(n);
        cur = n;
    }
    out
}

/// The goal cells of `state`'s map for red, computed once a map: for each
/// front (the cells within a ring of even between the HQs, by
/// [`lane`], at least 4 of them), its entrance, centre and outpost (see
/// the journal entry E083).
pub fn goal_cells(state: &State) -> Vec<[usize; 3]> {
    GOALS.with(|slot| {
        let mut goals = slot.borrow_mut();
        let valid = state.map().valid;
        if goals.as_ref().is_none_or(|g| g.valid != valid) {
            *goals = Some(Goals {
                valid,
                fronts: compute_goals(state),
            });
        }
        goals.as_ref().map(|g| g.fronts.clone()).unwrap_or_default()
    })
}

fn compute_goals(state: &State) -> Vec<[usize; 3]> {
    let valid = state.map().valid;
    let a = distances(state, Bits::cell(HQ[0]));
    let b = distances(state, Bits::cell(HQ[1]));
    let mut parts = [Bits::EMPTY; 3];
    for c in valid.iter() {
        if a[c] != u8::MAX && b[c] != u8::MAX && a[c].abs_diff(b[c]) <= 2 {
            parts[lane(c)].set(c);
        }
    }
    let kept: Vec<usize> = (0..3).filter(|&l| parts[l].count() >= 4).collect();
    let near: Vec<[u8; CELLS]> = kept.iter().map(|&l| distances(state, parts[l])).collect();
    // Each cell's region: the nearest kept front, its own lane first on a
    // tie, then the middle.
    let region = |c: usize| -> Option<usize> {
        kept.iter()
            .enumerate()
            .min_by_key(|&(i, &l)| {
                let tie = if l == lane(c) {
                    0
                } else if l == 1 {
                    1
                } else {
                    2
                };
                (near[i][c], tie)
            })
            .map(|(_, &l)| l)
    };
    let mut out = Vec::new();
    for &l in &kept {
        let cells: Vec<usize> = parts[l].iter().collect();
        let centre = if l == 1 {
            cells.iter().copied().min_by_key(|&c| {
                let (x, y) = xy(c);
                ((x - y).abs(), a[c], x, y)
            })
        } else {
            let n = cells.len() as i64;
            let (sx, sy) = cells.iter().fold((0, 0), |(sx, sy), &c| {
                let (x, y) = xy(c);
                (sx + x, sy + y)
            });
            cells.iter().copied().min_by_key(|&c| {
                let (x, y) = xy(c);
                let (dx, dy) = (n * x - sx, n * y - sy);
                (dx * dx + dy * dy, a[c], x, y)
            })
        };
        let Some(centre) = centre else { continue };
        let path = central_path(state, HQ[0], centre);
        let entrance = match path.iter().rposition(|&c| region(c) != Some(l)) {
            Some(i) if i + 1 < path.len() => path[i + 1],
            _ => centre,
        };
        let beyond = central_path(state, centre, HQ[1]);
        let outpost = beyond.get(2).copied().unwrap_or(centre);
        out.push([entrance, centre, outpost]);
    }
    out
}

/// The race and goal terms for `seat`, less the enemy's (0 after
/// [`FRONT_UNTIL`]); `band` is E073's band.
pub fn score(state: &State, seat: usize, band: Bits) -> i64 {
    if (RACE == 0 && GOAL_E == 0 && GOAL_C == 0 && GOAL_O == 0) || state.game_turn() > FRONT_UNTIL {
        return 0;
    }
    let ours = arrivals(state, seat);
    let theirs = arrivals(state, 1 - seat);
    let mut total = RACE * race_net(&ours, &theirs, band);
    if GOAL_E != 0 || GOAL_C != 0 || GOAL_O != 0 {
        let layers = |side: usize| if side == seat { &ours } else { &theirs };
        for front in goal_cells(state) {
            for side in 0..2 {
                let other = 1 - side;
                let value = [(2, GOAL_O), (1, GOAL_C), (0, GOAL_E)]
                    .into_iter()
                    .find(|&(i, _)| {
                        // The enemy's goal cells are the mirrors of red's.
                        let cell = if side == 0 {
                            front[i]
                        } else {
                            CELLS - 1 - front[i]
                        };
                        state.active(side).test(cell)
                            || first(layers(side), cell) < first(layers(other), cell)
                    })
                    .map_or(0, |(_, w)| w);
                total += if side == seat { value } else { -value };
            }
        }
    }
    total
}

/// What `seat`'s units holding the front count ([`HOLD`]): each on a band
/// cell it holds actively that is not in `exposed` (the cells the enemy
/// can take at its next turn).
pub fn hold(state: &State, seat: usize, band: Bits, exposed: Bits) -> i64 {
    if HOLD == 0 || state.game_turn() > FRONT_UNTIL {
        return 0;
    }
    let held = (state.units(seat) & state.active(seat) & band).and_not(exposed);
    HOLD * i64::from(held.count())
}

/// The cells an idle unit of `seat` walks toward (E060): with
/// [`IDLE_TO_FRONT`] until [`FRONT_UNTIL`], the enemy's cells and the band
/// cells not `seat`'s; otherwise (or if there is none) the cells not its.
pub fn idle_targets(state: &State, seat: usize, band: Bits) -> Bits {
    let valid = state.map().valid;
    let open = valid.and_not(state.owned(seat));
    if !IDLE_TO_FRONT || state.game_turn() > FRONT_UNTIL {
        return open;
    }
    let front = (state.owned(1 - seat) | band.and_not(state.owned(seat))) & valid;
    if front.is_empty() {
        open
    } else {
        front
    }
}

#[cfg(test)]
mod tests;
