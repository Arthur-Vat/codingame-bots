//! The opening: the turns before the two territories meet
//! (`games/ice-and-fire/research/early-game.md`).
//!
//! While the territories are more than [`Config::gap`] cells apart, a turn is
//! played by rules instead of the plan search: units take free cells, and
//! trainings and mines are bought while they pay back. Every command is
//! checked on a scratch copy of the state, so the turn holds only commands
//! the rules accept.

use std::sync::OnceLock;
use std::time::Instant;

use ice_and_fire_engine::state::BUILD_COST;
use ice_and_fire_engine::{Bits, Command, State, CELLS, HQ, SIZE};

/// Which opening is played.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    /// No opening: the plan search plays from the first turn (v002).
    Plan,
    /// Each unit and training to the cell nearest the enemy HQ.
    Rush,
    /// Free cells first, trainings while the side has free cells for them.
    Fill,
    /// `Fill`, trainings and mines bought by their return at a horizon.
    Econ,
    /// Variants of the `Econ` turn, each played on with `Econ` for both
    /// sides until contact; the one that ends best is played.
    Rollout,
}

/// The settings of the opening.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub style: Style,
    /// The opening is played while the territories are more than this many
    /// steps apart.
    pub gap: u8,
    /// `Fill`: trainings stop when the free cells of our side are fewer than
    /// this many per unit.
    pub per_unit: i64,
    /// `Econ`: the game turn by which a purchase must pay back.
    pub horizon: i64,
    /// Gold kept once the territories are at most `reserve_gap` apart.
    pub reserve: i64,
    pub reserve_gap: u8,
    /// Whether mines are built (`Econ`).
    pub mines: bool,
    /// `Econ`: the cells a unit is expected to take after contact, on top of
    /// one a turn until contact.
    pub after_contact: i64,
    /// Whether the evaluation counts a side that cannot pay its upkeep as
    /// having lost its units.
    pub debt: bool,
    /// `Rollout`: most turns of each player simulated after a variant.
    pub depth: u32,
    /// `Rollout`: the worth of a unit and of a cell of the side's Voronoi
    /// share at the end of a simulation, in gold.
    pub unit_worth: i64,
    pub cell_worth: i64,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            style: Style::Plan,
            gap: 3,
            per_unit: 3,
            horizon: 15,
            reserve: 0,
            reserve_gap: 5,
            mines: true,
            after_contact: 2,
            debt: false,
            depth: 8,
            unit_worth: 0,
            cell_worth: 0,
        }
    }
}

/// The settings, read once from the environment (screening only).
pub fn config() -> Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    *CONFIG.get_or_init(|| {
        let mut c = Config::default();
        let var = |name: &str| std::env::var(name).ok();
        let num = |name: &str| var(name).and_then(|v| v.trim().parse::<i64>().ok());
        if let Some(style) = var("IAF_OPENING") {
            c.style = match style.trim() {
                "rush" => Style::Rush,
                "fill" => Style::Fill,
                "econ" => Style::Econ,
                "rollout" => Style::Rollout,
                _ => Style::Plan,
            };
        }
        if let Some(n) = num("IAF_GAP") {
            c.gap = n.clamp(1, 20) as u8;
        }
        if let Some(n) = num("IAF_PER_UNIT") {
            c.per_unit = n;
        }
        if let Some(n) = num("IAF_HORIZON") {
            c.horizon = n;
        }
        if let Some(n) = num("IAF_RESERVE") {
            c.reserve = n;
        }
        if let Some(n) = num("IAF_RESERVE_GAP") {
            c.reserve_gap = n.clamp(0, 30) as u8;
        }
        if let Some(n) = num("IAF_MINES") {
            c.mines = n != 0;
        }
        if let Some(n) = num("IAF_AFTER") {
            c.after_contact = n;
        }
        if let Some(n) = num("IAF_DEPTH") {
            c.depth = n.clamp(0, 50) as u32;
        }
        if let Some(n) = num("IAF_WU") {
            c.unit_worth = n;
        }
        if let Some(n) = num("IAF_WV") {
            c.cell_worth = n;
        }
        if let Some(n) = num("IAF_DEBT") {
            c.debt = n != 0;
        }
        c
    })
}

fn xy(cell: usize) -> (u8, u8) {
    ((cell % SIZE) as u8, (cell / SIZE) as u8)
}

/// Steps over non-void cells from the cells of `from`; `u8::MAX` where
/// there is no path.
pub fn distances(state: &State, from: Bits) -> [u8; CELLS] {
    let mut dist = [u8::MAX; CELLS];
    let valid = state.map().valid;
    let mut frontier = from & valid;
    let mut seen = frontier;
    let mut d = 0u8;
    while !frontier.is_empty() {
        for cell in frontier.iter() {
            dist[cell] = d;
        }
        let next = (frontier.neighbours() & valid).and_not(seen);
        seen |= next;
        frontier = next;
        d = d.saturating_add(1);
    }
    dist
}

/// The fewest steps from a cell of one territory to a cell of the other.
pub fn gap(state: &State) -> u8 {
    let dist = distances(state, state.owned(0));
    state
        .owned(1)
        .iter()
        .map(|cell| dist[cell])
        .min()
        .unwrap_or(u8::MAX)
}

/// The choices of one `Econ` turn that the rollout varies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Knobs {
    /// Most trainings and mines bought.
    purchases: usize,
    mines: bool,
    /// The weight of a free cell's nearness to the enemy.
    contest: i64,
}

impl Knobs {
    fn base(config: &Config) -> Knobs {
        Knobs {
            purchases: usize::MAX,
            mines: config.mines,
            contest: 2,
        }
    }
}

/// What one turn of the opening knows about the map.
struct Turn {
    me: usize,
    /// Steps from our territory and from the enemy's.
    mine: [u8; CELLS],
    theirs: [u8; CELLS],
    /// Steps from the enemy HQ.
    to_hq: [u8; CELLS],
    /// Neutral cells not yet claimed by a command of this turn.
    free: Bits,
    /// The free cells of our side: nearer our territory, or as near.
    side: Bits,
    reserve: i64,
    contest: i64,
}

impl Turn {
    fn new(state: &State, config: &Config, contest: i64) -> Turn {
        let me = state.to_act();
        let mine = distances(state, state.owned(me));
        let theirs = distances(state, state.owned(1 - me));
        let to_hq = distances(state, Bits::cell(HQ[1 - me]));
        let neutral = state.map().valid.and_not(state.owned(0) | state.owned(1));
        let mut side = Bits::EMPTY;
        for cell in neutral.iter() {
            if mine[cell] <= theirs[cell] {
                side.set(cell);
            }
        }
        let reserve = if gap(state) <= config.reserve_gap {
            config.reserve
        } else {
            0
        };
        Turn {
            me,
            mine,
            theirs,
            to_hq,
            free: neutral,
            side,
            reserve,
            contest,
        }
    }

    /// How much a free cell is worth taking now: contested cells (near the
    /// enemy) first, then cells that open more free cells.
    fn value(&self, cell: usize) -> i64 {
        let open = i64::from((Bits::cell(cell).neighbours() & self.free).count());
        let contest = 12 - i64::from(self.theirs[cell].min(12));
        3 * open + self.contest * contest - i64::from(self.to_hq[cell].min(30)) / 4
    }

    fn claim(&mut self, cell: usize) {
        self.free.clear(cell);
        self.side.clear(cell);
    }
}

/// Moves the unit on `from` toward `to`; whether the rules accepted it.
fn step(state: &mut State, from: usize, to: usize, out: &mut Vec<Command>) -> bool {
    let Some((id, _)) = state.unit_at(from) else {
        return false;
    };
    let (x, y) = xy(to);
    let command = Command::Move { id, x, y };
    if state.apply(command) {
        out.push(command);
        true
    } else {
        false
    }
}

fn train(state: &mut State, cell: usize, out: &mut Vec<Command>) -> bool {
    let (x, y) = xy(cell);
    let command = Command::Train { level: 1, x, y };
    if state.apply(command) {
        out.push(command);
        true
    } else {
        false
    }
}

/// The opening's commands for the player to act, or `None` once the
/// territories are near (or the style is `Plan`).
/// `deadline` bounds the rollout's time.
pub fn turn(state: &State, config: &Config, deadline: Option<Instant>) -> Option<Vec<Command>> {
    if config.style == Style::Plan || state.outcome().is_some() || gap(state) <= config.gap {
        return None;
    }
    match config.style {
        Style::Rush => {
            let mut scratch = state.clone();
            let mut out = Vec::new();
            let mut turn = Turn::new(state, config, 2);
            rush(&mut scratch, &mut turn, &mut out);
            Some(out)
        }
        Style::Fill | Style::Econ => Some(base_turn(state, config, Knobs::base(config))),
        Style::Rollout => Some(rollout(state, config, deadline)),
        Style::Plan => None,
    }
}

/// The `Fill` or `Econ` turn with these knobs (`Econ` for the rollout).
fn base_turn(state: &State, config: &Config, knobs: Knobs) -> Vec<Command> {
    let mut scratch = state.clone();
    let mut out = Vec::new();
    let mut turn = Turn::new(state, config, knobs.contest);
    fill(&mut scratch, &mut turn, config, knobs, &mut out);
    out
}

/// What a side will have collected by the horizon if nothing changes: its
/// gold plus its income for the turns left.
fn worth(state: &State, seat: usize, config: &Config) -> i64 {
    let left = (config.horizon - i64::from(state.game_turn())).max(1);
    let mut value = state.gold(seat) + left * state.income(seat);
    value += config.unit_worth * state.unit_count(seat) as i64;
    if config.cell_worth != 0 {
        let mine = distances(state, state.owned(seat));
        let theirs = distances(state, state.owned(1 - seat));
        let share = state
            .map()
            .valid
            .iter()
            .filter(|&c| mine[c] < theirs[c])
            .count();
        value += config.cell_worth * share as i64;
    }
    value
}

/// Tries variants of the `Econ` turn, plays each on with `Econ` for both
/// sides until contact or `config.depth` turns each, and returns the one
/// whose end is worth most to us against the enemy. The base turn is tried
/// first and kept on a tie or when time runs out.
fn rollout(state: &State, config: &Config, deadline: Option<Instant>) -> Vec<Command> {
    let me = state.to_act();
    let mut econ = *config;
    econ.style = Style::Econ;
    let base = Knobs::base(&econ);
    let mut variants = vec![base];
    for purchases in [0, 1, 2, 3, usize::MAX] {
        for mines in [true, false] {
            for contest in [0, 2, 5] {
                let knobs = Knobs {
                    purchases,
                    mines: mines && econ.mines,
                    contest,
                };
                if !variants.contains(&knobs) {
                    variants.push(knobs);
                }
            }
        }
    }
    let mut tried: Vec<Vec<Command>> = Vec::new();
    let mut best: Option<(i64, Vec<Command>)> = None;
    for knobs in variants {
        if best.is_some() && deadline.is_some_and(|t| Instant::now() >= t) {
            break;
        }
        let commands = base_turn(state, &econ, knobs);
        if tried.contains(&commands) {
            continue;
        }
        let mut sim = state.clone();
        sim.play_turn(&commands);
        for _ in 0..2 * econ.depth {
            if sim.outcome().is_some() || gap(&sim) <= econ.gap {
                break;
            }
            let reply = base_turn(&sim, &econ, base);
            sim.play_turn(&reply);
        }
        let value = worth(&sim, me, &econ) - worth(&sim, 1 - me, &econ);
        if best.as_ref().is_none_or(|(top, _)| value > *top) {
            best = Some((value, commands.clone()));
        }
        tried.push(commands);
    }
    best.map(|(_, commands)| commands).unwrap_or_default()
}

/// The gold boss's rule: each unit, nearest the enemy HQ first, to the
/// neighbour nearest the enemy HQ (a free cell on a tie), then trainings on
/// the free cells next to the territory nearest the enemy HQ.
fn rush(state: &mut State, turn: &mut Turn, out: &mut Vec<Command>) {
    let me = turn.me;
    let mut units: Vec<usize> = state.movable_units().iter().collect();
    units.sort_by_key(|&cell| (turn.to_hq[cell], cell));
    for from in units {
        let enter = state.enterable(me, 1);
        let best = (Bits::cell(from).neighbours() & enter)
            .iter()
            .min_by_key(|&cell| (turn.to_hq[cell], !turn.free.test(cell), cell));
        if let Some(to) = best {
            if (turn.to_hq[to] < turn.to_hq[from] || turn.free.test(to))
                && step(state, from, to, out)
            {
                turn.claim(to);
            }
        }
    }
    while state.gold(me) >= 10 + turn.reserve {
        let cells = state.near_active(me) & turn.free & state.enterable(me, 1);
        let Some(cell) = cells.iter().min_by_key(|&cell| (turn.to_hq[cell], cell)) else {
            break;
        };
        if !train(state, cell, out) {
            break;
        }
        turn.claim(cell);
    }
}

/// Gold collected by the horizon from a unit trained now that takes a new
/// cell on each of its next `k` turns, `h` turns before the horizon.
fn unit_return(h: i64, k: i64) -> i64 {
    let k = k.clamp(0, (h - 1).max(0));
    k * h - k * (k + 1) / 2 - 10
}

/// Free cells first. Units with a free neighbour, fewest choices first,
/// each take the best free neighbour no other unit took; the others walk to
/// the nearest free cell of our side. Then, by the style, trainings (and
/// mines) while they pay.
fn fill(state: &mut State, turn: &mut Turn, config: &Config, knobs: Knobs, out: &mut Vec<Command>) {
    let me = turn.me;
    let mut idle: Vec<usize> = Vec::new();
    loop {
        // The unit with the fewest free neighbours left moves first.
        let enter = state.enterable(me, 1);
        let mut pick: Option<(u32, usize)> = None;
        for from in state.movable_units().iter() {
            if idle.contains(&from) {
                continue;
            }
            let options = (Bits::cell(from).neighbours() & turn.free & enter).count();
            if options == 0 {
                idle.push(from);
                continue;
            }
            if pick.is_none_or(|(n, _)| options < n) {
                pick = Some((options, from));
            }
        }
        let Some((_, from)) = pick else {
            break;
        };
        let to = (Bits::cell(from).neighbours() & turn.free & enter)
            .iter()
            .max_by_key(|&cell| (turn.value(cell), std::cmp::Reverse(cell)));
        match to {
            Some(to) if step(state, from, to, out) => turn.claim(to),
            _ => idle.push(from),
        }
    }
    // Units with no free neighbour walk to the nearest free cell, ours first.
    for from in idle {
        if !state.movable_units().test(from) {
            continue;
        }
        let target = [turn.side, turn.free].into_iter().find_map(|cells| {
            cells
                .iter()
                .min_by_key(|&cell| (state.map().distance(from, cell), cell))
        });
        if let Some(target) = target {
            let mut trial = state.clone();
            let mut moved = Vec::new();
            if step(&mut trial, from, target, &mut moved) {
                *state = trial;
                out.extend(moved);
            }
        }
    }

    // Purchases.
    let h = (config.horizon - i64::from(state.game_turn())).max(0);
    // Both sides grow toward each other, so the gap closes by about two
    // steps a turn.
    let to_contact = (i64::from(gap(state)) - 2).max(0) / 2;
    let mut bought = 0;
    while bought < knobs.purchases {
        bought += 1;
        let gold = state.gold(me) - turn.reserve;
        let units = i64::from(state.unit_count(me) as u32);
        let room = i64::from(turn.side.count());
        let cells = state.near_active(me) & turn.free & state.enterable(me, 1);
        let best_cell = cells
            .iter()
            .max_by_key(|&cell| (turn.value(cell), std::cmp::Reverse(cell)));
        let unit_gain = match config.style {
            Style::Econ => unit_return(
                h,
                (room / (units + 1)).min(to_contact + config.after_contact),
            ),
            _ if room >= config.per_unit * (units + 1) => 1,
            _ => -1,
        };
        let spot = if config.style == Style::Econ && knobs.mines {
            let spots = state.map().spots & state.active(me) & state.empty_cells();
            spots
                .iter()
                .filter(|&cell| turn.mine[cell] == 0 && turn.theirs[cell] > 3)
                .min_by_key(|&cell| (state.map().distance(HQ[me], cell), cell))
        } else {
            None
        };
        let mine_gain = if spot.is_some() {
            4 * h - BUILD_COST
        } else {
            -1
        };
        let can_unit = gold >= 10 && best_cell.is_some() && unit_gain > 0;
        let can_mine = gold >= BUILD_COST && spot.is_some() && mine_gain > 0;
        if can_mine && (!can_unit || mine_gain >= unit_gain) {
            let (x, y) = xy(spot.unwrap_or(0));
            let command = Command::BuildMine { x, y };
            if state.apply(command) {
                out.push(command);
                continue;
            }
            break;
        }
        if !can_unit {
            break;
        }
        let cell = best_cell.unwrap_or(0);
        if !train(state, cell, out) {
            break;
        }
        turn.claim(cell);
    }
}
