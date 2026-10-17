//! Reference implementation of CodinGame's A Code of Ice and Fire rules, as
//! described in `games/ice-and-fire/RULES.md`.
//!
//! It is written to be obviously correct rather than fast: the arena uses it
//! to judge matches, and faster engines used inside bots are checked against
//! it. Maps come from our own generator, [`map::generate`] (ADR 0039).

use std::collections::VecDeque;
use std::time::Duration;

use cg_arena::referee::{Answer, GameSetup, InvalidAnswer, Outcome, Referee, TimeLimits};

pub mod map;

use map::{Map, CELLS};

/// The map is `SIZE` × `SIZE` cells.
pub const SIZE: usize = 12;

/// Each player plays at most this many turns.
const MAX_TURNS: u32 = 100;
/// Gold each player starts with.
const START_GOLD: i64 = 20;
/// Cost of a mine or a tower.
const BUILD_COST: i64 = 15;
/// The longest part of an answer quoted in an error message.
const QUOTE_LIMIT: usize = 40;
/// Distance of cells with no path between them.
const UNREACHABLE: u16 = u16::MAX;

/// Neighbour order for red: up, right, down, left (up is y - 1).
const RED_ORDER: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
/// Neighbour order for blue: down, left, up, right.
const BLUE_ORDER: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

/// The league whose rules are played (RULES.md, "Leagues").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum League {
    Wood3,
    Wood2,
    Wood1,
    #[default]
    Bronze,
}

impl League {
    /// The highest unit level that can be trained.
    pub fn max_level(self) -> u8 {
        match self {
            League::Wood3 => 1,
            _ => 3,
        }
    }

    /// Whether mines can be built.
    pub fn allows_mines(self) -> bool {
        matches!(self, League::Wood1 | League::Bronze)
    }

    /// Whether towers can be built.
    pub fn allows_towers(self) -> bool {
        self == League::Bronze
    }

    /// Whether the map has mine spots.
    pub fn has_mine_spots(self) -> bool {
        matches!(self, League::Wood1 | League::Bronze)
    }

    /// Whether a unit of level `attacker` beats a unit of level `defender`.
    fn beats(self, attacker: u8, defender: u8) -> bool {
        attacker == 3
            || (attacker == 2 && defender == 1)
            || (attacker == 1 && defender == 1 && self == League::Wood3)
    }
}

/// What a cell can hold besides a unit. The numbers are the input's types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Building {
    Hq = 0,
    Mine = 1,
    Tower = 2,
}

/// A unit. It stands on a cell its owner owns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Unit {
    id: u32,
    owner: usize,
    level: u8,
    cell: usize,
    /// False for a unit trained or moved this turn.
    can_move: bool,
}

impl Unit {
    /// Gold to train the unit, and its share of military power.
    fn cost(&self) -> i64 {
        training_cost(self.level)
    }

    /// Gold per turn while on an active cell.
    fn upkeep(&self) -> i64 {
        match self.level {
            1 => 1,
            2 => 4,
            _ => 20,
        }
    }
}

fn training_cost(level: u8) -> i64 {
    10 * i64::from(level)
}

/// A command that is carried out, once the answer has been read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    Move { id: u32, cell: usize },
    Train { level: u8, cell: usize },
    Build { building: Building, cell: usize },
}

/// A recognised part of an answer, before the rules drop any.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Command {
    Wait,
    Msg,
    Move { id: u32, x: u32, y: u32 },
    Train { level: u32, x: u32, y: u32 },
    Build { building: Building, x: u32, y: u32 },
}

/// A non-negative integer that fits a 32-bit signed integer: ASCII digits only.
fn parse_int(word: &str) -> Option<u32> {
    if word.is_empty() || !word.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    word.parse::<i32>().ok().map(|value| value as u32)
}

/// Reads one trimmed part of an answer; `None` if it is unrecognised.
fn parse_command(part: &str) -> Option<Command> {
    if part == "WAIT" {
        return Some(Command::Wait);
    }
    // The part is trimmed, so a text after `MSG ` is never empty.
    if part.starts_with("MSG ") {
        return Some(Command::Msg);
    }
    let words: Vec<&str> = part.split(' ').collect();
    match words.as_slice() {
        ["MOVE", id, x, y] => Some(Command::Move {
            id: parse_int(id)?,
            x: parse_int(x)?,
            y: parse_int(y)?,
        }),
        ["TRAIN", level, x, y] => Some(Command::Train {
            level: parse_int(level)?,
            x: parse_int(x)?,
            y: parse_int(y)?,
        }),
        ["BUILD", kind @ ("MINE" | "TOWER"), x, y] => Some(Command::Build {
            building: if *kind == "MINE" {
                Building::Mine
            } else {
                Building::Tower
            },
            x: parse_int(x)?,
            y: parse_int(y)?,
        }),
        _ => None,
    }
}

/// Splits an answer on `;` as Java's `String.split` does: when there is a
/// `;`, the empty parts at the end are dropped.
fn split_answer(line: &str) -> Vec<&str> {
    let mut parts: Vec<&str> = line.split(';').collect();
    if parts.len() > 1 {
        while parts.last() == Some(&"") {
            parts.pop();
        }
    }
    parts
}

/// Java's `trim`: removes the characters up to U+0020 at both ends.
fn trim(part: &str) -> &str {
    part.trim_matches(|c: char| c <= ' ')
}

/// The HQ cell of `seat`.
fn hq_cell(seat: usize) -> usize {
    if seat == 0 {
        0
    } else {
        CELLS - 1
    }
}

/// The non-void neighbours of `cell`, in the order of `order`.
fn neighbours(map: &Map, cell: usize, order: &[(i32, i32); 4]) -> Vec<usize> {
    let (x, y) = ((cell % SIZE) as i32, (cell / SIZE) as i32);
    order
        .iter()
        .filter_map(|&(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            if !(0..SIZE as i32).contains(&nx) || !(0..SIZE as i32).contains(&ny) {
                return None;
            }
            let next = ny as usize * SIZE + nx as usize;
            (!map.void[next]).then_some(next)
        })
        .collect()
}

/// The shortest path lengths between all cells of `map` (breadth-first search
/// from each non-void cell), ignoring units, buildings and owners.
fn distances(map: &Map) -> Vec<u16> {
    let mut dist = vec![UNREACHABLE; CELLS * CELLS];
    for from in (0..CELLS).filter(|&cell| !map.void[cell]) {
        let row = &mut dist[from * CELLS..(from + 1) * CELLS];
        row[from] = 0;
        let mut queue = VecDeque::from([from]);
        while let Some(cell) = queue.pop_front() {
            for next in neighbours(map, cell, &RED_ORDER) {
                if row[next] == UNREACHABLE {
                    row[next] = row[cell] + 1;
                    queue.push_back(next);
                }
            }
        }
    }
    dist
}

/// The state of an A Code of Ice and Fire game, with the referee's protocol.
#[derive(Clone, Debug)]
pub struct IceAndFireReferee {
    map: Map,
    league: League,
    /// `dist[from * CELLS + to]`: the shortest path over non-void cells.
    dist: Vec<u16>,
    owner: [Option<usize>; CELLS],
    building: [Option<Building>; CELLS],
    /// Whether the cell is connected to its owner's HQ.
    active: [bool; CELLS],
    /// Units in the order they were trained.
    units: Vec<Unit>,
    next_id: u32,
    gold: [i64; 2],
    income: [i64; 2],
    to_act: usize,
    /// 1-based, grows after blue's turn.
    game_turn: u32,
    outcome: Option<Outcome>,
}

impl IceAndFireReferee {
    /// A Bronze game on the map of `setup.seed`.
    pub fn new(setup: &GameSetup) -> Self {
        Self::with_league(setup, League::Bronze)
    }

    /// A game of `league` on the map of `setup.seed`.
    ///
    /// `setup.opening_plies` is ignored: the map varies each game, so the
    /// game's `evaluation.env` sets `OPENING_PLIES=0`.
    pub fn with_league(setup: &GameSetup, league: League) -> Self {
        Self::from_map(map::generate(setup.seed, league), league)
    }

    /// A game on `map`, whose two HQ cells, (0, 0) and (11, 11), are not void.
    /// Red's start of turn has run.
    pub fn from_map(map: Map, league: League) -> Self {
        assert!(
            !map.void[hq_cell(0)] && !map.void[hq_cell(1)],
            "the HQ cells (0, 0) and (11, 11) of the map must not be void"
        );
        let mut owner = [None; CELLS];
        let mut building = [None; CELLS];
        for seat in 0..2 {
            owner[hq_cell(seat)] = Some(seat);
            building[hq_cell(seat)] = Some(Building::Hq);
        }
        let blue_extra = CELLS - 1 - SIZE; // (11, 10)
        if !map.void[blue_extra] {
            owner[blue_extra] = Some(1);
        }
        let dist = distances(&map);
        let mut game = IceAndFireReferee {
            map,
            league,
            dist,
            owner,
            building,
            active: [false; CELLS],
            units: Vec::new(),
            next_id: 1,
            gold: [START_GOLD; 2],
            income: [0; 2],
            to_act: 0,
            game_turn: 1,
            outcome: None,
        };
        game.start_turn(0);
        game
    }

    /// Gold plus the training cost of the units alive.
    pub fn military_power(&self, seat: usize) -> i64 {
        self.gold[seat]
            + self
                .units
                .iter()
                .filter(|unit| unit.owner == seat)
                .map(Unit::cost)
                .sum::<i64>()
    }

    /// The game turn of the player to act, from 1.
    pub fn game_turn(&self) -> u32 {
        self.game_turn
    }

    // --- Turn structure ---

    /// A player's start of turn (RULES.md, "Start of a player's turn").
    fn start_turn(&mut self, seat: usize) {
        // 1. Mark the cells.
        self.compute_active();
        // 2. Units on inactive cells die.
        let active = self.active;
        self.units
            .retain(|unit| unit.owner != seat || active[unit.cell]);
        // 3. Both incomes.
        self.compute_incomes();
        // 4. The income is paid.
        self.gold[seat] += self.income[seat];
        // 5. Debt wipes out the army; the income shown stays.
        if self.gold[seat] < 0 {
            self.gold[seat] = 0;
            self.units.retain(|unit| unit.owner != seat);
        }
        // 6. The units may move.
        for unit in self.units.iter_mut().filter(|unit| unit.owner == seat) {
            unit.can_move = true;
        }
    }

    /// Marks the cells connected to their owner's HQ through the owner's cells.
    fn compute_active(&mut self) {
        self.active = [false; CELLS];
        for seat in 0..2 {
            let hq = hq_cell(seat);
            if self.owner[hq] != Some(seat) {
                continue;
            }
            self.active[hq] = true;
            let mut stack = vec![hq];
            while let Some(cell) = stack.pop() {
                for next in neighbours(&self.map, cell, &RED_ORDER) {
                    if self.owner[next] == Some(seat) && !self.active[next] {
                        self.active[next] = true;
                        stack.push(next);
                    }
                }
            }
        }
    }

    fn compute_incomes(&mut self) {
        for seat in 0..2 {
            let mut income = 0;
            for cell in 0..CELLS {
                if self.owner[cell] == Some(seat) && self.active[cell] {
                    income += 1;
                    if self.building[cell] == Some(Building::Mine) {
                        income += 4;
                    }
                }
            }
            for unit in &self.units {
                if unit.owner == seat && self.active[unit.cell] {
                    income -= unit.upkeep();
                }
            }
            self.income[seat] = income;
        }
    }

    /// Ends the turn of the player to act: the next start of turn, or the end
    /// of the game after blue's last turn.
    fn end_turn(&mut self) {
        if self.to_act == 1 {
            if self.game_turn >= MAX_TURNS {
                // Provisional: RULES.md lists equal military power under
                // "Platform facts to check"; a draw is assumed until the
                // `rules-check` bot shows what CodinGame reports.
                self.outcome = Some(match self.military_power(0).cmp(&self.military_power(1)) {
                    std::cmp::Ordering::Greater => Outcome::Win(0),
                    std::cmp::Ordering::Less => Outcome::Win(1),
                    std::cmp::Ordering::Equal => Outcome::Draw,
                });
                return;
            }
            self.game_turn += 1;
        }
        self.to_act = 1 - self.to_act;
        self.start_turn(self.to_act);
    }

    /// Ends the game when a player's HQ cell belongs to its opponent.
    fn check_hq_capture(&mut self) {
        for seat in 0..2 {
            if self.owner[hq_cell(seat)] != Some(seat) {
                self.outcome = Some(Outcome::Win(1 - seat));
            }
        }
    }

    // --- Board queries ---

    fn distance(&self, from: usize, to: usize) -> u16 {
        self.dist[from * CELLS + to]
    }

    fn unit_at(&self, cell: usize) -> Option<&Unit> {
        self.units.iter().find(|unit| unit.cell == cell)
    }

    /// Whether `cell` is the seat's active cell or next to one.
    fn near_active_own(&self, seat: usize, cell: usize) -> bool {
        let own_active = |c: usize| self.owner[c] == Some(seat) && self.active[c];
        own_active(cell)
            || neighbours(&self.map, cell, &RED_ORDER)
                .into_iter()
                .any(own_active)
    }

    // --- Capture (RULES.md, "Capture" and "Protection") ---

    /// Whether a tower protects `cell` from the opponent of its owner.
    fn is_protected(&self, cell: usize) -> bool {
        let Some(owner) = self.owner[cell] else {
            return false;
        };
        if self.building[cell] == Some(Building::Tower) {
            return true;
        }
        self.active[cell]
            && neighbours(&self.map, cell, &RED_ORDER)
                .into_iter()
                .any(|next| {
                    self.owner[next] == Some(owner)
                        && self.active[next]
                        && self.building[next] == Some(Building::Tower)
                })
    }

    /// Whether a unit of `level` of `seat` may enter `cell`.
    fn can_enter(&self, seat: usize, level: u8, cell: usize) -> bool {
        let opponent_cell = self.owner[cell] == Some(1 - seat);
        if level < 3 && opponent_cell {
            if self.active[cell] && self.is_protected(cell) {
                return false;
            }
            if self.building[cell] == Some(Building::Tower) {
                return false;
            }
        }
        let unit = self.unit_at(cell);
        if unit.is_none() && self.building[cell].is_none() {
            return true;
        }
        self.owner[cell] != Some(seat)
            && unit.is_none_or(|defender| self.league.beats(level, defender.level))
    }

    /// A unit of `seat` takes `cell`: a unit there dies, a mine or tower
    /// there is destroyed, and the cell becomes the seat's. An HQ stays.
    fn take_cell(&mut self, seat: usize, cell: usize) {
        self.units.retain(|unit| unit.cell != cell);
        if self.building[cell] != Some(Building::Hq) {
            self.building[cell] = None;
        }
        self.owner[cell] = Some(seat);
    }

    // --- Actions ---

    /// Carries out `action` for `seat` if its checks pass, then updates the
    /// active cells, the incomes and the HQ check.
    fn apply(&mut self, seat: usize, action: Action) {
        let done = match action {
            Action::Train { level, cell } => self.train(seat, level, cell),
            Action::Move { id, cell } => self.move_unit(seat, id, cell),
            Action::Build { building, cell } => self.build(seat, building, cell),
        };
        if !done {
            return;
        }
        // A building changes no cell's owner, so only the incomes change.
        if !matches!(action, Action::Build { .. }) {
            self.compute_active();
        }
        self.compute_incomes();
        self.check_hq_capture();
    }

    fn train(&mut self, seat: usize, level: u8, cell: usize) -> bool {
        if !self.near_active_own(seat, cell)
            || level > self.league.max_level()
            || self.gold[seat] < training_cost(level)
            || !self.can_enter(seat, level, cell)
        {
            return false;
        }
        self.take_cell(seat, cell);
        self.units.push(Unit {
            id: self.next_id,
            owner: seat,
            level,
            cell,
            can_move: false,
        });
        self.next_id += 1;
        self.gold[seat] -= training_cost(level);
        true
    }

    fn move_unit(&mut self, seat: usize, id: u32, target: usize) -> bool {
        let Some(unit) = self
            .units
            .iter()
            .find(|unit| unit.id == id && unit.owner == seat)
            .copied()
        else {
            return false;
        };
        if !unit.can_move {
            return false;
        }
        let order = if seat == 0 { &RED_ORDER } else { &BLUE_ORDER };
        let around = neighbours(&self.map, unit.cell, order);
        if around.contains(&target) && !self.can_enter(seat, unit.level, target) {
            return false;
        }
        // The first neighbour in the order wins a tie: only a strictly
        // smaller distance replaces the best.
        let mut best = self.distance(unit.cell, target);
        let mut destination = None;
        for next in around {
            let distance = self.distance(next, target);
            if distance < best && self.can_enter(seat, unit.level, next) {
                best = distance;
                destination = Some(next);
            }
        }
        let Some(destination) = destination else {
            return false;
        };
        self.take_cell(seat, destination);
        if let Some(moved) = self.units.iter_mut().find(|u| u.id == id) {
            moved.cell = destination;
            moved.can_move = false;
        }
        true
    }

    fn build(&mut self, seat: usize, building: Building, cell: usize) -> bool {
        let allowed = match building {
            Building::Mine => self.league.allows_mines(),
            _ => self.league.allows_towers(),
        };
        let on_spot = self.map.is_mine_spot(cell % SIZE, cell / SIZE);
        if !self.near_active_own(seat, cell)
            || !allowed
            || self.unit_at(cell).is_some()
            || self.building[cell].is_some()
            || self.owner[cell] != Some(seat)
            || self.gold[seat] < BUILD_COST
            || on_spot != (building == Building::Mine)
        {
            return false;
        }
        self.building[cell] = Some(building);
        self.gold[seat] -= BUILD_COST;
        true
    }

    // --- Reading an answer (RULES.md, "Reading the answer") ---

    /// The actions of `seat`'s answer, or the reason it loses.
    fn read_answer(&self, seat: usize, line: &str) -> Result<Vec<Action>, String> {
        let mut commands = Vec::new();
        for part in split_answer(line) {
            let part = trim(part);
            let Some(command) = parse_command(part) else {
                let quoted: String = part.chars().take(QUOTE_LIMIT).collect();
                let more = if part.chars().count() > QUOTE_LIMIT {
                    "..."
                } else {
                    ""
                };
                return Err(format!("unrecognised command {quoted:?}{more}"));
            };
            commands.push(command);
        }
        Ok(commands
            .into_iter()
            .filter_map(|command| self.to_action(seat, command))
            .collect())
    }

    /// The action of a recognised command, or `None` when the rules drop it
    /// while the line is read.
    fn to_action(&self, seat: usize, command: Command) -> Option<Action> {
        let cell_of = |x: u32, y: u32| {
            let (x, y) = (usize::try_from(x).ok()?, usize::try_from(y).ok()?);
            (x < SIZE && y < SIZE && !self.map.void[y * SIZE + x]).then_some(y * SIZE + x)
        };
        match command {
            Command::Wait | Command::Msg => None,
            Command::Move { id, x, y } => {
                let cell = cell_of(x, y)?;
                self.units
                    .iter()
                    .any(|unit| unit.id == id && unit.owner == seat)
                    .then_some(Action::Move { id, cell })
            }
            Command::Train { level, x, y } => {
                let cell = cell_of(x, y)?;
                let level = u8::try_from(level).ok()?;
                (1..=self.league.max_level())
                    .contains(&level)
                    .then_some(Action::Train { level, cell })
            }
            Command::Build { building, x, y } => {
                let cell = cell_of(x, y)?;
                Some(Action::Build { building, cell })
            }
        }
    }
}

impl Referee for IceAndFireReferee {
    fn time_limits(&self) -> TimeLimits {
        TimeLimits {
            first_answer: Duration::from_millis(1000),
            later_answers: Duration::from_millis(50),
        }
    }

    fn initial_input(&self, _seat: usize) -> String {
        let mut input = format!("{}\n", self.map.mine_spots.len());
        for (x, y) in &self.map.mine_spots {
            input.push_str(&format!("{x} {y}\n"));
        }
        input
    }

    fn players_to_act(&self) -> Vec<usize> {
        if self.outcome.is_some() {
            Vec::new()
        } else {
            vec![self.to_act]
        }
    }

    fn turn_input(&self, seat: usize) -> String {
        let other = 1 - seat;
        let mut input = format!(
            "{}\n{}\n{}\n{}\n",
            self.gold[seat], self.income[seat], self.gold[other], self.income[other]
        );
        for y in 0..SIZE {
            for x in 0..SIZE {
                let cell = y * SIZE + x;
                input.push(match (self.map.void[cell], self.owner[cell]) {
                    (true, _) => '#',
                    (false, None) => '.',
                    (false, Some(owner)) => match (owner == seat, self.active[cell]) {
                        (true, true) => 'O',
                        (true, false) => 'o',
                        (false, true) => 'X',
                        (false, false) => 'x',
                    },
                });
            }
            input.push('\n');
        }

        // Own buildings first, then by type, x and y.
        let mut buildings: Vec<(usize, u8, usize, usize)> = (0..CELLS)
            .filter_map(|cell| {
                let building = self.building[cell]?;
                let owner = self.owner[cell]?;
                Some((
                    usize::from(owner != seat),
                    building as u8,
                    cell % SIZE,
                    cell / SIZE,
                ))
            })
            .collect();
        buildings.sort_unstable();
        input.push_str(&format!("{}\n", buildings.len()));
        for (owner, kind, x, y) in buildings {
            input.push_str(&format!("{owner} {kind} {x} {y}\n"));
        }

        // Own units first, then by id.
        let mut units: Vec<&Unit> = self.units.iter().collect();
        units.sort_by_key(|unit| (usize::from(unit.owner != seat), unit.id));
        input.push_str(&format!("{}\n", units.len()));
        for unit in units {
            input.push_str(&format!(
                "{} {} {} {} {}\n",
                usize::from(unit.owner != seat),
                unit.id,
                unit.level,
                unit.cell % SIZE,
                unit.cell / SIZE
            ));
        }
        input
    }

    fn play(&mut self, answers: &[Answer]) -> Result<(), InvalidAnswer> {
        let answer = answers
            .first()
            .expect("the arena sends one answer per turn");
        let invalid = |reason: String| InvalidAnswer {
            seat: answer.seat,
            reason,
        };
        if self.outcome.is_some() {
            return Err(invalid("the game is over".to_string()));
        }
        if answer.seat != self.to_act {
            return Err(invalid(format!("it is seat {}'s turn", self.to_act)));
        }
        let line = answer.lines.first().map_or("", String::as_str);
        let actions = self.read_answer(answer.seat, line).map_err(invalid)?;
        for action in actions {
            self.apply(answer.seat, action);
            if self.outcome.is_some() {
                return Ok(());
            }
        }
        self.end_turn();
        Ok(())
    }

    fn outcome(&self) -> Option<Outcome> {
        self.outcome
    }
}

#[cfg(test)]
mod tests;
