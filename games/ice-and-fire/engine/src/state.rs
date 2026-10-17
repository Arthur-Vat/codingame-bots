//! The state of a game and the rules that change it (RULES.md).
//!
//! Cells are bitboards ([`Bits`]): who owns them, which are active, where the
//! mines and towers are, where each owner's units and each level's units are.
//! At most one unit stands on a cell, so a game never holds more than 144
//! units and the per-cell array of unit ids cannot overflow. Nothing in
//! `apply`, `end_turn` or the start of a turn allocates.

use std::fmt;
use std::sync::Arc;

use crate::bits::{Bits, CELLS, SIZE};
use crate::command::Command;
use crate::league::League;
use crate::map::{MapData, NONE};

/// Each player plays at most this many turns.
pub const MAX_TURNS: u32 = 100;
/// Gold each player starts with.
const START_GOLD: i64 = 20;
/// Cost of a mine or a tower.
const BUILD_COST: i64 = 15;
/// Gold per turn of a unit of level 1, 2 and 3 on an active cell.
const UPKEEP: [i64; 3] = [1, 4, 20];
/// The headquarters cell of each seat.
pub const HQ: [usize; 2] = [0, CELLS - 1];

/// How a finished game ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The player in this seat won.
    Win(usize),
    Draw,
}

/// What a unit taking a cell changed.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Capture {
    /// The cell was already the player's.
    Same,
    /// The cell was neutral.
    Neutral,
    /// The cell was the opponent's.
    Opponent,
}

/// A game of A Code of Ice and Fire. Seats are absolute: 0 is red (HQ at
/// (0, 0), plays first), 1 is blue.
#[derive(Clone)]
pub struct State {
    pub(crate) map: Arc<MapData>,
    pub(crate) league: League,
    /// Each seat's cells.
    pub(crate) own: [Bits; 2],
    /// Each seat's cells joined to its HQ through its own cells.
    pub(crate) active: [Bits; 2],
    pub(crate) mine: Bits,
    pub(crate) tower: Bits,
    /// Each seat's units.
    pub(crate) units: [Bits; 2],
    /// The units of level 1, 2 and 3, of either seat.
    pub(crate) level: [Bits; 3],
    /// The units that have not moved this turn and were not trained this
    /// turn. Only the units of the player to act are meaningful.
    pub(crate) movable: Bits,
    /// `unit_id[cell]` is the id of the unit on `cell`, if there is one.
    pub(crate) unit_id: [u32; CELLS],
    pub(crate) gold: [i64; 2],
    pub(crate) income: [i64; 2],
    pub(crate) to_act: usize,
    pub(crate) game_turn: u32,
    pub(crate) next_id: u32,
    pub(crate) outcome: Option<Outcome>,
}

/// The cells joined to `hq` through the cells of `own`.
fn flood(own: Bits, hq: usize) -> Bits {
    if !own.test(hq) {
        return Bits::EMPTY;
    }
    let mut region = Bits::cell(hq);
    loop {
        let next = (region | region.neighbours()) & own;
        if next == region {
            return region;
        }
        region = next;
    }
}

impl State {
    /// The start of a game on a map whose HQ cells, (0, 0) and (11, 11), are
    /// not void. Red's start of turn has run, as in the referee's `from_map`.
    /// Mine spots outside the map or on void cells are ignored.
    ///
    /// The caller's map must follow the league, as the referee's
    /// `map::generate` does: Wood 3 and Wood 2 have no mine spots (RULES.md,
    /// "Mine spots"). A debug build panics if `mine_spots` is not empty there.
    pub fn new(void: &[bool; CELLS], mine_spots: &[(usize, usize)], league: League) -> State {
        debug_assert!(
            league.has_mine_spots() || mine_spots.is_empty(),
            "{league:?} has no mine spots"
        );
        let mut void_bits = Bits::EMPTY;
        for (cell, _) in void.iter().enumerate().filter(|&(_, &is_void)| is_void) {
            void_bits.set(cell);
        }
        let mut spots = Bits::EMPTY;
        for &(x, y) in mine_spots {
            if x < SIZE && y < SIZE && !void_bits.test(y * SIZE + x) {
                spots.set(y * SIZE + x);
            }
        }
        let mut own = [Bits::cell(HQ[0]), Bits::cell(HQ[1])];
        let blue_extra = CELLS - 1 - SIZE;
        if !void_bits.test(blue_extra) {
            own[1].set(blue_extra);
        }
        let mut state = State {
            map: MapData::shared(void_bits, spots),
            league,
            own,
            active: [Bits::EMPTY; 2],
            mine: Bits::EMPTY,
            tower: Bits::EMPTY,
            units: [Bits::EMPTY; 2],
            level: [Bits::EMPTY; 3],
            movable: Bits::EMPTY,
            unit_id: [0; CELLS],
            gold: [START_GOLD; 2],
            income: [0; 2],
            to_act: 0,
            game_turn: 1,
            next_id: 1,
            outcome: None,
        };
        state.start_turn(0);
        state
    }

    /// The seat (0 red, 1 blue) whose turn it is.
    pub fn to_act(&self) -> usize {
        self.to_act
    }

    /// The game turn of the player to act, from 1.
    pub fn game_turn(&self) -> u32 {
        self.game_turn
    }

    /// How the game ended, if it has.
    pub fn outcome(&self) -> Option<Outcome> {
        self.outcome
    }

    /// The league whose rules are played.
    pub fn league(&self) -> League {
        self.league
    }

    /// Gold plus the training cost of the units alive.
    pub fn military_power(&self, seat: usize) -> i64 {
        let units = self.units[seat];
        self.gold[seat]
            + 10 * i64::from((units & self.level[0]).count())
            + 20 * i64::from((units & self.level[1]).count())
            + 30 * i64::from((units & self.level[2]).count())
    }

    /// The seat's gold.
    pub fn gold(&self, seat: usize) -> i64 {
        self.gold[seat]
    }

    /// The seat's income, as the turn's input shows it: recomputed after
    /// every command that succeeds and at a start of turn, but not after a
    /// wipe-out (RULES.md, "Start of a player's turn").
    pub fn income(&self, seat: usize) -> i64 {
        self.income[seat]
    }

    /// The id the next trained unit gets.
    pub fn next_unit_id(&self) -> u32 {
        self.next_id
    }

    /// The number of units the seat has.
    pub fn unit_count(&self, seat: usize) -> usize {
        self.units[seat].count() as usize
    }

    /// The fixed map: void cells, mine spots, neighbours and distances.
    pub fn map(&self) -> &MapData {
        &self.map
    }

    /// The seat's cells.
    pub fn owned(&self, seat: usize) -> Bits {
        self.own[seat]
    }

    /// The seat's active cells.
    pub fn active(&self, seat: usize) -> Bits {
        self.active[seat]
    }

    /// The seat's units.
    pub fn units(&self, seat: usize) -> Bits {
        self.units[seat]
    }

    /// The cells with a mine, whoever owns them.
    pub fn mines(&self) -> Bits {
        self.mine
    }

    /// The cells with a tower, whoever owns them.
    pub fn towers(&self) -> Bits {
        self.tower
    }

    /// The level of the unit on `cell`, if there is one.
    pub fn unit_level(&self, cell: usize) -> Option<u8> {
        (cell < CELLS && (self.units[0] | self.units[1]).test(cell)).then(|| self.level_at(cell))
    }

    /// The id of the unit on `cell`, if there is one.
    pub fn unit_id(&self, cell: usize) -> Option<u32> {
        (cell < CELLS && (self.units[0] | self.units[1]).test(cell)).then(|| self.unit_id[cell])
    }

    // --- Turn structure ---

    /// A player's start of turn (RULES.md, "Start of a player's turn").
    fn start_turn(&mut self, seat: usize) {
        // 1. Mark the cells.
        self.active = [flood(self.own[0], HQ[0]), flood(self.own[1], HQ[1])];
        // 2. Units on inactive cells die.
        let dead = self.units[seat].and_not(self.active[seat]);
        self.remove_units(dead);
        // 3. Both incomes.
        self.compute_incomes();
        // 4. The income is paid.
        self.gold[seat] += self.income[seat];
        // 5. Debt wipes out the army; the income shown stays.
        if self.gold[seat] < 0 {
            self.gold[seat] = 0;
            self.remove_units(self.units[seat]);
        }
        // 6. The units may move.
        self.movable |= self.units[seat];
    }

    fn compute_incomes(&mut self) {
        for seat in 0..2 {
            let active = self.active[seat];
            let units = active & self.units[seat];
            let mut income =
                i64::from(active.count()) + 4 * i64::from((active & self.mine).count());
            for (level, upkeep) in UPKEEP.iter().enumerate() {
                income -= upkeep * i64::from((units & self.level[level]).count());
            }
            self.income[seat] = income;
        }
    }

    /// Ends the turn of the player to act: the next start of turn, or the end
    /// of the game after blue's last turn. Does nothing once the game is over.
    pub fn end_turn(&mut self) {
        if self.outcome.is_some() {
            return;
        }
        if self.to_act == 1 {
            if self.game_turn >= MAX_TURNS {
                // Provisional, as in the referee: RULES.md lists equal
                // military power under "Platform facts to check".
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

    /// Plays a whole turn for the player to act: the commands the rules drop
    /// while the line is read, then the others in order, stopping when the
    /// game ends; then [`State::end_turn`].
    pub fn play_turn(&mut self, commands: &[Command]) {
        if self.outcome.is_some() {
            return;
        }
        // A unit trained during the turn does not exist when the line is read.
        let first_new_id = self.next_id;
        for &command in commands {
            if self.dropped_on_read(command, first_new_id) {
                continue;
            }
            self.apply(command);
            if self.outcome.is_some() {
                return;
            }
        }
        self.end_turn();
    }

    /// Whether the rules drop `command` while the answer is read (RULES.md,
    /// "Reading the answer"). Carrying out such a command would fail anyway;
    /// this keeps the two steps apart as the rules state them.
    fn dropped_on_read(&self, command: Command, first_new_id: u32) -> bool {
        match command {
            Command::Wait => true,
            Command::Move { id, x, y } => {
                self.cell_at(x, y).is_none()
                    || id >= first_new_id
                    || self.find_unit(self.to_act, id).is_none()
            }
            Command::Train { level, x, y } => {
                self.cell_at(x, y).is_none() || level == 0 || level > self.league.max_level()
            }
            Command::BuildMine { x, y } | Command::BuildTower { x, y } => {
                self.cell_at(x, y).is_none()
            }
        }
    }

    // --- Board queries ---

    /// The cell at `(x, y)`, if it is on the map and not void.
    #[inline]
    pub(crate) fn cell_at(&self, x: u8, y: u8) -> Option<usize> {
        let (x, y) = (usize::from(x), usize::from(y));
        (x < SIZE && y < SIZE && !self.map.void.test(y * SIZE + x)).then_some(y * SIZE + x)
    }

    /// The cell of the seat's unit `id`.
    pub(crate) fn find_unit(&self, seat: usize, id: u32) -> Option<usize> {
        self.units[seat]
            .iter()
            .find(|&cell| self.unit_id[cell] == id)
    }

    /// The level of the unit on `cell`, which must hold one.
    #[inline]
    pub(crate) fn level_at(&self, cell: usize) -> u8 {
        if self.level[0].test(cell) {
            1
        } else if self.level[1].test(cell) {
            2
        } else {
            3
        }
    }

    /// The cells of `opponent` that a tower protects from units below level 3:
    /// those with a tower, and its active cells next to one of its active
    /// towers (RULES.md, "Protection" and "Capture" rules 1 and 2).
    pub(crate) fn blocked(&self, opponent: usize) -> Bits {
        let towers = self.tower & self.own[opponent];
        let active = self.active[opponent];
        towers | (active & (active & towers).neighbours())
    }

    /// The cells that a unit of `level` of `seat` may enter, by training or
    /// moving (RULES.md, "Capture"): empty cells, and the opponent's cells
    /// whose unit, if any, it beats, unless a tower blocks them from units
    /// below level 3.
    pub(crate) fn enterable(&self, seat: usize, level: u8) -> Bits {
        let opponent = 1 - seat;
        let units = self.units[0] | self.units[1];
        let empty = self.empty_cells();
        let unbeaten = match level {
            1 if self.league == League::Wood3 => self.level[1] | self.level[2],
            1 => units,
            2 => self.level[1] | self.level[2],
            _ => Bits::EMPTY,
        };
        let enterable = empty | self.own[opponent].and_not(unbeaten);
        if level < 3 {
            enterable.and_not(self.blocked(opponent))
        } else {
            enterable
        }
    }

    /// The cells that are the seat's active cells or next to one.
    pub(crate) fn near_active(&self, seat: usize) -> Bits {
        self.active[seat] | self.active[seat].neighbours()
    }

    /// The cells with no unit and no building.
    pub(crate) fn empty_cells(&self) -> Bits {
        let units = self.units[0] | self.units[1];
        let hqs = Bits::cell(HQ[0]) | Bits::cell(HQ[1]);
        self.map.valid.and_not(units | self.mine | self.tower | hqs)
    }

    // --- Actions ---

    /// Carries out one command for the player to act, with every check of
    /// RULES.md, "Carrying out the commands". Returns whether it changed the
    /// game: a command that fails a check changes nothing, `Wait` changes
    /// nothing, and nothing is carried out once the game is over. Cells off the
    /// map or void, and levels out of range, fail like any other check.
    pub fn apply(&mut self, command: Command) -> bool {
        if self.outcome.is_some() {
            return false;
        }
        let seat = self.to_act;
        let capture = match command {
            Command::Wait => return false,
            Command::Train { level, x, y } => self.train(seat, level, x, y),
            Command::Move { id, x, y } => self.move_unit(seat, id, x, y),
            Command::BuildMine { x, y } => self.build(seat, true, x, y),
            Command::BuildTower { x, y } => self.build(seat, false, x, y),
        };
        let Some(capture) = capture else {
            return false;
        };
        match capture {
            Capture::Same => {}
            Capture::Neutral => self.active[seat] = flood(self.own[seat], HQ[seat]),
            Capture::Opponent => {
                self.active = [flood(self.own[0], HQ[0]), flood(self.own[1], HQ[1])];
            }
        }
        self.compute_incomes();
        if !self.own[0].test(HQ[0]) {
            self.outcome = Some(Outcome::Win(1));
        } else if !self.own[1].test(HQ[1]) {
            self.outcome = Some(Outcome::Win(0));
        }
        true
    }

    /// Removes the units on `cells`.
    fn remove_units(&mut self, cells: Bits) {
        self.units[0] = self.units[0].and_not(cells);
        self.units[1] = self.units[1].and_not(cells);
        for level in &mut self.level {
            *level = level.and_not(cells);
        }
        self.movable = self.movable.and_not(cells);
    }

    /// A unit of `seat` takes `cell`: a unit there dies, a mine or tower there
    /// is destroyed, and the cell becomes the seat's. An HQ stays.
    fn take_cell(&mut self, seat: usize, cell: usize) -> Capture {
        self.remove_units(Bits::cell(cell));
        self.mine.clear(cell);
        self.tower.clear(cell);
        let opponent = 1 - seat;
        if self.own[opponent].test(cell) {
            self.own[opponent].clear(cell);
            self.own[seat].set(cell);
            Capture::Opponent
        } else if self.own[seat].test(cell) {
            Capture::Same
        } else {
            self.own[seat].set(cell);
            Capture::Neutral
        }
    }

    fn train(&mut self, seat: usize, level: u8, x: u8, y: u8) -> Option<Capture> {
        if level == 0 || level > self.league.max_level() {
            return None;
        }
        let cell = self.cell_at(x, y)?;
        let cost = 10 * i64::from(level);
        if self.gold[seat] < cost
            || !self.near_active(seat).test(cell)
            || !self.enterable(seat, level).test(cell)
        {
            return None;
        }
        let capture = self.take_cell(seat, cell);
        self.place_unit(seat, level, cell, self.next_id);
        self.next_id = self.next_id.saturating_add(1);
        self.gold[seat] -= cost;
        Some(capture)
    }

    /// Puts a unit on `cell`, which holds none. It cannot move this turn.
    fn place_unit(&mut self, seat: usize, level: u8, cell: usize, id: u32) {
        self.units[seat].set(cell);
        self.level[usize::from(level) - 1].set(cell);
        self.unit_id[cell] = id;
    }

    fn move_unit(&mut self, seat: usize, id: u32, x: u8, y: u8) -> Option<Capture> {
        let target = self.cell_at(x, y)?;
        let from = self.find_unit(seat, id)?;
        if !self.movable.test(from) {
            return None;
        }
        let level = self.level_at(from);
        let enter = self.enterable(seat, level);
        let around = self.map.neighbours[seat][from];
        if around.contains(&(target as u8)) && !enter.test(target) {
            return None;
        }
        // The first neighbour in the order wins a tie: only a strictly
        // smaller distance replaces the best.
        let mut best = self.map.distance(from, target);
        let mut destination = None;
        for &next in around.iter().take_while(|&&next| next != NONE) {
            let next = usize::from(next);
            let distance = self.map.distance(next, target);
            if distance < best && enter.test(next) {
                best = distance;
                destination = Some(next);
            }
        }
        let destination = destination?;
        let capture = self.take_cell(seat, destination);
        self.units[seat].clear(from);
        self.level[usize::from(level) - 1].clear(from);
        self.movable.clear(from);
        self.place_unit(seat, level, destination, id);
        Some(capture)
    }

    fn build(&mut self, seat: usize, mine: bool, x: u8, y: u8) -> Option<Capture> {
        let allowed = if mine {
            self.league.allows_mines()
        } else {
            self.league.allows_towers()
        };
        let cell = self.cell_at(x, y)?;
        let on_spot = self.map.spots.test(cell);
        if !allowed
            || self.gold[seat] < BUILD_COST
            || on_spot != mine
            || !self.own[seat].test(cell)
            || !self.near_active(seat).test(cell)
            || !self.empty_cells().test(cell)
        {
            return None;
        }
        if mine {
            self.mine.set(cell);
        } else {
            self.tower.set(cell);
        }
        self.gold[seat] -= BUILD_COST;
        Some(Capture::Same)
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("league", &self.league)
            .field("to_act", &self.to_act)
            .field("game_turn", &self.game_turn)
            .field("gold", &self.gold)
            .field("income", &self.income)
            .field("outcome", &self.outcome)
            .finish_non_exhaustive()
    }
}
