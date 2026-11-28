//! `terms`: the value of each term of the evaluation, before its weight
//! (`crate::eval::WEIGHTS`), on each position of a bank, for fitting the
//! weights on game outcomes (E116).
//!
//! ```text
//! terms --positions BANK [--every K]
//! ```
//!
//! One JSON line for every K-th position (default 1), for the side to act:
//! the position's `name`, its game `turn`, `finished` (the game is over),
//! `threat` (the opponent could take the HQ: the evaluation counts
//! `-WIN / 2`, which no weight touches), `eval` (the bot's evaluation) and
//! `terms`, the terms in the order of `crate::eval::WEIGHTS`, ours less
//! theirs. At weights of 100, for a position neither finished nor with the
//! threat, the terms add up to `eval`. A position that cannot be read is
//! reported on stderr and left out.

use std::fmt::Write as _;

use ice_and_fire_engine::State;
use ice_and_fire_tactics::eval::{evaluate_with_movers, pressure, side_parts, WIN};

use super::read_bank;
use crate::eval::*;

/// The names of the terms, in the order of [`WEIGHTS`].
pub const NAMES: [&str; TERMS] = [
    "gold",
    "income",
    "cells",
    "units",
    "towers",
    "pressure",
    "standing",
    "debt",
    "shape",
    "strong",
    "idle",
    "threatened",
    "mines",
    "wall",
    "goal",
    "voronoi",
    "flank",
];

/// The terms of the score of `seat` in a game not over, before the weights.
pub fn raw(state: &State, seat: usize) -> [i64; TERMS] {
    let other = 1 - seat;
    let movers = [
        movers_from(state, seat, MARGIN_MOVER_LEVEL),
        movers_from(state, other, MARGIN_MOVER_LEVEL),
    ];
    let (_, margins) = evaluate_with_movers(state, seat, movers);
    let (ours, theirs) = (side_parts(state, seat), side_parts(state, other));
    let exposed_ours = exposed(state, seat);
    let exposed_theirs = exposed(state, other);
    let threats = i64::from(exposed_theirs.count()) - i64::from(exposed_ours.count());
    let mut out = [0; TERMS];
    for i in 0..5 {
        out[i] = ours[i] - theirs[i];
    }
    out[5] = pressure(margins[0]) - pressure(margins[1]);
    out[6] = standing(margins[0]) - standing(margins[1]);
    out[7] = -2 * debt(state, seat) + 2 * debt(state, other);
    out[8] = shape(state, seat);
    out[9] = strong_units(state, seat) - strong_units(state, other);
    out[10] = -idle(state, seat) + idle(state, other);
    out[11] = THREAT_QUARTERS * threats / 4;
    out[12] = mines(state, seat, exposed_ours) - mines(state, other, exposed_theirs);
    out[13] = wall(state, seat) - wall(state, other);
    out[14] = goal(state, seat);
    out[15] = voronoi(state, seat);
    out[16] = flank(state, seat);
    out
}

/// Runs `terms` with its arguments (after the word `terms`).
pub fn run(args: &[String]) -> Result<String, String> {
    let mut positions = None;
    let mut every = 1usize;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let mut value = || {
            args.next()
                .cloned()
                .ok_or_else(|| format!("{arg} needs a value"))
        };
        match arg.as_str() {
            "--positions" => positions = Some(value()?),
            "--every" => every = value()?.parse().map_err(|_| "bad --every")?,
            other => return Err(format!("unknown argument {other}")),
        }
    }
    if every == 0 {
        return Err("--every must be at least 1".into());
    }
    let path = positions.ok_or("terms needs --positions FILE")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let mut out = String::new();
    for position in read_bank(&text)?.iter().step_by(every) {
        let state = match position.state() {
            Ok(state) => state,
            Err(err) => {
                eprintln!("left out: {}: {err}", position.name);
                continue;
            }
        };
        let seat = state.to_act();
        let finished = state.outcome().is_some();
        let value = eval(&state, seat);
        let terms = if finished {
            [0; TERMS]
        } else {
            raw(&state, seat)
        };
        let sum: i64 = terms.iter().sum();
        let threat = !finished && value - sum == -WIN / 2;
        let mut fields = String::new();
        for (name, term) in NAMES.iter().zip(terms) {
            if !fields.is_empty() {
                fields.push(',');
            }
            let _ = write!(fields, "\"{name}\":{term}");
        }
        let _ = writeln!(
            out,
            "{{\"name\":\"{}\",\"turn\":{},\"finished\":{finished},\"threat\":{threat},\"eval\":{value},\"terms\":{{{fields}}}}}",
            position.name, position.game_turn
        );
    }
    Ok(out)
}
