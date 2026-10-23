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

/// How a game's turns are lined up across games.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Align {
    /// Turn `t` from the start (1 first).
    Start,
    /// `t` turns before the game's last turn (0 the last).
    End,
}

/// The game's last turn.
fn last_turn(game: &Game) -> usize {
    game.rows.len().div_ceil(2)
}

/// The turn at `step` of `align` in `game`, if the game has it.
fn turn_at(game: &Game, align: Align, step: usize) -> Option<usize> {
    match align {
        Align::Start => (step >= 1 && step <= last_turn(game)).then_some(step),
        Align::End => last_turn(game).checked_sub(step).filter(|&t| t >= 1),
    }
}

/// The bots of `games`, sorted.
pub fn bots_of(games: &[Game]) -> Vec<String> {
    let mut bots: Vec<String> = games
        .iter()
        .flat_map(|g| g.summary.players.iter().cloned())
        .collect();
    bots.sort();
    bots.dedup();
    bots
}

/// The curves of `games`, all with the same metrics: those of the first;
/// turns from the start.
pub fn curves(games: &[Game]) -> Curves {
    let Some(first) = games.first() else {
        return Curves::default();
    };
    let metrics = first.summary.metrics.clone();
    let bots = bots_of(games);
    let refs: Vec<&Game> = games.iter().collect();
    let points = points_of(&refs, &metrics, &bots, Align::Start);
    Curves {
        bots,
        metrics,
        points,
    }
}

/// Each metric's points over `games` (those with `metrics`), at steps 1 to
/// [`MAX_TURN`] from the start, or 0 to [`MAX_TURN`] - 1 before the end;
/// `bot_means` in the order of `bots`.
pub fn points_of(
    games: &[&Game],
    metrics: &[String],
    bots: &[String],
    align: Align,
) -> Vec<Vec<Point>> {
    let games: Vec<&Game> = games
        .iter()
        .copied()
        .filter(|g| g.summary.metrics == metrics)
        .collect();
    let steps: Vec<usize> = match align {
        Align::Start => (1..=MAX_TURN).collect(),
        Align::End => (0..MAX_TURN).collect(),
    };
    let mut points = Vec::new();
    for (index, key) in metrics.iter().enumerate() {
        let mut row = Vec::new();
        for &step in &steps {
            let mut p = Point {
                turn: step,
                bot_means: vec![None; bots.len()],
                ..Point::default()
            };
            let (mut win_sum, mut win_n, mut lose_sum, mut lose_n) = (0, 0, 0, 0);
            let mut bot_sums = vec![(0i64, 0usize); bots.len()];
            for game in &games {
                let Some(turn) = turn_at(game, align, step) else {
                    continue;
                };
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
    points
}

/// The turn at which [`slices`] looks for the leader in the lead metric.
pub const LEAD_TURN: usize = 10;

/// A subset of a batch's games, along one dimension.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Slice {
    /// What splits the batch: `All`, `Length`, `Seats`, `Winner` or `Lead`.
    pub dimension: String,
    pub name: String,
    /// The games' indices in the batch.
    pub games: Vec<usize>,
}

/// The batch's slices: all games; the shortest, regular and longest thirds
/// by length; for two bots, each seating and each bot's wins; and the games
/// the side ahead in the lead metric at turn [`LEAD_TURN`] went on to win or
/// to lose (games without a leader then are in neither).
pub fn slices(games: &[Game], bots: &[String]) -> Vec<Slice> {
    let all: Vec<usize> = (0..games.len()).collect();
    let mut out = vec![Slice {
        dimension: "All".to_string(),
        name: "All games".to_string(),
        games: all.clone(),
    }];
    // Thirds by rank: the games sorted by length (ties in batch order),
    // each third named by the lengths it holds.
    let mut by_length = all.clone();
    by_length.sort_by_key(|&i| (last_turn(&games[i]), i));
    let n = by_length.len();
    for (k, name) in ["Short", "Regular", "Long"].into_iter().enumerate() {
        let mut part: Vec<usize> = by_length[k * n / 3..(k + 1) * n / 3].to_vec();
        let (Some(&first), Some(&last)) = (part.first(), part.last()) else {
            continue;
        };
        let (low, high) = (last_turn(&games[first]), last_turn(&games[last]));
        part.sort_unstable();
        out.push(Slice {
            dimension: "Length".to_string(),
            name: if low == high {
                format!("{name} ({low} turns)")
            } else {
                format!("{name} ({low} to {high} turns)")
            },
            games: part,
        });
    }
    if bots.len() == 2 {
        for red in bots {
            let blue = bots.iter().find(|b| *b != red).expect("two bots");
            out.push(Slice {
                dimension: "Seats".to_string(),
                name: format!("{red} red, {blue} blue"),
                games: all
                    .iter()
                    .copied()
                    .filter(|&i| &games[i].summary.players[0] == red)
                    .collect(),
            });
        }
        for bot in bots {
            out.push(Slice {
                dimension: "Winner".to_string(),
                name: format!("{bot} won"),
                games: all
                    .iter()
                    .copied()
                    .filter(|&i| {
                        let g = &games[i].summary;
                        g.winner.is_some_and(|w| &g.players[w] == bot)
                    })
                    .collect(),
            });
        }
    }
    let leader = |game: &Game| -> Option<usize> {
        let index = game
            .summary
            .metrics
            .iter()
            .position(|k| *k == game.summary.lead)?;
        let red = at(game, None, index, LEAD_TURN, 0)?;
        let blue = at(game, None, index, LEAD_TURN, 1)?;
        match red.cmp(&blue) {
            std::cmp::Ordering::Greater => Some(0),
            std::cmp::Ordering::Less => Some(1),
            std::cmp::Ordering::Equal => None,
        }
    };
    for (name, held) in [("won", true), ("lost", false)] {
        out.push(Slice {
            dimension: "Lead".to_string(),
            name: format!("Leader at turn {LEAD_TURN} {name}"),
            games: all
                .iter()
                .copied()
                .filter(|&i| {
                    let g = &games[i];
                    match (leader(g), g.summary.winner) {
                        (Some(l), Some(w)) => (l == w) == held,
                        _ => false,
                    }
                })
                .collect(),
        });
    }
    out
}

/// A slice's points, from the start and before the end.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SlicePoints {
    #[serde(flatten)]
    pub slice: Slice,
    pub start: Vec<Vec<Point>>,
    pub end: Vec<Vec<Point>>,
}

/// Every slice's points, for charts.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub struct Sliced {
    pub bots: Vec<String>,
    pub metrics: Vec<String>,
    pub slices: Vec<SlicePoints>,
}

/// The points of every slice of `games`.
pub fn sliced(games: &[Game]) -> Sliced {
    let Some(first) = games.first() else {
        return Sliced::default();
    };
    let metrics = first.summary.metrics.clone();
    let bots = bots_of(games);
    let slices = slices(games, &bots)
        .into_iter()
        .map(|slice| {
            let refs: Vec<&Game> = slice.games.iter().map(|&i| &games[i]).collect();
            SlicePoints {
                start: points_of(&refs, &metrics, &bots, Align::Start),
                end: points_of(&refs, &metrics, &bots, Align::End),
                slice,
            }
        })
        .collect();
    Sliced {
        bots,
        metrics,
        slices,
    }
}

/// The first turn, with at least [`MIN_GAMES`] games, at which the metric
/// tells the winner (see [`TELLS`]), with the higher share then.
///
/// Turn 1 is left out: its values are the start of the game, the same in
/// every game for each seat, so any share there is the seats' and not the
/// play's.
pub fn first_telling(points: &[Point]) -> Option<(usize, usize)> {
    points.iter().filter(|p| p.turn >= 2).find_map(|p| {
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
    let running = |turn: usize| games.iter().filter(|g| last_turn(g) >= turn).count();
    out.push_str("\nGames still running at turn:");
    for t in CHECKPOINTS {
        let _ = write!(out, " {t}: {};", running(t));
    }
    out.push('\n');

    // When each metric tells the winner.
    let _ = writeln!(
        out,
        "\n## When each metric tells the winner\n\nThe share of decided games still running in which the winner's value was higher than the loser's (a tie counting half), at each turn; below 50% a lower value goes with winning. \"Tells from\" is the first turn from turn 2 with at least {MIN_GAMES} games at which the share is at least {TELLS}% or at most {}%.\n\nRed and blue are each measured at the start of their own turn, and blue starts with one more cell: in the first turns a metric can tell the winner only because the winner sat in the better seat, above all in a slice where one seat wins most games. Read early turns with the seats in mind.\n",
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
    out.push_str(&by_slice(games, &c));
    out
}

/// The report's sections by slice (see [`slices`]).
fn by_slice(games: &[Game], c: &Curves) -> String {
    let mut out = String::new();
    let all = slices(games, &c.bots);
    let refs = |slice: &Slice| -> Vec<&Game> { slice.games.iter().map(|&i| &games[i]).collect() };
    let starts: Vec<Vec<Vec<Point>>> = all
        .iter()
        .map(|s| points_of(&refs(s), &c.metrics, &c.bots, Align::Start))
        .collect();
    let ends: Vec<Vec<Vec<Point>>> = all
        .iter()
        .map(|s| points_of(&refs(s), &c.metrics, &c.bots, Align::End))
        .collect();

    out.push_str("\n## How the batch splits\n\nThe same games cut along four dimensions: length (the shortest, regular and longest thirds), seats, which bot won, and whether the side ahead in the lead metric at turn ");
    let _ = write!(out, "{LEAD_TURN} went on to win. Each slice is analysed as the whole batch is above.\n\n| Slice | Games |");
    for bot in &c.bots {
        let _ = write!(out, " {bot} won |");
    }
    out.push_str(" Median length |\n| --- | ---: |");
    for _ in &c.bots {
        out.push_str(" ---: |");
    }
    out.push_str(" ---: |\n");
    for slice in &all {
        let _ = write!(out, "| {} | {} |", slice.name, slice.games.len());
        for bot in &c.bots {
            let wins = slice
                .games
                .iter()
                .filter(|&&i| {
                    let g = &games[i].summary;
                    g.winner.is_some_and(|w| &g.players[w] == bot)
                })
                .count();
            let _ = write!(out, " {wins} |");
        }
        let mut lengths: Vec<usize> = slice.games.iter().map(|&i| last_turn(&games[i])).collect();
        lengths.sort_unstable();
        let median = lengths
            .get(lengths.len() / 2)
            .map_or("–".to_string(), |m| m.to_string());
        let _ = writeln!(out, " {median} |");
    }

    let header = |out: &mut String| {
        out.push_str("| Metric |");
        for slice in &all {
            let _ = write!(out, " {} |", slice.name);
        }
        out.push_str("\n| --- |");
        for _ in &all {
            out.push_str(" ---: |");
        }
        out.push('\n');
    };
    let _ = writeln!(
        out,
        "\n## When each metric tells the winner, by slice\n\nFrom the start: the first turn at which the metric tells the winner in that slice (at least {MIN_GAMES} games, a share of at least {TELLS}% or at most {}%), with the share then; – when it never does.\n",
        100 - TELLS
    );
    header(&mut out);
    for (i, metric) in c.metrics.iter().enumerate() {
        let _ = write!(out, "| {metric} |");
        for points in &starts {
            let cell =
                first_telling(&points[i]).map_or("–".to_string(), |(t, s)| format!("{t} ({s}%)"));
            let _ = write!(out, " {cell} |");
        }
        out.push('\n');
    }

    for step in [5, 1] {
        let _ = writeln!(
            out,
            "\n## {step} turn{} before the end, by slice\n\nEach game lined up on its last turn: the share of games in which the winner's value was higher, {step} turn{} before it (a tie counting half); – with fewer than {MIN_GAMES} games.\n",
            if step == 1 { "" } else { "s" },
            if step == 1 { "" } else { "s" }
        );
        header(&mut out);
        for (i, metric) in c.metrics.iter().enumerate() {
            let _ = write!(out, "| {metric} |");
            for points in &ends {
                let p = &points[i][step];
                let cell = p
                    .higher_share()
                    .filter(|_| p.games >= MIN_GAMES)
                    .map_or("–".to_string(), |s| format!("{s}%"));
                let _ = write!(out, " {cell} |");
            }
            out.push('\n');
        }
    }

    if c.bots.len() == 2 {
        for (slice, points) in all
            .iter()
            .zip(&starts)
            .filter(|(s, _)| s.dimension == "Seats")
        {
            let _ = writeln!(
                out,
                "\n## {}: the two bots at turn 10\n\nThe ten metrics on which the bots' means differ most at turn 10 in these games, as `{}` / `{}`, at turns 8, 10 and 12.\n\n| Metric | Turn 8 | Turn 10 | Turn 12 |\n| --- | ---: | ---: | ---: |",
                slice.name, c.bots[0], c.bots[1]
            );
            let gap = |i: usize| {
                let p = &points[i][9];
                match (p.bot_means[0], p.bot_means[1]) {
                    (Some(a), Some(b)) if a.abs().max(b.abs()) > 0.0 => {
                        ((a - b).abs() / a.abs().max(b.abs()) * 1000.0) as i64
                    }
                    _ => 0,
                }
            };
            let mut by_gap: Vec<usize> = (0..c.metrics.len()).collect();
            by_gap.sort_by_key(|&i| (std::cmp::Reverse(gap(i)), i));
            for &i in by_gap.iter().take(10) {
                let _ = write!(out, "| {} |", c.metrics[i]);
                for t in [8, 10, 12] {
                    let p = &points[i][t - 1];
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
    }
    out
}

#[cfg(test)]
mod tests;
