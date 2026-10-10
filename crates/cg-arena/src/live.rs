//! Games and bots driven one turn at a time, for the studio
//! ([ADR 0025](../../../docs/adr/0025-studio.md)).
//!
//! The arena plays a whole match in one call. The studio needs the opposite:
//! a game that waits for each answer, possibly typed by a human, and a bot
//! that can be restarted and brought back to a position by replaying the
//! game into it.
//!
//! - [`LiveBot`]: one bot process, asked one answer at a time.
//! - [`LiveGame`]: a referee and the answers played so far.
//! - [`resync_bot`]: restarts a bot and replays a game into it.

use std::fmt;
use std::io;
use std::time::Duration;

use crate::record::RecordedAnswer;
use crate::referee::{Answer, GameSetup, InvalidAnswer, Outcome, Referee};
use crate::runner::{AskError, BotProcess, BotSpec};

/// How to run one bot process.
#[derive(Clone, Debug)]
pub struct BotSettings {
    /// The bot's `CG_SEED`.
    pub seed: u64,
    /// The factor applied to the game's time limits, given to the bot in
    /// `CG_TIME_SCALE` (left unset when it is 1).
    pub time_scale: f64,
    /// Sets the bot's `CG_FIXED_ITERS`, which replaces its time budget by a
    /// fixed number of search iterations. `None` leaves the variable as the
    /// parent process has it.
    pub fixed_iters: Option<u64>,
    /// Lets the bot write to this process's stderr instead of discarding it.
    pub show_stderr: bool,
}

/// Why a bot did not answer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiveError {
    /// No answer within the time limit.
    Timeout,
    /// The bot exited or closed its output.
    Closed { detail: String },
}

impl fmt::Display for LiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiveError::Timeout => write!(f, "the bot did not answer in time"),
            LiveError::Closed { detail } => write!(f, "the bot {detail}"),
        }
    }
}

impl std::error::Error for LiveError {}

/// A bot process that is asked for one answer at a time. Dropping it stops
/// the process.
///
/// A new bot knows nothing of a game in progress. To put one in a seat in
/// the middle of a game, use [`resync_bot`].
pub struct LiveBot {
    process: BotProcess,
}

impl LiveBot {
    pub fn spawn(spec: &BotSpec, settings: &BotSettings) -> io::Result<LiveBot> {
        Ok(LiveBot {
            process: BotProcess::spawn(spec, settings)?,
        })
    }

    /// Sends `input` and waits for `lines` answer lines within `limit`.
    /// Returns the lines and how long they took, in milliseconds.
    pub fn ask(
        &mut self,
        input: &str,
        lines: usize,
        limit: Duration,
    ) -> Result<(Vec<String>, f64), LiveError> {
        self.process
            .ask(input, lines, limit)
            .map_err(|error| match error {
                AskError::Timeout => LiveError::Timeout,
                AskError::Closed => LiveError::Closed {
                    detail: self.process.exit_detail(),
                },
            })
    }

    /// How many answers the bot has given. The first one has its own, longer
    /// time limit.
    pub fn answers(&self) -> u32 {
        self.process.answers
    }
}

/// One game in progress: a referee and the answers it has been played.
///
/// A referee cannot undo a move, and the `Referee` trait does not say what
/// state it is left in when it rejects answers. So after
/// [`play`](LiveGame::play) returns an error the game is finished for good:
/// [`to_act`](LiveGame::to_act) is empty, [`failure`](LiveGame::failure)
/// tells why, and nothing is recorded. To go back to an earlier position, or
/// to carry on after an invalid answer, build a new game with
/// [`replay`](LiveGame::replay) from a fresh referee and the turns played.
pub struct LiveGame {
    referee: Box<dyn Referee>,
    setup: GameSetup,
    turns: Vec<Vec<RecordedAnswer>>,
    failure: Option<InvalidAnswer>,
}

impl LiveGame {
    /// A game at its start. `referee` must be built from `setup`.
    pub fn new(referee: Box<dyn Referee>, setup: GameSetup) -> LiveGame {
        LiveGame {
            referee,
            setup,
            turns: Vec::new(),
            failure: None,
        }
    }

    /// A fresh `referee`, built by the caller from `setup`, brought to the
    /// position after `turns`. A takeback is a replay of a prefix of the
    /// turns.
    ///
    /// Arena records keep a final turn whose answers broke the rules. To
    /// show such a game, replay `turns[..error.turn]` when this fails.
    pub fn replay(
        referee: Box<dyn Referee>,
        setup: GameSetup,
        turns: &[Vec<RecordedAnswer>],
    ) -> Result<LiveGame, ReplayError> {
        let mut game = LiveGame::new(referee, setup);
        for (turn, answers) in turns.iter().enumerate() {
            game.play(answers.clone())
                .map_err(|invalid| ReplayError { turn, invalid })?;
        }
        Ok(game)
    }

    /// The seats that must answer now, in order; empty once the game is over.
    pub fn to_act(&self) -> Vec<usize> {
        if self.failure.is_some() || self.referee.outcome().is_some() {
            Vec::new()
        } else {
            self.referee.players_to_act()
        }
    }

    /// What the arena sends `seat` now: its initial input, if it has not
    /// answered yet, then the turn's input.
    ///
    /// "Has not answered" is decided from the game's turns, not from the
    /// bot process: a bot started in the middle of a game must be brought
    /// there with [`resync_bot`], or it would miss its initial input.
    pub fn input_for(&self, seat: usize) -> String {
        let mut input = String::new();
        if !self.has_answered(seat) {
            input.push_str(&self.referee.initial_input(seat));
        }
        input.push_str(&self.referee.turn_input(seat));
        input
    }

    /// How many lines `seat` must answer now.
    pub fn answer_lines(&self, seat: usize) -> usize {
        self.referee.answer_lines(seat)
    }

    /// The game's time limit for `seat`'s next answer, before any scaling:
    /// the first answer of a seat has a longer one.
    pub fn time_limit(&self, seat: usize) -> Duration {
        let limits = self.referee.time_limits();
        if self.has_answered(seat) {
            limits.later_answers
        } else {
            limits.first_answer
        }
    }

    /// Plays this turn's `answers`, in [`to_act`](LiveGame::to_act) order.
    /// On success they are appended to the turns.
    ///
    /// The answers' seats must be exactly `to_act()`, in order, and not
    /// empty; otherwise this returns an [`InvalidAnswer`] without calling
    /// the referee, and the game is unchanged. That also refuses a play
    /// after the game is over. When the referee itself rejects the answers
    /// nothing is recorded and the game is over: see the type's
    /// documentation.
    pub fn play(&mut self, answers: Vec<RecordedAnswer>) -> Result<(), InvalidAnswer> {
        if let Some(failure) = &self.failure {
            return Err(failure.clone());
        }
        self.check_seats(&answers)?;
        let played: Vec<Answer> = answers
            .iter()
            .map(|answer| Answer {
                seat: answer.seat,
                lines: answer.lines.clone(),
            })
            .collect();
        match self.referee.play(&played) {
            Ok(()) => {
                self.turns.push(answers);
                Ok(())
            }
            Err(invalid) => {
                self.failure = Some(invalid.clone());
                Err(invalid)
            }
        }
    }

    /// Checks that `answers` come from the seats that must act, in order.
    fn check_seats(&self, answers: &[RecordedAnswer]) -> Result<(), InvalidAnswer> {
        let expected = self.to_act();
        if expected.is_empty() {
            return Err(InvalidAnswer {
                seat: answers.first().map_or(0, |answer| answer.seat),
                reason: "the game is over, no answer is expected".to_string(),
            });
        }
        if answers.is_empty() {
            return Err(InvalidAnswer {
                seat: expected[0],
                reason: format!("no answers; seats {expected:?} must answer"),
            });
        }
        for (index, answer) in answers.iter().enumerate() {
            if expected.get(index) != Some(&answer.seat) {
                return Err(InvalidAnswer {
                    seat: answer.seat,
                    reason: format!(
                        "seat {} answered at position {index}, but seats {expected:?} must answer, in that order",
                        answer.seat
                    ),
                });
            }
        }
        if let Some(&seat) = expected.get(answers.len()) {
            return Err(InvalidAnswer {
                seat,
                reason: format!("seat {seat} has not answered; seats {expected:?} must answer"),
            });
        }
        Ok(())
    }

    /// Why the referee rejected the answers of a [`play`](LiveGame::play), if it did.
    pub fn failure(&self) -> Option<&InvalidAnswer> {
        self.failure.as_ref()
    }

    /// The result once the game is over, `None` before. After an invalid
    /// answer it is `None` too: see [`failure`](LiveGame::failure).
    pub fn outcome(&self) -> Option<Outcome> {
        if self.failure.is_some() {
            None
        } else {
            self.referee.outcome()
        }
    }

    /// The answers of every turn played.
    pub fn turns(&self) -> &[Vec<RecordedAnswer>] {
        &self.turns
    }

    pub fn setup(&self) -> GameSetup {
        self.setup
    }

    fn has_answered(&self, seat: usize) -> bool {
        self.turns
            .iter()
            .flatten()
            .any(|answer| answer.seat == seat)
    }
}

/// A turn of a replayed game that the rules reject.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayError {
    /// The index of the turn, from 0.
    pub turn: usize,
    pub invalid: InvalidAnswer,
}

impl fmt::Display for ReplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "turn {}: the answers of seat {} break the rules: {}",
            self.turn, self.invalid.seat, self.invalid.reason
        )
    }
}

impl std::error::Error for ReplayError {}

/// Why a bot could not be brought back to a position.
#[derive(Debug)]
pub enum ResyncError {
    /// The bot's program could not be started.
    Spawn(io::Error),
    /// The bot answered differently from the record.
    Diverged {
        /// The index of the turn, from 0.
        turn: usize,
        expected: Vec<String>,
        got: Vec<String>,
    },
    /// The bot did not answer.
    Bot(LiveError),
    /// The recorded turns break the rules.
    Invalid(ReplayError),
}

impl fmt::Display for ResyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResyncError::Spawn(error) => write!(f, "cannot start the bot: {error}"),
            ResyncError::Diverged {
                turn,
                expected,
                got,
            } => write!(
                f,
                "the bot answered {got:?} in turn {turn}, the record has {expected:?}"
            ),
            ResyncError::Bot(error) => write!(f, "{error}"),
            ResyncError::Invalid(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for ResyncError {}

/// Starts a bot and replays `turns` into it, so that it can take `seat`'s
/// next turn as if it had played them all. `referee` is a fresh one, built
/// from `setup`; it walks the turns alongside. Whenever `seat` answered, the
/// bot gets the input the arena would have sent and must answer the same
/// lines within `limit`. A bot that does not (the release is not
/// deterministic, or its seed or iterations differ) gives
/// [`ResyncError::Diverged`]. It only succeeds when the recorded lines came
/// from that same bot, started with the same seed and settings.
///
/// A bot started mid-game must always go through this function: whether a
/// seat gets its initial input is decided by [`LiveGame::input_for`] from
/// the game's turns, not by the process.
pub fn resync_bot(
    spec: &BotSpec,
    settings: &BotSettings,
    referee: Box<dyn Referee>,
    setup: GameSetup,
    turns: &[Vec<RecordedAnswer>],
    seat: usize,
    limit: Duration,
) -> Result<LiveBot, ResyncError> {
    let mut bot = LiveBot::spawn(spec, settings).map_err(ResyncError::Spawn)?;
    let mut game = LiveGame::new(referee, setup);
    for (index, turn) in turns.iter().enumerate() {
        if let Some(recorded) = turn.iter().find(|answer| answer.seat == seat) {
            let input = game.input_for(seat);
            let (lines, _) = bot
                .ask(&input, game.answer_lines(seat), limit)
                .map_err(ResyncError::Bot)?;
            if lines != recorded.lines {
                return Err(ResyncError::Diverged {
                    turn: index,
                    expected: recorded.lines.clone(),
                    got: lines,
                });
            }
        }
        game.play(turn.clone()).map_err(|invalid| {
            ResyncError::Invalid(ReplayError {
                turn: index,
                invalid,
            })
        })?;
    }
    Ok(bot)
}

#[cfg(all(test, unix))]
mod tests;
