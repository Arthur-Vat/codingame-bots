//! Unit types and the tables of zasmu's actions by type (the design of
//! `games/ice-and-fire/research/codingame/2026-11-28-type-planner.md`,
//! stage 1: no play uses this yet).
//!
//! [`Typing::new`] looks once at a position for the side to act: its
//! distance to the enemy's land, its joints (cells whose loss cuts off part
//! of its active land from its HQ) and the guard zone around the joints near
//! the enemy. [`Typing::key`] then gives each unit a [`Key`], its type and
//! what the position says around it, at five levels of detail (see
//! [`Key::code`]); [`Typing::kind`] names what a unit did; [`prior`] gives
//! from the table (`table.rs`, counted offline by
//! `research/codingame/2026-11-28-type-planner/tables.py` on zasmu's
//! games and packed by the analysis mode `types-table`) how likely each
//! [`Kind`] of action is for a key, from the finest level the table knows.

use std::sync::OnceLock;

use ice_and_fire_engine::{Bits, State, CELLS, HQ};
use ice_and_fire_tactics::chain::{cost_to, UNREACHABLE};

mod table;
#[cfg(test)]
mod tests;

/// The distance of a cell from land that is not there: no enemy cell, no
/// joint, or no path.
pub const FAR: u8 = 99;

/// The levels of detail of a key, 0 (coarsest) to 4.
pub const LEVELS: usize = 5;

/// The number of kinds of action.
pub const KINDS: usize = 8;

/// The number of fields of a key.
const FIELDS: usize = 13;
/// How many leading fields a key has at each level.
const LENGTHS: [usize; LEVELS] = [2, 3, 4, 8, 13];
/// The bits of each field of a key's code.
const WIDTHS: [u32; FIELDS] = [3, 1, 3, 2, 1, 2, 2, 1, 3, 2, 1, 2, 2];
/// The largest value of each field of a key's code.
const MAXIMA: [u32; FIELDS] = [4, 1, 4, 3, 1, 2, 2, 1, 4, 3, 1, 2, 2];

/// What a unit is for, from where it stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    /// On or next to the enemy's land.
    Pusher,
    /// On or next to a joint within 3 steps of the enemy.
    Guard,
    /// Within 3 steps of the enemy's land.
    Reserve,
    /// Farther back, with a free neutral cell next to it.
    BackFree,
    /// Farther back, with no free neutral cell next to it.
    BackShut,
}

impl Type {
    pub const ALL: [Type; 5] = [
        Type::Pusher,
        Type::Guard,
        Type::Reserve,
        Type::BackFree,
        Type::BackShut,
    ];

    pub fn name(self) -> &'static str {
        ["pusher", "guard", "reserve", "back-free", "back-shut"][self as usize]
    }

    pub fn from_name(name: &str) -> Option<Type> {
        Type::ALL.into_iter().find(|t| t.name() == name)
    }
}

/// The stretch of the game, by game turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Band {
    /// Turns 5 and 6, and any turn before 5.
    T5,
    T7,
    T9,
    T13,
    T21,
}

impl Band {
    pub const ALL: [Band; 5] = [Band::T5, Band::T7, Band::T9, Band::T13, Band::T21];

    pub fn of(turn: u32) -> Band {
        match turn {
            0..=6 => Band::T5,
            7..=8 => Band::T7,
            9..=12 => Band::T9,
            13..=20 => Band::T13,
            _ => Band::T21,
        }
    }

    pub fn name(self) -> &'static str {
        ["5-6", "7-8", "9-12", "13-20", "21+"][self as usize]
    }

    pub fn from_name(name: &str) -> Option<Band> {
        Band::ALL.into_iter().find(|b| b.name() == name)
    }
}

/// The side's gold, or a turn on which a chain to an HQ can be paid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gold {
    /// One side can pay a chain of trainings to the other's HQ.
    Kill,
    Under40,
    Under80,
    Rich,
}

impl Gold {
    pub const ALL: [Gold; 4] = [Gold::Kill, Gold::Under40, Gold::Under80, Gold::Rich];

    pub fn name(self) -> &'static str {
        ["kill", "<40", "40-79", "80+"][self as usize]
    }

    pub fn from_name(name: &str) -> Option<Gold> {
        Gold::ALL.into_iter().find(|g| g.name() == name)
    }
}

/// What a unit's action did, in eight kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// The move ends on a cell of the enemy.
    Capture,
    /// Nearer the enemy's land, onto a neutral cell.
    ForwardN,
    /// Nearer the enemy's land, onto an own cell.
    ForwardO,
    SideN,
    SideO,
    BackN,
    BackO,
    /// The unit does not move.
    Stay,
}

impl Kind {
    pub const ALL: [Kind; KINDS] = [
        Kind::Capture,
        Kind::ForwardN,
        Kind::ForwardO,
        Kind::SideN,
        Kind::SideO,
        Kind::BackN,
        Kind::BackO,
        Kind::Stay,
    ];

    pub fn name(self) -> &'static str {
        [
            "capture",
            "forward-n",
            "forward-o",
            "side-n",
            "side-o",
            "back-n",
            "back-o",
            "stay",
        ][self as usize]
    }
}

/// What the position says of one unit, with the raw numbers (the clamps
/// belong to [`Key::code`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Key {
    pub typ: Type,
    /// The unit's level, 1 to 3.
    pub level: u8,
    pub band: Band,
    pub gold: Gold,
    /// 1 if a neighbour of the unit is the enemy's and it may enter it.
    pub cap: u8,
    /// Enemy units within 2 steps.
    pub en2: u8,
    /// Own other units within 2 steps.
    pub own2: u8,
    /// Enemy towers within 2 steps.
    pub tw2: u8,
    /// Distance to the nearest enemy cell.
    pub d: u8,
    /// Distance to the nearest joint.
    pub dj: u8,
    /// The unit stands on a joint.
    pub onj: bool,
    /// Free neutral cells next to the unit.
    pub free1: u8,
    /// Own units next to the unit.
    pub adj: u8,
}

impl Key {
    /// The clamped fields: type, level less 1 (at most 1), turn band, gold
    /// band, cap, en2, own2, tw2, d, dj, onj, free1, adj.
    pub fn fields(&self) -> [u32; FIELDS] {
        let u = u32::from;
        [
            self.typ as u32,
            u(self.level.clamp(1, 2) - 1),
            self.band as u32,
            self.gold as u32,
            u(self.cap.min(1)),
            u(self.en2.min(2)),
            u(self.own2.min(2)),
            u(self.tw2.min(1)),
            u(self.d.min(4)),
            u(self.dj.min(3)),
            u32::from(self.onj),
            u(self.free1.min(2)),
            u(self.adj.min(2)),
        ]
    }

    /// The key at `depth`, 0 to 4, as one number: L0 is the type and the
    /// level (2 and over together); L1 adds the turn band; L2 the gold band;
    /// L3 whether a capture is possible and the units and towers within 2
    /// steps (at most 2, 2 and 1); L4 the distances to the enemy (at most 4)
    /// and to a joint (at most 3), whether on a joint, the free neutral
    /// cells (at most 2) and the own units next to it (at most 2).
    pub fn code(&self, depth: usize) -> u32 {
        code_of(&self.fields(), depth)
    }
}

/// The number of the first `LENGTHS[depth]` of `fields`, packed by `WIDTHS`.
fn code_of(fields: &[u32; FIELDS], depth: usize) -> u32 {
    fields
        .iter()
        .zip(WIDTHS)
        .take(LENGTHS[depth.min(LEVELS - 1)])
        .fold(0, |code, (&v, width)| code << width | v)
}

/// What the position says to the side to act, found once.
pub struct Typing {
    seat: usize,
    /// Distance from each cell to the nearest enemy cell, [`FAR`] if none.
    d: [u8; CELLS],
    joints: Bits,
    /// The joints within 3 steps of the enemy, and their neighbours.
    guard: Bits,
    band: Band,
    /// Whether a chain of trainings to an HQ can be paid.
    kill: bool,
    gold: i64,
}

impl Typing {
    /// The typing of the side to act in `state`.
    pub fn new(state: &State) -> Typing {
        let seat = state.to_act();
        let enemy = 1 - seat;
        let valid = state.map().valid;
        let mut d = [FAR; CELLS];
        let mut seen = state.owned(enemy);
        let mut layer = seen;
        for cell in layer.iter() {
            d[cell] = 0;
        }
        let mut step = 0u8;
        while !layer.is_empty() && step < FAR - 1 {
            step += 1;
            layer = (layer.neighbours() & valid).and_not(seen);
            for cell in layer.iter() {
                d[cell] = step;
            }
            seen |= layer;
        }
        let joints = joints(state, seat);
        let mut guard = Bits::EMPTY;
        for cell in joints.iter() {
            if d[cell] <= 3 {
                guard.set(cell);
            }
        }
        guard |= guard.neighbours();
        let kill = cost_to(state, seat, true, HQ[enemy]) <= state.gold(seat) || {
            let theirs = cost_to(state, enemy, true, HQ[seat]);
            theirs != UNREACHABLE && theirs <= state.gold(enemy) + state.income(enemy)
        };
        Typing {
            seat,
            d,
            joints,
            guard,
            band: Band::of(state.game_turn()),
            kill,
            gold: state.gold(seat),
        }
    }

    /// The distance from `cell` to the nearest enemy cell ([`FAR`] if none
    /// or if `cell` is not on the map).
    pub fn distance(&self, cell: usize) -> u8 {
        self.d.get(cell).copied().unwrap_or(FAR)
    }

    /// The gold band of the side to act.
    pub fn gold_band(&self) -> Gold {
        if self.kill {
            Gold::Kill
        } else if self.gold < 40 {
            Gold::Under40
        } else if self.gold < 80 {
            Gold::Under80
        } else {
            Gold::Rich
        }
    }

    /// The key of the unit of the side to act on `cell`, or `None` if there
    /// is none.
    pub fn key(&self, state: &State, cell: usize) -> Option<Key> {
        let level = state.unit_level(cell)?;
        if !state.units(self.seat).test(cell) {
            return None;
        }
        let enemy = 1 - self.seat;
        let me = Bits::cell(cell);
        let near = me.neighbours();
        let occupied = state.units(0) | state.units(1);
        let free = (near & state.map().valid)
            .and_not(state.owned(0))
            .and_not(state.owned(1))
            .and_not(occupied);
        let ring = (me | near) | (me | near).neighbours();
        let count = |bits: Bits| u8::try_from(bits.count()).unwrap_or(u8::MAX);
        let d = self.distance(cell);
        let free1 = count(free);
        let typ = if d <= 1 {
            Type::Pusher
        } else if self.guard.test(cell) {
            Type::Guard
        } else if d <= 3 {
            Type::Reserve
        } else if free1 > 0 {
            Type::BackFree
        } else {
            Type::BackShut
        };
        let dj = self
            .joints
            .iter()
            .map(|j| state.map().distance(cell, j))
            .min()
            .map_or(FAR, |m| m.min(FAR));
        let capturable =
            (near & state.owned(enemy)) & state.enterable(self.seat, level.clamp(1, 3));
        Some(Key {
            typ,
            level,
            band: self.band,
            gold: self.gold_band(),
            cap: u8::from(!capturable.is_empty()),
            en2: count(state.units(enemy) & ring),
            own2: count(state.units(self.seat) & ring).saturating_sub(1),
            tw2: count(state.towers() & state.owned(enemy) & ring),
            d,
            dj,
            onj: self.joints.test(cell),
            free1,
            adj: count(near & state.units(self.seat)),
        })
    }

    /// The kind of an action that takes a unit of the side to act from
    /// `from` to `to` (both cells of the map), by the distances to the
    /// enemy's land and the owner of `to` in `state`.
    pub fn kind(&self, state: &State, from: usize, to: usize) -> Kind {
        if from == to {
            return Kind::Stay;
        }
        if state.owned(1 - self.seat).test(to) {
            return Kind::Capture;
        }
        let own = state.owned(self.seat).test(to);
        let (df, dt) = (self.distance(from), self.distance(to));
        match (dt.cmp(&df), own) {
            (std::cmp::Ordering::Less, false) => Kind::ForwardN,
            (std::cmp::Ordering::Less, true) => Kind::ForwardO,
            (std::cmp::Ordering::Equal, false) => Kind::SideN,
            (std::cmp::Ordering::Equal, true) => Kind::SideO,
            (std::cmp::Ordering::Greater, false) => Kind::BackN,
            (std::cmp::Ordering::Greater, true) => Kind::BackO,
        }
    }
}

/// The joints of `seat`: its active cells, except its HQ, whose loss leaves
/// part of its other active cells cut off from the HQ.
pub fn joints(state: &State, seat: usize) -> Bits {
    let active = state.active(seat);
    let hq = Bits::cell(HQ[seat]);
    let mut out = Bits::EMPTY;
    for v in active.iter() {
        if v == HQ[seat] {
            continue;
        }
        let rest = active.and_not(Bits::cell(v));
        let mut reached = hq & rest;
        loop {
            let next = (reached | reached.neighbours()) & rest;
            if next == reached {
                break;
            }
            reached = next;
        }
        if reached.count() + 1 < active.count() {
            out.set(v);
        }
    }
    out
}

/// A table: for each level, the keys' codes in increasing order with the
/// quantized probabilities of the eight kinds, which sum to 255.
#[derive(Debug, Default)]
pub struct Table {
    pub levels: [Vec<(u32, [u8; KINDS])>; LEVELS],
}

impl Table {
    /// The table written in `table.rs`; empty before the first one is made.
    pub fn embedded() -> &'static Table {
        static TABLE: OnceLock<Table> = OnceLock::new();
        TABLE.get_or_init(|| Table::decode(table::TABLE).unwrap_or_default())
    }

    /// The probabilities of the finest level that knows `key`, all 0 when no
    /// level does (the empty table).
    pub fn prior(&self, key: &Key) -> [u8; KINDS] {
        for depth in (0..LEVELS).rev() {
            let code = key.code(depth);
            let level = &self.levels[depth];
            if let Ok(at) = level.binary_search_by_key(&code, |&(c, _)| c) {
                return level[at].1;
            }
        }
        [0; KINDS]
    }

    /// Drops the entries that give what the table gives without them: those
    /// whose probabilities equal the finest coarser entry's. Lookups do not
    /// change. Returns how many were dropped.
    #[cfg(feature = "analysis")]
    pub fn prune(&mut self) -> usize {
        let mut dropped = 0;
        for depth in 1..LEVELS {
            let kept: Vec<(u32, [u8; KINDS])> = self.levels[depth]
                .iter()
                .copied()
                .filter(|&(code, p)| {
                    let mut at = code;
                    for coarser in (0..depth).rev() {
                        let cut: u32 = WIDTHS[LENGTHS[coarser]..LENGTHS[coarser + 1]].iter().sum();
                        at >>= cut;
                        let level = &self.levels[coarser];
                        if let Ok(i) = level.binary_search_by_key(&at, |&(c, _)| c) {
                            return level[i].1 != p;
                        }
                    }
                    true
                })
                .collect();
            dropped += self.levels[depth].len() - kept.len();
            self.levels[depth] = kept;
        }
        dropped
    }

    /// The table of the bytes `to_bytes` wrote, or why they are not one.
    pub fn from_bytes(bytes: &[u8]) -> Result<Table, String> {
        let mut at = 0;
        let mut table = Table::default();
        for (depth, level) in table.levels.iter_mut().enumerate() {
            let count = varint(bytes, &mut at)?;
            let mut code = 0u32;
            for i in 0..count {
                let delta = varint(bytes, &mut at)?;
                if i > 0 && delta == 0 {
                    return Err(format!("level {depth}: keys are not in increasing order"));
                }
                code = code
                    .checked_add(delta)
                    .ok_or_else(|| format!("level {depth}: a key's code overflows"))?;
                let end = at + KINDS - 1;
                let head: &[u8] = bytes.get(at..end).ok_or("the table ends inside an entry")?;
                let sum: u32 = head.iter().map(|&q| u32::from(q)).sum();
                let last = 255u32
                    .checked_sub(sum)
                    .ok_or_else(|| format!("level {depth}: probabilities over 255"))?;
                let mut p = [0u8; KINDS];
                p[..KINDS - 1].copy_from_slice(head);
                p[KINDS - 1] = last as u8;
                at = end;
                level.push((code, p));
            }
        }
        if at != bytes.len() {
            return Err(format!("{} bytes after the table", bytes.len() - at));
        }
        Ok(table)
    }

    /// The table of a packed text (`cg_pack`): empty for no text.
    pub fn decode(packed: &str) -> Result<Table, String> {
        if packed.is_empty() {
            return Ok(Table::default());
        }
        let bytes = cg_pack::unpack(packed).ok_or("the table is not packed text")?;
        Table::from_bytes(&bytes)
    }

    /// The bytes of the table: for each level, the number of keys, then for
    /// each key the difference of its code from the previous one (from 0
    /// for the first) as LEB128, and the probabilities of the first seven
    /// kinds, one byte each: the eighth is what is left of 255, so each
    /// entry sums to 255.
    #[cfg(feature = "analysis")]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for level in &self.levels {
            write_varint(&mut out, level.len() as u32);
            let mut previous = 0;
            for &(code, p) in level {
                write_varint(&mut out, code - previous);
                previous = code;
                debug_assert_eq!(p.iter().map(|&q| u32::from(q)).sum::<u32>(), 255);
                out.extend_from_slice(&p[..KINDS - 1]);
            }
        }
        out
    }
}

/// One LEB128 number of `bytes` at `at`, which moves on.
fn varint(bytes: &[u8], at: &mut usize) -> Result<u32, String> {
    let mut value = 0u32;
    for shift in (0..35).step_by(7) {
        let byte = *bytes.get(*at).ok_or("the table ends inside a number")?;
        *at += 1;
        let part = u32::from(byte & 0x7f);
        if shift == 28 && part > 0xf {
            return Err("a number of the table is too large".to_string());
        }
        value |= part << shift;
        if byte & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err("a number of the table is too long".to_string())
}

#[cfg(feature = "analysis")]
fn write_varint(out: &mut Vec<u8>, mut value: u32) {
    while value >= 0x80 {
        out.push((value & 0x7f) as u8 | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
}

/// The probabilities of the eight kinds for `key`, quantized to 255, from
/// the finest level of the embedded table that has its key; all 0 if the
/// table is empty.
pub fn prior(key: &Key) -> [u8; KINDS] {
    Table::embedded().prior(key)
}

/// The kinds in order of probability in `p`, likeliest first, ties by the
/// kinds' order; none when `p` is all 0.
pub fn likeliest(p: &[u8; KINDS]) -> Vec<Kind> {
    if p.iter().all(|&q| q == 0) {
        return Vec::new();
    }
    let mut order = Kind::ALL;
    order.sort_by(|a, b| {
        p[*b as usize]
            .cmp(&p[*a as usize])
            .then((*a as usize).cmp(&(*b as usize)))
    });
    order.to_vec()
}

/// The code of the key `fields` of level `depth` (a table's JSON gives
/// them), checked.
#[cfg(feature = "analysis")]
pub fn code_of_fields(fields: &[u32], depth: usize) -> Result<u32, String> {
    let length = *LENGTHS.get(depth).ok_or("no such level")?;
    if fields.len() != length {
        return Err(format!(
            "a key of level {depth} has {} fields, not {length}",
            fields.len()
        ));
    }
    let mut full = [0u32; FIELDS];
    for (i, &v) in fields.iter().enumerate() {
        if v > MAXIMA[i] {
            return Err(format!("field {i} of a key is {v}, over {}", MAXIMA[i]));
        }
        full[i] = v;
    }
    Ok(code_of(&full, depth))
}
