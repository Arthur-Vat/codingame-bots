//! The threads that start the bots of a session and play their turns.
//!
//! Nothing here holds a session's lock while it waits for a bot: a thread
//! takes the lock to read its inputs, lets go, waits, and takes the lock
//! again to store the result, if the session's generation is still the one
//! the thread started with (see [`crate::session`]).

use std::thread;
use std::time::Duration;

use cg_arena::live::{BotSettings, LiveBot, LiveGame, ReplayError, ResyncError};
use cg_arena::record::RecordedAnswer;
use cg_arena::referee::{GameSetup, SEATS};
use cg_arena::runner::{bot_seed, BotSpec, EndReason};
use studio_game::StudioGame;

use crate::session::{time_scale, Mode, Progress, SeatConfig, Session, Status};
use crate::speed::iterations;
use crate::state::{lock, SharedSession, State};

/// Starts the bots of a session, replays the game into them, and plays
/// their turns, on a new thread. The session's turns are what the bots are
/// brought back to: none for a new game, the kept ones after a takeback.
pub fn spawn_launch(state: &State, session: &SharedSession, generation: u64) {
    let (state, session) = (state.clone(), session.clone());
    thread::spawn(move || launch(&state, &session, generation));
}

/// Plays the turns of the bots on a new thread, until a human must act or
/// the game is over.
pub fn spawn_drive(state: &State, session: &SharedSession, generation: u64) {
    let (state, session) = (state.clone(), session.clone());
    thread::spawn(move || drive(&state, &session, generation));
}

/// Everything needed to start one bot.
struct Plan {
    seat: usize,
    release: String,
    spec: BotSpec,
    settings: BotSettings,
    limit: Duration,
}

fn launch(state: &State, session: &SharedSession, generation: u64) {
    match prepare(state, session, generation) {
        Ok(true) => drive(state, session, generation),
        Ok(false) => {}
        Err(message) => {
            let mut s = lock(session);
            if s.generation == generation {
                s.fail(message);
            }
        }
    }
}

/// Changes the session unless a newer generation replaced this thread.
/// Returns whether the change was made.
fn update(session: &SharedSession, generation: u64, change: impl FnOnce(&mut Session)) -> bool {
    let mut s = lock(session);
    if s.generation != generation {
        return false;
    }
    change(&mut s);
    true
}

/// Compiles the releases, measures the "fixed" ones, starts the bots and
/// replays the turns into them. `Ok(false)` when the thread was replaced.
fn prepare(state: &State, session: &SharedSession, generation: u64) -> Result<bool, String> {
    let (game_id, setup, seats, turns) = {
        let mut s = lock(session);
        if s.generation != generation {
            return Ok(false);
        }
        if s.status != Status::Rewinding {
            s.status = Status::Compiling;
        }
        (s.game.clone(), s.setup(), s.seats.clone(), s.turns.clone())
    };
    let game = state
        .game(&game_id)
        .ok_or_else(|| format!("unknown game {game_id:?}"))?;

    let mut plans = Vec::new();
    for (seat, config) in seats.iter().enumerate() {
        let SeatConfig::Bot {
            release,
            think_ms,
            mode,
        } = config
        else {
            continue;
        };
        let compiled = state.releases().compile(&game_id, release)?;
        let (scale, fixed_iters, limit) = match mode {
            Mode::Fixed => {
                if state.speed().cached(release, &compiled).is_none()
                    && !update(session, generation, |s| s.status = Status::Measuring)
                {
                    return Ok(false);
                }
                let rate = state.speed().rate(game, release, &compiled, &|done| {
                    update(session, generation, |s| {
                        s.progress = Some(Progress { done, total: None });
                    });
                })?;
                let fixed_iters = iterations(*think_ms, rate);
                if !update(session, generation, |s| {
                    s.fixed_iters[seat] = Some(fixed_iters);
                    s.progress = None;
                }) {
                    return Ok(false);
                }
                (
                    1.0,
                    Some(fixed_iters),
                    Duration::from_millis(10_000 + 5 * think_ms),
                )
            }
            // The limit of a "realtime" seat comes from the game's limits
            // when it plays; a takeback is refused for it, so a replay only
            // needs a generous one.
            Mode::Realtime => (time_scale(*think_ms), None, Duration::from_secs(60)),
        };
        plans.push(Plan {
            seat,
            release: release.clone(),
            spec: BotSpec {
                name: release.clone(),
                program: compiled.path.to_string_lossy().into_owned(),
                args: Vec::new(),
            },
            settings: BotSettings {
                seed: bot_seed(setup.seed, seat, release),
                time_scale: scale,
                fixed_iters,
                show_stderr: false,
            },
            limit,
        });
    }

    // Each bot answers its recorded turns again: that is the replay.
    let total: usize = plans
        .iter()
        .map(|plan| {
            turns
                .iter()
                .flatten()
                .filter(|answer| answer.seat == plan.seat)
                .count()
        })
        .sum();
    if total > 0
        && !update(session, generation, |s| {
            s.status = Status::Rewinding;
            s.progress = Some(Progress {
                done: 0,
                total: Some(total),
            });
        })
    {
        return Ok(false);
    }
    let mut done = 0;
    let mut started: [Option<LiveBot>; SEATS] = [None, None];
    for plan in &plans {
        let mut tick = || {
            done += 1;
            update(session, generation, |s| {
                s.progress = Some(Progress {
                    done,
                    total: Some(total),
                });
            })
        };
        match resync(game, plan, setup, &turns, &mut tick) {
            Ok(Some(bot)) => started[plan.seat] = Some(bot),
            Ok(None) => return Ok(false),
            Err(ResyncError::Diverged { .. }) => {
                return Err(format!(
                    "{} did not replay identically; takebacks are unavailable against it",
                    plan.release
                ));
            }
            Err(error) => return Err(format!("cannot restart {}: {error}", plan.release)),
        }
    }
    Ok(update(session, generation, |s| {
        s.bots = started;
        s.progress = None;
    }))
}

/// Like [`cg_arena::live::resync_bot`], which this follows step by step, but
/// reports each replayed answer to `tick` and stops (`Ok(None)`) when `tick`
/// returns false, so a takeback can show its progress and a deleted session
/// stops being replayed.
fn resync(
    game: &dyn StudioGame,
    plan: &Plan,
    setup: GameSetup,
    turns: &[Vec<RecordedAnswer>],
    tick: &mut dyn FnMut() -> bool,
) -> Result<Option<LiveBot>, ResyncError> {
    let mut bot = LiveBot::spawn(&plan.spec, &plan.settings).map_err(ResyncError::Spawn)?;
    let mut live = LiveGame::new(game.new_referee(&setup), setup);
    for (index, turn) in turns.iter().enumerate() {
        if let Some(recorded) = turn.iter().find(|answer| answer.seat == plan.seat) {
            let input = live.input_for(plan.seat);
            let (lines, _) = bot
                .ask(&input, live.answer_lines(plan.seat), plan.limit)
                .map_err(ResyncError::Bot)?;
            if lines != recorded.lines {
                return Err(ResyncError::Diverged {
                    turn: index,
                    expected: recorded.lines.clone(),
                    got: lines,
                });
            }
            if !tick() {
                return Ok(None);
            }
        }
        live.play(turn.clone()).map_err(|invalid| {
            ResyncError::Invalid(ReplayError {
                turn: index,
                invalid,
            })
        })?;
    }
    Ok(Some(bot))
}

/// What a bot must be asked.
struct Job {
    seat: usize,
    bot: LiveBot,
    input: String,
    lines: usize,
    limit: Duration,
}

/// Plays the bots' turns until a human must act or the game is over.
fn drive(state: &State, session: &SharedSession, generation: u64) {
    loop {
        let job = {
            let mut s = lock(session);
            if s.generation != generation {
                return;
            }
            let Some(game) = state.game(&s.game) else {
                return;
            };
            if !s.settle(game) {
                return;
            }
            let (live, _) = s.position(game);
            let seat = live.to_act()[0];
            let Some(bot) = s.bots[seat].take() else {
                s.fail(format!("the bot of seat {seat} is not running"));
                return;
            };
            Job {
                seat,
                bot,
                input: live.input_for(seat),
                lines: live.answer_lines(seat),
                limit: s.limit_for(seat, &live),
            }
        };
        let Job {
            seat,
            mut bot,
            input,
            lines,
            limit,
        } = job;
        let answer = bot.ask(&input, lines, limit);

        let mut s = lock(session);
        if s.generation != generation {
            return;
        }
        let Some(game) = state.game(&s.game) else {
            return;
        };
        s.bots[seat] = Some(bot);
        match answer {
            Ok((lines, ms)) => s.apply_answer(game, RecordedAnswer { seat, lines, ms }),
            Err(cg_arena::live::LiveError::Timeout) => s.lose(
                seat,
                EndReason::Timeout {
                    seat,
                    limit_ms: limit.as_secs_f64() * 1000.0,
                },
            ),
            Err(cg_arena::live::LiveError::Closed { detail }) => {
                s.lose(seat, EndReason::Crash { seat, detail });
            }
        }
    }
}
