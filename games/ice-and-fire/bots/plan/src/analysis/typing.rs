//! The analysis modes of the unit types (`crate::types`):
//!
//! ```sh
//! ice-and-fire-bot-plan types --positions BANK > units.jsonl
//! ice-and-fire-bot-plan types-table --json TABLE.json --out games/ice-and-fire/bots/plan/src/types/table.rs
//! ```
//!
//! `types` reads a bank whose blocks carry the answer a player played
//! (`played`), and prints a JSON line a position, for the side to act:
//!
//! ```text
//! {"game":NAME,"turn":T,"units":[[type, level, turn band, gold band, cap, en2, own2, tw2, d, dj, onj, free1, adj, kind, top3], ...]}
//! ```
//!
//! with the raw numbers (no clamp), `kind` the kind of the unit's first
//! `MOVE` in the answer (`stay` if it has none), and `top3` the three
//! likeliest kinds by the embedded table (none while the table is empty).
//! `research/codingame/2026-11-28-type-planner/tables.py` counts them.
//!
//! `types-table` reads the table that script writes (`{"levels": [L0, ..,
//! L4]}`, a level being a list of `[[fields], [8 probabilities]]`, the
//! fields those of `types::Key::fields` with names for the type, the turn
//! band and the gold band), quantizes the probabilities to 255 and writes
//! the module `types/table.rs`: the table's bytes (`types::Table::to_bytes`)
//! packed into characters outside ASCII (`cg_pack`'s format,
//! docs/research/utf16-packing.md; `cg_packer` is a tool with
//! dependencies a bot's bundle cannot have, so the encoder is repeated here
//! and a test checks that it gives `cg_packer`'s text).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use ice_and_fire_engine::{Command, State};

use super::judge::commands_of;
use super::{read_bank, Position};
use crate::types::{
    code_of_fields, likeliest, prior, Band, Gold, Kind, Table, Type, Typing, KINDS, LEVELS,
};

#[cfg(test)]
mod tests;

/// The cell of the unit `id` of `seat` in `state`.
fn cell_of(state: &State, seat: usize, id: u32) -> Option<usize> {
    state
        .units(seat)
        .iter()
        .find(|&cell| state.unit_id(cell) == Some(id))
}

/// The kind of the first accepted `MOVE` of each unit of the side to act, by
/// the cell it started on.
pub fn kinds_of(state: &State, typing: &Typing, commands: &[Command]) -> BTreeMap<usize, Kind> {
    let seat = state.to_act();
    let mut scratch = state.clone();
    let mut done = BTreeSet::new();
    let mut out = BTreeMap::new();
    for &command in commands {
        let Command::Move { id, .. } = command else {
            scratch.apply(command);
            continue;
        };
        let from = cell_of(&scratch, seat, id);
        if !scratch.apply(command) || !done.insert(id) {
            continue;
        }
        if let (Some(from), Some(to)) = (from, cell_of(&scratch, seat, id)) {
            out.insert(from, typing.kind(state, from, to));
        }
    }
    out
}

/// A JSON string.
fn json_string(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c.is_control() => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// The line of `types` for a position.
pub fn unit_line(position: &Position) -> Result<String, String> {
    let state = position.state()?;
    let played = position
        .played
        .as_deref()
        .ok_or_else(|| format!("{}: the block has no played line", position.name))?;
    let typing = Typing::new(&state);
    let real = kinds_of(&state, &typing, &commands_of(played));
    let mut units = Vec::new();
    for cell in state.units(state.to_act()).iter() {
        let Some(key) = typing.key(&state, cell) else {
            continue;
        };
        let kind = real.get(&cell).copied().unwrap_or(Kind::Stay);
        let top: Vec<String> = likeliest(&prior(&key))
            .iter()
            .take(3)
            .map(|k| json_string(k.name()))
            .collect();
        units.push(format!(
            "[{},{},{},{},{},{},{},{},{},{},{},{},{},{},[{}]]",
            json_string(key.typ.name()),
            key.level,
            json_string(key.band.name()),
            json_string(key.gold.name()),
            key.cap,
            key.en2,
            key.own2,
            key.tw2,
            key.d,
            key.dj,
            u8::from(key.onj),
            key.free1,
            key.adj,
            json_string(kind.name()),
            top.join(",")
        ));
    }
    Ok(format!(
        "{{\"game\":{},\"turn\":{},\"units\":[{}]}}",
        json_string(&position.name),
        position.game_turn,
        units.join(",")
    ))
}

/// The value after `flag` in `args`.
fn option(args: &[String], flag: &str) -> Result<Option<String>, String> {
    let Some(at) = args.iter().position(|a| a == flag) else {
        return Ok(None);
    };
    args.get(at + 1)
        .cloned()
        .map(Some)
        .ok_or_else(|| format!("{flag} needs a value"))
}

/// Refuses arguments other than `known` flags with their values.
fn check_flags(args: &[String], known: &[&str]) -> Result<(), String> {
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        if !known.contains(&arg.as_str()) {
            return Err(format!("unknown argument {arg}"));
        }
        args.next();
    }
    Ok(())
}

/// Runs `types` with its arguments (after the word `types`).
pub fn run_positions(args: &[String]) -> Result<String, String> {
    check_flags(args, &["--positions"])?;
    let path = option(args, "--positions")?.ok_or("types needs --positions BANK")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let mut out = String::new();
    for position in read_bank(&text)? {
        out.push_str(&unit_line(&position)?);
        out.push('\n');
    }
    Ok(out)
}

// --- The table ---

/// A JSON value (numbers as `f64`; the table needs nothing more).
#[derive(Debug, PartialEq)]
pub enum Json {
    Number(f64),
    Text(String),
    List(Vec<Json>),
    Object(Vec<(String, Json)>),
    Other,
}

struct Parser<'a> {
    text: &'a [u8],
    at: usize,
}

impl Parser<'_> {
    fn space(&mut self) {
        while self.text.get(self.at).is_some_and(u8::is_ascii_whitespace) {
            self.at += 1;
        }
    }

    fn fail<T>(&self, what: &str) -> Result<T, String> {
        Err(format!("JSON, byte {}: {what}", self.at))
    }

    fn expect(&mut self, byte: u8) -> Result<(), String> {
        self.space();
        if self.text.get(self.at) == Some(&byte) {
            self.at += 1;
            Ok(())
        } else {
            self.fail(&format!("expected {:?}", byte as char))
        }
    }

    fn string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let mut out = Vec::new();
        loop {
            let Some(&byte) = self.text.get(self.at) else {
                return self.fail("a string is not closed");
            };
            self.at += 1;
            match byte {
                b'"' => break,
                b'\\' => {
                    let Some(&escaped) = self.text.get(self.at) else {
                        return self.fail("a string is not closed");
                    };
                    self.at += 1;
                    out.push(match escaped {
                        b'n' => b'\n',
                        b't' => b'\t',
                        b'"' | b'\\' | b'/' => escaped,
                        _ => return self.fail("an escape the table does not use"),
                    });
                }
                _ => out.push(byte),
            }
        }
        String::from_utf8(out).map_err(|_| "JSON: a string is not UTF-8".to_string())
    }

    fn value(&mut self, depth: usize) -> Result<Json, String> {
        if depth > 8 {
            return self.fail("nested too deep");
        }
        self.space();
        match self.text.get(self.at) {
            Some(b'"') => Ok(Json::Text(self.string()?)),
            Some(b'[') => {
                self.at += 1;
                let mut items = Vec::new();
                self.space();
                if self.text.get(self.at) == Some(&b']') {
                    self.at += 1;
                    return Ok(Json::List(items));
                }
                loop {
                    items.push(self.value(depth + 1)?);
                    self.space();
                    match self.text.get(self.at) {
                        Some(b',') => self.at += 1,
                        Some(b']') => {
                            self.at += 1;
                            return Ok(Json::List(items));
                        }
                        _ => return self.fail("expected , or ]"),
                    }
                }
            }
            Some(b'{') => {
                self.at += 1;
                let mut items = Vec::new();
                self.space();
                if self.text.get(self.at) == Some(&b'}') {
                    self.at += 1;
                    return Ok(Json::Object(items));
                }
                loop {
                    let key = self.string()?;
                    self.expect(b':')?;
                    items.push((key, self.value(depth + 1)?));
                    self.space();
                    match self.text.get(self.at) {
                        Some(b',') => self.at += 1,
                        Some(b'}') => {
                            self.at += 1;
                            return Ok(Json::Object(items));
                        }
                        _ => return self.fail("expected , or }"),
                    }
                }
            }
            Some(b't' | b'f' | b'n') => {
                for word in ["true", "false", "null"] {
                    if self.text[self.at..].starts_with(word.as_bytes()) {
                        self.at += word.len();
                        return Ok(Json::Other);
                    }
                }
                self.fail("an unknown word")
            }
            Some(b'-' | b'0'..=b'9') => {
                let start = self.at;
                while self
                    .text
                    .get(self.at)
                    .is_some_and(|b| b"+-.eE0123456789".contains(b))
                {
                    self.at += 1;
                }
                std::str::from_utf8(&self.text[start..self.at])
                    .ok()
                    .and_then(|s| s.parse::<f64>().ok())
                    .map(Json::Number)
                    .ok_or_else(|| format!("JSON, byte {start}: a bad number"))
            }
            _ => self.fail("a value was expected"),
        }
    }
}

/// Reads one JSON value, and nothing after it.
pub fn parse_json(text: &str) -> Result<Json, String> {
    let mut parser = Parser {
        text: text.as_bytes(),
        at: 0,
    };
    let value = parser.value(0)?;
    parser.space();
    if parser.at != text.len() {
        return parser.fail("text after the value");
    }
    Ok(value)
}

/// The number of a field of a key: names of the type, turn band and gold
/// band, else a whole number.
fn field(index: usize, value: &Json) -> Result<u32, String> {
    let bad = || format!("field {index} of a key is not what the table takes: {value:?}");
    match (index, value) {
        (0, Json::Text(t)) => Type::from_name(t).map(|v| v as u32).ok_or_else(bad),
        (2, Json::Text(t)) => Band::from_name(t).map(|v| v as u32).ok_or_else(bad),
        (3, Json::Text(t)) => Gold::from_name(t).map(|v| v as u32).ok_or_else(bad),
        // The level is written 1 or 2 (2 for 2 and over).
        (1, Json::Number(n)) if (1.0..=2.0).contains(n) && n.fract() == 0.0 => Ok(*n as u32 - 1),
        (0..=3, _) => Err(bad()),
        (_, Json::Number(n)) if *n >= 0.0 && *n < 100.0 && n.fract() == 0.0 => Ok(*n as u32),
        _ => Err(bad()),
    }
}

/// `weights` as numbers out of 255 that sum to 255, by the largest
/// remainders (each is within 1 of its share), then one unit moved where
/// needed so that none of the three likeliest kinds is rounded to no more
/// than a kind that is less likely (the set of the three likeliest is what
/// the table is checked on); `None` if they sum to 0.
pub fn quantize(weights: &[f64; KINDS]) -> Option<[u8; KINDS]> {
    let total: f64 = weights.iter().sum();
    if total <= 0.0 || !total.is_finite() {
        return None;
    }
    let shares: Vec<f64> = weights.iter().map(|w| 255.0 * w / total).collect();
    let mut out = [0u8; KINDS];
    for (q, share) in out.iter_mut().zip(&shares) {
        *q = *share as u8;
    }
    let mut left = 255 - out.iter().map(|&q| usize::from(q)).sum::<usize>();
    let mut order: Vec<usize> = (0..KINDS).collect();
    order.sort_by(|&a, &b| {
        let (fa, fb) = (shares[a].fract(), shares[b].fract());
        fb.total_cmp(&fa).then(a.cmp(&b))
    });
    for &i in &order {
        if left == 0 {
            break;
        }
        out[i] += 1;
        left -= 1;
    }
    let mut likely: Vec<usize> = (0..KINDS).collect();
    likely.sort_by(|&a, &b| weights[b].total_cmp(&weights[a]).then(a.cmp(&b)));
    for _ in 0..KINDS {
        let mut moved = false;
        for &a in &likely[..3] {
            for &b in &likely[3..] {
                if weights[a] > weights[b] && out[a] <= out[b] && out[b] > 0 {
                    out[a] += 1;
                    out[b] -= 1;
                    moved = true;
                }
            }
        }
        if !moved {
            break;
        }
    }
    Some(out)
}

/// The table of the JSON the builder writes.
pub fn table_of_json(text: &str) -> Result<Table, String> {
    let Json::Object(top) = parse_json(text)? else {
        return Err("the table is not a JSON object".to_string());
    };
    let Some((_, Json::List(levels))) = top.iter().find(|(k, _)| k == "levels") else {
        return Err("the table has no \"levels\" list".to_string());
    };
    if levels.len() != LEVELS {
        return Err(format!("{} levels, not {LEVELS}", levels.len()));
    }
    let mut table = Table::default();
    for (depth, level) in levels.iter().enumerate() {
        let Json::List(entries) = level else {
            return Err(format!("level {depth} is not a list"));
        };
        let mut rows: Vec<(u32, [u8; KINDS])> = Vec::new();
        for entry in entries {
            let Json::List(pair) = entry else {
                return Err(format!("an entry of level {depth} is not a list"));
            };
            let [Json::List(fields), Json::List(probs)] = pair.as_slice() else {
                return Err(format!(
                    "an entry of level {depth} is not [fields, probabilities]"
                ));
            };
            let fields: Vec<u32> = fields
                .iter()
                .enumerate()
                .map(|(i, v)| field(i, v))
                .collect::<Result<_, _>>()?;
            let code = code_of_fields(&fields, depth).map_err(|e| format!("level {depth}: {e}"))?;
            if probs.len() != KINDS {
                return Err(format!(
                    "level {depth}: {} probabilities, not {KINDS}",
                    probs.len()
                ));
            }
            let mut weights = [0.0f64; KINDS];
            for (w, value) in weights.iter_mut().zip(probs) {
                let Json::Number(x) = value else {
                    return Err(format!("level {depth}: a probability is not a number"));
                };
                if !(0.0..=1.0).contains(x) {
                    return Err(format!("level {depth}: a probability {x} is not in 0 to 1"));
                }
                *w = *x;
            }
            let p = quantize(&weights)
                .ok_or_else(|| format!("level {depth}: probabilities sum to 0"))?;
            rows.push((code, p));
        }
        rows.sort_by_key(|&(code, _)| code);
        if rows.windows(2).any(|w| w[0].0 == w[1].0) {
            return Err(format!("level {depth} has a key twice"));
        }
        table.levels[depth] = rows;
    }
    Ok(table)
}

/// `data` packed as `cg_packer::pack` does (docs/research/utf16-packing.md),
/// decoded by `cg_pack::unpack`.
pub fn pack(data: &[u8]) -> Result<String, String> {
    use cg_pack::{in_alphabet, BASE, BLOCK_BYTES, BLOCK_CHARS};
    let len = u32::try_from(data.len()).map_err(|_| "the table is too large to pack")?;
    let alphabet: Vec<char> = (0..0x10000u32)
        .filter(|&c| in_alphabet(c))
        .filter_map(char::from_u32)
        .collect();
    if alphabet.len() != BASE as usize {
        return Err("cg_pack's alphabet and BASE disagree".to_string());
    }
    let mut bytes = Vec::with_capacity(data.len() + 4 + BLOCK_BYTES);
    bytes.extend_from_slice(&len.to_le_bytes());
    bytes.extend_from_slice(data);
    bytes.resize(bytes.len().div_ceil(BLOCK_BYTES) * BLOCK_BYTES, 0);
    let mut packed = String::new();
    let mut digits = [0u32; BLOCK_CHARS];
    for block in bytes.chunks(BLOCK_BYTES) {
        // 1,024 bits in 32 limbs of 32 bits, most significant first; the top
        // byte stays 0.
        let mut wide = [0u8; BLOCK_BYTES + 1];
        wide[1..].copy_from_slice(block);
        let mut limbs = [0u32; 32];
        for (limb, chunk) in limbs.iter_mut().zip(wide.chunks(4)) {
            *limb = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        // The digits come out least significant first.
        for digit in digits.iter_mut().rev() {
            let mut remainder = 0u64;
            for limb in limbs.iter_mut() {
                let v = (remainder << 32) | u64::from(*limb);
                *limb = (v / u64::from(BASE)) as u32;
                remainder = v % u64::from(BASE);
            }
            *digit = remainder as u32;
        }
        packed.extend(digits.iter().map(|&digit| alphabet[digit as usize]));
    }
    Ok(packed)
}

/// The module `types/table.rs` for `table`: a short comment (every character
/// counts towards the bot's size) and the packed bytes. Also the number of
/// bytes packed.
pub fn module_text(table: &Table) -> Result<(String, usize), String> {
    let bytes = table.to_bytes();
    let packed = pack(&bytes)?;
    Ok((
        format!("// `types-table`; do not edit.\npub const TABLE: &str = \"{packed}\";\n"),
        bytes.len(),
    ))
}

/// Runs `types-table` with its arguments (after the word).
pub fn run_table(args: &[String]) -> Result<String, String> {
    check_flags(args, &["--json", "--out"])?;
    let input = option(args, "--json")?.ok_or("types-table needs --json FILE")?;
    let out = option(args, "--out")?.ok_or("types-table needs --out FILE")?;
    let text = std::fs::read_to_string(&input).map_err(|e| format!("{input}: {e}"))?;
    let mut table = table_of_json(&text)?;
    let pruned = table.prune();
    let (module, bytes) = module_text(&table)?;
    std::fs::write(&out, &module).map_err(|e| format!("{out}: {e}"))?;
    Ok(format!(
        "wrote {out}: keys by level {}; {pruned} entries dropped as equal to their coarser entry; {bytes} bytes; {} characters (UTF-16 units: {})\n",
        table
            .levels
            .iter()
            .map(|l| l.len().to_string())
            .collect::<Vec<_>>()
            .join("/"),
        module.chars().count(),
        module.encode_utf16().count()
    ))
}
