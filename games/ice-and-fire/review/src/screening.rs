//! The screening report of a match (`ice-and-fire-review screening`): for
//! each bot of the records, its results by seat and what it did with its
//! gold, its towers and its units, and how long it thought. It reads the
//! arena's records directly, since the think times are only there, and
//! counts as the metrics of ADR 0044's report count: a tower or a unit is
//! one the rules accepted.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use cg_arena::runner::EndReason;
use ice_and_fire_engine::Command;

use crate::replay::Game;

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
    out
}

#[cfg(test)]
mod tests;
