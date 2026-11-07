//! `judge`: on each position of a bank that carries a `played` line, the plan
//! the search chooses against the plan a player played, both scored the way
//! the search scores a candidate plan (`search::judge`).
//!
//! The search runs as the bot's does, with `--tries` tries and no deadline,
//! so a run repeats on any machine, and with the opening over (as `bank`
//! and `speed` have it: `ready`). One line a position:
//!
//! ```text
//! NAME<TAB>PLY<TAB>ours=COMMANDS<TAB>ours_score=N<TAB>played_score=N<TAB>same=yes|first|no
//! ```
//!
//! `COMMANDS` is the answer line of the plan the search chose. `same` says
//! whether the played answer has the same commands (in any order), only the
//! same first command, or neither. A position without a `played` line prints
//! `played_score=-` and `same=-`. A position that cannot be read is
//! reported on stderr and left out.

use std::fmt::Write as _;

use ice_and_fire_engine::{answer_line, Command};
use ice_and_fire_tactics::eval::WIN;

use super::{read_bank, ready, Position};
use crate::eval::eval;
use crate::search::judge::{set_played, take};
use crate::search::{think, Limit};

/// The tries of a search when `--tries` is not given.
pub const TRIES: u64 = 600;

/// One command of an answer line, as the referee reads it.
fn parse(part: &str) -> Option<Command> {
    let words: Vec<&str> = part.split_whitespace().collect();
    let num = |i: usize| words.get(i).and_then(|w| w.parse::<u32>().ok());
    let small = |i: usize| num(i).and_then(|n| u8::try_from(n).ok());
    match words.first().copied()? {
        "MOVE" => Some(Command::Move {
            id: num(1)?,
            x: small(2)?,
            y: small(3)?,
        }),
        "TRAIN" => Some(Command::Train {
            level: small(1)?,
            x: small(2)?,
            y: small(3)?,
        }),
        "BUILD" => {
            let (x, y) = (small(2)?, small(3)?);
            match words.get(1).copied()? {
                "MINE" => Some(Command::BuildMine { x, y }),
                "TOWER" => Some(Command::BuildTower { x, y }),
                _ => None,
            }
        }
        _ => None,
    }
}

/// The commands of an answer line (`WAIT` and unreadable parts left out).
pub fn commands_of(line: &str) -> Vec<Command> {
    line.split(';')
        .filter_map(|part| parse(part.trim()))
        .collect()
}

/// The words of `commands` in a fixed order.
fn words(commands: &[Command]) -> Vec<String> {
    let mut out: Vec<String> = commands.iter().map(Command::to_text).collect();
    out.sort();
    out
}

/// `yes` if the plans have the same commands, `first` if they only start
/// with the same one, else `no`.
pub fn same(ours: &[Command], played: &[Command]) -> &'static str {
    if words(ours) == words(played) {
        "yes"
    } else if matches!((ours.first(), played.first()), (Some(a), Some(b)) if a == b) {
        "first"
    } else {
        "no"
    }
}

/// The result of one position.
pub struct Judgement {
    pub ours: Vec<Command>,
    pub ours_score: i64,
    pub played_score: Option<i64>,
    pub same: &'static str,
}

/// Judges `position` with `tries` tries.
pub fn judge(position: &Position, tries: u64) -> Result<Judgement, String> {
    let (state, mut opening) = ready(position)?;
    let played = position.played.as_deref().map(commands_of);
    set_played(played.clone());
    let limit = Limit {
        soft: None,
        hard: None,
        max_evals: tries,
    };
    let thought = think(&state, limit, &mut opening);
    let (ours_score, played_score) = match take() {
        Some(judged) => (judged.ours, judged.played),
        // The search stopped before its final choice: a lethal plan, which
        // wins. A played plan is then scored by the evaluation of its end.
        None => {
            let me = state.to_act();
            let played_score = played.as_deref().map(|commands| {
                let mut end = state.clone();
                for &command in commands {
                    end.apply(command);
                }
                eval(&end, me)
            });
            (WIN, played_score)
        }
    };
    let same = played
        .as_deref()
        .map_or("-", |played| same(&thought.commands, played));
    Ok(Judgement {
        ours: thought.commands,
        ours_score,
        played_score,
        same,
    })
}

/// The output line of a judged position.
pub fn line(position: &Position, j: &Judgement) -> String {
    let played = j.played_score.map_or("-".to_string(), |s| s.to_string());
    format!(
        "{}\t{}\tours={}\tours_score={}\tplayed_score={}\tsame={}",
        position.name,
        position.ply,
        answer_line(&j.ours),
        j.ours_score,
        played,
        j.same
    )
}

/// Runs `judge` with its arguments (after the word `judge`).
pub fn run(args: &[String]) -> Result<String, String> {
    let mut positions = None;
    let mut tries = TRIES;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let mut value = || {
            args.next()
                .cloned()
                .ok_or_else(|| format!("{arg} needs a value"))
        };
        match arg.as_str() {
            "--positions" => positions = Some(value()?),
            "--tries" => tries = value()?.parse().map_err(|_| "bad --tries")?,
            other => return Err(format!("unknown argument {other}")),
        }
    }
    let path = positions.ok_or("judge needs --positions FILE")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let bank = read_bank(&text)?;
    let mut out = String::new();
    for position in &bank {
        match judge(position, tries) {
            Ok(j) => {
                let _ = writeln!(out, "{}", line(position, &j));
            }
            Err(err) => eprintln!("left out: {err}"),
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests;
