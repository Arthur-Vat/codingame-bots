//! The screening report of a match (`ice-and-fire-review screening`): for
//! each bot of the records, its results by seat and what it did with its
//! gold, its towers and its units, and how long it thought. It reads the
//! arena's records directly, since the think times are only there, and
//! counts as the metrics of ADR 0044's report count: a tower or a unit is
//! one the rules accepted.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use cg_arena::runner::EndReason;
use ice_and_fire_engine::{Bits, Command, State};

use crate::replay::Game;

/// The game turns whose crowding is read.
pub const CROWD_TURNS: std::ops::RangeInclusive<u32> = 5..=12;

/// The crowding of one bot's units at the start of its ply of one game
/// turn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Crowd {
    pub units: usize,
    /// Own units next to a unit, summed over the units.
    pub adjacent: usize,
    /// Units with no free neutral cell next to them.
    pub shut: usize,
}

/// The crowding of the units of `seat` in `state`: the own units next to
/// each, and whether a free neutral cell (a valid cell owned by neither side
/// and holding no unit) is next to it, as `types::Typing::key` counts `adj`
/// and `free1`.
pub fn crowd(state: &State, seat: usize) -> Crowd {
    let occupied = state.units(0) | state.units(1);
    let mut out = Crowd::default();
    for cell in state.units(seat).iter() {
        let near = Bits::cell(cell).neighbours();
        let free = (near & state.map().valid)
            .and_not(state.owned(0))
            .and_not(state.owned(1))
            .and_not(occupied);
        out.units += 1;
        out.adjacent += (near & state.units(seat)).count() as usize;
        out.shut += usize::from(free.is_empty());
    }
    out
}

/// What one bot did in one game.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Side {
    pub bot: String,
    pub seat: usize,
    /// 1 for a win, 1/2 for a draw, 0 for a loss.
    pub points: f64,
    pub timeout: bool,
    pub crash: bool,
    pub towers: usize,
    pub mines: usize,
    /// Units trained by level.
    pub trained: [usize; 3],
    /// The gold left after each of its turns but the first, and its share
    /// of the gold at the start of the turn, in percent.
    pub kept: Vec<i64>,
    pub kept_share: Vec<i64>,
    /// Its answers' times in milliseconds: the first, then the others.
    pub first_ms: Option<f64>,
    pub ms: Vec<f64>,
    /// Its owned cells less the opponent's at game turn 12, read at the start
    /// of its ply; `None` if the game ended before.
    pub lead12: Option<i64>,
    /// Its units at game turns 7 and 12, read the same way.
    pub units7: Option<usize>,
    pub units12: Option<usize>,
    /// Its crowding at game turns 5 to 12 (`CROWD_TURNS`), by turn.
    pub crowd: [Option<Crowd>; 8],
}

/// The two sides of one replayed game.
pub fn sides(game: &Game) -> [Side; 2] {
    let record = &game.record;
    let mut out: [Side; 2] = [0, 1].map(|seat| Side {
        bot: record.players[seat].name.clone(),
        seat,
        points: match record.winner {
            Some(w) if w == seat => 1.0,
            Some(_) => 0.0,
            None => 0.5,
        },
        ..Side::default()
    });
    match &record.end {
        EndReason::Timeout { seat, .. } => out[*seat].timeout = true,
        EndReason::Crash { seat, .. } => out[*seat].crash = true,
        _ => {}
    }
    for step in &game.steps {
        let seat = step.state.to_act();
        let side = &mut out[seat];
        let turn = step.state.game_turn();
        let count = |bits: Bits| bits.count() as usize;
        match turn {
            7 => side.units7 = Some(count(step.state.units(seat))),
            12 => {
                side.units12 = Some(count(step.state.units(seat)));
                side.lead12 = Some(
                    i64::from(step.state.owned(seat).count())
                        - i64::from(step.state.owned(1 - seat).count()),
                );
            }
            _ => {}
        }
        if CROWD_TURNS.contains(&turn) {
            side.crowd[(turn - CROWD_TURNS.start()) as usize] = Some(crowd(&step.state, seat));
        }
        let mut sim = step.state.clone();
        let gold = sim.gold(seat);
        for &command in &step.commands {
            if !sim.apply(command) {
                continue;
            }
            match command {
                Command::BuildTower { .. } => side.towers += 1,
                Command::BuildMine { .. } => side.mines += 1,
                Command::Train { level, .. } => side.trained[usize::from(level) - 1] += 1,
                _ => {}
            }
        }
        if let Some(ms) = step.ms {
            if side.first_ms.is_none() {
                side.first_ms = Some(ms);
                continue;
            }
            side.ms.push(ms);
        }
        side.kept.push(sim.gold(seat).max(0));
        side.kept_share.push(if gold > 0 {
            sim.gold(seat).max(0) * 100 / gold
        } else {
            100
        });
    }
    out
}

fn mean(values: impl Iterator<Item = f64>) -> Option<f64> {
    let (sum, n) = values.fold((0.0, 0usize), |(s, n), v| (s + v, n + 1));
    (n > 0).then(|| sum / n as f64)
}

fn fixed(value: Option<f64>, digits: usize) -> String {
    value.map_or("–".to_string(), |v| format!("{v:.digits$}"))
}

/// The value at quantile `q` of `values`.
fn quantile(values: &mut [f64], q: f64) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(f64::total_cmp);
    Some(values[((values.len() - 1) as f64 * q).round() as usize])
}

/// The Markdown report of the games' sides, one row a bot (in name order).
pub fn report(games: &[[Side; 2]]) -> String {
    let mut by_bot: BTreeMap<&str, Vec<&Side>> = BTreeMap::new();
    for game in games {
        for side in game {
            by_bot.entry(side.bot.as_str()).or_default().push(side);
        }
    }
    let mut out = String::new();
    let _ = writeln!(out, "# Screening\n");
    let _ = writeln!(
        out,
        "{} games. Per bot: its score in all its games and by seat; towers, mines and units of each level it trained, a game (as the rules accepted them); the gold it kept unspent after its turns but the first, on average, and as a share of the gold it had; its faults; and its answers' times after the first (median, 99%, maximum) and the slowest first answer.\n",
        games.len()
    );
    let _ = writeln!(
        out,
        "| Bot | Games | Score | As red | As blue | Towers | Mines | Level 1 | Level 2 | Level 3 | Gold kept | Kept share | Timeouts | Crashes | Median ms | 99% ms | Max ms | Max first ms |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"
    );
    for (bot, sides) in &by_bot {
        let n = sides.len() as f64;
        let score = |seat: Option<usize>| {
            mean(
                sides
                    .iter()
                    .filter(|s| seat.is_none_or(|x| s.seat == x))
                    .map(|s| s.points),
            )
            .map(|v| 100.0 * v)
        };
        let per_game =
            |f: &dyn Fn(&Side) -> usize| sides.iter().map(|s| f(s)).sum::<usize>() as f64 / n;
        let mut ms: Vec<f64> = sides.iter().flat_map(|s| s.ms.iter().copied()).collect();
        let mut first: Vec<f64> = sides.iter().filter_map(|s| s.first_ms).collect();
        let _ = writeln!(
            out,
            "| {bot} | {} | {}% | {}% | {}% | {:.2} | {:.2} | {:.2} | {:.2} | {:.2} | {} | {}% | {} | {} | {} | {} | {} | {} |",
            sides.len(),
            fixed(score(None), 1),
            fixed(score(Some(0)), 1),
            fixed(score(Some(1)), 1),
            per_game(&|s| s.towers),
            per_game(&|s| s.mines),
            per_game(&|s| s.trained[0]),
            per_game(&|s| s.trained[1]),
            per_game(&|s| s.trained[2]),
            fixed(mean(sides.iter().flat_map(|s| s.kept.iter().map(|&v| v as f64))), 1),
            fixed(mean(sides.iter().flat_map(|s| s.kept_share.iter().map(|&v| v as f64))), 1),
            sides.iter().filter(|s| s.timeout).count(),
            sides.iter().filter(|s| s.crash).count(),
            fixed(quantile(&mut ms, 0.5), 1),
            fixed(quantile(&mut ms, 0.99), 1),
            fixed(quantile(&mut ms, 1.0), 1),
            fixed(quantile(&mut first, 1.0), 1),
        );
    }
    out.push_str(&progress(&by_bot));
    out
}

/// The tables of the progress measures, one row a bot: the median land lead
/// at game turn 12, the mean units at game turns 7 and 12, and for game
/// turns 5 to 12 the mean own units next to each unit and the share of units
/// with no free neutral neighbour, each read at the start of the bot's own
/// ply of the turn.
fn progress(by_bot: &BTreeMap<&str, Vec<&Side>>) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "\n## Progress\n\nPer bot, over its games that reached the turn: the median land lead at game turn 12 (its owned cells less the opponent's), its mean units at game turns 7 and 12; then, for each game turn 5 to 12, the mean own units next to each of its units and the share of its units with no free neutral cell next to them (a valid cell owned by neither side and holding no unit). Each is read at the start of the bot's own ply of the turn.\n"
    );
    let _ = writeln!(
        out,
        "| Bot | Lead at 12 (median) | Units at 7 | Units at 12 |\n| --- | ---: | ---: | ---: |"
    );
    for (bot, sides) in by_bot {
        let mut lead: Vec<f64> = sides
            .iter()
            .filter_map(|s| s.lead12)
            .map(|v| v as f64)
            .collect();
        let units = |f: fn(&Side) -> Option<usize>| {
            fixed(mean(sides.iter().filter_map(|s| f(s)).map(|v| v as f64)), 2)
        };
        let _ = writeln!(
            out,
            "| {bot} | {} | {} | {} |",
            fixed(quantile(&mut lead, 0.5), 1),
            units(|s| s.units7),
            units(|s| s.units12),
        );
    }
    let turns: Vec<u32> = CROWD_TURNS.collect();
    for (title, share) in [
        ("Own units next to each unit", false),
        ("Share of units with no free neutral neighbour", true),
    ] {
        let _ = writeln!(out, "\n{title}, by game turn:\n");
        let names: Vec<String> = turns.iter().map(|t| t.to_string()).collect();
        let _ = writeln!(out, "| Bot | {} |", names.join(" | "));
        let _ = writeln!(out, "| --- |{}", " ---: |".repeat(turns.len()));
        for (bot, sides) in by_bot {
            let cells: Vec<String> = (0..turns.len())
                .map(|i| {
                    let (units, num) = sides
                        .iter()
                        .filter_map(|s| s.crowd[i])
                        .fold((0usize, 0usize), |(u, n), c| {
                            (u + c.units, n + if share { c.shut } else { c.adjacent })
                        });
                    if units == 0 {
                        "–".to_string()
                    } else if share {
                        format!("{:.0}%", 100.0 * num as f64 / units as f64)
                    } else {
                        format!("{:.2}", num as f64 / units as f64)
                    }
                })
                .collect();
            let _ = writeln!(out, "| {bot} | {} |", cells.join(" | "));
        }
    }
    out
}

#[cfg(test)]
mod tests;
