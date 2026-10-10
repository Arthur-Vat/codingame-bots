//! One game in progress.
//!
//! A session is a game's setup, the seats' players and the answers played so
//! far. Everything else (the frames, the legal moves, whose turn it is) is
//! rebuilt by replaying the game's referee, so the session cannot disagree
//! with the rules. Bot processes live in the session while the game runs;
//! they are started and driven by [`crate::runner`].
//!
//! # Generation
//!
//! A takeback, an ended game and a deleted session invalidate the thread
//! that is waiting for a bot. Each of them raises `generation`, and a thread
//! checks that it still holds the generation it started with before it
//! stores anything.
//!
//! # Ending a game by hand
//!
//! `EndReason` has no variant for a resignation, so a resignation is
//! `Aborted { reason: "seat N resigned" }` while the session's result names
//! the other seat as the winner. A human's flag falling is
//! `Timeout { seat, limit_ms: 0 }`: the front end runs the clocks.

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use cg_arena::live::{LiveBot, LiveGame};
use cg_arena::record::{Player, PlayerKind, Record, RecordedAnswer, RECORD_FORMAT};
use cg_arena::referee::{GameSetup, Outcome, SEATS};
use cg_arena::runner::{bot_seed, EndReason};
use serde::Deserialize;
use serde_json::{json, Value};
use studio_game::{live_game, StudioGame};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Compiling,
    Measuring,
    Rewinding,
    BotThinking,
    WaitingHuman,
    Over,
    Failed,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Status::Compiling => "compiling",
            Status::Measuring => "measuring",
            Status::Rewinding => "rewinding",
            Status::BotThinking => "bot_thinking",
            Status::WaitingHuman => "waiting_human",
            Status::Over => "over",
            Status::Failed => "failed",
        }
    }
}

/// How a finished game ended.
#[derive(Clone, Debug, PartialEq)]
pub struct GameResult {
    /// The winning seat, `None` for a draw.
    pub winner: Option<usize>,
    pub end: EndReason,
}

/// How far a long step is. `total` is `None` when it is not known.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Progress {
    pub done: usize,
    pub total: Option<usize>,
}

pub struct Session {
    pub id: String,
    pub game: String,
    pub seed: u64,
    pub opening_plies: u32,
    pub opening_turns: usize,
    pub seats: [SeatConfig; SEATS],
    pub turns: Vec<Vec<RecordedAnswer>>,
    pub status: Status,
    pub error: Option<String>,
    pub result: Option<GameResult>,
    pub progress: Option<Progress>,
    pub generation: u64,
    /// The running bot of each seat, `None` while its thread has it.
    pub bots: [Option<LiveBot>; SEATS],
    /// The iterations of each "fixed" seat, known once measured.
    pub fixed_iters: [Option<u64>; SEATS],
    pub touched: Instant,
}

/// The seat that is not `seat`.
pub fn other(seat: usize) -> usize {
    1 - seat
}

impl Session {
    pub fn new(
        id: String,
        game: &dyn StudioGame,
        seed: u64,
        opening_plies: u32,
        seats: [SeatConfig; SEATS],
    ) -> Session {
        let setup = GameSetup {
            seed,
            opening_plies,
        };
        Session {
            id,
            game: game.info().id.to_string(),
            seed,
            opening_plies,
            opening_turns: game.opening_turns(&setup),
            seats,
            turns: Vec::new(),
            status: Status::WaitingHuman,
            error: None,
            result: None,
            progress: None,
            generation: 0,
            bots: [None, None],
            fixed_iters: [None, None],
            touched: Instant::now(),
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
            .filter(|answer| self.seats[answer.seat].is_bot())
            .count()
    }

    /// Stops the bots and the threads waiting for them.
    pub fn stop(&mut self) {
        self.generation += 1;
        self.bots = [None, None];
    }

    /// Ends the game with `result`.
    pub fn finish(&mut self, result: GameResult) {
        self.stop();
        self.result = Some(result);
        self.status = Status::Over;
        self.progress = None;
        self.error = None;
    }

    /// Gives up on the session: it cannot go on.
    pub fn fail(&mut self, message: String) {
        self.stop();
        self.status = Status::Failed;
        self.progress = None;
        self.error = Some(message);
    }

    /// `seat` loses because of `end`.
    pub fn lose(&mut self, seat: usize, end: EndReason) {
        self.finish(GameResult {
            winner: Some(other(seat)),
            end,
        });
    }

    /// Looks at the position and sets the status: over, waiting for a
    /// human, or a bot to think. Returns whether a bot must act now.
    pub fn settle(&mut self, game: &dyn StudioGame) -> bool {
        if self.result.is_some() {
            return false;
        }
        let (live, _) = self.position(game);
        if let Some(outcome) = live.outcome() {
            let winner = match outcome {
                Outcome::Win(seat) => Some(seat),
                Outcome::Draw => None,
            };
            self.finish(GameResult {
                winner,
                end: EndReason::Finished,
            });
            return false;
        }
        let seats = live.to_act();
        let [seat] = seats[..] else {
            self.fail("games where several seats answer at once are not supported".to_string());
            return false;
        };
        self.error = None;
        if self.seats[seat].is_bot() {
            self.status = Status::BotThinking;
            true
        } else {
            self.status = Status::WaitingHuman;
            false
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
            } => Duration::from_millis(10_000 + 5 * think_ms),
            SeatConfig::Bot {
                think_ms,
                mode: Mode::Realtime,
                ..
            } => live.time_limit(seat).mul_f64(time_scale(*think_ms)) + Duration::from_millis(50),
        }
    }

    /// The JSON the front end polls.
    pub fn view(&self, game: &dyn StudioGame) -> Value {
        let setup = self.setup();
        let (live, valid) = self.position(game);
        let over = self.result.is_some() || self.status == Status::Failed;
        let to_act = if over { Vec::new() } else { live.to_act() };
        let frames = game
            .frames(&setup, &self.turns[..valid])
            .or_else(|_| game.frames(&setup, &[]))
            .unwrap_or_default();
        let human_moves = match to_act[..] {
            [seat] if self.status == Status::WaitingHuman => game
                .human_moves(&setup, &self.turns[..valid], seat)
                .unwrap_or_default(),
            _ => Vec::new(),
        };
        let turns: Vec<Vec<Value>> = self
            .turns
            .iter()
            .map(|turn| {
                turn.iter()
                    .map(|a| json!({"seat": a.seat, "lines": a.lines, "ms": a.ms}))
                    .collect()
            })
            .collect();
        json!({
            "id": self.id,
            "game": self.game,
            "seed": self.seed,
            "opening_plies": self.opening_plies,
            "seats": (0..SEATS).map(|seat| self.seat_json(seat)).collect::<Vec<_>>(),
            "status": self.status.as_str(),
            "error": self.error,
            "to_act": to_act,
            "opening_turns": self.opening_turns,
            "turns": turns,
            "frames": frames,
            "human_moves": human_moves,
            "result": self.result.as_ref().map(|result| json!({
                "winner": result.winner,
                "end": serde_json::to_value(&result.end).unwrap_or(Value::Null),
            })),
            "progress": self.progress.map(|progress| json!({
                "done": progress.done,
                "total": progress.total,
            })),
        })
    }

    fn seat_json(&self, seat: usize) -> Value {
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

    /// The record of the game so far (format 1). A game still going ends
    /// with `Aborted { reason: "unfinished" }` and no winner.
    pub fn record(&self) -> Record {
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
            unix_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |elapsed| elapsed.as_secs()),
            source: "studio".to_string(),
            players,
            turns: self.turns.clone(),
            end: result.end,
            winner: result.winner,
        }
    }
}

/// The `CG_TIME_SCALE` of a "realtime" seat: its think time over the 100 ms
/// of a CodinGame turn.
pub fn time_scale(think_ms: u64) -> f64 {
    think_ms as f64 / 100.0
}
