//! The analysis modes of zasmu's spending (`crate::types::SpendTable`, E109):
//!
//! ```sh
//! ice-and-fire-bot-plan spending --positions BANK > buys.jsonl
//! ice-and-fire-bot-plan spending-table --json TABLE.json --out games/ice-and-fire/bots/plan/src/types/spending/table.rs
//! ```
//!
//! `spending` reads a bank whose blocks carry the answer a player played
//! (`played`) and prints a JSON line a position, for the side to act:
//!
//! ```text
//! {"game":NAME,"turn":T,"band":BAND,"gold":GOLD,"buys":[KIND, ...]}
//! ```
//!
//! `buys` has the kind (`Buy::name`) of each training, tower and mine of the
//! answer, typed in the position; none if the reply buys nothing.
//! `research/codingame/2026-11-28-type-planner/spending.py` counts them.
//!
//! `spending-table` reads the table that script writes (`{"rows": [[nothing,
//! s0, .., s17], ..]}`: 20 rows, the turn band times 4 plus the gold band,
//! each the share of replies that buy nothing and the shares of the 18 kinds
//! among the purchases, as fractions), quantizes them to 255 and writes the
//! module `types/spending/table.rs`, the bytes packed as `types-table` does.

use std::fmt::Write as _;

use ice_and_fire_engine::Command;

use super::judge::commands_of;
use super::typing::{check_flags, json_string, option, pack, parse_json, Json};
use super::{read_bank, Position};
use crate::types::{Buy, SpendTable, Typing, BUYS, ROWS, WIDTH};

#[cfg(test)]
mod tests;

impl Buy {
    pub fn name(self) -> &'static str {
        [
            "t1-capture",
            "t1-front",
            "t1-near",
            "t1-far",
            "t2-capture",
            "t2-front",
            "t2-near",
            "t2-far",
            "t3-capture",
            "t3-front",
            "t3-near",
            "t3-far",
            "tower-joint",
            "tower-next-joint",
            "tower-front",
            "tower-near",
            "tower-far",
            "mine",
        ][self as usize]
    }
}

/// The line of `spending` for a position.
pub fn buy_line(position: &Position) -> Result<String, String> {
    let state = position.state()?;
    let played = position
        .played
        .as_deref()
        .ok_or_else(|| format!("{}: the block has no played line", position.name))?;
    let typing = Typing::new(&state);
    let buys: Vec<String> = commands_of(played)
        .into_iter()
        .filter(|c| !matches!(c, Command::Move { .. } | Command::Wait))
        .filter_map(|c| typing.buy(c))
        .map(|b| json_string(b.name()))
        .collect();
    let row = typing.row();
    Ok(format!(
        "{{\"game\":{},\"turn\":{},\"band\":{},\"gold\":{},\"buys\":[{}]}}",
        json_string(&position.name),
        position.game_turn,
        json_string(crate::types::Band::ALL[row / 4].name()),
        json_string(crate::types::Gold::ALL[row % 4].name()),
        buys.join(",")
    ))
}

/// Runs `spending` with its arguments (after the word).
pub fn run_positions(args: &[String]) -> Result<String, String> {
    check_flags(args, &["--positions"])?;
    let path = option(args, "--positions")?.ok_or("spending needs --positions BANK")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let mut out = String::new();
    for position in read_bank(&text)? {
        out.push_str(&buy_line(&position)?);
        out.push('\n');
    }
    Ok(out)
}

/// `shares` (fractions) quantized to sum 255 by largest remainders, ties to
/// the earlier kind; all 0 if they sum to nothing.
pub fn quantize(shares: &[f64]) -> Vec<u8> {
    let total: f64 = shares.iter().sum();
    if total <= 0.0 {
        return vec![0; shares.len()];
    }
    let scaled: Vec<f64> = shares.iter().map(|s| s / total * 255.0).collect();
    let mut out: Vec<u8> = scaled.iter().map(|s| s.floor() as u8).collect();
    let mut left = 255 - out.iter().map(|&q| u32::from(q)).sum::<u32>();
    let mut order: Vec<usize> = (0..scaled.len()).collect();
    order.sort_by(|&a, &b| {
        let (fa, fb) = (scaled[a].fract(), scaled[b].fract());
        fb.partial_cmp(&fa).unwrap_or(std::cmp::Ordering::Equal)
    });
    for &i in &order {
        if left == 0 {
            break;
        }
        out[i] += 1;
        left -= 1;
    }
    out
}

/// The table of the builder's JSON.
pub fn table_of_json(text: &str) -> Result<SpendTable, String> {
    let Json::Object(items) = parse_json(text)? else {
        return Err("the table is not an object".to_string());
    };
    let Some((_, Json::List(rows))) = items.iter().find(|(k, _)| k == "rows") else {
        return Err("the table has no rows list".to_string());
    };
    if rows.len() != ROWS {
        return Err(format!("{} rows, expected {ROWS}", rows.len()));
    }
    let mut table = SpendTable::empty();
    for (at, row) in rows.iter().enumerate() {
        let Json::List(values) = row else {
            return Err(format!("row {at} is not a list"));
        };
        if values.len() != WIDTH {
            return Err(format!(
                "row {at} has {} numbers, expected {WIDTH}",
                values.len()
            ));
        }
        let mut numbers = Vec::new();
        for value in values {
            match value {
                Json::Number(n) if (0.0..=1.0).contains(n) => numbers.push(*n),
                _ => return Err(format!("row {at}: a share is not a number from 0 to 1")),
            }
        }
        let shares = quantize(&numbers[1..]);
        table.rows[at][..BUYS].copy_from_slice(&shares);
        table.rows[at][BUYS] = (numbers[0] * 255.0).round() as u8;
    }
    Ok(table)
}

/// The bytes of `table`, row after row.
pub fn to_bytes(table: &SpendTable) -> Vec<u8> {
    table.rows.iter().flatten().copied().collect()
}

/// The module `types/spending/table.rs` for `table`, and the bytes packed.
pub fn module_text(table: &SpendTable) -> Result<(String, usize), String> {
    let bytes = to_bytes(table);
    let packed = pack(&bytes)?;
    let mut text = String::from("// `spending-table`; do not edit.\n");
    let _ = writeln!(text, "pub const TABLE: &str = \"{packed}\";");
    Ok((text, bytes.len()))
}

/// Runs `spending-table` with its arguments (after the word).
pub fn run_table(args: &[String]) -> Result<String, String> {
    check_flags(args, &["--json", "--out"])?;
    let input = option(args, "--json")?.ok_or("spending-table needs --json FILE")?;
    let out = option(args, "--out")?.ok_or("spending-table needs --out FILE")?;
    let text = std::fs::read_to_string(&input).map_err(|e| format!("{input}: {e}"))?;
    let table = table_of_json(&text)?;
    let (module, bytes) = module_text(&table)?;
    std::fs::write(&out, &module).map_err(|e| format!("{out}: {e}"))?;
    Ok(format!(
        "wrote {out}: {ROWS} rows; {bytes} bytes; {} characters (UTF-16 units: {})\n",
        module.chars().count(),
        module.encode_utf16().count()
    ))
}
