//! A metrics report of a large batch (ADR 0044): every metric of every game
//! followed over the turns, by code alone, for a model to interpret.
//!
//! A game's value of a metric at turn `t` for a side is the one at the start
//! of that side's own ply of turn `t`. The report says, for each metric,
//! from which turn it tells the winner, how strongly at a few turns, and
//! how the two bots of the batch differ; [`curves`] gives the same numbers
//! turn by turn, for charts.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use serde::Serialize;

use crate::summary::Game;

/// The turns at which the report's tables look.
pub const CHECKPOINTS: [usize; 6] = [5, 8, 10, 12, 15, 20];
/// The share of games (in percent, a tie counting half) at which a metric
/// is said to tell the winner: the winner's value higher in at least this
/// share, or lower in at least this share.
pub const TELLS: usize = 70;
/// The fewest decided games still running at a turn for that turn to count.
pub const MIN_GAMES: usize = 30;
/// The last turn followed.
pub const MAX_TURN: usize = 40;

/// The value of metric `index` for `side` at the start of its ply of turn
/// `turn` (1 first), unless the game had ended or the value is `none`, the
/// metric's value meaning "none".
fn at(game: &Game, none: Option<i64>, index: usize, turn: usize, side: usize) -> Option<i64> {
    let row = game.rows.get(2 * (turn - 1) + side)?;
    if row.to_act != side {
        return None;
    }
    let value = *row.values[side].get(index)?;
    (none != Some(value)).then_some(value)
}

/// Means and counts of one metric at one turn.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Point {
    pub turn: usize,
    /// Decided games still running at this turn.
    pub games: usize,
    /// Games in which the winner's value was higher, lower and equal.
    pub higher: usize,
    pub lower: usize,
    pub equal: usize,
    pub winner_mean: Option<f64>,
    pub loser_mean: Option<f64>,
    /// Each bot's mean, in the order of [`Curves::bots`].
    pub bot_means: Vec<Option<f64>>,
}

impl Point {
    /// The share of games in which the winner's value was higher, a tie
    /// counting half, in percent.
    pub fn higher_share(&self) -> Option<usize> {
        let n = self.games;
        (n > 0).then(|| (self.higher * 200 + self.equal * 100 + n) / (2 * n))
    }

    /// How strongly the metric tells the winner here: how far the higher
    /// share is from 50, in points.
    pub fn strength(&self) -> usize {
        self.higher_share().map_or(0, |p| p.abs_diff(50))
    }
}

/// Every metric's points, turn by turn.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Curves {
    pub bots: Vec<String>,
    pub metrics: Vec<String>,
    /// For each metric, its points at turns 1 to [`MAX_TURN`].
    pub points: Vec<Vec<Point>>,
}

fn mean(sum: i64, n: usize) -> Option<f64> {
    (n > 0).then(|| sum as f64 / n as f64)
}

/// The curves of `games`, all with the same metrics: those of the first.
pub fn curves(games: &[Game]) -> Curves {
    let Some(first) = games.first() else {
        return Curves::default();
    };
    let metrics = first.summary.metrics.clone();
    let mut bots: Vec<String> = games
        .iter()
        .flat_map(|g| g.summary.players.iter().cloned())
        .collect();
    bots.sort();
    bots.dedup();
    let games: Vec<&Game> = games
        .iter()
        .filter(|g| g.summary.metrics == metrics)
        .collect();
    let mut points = Vec::new();
    for (index, key) in metrics.iter().enumerate() {
        let mut row = Vec::new();
        for turn in 1..=MAX_TURN {
            let mut p = Point {
                turn,
                bot_means: vec![None; bots.len()],
                ..Point::default()
            };
            let (mut win_sum, mut win_n, mut lose_sum, mut lose_n) = (0, 0, 0, 0);
            let mut bot_sums = vec![(0i64, 0usize); bots.len()];
            for game in &games {
                let none = game.summary.none.get(key).copied();
                let values = [0, 1].map(|side| at(game, none, index, turn, side));
                for (side, value) in values.iter().enumerate() {
                    if let Some(v) = value {
                        let bot = &game.summary.players[side];
                        if let Some(b) = bots.iter().position(|x| x == bot) {
                            bot_sums[b].0 += v;
                            bot_sums[b].1 += 1;
                        }
                    }
                }
                let Some(w) = game.summary.winner else {
                    continue;
                };
                let (Some(win), Some(lose)) = (values[w], values[1 - w]) else {
                    continue;
                };
                p.games += 1;
                win_sum += win;
                win_n += 1;
                lose_sum += lose;
                lose_n += 1;
                match win.cmp(&lose) {
                    std::cmp::Ordering::Greater => p.higher += 1,
                    std::cmp::Ordering::Less => p.lower += 1,
                    std::cmp::Ordering::Equal => p.equal += 1,
                }
            }
            p.winner_mean = mean(win_sum, win_n);
            p.loser_mean = mean(lose_sum, lose_n);
            p.bot_means = bot_sums.iter().map(|&(s, n)| mean(s, n)).collect();
            row.push(p);
        }
        points.push(row);
    }
    Curves {
        bots,
        metrics,
        points,
    }
}

/// The first turn, with at least [`MIN_GAMES`] games, at which the metric
/// tells the winner (see [`TELLS`]), with the higher share then.
pub fn first_telling(points: &[Point]) -> Option<(usize, usize)> {
    points.iter().find_map(|p| {
        let share = p.higher_share()?;
        (p.games >= MIN_GAMES && (share >= TELLS || share <= 100 - TELLS))
            .then_some((p.turn, share))
    })
}

fn number(value: Option<f64>) -> String {
    value.map_or("–".to_string(), |v| {
        if v.abs() >= 100.0 {
            format!("{v:.0}")
        } else {
            format!("{v:.1}")
        }
    })
}

/// The report, in Markdown, of `games` (the batch) with `meanings`, each
/// metric's one-line meaning by key.
pub fn report(games: &[Game], meanings: &BTreeMap<String, String>) -> String {
    let c = curves(games);
    let mut out = String::from("# Metrics report\n\n");
    let decided: Vec<&Game> = games
        .iter()
        .filter(|g| g.summary.winner.is_some())
        .collect();
    let _ = writeln!(
        out,
        "{} games, {} decided; {} metrics a side, followed at the start of each side's own ply, turns 1 to {MAX_TURN}. Bots: {}.\n",
        games.len(),
        decided.len(),
        c.metrics.len(),
        c.bots.join(", ")
    );
    out.push_str("| Bot | Games | Wins | Wins as red | Wins as blue |\n| --- | ---: | ---: | ---: | ---: |\n");
    for bot in &c.bots {
        let mut row = [0usize; 4];
        for g in games {
            if let Some(seat) = g.summary.players.iter().position(|p| p == bot) {
                row[0] += 1;
                if g.summary.winner == Some(seat) {
                    row[1] += 1;
                    row[2 + seat] += 1;
                }
            }
        }
        let _ = writeln!(
            out,
            "| {bot} | {} | {} | {} | {} |",
            row[0], row[1], row[2], row[3]
        );
    }
    let mut lengths: Vec<usize> = games.iter().map(|g| g.rows.len().div_ceil(2)).collect();
    lengths.sort_unstable();
    if let Some(median) = lengths.get(lengths.len() / 2) {
        let _ = writeln!(out, "\nMedian length: {median} turns.");
    }
    let running = |turn: usize| games.iter().filter(|g| g.rows.len() >= 2 * turn).count();
    out.push_str("\nGames still running at turn:");
    for t in CHECKPOINTS {
        let _ = write!(out, " {t}: {};", running(t));
    }
    out.push('\n');

    // When each metric tells the winner.
    let _ = writeln!(
        out,
        "\n## When each metric tells the winner\n\nThe share of decided games still running in which the winner's value was higher than the loser's (a tie counting half), at each turn; below 50% a lower value goes with winning. \"Tells from\" is the first turn with at least {MIN_GAMES} games at which the share is at least {TELLS}% or at most {}%.\n",
        100 - TELLS
    );
    out.push_str("| Metric | Tells from |");
    for t in CHECKPOINTS {
        let _ = write!(out, " Turn {t} |");
    }
    out.push_str(" Meaning |\n| --- | ---: |");
    for _ in CHECKPOINTS {
        out.push_str(" ---: |");
    }
    out.push_str(" --- |\n");
    let mut order: Vec<usize> = (0..c.metrics.len()).collect();
    order.sort_by_key(|&i| {
        (
            first_telling(&c.points[i]).map_or(usize::MAX, |(t, _)| t),
            i,
        )
    });
    for &i in &order {
        let tells =
            first_telling(&c.points[i]).map_or("–".to_string(), |(t, s)| format!("{t} ({s}%)"));
        let _ = write!(out, "| {} | {tells} |", c.metrics[i]);
        for t in CHECKPOINTS {
            let p = &c.points[i][t - 1];
            let cell = p
                .higher_share()
                .filter(|_| p.games >= MIN_GAMES)
                .map_or("–".to_string(), |s| format!("{s}%"));
            let _ = write!(out, " {cell} |");
        }
        let meaning = meanings.get(&c.metrics[i]).map_or("", String::as_str);
        let _ = writeln!(out, " {meaning} |");
    }

    // The strongest at a few turns.
    out.push_str("\n## The metrics that tell the winner best\n\nAt each turn, the ten metrics whose share is farthest from 50%, with the winner's and the loser's means.\n");
    for t in [8, 12, 16, 20] {
        let mut ranked: Vec<usize> = (0..c.metrics.len())
            .filter(|&i| c.points[i][t - 1].games >= MIN_GAMES)
            .collect();
        if ranked.is_empty() {
            continue;
        }
        ranked.sort_by_key(|&i| (std::cmp::Reverse(c.points[i][t - 1].strength()), i));
        let _ = writeln!(
            out,
            "\n### Turn {t} ({} games)\n\n| Metric | Winner higher | Winner's mean | Loser's mean |\n| --- | ---: | ---: | ---: |",
            c.points[ranked[0]][t - 1].games
        );
        for &i in ranked.iter().take(10) {
            let p = &c.points[i][t - 1];
            let _ = writeln!(
                out,
                "| {} | {}% | {} | {} |",
                c.metrics[i],
                p.higher_share().unwrap_or(50),
                number(p.winner_mean),
                number(p.loser_mean)
            );
        }
    }

    // The bots compared.
    if c.bots.len() == 2 {
        let _ = writeln!(
            out,
            "\n## {} against {}\n\nEach bot's mean over its games (both seats) at each turn, as `{}` / `{}`, and the metrics on which they differ most at turn 10 (by the difference over the larger mean).\n",
            c.bots[0], c.bots[1], c.bots[0], c.bots[1]
        );
        out.push_str("| Metric |");
        for t in CHECKPOINTS {
            let _ = write!(out, " Turn {t} |");
        }
        out.push_str("\n| --- |");
        for _ in CHECKPOINTS {
            out.push_str(" ---: |");
        }
        out.push('\n');
        let gap = |i: usize| {
            let p = &c.points[i][9];
            match (p.bot_means[0], p.bot_means[1]) {
                (Some(a), Some(b)) if a.abs().max(b.abs()) > 0.0 => {
                    ((a - b).abs() / a.abs().max(b.abs()) * 1000.0) as i64
                }
                _ => 0,
            }
        };
        let mut by_gap: Vec<usize> = (0..c.metrics.len()).collect();
        by_gap.sort_by_key(|&i| (std::cmp::Reverse(gap(i)), i));
        for &i in &by_gap {
            let _ = write!(out, "| {} |", c.metrics[i]);
            for t in CHECKPOINTS {
                let p = &c.points[i][t - 1];
                let _ = write!(
                    out,
                    " {} / {} |",
                    number(p.bot_means[0]),
                    number(p.bot_means[1])
                );
            }
            out.push('\n');
        }
    }
    out
}

#[cfg(test)]
mod tests;
