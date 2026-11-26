//! The actions of the search: what can be added to a plan in a state.
//!
//! Single commands: each move of a movable unit to a neighbouring cell it may
//! enter, each training at the lowest level that may enter a cell (so a
//! higher level only where it kills a unit or passes a tower), each tower on a
//! border cell that guards something (E026, [`guards`]) and each mine on a
//! mine spot. Macro actions: the best few cuts
//! of the enemy's territory, the lethal chain of trainings, if there is
//! one, and with [`STEP_GUARD`] a step into a neutral cell with a guard on
//! the cell left behind (E070). Every action is checked by applying it on a clone, so a list may hold
//! some that the rules refuse.

use ice_and_fire_engine::state::BUILD_COST;
use ice_and_fire_engine::{Bits, Command, State, HQ, SIZE};
use ice_and_fire_tactics::chain::{chain_to, cost_to};
use ice_and_fire_tactics::cut::cuts;

use crate::eval::threatened;

/// Whether a tower that raises the enemy's chain to our HQ, while the enemy
/// can pay for it within two of its turns, is offered too (E026's second
/// variant).
pub const HQ_GUARD: bool = true;

/// The enemy's incomes, beyond its gold, within which its chain to our HQ
/// must be affordable for [`HQ_GUARD`] to offer a tower (E026: 2; E092
/// tries 1, its next turn alone).
pub const HQ_GUARD_TURNS: i64 = 2;

/// Whether a tower guards (E092, the owner: a tower must parry an immediate
/// threat) only when it lowers the active cells the enemy's best cut
/// removes once our turn ends, rather than the cells its chains of
/// trainings could reach ([`threatened`]). Off in v032.
pub const GUARD_BY_CUT: bool = false;

/// How many steps from the enemy's active cells or units a guard tower may
/// stand (E092, the owner: a tower near the border guards best; a cheap
/// stand-in for [`GUARD_BY_CUT`]): 0 for anywhere (v032).
pub const GUARD_NEAR: u8 = 3;

/// Whether [`GUARD_NEAR`] binds the HQ guard too (E092).
pub const GUARD_NEAR_HQ: bool = false;

/// The cells where a guard tower may stand by [`GUARD_NEAR`] for the player
/// to act: those at most that many steps, over non-void cells, from the
/// enemy's active cells or units; every cell when it is 0.
pub(crate) fn guard_zone(state: &State) -> Bits {
    let valid = state.map().valid;
    if GUARD_NEAR == 0 {
        return valid;
    }
    let enemy = 1 - state.to_act();
    let mut zone = state.active(enemy) | state.units(enemy);
    let mut steps = GUARD_NEAR;
    while steps != 0 {
        zone |= zone.neighbours() & valid;
        steps -= 1;
    }
    zone
}

/// The active cells of ours that the enemy's best cut removes once the turn
/// that ends in `state` is over (E092).
pub(crate) fn best_cut(state: &State) -> i64 {
    let mut next = state.clone();
    next.end_turn();
    if next.outcome().is_some() {
        return 0;
    }
    let gold = next.gold(next.to_act());
    cuts(&next, CUT_LENGTH, gold, 3)
        .iter()
        .map(|c| c.cells)
        .max()
        .unwrap_or(0)
        .max(0)
}

/// Whether a tower on `cell` guards something for the player to act (E026):
/// it lowers the cells of ours the enemy can take at its next turn
/// ([`threatened`]), or, with [`HQ_GUARD`], it raises the enemy's chain to
/// our HQ while that chain costs at most the enemy's gold plus two of its
/// incomes. `threats` is `threatened(state, me)`, `reach` the cells the
/// enemy's chains reach behind it, and `chain` the enemy's chain to our HQ,
/// before the tower.
///
/// A tower only makes harder to enter its own cell and the cells next to
/// it (RULES.md, protection), so where none of them is in `reach` it cannot
/// lower `threats`, and the state is not copied unless the HQ guard may
/// apply: the same answer, cheaper.
#[allow(clippy::too_many_arguments)]
fn guards(
    state: &State,
    cell: usize,
    threats: i64,
    reach: Bits,
    chain: i64,
    cut: i64,
    near: Bits,
) -> bool {
    let me = state.to_act();
    let enemy = 1 - me;
    let in_zone = near.test(cell);
    let hq_guard = HQ_GUARD
        && (in_zone || !GUARD_NEAR_HQ)
        && chain <= state.gold(enemy) + HQ_GUARD_TURNS * state.income(enemy).max(0);
    let zone = Bits::cell(cell) | Bits::cell(cell).neighbours();
    if !hq_guard && (!in_zone || (zone & reach).is_empty()) {
        return false;
    }
    let (x, y) = xy(cell);
    let mut after = state.clone();
    if !after.apply(Command::BuildTower { x, y }) {
        return false;
    }
    let lowers = in_zone
        && if GUARD_BY_CUT {
            best_cut(&after) < cut
        } else {
            threatened(&after, me) < threats
        };
    if lowers {
        return true;
    }
    hq_guard && cost_to(&after, enemy, false, HQ[me]) > chain
}

/// Whether a tower is also offered on each free border cell that would
/// guard a frontier cell of ours not yet guarded (E068's tower wall).
pub const WALL_TOWERS: bool = false;

/// Which of our own empty active cells get a training offered (E045): 0,
/// all of them (v021); 1, those next to a cell that is not active for us;
/// 2, none (E045's screening). E044's blockers may still offer a unit on a
/// cell a reply enters.
pub const OWN_CELLS: u8 = 2;

/// The step-and-guard macro action (E070): a unit steps into a neutral cell
/// from a cell of ours the enemy can reach at its next turn, and that cell
/// is guarded at once, so that a unit that holds the front can advance.
/// 0: none (v028); 1: a tower on the cell it leaves; 2: a level-1 training
/// on the cell it leaves.
pub const STEP_GUARD: u8 = 1;

/// The fall-back-and-guard macro action (E076, after zasmu's games): a unit
/// on a cell of ours the enemy can reach at its next turn steps onto a cell
/// that is owned (E070 offers the steps into neutral cells), and the cell
/// it leaves gets a tower. 0: none (v030); 1: steps onto our cells farther
/// from the enemy HQ (back); 2: steps onto our cells next to an enemy cell
/// (along the border, ice-and-fire-v031); 3: steps onto any of our cells
/// or an enemy cell other than its HQ.
pub const FALL_BACK: u8 = 2;

/// The rush from the tip (E076): chains of level-1 trainings into neutral
/// cells, from the trainable neutral cells nearest the enemy HQ, each next
/// cell the neutral neighbour nearest it, of 2 up to this many trainings;
/// 0 for none (v030).
pub const RUSH_LENGTH: usize = 0;

/// The starts of [`RUSH_LENGTH`]'s chains: this many trainable neutral
/// cells, the nearest the enemy HQ first (or, with [`RUSH_GOAL`], this many
/// goals, the nearest first).
pub const RUSH_STARTS: usize = 2;

/// Whether [`RUSH_LENGTH`]'s chains head for goals rather than the enemy
/// HQ (the owner's choice, 2026-11-20): the middle chokes we do not hold,
/// then, once each of them is claimed, the band cells we do not hold
/// (`eval::goal_zones`). A goal is claimed when one of our units is no
/// farther from it than our nearest trainable cell and nearer to it than
/// every enemy unit. Each chain starts at the trainable cell nearest its
/// goal and steps toward it. False: the enemy HQ (rounds 1 to 3).
pub const RUSH_GOAL: bool = false;

/// With [`RUSH_GOAL`], whether goals the enemy wins are left out (the
/// owner's choice, 2026-11-20): those it holds, and those one of its units
/// is nearer to than our nearest trainable cell.
pub const RUSH_SKIP_ENEMY: bool = false;

/// Whether each rush chain is also offered as a leapfrog (the owner's
/// choice, 2026-11-20, after zasmu trains at the tip once its units have
/// moved): a movable unit of ours next to the chain's first cell steps onto
/// it, and the trainings go on from there.
pub const RUSH_LEAP: bool = false;

/// Whether the centre cells the enemy can grab are defended (the owner's
/// choices, 2026-11-20): the middle chokes and band cells it reaches before
/// our nearest trainable cell, and those of ours its chains reach at its
/// next turn (`eval::reach`), the nearest to our land first, at most twice
/// [`RUSH_STARTS`]. For each: the chain of level-1 trainings that takes it
/// this turn, when one of at most [`RUSH_LENGTH`] (3 if 0) trainings
/// reaches it; and a tower on it, if it is ours, or on our cells next to
/// it. Blocking the enemy's path needs no action of its own: every training
/// on a trainable cell is offered.
pub const RUSH_DEFEND: bool = false;

/// Cuts offered as macro actions.
pub const MACRO_CUTS: usize = 4;
/// Longest cut, in trainings: 5 since E014 (4 before).
pub const CUT_LENGTH: usize = 5;

/// One step of a plan: a command, or a short chain of commands that go
/// together.
#[derive(Clone)]
pub enum Action {
    One(Command),
    Many(Vec<Command>),
}

impl Action {
    /// The commands, in the order they are applied.
    pub fn commands(&self) -> &[Command] {
        match self {
            Action::One(command) => std::slice::from_ref(command),
            Action::Many(commands) => commands,
        }
    }
}

pub fn xy(cell: usize) -> (u8, u8) {
    ((cell % SIZE) as u8, (cell / SIZE) as u8)
}

/// The actions for the player to act in `state`, in `out` (cleared first), in
/// a fixed order.
pub fn generate(state: &State, out: &mut Vec<Action>) {
    out.clear();
    if state.outcome().is_some() {
        return;
    }
    let me = state.to_act();
    let max = state.league().max_level();
    let mut enter = [Bits::EMPTY; 4];
    for level in 1..=max {
        enter[usize::from(level)] = state.enterable(me, level);
    }

    // Moves.
    for from in state.movable_units().iter() {
        let Some((id, level)) = state.unit_at(from) else {
            continue;
        };
        let steps = Bits::cell(from).neighbours() & enter[usize::from(level)];
        for to in steps.iter() {
            let (x, y) = xy(to);
            out.push(Action::One(Command::Move { id, x, y }));
        }
    }

    // Trainings, at the lowest level that may enter the cell; on our own
    // cells as OWN_CELLS says (E045).
    let gold = state.gold(me);
    // Our inactive cells stay offered: a training there joins them again.
    let active = state.active(me);
    let near = state.near_active(me);
    let trainable = match OWN_CELLS {
        0 => near,
        1 => {
            let border = state.map().valid.and_not(active).neighbours() & active;
            near.and_not(active) | border
        }
        _ => near.and_not(active),
    };
    for level in 1..=max {
        let cost = 10 * i64::from(level);
        if gold < cost {
            break;
        }
        let mut cells = trainable & enter[usize::from(level)];
        for lower in 1..level {
            cells = cells.and_not(enter[usize::from(lower)]);
        }
        for cell in cells.iter() {
            let (x, y) = xy(cell);
            out.push(Action::One(Command::Train { level, x, y }));
        }
    }

    // Buildings.
    if gold >= BUILD_COST {
        let free = state.owned(me) & near & state.empty_cells();
        if state.league().allows_mines() {
            for cell in (free & state.map().spots).iter() {
                let (x, y) = xy(cell);
                out.push(Action::One(Command::BuildMine { x, y }));
            }
        }
        if state.league().allows_towers() {
            // Towers only on cells next to a cell that is not ours.
            let border = state.map().valid.and_not(state.owned(me)).neighbours();
            let cells = (free & border).and_not(state.map().spots);
            // `threatened(state, me)`, keeping the enemy's reach.
            let reach = crate::eval::reach(state, 1 - me);
            let threats = i64::from((reach & state.active(me)).count());
            let chain = cost_to(state, 1 - me, false, HQ[me]);
            let cut = if GUARD_BY_CUT { best_cut(state) } else { 0 };
            let near = guard_zone(state);
            // Frontier cells of ours not yet guarded by a tower (E068).
            let open = if WALL_TOWERS {
                let (front, guarded) = crate::eval::frontier(state, me);
                front.and_not(guarded)
            } else {
                Bits::EMPTY
            };
            for cell in cells.iter().filter(|&cell| {
                let zone = Bits::cell(cell) | Bits::cell(cell).neighbours();
                !(zone & open).is_empty() || guards(state, cell, threats, reach, chain, cut, near)
            }) {
                let (x, y) = xy(cell);
                out.push(Action::One(Command::BuildTower { x, y }));
            }
        }
    }

    // Step and guard (E070): the step, then the cell left behind guarded.
    let guard_cost = match STEP_GUARD {
        1 => BUILD_COST,
        2 => 10,
        _ => i64::MAX,
    };
    if gold >= guard_cost {
        let reach = crate::eval::reach(state, 1 - me);
        let neutral = state
            .map()
            .valid
            .and_not(state.owned(me) | state.owned(1 - me));
        for from in (state.movable_units() & reach).iter() {
            let Some((id, level)) = state.unit_at(from) else {
                continue;
            };
            let steps = Bits::cell(from).neighbours() & enter[usize::from(level)] & neutral;
            let (fx, fy) = xy(from);
            let guard = if STEP_GUARD == 1 {
                if state.map().spots.test(from) {
                    continue;
                }
                Command::BuildTower { x: fx, y: fy }
            } else {
                Command::Train {
                    level: 1,
                    x: fx,
                    y: fy,
                }
            };
            for to in steps.iter() {
                let (x, y) = xy(to);
                out.push(Action::Many(vec![Command::Move { id, x, y }, guard]));
            }
        }
    }

    // Fall back and guard (E076): a step onto an owned cell, then a tower
    // on the cell left behind.
    if FALL_BACK != 0 && gold >= BUILD_COST && state.league().allows_towers() {
        fall_back(state, &enter, out);
    }

    // Rush from the tip (E076): chains of level-1 trainings toward the
    // enemy HQ.
    if RUSH_LENGTH != 0 && gold >= 20 {
        rush(state, enter[1], gold, out);
    }

    // Defend the centre cells the enemy can grab (E076).
    if RUSH_DEFEND {
        defend(state, enter[1], gold, out);
    }

    // Macro actions: the best cuts, and the lethal chain. A cut of one
    // training is a single training already.
    for cut in cuts(state, CUT_LENGTH, gold, MACRO_CUTS) {
        if cut.commands.len() > 1 {
            out.push(Action::Many(cut.commands));
        }
    }
    if let Some(chain) = chain_to(state, HQ[1 - me], gold) {
        if chain.commands.len() > 1 {
            out.push(Action::Many(chain.commands));
        }
    }
}

/// The fall-back-and-guard actions (E076, [`FALL_BACK`]) of the player to
/// act, with `enter` as [`generate`] has it.
fn fall_back(state: &State, enter: &[Bits; 4], out: &mut Vec<Action>) {
    let me = state.to_act();
    let enemy = 1 - me;
    let reach = crate::eval::reach(state, enemy);
    let ours = state.owned(me);
    let theirs = state.owned(enemy);
    let border = theirs.neighbours();
    let spots = state.map().spots;
    for from in (state.movable_units() & reach & state.active(me)).iter() {
        if spots.test(from) {
            continue;
        }
        let Some((id, level)) = state.unit_at(from) else {
            continue;
        };
        let away = state.map().distance(from, HQ[enemy]);
        let steps = Bits::cell(from).neighbours() & enter[usize::from(level)];
        let (fx, fy) = xy(from);
        for to in steps.iter() {
            let allowed = match FALL_BACK {
                1 => ours.test(to) && state.map().distance(to, HQ[enemy]) > away,
                2 => ours.test(to) && border.test(to),
                // A step onto the enemy HQ wins: it is offered alone.
                _ => ours.test(to) || (theirs.test(to) && to != HQ[enemy]),
            };
            if !allowed {
                continue;
            }
            let (x, y) = xy(to);
            out.push(Action::Many(vec![
                Command::Move { id, x, y },
                Command::BuildTower { x: fx, y: fy },
            ]));
        }
    }
}

/// The rush-from-the-tip actions (E076, [`RUSH_LENGTH`]): `level1` is where
/// a level-1 unit of the player to act may go.
fn rush(state: &State, level1: Bits, gold: i64, out: &mut Vec<Action>) {
    let me = state.to_act();
    let map = state.map();
    let neutral = map.valid.and_not(state.owned(me) | state.owned(1 - me));
    let open = neutral & level1;
    let trainable: Vec<usize> = (state.active(me).neighbours() & open).iter().collect();
    if trainable.is_empty() {
        return;
    }
    // The chains' starts, each with the cell it heads for.
    let nearest = |target: usize| {
        trainable
            .iter()
            .copied()
            .min_by_key(|&cell| (map.distance(cell, target), cell))
    };
    let mut chains: Vec<(usize, usize)> = Vec::new();
    if RUSH_GOAL {
        for goal in rush_goals(state, &trainable) {
            if let Some(start) = nearest(goal) {
                chains.push((start, goal));
            }
        }
    } else {
        let target = HQ[1 - me];
        let mut starts = trainable.clone();
        starts.sort_by_key(|&cell| (map.distance(cell, target), cell));
        starts.truncate(RUSH_STARTS);
        chains.extend(starts.into_iter().map(|start| (start, target)));
    }
    let most = usize::try_from(gold / 10).unwrap_or(0).min(RUSH_LENGTH);
    for (start, target) in chains {
        let mut chain = vec![start];
        let mut taken = Bits::cell(start);
        while chain.len() < most {
            let last = chain[chain.len() - 1];
            if last == target {
                break;
            }
            let next = (Bits::cell(last).neighbours() & open)
                .and_not(taken)
                .iter()
                .min_by_key(|&cell| (map.distance(cell, target), cell));
            let Some(next) = next else {
                break;
            };
            chain.push(next);
            taken.set(next);
            let commands = chain
                .iter()
                .map(|&cell| {
                    let (x, y) = xy(cell);
                    Command::Train { level: 1, x, y }
                })
                .collect();
            out.push(Action::Many(commands));
        }
        if RUSH_LEAP {
            leapfrogs(state, &chain, out);
        }
    }
}

/// The leapfrogs of `chain` ([`RUSH_LEAP`]): for each movable unit of ours
/// next to the chain's first cell that may enter it, the step onto it, then
/// the trainings of the chain's next cells, one more each time.
fn leapfrogs(state: &State, chain: &[usize], out: &mut Vec<Action>) {
    let me = state.to_act();
    let start = chain[0];
    let movers = state.movable_units() & state.units(me) & Bits::cell(start).neighbours();
    for from in movers.iter() {
        let Some((id, level)) = state.unit_at(from) else {
            continue;
        };
        if !state.enterable(me, level).test(start) {
            continue;
        }
        let (x, y) = xy(start);
        let mut commands = vec![Command::Move { id, x, y }];
        for &cell in &chain[1..] {
            let (x, y) = xy(cell);
            commands.push(Command::Train { level: 1, x, y });
            out.push(Action::Many(commands.clone()));
        }
    }
}

/// The goals of [`RUSH_GOAL`]'s chains, the nearest to `trainable` (our
/// trainable cells) first, at most [`RUSH_STARTS`]: the middle chokes we do
/// not hold and no unit of ours claims, or, when there is none, the band
/// cells alike.
pub fn rush_goals(state: &State, trainable: &[usize]) -> Vec<usize> {
    let me = state.to_act();
    let map = state.map();
    let (band, chokes) = crate::eval::goal_zones(state);
    let ours: Vec<usize> = state.units(me).iter().collect();
    let theirs: Vec<usize> = state.units(1 - me).iter().collect();
    let from_trainable = |goal: usize| {
        trainable
            .iter()
            .map(|&cell| map.distance(cell, goal))
            .min()
            .unwrap_or(u8::MAX)
    };
    let claimed = |goal: usize| {
        let reach = from_trainable(goal);
        ours.iter().any(|&unit| {
            let d = map.distance(unit, goal);
            d <= reach && theirs.iter().all(|&enemy| d < map.distance(enemy, goal))
        })
    };
    // RUSH_SKIP_ENEMY: the enemy holds it, or one of its units is nearer.
    let lost = |goal: usize| {
        RUSH_SKIP_ENEMY
            && (state.owned(1 - me).test(goal)
                || theirs
                    .iter()
                    .any(|&enemy| map.distance(enemy, goal) < from_trainable(goal)))
    };
    let open = |cells: Bits| -> Vec<usize> {
        let mut goals: Vec<usize> = cells
            .and_not(state.owned(me))
            .iter()
            .filter(|&goal| from_trainable(goal) != u8::MAX && !claimed(goal) && !lost(goal))
            .collect();
        goals.sort_by_key(|&goal| (from_trainable(goal), goal));
        goals
    };
    let mut goals = open(chokes);
    if goals.is_empty() {
        goals = open(band);
    }
    goals.truncate(RUSH_STARTS);
    goals
}

/// The centre cells [`RUSH_DEFEND`] defends: chokes and band cells the
/// enemy reaches first (not ours) or can take at its next turn (ours), the
/// nearest to our active land first.
pub fn contested(state: &State) -> Vec<usize> {
    let me = state.to_act();
    let map = state.map();
    let (band, chokes) = crate::eval::goal_zones(state);
    let active = state.active(me);
    let ours = state.owned(me);
    let reach = crate::eval::reach(state, 1 - me);
    let theirs: Vec<usize> = state.units(1 - me).iter().collect();
    let from_land = |cell: usize| {
        active
            .iter()
            .map(|a| map.distance(a, cell))
            .min()
            .unwrap_or(u8::MAX)
    };
    let mut cells: Vec<usize> = (band | chokes)
        .iter()
        .filter(|&cell| {
            if ours.test(cell) {
                active.test(cell) && reach.test(cell)
            } else {
                !state.owned(1 - me).test(cell)
                    && theirs
                        .iter()
                        .any(|&e| map.distance(e, cell) < from_land(cell))
            }
        })
        .collect();
    cells.sort_by_key(|&cell| (u8::from(!chokes.test(cell)), from_land(cell), cell));
    cells.truncate(2 * RUSH_STARTS);
    cells
}

/// The [`RUSH_DEFEND`] actions: `level1` is where a level-1 unit of the
/// player to act may go.
fn defend(state: &State, level1: Bits, gold: i64, out: &mut Vec<Action>) {
    let me = state.to_act();
    let map = state.map();
    let neutral = map.valid.and_not(state.owned(me) | state.owned(1 - me));
    let open = neutral & level1;
    let active = state.active(me);
    let free = active & state.empty_cells();
    let longest = if RUSH_LENGTH == 0 { 3 } else { RUSH_LENGTH };
    for cell in contested(state) {
        if !state.owned(me).test(cell) {
            // Train onto it first: the shortest chain from our land.
            let start = (active.neighbours() & open)
                .iter()
                .min_by_key(|&c| (map.distance(c, cell), c));
            if let Some(start) = start {
                let mut chain = vec![start];
                let mut taken = Bits::cell(start);
                while chain[chain.len() - 1] != cell && chain.len() < longest {
                    let last = chain[chain.len() - 1];
                    let next = (Bits::cell(last).neighbours() & open)
                        .and_not(taken)
                        .iter()
                        .min_by_key(|&c| (map.distance(c, cell), c));
                    let Some(next) = next else {
                        break;
                    };
                    chain.push(next);
                    taken.set(next);
                }
                let cost = 10 * i64::try_from(chain.len()).unwrap_or(i64::MAX);
                if chain.len() >= 2 && chain[chain.len() - 1] == cell && cost <= gold {
                    let commands = chain
                        .iter()
                        .map(|&c| {
                            let (x, y) = xy(c);
                            Command::Train { level: 1, x, y }
                        })
                        .collect();
                    out.push(Action::Many(commands));
                }
            }
        }
        // A tower on it or behind it.
        if gold >= BUILD_COST && state.league().allows_towers() {
            let spots = (Bits::cell(cell) | Bits::cell(cell).neighbours()) & free;
            for tower in spots.and_not(map.spots).iter() {
                let (x, y) = xy(tower);
                out.push(Action::One(Command::BuildTower { x, y }));
            }
        }
    }
}
