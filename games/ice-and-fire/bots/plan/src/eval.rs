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
//! - **Tower wall** (E068): in its CodinGame games against us, zasmu guards
//!   75 to 85% of its frontier cells with towers (the other top players and
//!   we about 50%) and keeps its upkeep low. Each active cell of a side next
//!   to an enemy cell that is one of its active towers or next to one counts
//!   [`WALL`] more ([`wall`]).
//! - **The opening's goal** (E073): the opening chooses its plans by the
//!   share of the contested band they claim (cells within [`GOAL_BAND`]
//!   rings of the frontline, `ice_and_fire_opening::solo::rings`), and the
//!   search takes over at game turn 7 without it. Until game turn
//!   [`GOAL_UNTIL`], band cells near a side's land, held by it, guarded by
//!   its towers, and the middle chokes it holds count for it ([`goal`]).
//! - **The front** (E083): the race to the band cells in which units count,
//!   the goal cells of each front, and units holding the front line
//!   (`crate::front`).

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
pub(crate) fn movers_from(state: &State, seat: usize, from: u8) -> Bits {
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
pub(crate) fn exposed(state: &State, seat: usize) -> Bits {
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
pub(crate) fn mines(state: &State, seat: usize, exposed: Bits) -> i64 {
    let active = state.active(seat);
    let mines = (state.mines() & active).and_not(exposed);
    let cells = i64::from(active.count());
    let bonus = if state.game_turn() <= EARLY_MINE_TURN {
        cells / EARLY_MINE_PER_CELLS + EARLY_MINE_BONUS
    } else {
        mine_bonus(cells)
    };
    bonus * i64::from(mines.count())
}

/// The last game turn on which a safe mine counts [`EARLY_MINE_BONUS`] more
/// and one more per [`EARLY_MINE_PER_CELLS`] active cells (E058): 0 for
/// none (v024); 15 won the screening (12 and 18 did less).
pub const EARLY_MINE_TURN: u32 = 15;

/// What a safe mine counts more until [`EARLY_MINE_TURN`] (E058): 6 won the
/// screening (4 and 8 did less).
pub const EARLY_MINE_BONUS: i64 = 6;

/// The active cells for which a safe mine counts one more until
/// [`EARLY_MINE_TURN`] (E058): [`MINE_PER_CELLS`] in v024.
pub const EARLY_MINE_PER_CELLS: i64 = MINE_PER_CELLS;

/// The weight of a joint and of a cell that one capture cuts off.
const SHAPE: i64 = 2;

/// Which joints the shape term counts (E081, after the diagnosis of E080:
/// a joint deep in our land, beside the HQ, made the search train units
/// behind its own lines): 0, all of them (v031); 1, only those at most
/// [`SHAPE_NEAR`] steps from the enemy's active land; 2, each weighted by
/// its distance from it, fully up to [`SHAPE_NEAR`] steps, then less and
/// less to nothing at [`SHAPE_FAR`]. 1 with 3 steps (k3) is
/// ice-and-fire-v032: +75.4 and +67.7 in screening on two seeds.
pub const SHAPE_REACH: u8 = 1;
/// See [`SHAPE_REACH`].
pub const SHAPE_NEAR: u8 = 3;
/// See [`SHAPE_REACH`].
pub const SHAPE_FAR: u8 = 6;

/// The weight in quarters of a joint `d` steps from the enemy's active land
/// by [`SHAPE_REACH`]'s rule `mode`.
pub fn joint_weight(mode: u8, near: u8, far: u8, d: u8) -> u8 {
    match mode {
        0 => 4,
        1 if d <= near => 4,
        1 => 0,
        _ if d <= near => 4,
        _ if d >= far => 0,
        _ => (4 * u32::from(far - d) / u32::from(far - near)) as u8,
    }
}

/// The weakness of `seat`'s shape: its joints plus the most cells that one
/// capture cuts off, as `ice_and_fire_opening::solo::severed` counts them,
/// each joint weighted by [`SHAPE_REACH`]. The same depth-first search,
/// with arrays of bytes and only these two numbers kept, since it runs
/// twice in every evaluation: the cells one capture cuts off do not depend
/// on the order of the search.
fn weakness(state: &State, seat: usize) -> i64 {
    let active = state.active(seat);
    // With SHAPE_REACH, the weakness also depends on the enemy's land.
    let enemy = if SHAPE_REACH == 0 {
        Bits::EMPTY
    } else {
        state.active(1 - seat)
    };
    let [a, b, c] = active.words();
    let [e, f, g] = enemy.words();
    let mut hash = (a
        ^ b.rotate_left(21)
        ^ c.rotate_left(42)
        ^ (e ^ f.rotate_left(13) ^ g.rotate_left(37)).wrapping_mul(0x2545_f491_4f6c_dd1d)
        ^ seat as u64)
        .wrapping_mul(0x9e37_79b9_7f4a_7c15);
    hash ^= hash >> 29;
    let index = (hash as usize) & (WEAKNESS_SLOTS - 1);
    WEAKNESS.with(|cache| {
        let mut cache = cache.borrow_mut();
        let slot = &mut cache[index];
        if slot.0 == active && slot.1 == enemy && slot.2 == seat as u8 + 1 {
            return slot.3;
        }
        let value = if SHAPE_REACH == 0 {
            weakness_of(active, HQ[seat])
        } else {
            let d = ice_and_fire_opening::econ::distances(state, enemy);
            let mut weights = [0u8; CELLS];
            for (w, &steps) in weights.iter_mut().zip(d.iter()) {
                *w = joint_weight(SHAPE_REACH, SHAPE_NEAR, SHAPE_FAR, steps);
            }
            weakness_weighted(active, HQ[seat], &weights)
        };
        *slot = (active, enemy, seat as u8 + 1, value);
        value
    })
}

/// Slots of the cache of [`weakness`], a power of two.
const WEAKNESS_SLOTS: usize = 1 << 12;

thread_local! {
    /// [`weakness`] by territory, the enemy's active land (empty when
    /// [`SHAPE_REACH`] is 0) and seat (the seat plus one, 0 for an empty
    /// slot), so that a territory a search meets again is not searched again:
    /// most actions leave one side's territory as it was. The map is not in
    /// the key: a bot plays one map, but the analysis modes, which read
    /// positions of many games in one thread, could reuse a value of another
    /// map with the same two lands.
    static WEAKNESS: std::cell::RefCell<Vec<(Bits, Bits, u8, i64)>> =
        std::cell::RefCell::new(vec![(Bits::EMPTY, Bits::EMPTY, 0, 0); WEAKNESS_SLOTS]);
}

/// [`weakness`] of the territory `active` whose HQ is `root`, every joint
/// counted.
pub fn weakness_of(active: Bits, root: usize) -> i64 {
    weakness_weighted(active, root, &[4; CELLS])
}

/// [`weakness_of`] with each joint `v` weighted by `weights[v]` quarters:
/// its count and the cells it cuts off, rounded down once at the end.
pub fn weakness_weighted(active: Bits, root: usize, weights: &[u8; CELLS]) -> i64 {
    struct Dfs<'a> {
        weights: &'a [u8; CELLS],
        active: Bits,
        disc: [u8; CELLS],
        low: [u8; CELLS],
        size: [u8; CELLS],
        time: u8,
        joints: i64,
        most: i64,
    }
    fn visit(d: &mut Dfs<'_>, v: usize) -> u8 {
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
            let w = i64::from(d.weights[v]);
            d.joints += w;
            d.most = d.most.max(w * i64::from(cut));
        }
        size
    }
    if !active.test(root) {
        return 0;
    }
    let mut d = Dfs {
        weights,
        active,
        disc: [0; CELLS],
        low: [0; CELLS],
        size: [0; CELLS],
        time: 0,
        joints: 0,
        most: 0,
    };
    visit(&mut d, root);
    (d.joints + d.most) / 4
}

/// The shape term of `seat`'s score: [`SHAPE`] times the other side's
/// weakness less its own.
pub fn shape(state: &State, seat: usize) -> i64 {
    SHAPE * (weakness(state, 1 - seat) - weakness(state, seat))
}

/// The training cost of the units of `seat` on active cells, if it cannot
/// pay its upkeep, else 0.
pub(crate) fn debt(state: &State, seat: usize) -> i64 {
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

/// What each step of a unit's distance to the nearest cell that is not its
/// side's costs that side, beyond the first (E060): 0 for nothing (v025).
/// A walk through our own land toward free or enemy cells then raises the
/// score at each step, as the top bots' units walk for several turns; 2 won
/// the screening (1, 3 and 4 did a little less).
pub const IDLE_STEP: i64 = 2;

/// The farthest distance counted (E060).
pub const IDLE_CAP: i64 = 4;

/// The highest level of a unit counted (E060).
pub const IDLE_LEVEL: u8 = 3;

/// What the units of `seat` on its active cells cost it for standing away
/// from the cells that are not its own (E060).
pub(crate) fn idle(state: &State, seat: usize) -> i64 {
    if IDLE_STEP == 0 {
        return 0;
    }
    let valid = state.map().valid;
    let mut units = Bits::EMPTY;
    for cell in (state.units(seat) & state.active(seat)).iter() {
        if state
            .unit_level(cell)
            .is_some_and(|level| level <= IDLE_LEVEL)
        {
            units |= Bits::cell(cell);
        }
    }
    let mut within = crate::front::idle_targets(state, seat, goal_zones(state).0);
    let mut counted = Bits::EMPTY;
    let mut steps = 0;
    for distance in 1..=IDLE_CAP {
        within = (within | within.neighbours()) & valid;
        let here = (units & within).and_not(counted);
        steps += (distance - 1) * i64::from(here.count());
        counted |= here;
    }
    steps += IDLE_CAP * i64::from(units.and_not(counted).count());
    IDLE_STEP * steps
}

/// What each frontier cell of a side guarded by one of its towers counts
/// (E068): 0 for nothing (v027).
pub const WALL: i64 = 8;

/// The active cells of `seat` next to an enemy cell (its frontier), and
/// those of them that are an active tower of `seat`'s or next to one.
pub fn frontier(state: &State, seat: usize) -> (Bits, Bits) {
    let active = state.active(seat);
    let front = active & state.owned(1 - seat).neighbours();
    let towers = state.towers() & active;
    (front, front & (towers | towers.neighbours()))
}

/// What the tower wall of `seat` counts (E068): [`WALL`] for each of its
/// frontier cells guarded by one of its towers.
pub(crate) fn wall(state: &State, seat: usize) -> i64 {
    if WALL == 0 {
        return 0;
    }
    WALL * i64::from(frontier(state, seat).1.count())
}

/// The opening's goal kept in the search (E073), as weights of its parts,
/// each counted for `seat` less for the enemy, until [`GOAL_UNTIL`]; all 0
/// for nothing (v029). Band cells are those within [`GOAL_BAND`] rings of
/// the frontline.
///
/// - [`GOAL`]: each band cell at least [`GOAL_MARGIN`] steps nearer the
///   side's active land than the enemy's (being near: 0 steps is v029's
///   tie rule, a strict nearness);
/// - [`GOAL_OWNED`]: each band cell the side holds actively;
/// - [`GOAL_GUARD`]: each band cell it holds that is one of its active
///   towers or next to one (a tower stops units below level 3);
/// - [`GOAL_CHOKE`]: each middle choke it holds actively.
pub const GOAL: i64 = 0;
/// See [`GOAL`].
pub const GOAL_MARGIN: u8 = 1;
/// See [`GOAL`].
pub const GOAL_OWNED: i64 = 0;
/// See [`GOAL`].
pub const GOAL_GUARD: i64 = 0;
/// See [`GOAL`].
pub const GOAL_CHOKE: i64 = 0;
/// The last game turn at which the goal counts: 14 since E076 (10 in v030).
pub const GOAL_UNTIL: u32 = 14;
/// The band's width: cells within this many rings of the frontline.
pub const GOAL_BAND: i64 = 2;

/// The land a side's position decides (E076, the owner, 2026-11-20: a
/// centre cell the enemy takes is lost to it and shrinks the land behind
/// it): each cell of the map nearer to a side's active land than to the
/// enemy's (in steps over valid cells, as `econ::distances`) counts this
/// much for it, until game turn [`VORONOI_UNTIL`]; 0 for nothing (v030).
pub const VORONOI: i64 = 0;
/// See [`VORONOI`].
pub const VORONOI_UNTIL: u32 = 14;

/// [`VORONOI`] for `seat`: its weight times the cells nearer to `seat`'s
/// active land less those nearer to the enemy's.
pub fn voronoi(state: &State, seat: usize) -> i64 {
    if VORONOI == 0 || state.game_turn() > VORONOI_UNTIL {
        return 0;
    }
    let ours = ice_and_fire_opening::econ::distances(state, state.active(seat));
    let theirs = ice_and_fire_opening::econ::distances(state, state.active(1 - seat));
    let mut net = 0;
    for c in state.map().valid.iter() {
        match ours[c].cmp(&theirs[c]) {
            std::cmp::Ordering::Less => net += 1,
            std::cmp::Ordering::Greater => net -= 1,
            std::cmp::Ordering::Equal => {}
        }
    }
    VORONOI * net
}

/// The map's band cells and middle chokes, computed once a map.
struct GoalZones {
    valid: Bits,
    band: Bits,
    chokes: Bits,
}

thread_local! {
    static GOAL_ZONES: std::cell::RefCell<Option<GoalZones>> =
        const { std::cell::RefCell::new(None) };
}

/// The band cells and middle chokes of `state`'s map.
pub fn goal_zones(state: &State) -> (Bits, Bits) {
    GOAL_ZONES.with(|cell| {
        let mut zones = cell.borrow_mut();
        let valid = state.map().valid;
        if zones.as_ref().is_none_or(|z| z.valid != valid) {
            let rings = ice_and_fire_opening::solo::rings(state);
            let mut band = Bits::EMPTY;
            for c in valid.iter() {
                if rings[c] <= GOAL_BAND {
                    band.set(c);
                }
            }
            *zones = Some(GoalZones {
                valid,
                band,
                chokes: ice_and_fire_opening::solo::middle_chokes(state),
            });
        }
        zones
            .as_ref()
            .map_or((Bits::EMPTY, Bits::EMPTY), |z| (z.band, z.chokes))
    })
}

/// The cells of `seat` a tower guards: its active towers and their
/// neighbours among its active cells.
fn guarded(state: &State, seat: usize) -> Bits {
    let active = state.active(seat);
    let towers = state.towers() & active;
    (towers | towers.neighbours()) & active
}

/// The opening's goal for `seat` (E073), see [`GOAL`]; 0 after
/// [`GOAL_UNTIL`].
pub fn goal(state: &State, seat: usize) -> i64 {
    if (GOAL == 0 && GOAL_OWNED == 0 && GOAL_GUARD == 0 && GOAL_CHOKE == 0)
        || state.game_turn() > GOAL_UNTIL
    {
        return 0;
    }
    let (band, chokes) = goal_zones(state);
    let mut total = 0;
    if GOAL != 0 {
        let ours = ice_and_fire_opening::econ::distances(state, state.active(seat));
        let theirs = ice_and_fire_opening::econ::distances(state, state.active(1 - seat));
        let mut net = 0;
        for c in band.iter() {
            let (a, b) = (ours[c], theirs[c]);
            if a != u8::MAX && a.saturating_add(GOAL_MARGIN) <= b {
                net += 1;
            } else if b != u8::MAX && b.saturating_add(GOAL_MARGIN) <= a {
                net -= 1;
            }
        }
        total += GOAL * net;
    }
    let count = |bits: Bits| i64::from(bits.count());
    let (mine, theirs) = (state.active(seat), state.active(1 - seat));
    total += GOAL_OWNED * (count(mine & band) - count(theirs & band));
    if GOAL_GUARD != 0 {
        total += GOAL_GUARD
            * (count(guarded(state, seat) & band) - count(guarded(state, 1 - seat) & band));
    }
    total += GOAL_CHOKE * (count(mine & chokes) - count(theirs & chokes));
    total
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
    score - 2 * debt(state, seat)
        + 2 * debt(state, 1 - seat)
        + shape(state, seat)
        + strong_units(state, seat)
        - strong_units(state, 1 - seat)
        - idle(state, seat)
        + idle(state, 1 - seat)
        + THREAT_QUARTERS * threats / 4
        + mines(state, seat, ours)
        - mines(state, 1 - seat, theirs)
        + wall(state, seat)
        - wall(state, 1 - seat)
        + goal(state, seat)
        + voronoi(state, seat)
        + front(state, seat, ours, theirs)
}

/// The front's terms (E083, `crate::front`): the race and goal cells, and
/// the units holding the front of each side.
fn front(state: &State, seat: usize, ours: Bits, theirs: Bits) -> i64 {
    use crate::front::{hold, score, FRONT_UNTIL, GOAL_C, GOAL_E, GOAL_O, HOLD, RACE};
    if (RACE == 0 && GOAL_E == 0 && GOAL_C == 0 && GOAL_O == 0 && HOLD == 0)
        || state.game_turn() > FRONT_UNTIL
    {
        return 0;
    }
    let band = goal_zones(state).0;
    score(state, seat, band) + hold(state, seat, band, ours) - hold(state, 1 - seat, band, theirs)
}
