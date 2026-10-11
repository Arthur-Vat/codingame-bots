//! The state of one game in progress.
//!
//! A [`GameState`] is a game's setup, the seats' players and the answers
//! played so far. Everything else (the frames, the legal moves, whose turn it
//! is) is rebuilt by replaying the game's referee, so the state cannot
//! disagree with the rules. The server wraps it with what it needs to run
//! bots (status, threads, processes); the browser's program keeps it in the
//! page and sends it with each request.
//!
//! # Ending a game by hand
//!
//! Only a human seat can end a game this way. A resignation is
//! `Resigned { seat }`; a human's flag falling is `Timeout { seat, limit_ms: 0 }`
//! (the front end runs the clocks). In both the other seat wins.
//!
//! # Records
//!
//! A bot's `Player.command` in a studio record is the release's name, not
//! the path of the compiled binary: the path is specific to the computer.

use std::time::Duration;

use cg_arena::live::LiveGame;
use cg_arena::record::{Player, PlayerKind, Record, RecordedAnswer, RECORD_FORMAT};
use cg_arena::referee::{GameSetup, Outcome, SEATS};
use cg_arena::runner::{bot_seed, EndReason};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use studio_game::{live_game, HumanMove, StudioGame};

/// The message of a game where several seats answer at once.
pub const SEVERAL_SEATS: &str = "games where several seats answer at once are not supported";

/// How a computer seat plays.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    /// A fixed number of iterations, from the measured speed of the
    /// release: a takeback can replay it exactly.
    Fixed,
    /// The release's own time budget, scaled by `think_ms / 100`: the mode
    /// of bots playing each other.
    Realtime,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Fixed => "fixed",
            Mode::Realtime => "realtime",
        }
    }
}

/// Who plays a seat.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SeatConfig {
    Human {
        name: String,
    },
    Bot {
        release: String,
        think_ms: u64,
        mode: Mode,
    },
}

impl SeatConfig {
    pub fn name(&self) -> &str {
        match self {
            SeatConfig::Human { name } => name,
            SeatConfig::Bot { release, .. } => release,
        }
    }

    pub fn is_bot(&self) -> bool {
        matches!(self, SeatConfig::Bot { .. })
    }
}

/// How a finished game ended.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameResult {
    /// The winning seat, `None` for a draw.
    pub winner: Option<usize>,
    pub end: EndReason,
}

/// The seat that is not `seat`.
pub fn other(seat: usize) -> usize {
    1 - seat
}

/// The `CG_TIME_SCALE` of a "realtime" seat: its think time over the 100 ms
/// of a CodinGame turn.
pub fn time_scale(think_ms: u64) -> f64 {
    think_ms as f64 / 100.0
}

/// What happens next in a game.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Next {
    /// The game is over.
    Over,
    /// A human must answer.
    Human { seat: usize },
    /// A bot must answer.
    Bot { seat: usize },
    /// Several seats, or none, must answer: the studio cannot play it.
    Unsupported,
}

/// One game: its setup, seats, answers and result.
#[derive(Clone, Debug)]
pub struct GameState {
    pub game: String,
    pub seed: u64,
    pub opening_plies: u32,
    pub opening_turns: usize,
    pub seats: [SeatConfig; SEATS],
    pub turns: Vec<Vec<RecordedAnswer>>,
    pub result: Option<GameResult>,
    /// The iterations of each "fixed" seat, known once measured.
    pub fixed_iters: [Option<u64>; SEATS],
}

impl GameState {
    /// A game at its start.
    pub fn new(
        game: &dyn StudioGame,
        seed: u64,
        opening_plies: u32,
        seats: [SeatConfig; SEATS],
    ) -> GameState {
        let setup = GameSetup {
            seed,
            opening_plies,
        };
        GameState {
            game: game.info().id.to_string(),
            seed,
            opening_plies,
            opening_turns: game.opening_turns(&setup),
            seats,
            turns: Vec::new(),
            result: None,
            fixed_iters: [None, None],
        }
    }

    pub fn setup(&self) -> GameSetup {
        GameSetup {
            seed: self.seed,
            opening_plies: self.opening_plies,
        }
    }

    /// Whether any seat is a computer in `mode`.
    pub fn has_bot_in_mode(&self, wanted: Mode) -> bool {
        self.seats
            .iter()
            .any(|seat| matches!(seat, SeatConfig::Bot { mode, .. } if *mode == wanted))
    }

    /// The game after the turns played, and how many turns that is. When the
    /// last turn is one the rules reject (a bot's invalid answer is kept in
    /// the record), the game stops before it.
    pub fn position(&self, game: &dyn StudioGame) -> (LiveGame, usize) {
        let setup = self.setup();
        match live_game(game, setup, &self.turns) {
            Ok(live) => (live, self.turns.len()),
            Err(error) => {
                let live = live_game(game, setup, &self.turns[..error.turn])
                    .expect("the turns before the first invalid one are valid");
                (live, error.turn)
            }
        }
    }

    /// How many answers the computer seats have given in the turns played:
    /// what a restarted bot has to replay.
    pub fn bot_answers(&self) -> usize {
        self.turns
            .iter()
            .flatten()
            .filter(|answer| self.seats.get(answer.seat).is_some_and(SeatConfig::is_bot))
            .count()
    }

    /// `seat` loses because of `end`.
    pub fn lose(&mut self, seat: usize, end: EndReason) {
        self.result = Some(GameResult {
            winner: Some(other(seat)),
            end,
        });
    }

    /// Looks at the position: sets the result when the rules say the game is
    /// over (unless it is set already), and says who acts next.
    pub fn settle(&mut self, game: &dyn StudioGame) -> Next {
        if self.result.is_some() {
            return Next::Over;
        }
        let (live, _) = self.position(game);
        if let Some(outcome) = live.outcome() {
            let winner = match outcome {
                Outcome::Win(seat) => Some(seat),
                Outcome::Draw => None,
            };
            self.result = Some(GameResult {
                winner,
                end: EndReason::Finished,
            });
            return Next::Over;
        }
        let seats = live.to_act();
        let [seat] = seats[..] else {
            return Next::Unsupported;
        };
        if self.seats[seat].is_bot() {
            Next::Bot { seat }
        } else {
            Next::Human { seat }
        }
    }

    /// Plays the answer of `seat`. A move the rules reject loses the game
    /// for `seat`, and the turn is kept in the record.
    pub fn apply_answer(&mut self, game: &dyn StudioGame, answer: RecordedAnswer) {
        let seat = answer.seat;
        let (mut live, _) = self.position(game);
        match live.play(vec![answer.clone()]) {
            Ok(()) => self.turns.push(vec![answer]),
            Err(invalid) => {
                self.turns.push(vec![answer]);
                self.lose(
                    seat,
                    EndReason::Invalid {
                        seat: invalid.seat,
                        reason: invalid.reason,
                    },
                );
            }
        }
    }

    /// The time `seat`'s bot has for its next answer.
    pub fn limit_for(&self, seat: usize, live: &LiveGame) -> Duration {
        match &self.seats[seat] {
            SeatConfig::Human { .. } => Duration::ZERO,
            SeatConfig::Bot {
                think_ms,
                mode: Mode::Fixed,
                ..
            } => Duration::from_millis(10_000u64.saturating_add(think_ms.saturating_mul(5))),
            SeatConfig::Bot {
                think_ms,
                mode: Mode::Realtime,
                ..
            } => live.time_limit(seat).mul_f64(time_scale(*think_ms)) + Duration::from_millis(50),
        }
    }

    /// The frames of the first `valid` turns; the start position if those
    /// turns do not replay.
    pub fn frames(&self, game: &dyn StudioGame, valid: usize) -> Vec<Value> {
        let setup = self.setup();
        game.frames(&setup, &self.turns[..valid])
            .or_else(|_| game.frames(&setup, &[]))
            .unwrap_or_default()
    }

    /// The moves a human in `seat` may play after the first `valid` turns.
    pub fn human_moves(&self, game: &dyn StudioGame, valid: usize, seat: usize) -> Vec<HumanMove> {
        game.human_moves(&self.setup(), &self.turns[..valid], seat)
            .unwrap_or_default()
    }

    /// The turns as the front end reads them.
    pub fn turns_json(&self) -> Vec<Vec<Value>> {
        self.turns
            .iter()
            .map(|turn| {
                turn.iter()
                    .map(|a| json!({"seat": a.seat, "lines": a.lines, "ms": a.ms}))
                    .collect()
            })
            .collect()
    }

    /// A seat as the front end reads it.
    pub fn seat_json(&self, seat: usize) -> Value {
        match &self.seats[seat] {
            SeatConfig::Human { name } => json!({"kind": "human", "name": name}),
            SeatConfig::Bot {
                release,
                think_ms,
                mode,
            } => json!({
                "kind": "bot",
                "name": release,
                "release": release,
                "think_ms": think_ms,
                "mode": mode.as_str(),
                "fixed_iters": self.fixed_iters[seat],
            }),
        }
    }

    /// The result as the front end reads it.
    pub fn result_json(&self) -> Value {
        self.result.as_ref().map_or(Value::Null, |result| {
            json!({
                "winner": result.winner,
                "end": serde_json::to_value(&result.end).unwrap_or(Value::Null),
            })
        })
    }

    /// The record of the game so far (format 1), made at `unix_time`. A game
    /// still going ends with `Aborted { reason: "unfinished" }` and no
    /// winner.
    pub fn record(&self, unix_time: u64) -> Record {
        let result = self.result.clone().unwrap_or_else(|| GameResult {
            winner: None,
            end: EndReason::Aborted {
                reason: "unfinished".to_string(),
            },
        });
        let players = std::array::from_fn(|seat| match &self.seats[seat] {
            SeatConfig::Human { name } => Player {
                name: name.clone(),
                kind: PlayerKind::Human,
                bot_seed: None,
                command: None,
                time_scale: 1.0,
                fixed_iters: None,
            },
            SeatConfig::Bot {
                release,
                think_ms,
                mode,
            } => Player {
                name: release.clone(),
                kind: PlayerKind::Bot,
                bot_seed: Some(bot_seed(self.seed, seat, release)),
                command: Some(release.clone()),
                time_scale: match mode {
                    Mode::Fixed => 1.0,
                    Mode::Realtime => time_scale(*think_ms),
                },
                fixed_iters: match mode {
                    Mode::Fixed => self.fixed_iters[seat],
                    Mode::Realtime => None,
                },
            },
        });
        Record {
            format: RECORD_FORMAT,
            game: self.game.clone(),
            seed: self.seed,
            opening_plies: self.opening_plies,
            unix_time,
            source: "studio".to_string(),
            players,
            turns: self.turns.clone(),
            end: result.end,
            winner: result.winner,
        }
    }
}
