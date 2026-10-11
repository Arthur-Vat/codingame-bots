//! One game in progress.
//!
//! A session is a [`GameState`] (the game's setup, the seats' players and
//! the answers played so far, in `studio-core`) with what the server needs
//! to run bots. Everything else (the frames, the legal moves, whose turn it
//! is) is rebuilt by replaying the game's referee, so the session cannot
//! disagree with the rules. Bot processes live in the session while the game
//! runs; they are started and driven by [`crate::runner`].
//!
//! # Generation
//!
//! A takeback, an ended game and a deleted session invalidate the thread
//! that is waiting for a bot. Each of them raises `generation`, and a thread
//! checks that it still holds the generation it started with before it
//! stores anything.
//!
//! # Ending a game by hand and records
//!
//! See `studio_core::session`.

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use cg_arena::live::{LiveBot, LiveGame};
use cg_arena::record::{Record, RecordedAnswer};
use cg_arena::referee::SEATS;
use cg_arena::runner::EndReason;
use serde_json::{json, Value};
use studio_core::session::{GameState, Next, SEVERAL_SEATS};
use studio_game::StudioGame;

pub use studio_core::session::{other, time_scale, GameResult, Mode, SeatConfig};

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

/// How far a long step is. `total` is `None` when it is not known.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Progress {
    pub done: usize,
    pub total: Option<usize>,
}

pub struct Session {
    pub id: String,
    /// The game in progress: setup, seats, turns, result.
    pub core: GameState,
    pub status: Status,
    pub error: Option<String>,
    pub progress: Option<Progress>,
    pub generation: u64,
    /// The running bot of each seat, `None` while its thread has it.
    pub bots: [Option<LiveBot>; SEATS],
    pub touched: Instant,
}

impl Session {
    pub fn new(id: String, core: GameState) -> Session {
        Session {
            id,
            core,
            status: Status::WaitingHuman,
            error: None,
            progress: None,
            generation: 0,
            bots: [None, None],
            touched: Instant::now(),
        }
    }

    /// Stops the bots and the threads waiting for them.
    pub fn stop(&mut self) {
        self.generation += 1;
        self.bots = [None, None];
    }

    /// Ends the game with `result`.
    pub fn finish(&mut self, result: GameResult) {
        self.stop();
        self.core.result = Some(result);
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
        if self.core.result.is_some() {
            return false;
        }
        match self.core.settle(game) {
            Next::Over => {
                if let Some(result) = self.core.result.clone() {
                    self.finish(result);
                }
                false
            }
            Next::Unsupported => {
                self.fail(SEVERAL_SEATS.to_string());
                false
            }
            Next::Bot { .. } => {
                self.error = None;
                self.status = Status::BotThinking;
                true
            }
            Next::Human { .. } => {
                self.error = None;
                self.status = Status::WaitingHuman;
                false
            }
        }
    }

    /// Plays the answer of `seat`. A move the rules reject loses the game
    /// for `seat`, and the turn is kept in the record.
    pub fn apply_answer(&mut self, game: &dyn StudioGame, answer: RecordedAnswer) {
        self.core.apply_answer(game, answer);
        if let Some(result) = self.core.result.clone() {
            self.finish(result);
        }
    }

    /// The time `seat`'s bot has for its next answer.
    pub fn limit_for(&self, seat: usize, live: &LiveGame) -> Duration {
        self.core.limit_for(seat, live)
    }

    /// The JSON the front end polls.
    pub fn view(&self, game: &dyn StudioGame) -> Value {
        let core = &self.core;
        let (live, valid) = core.position(game);
        let over = core.result.is_some() || self.status == Status::Failed;
        let to_act = if over { Vec::new() } else { live.to_act() };
        let frames = core.frames(game, valid);
        let human_moves = match to_act[..] {
            [seat] if self.status == Status::WaitingHuman => core.human_moves(game, valid, seat),
            _ => Vec::new(),
        };
        json!({
            "id": self.id,
            "game": core.game,
            "seed": core.seed,
            "opening_plies": core.opening_plies,
            "seats": (0..SEATS).map(|seat| core.seat_json(seat)).collect::<Vec<_>>(),
            "status": self.status.as_str(),
            "error": self.error,
            "to_act": to_act,
            "opening_turns": core.opening_turns,
            "turns": core.turns_json(),
            "frames": frames,
            "human_moves": human_moves,
            "result": core.result_json(),
            "progress": self.progress.map(|progress| json!({
                "done": progress.done,
                "total": progress.total,
            })),
        })
    }

    /// The record of the game so far (format 1), made now.
    pub fn record(&self) -> Record {
        self.core.record(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |elapsed| elapsed.as_secs()),
        )
    }
}
