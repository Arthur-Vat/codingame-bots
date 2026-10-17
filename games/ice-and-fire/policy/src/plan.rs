//! The plan of one turn, built on a scratch state (see the crate's docs).
//!
//! In order: a lethal move or chain of trainings; the moves of the units,
//! nearest to the enemy HQ first; the lethal check again, since the moves
//! extended the territory; buildings (a tower by the HQ under threat, else
//! mines); then trainings with the gold left.

use ice_and_fire_engine::map::NONE;
use ice_and_fire_engine::state::BUILD_COST;
use ice_and_fire_engine::{Bits, Command, Outcome, State, CELLS, HQ, SIZE};

/// Enemy units this close to our HQ (steps on the map) are a threat.
const THREAT_DISTANCE: i64 = 3;
/// Enemy units this close can take the HQ next turn.
const IMMINENT_DISTANCE: i64 = 2;
/// Most trainings in one turn; the gold ends the loop long before.
const MAX_TRAININGS: usize = 16;

/// Plans the turn of the player to act in `state`, appending to `out`.
pub fn plan(state: &State, out: &mut Vec<Command>) {
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
        self.buildings();
        self.trainings();
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
        let Some(chain) = self.cheapest_chain() else {
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
    /// enter it in the present state (RULES.md, "Capture").
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

    // --- Buildings ---

    fn buildings(&mut self) {
        if self.s.outcome().is_some() {
            return;
        }
        if self.enemy_within(THREAT_DISTANCE) {
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
    fn trainings(&mut self) {
        let reserve = if self.turn <= 3 { 0 } else { 5 };
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
