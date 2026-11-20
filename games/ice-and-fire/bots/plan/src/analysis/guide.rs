//! `guide --positions FILE [--tries N]`: the strategic turn of E077 against
//! the search's own plan, on each position of a bank of game turns
//! `strategy::FROM_TURN` to `UNTIL_TURN`, with a fresh role memory.
//!
//! For each position, the two plans are each played, then the opponent's
//! answer (the search from its seat, `--tries` tries, its opening over),
//! and the state after is read from our side. One tab-separated line per
//! position and plan: name, ply, game turn, seat, plan (`search` or
//! `strategic`), its commands, frontline units, units we lost to the answer,
//! our cells less the enemy's after the answer, the evaluation after the
//! answer, and the objectives we hold after it.

use std::fmt::Write as _;

use ice_and_fire_engine::{answer_line, Command, State};

use super::{read_bank, ready};
use crate::eval::eval;
use crate::search::{think, Limit};
use crate::strategy::objectives::{counted, objectives};
use crate::strategy::{guide_with, Memory, FROM_TURN, UNTIL_TURN};

/// The opponent's answer to the state `after` (our turn played and ended):
/// its search with `tries` tries, its opening over.
fn answer(after: &State, tries: u64) -> Vec<Command> {
    let mut opening = crate::opening();
    opening.end();
    let limit = Limit {
        soft: None,
        hard: None,
        max_evals: tries,
    };
    think(after, limit, &mut opening).commands
}

/// Runs `guide` with its arguments (after the word `guide`).
pub fn run(args: &[String]) -> Result<String, String> {
    let mut positions = None;
    let mut tries: u64 = 1_000;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--positions" => {
                positions = Some(args.next().cloned().ok_or("--positions needs a value")?);
            }
            "--tries" => {
                tries = args
                    .next()
                    .and_then(|n| n.parse().ok())
                    .ok_or("--tries needs a number")?;
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    let path = positions.ok_or("guide needs --positions FILE")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let mut out = String::from(
        "name\tply\tgame_turn\tseat\tplan\tcommands\tfrontline\tunits_lost\tcells_lead\teval\tobjectives_held\n",
    );
    for position in read_bank(&text)? {
        if !(FROM_TURN..=UNTIL_TURN).contains(&position.game_turn) {
            continue;
        }
        let (state, mut opening) = match ready(&position) {
            Ok(ready) => ready,
            Err(err) => {
                eprintln!("left out: {err}");
                continue;
            }
        };
        let seat = state.to_act();
        let Some(guide) = guide_with(&state, &mut Memory::default(), 1, 1, 0, false) else {
            continue;
        };
        let frontline = guide
            .plan
            .iter()
            .filter(|c| matches!(c, Command::Move { .. }))
            .count();
        let limit = Limit {
            soft: None,
            hard: None,
            max_evals: tries,
        };
        let search = think(&state, limit, &mut opening).commands;
        let goals = counted(&objectives(&state, seat));
        for (name, plan) in [("search", &search), ("strategic", &guide.plan)] {
            let mut after = state.clone();
            for &command in plan.iter() {
                after.apply(command);
            }
            after.end_turn();
            if after.outcome().is_none() {
                for command in answer(&after, tries) {
                    after.apply(command);
                }
                after.end_turn();
            }
            // Units we had after our plan, less those left once the answer
            // and the start of our next turn (cut-off units die) are done.
            let mut ours = state.clone();
            for &command in plan.iter() {
                ours.apply(command);
            }
            let lost = ours.unit_count(seat).saturating_sub(after.unit_count(seat));
            let lead =
                i64::from(after.active(seat).count()) - i64::from(after.active(1 - seat).count());
            let held = goals
                .iter()
                .filter(|o| after.active(seat).test(o.cell))
                .count();
            let _ = writeln!(
                out,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                position.name,
                position.ply,
                position.game_turn,
                seat,
                name,
                answer_line(plan),
                frontline,
                lost,
                lead,
                eval(&after, seat),
                held
            );
        }
    }
    Ok(out)
}
