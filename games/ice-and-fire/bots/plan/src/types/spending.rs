//! The kinds of purchase and zasmu's spending table (E109, stage 6 of
//! `games/ice-and-fire/research/codingame/2026-11-28-type-planner.md`).
//!
//! [`Typing::buy`] names a training, a tower or a mine by its kind
//! ([`Buy`]), and [`Typing::row`] gives the row of the table for the side to
//! act: its turn band and gold band (as `types` has them). The table
//! (`spending/table.rs`, counted offline by
//! `research/codingame/2026-11-28-type-planner/spending.py` on zasmu's games
//! and packed by the analysis mode `spending-table`) gives, for each row, how
//! often each kind is among zasmu's purchases, and how often its reply buys
//! nothing.

use std::sync::OnceLock;

use ice_and_fire_engine::{Command, SIZE};

use super::Typing;

mod table;
#[cfg(test)]
mod tests;

/// The number of kinds of purchase.
pub const BUYS: usize = 18;
/// The rows of the table: the five turn bands by the four gold bands.
pub const ROWS: usize = 20;
/// The bytes of a row: the share of each kind of purchase, then the share of
/// replies that buy nothing.
pub const WIDTH: usize = BUYS + 1;

/// A purchase, by what it is and where it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Buy {
    /// A training of level 1 on a cell of the enemy.
    Train1Capture,
    /// On a cell next to the enemy's land.
    Train1Front,
    /// Within 3 steps of the enemy's land.
    Train1Near,
    Train1Far,
    Train2Capture,
    Train2Front,
    Train2Near,
    Train2Far,
    Train3Capture,
    Train3Front,
    Train3Near,
    Train3Far,
    /// A tower on a joint of ours.
    TowerJoint,
    /// On a cell next to a joint.
    TowerNextJoint,
    /// Next to the enemy's land.
    TowerFront,
    TowerNear,
    TowerFar,
    Mine,
}

impl Buy {
    pub const ALL: [Buy; BUYS] = [
        Buy::Train1Capture,
        Buy::Train1Front,
        Buy::Train1Near,
        Buy::Train1Far,
        Buy::Train2Capture,
        Buy::Train2Front,
        Buy::Train2Near,
        Buy::Train2Far,
        Buy::Train3Capture,
        Buy::Train3Front,
        Buy::Train3Near,
        Buy::Train3Far,
        Buy::TowerJoint,
        Buy::TowerNextJoint,
        Buy::TowerFront,
        Buy::TowerNear,
        Buy::TowerFar,
        Buy::Mine,
    ];
}

impl Typing {
    /// The row of the table for the side to act: its turn band times 4, plus
    /// its gold band.
    pub fn row(&self) -> usize {
        self.band as usize * 4 + self.gold_band() as usize
    }

    /// The kind of a training, tower or mine of the side to act, by the
    /// distances to the enemy's land and the joints of the position typed;
    /// `None` for any other command.
    pub fn buy(&self, command: Command) -> Option<Buy> {
        let cell = |x: u8, y: u8| usize::from(y) * SIZE + usize::from(x);
        // 0 on the enemy's land, 1 next to it, 2 within 3 steps, 3 farther.
        let place = |cell: usize| match self.distance(cell) {
            0 => 0,
            1 => 1,
            2..=3 => 2,
            _ => 3,
        };
        match command {
            Command::Train { level, x, y } => {
                let at = usize::from(level.clamp(1, 3) - 1) * 4 + place(cell(x, y));
                Some(Buy::ALL[at])
            }
            Command::BuildTower { x, y } => {
                let c = cell(x, y);
                let at = if self.joints.test(c) {
                    0
                } else if self.joints.neighbours().test(c) {
                    1
                } else {
                    // front 2, near 3, far 4 (a tower is never on enemy land).
                    place(c).max(1) + 1
                };
                Some(Buy::ALL[12 + at])
            }
            Command::BuildMine { .. } => Some(Buy::Mine),
            _ => None,
        }
    }
}

/// zasmu's spending: for each row, the shares of the kinds of purchase and
/// of the replies that buy nothing, quantized to 255 (the shares of the
/// kinds sum to 255, or all 0 for a row without purchases).
#[derive(Debug, PartialEq, Eq)]
pub struct SpendTable {
    pub rows: [[u8; WIDTH]; ROWS],
}

impl SpendTable {
    /// The table written in `spending/table.rs`; all 0 before the first one
    /// is made.
    pub fn embedded() -> &'static SpendTable {
        static TABLE: OnceLock<SpendTable> = OnceLock::new();
        TABLE.get_or_init(|| {
            SpendTable::decode(table::TABLE).unwrap_or_else(|_| SpendTable::empty())
        })
    }

    pub fn empty() -> SpendTable {
        SpendTable {
            rows: [[0; WIDTH]; ROWS],
        }
    }

    /// The table of the bytes of the analysis' `to_bytes`, or why they are
    /// not one.
    pub fn from_bytes(bytes: &[u8]) -> Result<SpendTable, String> {
        if bytes.len() != ROWS * WIDTH {
            return Err(format!(
                "{} bytes, the table has {}",
                bytes.len(),
                ROWS * WIDTH
            ));
        }
        let mut out = SpendTable::empty();
        for (row, chunk) in out.rows.iter_mut().zip(bytes.chunks(WIDTH)) {
            row.copy_from_slice(chunk);
            let sum: u32 = row[..BUYS].iter().map(|&q| u32::from(q)).sum();
            if sum > 255 {
                return Err("the shares of a row are over 255".to_string());
            }
        }
        Ok(out)
    }

    /// The table of a packed text (`cg_pack`): all 0 for no text.
    pub fn decode(packed: &str) -> Result<SpendTable, String> {
        if packed.is_empty() {
            return Ok(SpendTable::empty());
        }
        let bytes = cg_pack::unpack(packed).ok_or("the table is not packed text")?;
        SpendTable::from_bytes(&bytes)
    }

    /// Whether `buy` is among the `k` most frequent kinds of `row` (ties by
    /// the kinds' order); always, for a row without purchases.
    pub fn among(&self, row: usize, buy: Buy, k: usize) -> bool {
        let shares = &self.rows[row][..BUYS];
        if shares.iter().all(|&s| s == 0) {
            return true;
        }
        let at = buy as usize;
        let ahead = (0..BUYS)
            .filter(|&o| shares[o] > shares[at] || (shares[o] == shares[at] && o < at))
            .count();
        ahead < k
    }
}
