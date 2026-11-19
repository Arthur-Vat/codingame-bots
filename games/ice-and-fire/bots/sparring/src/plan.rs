//! The sparring bot's turn, built on a scratch state in phases. It starts
//! from the banker's turn (`games/ice-and-fire/ladder`), copied here so
//! that the ladder stays as it is, and changes it after the study of
//! zasmu's games (`research/codingame/2026-11-18-zasmu-strategy.md`):
//!
//! 1. Lethal: a unit next to the enemy HQ steps in, or the cheapest chain of
//!    trainings that knows the towers takes it (the banker's).
//! 2. The moves of the units, nearest to the enemy HQ first, then lethal
//!    again (the banker's).
//! 3. Defence: while the enemy could take our HQ next turn, the tower or
//!    training that most raises what the enemy lacks (the banker's), then
//!    the banker's cuts (below).
//! 4. The strike: when our chain to the enemy HQ is affordable next turn,
//!    only towers of the wall that leave it affordable are built, so that
//!    the gold pays for it then (the banker's banking, with the wall; zasmu
//!    keeps such a strike affordable in 71 to 76% of its positions, the
//!    capability tests' `save-1`).
//! 5. The wall, from game turn [`WALL_FIRST`]: towers on our cells at most
//!    [`WALL_STEPS`] from the enemy's land, each where it leaves the fewest of
//!    our active cells within reach of the enemy's gold at its next turn, if
//!    it takes at least [`WALL_MIN`] of them out of reach (zasmu guards 73 to
//!    88% of its frontier with towers by turns 15 to 25, and builds its first
//!    towers before game turn 10 in a fifth of the capability tests' wall
//!    positions). This is the capability tests' wall measure, so the wall
//!    rate does not judge the rule independently.
//! 6. A mine, from game turn [`MINE_FIRST`] to [`MINE_LAST`], on the free
//!    spot farthest from the enemy's land, only if the enemy is at least
//!    [`MINE_SAFE`] steps away; after turn [`MINE_RICH_TURN`] only with less
//!    than [`MINE_RICH_GOLD`] gold (zasmu: first mine at turns 8 to 10, the
//!    safest spot in 98% of its mines, few once it banks).
//! 7. Trainings of level 1 only, on the new cells that gain the most nearest
//!    the enemy HQ (zasmu keeps few level 2 and 3 units; its level 3 units
//!    come with the strike, which the chain of phase 1 trains).
//! 8. A last check: spending that made us exposed is undone (the banker's).
//!
//! Between 3 and 4, the banker's cuts of the enemy's territory (at most 2 a
//! turn, each leaving us no more exposed): without them the bot lost to the
//! banker by 127 Elo over 100 pairs, with them it wins by 56.

use ice_and_fire_engine::map::NONE;
use ice_and_fire_engine::state::BUILD_COST;
use ice_and_fire_engine::{Bits, Command, Outcome, State, HQ, SIZE};
use ice_and_fire_tactics::chain::{chain_to, reach_within_moving, Reach, UNREACHABLE};
use ice_and_fire_tactics::cut::cuts;

/// Most trainings in one turn; the gold ends the loop long before.
const MAX_TRAININGS: usize = 16;
/// Most cuts taken in one turn.
const CUT_ROUNDS: usize = 2;
/// Longest chain of trainings of a cut.
const CUT_LENGTH: usize = 4;
/// Cuts listed by one search.
const CUT_LIMIT: usize = 8;
/// Lowest score of a cut worth taking.
const CUT_SCORE: i64 = 10;
/// Most rounds of defence in one turn.
const DEFENCE_ROUNDS: usize = 3;

/// The first game turn a mine may be built.
pub(crate) const MINE_FIRST: u32 = 7;
/// The last game turn a mine may be built.
pub(crate) const MINE_LAST: u32 = 20;
/// A mine spot is safe when the enemy's land is at least this many steps
/// away.
pub(crate) const MINE_SAFE: usize = 3;
/// After this game turn, a mine only when the gold is below
/// [`MINE_RICH_GOLD`]: more gold is a strike being saved.
pub(crate) const MINE_RICH_TURN: u32 = 14;
/// See [`MINE_RICH_TURN`].
pub(crate) const MINE_RICH_GOLD: i64 = 60;

/// The first game turn a tower of the wall may be built.
pub(crate) const WALL_FIRST: u32 = 5;
/// A tower of the wall stands this many steps or fewer from the enemy's
/// land (zasmu's: 1 to 4).
pub(crate) const WALL_STEPS: usize = 4;
/// A tower of the wall must take at least this many of our active cells out
/// of the enemy's reach.
pub(crate) const WALL_MIN: u32 = 1;
/// Most towers of the wall in one turn.
const WALL_ROUNDS: usize = 3;

/// The margin of a position the enemy cannot attack at all.
const SAFE: i64 = 1_000_000;

/// Plans the turn of the player to act in `state`, in `out` (cleared first).
/// Each command succeeds when applied in order to `state`.
pub fn plan(state: &State, out: &mut Vec<Command>) {
    out.clear();
    let seat = state.to_act();
    let mut planner = Planner {
        s: state.clone(),
        seat,
        enemy: 1 - seat,
        target: HQ[1 - seat],
        home: HQ[seat],
        turn: state.game_turn(),
        out,
    };
    planner.run();
}

/// How far the enemy is from taking the HQ of `seat` on its next turn, in
/// `state` (the end of a turn of `seat`): the gold of the cheapest chain of
/// trainings, from the territory and the steps of its units, minus the gold
/// the enemy then has. At most 0 means it can; [`SAFE`] when it cannot reach
/// the HQ or the game ends first.
pub(crate) fn margin(state: &State, seat: usize) -> i64 {
    let mut next = state.clone();
    next.end_turn();
    margin_after(&next, seat)
}

/// [`margin`] on the state `next` that follows the end of the turn.
fn margin_after(next: &State, seat: usize) -> i64 {
    let enemy = 1 - seat;
    if next.outcome().is_some() {
        return SAFE;
    }
    let cost = Reach::new(next, enemy, true).cost[HQ[seat]];
    if cost == UNREACHABLE {
        return SAFE;
    }
    (cost - next.gold(enemy)).min(SAFE)
}

/// The cells of the map at most `steps` steps from `from` (`from` included),
/// through cells that are not void.
pub(crate) fn within(state: &State, from: Bits, steps: usize) -> Bits {
    let valid = state.map().valid;
    let mut zone = from & valid;
    for _ in 0..steps {
        zone = (zone | zone.neighbours()) & valid;
    }
    zone
}

/// How many active cells of `seat` the enemy could take at its next turn
/// with the gold it will then have, its units stepping first, if `seat`
/// ended its turn in `state` (the capability tests' wall measure).
pub(crate) fn exposure(state: &State, seat: usize) -> u32 {
    let enemy = 1 - seat;
    let mut next = state.clone();
    next.end_turn();
    let reach = reach_within_moving(&next, enemy, next.units(enemy), next.gold(enemy));
    (reach & next.active(seat)).count()
}

struct Planner<'a> {
    /// The scratch state: the turn so far has been played on it.
    s: State,
    seat: usize,
    enemy: usize,
    /// The enemy HQ.
    target: usize,
    /// Our HQ.
    home: usize,
    turn: u32,
    out: &'a mut Vec<Command>,
}

fn xy(cell: usize) -> (u8, u8) {
    ((cell % SIZE) as u8, (cell / SIZE) as u8)
}

fn move_to(id: u32, cell: usize) -> Command {
    let (x, y) = xy(cell);
    Command::Move { id, x, y }
}

fn train_on(level: u8, cell: usize) -> Command {
    let (x, y) = xy(cell);
    Command::Train { level, x, y }
}

impl Planner<'_> {
    fn run(&mut self) {
        if self.lethal() {
            return;
        }
        self.moves();
        if self.lethal() {
            return;
        }
        self.defence();
        let reserve = if self.turn <= 3 { 0 } else { 5 };
        // For the last check: the state and the commands after the defence,
        // and whether the HQ was exposed then.
        let saved = (self.s.clone(), self.out.len(), self.threat(&self.s));
        self.cuts(reserve);
        match self.strike_reserve() {
            // Saving for the strike: only towers of the wall that leave it
            // affordable (a tower costs no upkeep).
            Some(keep) => self.wall(reserve.max(keep)),
            None => {
                self.wall(reserve);
                self.mine();
                self.trainings(reserve);
            }
        }
        let (state, length, exposed) = saved;
        if !exposed && self.threat(&self.s) {
            self.s = state;
            self.out.truncate(length);
        }
    }

    /// Steps between two cells on the map.
    fn dist(&self, from: usize, to: usize) -> i64 {
        i64::from(self.s.map().distance(from, to))
    }

    /// Applies `command` to the scratch state and keeps it if it succeeded.
    fn apply(&mut self, command: Command) -> bool {
        let done = self.s.apply(command);
        if done {
            self.out.push(command);
        }
        done
    }

    /// Whether `trial` is the end of the game with our win.
    fn wins(&self, trial: &State) -> bool {
        trial.outcome() == Some(Outcome::Win(self.seat))
    }

    /// Whether the enemy could take our HQ on its next turn in `state`.
    fn threat(&self, state: &State) -> bool {
        margin(state, self.seat) <= 0
    }

    /// Steps from `cell` to the enemy's land, up to `limit` (`limit` + 1 when
    /// farther).
    fn steps_to_enemy(&self, cell: usize, limit: usize) -> usize {
        let land = self.s.owned(self.enemy);
        (0..=limit)
            .find(|&steps| within(&self.s, land, steps).test(cell))
            .unwrap_or(limit + 1)
    }

    // --- Lethal ---

    /// Plays a move or a chain of trainings that takes the enemy HQ, if there
    /// is one. Returns whether it did.
    fn lethal(&mut self) -> bool {
        if self.s.outcome().is_some() {
            return false;
        }
        // A unit next to the HQ walks in.
        for cell in self.s.movable_units().iter() {
            if self.dist(cell, self.target) != 1 {
                continue;
            }
            let Some((id, _)) = self.s.unit_at(cell) else {
                continue;
            };
            let command = move_to(id, self.target);
            let mut trial = self.s.clone();
            if trial.apply(command) && self.wins(&trial) {
                return self.apply(command);
            }
        }
        // A chain of trainings, each extending the territory.
        let Some(chain) = chain_to(&self.s, self.target, self.s.gold(self.seat)) else {
            return false;
        };
        let mut trial = self.s.clone();
        if !chain.commands.iter().all(|&command| trial.apply(command)) || !self.wins(&trial) {
            return false;
        }
        for command in chain.commands {
            self.apply(command);
        }
        true
    }

    // --- Evaluating a command ---

    /// What the command that led from the scratch state to `after` gained:
    /// territory, enemy territory taken or cut off, enemy units killed, enemy
    /// mines destroyed.
    fn gain(&self, after: &State) -> i64 {
        let (me, them) = (self.seat, self.enemy);
        let before = &self.s;
        let count = |bits: Bits| i64::from(bits.count());
        let own = count(after.owned(me)) - count(before.owned(me));
        let taken = count(before.owned(them)) - count(after.owned(them));
        let lost = count(before.active(them)) - count(after.active(them));
        let killed = before.military_power(them) - after.military_power(them);
        let mines =
            count(before.mines() & before.owned(them)) - count(after.mines() & after.owned(them));
        12 * own + 20 * taken + 8 * (lost - taken).max(0) + 5 * killed + 25 * mines
    }

    /// A small bonus for claiming a mine spot that is not ours yet.
    fn spot_bonus(&self, cell: usize) -> i64 {
        let spot = self.s.map().spots.test(cell);
        if spot && self.s.league().allows_mines() && !self.s.owned(self.seat).test(cell) {
            20
        } else {
            0
        }
    }

    // --- Moves ---

    /// Each unit that may move steps toward the enemy HQ, to the neighbouring
    /// cell with the best gain, or stays when no step is worth it.
    fn moves(&mut self) {
        let mut units: Vec<(i64, usize)> = self
            .s
            .movable_units()
            .iter()
            .map(|cell| (self.dist(cell, self.target), cell))
            .collect();
        units.sort_unstable();
        for (here, cell) in units {
            if self.s.outcome().is_some() {
                return;
            }
            let Some((id, level)) = self.s.unit_at(cell) else {
                continue;
            };
            let enter = self.s.enterable(self.seat, level);
            let around = self.s.map().neighbours[self.seat][cell];
            let mut best: Option<(i64, usize)> = None;
            for &next in around.iter().take_while(|&&next| next != NONE) {
                let next = usize::from(next);
                if !enter.test(next) {
                    continue;
                }
                let mut trial = self.s.clone();
                if !trial.apply(move_to(id, next)) {
                    continue;
                }
                let value = if self.wins(&trial) {
                    100_000
                } else {
                    self.gain(&trial)
                        + 10 * (here - self.dist(next, self.target))
                        + self.spot_bonus(next)
                };
                if best.is_none_or(|(top, _)| value > top) {
                    best = Some((value, next));
                }
            }
            if let Some((value, next)) = best {
                if value > 0 {
                    self.apply(move_to(id, next));
                }
            }
        }
    }

    // --- Defence ---

    /// While the enemy could take our HQ on its next turn (at most
    /// [`DEFENCE_ROUNDS`] rounds), plays the tower or training that leaves it
    /// the largest margin, if that raises the margin.
    fn defence(&mut self) {
        for _ in 0..DEFENCE_ROUNDS {
            if self.s.outcome().is_some() {
                break;
            }
            let mut next = self.s.clone();
            next.end_turn();
            let current = margin_after(&next, self.seat);
            if current > 0 {
                break;
            }
            let Some(command) = self.best_defence(&next, current) else {
                break;
            };
            if !self.apply(command) {
                break;
            }
        }
    }

    /// The best candidate defence that raises the margin above `current`
    /// (the lowest gold spent, then the first found, on a tie); `next` is the
    /// scratch state after the end of the turn.
    fn best_defence(&self, next: &State, current: i64) -> Option<Command> {
        let gold = self.s.gold(self.seat);
        let path = Reach::new(next, self.enemy, true).path(self.home);
        let mut on_path = Bits::EMPTY;
        for &cell in &path {
            on_path.set(cell);
        }
        let mut candidates: Vec<(i64, Command)> = Vec::new();
        // Towers on our empty active cells on the path, next to it or next to
        // our HQ.
        if self.s.league().allows_towers() && gold >= BUILD_COST {
            let zone = on_path | on_path.neighbours() | Bits::cell(self.home).neighbours();
            for cell in (zone & self.s.active(self.seat) & self.s.empty_cells()).iter() {
                let (x, y) = xy(cell);
                candidates.push((BUILD_COST, Command::BuildTower { x, y }));
            }
        }
        // The lowest level that can enter, on the path cells we may train on.
        for cell in (on_path & self.s.near_active(self.seat)).iter() {
            let entry = (1..=self.s.league().max_level())
                .find(|&level| self.s.enterable(self.seat, level).test(cell));
            if let Some(level) = entry {
                let price = 10 * i64::from(level);
                if price <= gold {
                    candidates.push((price, train_on(level, cell)));
                }
            }
        }
        let mut best: Option<(i64, i64, Command)> = None;
        for (price, command) in candidates {
            let mut trial = self.s.clone();
            if !trial.apply(command) {
                continue;
            }
            let value = margin(&trial, self.seat);
            let better = best.is_none_or(|(top, spent, _)| (value, -price) > (top, -spent));
            if better {
                best = Some((value, price, command));
            }
        }
        best.filter(|&(value, _, _)| value > current)
            .map(|(_, _, command)| command)
    }

    // --- Cuts ---

    /// Up to [`CUT_ROUNDS`] times, takes the first cut of the search that is
    /// worth [`CUT_SCORE`] and leaves us no more exposed than before (the
    /// banker's, without its deadline: the bot is never short of time).
    fn cuts(&mut self, reserve: i64) {
        for _ in 0..CUT_ROUNDS {
            if self.s.outcome().is_some() {
                return;
            }
            let budget = self.s.gold(self.seat) - reserve;
            if budget < 10 {
                return;
            }
            let exposed = self.threat(&self.s);
            let mut taken = false;
            for cut in cuts(&self.s, CUT_LENGTH, budget, CUT_LIMIT) {
                if cut.score < CUT_SCORE {
                    continue;
                }
                let mut trial = self.s.clone();
                if !cut.commands.iter().all(|&command| trial.apply(command)) {
                    continue;
                }
                if !exposed && self.threat(&trial) {
                    continue;
                }
                for command in cut.commands {
                    self.apply(command);
                }
                taken = true;
                break;
            }
            if !taken {
                return;
            }
        }
    }

    // --- The strike ---

    /// The gold to keep for the strike, when there is no threat and the chain
    /// to the enemy HQ (our units stepping first) is affordable next turn if
    /// nothing more is spent: its cost less our income. None otherwise.
    fn strike_reserve(&self) -> Option<i64> {
        if self.s.outcome().is_some() || self.threat(&self.s) {
            return None;
        }
        let cost = Reach::new(&self.s, self.seat, true).cost[self.target];
        let income = self.s.income(self.seat);
        (cost != UNREACHABLE && cost <= self.s.gold(self.seat) + income).then_some(cost - income)
    }

    // --- Mines ---

    /// One mine on the free spot of ours farthest from the enemy's land (then
    /// nearest our HQ), if that spot is safe; see the module's phase 5.
    fn mine(&mut self) {
        if self.s.outcome().is_some()
            || !self.s.league().allows_mines()
            || !(MINE_FIRST..=MINE_LAST).contains(&self.turn)
            || (self.turn > MINE_RICH_TURN && self.s.gold(self.seat) >= MINE_RICH_GOLD)
            || self.s.gold(self.seat) < BUILD_COST
        {
            return;
        }
        let free = self.s.map().spots
            & self.s.owned(self.seat)
            & self.s.active(self.seat)
            & self.s.empty_cells();
        let limit = SIZE * 2;
        let best = free
            .iter()
            .map(|cell| (self.steps_to_enemy(cell, limit), cell))
            .filter(|&(steps, _)| steps >= MINE_SAFE)
            .max_by_key(|&(steps, cell)| {
                (steps, -self.dist(cell, self.home), std::cmp::Reverse(cell))
            });
        if let Some((_, cell)) = best {
            let (x, y) = xy(cell);
            self.apply(Command::BuildMine { x, y });
        }
    }

    // --- The wall ---

    /// Towers on our free cells near the enemy's land, each on the cell that
    /// leaves the fewest of our active cells within reach of the enemy's gold
    /// at its next turn; see the module's phase 6.
    fn wall(&mut self, reserve: i64) {
        if !self.s.league().allows_towers() || self.turn < WALL_FIRST {
            return;
        }
        for _ in 0..WALL_ROUNDS {
            if self.s.outcome().is_some() || self.s.gold(self.seat) < BUILD_COST + reserve {
                return;
            }
            let now = exposure(&self.s, self.seat);
            if now < WALL_MIN {
                return;
            }
            let near = within(&self.s, self.s.owned(self.enemy), WALL_STEPS);
            let candidates =
                self.s.active(self.seat) & self.s.empty_cells() & near.and_not(self.s.map().spots);
            let mut best: Option<(u32, usize, usize)> = None;
            for cell in candidates.iter() {
                let (x, y) = xy(cell);
                let mut trial = self.s.clone();
                if !trial.apply(Command::BuildTower { x, y }) {
                    continue;
                }
                let left = exposure(&trial, self.seat);
                let steps = self.steps_to_enemy(cell, WALL_STEPS);
                if best.is_none_or(|top| (left, steps, cell) < top) {
                    best = Some((left, steps, cell));
                }
            }
            let Some((left, _, cell)) = best else {
                return;
            };
            if now - left.min(now) < WALL_MIN {
                return;
            }
            let (x, y) = xy(cell);
            if !self.apply(Command::BuildTower { x, y }) {
                return;
            }
        }
    }

    // --- Trainings ---

    /// Trains level 1 units with the gold left, on the cells next to the
    /// territory that gain the most and are nearest to the enemy HQ, while
    /// the income after upkeep stays at least 0 and a small reserve of gold
    /// stays unspent.
    fn trainings(&mut self, reserve: i64) {
        for _ in 0..MAX_TRAININGS {
            if self.s.outcome().is_some() || self.s.gold(self.seat) < 10 {
                return;
            }
            let options = self.s.near_active(self.seat) & self.s.enterable(self.seat, 1);
            let mut best: Option<(i64, Command)> = None;
            for cell in options.iter() {
                let command = train_on(1, cell);
                let mut trial = self.s.clone();
                if !trial.apply(command) {
                    continue;
                }
                if trial.income(self.seat) < 0 || trial.gold(self.seat) < reserve {
                    continue;
                }
                let gain = self.gain(&trial);
                if gain <= 0 {
                    continue;
                }
                let value = gain + self.spot_bonus(cell) - 6 * self.dist(cell, self.target);
                if best.is_none_or(|(top, _)| value > top) {
                    best = Some((value, command));
                }
            }
            let Some((_, command)) = best else {
                return;
            };
            if !self.apply(command) {
                return;
            }
        }
    }
}
