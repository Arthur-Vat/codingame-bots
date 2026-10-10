//! Game records: one versioned JSON document per game, with every answer
//! the referee received, so the studio can replay the game and show any of
//! its positions ([ADR 0027](../../../docs/adr/0027-game-records.md)).
//!
//! [`Sampler`] chooses which games of a run to keep.

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::referee::SEATS;
use crate::runner::{bot_seed, BotSpec, EndReason, MatchOptions, MatchRecord};
use crate::tournament::GameRecord;

/// The version of the record format. A change to the format raises it.
pub const RECORD_FORMAT: u32 = 1;

/// The environment variable that replaces the bots' time budget by a fixed
/// iteration count. The arena does not depend on `cg-search`, which defines
/// it as `cg_search::budget::FIXED_ITERATIONS_ENV`.
const FIXED_ITERS_ENV: &str = "CG_FIXED_ITERS";

/// The lines one seat answered in one turn, as the referee received them.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecordedAnswer {
    pub seat: usize,
    pub lines: Vec<String>,
    /// How long the answer took, in milliseconds (the measurement behind
    /// the match's `max_answer_ms`).
    pub ms: f64,
}

/// Who sat in a seat.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlayerKind {
    Human,
    Bot,
}

/// A seat's player and the settings it played with.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub kind: PlayerKind,
    /// The `CG_SEED` the bot received; `None` for humans.
    pub bot_seed: Option<u64>,
    /// How the bot was started: its program and arguments, joined with
    /// single spaces; `None` for humans.
    pub command: Option<String>,
    /// The factor applied to the game's time limits.
    pub time_scale: f64,
    /// The `CG_FIXED_ITERS` the bot inherited, if it is set to a number.
    pub fixed_iters: Option<u64>,
}

/// One game: enough to rebuild every position by replaying the answers
/// with the game's referee.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Record {
    /// [`RECORD_FORMAT`].
    pub format: u32,
    /// The game's folder name, for example `uttt`.
    pub game: String,
    pub seed: u64,
    pub opening_plies: u32,
    /// Seconds since the Unix epoch when the record was made.
    pub unix_time: u64,
    /// Where the game comes from, for example `arena match`.
    pub source: String,
    pub players: [Player; SEATS],
    /// The answers of every turn, in the order the referee asked for them.
    pub turns: Vec<Vec<RecordedAnswer>>,
    pub end: EndReason,
    /// The winning seat, or `None` for a draw.
    pub winner: Option<usize>,
}

impl Record {
    /// The record of a game the arena played, made now. `bots` are the
    /// players by seat.
    pub fn from_match(
        game: &str,
        source: &str,
        opening_plies: u32,
        played: &MatchRecord,
        options: &MatchOptions,
        bots: [&BotSpec; SEATS],
    ) -> Record {
        let fixed_iters = fixed_iters(std::env::var(FIXED_ITERS_ENV).ok().as_deref());
        let unix_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |elapsed| elapsed.as_secs());
        Record {
            format: RECORD_FORMAT,
            game: game.to_string(),
            seed: played.seed,
            opening_plies,
            unix_time,
            source: source.to_string(),
            players: std::array::from_fn(|seat| Player {
                name: played.seats[seat].clone(),
                kind: PlayerKind::Bot,
                bot_seed: Some(bot_seed(played.seed, seat, &played.seats[seat])),
                command: Some(command_line(bots[seat])),
                time_scale: options.time_scale,
                fixed_iters,
            }),
            turns: played.recorded_turns.clone(),
            end: played.end.clone(),
            winner: played.winner,
        }
    }
}

/// The program and arguments of `bot`, joined with single spaces.
fn command_line(bot: &BotSpec) -> String {
    let mut words = vec![bot.program.as_str()];
    words.extend(bot.args.iter().map(String::as_str));
    words.join(" ")
}

/// The iteration count in `CG_FIXED_ITERS`, read as the bots read it.
fn fixed_iters(value: Option<&str>) -> Option<u64> {
    value?.trim().parse().ok().filter(|&n| n > 0)
}

/// Decides which games of a run to keep: every game ended by a fault, and
/// with a limit, a small mix of wins, draws and losses of the first bot.
#[derive(Clone, Debug)]
pub struct Sampler {
    limit: Option<usize>,
    kept: usize,
    /// Games kept by result for the first bot: wins, draws, losses.
    kept_by_class: [usize; 3],
}

impl Sampler {
    /// `limit` is the number of games without a fault to keep; `None` keeps
    /// every game. At most a third of the limit, rounded up, is kept for
    /// each result.
    pub fn new(limit: Option<usize>) -> Sampler {
        Sampler {
            limit,
            kept: 0,
            kept_by_class: [0; 3],
        }
    }

    /// Whether to keep `game`. Call it for each game in completion order.
    pub fn keep(&mut self, game: &GameRecord) -> bool {
        if game.game.end.faulty_seat().is_some() {
            return true;
        }
        let Some(limit) = self.limit else {
            return true;
        };
        let first_seat = usize::from(game.swapped);
        let class = match game.game.winner {
            Some(seat) if seat == first_seat => 0,
            None => 1,
            Some(_) => 2,
        };
        if self.kept >= limit || self.kept_by_class[class] >= limit.div_ceil(3) {
            return false;
        }
        self.kept += 1;
        self.kept_by_class[class] += 1;
        true
    }
}

#[cfg(test)]
mod tests;
