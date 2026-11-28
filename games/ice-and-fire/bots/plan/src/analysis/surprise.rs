//! `surprise`: on each position of a bank that carries a `played` line and a
//! `reply` line, how the search scored the plan played, against what the
//! opponent's real reply did to it.
//!
//! The search runs as `judge` runs it (`--tries` tries, no deadline, the
//! opening over). The played plan's score is the worst evaluation, from the
//! side that played it, at its next start of turn, over the replies the
//! search models: its final memory of replies and the plan's own replies
//! (a chain to the HQ, the best cuts, a greedy turn and the opponent's own
//! climb; `search::judge`). That is what the search **expected**. The
//! opponent's real reply is applied the same way (`search::after_reply`:
//! the end of our turn, the reply's commands the rules accept, the end of
//! its turn) and evaluated from the same side: what **happened**. One line
//! a position:
//!
//! ```text
//! NAME<TAB>PLY<TAB>expected=N<TAB>actual=N<TAB>pass=N<TAB>modeled=K<TAB>modeled_best=N<TAB>same_reply=yes|no
//! ```
//!
//! `pass` is the evaluation if the opponent did nothing; `modeled` the
//! number of replies modeled, `modeled_best` the evaluation after the
//! modeled reply best for us; `same_reply` whether the real reply has the
//! same commands (in any order) as a modeled one. `actual` below
//! `expected` means the opponent found a reply worse for us than every
//! reply the search modeled. A position whose search ends before its
//! final choice (a lethal plan), or that lacks either line, is reported on
//! stderr and left out.

use std::fmt::Write as _;

use ice_and_fire_engine::{Command, State};

use super::judge::{commands_of, TRIES};
use super::{read_bank, ready, Position};
use crate::eval::eval;
use crate::search::judge::{set_played, take};
use crate::search::{think, Limit};

/// What the search expected of one position, and what happened.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Surprise {
    pub expected: i64,
    pub actual: i64,
    pub pass: i64,
    pub modeled: usize,
    pub modeled_best: i64,
    pub same_reply: bool,
}

/// `end`, our turn's end, then the opponent's `reply` (the commands the
/// rules accept), at our next start of turn: as the search applies a reply.
fn after(end: &State, reply: &[Command]) -> State {
    let mut next = end.clone();
    next.end_turn();
    for &command in reply {
        next.apply(command);
    }
    next.end_turn();
    next
}

/// The commands of `a` and `b` are the same, in any order.
fn same_commands(a: &[Command], b: &[Command]) -> bool {
    let words = |c: &[Command]| {
        let mut w: Vec<String> = c
            .iter()
            .filter(|c| **c != Command::Wait)
            .map(Command::to_text)
            .collect();
        w.sort();
        w
    };
    words(a) == words(b)
}

/// Judges `position` with `tries` tries.
pub fn surprise(position: &Position, tries: u64) -> Result<Surprise, String> {
    let played = position
        .played
        .as_deref()
        .ok_or_else(|| format!("{} {}: no `played` line", position.name, position.ply))?;
    let reply = position
        .reply
        .as_deref()
        .ok_or_else(|| format!("{} {}: no `reply` line", position.name, position.ply))?;
    let (state, mut opening) = ready(position)?;
    let me = state.to_act();
    set_played(Some(commands_of(played)));
    let limit = Limit {
        soft: None,
        hard: None,
        max_evals: tries,
        deep: None,
    };
    think(&state, limit, &mut opening);
    let judged = take().ok_or_else(|| {
        format!(
            "{} {}: the search stopped before its final choice (a lethal plan)",
            position.name, position.ply
        )
    })?;
    let (Some(expected), Some(end)) = (judged.played, judged.played_end) else {
        return Err(format!(
            "{} {}: the played plan was not scored",
            position.name, position.ply
        ));
    };
    let reply = commands_of(reply);
    let actual = eval(&after(&end, &reply), me);
    let pass = eval(&after(&end, &[]), me);
    let modeled_best = judged.modeled.iter().map(|(_, d)| *d).max().unwrap_or(pass);
    let same_reply = judged
        .modeled
        .iter()
        .any(|(commands, _)| same_commands(commands, &reply));
    Ok(Surprise {
        expected,
        actual,
        pass,
        modeled: judged.modeled.len(),
        modeled_best,
        same_reply,
    })
}

/// The output line of a judged position.
pub fn line(position: &Position, s: &Surprise) -> String {
    format!(
        "{}\t{}\texpected={}\tactual={}\tpass={}\tmodeled={}\tmodeled_best={}\tsame_reply={}",
        position.name,
        position.ply,
        s.expected,
        s.actual,
        s.pass,
        s.modeled,
        s.modeled_best,
        if s.same_reply { "yes" } else { "no" }
    )
}

/// Runs `surprise` with its arguments (after the word `surprise`).
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
    let path = positions.ok_or("surprise needs --positions FILE")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let bank = read_bank(&text)?;
    let mut out = String::new();
    for position in &bank {
        match surprise(position, tries) {
            Ok(s) => {
                let _ = writeln!(out, "{}", line(position, &s));
            }
            Err(err) => eprintln!("left out: {err}"),
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests;
