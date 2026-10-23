//! What a game's replay gives the review: every ply, the phases and the
//! decisive moments.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// The sides, by seat.
pub const SIDES: [&str; 2] = ["red", "blue"];

/// A metric measured for each side at each ply.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metric {
    /// A short name without spaces or commas, used in tables and CSV.
    pub key: String,
    /// What it measures, in one sentence.
    pub meaning: String,
    /// A value that means "none" (for example no path exists), which the
    /// summary leaves out of its means.
    pub none: Option<i64>,
}

/// One ply: the position at the start of a player's turn, and what the
/// player then played.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ply {
    /// The game's turn number, as the game counts it.
    pub game_turn: u32,
    /// The seat to play.
    pub to_act: usize,
    /// The board, one line a row.
    pub board: Vec<String>,
    /// Each side's metrics, in the order of [`Trace::metrics`].
    pub metrics: [Vec<i64>; 2],
    /// What the player to act answered.
    pub commands: String,
    /// The gold, or other resource, the player spent this ply.
    pub spent: i64,
}

/// A part of the game.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    Early,
    Middle,
    End,
}

impl Phase {
    /// Its name in the game file.
    pub fn name(self) -> &'static str {
        match self {
            Phase::Early => "early game",
            Phase::Middle => "middle game",
            Phase::End => "endgame",
        }
    }
}

/// A decisive moment found by a rule.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Moment {
    /// The ply whose play caused it (or, for a lead change, the first ply
    /// that shows it).
    pub ply: usize,
    /// The kind, for example `large cut`; the summary groups by it.
    pub kind: String,
    /// The side it favours, if any.
    pub favours: Option<usize>,
    /// One sentence for the reviewer.
    pub text: String,
}

/// A replayed game.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Trace {
    /// A name for the game's files, for example the record's file stem.
    pub name: String,
    /// The players' names, by seat.
    pub players: [String; 2],
    /// The winning seat, or `None` for a draw.
    pub winner: Option<usize>,
    /// How the game ended, in a few words.
    pub end: String,
    /// What the reviewer must know to read the game: the rules in short, the
    /// board's legend. One line each.
    pub primer: Vec<String>,
    pub metrics: Vec<Metric>,
    /// The key of the metric whose difference between the sides is the
    /// lead.
    pub lead: String,
    pub plies: Vec<Ply>,
    /// The first ply of the middle game, if it was reached.
    pub middle: Option<usize>,
    /// The first ply of the endgame, if it was reached.
    pub endgame: Option<usize>,
    pub moments: Vec<Moment>,
}

impl Trace {
    /// The phase of `ply`.
    pub fn phase(&self, ply: usize) -> Phase {
        if self.endgame.is_some_and(|e| ply >= e) {
            Phase::End
        } else if self.middle.is_some_and(|m| ply >= m) {
            Phase::Middle
        } else {
            Phase::Early
        }
    }

    /// The index of the metric `key`.
    pub fn metric(&self, key: &str) -> Option<usize> {
        self.metrics.iter().position(|m| m.key == key)
    }

    /// The value of metric `key` for `side` at `ply`.
    pub fn value(&self, ply: usize, side: usize, key: &str) -> Option<i64> {
        let index = self.metric(key)?;
        self.plies.get(ply)?.metrics[side].get(index).copied()
    }

    /// What the summary needs besides the metrics.
    pub fn summary(&self) -> GameSummary {
        GameSummary {
            name: self.name.clone(),
            players: self.players.clone(),
            winner: self.winner,
            end: self.end.clone(),
            plies: self.plies.len(),
            last_turn: self.plies.last().map_or(0, |p| p.game_turn),
            middle: self.middle,
            endgame: self.endgame,
            lead: self.lead.clone(),
            metrics: self.metrics.iter().map(|m| m.key.clone()).collect(),
            none: self
                .metrics
                .iter()
                .filter_map(|m| m.none.map(|value| (m.key.clone(), value)))
                .collect(),
            moments: self.moments.clone(),
        }
    }
}

/// A game's facts for the batch summary, written next to its game file.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSummary {
    pub name: String,
    pub players: [String; 2],
    pub winner: Option<usize>,
    pub end: String,
    pub plies: usize,
    pub last_turn: u32,
    pub middle: Option<usize>,
    pub endgame: Option<usize>,
    pub lead: String,
    pub metrics: Vec<String>,
    /// The metrics that have a value meaning "none", with that value.
    #[serde(default)]
    pub none: BTreeMap<String, i64>,
    pub moments: Vec<Moment>,
}
