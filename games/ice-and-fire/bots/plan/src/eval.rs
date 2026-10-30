//! The bot's evaluation of a state, from the point of view of `seat`.
//!
//! It starts as the shared static evaluation
//! (`ice_and_fire_tactics::eval::evaluate`: what each side has, the threat to
//! the HQ and how far each side is from affording the chain to the other's
//! HQ). Terms of the bot's own go here, each documented:
//!
//! - **Debt** (E003): a side whose gold plus income is below 0 loses all its
//!   units at its next start of turn (RULES.md), so twice the training cost
//!   of its units on active cells is taken from its score: once for the
//!   units the shared evaluation counts, once for the loss.
//! - **Shape** (E010): a side's territory is weak where one capture cuts
//!   cells off from its HQ. For each side, [`SHAPE`] times the number of its
//!   joints (active cells whose loss cuts others off) and the most cells
//!   one loss cuts off (`ice_and_fire_opening::solo::severed`) is taken from
//!   its score, as the opening's plans do.
//! - **Level-2 units** (E020): a level-2 unit costs 4 gold of upkeep a turn
//!   against 1 for a level-1 one, and in v008's games a side with more of
//!   them than the score said won less often: each counts [`LEVEL2_DISCOUNT`]
//!   less than its training cost, 5 in all.
//! - **Threatened cells** (E023): in v009's games a cut the enemy could
//!   first afford with one more income was made within one turn about a
//!   quarter of the time, so a cell it can take at its next turn counts 0.75
//!   of a cell's value (4 as a cell, 3 as its income): [`threatened`] cells
//!   cost a side `7 / 4` each.

use ice_and_fire_engine::{Bits, State, CELLS, HQ};
use ice_and_fire_tactics::chain::reach_within;
use ice_and_fire_tactics::eval::evaluate;

/// The active cells of `seat` that the enemy can take at its next turn:
/// those its chains of trainings reach for its gold plus its income (moves
/// left out, as in the pressure term).
pub fn threatened(state: &State, seat: usize) -> i64 {
    let enemy = 1 - seat;
    let budget = state.gold(enemy) + state.income(enemy);
    i64::from((reach_within(state, enemy, false, budget) & state.active(seat)).count())
}

/// The weight of a joint and of a cell that one capture cuts off.
const SHAPE: i64 = 2;

/// The weakness of `seat`'s shape: its joints plus the most cells that one
/// capture cuts off, as `ice_and_fire_opening::solo::severed` counts them.
/// The same depth-first search, with arrays of bytes and only these two
/// numbers kept, since it runs twice in every evaluation: the cells one
/// capture cuts off do not depend on the order of the search.
fn weakness(state: &State, seat: usize) -> i64 {
    let active = state.active(seat);
    let [a, b, c] = active.words();
    let mut hash = (a ^ b.rotate_left(21) ^ c.rotate_left(42) ^ seat as u64)
        .wrapping_mul(0x9e37_79b9_7f4a_7c15);
    hash ^= hash >> 29;
    let index = (hash as usize) & (WEAKNESS_SLOTS - 1);
    WEAKNESS.with(|cache| {
        let mut cache = cache.borrow_mut();
        let slot = &mut cache[index];
        if slot.0 == active && slot.1 == seat as u8 + 1 {
            return slot.2;
        }
        let value = weakness_of(active, HQ[seat]);
        *slot = (active, seat as u8 + 1, value);
        value
    })
}

/// Slots of the cache of [`weakness`], a power of two.
const WEAKNESS_SLOTS: usize = 1 << 12;

thread_local! {
    /// [`weakness`] by territory and seat (the seat plus one, 0 for an empty
    /// slot), so that a territory a search meets again is not searched again:
    /// most actions leave one side's territory as it was.
    static WEAKNESS: std::cell::RefCell<Vec<(Bits, u8, i64)>> =
        std::cell::RefCell::new(vec![(Bits::EMPTY, 0, 0); WEAKNESS_SLOTS]);
}

/// [`weakness`] of the territory `active` whose HQ is `root`.
pub fn weakness_of(active: Bits, root: usize) -> i64 {
    struct Dfs {
        active: Bits,
        disc: [u8; CELLS],
        low: [u8; CELLS],
        time: u8,
        joints: i64,
        most: i64,
    }
    fn visit(d: &mut Dfs, v: usize) -> u8 {
        d.time += 1;
        let disc = d.time;
        d.disc[v] = disc;
        let mut low = disc;
        let mut size = 1u8;
        let mut cut = 0u8;
        for w in (Bits::cell(v).neighbours() & d.active).iter() {
            if d.disc[w] == 0 {
                let sub = visit(d, w);
                size += sub;
                low = low.min(d.low[w]);
                if d.low[w] >= disc {
                    cut += sub;
                }
            } else {
                low = low.min(d.disc[w]);
            }
        }
        d.low[v] = low;
        if cut > 0 && d.disc[v] > 1 {
            d.joints += 1;
            d.most = d.most.max(i64::from(cut));
        }
        size
    }
    if !active.test(root) {
        return 0;
    }
    let mut d = Dfs {
        active,
        disc: [0; CELLS],
        low: [0; CELLS],
        time: 0,
        joints: 0,
        most: 0,
    };
    visit(&mut d, root);
    d.joints + d.most
}

/// The shape term of `seat`'s score: [`SHAPE`] times the other side's
/// weakness less its own.
pub fn shape(state: &State, seat: usize) -> i64 {
    SHAPE * (weakness(state, 1 - seat) - weakness(state, seat))
}

/// The training cost of the units of `seat` on active cells, if it cannot
/// pay its upkeep, else 0.
fn debt(state: &State, seat: usize) -> i64 {
    if state.gold(seat) + state.income(seat) >= 0 {
        return 0;
    }
    (state.units(seat) & state.active(seat))
        .iter()
        .filter_map(|cell| state.unit_level(cell))
        .map(|level| 10 * i64::from(level))
        .sum()
}

/// How much less than its training cost a level-2 unit is worth (E020).
pub const LEVEL2_DISCOUNT: i64 = 15;

/// The level-2 units of `seat` on active cells.
fn twos(state: &State, seat: usize) -> i64 {
    (state.units(seat) & state.active(seat))
        .iter()
        .filter(|&cell| state.unit_level(cell) == Some(2))
        .count() as i64
}

/// The score of `state` for `seat`, higher is better for it.
pub fn eval(state: &State, seat: usize) -> i64 {
    let score = evaluate(state, seat);
    if state.outcome().is_some() {
        return score;
    }
    #[cfg(feature = "analysis")]
    let score = score - crate::analysis::removed(state, seat);
    score - 2 * debt(state, seat)
        + 2 * debt(state, 1 - seat)
        + shape(state, seat)
        + LEVEL2_DISCOUNT * (twos(state, 1 - seat) - twos(state, seat))
        + 7 * (threatened(state, 1 - seat) - threatened(state, seat)) / 4
}
