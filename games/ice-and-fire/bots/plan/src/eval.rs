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
//!   cost a side `7 / 4` each (`5 / 4` since E037).
//! - **Mines** (E033): the shared evaluation counts a mine as its income of
//!   4, three times, 12 against the 15 gold it costs, so a climb rarely
//!   gained by building one, while the top 10 of CodinGame build one in a
//!   fifth of their games at every turn from turn 9, once their territory
//!   is large. A mine on an active cell that the enemy cannot take at its
//!   next turn counts one more for every [`MINE_PER_CELLS`] active cells of
//!   its side ([`mine_bonus`]): little on a small territory, 9 at 36 cells.
//! - **Units' moves** (E037): the threatened cells of E023 were those the
//!   enemy's chains of trainings reach, so a unit at the front threatened
//!   nothing, though a move costs no gold. The enemy's units on its active
//!   cells of at least level [`MOVER_LEVEL`] now step first onto every
//!   neighbour they may enter, as `ice_and_fire_tactics::chain::Reach` lets
//!   units do, and the chains go on from there; a threatened cell costs
//!   [`THREAT_QUARTERS`] quarters.
//! - **Standing threat** (E042): in v014's CodinGame games the top 10 could
//!   pay, after 28% of their long chains, for a chain of trainings to our
//!   HQ at their next turn (we after 4% of ours), so that punishing their
//!   chain would leave our HQ open; the pressure term stops counting once a
//!   side can pay. A side that can pays counts [`standing`] more.
//! - **Moves toward the HQ** (E043): the chain to the enemy HQ behind the
//!   pressure term and [`standing`] left moves out; with
//!   [`MARGIN_MOVER_LEVEL`], a side's units of that level or more on its
//!   active cells step first, at no cost, as in E037's reach.
//! - **Upkeep of strong units** (E036): the shared evaluation counts income
//!   three times, and income is net of upkeep, so a level-3 unit (upkeep 20)
//!   costs 60 there and a level-2 unit (upkeep 4) 12, beyond their training
//!   cost: a level-3 training scores about -50 before what it takes. Each
//!   level-2 and level-3 unit on an active cell gives back the upkeep of
//!   the turns beyond [`UPKEEP_TURNS`].

use ice_and_fire_engine::{Bits, State, CELLS, HQ};
use ice_and_fire_tactics::chain::reach_within_moving;
use ice_and_fire_tactics::eval::evaluate_with_movers;

/// The lowest level of a unit whose moves count in [`exposed`] (E037): 1 to
/// 3, or 4 for none (v016). Level 2 won the screening.
pub const MOVER_LEVEL: u8 = 2;

/// What a threatened cell costs its side, in quarters: 7 in E023 (a quarter
/// of a cell's value of 7), 5 since E037, which finds more of them.
pub const THREAT_QUARTERS: i64 = 5;

/// The units of `seat` whose moves count in [`exposed`]: those on its active
/// cells of at least [`MOVER_LEVEL`], all of which may move at its next turn.
fn movers(state: &State, seat: usize) -> Bits {
    movers_from(state, seat, MOVER_LEVEL)
}

/// The units of `seat` on its active cells of at least level `from` (none
/// when `from` is above 3).
fn movers_from(state: &State, seat: usize, from: u8) -> Bits {
    let mut movers = Bits::EMPTY;
    if from > 3 {
        return movers;
    }
    for cell in (state.units(seat) & state.active(seat)).iter() {
        if state.unit_level(cell).is_some_and(|level| level >= from) {
            movers |= Bits::cell(cell);
        }
    }
    movers
}

/// The cells, other than its active cells, that `side` can take at its next
/// turn: those its chains of trainings reach for its gold plus its income,
/// after its [`movers`] step (E037; the pressure term leaves moves out).
pub fn reach(state: &State, side: usize) -> Bits {
    let budget = state.gold(side) + state.income(side);
    reach_within_moving(state, side, movers(state, side), budget)
}

/// The active cells of `seat` that the enemy can take at its next turn
/// ([`reach`]).
fn exposed(state: &State, seat: usize) -> Bits {
    reach(state, 1 - seat) & state.active(seat)
}

/// The number of active cells of `seat` that the enemy can take at its next
/// turn ([`exposed`]).
pub fn threatened(state: &State, seat: usize) -> i64 {
    i64::from(exposed(state, seat).count())
}

/// The active cells of a side for which each of its mines counts one more
/// than its income (E033).
pub const MINE_PER_CELLS: i64 = 4;

/// What each mine of a side with `cells` active cells counts beyond its
/// income.
pub fn mine_bonus(cells: i64) -> i64 {
    cells / MINE_PER_CELLS
}

/// What the mines of `seat` count beyond their income: [`mine_bonus`] for
/// each on its active cells that the enemy cannot take at its next turn.
/// `exposed` is `exposed(state, seat)`.
fn mines(state: &State, seat: usize, exposed: Bits) -> i64 {
    let active = state.active(seat);
    let mines = (state.mines() & active).and_not(exposed);
    mine_bonus(i64::from(active.count())) * i64::from(mines.count())
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
        size: [u8; CELLS],
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
        d.size[v] = size;
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
        size: [0; CELLS],
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

/// The turns of upkeep the evaluation charges for a level-2 and a level-3
/// unit (E036); the shared evaluation charges 3, its weight of income.
pub const UPKEEP_TURNS: [i64; 2] = [3, 1];

/// The upkeep of a level-2 and a level-3 unit (RULES.md, "Income").
const UPKEEP: [i64; 2] = [4, 20];

/// The level-2 and level-3 units of `seat` on active cells.
fn strong(state: &State, seat: usize) -> [i64; 2] {
    let mut count = [0; 2];
    for cell in (state.units(seat) & state.active(seat)).iter() {
        match state.unit_level(cell) {
            Some(2) => count[0] += 1,
            Some(3) => count[1] += 1,
            _ => {}
        }
    }
    count
}

/// What the level-2 and level-3 units of `seat` count beyond the shared
/// evaluation: less [`LEVEL2_DISCOUNT`] each level 2 (E020), plus the
/// upkeep of the turns beyond [`UPKEEP_TURNS`] (E036).
pub fn strong_units(state: &State, seat: usize) -> i64 {
    let [twos, threes] = strong(state, seat);
    let back = |level: usize| (3 - UPKEEP_TURNS[level]) * UPKEEP[level];
    -LEVEL2_DISCOUNT * twos + back(0) * twos + back(1) * threes
}

/// What a side that can pay, at its next turn, for its chain of trainings to
/// the enemy HQ counts more (E042): the enemy must keep defending. 0 for
/// nothing (v018).
pub const HQ_BONUS: i64 = 0;
/// Halves of the gold left after paying for that chain that count too (2,
/// all of it, won the screening), up
/// to [`HQ_MARGIN_CAP`] (E042).
pub const HQ_MARGIN_HALVES: i64 = 2;
/// The most of that gold counted.
pub const HQ_MARGIN_CAP: i64 = 100;

/// What a side whose margin (`ice_and_fire_tactics::eval::margin`) is
/// `margin` counts for its standing threat on the enemy HQ (E042).
pub fn standing(margin: i64) -> i64 {
    if margin < 0 {
        return 0;
    }
    HQ_BONUS + HQ_MARGIN_HALVES * margin.min(HQ_MARGIN_CAP) / 2
}

/// The lowest level of a unit whose moves count in the chain to the enemy
/// HQ of the pressure term and of [`standing`] (E043): 1 to 3, or 4 for
/// none (v019). Level 2 won the screening.
pub const MARGIN_MOVER_LEVEL: u8 = 2;

/// The lead in active cells at or below which a side's pressure term counts
/// only [`BEHIND_GOLD_QUARTERS`] quarters of its gold (E049): `None` for
/// never (v022).
pub const BEHIND_CELLS: Option<i64> = Some(-3);

/// The quarters of its gold that the pressure term of a side behind by
/// [`BEHIND_CELLS`] counts (E049).
pub const BEHIND_GOLD_QUARTERS: i64 = 2;

/// The lowest value of the shared pressure term
/// (`ice_and_fire_tactics::eval`).
const MAX_PRESSURE: i64 = 200;

/// The lead of `seat` in active cells: its active cells less the enemy's.
fn lead(state: &State, seat: usize) -> i64 {
    i64::from(state.active(seat).count()) - i64::from(state.active(1 - seat).count())
}

/// What the pressure term of `seat`, whose margin to the enemy HQ is
/// `margin`, loses when it counts only [`BEHIND_GOLD_QUARTERS`] of its gold
/// (E049): 0 unless `seat` is behind by [`BEHIND_CELLS`].
fn behind_gold(state: &State, seat: usize, margin: i64) -> i64 {
    match BEHIND_CELLS {
        Some(cells) if margin != i64::MIN && lead(state, seat) <= cells => {
            let cut = (4 - BEHIND_GOLD_QUARTERS) * state.gold(seat) / 4;
            (margin - cut).clamp(-MAX_PRESSURE, 0) - margin.clamp(-MAX_PRESSURE, 0)
        }
        _ => 0,
    }
}

/// The score of `state` for `seat`, higher is better for it.
pub fn eval(state: &State, seat: usize) -> i64 {
    let movers = [
        movers_from(state, seat, MARGIN_MOVER_LEVEL),
        movers_from(state, 1 - seat, MARGIN_MOVER_LEVEL),
    ];
    let (score, margins) = evaluate_with_movers(state, seat, movers);
    let score = score + standing(margins[0]) - standing(margins[1]);
    if state.outcome().is_some() {
        return score;
    }
    #[cfg(feature = "analysis")]
    let score = score - crate::analysis::removed(state, seat);
    let ours = exposed(state, seat);
    let theirs = exposed(state, 1 - seat);
    let threats = i64::from(theirs.count()) - i64::from(ours.count());
    score + behind_gold(state, seat, margins[0])
        - behind_gold(state, 1 - seat, margins[1])
        - 2 * debt(state, seat)
        + 2 * debt(state, 1 - seat)
        + shape(state, seat)
        + strong_units(state, seat)
        - strong_units(state, 1 - seat)
        + THREAT_QUARTERS * threats / 4
        + mines(state, seat, ours)
        - mines(state, 1 - seat, theirs)
}
