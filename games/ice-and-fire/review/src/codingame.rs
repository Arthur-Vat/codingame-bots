//! CodinGame logs of A Code of Ice and Fire turned into records
//! (`cg_arena::codingame`), each replayed with our engine and checked
//! against what CodinGame shows ([ADR 0056], decision 4).
//!
//! The checks, per game:
//! 1. the replay uses every answer: none is left once our game is over;
//! 2. the result agrees: the same winner (a draw when CodinGame's scores are
//!    equal), or, for a game stopped by a fault, our game is still on at the
//!    faulty seat's turn and the other seat wins;
//! 3. every command of every answer is one the parser reads, or `WAIT` or
//!    `MSG text` (the rules allow both and they change nothing);
//! 4. per ply, the commands our engine carries out are as many as the
//!    effects CodinGame lists (moved, trained, built) and the commands it
//!    refuses as many as the refusals (`Invalid action`, `stayed still`).
//!
//! [ADR 0056]: ../../../../docs/adr/0056-codingame-game-analysis.md

use std::fs;
use std::path::{Path, PathBuf};

use cg_arena::codingame::{read_log, to_record, Frame, FromCodinGame, Log};
use cg_arena::record::{Record, RecordedAnswer};
use cg_arena::runner::EndReason;
use ice_and_fire_engine::{Command, Outcome, State, SIZE};

use crate::replay::{replay, Game};
use crate::trace::parse;

/// The conversion of this game's logs.
pub struct IceAndFire;

impl FromCodinGame for IceAndFire {
    const GAME: &'static str = "ice-and-fire";

    /// The `seed=<number>` line of the options. The seed is a signed 64-bit
    /// number there; its bits are the record's seed, as the map generator
    /// takes them.
    fn seed(options: &str) -> Result<u64, String> {
        let value = options
            .lines()
            .find_map(|line| line.trim().strip_prefix("seed="))
            .ok_or("the game options have no seed")?;
        let seed: i64 = value
            .trim()
            .parse()
            .map_err(|_| format!("the seed {value:?} is not a 64-bit integer"))?;
        Ok(seed as u64)
    }

    /// Each frame with a `stdout` is one ply; the agents alternate, 0 first.
    /// The frame of a timeout is not one (see [`answer`]).
    fn answers(log: &Log) -> Result<Vec<Vec<RecordedAnswer>>, String> {
        let mut turns = Vec::new();
        for frame in &log.frames {
            let Some(line) = answer(frame) else { continue };
            let n = frame
                .frame
                .map_or_else(String::new, |f| format!(" (frame {f})"));
            let seat = frame
                .agent
                .ok_or_else(|| format!("answer {}{n} has no agent", turns.len() + 1))?;
            if seat != turns.len() % 2 {
                return Err(format!(
                    "answer {}{n} is from agent {seat}, expected agent {}",
                    turns.len() + 1,
                    turns.len() % 2
                ));
            }
            turns.push(vec![RecordedAnswer {
                seat,
                lines: vec![line.clone()],
                ms: 0.0,
            }]);
        }
        Ok(turns)
    }
}

/// The answer a frame holds. The frame of a timeout (`$N timeout!`) may carry
/// the `stdout` the player sent too late, cut or whole; the referee did not
/// use it, so it is not an answer.
fn answer(frame: &Frame) -> Option<&String> {
    let late = frame
        .summary
        .as_deref()
        .is_some_and(|text| text.lines().any(|line| line.trim().ends_with(" timeout!")));
    frame.stdout.as_ref().filter(|_| !late)
}

/// One way a game disagrees with CodinGame.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    /// The 1-based number of the answer, if the finding is about one.
    pub ply: Option<usize>,
    pub check: u8,
    pub detail: String,
}

/// What CodinGame shows after one answer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Effects {
    /// Moved, trained or built.
    pub done: usize,
    /// `Invalid action` and `stayed still`.
    pub refused: usize,
}

/// The effects listed after each answer, from the summaries: every line of
/// every summary after an answer and before the next one, for the seat that
/// answered (the `agent` of a frame is not reliable, the `$N` of the text
/// is).
pub fn effects(log: &Log) -> Vec<Effects> {
    let mut plies: Vec<Effects> = Vec::new();
    for frame in &log.frames {
        if answer(frame).is_some() {
            plies.push(Effects::default());
        }
        let (Some(ply), Some(summary)) = (plies.len().checked_sub(1), &frame.summary) else {
            continue;
        };
        let seat = ply % 2;
        for line in summary.lines() {
            let Some(text) = line.trim().strip_prefix(&format!("${seat}")) else {
                continue;
            };
            let effect = &mut plies[ply];
            if text.starts_with(" moved ")
                || text.starts_with(" trained ")
                || text.starts_with(" built ")
            {
                effect.done += 1;
            } else if text.starts_with(": Invalid action") || text.contains("stayed still") {
                effect.refused += 1;
            }
        }
    }
    plies
}

/// The parts of an answer: split at `;`, trimmed, the empty parts at the end
/// dropped as Java's `String.split` drops them (RULES.md, "Reading the
/// answer"); an empty line is one empty part.
fn parts(line: &str) -> Vec<&str> {
    let mut raw: Vec<&str> = line.split(';').collect();
    while raw.len() > 1 && raw.last() == Some(&"") {
        raw.pop();
    }
    raw.into_iter().map(str::trim).collect()
}

/// Whether the rules allow `part` although it changes nothing.
fn is_no_op(part: &str) -> bool {
    part == "WAIT" || part.strip_prefix("MSG ").is_some()
}

/// Whether the rules drop `command` while the answer is read, at the start
/// of the ply in `before` (RULES.md, "Reading the answer"): a cell off the
/// map or void, a level out of range, a unit that does not exist yet.
fn dropped_on_read(before: &State, command: Command) -> bool {
    let void = |x: u8, y: u8| {
        let (x, y) = (usize::from(x), usize::from(y));
        x >= SIZE || y >= SIZE || before.map().void.test(y * SIZE + x)
    };
    match command {
        Command::Wait => true,
        Command::Move { id, x, y } => {
            void(x, y)
                || !before
                    .units(before.to_act())
                    .iter()
                    .any(|cell| before.unit_id(cell) == Some(id))
        }
        Command::Train { level, x, y } => {
            void(x, y) || level == 0 || level > before.league().max_level()
        }
        Command::BuildMine { x, y } | Command::BuildTower { x, y } => void(x, y),
    }
}

/// The commands our engine carries out and refuses in one ply, by the loop
/// of `State::play_turn` (a command the rules drop when the answer is read
/// would fail anyway, and `apply` refuses it), stopping when the game ends.
/// CodinGame notes the commands dropped when the answer is read even when
/// the game ends before they would be carried out, so those still count as
/// refused.
fn carried_out(game: &Game, ply: usize) -> (usize, usize) {
    let step = &game.steps[ply];
    let mut state = step.state.clone();
    let (mut accepted, mut refused) = (0, 0);
    for (i, &command) in step.commands.iter().enumerate() {
        if state.outcome().is_some() {
            refused += step.commands[i..]
                .iter()
                .filter(|&&c| dropped_on_read(&step.state, c))
                .count();
            break;
        }
        if state.apply(command) {
            accepted += 1;
        } else {
            refused += 1;
        }
    }
    (accepted, refused)
}

/// What checking one game found, and how much it looked at.
pub struct Checked {
    pub findings: Vec<Finding>,
    pub plies: usize,
    pub accepted: usize,
    pub refused: usize,
}

/// Replays `record` (converted from `log`) and checks it against the log.
pub fn check(log: &Log, record: &Record) -> Checked {
    let game = replay(&log.game_id.to_string(), record.clone());
    let mut findings = Vec::new();
    let mut note = |ply: Option<usize>, check: u8, detail: String| {
        findings.push(Finding { ply, check, detail });
    };
    let answers = record.turns.len();

    // 1. Every answer is used.
    if game.steps.len() < answers {
        note(
            Some(game.steps.len() + 1),
            1,
            format!(
                "our game is over ({:?}) after {} of {answers} answers",
                game.end.outcome(),
                game.steps.len()
            ),
        );
    }

    // 2. The result.
    let ours = game.end.outcome();
    match &record.end {
        EndReason::Finished => {
            let theirs = record.winner;
            let ours_winner = match ours {
                Some(Outcome::Win(seat)) => Some(Some(seat)),
                Some(Outcome::Draw) => Some(None),
                None => None,
            };
            match ours_winner {
                None => note(None, 2, "our game is not over after the last answer".into()),
                Some(winner) if winner != theirs => note(
                    None,
                    2,
                    format!("winner is {winner:?} for us, {theirs:?} for CodinGame"),
                ),
                Some(_) => {}
            }
        }
        EndReason::Timeout { seat, .. } | EndReason::Crash { seat, .. } => {
            if ours.is_some() {
                note(
                    None,
                    2,
                    format!("our game is over ({ours:?}) before the fault of seat {seat}"),
                );
            } else if game.end.to_act() != *seat {
                note(
                    None,
                    2,
                    format!(
                        "seat {seat} failed but it is seat {}'s turn for us",
                        game.end.to_act()
                    ),
                );
            }
            if record.winner != Some(1 - seat) {
                note(
                    None,
                    2,
                    format!("seat {seat} failed but the winner is {:?}", record.winner),
                );
            }
        }
        other => note(
            None,
            2,
            format!("an end this check does not know: {other:?}"),
        ),
    }

    // 3. Every command is read.
    for (ply, turn) in record.turns.iter().enumerate() {
        for line in turn.iter().flat_map(|answer| &answer.lines) {
            for part in parts(line) {
                if parse(part).is_none() && !is_no_op(part) {
                    note(Some(ply + 1), 3, format!("not read: {part:?}"));
                }
            }
        }
    }

    // 4. Per ply, the carried-out and refused commands.
    let shown = effects(log);
    let (mut accepted_total, mut refused_total) = (0, 0);
    for ply in 0..game.steps.len() {
        let (accepted, refused) = carried_out(&game, ply);
        accepted_total += accepted;
        refused_total += refused;
        let theirs = shown.get(ply).copied().unwrap_or_default();
        if (accepted, refused) != (theirs.done, theirs.refused) {
            note(
                Some(ply + 1),
                4,
                format!(
                    "{} (seat {}): we carry out {accepted} and refuse {refused}, CodinGame shows {} done and {} refused",
                    record.turns[ply]
                        .iter()
                        .flat_map(|a| a.lines.iter().map(String::as_str))
                        .collect::<Vec<_>>()
                        .join(" | "),
                    ply % 2,
                    theirs.done,
                    theirs.refused
                ),
            );
        }
        // The loop above is the one of `play_turn`: both must end alike.
        let mut state = game.steps[ply].state.clone();
        for &command in &game.steps[ply].commands {
            if state.outcome().is_some() {
                break;
            }
            state.apply(command);
        }
        if state.outcome().is_none() {
            state.end_turn();
        }
        let next = game.steps.get(ply + 1).map_or(&game.end, |s| &s.state);
        if state.turn_input(0) != next.turn_input(0) || state.outcome() != next.outcome() {
            note(
                Some(ply + 1),
                4,
                "the check's loop and play_turn end differently".into(),
            );
        }
    }
    Checked {
        findings,
        plies: game.steps.len(),
        accepted: accepted_total,
        refused: refused_total,
    }
}

/// The outcome of one log.
pub enum Verdict {
    Checked(u64, Checked),
    Failed(String),
}

/// Converts, writes and checks the log at `path`; the record goes to
/// `out/<game_id>.json`.
pub fn run_log(path: &Path, out: &Path, kind: &str) -> Verdict {
    let log = match read_log(path) {
        Ok(log) => log,
        Err(error) => return Verdict::Failed(error),
    };
    let record = match to_record::<IceAndFire>(&log, kind) {
        Ok(record) => record,
        Err(error) => return Verdict::Failed(format!("{}: {error}", path.display())),
    };
    let file: PathBuf = out.join(format!("{}.json", log.game_id));
    let written = serde_json::to_string_pretty(&record)
        .map_err(|e| e.to_string())
        .and_then(|text| fs::write(&file, text + "\n").map_err(|e| e.to_string()));
    if let Err(error) = written {
        return Verdict::Failed(format!("{}: {error}", file.display()));
    }
    Verdict::Checked(log.game_id, check(&log, &record))
}

/// Everything `codingame` found, as a Markdown report.
pub fn report(read: usize, verdicts: &[Verdict]) -> String {
    let mut agreeing = 0;
    let mut disagreements = String::new();
    let mut failures = String::new();
    let (mut plies, mut accepted, mut refused) = (0, 0, 0);
    let mut converted = 0;
    let mut disagreeing = 0;
    for verdict in verdicts {
        match verdict {
            Verdict::Failed(error) => failures.push_str(&format!("- {error}\n")),
            Verdict::Checked(id, checked) => {
                converted += 1;
                plies += checked.plies;
                accepted += checked.accepted;
                refused += checked.refused;
                if checked.findings.is_empty() {
                    agreeing += 1;
                } else {
                    disagreeing += 1;
                }
                for finding in &checked.findings {
                    let ply = finding.ply.map_or("-".to_string(), |p| p.to_string());
                    disagreements.push_str(&format!(
                        "| {id} | {ply} | {} | {} |\n",
                        finding.check,
                        finding.detail.replace('|', "\\|")
                    ));
                }
            }
        }
    }
    let mut text = format!(
        "# CodinGame logs checked against our engine\n\n\
         | games read | converted | agreeing | disagreeing |\n|---:|---:|---:|---:|\n\
         | {read} | {converted} | {agreeing} | {disagreeing} |\n\n\
         {plies} plies replayed: our engine carried out {accepted} commands and refused {refused}.\n"
    );
    if !disagreements.is_empty() {
        text.push_str(&format!(
            "\n## Disagreements\n\n| game | ply | check | detail |\n|---|---:|---:|---|\n{disagreements}"
        ));
    }
    if !failures.is_empty() {
        text.push_str(&format!("\n## Conversion errors\n\n{failures}"));
    }
    text
}

/// Runs `codingame` over `logs`: the report, and whether every game agreed
/// and converted.
pub fn run(logs: &[PathBuf], out: &Path, kind: &str) -> Result<(String, bool), String> {
    fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let verdicts: Vec<Verdict> = logs.iter().map(|path| run_log(path, out, kind)).collect();
    let clean = verdicts.iter().all(|v| match v {
        Verdict::Checked(_, checked) => checked.findings.is_empty(),
        Verdict::Failed(_) => false,
    });
    Ok((report(logs.len(), &verdicts), clean))
}

#[cfg(test)]
mod tests;
