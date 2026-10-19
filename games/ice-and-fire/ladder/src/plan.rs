//! The plan of one turn, built on a scratch state, in phases. Each of the
//! [`Steps`] switches on one change to bronze's turn:
//!
//! 1. Lethal: a unit next to the enemy HQ steps in, or a chain of trainings
//!    takes it: bronze's static chain, or with `tower_chain` the cheapest
//!    chain that knows the towers.
//! 2. The moves of the units, nearest to the enemy HQ first (bronze's), then
//!    lethal again.
//! 3. `defence`: while the enemy could take our HQ next turn, the tower or
//!    training that most raises what the enemy lacks.
//! 4. `cuts`: cuts of the enemy's territory that leave us no more exposed.
//! 5. `banking`: gold is kept when the chain to the enemy HQ is affordable
//!    next turn.
//! 6. Buildings, 7. trainings: bronze's.
//! 8. With `defence`, a last check: spending that made us exposed is undone.
//!
//! The lethal chain, moves, buildings and trainings are those of the bronze
//! policy (`games/ice-and-fire/policy`), copied here so that it stays as it
//! is; with no step on, the turn is bronze's exactly.

use std::time::{Duration, Instant};

use ice_and_fire_engine::map::NONE;
use ice_and_fire_engine::state::BUILD_COST;
use ice_and_fire_engine::{Bits, Command, Outcome, State, CELLS, HQ, SIZE};
use ice_and_fire_tactics::chain::{chain_to, Reach, UNREACHABLE};
use ice_and_fire_tactics::cut::cuts;

use crate::Steps;

/// Enemy units this close to our HQ (steps on the map) are a threat.
const THREAT_DISTANCE: i64 = 3;
/// Enemy units this close can take the HQ next turn.
const IMMINENT_DISTANCE: i64 = 2;
/// Most trainings in one turn; the gold ends the loop long before.
const MAX_TRAININGS: usize = 16;

/// Most rounds of defence in one turn.
const DEFENCE_ROUNDS: usize = 3;
/// Most cuts taken in one turn.
const CUT_ROUNDS: usize = 2;
/// Longest chain of trainings of a cut.
const CUT_LENGTH: usize = 4;
/// Cuts listed by one search.
const CUT_LIMIT: usize = 8;
/// Lowest score of a cut worth taking.
const CUT_SCORE: i64 = 10;
/// The cut search is skipped once the turn has taken this long. A fixed 3 ms
/// that does not read `CG_TIME_SCALE`: the evaluation's limit is 10 ms a turn
/// (time scale 0.2) and a search of cuts takes about 2 ms at most, so the
/// bot keeps the same behaviour at every scale, on CodinGame as in the arena.
const CUT_DEADLINE: Duration = Duration::from_millis(3);

/// The margin of a position the enemy cannot attack at all.
const SAFE: i64 = 1_000_000;

/// Plans the turn of the player to act in `state`, appending to `out`.
pub fn plan(state: &State, steps: Steps, out: &mut Vec<Command>) {
    let seat = state.to_act();
    let mut planner = Planner {
        s: state.clone(),
        seat,
        enemy: 1 - seat,
        target: HQ[1 - seat],
        home: HQ[seat],
        turn: state.game_turn(),
        steps,
        start: Instant::now(),
        out,
    };
    planner.run();
}

/// How far the enemy is from taking the HQ of `seat` on its next turn, in
/// `state` (the end of a turn of `seat`): the gold of the cheapest chain of
/// trainings, from the territory and the steps of its units, minus the gold
/// the enemy then has. At most 0 means it can; [`SAFE`] when it cannot reach
/// the HQ or the game ends first.
pub fn margin(state: &State, seat: usize) -> i64 {
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
    steps: Steps,
    start: Instant,
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
        let defended = self.steps.defence && self.defence();
        let reserve = if self.turn <= 3 { 0 } else { 5 };
        // For the last check: the state and the commands after the defence,
        // and whether the HQ was exposed then.
        let saved = self
            .steps
            .defence
            .then(|| (self.s.clone(), self.out.len(), self.threat(&self.s)));
        if self.steps.cuts {
            self.cuts(reserve);
        }
        if !(self.steps.banking && self.banking()) {
            self.buildings(defended);
            self.trainings(reserve);
        }
        if let Some((state, length, exposed)) = saved {
            if !exposed && self.threat(&self.s) {
                self.s = state;
                self.out.truncate(length);
            }
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
        let chain = if self.steps.tower_chain {
            chain_to(&self.s, self.target, self.s.gold(self.seat)).map(|chain| chain.commands)
        } else {
            self.cheapest_chain()
        };
        let Some(chain) = chain else {
            return false;
        };
        let mut trial = self.s.clone();
        if !chain.iter().all(|&command| trial.apply(command)) || !self.wins(&trial) {
            return false;
        }
        for command in chain {
            self.apply(command);
        }
        true
    }

    /// The cheapest chain of trainings from our active cells to the enemy HQ
    /// that the gold pays for. Each cell costs the cheapest level that may
    /// enter it in the present state (RULES.md, "Capture"). Bronze's.
    fn cheapest_chain(&self) -> Option<Vec<Command>> {
        const FAR: i64 = i64::MAX;
        let max = self.s.league().max_level();
        let enter: Vec<Bits> = (1..=max)
            .map(|level| self.s.enterable(self.seat, level))
            .collect();
        // The cheapest level that may enter `cell`, and its cost.
        let entry = |cell: usize| {
            enter
                .iter()
                .position(|bits| bits.test(cell))
                .map(|index| (index as u8 + 1, 10 * (index as i64 + 1)))
        };
        let active = self.s.active(self.seat);
        let mut cost = [FAR; CELLS];
        let mut before = [usize::MAX; CELLS];
        let mut done = [false; CELLS];
        for cell in active.iter() {
            cost[cell] = 0;
        }
        loop {
            let mut next: Option<usize> = None;
            for cell in 0..CELLS {
                if !done[cell] && cost[cell] < FAR && next.is_none_or(|n| cost[cell] < cost[n]) {
                    next = Some(cell);
                }
            }
            let Some(cell) = next else {
                break;
            };
            done[cell] = true;
            if cell == self.target {
                break;
            }
            for &around in &self.s.map().neighbours[0][cell] {
                if around == NONE {
                    break;
                }
                let around = usize::from(around);
                if done[around] || active.test(around) {
                    continue;
                }
                let Some((_, price)) = entry(around) else {
                    continue;
                };
                if cost[cell] + price < cost[around] {
                    cost[around] = cost[cell] + price;
                    before[around] = cell;
                }
            }
        }
        if cost[self.target] > self.s.gold(self.seat) {
            return None;
        }
        let mut chain = Vec::new();
        let mut cell = self.target;
        while !active.test(cell) {
            let (level, _) = entry(cell)?;
            chain.push(train_on(level, cell));
            cell = before[cell];
        }
        chain.reverse();
        Some(chain)
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

    /// Whether an enemy unit is within `limit` steps of our HQ.
    fn enemy_within(&self, limit: i64) -> bool {
        self.s
            .units(self.enemy)
            .iter()
            .any(|cell| self.dist(cell, self.home) <= limit)
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
    /// the largest margin (the lowest gold spent, then the first found, on a
    /// tie), if that raises the margin. Returns whether a defence was applied.
    fn defence(&mut self) -> bool {
        let mut applied = false;
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
            let Some((_, command)) = self.best_defence(&next, current) else {
                break;
            };
            if !self.apply(command) {
                break;
            }
            applied = true;
        }
        applied
    }

    /// The best candidate defence (its margin, its command) that raises the
    /// margin above `current`; `next` is the scratch state after the end of
    /// the turn.
    fn best_defence(&self, next: &State, current: i64) -> Option<(i64, Command)> {
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
            .map(|(value, _, command)| (value, command))
    }

    // --- Cuts ---

    /// Up to [`CUT_ROUNDS`] times, takes the first cut of the search that is
    /// worth [`CUT_SCORE`] and leaves us no more exposed than before.
    fn cuts(&mut self, reserve: i64) {
        for _ in 0..CUT_ROUNDS {
            if self.s.outcome().is_some() || self.start.elapsed() > CUT_DEADLINE {
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

    // --- Banking ---

    /// Whether to keep the gold: no threat, and the chain to the enemy HQ is
    /// affordable next turn if nothing more is spent.
    fn banking(&self) -> bool {
        if self.threat(&self.s) {
            return false;
        }
        let cost = Reach::new(&self.s, self.seat, false).cost[self.target];
        cost != UNREACHABLE && cost <= self.s.gold(self.seat) + self.s.income(self.seat)
    }

    // --- Buildings ---

    /// A tower by the HQ when the enemy is close and no defence was applied,
    /// else mines.
    fn buildings(&mut self, defended: bool) {
        if self.s.outcome().is_some() {
            return;
        }
        if !defended && self.enemy_within(THREAT_DISTANCE) {
            self.tower_by_home();
        } else {
            self.mines();
        }
    }

    /// One tower on a free cell of ours next to the HQ, which then needs a
    /// level 3 unit to enter, unless there is one already.
    fn tower_by_home(&mut self) {
        if !self.s.league().allows_towers() || self.s.gold(self.seat) < BUILD_COST {
            return;
        }
        let around = self.s.map().neighbours[self.seat][self.home];
        let cells: Vec<usize> = around
            .iter()
            .take_while(|&&cell| cell != NONE)
            .map(|&cell| usize::from(cell))
            .collect();
        let mine = self.s.owned(self.seat);
        if cells
            .iter()
            .any(|&cell| mine.test(cell) && self.s.towers().test(cell))
        {
            return;
        }
        for cell in cells {
            let (x, y) = xy(cell);
            if self.apply(Command::BuildTower { x, y }) {
                return;
            }
        }
    }

    /// Mines on the free mine spots of ours, nearest to the HQ first.
    fn mines(&mut self) {
        if !self.s.league().allows_mines() {
            return;
        }
        while self.s.gold(self.seat) >= BUILD_COST {
            let free = self.s.map().spots
                & self.s.owned(self.seat)
                & self.s.active(self.seat)
                & self.s.empty_cells();
            let Some(cell) = free
                .iter()
                .min_by_key(|&cell| (self.dist(cell, self.home), cell))
            else {
                return;
            };
            let (x, y) = xy(cell);
            if !self.apply(Command::BuildMine { x, y }) {
                return;
            }
        }
    }

    // --- Trainings ---

    /// Trains units with the gold left: on the cells next to the territory
    /// that gain the most and are nearest to the enemy HQ. Level 1 where it
    /// can enter; a higher level only where a lower one cannot (to kill a unit
    /// or pass a tower). The income after upkeep stays at least 0 and a small
    /// reserve of gold stays unspent, except for a kill near our HQ.
    fn trainings(&mut self, reserve: i64) {
        for _ in 0..MAX_TRAININGS {
            if self.s.outcome().is_some() {
                return;
            }
            let gold = self.s.gold(self.seat);
            if gold < 10 {
                return;
            }
            let imminent = self.enemy_within(IMMINENT_DISTANCE);
            let near = self.s.near_active(self.seat);
            let mut best: Option<(i64, Command)> = None;
            // The best own cell to stage a unit on, if nothing gains.
            let mut staging: Option<(i64, Command)> = None;
            let mut below = Bits::EMPTY;
            for level in 1..=self.s.league().max_level() {
                let enter = self.s.enterable(self.seat, level);
                let options = (near & enter).and_not(below);
                below = enter;
                let price = 10 * i64::from(level);
                if price > gold {
                    continue;
                }
                for cell in options.iter() {
                    let command = train_on(level, cell);
                    let mut trial = self.s.clone();
                    if !trial.apply(command) {
                        continue;
                    }
                    let left = trial.gold(self.seat);
                    let income = trial.income(self.seat);
                    let killed =
                        self.s.military_power(self.enemy) - trial.military_power(self.enemy);
                    let affordable = if killed > 0 && imminent {
                        left + income >= 0
                    } else {
                        income >= 0 && (left >= reserve || killed > 0)
                    };
                    if !affordable {
                        continue;
                    }
                    let gain = self.gain(&trial);
                    let value = gain + self.spot_bonus(cell)
                        - 6 * self.dist(cell, self.target)
                        - 10 * (i64::from(level) - 1);
                    if gain > 0 {
                        if best.is_none_or(|(top, _)| value > top) {
                            best = Some((value, command));
                        }
                    } else if income >= 3 && left >= reserve + 10 && level == 1 {
                        // A unit behind the front: only when income is spare.
                        if staging.is_none_or(|(top, _)| value > top) {
                            staging = Some((value, command));
                        }
                    }
                }
            }
            let Some((_, command)) = best.or(staging) else {
                return;
            };
            if !self.apply(command) {
                return;
            }
        }
    }
}
