//! `guide --positions FILE [--tries N] [--guard 0|1|2] [--fill-behind]
//! [--safe-step] [--back-only]`: the strategic turn of E077 against
//! the search's own plan, on each position of a bank of game turns
//! `strategy::FROM_TURN` to `UNTIL_TURN`, with a fresh role memory.
//!
//! For each position, the two plans are each played, then the opponent's
//! answer (the search from its seat, `--tries` tries, its opening over),
//! and the state after is read from our side. One tab-separated line per
//! position and plan: name, ply, game turn, seat, plan (`search`,
//! `strategic`, or `completed`: the strategic turn then the search's plan
//! from there), its commands, frontline units, units we lost to the answer,
//! our cells less the enemy's after the answer, the evaluation after the
//! answer, the objectives we hold after it, our units killed and cut off,
//! the gold the plan spent, the cells its moves left that the enemy then
//! holds, and the cells of ours the enemy holds after its answer.

use std::fmt::Write as _;

use ice_and_fire_engine::{answer_line, Command, State};

use super::{read_bank, ready};
use crate::eval::eval;
use crate::search::{think, Limit};
use crate::strategy::objectives::{counted, objectives};
use crate::strategy::{guide_with, Memory, Settings, FROM_TURN, UNTIL_TURN};

/// The opponent's answer to the state `after` (our turn played and ended):
/// its search with `tries` tries, its opening over.
fn answer(after: &State, tries: u64) -> Vec<Command> {
    let mut opening = crate::opening();
    opening.end();
    let limit = Limit {
        soft: None,
        hard: None,
        max_evals: tries,
        deep: None,
    };
    think(after, limit, &mut opening).commands
}

/// Runs `guide` with its arguments (after the word `guide`).
pub fn run(args: &[String]) -> Result<String, String> {
    let mut positions = None;
    let mut tries: u64 = 1_000;
    let mut settings = Settings {
        mode: 1,
        ..Settings::default()
    };
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
            "--guard" => {
                settings.rules.guard = args
                    .next()
                    .and_then(|n| n.parse().ok())
                    .ok_or("--guard needs 0, 1 or 2")?;
            }
            "--fill-behind" => settings.rules.fill_behind = true,
            "--safe-step" => settings.rules.safe_step = true,
            "--back-only" => settings.back_only = true,
            other => return Err(format!("unknown argument {other}")),
        }
    }
    let path = positions.ok_or("guide needs --positions FILE")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let mut out = String::from(
        "name\tply\tgame_turn\tseat\tplan\tcommands\tfrontline\tunits_lost\tcells_lead\teval\tobjectives_held\tkilled\tcut_off\tspent\tvacated_taken\ttaken\n",
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
        let Some(guide) = guide_with(&state, &mut Memory::default(), settings) else {
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
            deep: None,
        };
        let search = think(&state, limit, &mut opening).commands;
        let goals = counted(&objectives(&state, seat));
        // The strategic turn, then the search's plan for the rest of the
        // turn from there (as E077's forced first climb).
        let mut mid = state.clone();
        for &command in &guide.plan {
            mid.apply(command);
        }
        let mut completed = guide.plan.clone();
        let mut rest_opening = crate::opening();
        rest_opening.end();
        completed.extend(think(&mid, limit, &mut rest_opening).commands);
        for (name, plan) in [
            ("search", &search),
            ("strategic", &guide.plan),
            ("completed", &completed),
        ] {
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
            // Of the units lost, those whose cell the enemy took (killed),
            // the others being cut off.
            let missing = ours.units(seat).and_not(after.units(seat));
            let killed = (missing & after.owned(1 - seat)).count();
            let cut = missing.count() - killed;
            let spent = state.gold(seat) - ours.gold(seat);
            // Cells our moves left that the enemy holds after its answer.
            let vacated = plan
                .iter()
                .filter_map(|c| match *c {
                    Command::Move { id, .. } => state
                        .units(seat)
                        .iter()
                        .find(|&cell| state.unit_id(cell) == Some(id)),
                    _ => None,
                })
                .filter(|&cell| after.owned(1 - seat).test(cell))
                .count();
            // Cells the enemy holds after its answer that were ours before.
            let taken = (state.owned(seat) & after.owned(1 - seat)).count();
            let lead =
                i64::from(after.active(seat).count()) - i64::from(after.active(1 - seat).count());
            let held = goals
                .iter()
                .filter(|o| after.active(seat).test(o.cell))
                .count();
            let _ = writeln!(
                out,
                "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
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
                held,
                killed,
                cut,
                spent,
                vacated,
                taken
            );
        }
    }
    Ok(out)
}
