//! `explain --positions FILE [--tries N] [--cell X,Y]`: for each position,
//! the search's plan (at `--tries` tries, its opening over), and for each
//! level-1 training of the plan (or only the one on `--cell`), what the
//! evaluation gains by it: the plan's end state against the same plan
//! without that training, each term of the evaluation (`eval::terms`), at
//! the end of our turn (no reply) and after the opponent's answer (the
//! search from its seat). One tab-separated line per training: name, ply,
//! game turn, seat, the training, the plan, the total difference without
//! and with the answer, then each term's difference without the answer.

use std::fmt::Write as _;

use ice_and_fire_engine::{answer_line, Command, State};

use super::{read_bank, ready};
use crate::eval::{
    debt, eval, exposed, goal, idle, mines, movers_from, shape, standing, strong_units, voronoi,
    wall, MARGIN_MOVER_LEVEL, THREAT_QUARTERS,
};
use crate::search::{think, Limit};
use ice_and_fire_tactics::eval::evaluate_with_movers;

fn limit(tries: u64) -> Limit {
    Limit {
        soft: None,
        hard: None,
        max_evals: tries,
    }
}

/// The state after `plan` and our turn's end, then (with `answer`) the
/// opponent's answer by the search at `tries` and its turn's end.
fn after(state: &State, plan: &[Command], answer: bool, tries: u64) -> State {
    let mut s = state.clone();
    for &c in plan {
        s.apply(c);
    }
    if !answer {
        return s;
    }
    s.end_turn();
    if s.outcome().is_none() {
        let mut opening = crate::opening();
        opening.end();
        for c in think(&s, limit(tries), &mut opening).commands {
            s.apply(c);
        }
        s.end_turn();
    }
    s
}

/// Runs `explain` with its arguments (after the word `explain`).
pub fn run(args: &[String]) -> Result<String, String> {
    let mut positions = None;
    let mut tries: u64 = 1_000;
    let mut only: Option<(u8, u8)> = None;
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
            "--cell" => {
                let text = args.next().ok_or("--cell needs X,Y")?;
                let (x, y) = text.split_once(',').ok_or("--cell needs X,Y")?;
                only = Some((
                    x.parse().map_err(|_| "--cell needs X,Y")?,
                    y.parse().map_err(|_| "--cell needs X,Y")?,
                ));
            }
            other => return Err(format!("unknown argument {other}")),
        }
    }
    let path = positions.ok_or("explain needs --positions FILE")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let mut out = String::new();
    let mut header = false;
    for position in read_bank(&text)? {
        let (state, mut opening) = match ready(&position) {
            Ok(ready) => ready,
            Err(err) => {
                eprintln!("left out: {err}");
                continue;
            }
        };
        let seat = state.to_act();
        let plan = think(&state, limit(tries), &mut opening).commands;
        for (i, command) in plan.iter().enumerate() {
            let Command::Train { level: 1, x, y } = *command else {
                continue;
            };
            if only.is_some_and(|cell| cell != (x, y)) {
                continue;
            }
            let mut without = plan.clone();
            without.remove(i);
            let (a, b) = (
                after(&state, &plan, false, tries),
                after(&state, &without, false, tries),
            );
            let (ta, tb) = (terms(&a, seat), terms(&b, seat));
            if !header {
                out.push_str("name\tply\tgame_turn\tseat\ttraining\tplan\tgain\tgain_after_answer");
                for (name, _) in &ta {
                    let _ = write!(out, "\t{name}");
                }
                out.push('\n');
                header = true;
            }
            let answered = eval(&after(&state, &plan, true, tries), seat)
                - eval(&after(&state, &without, true, tries), seat);
            let _ = write!(
                out,
                "{}\t{}\t{}\t{}\t{x},{y}\t{}\t{}\t{}",
                position.name,
                position.ply,
                position.game_turn,
                seat,
                answer_line(&plan),
                eval(&a, seat) - eval(&b, seat),
                answered
            );
            for ((_, va), (_, vb)) in ta.iter().zip(&tb) {
                let _ = write!(out, "\t{}", va - vb);
            }
            out.push('\n');
        }
    }
    Ok(out)
}

/// The parts of [`eval`] for `seat` in `state`, by name, for the analysis
/// mode `explain` (their sum is [`eval`] when the game is not over).
pub fn terms(state: &State, seat: usize) -> Vec<(&'static str, i64)> {
    let movers = [
        movers_from(state, seat, MARGIN_MOVER_LEVEL),
        movers_from(state, 1 - seat, MARGIN_MOVER_LEVEL),
    ];
    let (score, margins) = evaluate_with_movers(state, seat, movers);
    let ours = exposed(state, seat);
    let theirs = exposed(state, 1 - seat);
    let threats = i64::from(theirs.count()) - i64::from(ours.count());
    vec![
        ("shared", score),
        ("standing", standing(margins[0]) - standing(margins[1])),
        ("removed", -crate::analysis::removed(state, seat)),
        ("debt", -2 * debt(state, seat) + 2 * debt(state, 1 - seat)),
        ("shape", shape(state, seat)),
        (
            "strong",
            strong_units(state, seat) - strong_units(state, 1 - seat),
        ),
        ("idle", -idle(state, seat) + idle(state, 1 - seat)),
        ("threats", THREAT_QUARTERS * threats / 4),
        (
            "mines",
            mines(state, seat, ours) - mines(state, 1 - seat, theirs),
        ),
        ("wall", wall(state, seat) - wall(state, 1 - seat)),
        ("goal", goal(state, seat)),
        ("voronoi", voronoi(state, seat)),
        ("sealed", crate::eval::sealed(state, seat)),
    ]
}
