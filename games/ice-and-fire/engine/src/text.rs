//! CodinGame's text protocol (RULES.md, "Protocol"): the input a player
//! receives, written from a state and read back into one.

use std::fmt::Write;
use std::str::Lines;

use crate::bits::{Bits, CELLS, SIZE};
use crate::league::League;
use crate::map::MapData;
use crate::state::{State, HQ};

/// Building types in the input.
const HQ_TYPE: i64 = 0;
const MINE_TYPE: i64 = 1;
const TOWER_TYPE: i64 = 2;

impl State {
    /// What a player receives once, before its first turn: the number of mine
    /// spots, then one line `x y` per spot, row by row. The same for both
    /// seats.
    pub fn initial_input(&self, _seat: usize) -> String {
        let mut input = String::with_capacity(8 + 6 * self.map.spots.count() as usize);
        let _ = writeln!(input, "{}", self.map.spots.count());
        for cell in self.map.spots.iter() {
            let _ = writeln!(input, "{} {}", cell % SIZE, cell / SIZE);
        }
        input
    }

    /// What `seat` receives at its turn: gold and income of both players, the
    /// map, the buildings and the units, from the seat's point of view.
    pub fn turn_input(&self, seat: usize) -> String {
        let other = 1 - seat;
        let mut input = String::with_capacity(512);
        let _ = writeln!(
            input,
            "{}\n{}\n{}\n{}",
            self.gold[seat], self.income[seat], self.gold[other], self.income[other]
        );
        for y in 0..SIZE {
            for x in 0..SIZE {
                let cell = y * SIZE + x;
                input.push(if self.map.void.test(cell) {
                    '#'
                } else if self.own[seat].test(cell) {
                    if self.active[seat].test(cell) {
                        'O'
                    } else {
                        'o'
                    }
                } else if self.own[other].test(cell) {
                    if self.active[other].test(cell) {
                        'X'
                    } else {
                        'x'
                    }
                } else {
                    '.'
                });
            }
            input.push('\n');
        }

        // Own buildings first, then by type, x and y.
        let mut buildings: Vec<(u8, u8, usize, usize)> = Vec::new();
        for owner in [seat, other] {
            let relative = u8::from(owner != seat);
            let mut push = |kind: i64, cell: usize| {
                buildings.push((relative, kind as u8, cell % SIZE, cell / SIZE));
            };
            if self.own[owner].test(HQ[0]) {
                push(HQ_TYPE, HQ[0]);
            }
            if self.own[owner].test(HQ[1]) {
                push(HQ_TYPE, HQ[1]);
            }
            for cell in (self.mine & self.own[owner]).iter() {
                push(MINE_TYPE, cell);
            }
            for cell in (self.tower & self.own[owner]).iter() {
                push(TOWER_TYPE, cell);
            }
        }
        buildings.sort_unstable();
        let _ = writeln!(input, "{}", buildings.len());
        for (owner, kind, x, y) in buildings {
            let _ = writeln!(input, "{owner} {kind} {x} {y}");
        }

        // Own units first, then by id.
        let mut units: Vec<(u8, u32, u8, usize, usize)> = Vec::new();
        for owner in [seat, other] {
            let relative = u8::from(owner != seat);
            for cell in self.units[owner].iter() {
                let (id, level) = (self.unit_id[cell], self.level_at(cell));
                units.push((relative, id, level, cell % SIZE, cell / SIZE));
            }
        }
        units.sort_unstable();
        let _ = writeln!(input, "{}", units.len());
        for (owner, id, level, x, y) in units {
            let _ = writeln!(input, "{owner} {id} {level} {x} {y}");
        }
        input
    }

    /// Rebuilds the state that a bot sees from what CodinGame sent it: the
    /// initial input and one turn's input. The bot's seat is red if its own HQ
    /// is at (0, 0), else blue; it is the player to act, and all its units may
    /// move. The next unit id is one more than the largest seen (1 if there is
    /// none). The turn does not say which game turn it is, so the caller does.
    ///
    /// Malformed text gives an `Err` that says what is wrong; it never panics.
    pub fn read(
        initial: &str,
        turn: &str,
        league: League,
        game_turn: u32,
    ) -> Result<State, String> {
        let mut spots = Vec::new();
        let mut lines = initial.lines();
        let count = count_line(&mut lines, "mine spot count")?;
        for _ in 0..count {
            let [x, y] = numbers(next_line(&mut lines, "a mine spot")?, "a mine spot")?;
            spots.push((coordinate(x)?, coordinate(y)?));
        }
        end_of_text(lines, "the initial input")?;

        let mut lines = turn.lines();
        let mut header = [0i64; 4];
        for (value, what) in
            header
                .iter_mut()
                .zip(["gold", "income", "opponent's gold", "opponent's income"])
        {
            let [number] = numbers(next_line(&mut lines, what)?, what)?;
            *value = number;
        }

        // The map, with owners relative to the bot: 0 is the bot's.
        let mut void = Bits::EMPTY;
        let mut cells = [Bits::EMPTY; 2];
        let mut active = [Bits::EMPTY; 2];
        for y in 0..SIZE {
            let row = next_line(&mut lines, "a map row")?.trim();
            if row.len() != SIZE {
                return Err(format!("map row {y} is not {SIZE} characters: {row:?}"));
            }
            for (x, symbol) in row.bytes().enumerate() {
                let cell = y * SIZE + x;
                match symbol {
                    b'#' => void.set(cell),
                    b'.' => {}
                    b'O' | b'o' | b'X' | b'x' => {
                        let relative = usize::from(symbol == b'X' || symbol == b'x');
                        cells[relative].set(cell);
                        if symbol.is_ascii_uppercase() {
                            active[relative].set(cell);
                        }
                    }
                    _ => return Err(format!("map row {y} has {:?} at {x}", symbol as char)),
                }
            }
        }

        let building_count = count_line(&mut lines, "building count")?;
        let mut buildings = Vec::new();
        for _ in 0..building_count {
            let numbers = numbers(next_line(&mut lines, "a building")?, "a building")?;
            buildings.push(numbers);
        }
        let unit_count = count_line(&mut lines, "unit count")?;
        let mut units = Vec::new();
        for _ in 0..unit_count {
            let numbers = numbers(next_line(&mut lines, "a unit")?, "a unit")?;
            units.push(numbers);
        }
        end_of_text(lines, "the turn's input")?;

        // The bot's seat comes from its own HQ.
        let own_hq = buildings
            .iter()
            .find(|building| building[0] == 0 && building[1] == HQ_TYPE)
            .ok_or("the bot's own HQ is not among the buildings")?;
        let hq_cell = cell_of(own_hq[2], own_hq[3], void)?;
        let seat = HQ
            .iter()
            .position(|&hq| hq == hq_cell)
            .ok_or_else(|| format!("the bot's HQ is at cell {hq_cell}, not in a corner"))?;

        let mut spot_bits = Bits::EMPTY;
        for (x, y) in spots {
            if void.test(y * SIZE + x) {
                return Err(format!("mine spot ({x}, {y}) is a void cell"));
            }
            spot_bits.set(y * SIZE + x);
        }

        // Relative owner 0 is `seat`, 1 is the other seat.
        let absolute = |relative: usize| if relative == 0 { seat } else { 1 - seat };
        let mut state = State {
            map: MapData::shared(void, spot_bits),
            league,
            own: [Bits::EMPTY; 2],
            active: [Bits::EMPTY; 2],
            mine: Bits::EMPTY,
            tower: Bits::EMPTY,
            units: [Bits::EMPTY; 2],
            level: [Bits::EMPTY; 3],
            movable: Bits::EMPTY,
            unit_id: [0; CELLS],
            gold: [0; 2],
            income: [0; 2],
            to_act: seat,
            game_turn,
            next_id: 1,
            outcome: None,
        };
        for relative in 0..2 {
            let owner = absolute(relative);
            state.own[owner] = cells[relative];
            state.active[owner] = active[relative];
            state.gold[owner] = header[2 * relative];
            state.income[owner] = header[2 * relative + 1];
        }

        let mut occupied = Bits::EMPTY;
        for [owner, kind, x, y] in buildings {
            let relative = relative_owner(owner)?;
            let cell = cell_of(x, y, void)?;
            if !cells[relative].test(cell) {
                return Err(format!("building at ({x}, {y}) is not on its owner's cell"));
            }
            if occupied.test(cell) {
                return Err(format!("two buildings at ({x}, {y})"));
            }
            occupied.set(cell);
            match kind {
                HQ_TYPE if cell == HQ[absolute(relative)] => {}
                HQ_TYPE => return Err(format!("an HQ at ({x}, {y}), not in its owner's corner")),
                MINE_TYPE => state.mine.set(cell),
                TOWER_TYPE => state.tower.set(cell),
                _ => return Err(format!("unknown building type {kind}")),
            }
        }

        let mut seen = Bits::EMPTY;
        let mut largest_id = 0u32;
        for [owner, id, level, x, y] in units {
            let relative = relative_owner(owner)?;
            let cell = cell_of(x, y, void)?;
            if !cells[relative].test(cell) {
                return Err(format!("unit at ({x}, {y}) is not on its owner's cell"));
            }
            if seen.test(cell) {
                return Err(format!("two units at ({x}, {y})"));
            }
            if !(1..=3).contains(&level) {
                return Err(format!("unit level {level} is not 1, 2 or 3"));
            }
            let id = u32::try_from(id).map_err(|_| format!("unit id {id} is out of range"))?;
            if seen.iter().any(|other| state.unit_id[other] == id) {
                return Err(format!("two units have id {id}"));
            }
            seen.set(cell);
            state.units[absolute(relative)].set(cell);
            state.level[level as usize - 1].set(cell);
            state.unit_id[cell] = id;
            largest_id = largest_id.max(id);
        }
        if !seen.is_empty() {
            state.next_id = largest_id
                .checked_add(1)
                .ok_or_else(|| format!("unit id {largest_id} is out of range"))?;
        }
        state.movable = state.units[seat];
        Ok(state)
    }
}

/// The next line of the text, or an error naming what was expected.
fn next_line<'a>(lines: &mut Lines<'a>, what: &str) -> Result<&'a str, String> {
    lines
        .next()
        .ok_or_else(|| format!("the text ends before {what}"))
}

/// Only blank lines may follow the end of a text.
fn end_of_text(mut lines: Lines<'_>, what: &str) -> Result<(), String> {
    match lines.find(|line| !line.trim().is_empty()) {
        Some(line) => Err(format!("extra text after the end of {what}: {line:?}")),
        None => Ok(()),
    }
}

/// A line of exactly `N` integers.
fn numbers<const N: usize>(line: &str, what: &str) -> Result<[i64; N], String> {
    let mut words = line.split_whitespace();
    let mut values = [0i64; N];
    for value in &mut values {
        let word = words
            .next()
            .ok_or_else(|| format!("{what}: expected {N} numbers in {line:?}"))?;
        *value = word
            .parse()
            .map_err(|_| format!("{what}: {word:?} is not an integer"))?;
    }
    if words.next().is_some() {
        return Err(format!("{what}: expected {N} numbers in {line:?}"));
    }
    Ok(values)
}

/// A line with a count of at most one line per cell.
fn count_line(lines: &mut Lines<'_>, what: &str) -> Result<usize, String> {
    let [count] = numbers(next_line(lines, what)?, what)?;
    usize::try_from(count)
        .ok()
        .filter(|&count| count <= CELLS)
        .ok_or_else(|| format!("{what}: {count} is out of range"))
}

fn coordinate(value: i64) -> Result<usize, String> {
    usize::try_from(value)
        .ok()
        .filter(|&value| value < SIZE)
        .ok_or_else(|| format!("coordinate {value} is off the map"))
}

/// The cell at `(x, y)`, which must be on the map and not void.
fn cell_of(x: i64, y: i64, void: Bits) -> Result<usize, String> {
    let cell = coordinate(y)? * SIZE + coordinate(x)?;
    if void.test(cell) {
        return Err(format!("({x}, {y}) is a void cell"));
    }
    Ok(cell)
}

fn relative_owner(owner: i64) -> Result<usize, String> {
    match owner {
        0 => Ok(0),
        1 => Ok(1),
        _ => Err(format!("owner {owner} is not 0 or 1")),
    }
}
