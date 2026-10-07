//! One match between two bot processes.

use std::fmt;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

use cg_core::rng::{Rng, SEED_ENV};
use cg_core::time::TIME_SCALE_ENV;
use serde::Serialize;

use crate::referee::{Answer, Outcome, Referee, SEATS};

/// How to start a bot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BotSpec {
    /// Name used in results.
    pub name: String,
    pub program: String,
    pub args: Vec<String>,
}

impl BotSpec {
    /// Parses `NAME=PROGRAM [ARG...]`. The command is split on whitespace, so
    /// paths with spaces are not supported.
    pub fn parse(spec: &str) -> Result<BotSpec, String> {
        let (name, command) = spec
            .split_once('=')
            .ok_or_else(|| format!("bot {spec:?} must look like NAME=COMMAND"))?;
        let mut words = command.split_whitespace().map(str::to_string);
        let program = words
            .next()
            .ok_or_else(|| format!("bot {spec:?} has no command"))?;
        let name = name.trim();
        if name.is_empty() {
            return Err(format!("bot {spec:?} has no name"));
        }
        Ok(BotSpec {
            name: name.to_string(),
            program,
            args: words.collect(),
        })
    }
}

/// Settings that apply to every match.
#[derive(Clone, Debug)]
pub struct MatchOptions {
    /// Multiplies the game's time limits. Above 1 tolerates slow or noisy
    /// machines; below 1 plays faster games; 1 reproduces CodinGame's
    /// limits. Bots receive the factor in `CG_TIME_SCALE` so they can scale
    /// their own budget.
    pub time_scale: f64,
    /// Added to every scaled time limit, and not told to bots: it absorbs
    /// the scheduling delays of a busy machine, which do not shrink with
    /// the time scale. Zero reproduces CodinGame's limits.
    pub time_tolerance: Duration,
    /// Turns after which the arena stops the game. It only guards against
    /// referee bugs; real games end long before.
    pub max_turns: u32,
    /// Lets bots write to the arena's stderr instead of discarding it.
    pub show_bot_stderr: bool,
}

impl Default for MatchOptions {
    fn default() -> Self {
        MatchOptions {
            time_scale: 1.0,
            time_tolerance: Duration::ZERO,
            max_turns: 10_000,
            show_bot_stderr: false,
        }
    }
}

/// Why a match ended.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EndReason {
    /// The rules ended the game.
    Finished,
    /// A bot did not answer within its time limit.
    Timeout { seat: usize, limit_ms: f64 },
    /// A bot exited or closed its output.
    Crash { seat: usize, detail: String },
    /// A bot's answer broke the rules.
    Invalid { seat: usize, reason: String },
    /// The arena stopped the game; it counts as a draw.
    Aborted { reason: String },
}

impl EndReason {
    /// The seat at fault, if a bot caused the end of the game.
    pub fn faulty_seat(&self) -> Option<usize> {
        match self {
            EndReason::Timeout { seat, .. }
            | EndReason::Crash { seat, .. }
            | EndReason::Invalid { seat, .. } => Some(*seat),
            EndReason::Finished | EndReason::Aborted { .. } => None,
        }
    }
}

/// What happened in one match.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MatchRecord {
    pub seed: u64,
    /// Bot names by seat.
    pub seats: [String; SEATS],
    /// The winning seat, or `None` for a draw.
    pub winner: Option<usize>,
    pub end: EndReason,
    pub turns: u32,
    pub max_answer_ms: [f64; SEATS],
    pub mean_answer_ms: [f64; SEATS],
}

/// The match could not be played at all.
#[derive(Debug)]
pub enum ArenaError {
    /// A bot's program could not be started.
    Spawn { bot: String, error: io::Error },
}

impl fmt::Display for ArenaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArenaError::Spawn { bot, error } => write!(f, "cannot start bot {bot:?}: {error}"),
        }
    }
}

impl std::error::Error for ArenaError {}

/// The seed a bot receives in `CG_SEED`, derived from the match seed, its
/// seat and its name. The name matters when a bot plays a copy of itself:
/// without it, the two games of a pair would mirror each other exactly.
pub fn bot_seed(match_seed: u64, seat: usize, name: &str) -> u64 {
    // FNV-1a hash of the name.
    let name_hash = name.bytes().fold(0xCBF2_9CE4_8422_2325_u64, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01B3)
    });
    Rng::new(match_seed ^ name_hash ^ (0xB07_5EED << (8 * seat))).next_u64()
}

/// Plays one match between `bots` (by seat) under `referee`.
pub fn run_match(
    referee: &mut dyn Referee,
    bots: [&BotSpec; SEATS],
    seed: u64,
    options: &MatchOptions,
) -> Result<MatchRecord, ArenaError> {
    let mut processes = Vec::with_capacity(SEATS);
    for (seat, spec) in bots.iter().enumerate() {
        let process = BotProcess::spawn(spec, bot_seed(seed, seat, &spec.name), options).map_err(
            |error| ArenaError::Spawn {
                bot: spec.name.clone(),
                error,
            },
        )?;
        processes.push(process);
    }

    let limits = referee.time_limits();
    let scale = |limit: Duration| limit.mul_f64(options.time_scale) + options.time_tolerance;
    let mut turns = 0;
    let (outcome, end) = 'game: loop {
        if let Some(outcome) = referee.outcome() {
            break (outcome, EndReason::Finished);
        }
        let seats = referee.players_to_act();
        if seats.is_empty() {
            let reason = "the referee has no player to act but no outcome".to_string();
            break (Outcome::Draw, EndReason::Aborted { reason });
        }
        if turns >= options.max_turns {
            let reason = format!("turn limit of {} reached", options.max_turns);
            break (Outcome::Draw, EndReason::Aborted { reason });
        }
        turns += 1;

        let mut answers = Vec::with_capacity(seats.len());
        for seat in seats {
            let process = &mut processes[seat];
            let first = process.answers == 0;
            let mut input = String::new();
            if first {
                input.push_str(&referee.initial_input(seat));
            }
            input.push_str(&referee.turn_input(seat));
            let limit = scale(if first {
                limits.first_answer
            } else {
                limits.later_answers
            });
            match process.ask(&input, referee.answer_lines(seat), limit) {
                Ok(lines) => answers.push(Answer { seat, lines }),
                Err(AskError::Timeout) => {
                    let limit_ms = limit.as_secs_f64() * 1000.0;
                    break 'game (lose(seat), EndReason::Timeout { seat, limit_ms });
                }
                Err(AskError::Closed) => {
                    let detail = process.exit_detail();
                    break 'game (lose(seat), EndReason::Crash { seat, detail });
                }
            }
        }
        if let Err(invalid) = referee.play(&answers) {
            let seat = invalid.seat;
            let reason = invalid.reason;
            break (lose(seat), EndReason::Invalid { seat, reason });
        }
    };

    let mut record = MatchRecord {
        seed,
        seats: [bots[0].name.clone(), bots[1].name.clone()],
        winner: match outcome {
            Outcome::Win(seat) => Some(seat),
            Outcome::Draw => None,
        },
        end,
        turns,
        max_answer_ms: [0.0; SEATS],
        mean_answer_ms: [0.0; SEATS],
    };
    for (seat, process) in processes.iter_mut().enumerate() {
        record.max_answer_ms[seat] = process.max_answer.as_secs_f64() * 1000.0;
        if process.answers > 0 {
            record.mean_answer_ms[seat] =
                process.total_answer.as_secs_f64() * 1000.0 / f64::from(process.answers);
        }
        process.stop();
    }
    Ok(record)
}

/// The outcome when the player in `seat` loses.
fn lose(seat: usize) -> Outcome {
    Outcome::Win(1 - seat)
}

enum AskError {
    Timeout,
    Closed,
}

struct BotProcess {
    child: Child,
    stdin: Option<ChildStdin>,
    lines: Receiver<String>,
    answers: u32,
    max_answer: Duration,
    total_answer: Duration,
}

impl BotProcess {
    fn spawn(spec: &BotSpec, seed: u64, options: &MatchOptions) -> io::Result<BotProcess> {
        let mut command = Command::new(&spec.program);
        command
            .args(&spec.args)
            .env(SEED_ENV, seed.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(if options.show_bot_stderr {
                Stdio::inherit()
            } else {
                Stdio::null()
            });
        if options.time_scale != 1.0 {
            command.env(TIME_SCALE_ENV, options.time_scale.to_string());
        }
        let mut child = command.spawn()?;
        let stdin = child.stdin.take();
        let stdout = child.stdout.take().expect("stdout is piped");
        let (sender, lines) = mpsc::channel();
        // Reading happens on a separate thread so the arena can wait for an
        // answer with a timeout. The thread ends when the bot's output closes.
        thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                let Ok(line) = line else { break };
                if sender.send(line).is_err() {
                    break;
                }
            }
        });
        Ok(BotProcess {
            child,
            stdin,
            lines,
            answers: 0,
            max_answer: Duration::ZERO,
            total_answer: Duration::ZERO,
        })
    }

    /// Sends `input` and waits for `count` answer lines within `limit`.
    fn ask(&mut self, input: &str, count: usize, limit: Duration) -> Result<Vec<String>, AskError> {
        let start = Instant::now();
        let stdin = self.stdin.as_mut().ok_or(AskError::Closed)?;
        stdin
            .write_all(input.as_bytes())
            .and_then(|()| stdin.flush())
            .map_err(|_| AskError::Closed)?;
        let deadline = start + limit;
        let mut lines = Vec::with_capacity(count);
        while lines.len() < count {
            let left = deadline.saturating_duration_since(Instant::now());
            match self.lines.recv_timeout(left) {
                Ok(line) => lines.push(line),
                Err(RecvTimeoutError::Timeout) => return Err(AskError::Timeout),
                Err(RecvTimeoutError::Disconnected) => return Err(AskError::Closed),
            }
        }
        let elapsed = start.elapsed();
        self.answers += 1;
        self.max_answer = self.max_answer.max(elapsed);
        self.total_answer += elapsed;
        Ok(lines)
    }

    /// Describes how the process ended, waiting briefly for it to exit.
    fn exit_detail(&mut self) -> String {
        let deadline = Instant::now() + Duration::from_millis(200);
        loop {
            match self.child.try_wait() {
                Ok(Some(status)) => return format!("exited with {status}"),
                Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(5)),
                Ok(None) => return "closed its output but kept running".to_string(),
                Err(error) => return format!("cannot read exit status: {error}"),
            }
        }
    }

    fn stop(&mut self) {
        self.stdin = None;
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Drop for BotProcess {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(all(test, unix))]
mod tests;
