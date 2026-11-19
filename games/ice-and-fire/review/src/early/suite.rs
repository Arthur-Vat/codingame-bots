//! Opening suites (`early-suite`, `early-openings`): phase 0, step 4 of the
//! early-game plan. A suite lists the top players' CodinGame games, each
//! with the player's early measures at the start of its plies of game turns
//! [`TURNS`] and its zasmu-likeness at turn 12, so that a bot can be played
//! from the same map, seat and opponent and compared with them.
//!
//! `early-openings` plays the bot in the player's seat against the release
//! the player met in that game (a binary for each release is given), on the
//! game's map, up to the start of the bot's ply of turn 12, and reports
//! both sides' measures by player and seat. Each start (map, seat and
//! release) is played once, however many games of the suite share it (the
//! CodinGame test games replay a few dozen maps).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::time::Duration;

use cg_arena::live::{BotSettings, LiveBot};
use cg_arena::runner::BotSpec;
use ice_and_fire_early::score::{features, zasmu_likeness};
use ice_and_fire_early::zones::Zones;
use ice_and_fire_engine::{League, State};
use ice_and_fire_referee::map::generate;

use crate::profiles::{prepare, read_index, Request};
use crate::replay::{replay, Game};

/// The game turns measured; the last is the zasmu-likeness score's.
pub const TURNS: [u32; 7] = [5, 6, 7, 8, 9, 10, 12];

/// The measures at each turn, by their index in [`features`]: active
/// cells, Voronoi share, middle chokes, units, gold.
pub const MEASURES: [(&str, usize); 5] = [
    ("active", 0),
    ("voronoi", 1),
    ("mid", 2),
    ("units", 3),
    ("gold", 6),
];

/// The values of one side: each measure at each turn (turn-major), then
/// the zasmu-likeness at the last turn.
pub type Values = Vec<f64>;

/// One line of a suite.
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub game_id: u64,
    pub kind: String,
    /// The map's seed, as our generator takes it.
    pub seed: u64,
    pub seat: usize,
    pub player: String,
    /// The release the player met.
    pub opponent: String,
    pub values: Values,
}

/// The measures of `seat` in a game, from the side's ply-start states, if
/// it reached every turn of [`TURNS`].
fn side_values(states: &[&State], seat: usize, zones: &Zones) -> Option<Values> {
    let mut out = Vec::with_capacity(TURNS.len() * MEASURES.len() + 1);
    let mut score = 0.0;
    for &turn in &TURNS {
        let state = states
            .iter()
            .find(|s| s.to_act() == seat && s.game_turn() == turn)?;
        let x = features(state, seat, zones);
        out.extend(MEASURES.iter().map(|&(_, i)| x[i]));
        score = zasmu_likeness(state, seat, zones);
    }
    out.push(score);
    Some(out)
}

/// The measures of both seats of a replayed game.
fn game_values(game: &Game) -> [Option<Values>; 2] {
    let Some(first) = game.steps.first() else {
        return [None, None];
    };
    let zones = Zones::of(&first.state);
    let states: Vec<&State> = game.steps.iter().map(|s| &s.state).collect();
    [0, 1].map(|seat| side_values(&states, seat, &zones))
}

/// A value as the suite writes it: whole numbers bare, others to three
/// decimals.
fn number(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e9 {
        format!("{v:.0}")
    } else {
        format!("{v:.3}")
    }
}

/// The suite's header line.
fn header() -> String {
    let mut cols = vec!["game_id", "kind", "seed", "seat", "player", "opponent"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();
    for turn in TURNS {
        for (name, _) in MEASURES {
            cols.push(format!("{name}{turn}"));
        }
    }
    cols.push(format!("zl{}", TURNS[TURNS.len() - 1]));
    cols.join("\t")
}

/// The suite of `early-suite`: every game of `players` against one of our
/// releases that reached turn 12, as TSV.
pub fn suite(request: &Request, players: &[String], comment: &str) -> Result<String, String> {
    let index = request.data.join("index.jsonl");
    let (entries, _) = read_index(&index, request)?;
    let release_of = super::releases(&index)?;
    let mut lines = Vec::new();
    for entry in &entries {
        let Ok((meta, record)) = prepare(&request.data, entry, &request.me) else {
            continue;
        };
        let Some(ours) = meta.our_seat else { continue };
        let Some(release) = release_of.get(&meta.game_id) else {
            continue;
        };
        let seat = 1 - ours;
        let player = record.players[seat].name.clone();
        if !players.contains(&player) {
            continue;
        }
        let seed = record.seed;
        let game = replay(&meta.game_id.to_string(), record);
        let [red, blue] = game_values(&game);
        let Some(values) = [red, blue][seat].clone() else {
            continue;
        };
        lines.push(Line {
            game_id: meta.game_id,
            kind: meta.kind.clone(),
            seed,
            seat,
            player,
            opponent: release.clone(),
            values,
        });
    }
    lines.sort_by_key(|l| l.game_id);
    Ok(format_suite(&lines, comment))
}

/// The text of a suite: `comment` as `#` lines, the header, then a line a
/// game.
pub fn format_suite(lines: &[Line], comment: &str) -> String {
    let mut out = String::new();
    for c in comment.lines() {
        let _ = writeln!(out, "# {c}");
    }
    let _ = writeln!(out, "{}", header());
    for l in lines {
        let values: Vec<String> = l.values.iter().map(|&v| number(v)).collect();
        let _ = writeln!(
            out,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            l.game_id,
            l.kind,
            l.seed,
            l.seat,
            l.player,
            l.opponent,
            values.join("\t")
        );
    }
    out
}

/// Reads a suite written by [`suite`].
pub fn read_suite(text: &str) -> Result<Vec<Line>, String> {
    let mut lines = text
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty());
    let head = lines.next().ok_or("the suite is empty")?;
    if head != header() {
        return Err("the suite's header is not this tool's".to_string());
    }
    let width = TURNS.len() * MEASURES.len() + 1;
    lines
        .enumerate()
        .map(|(n, line)| {
            let f: Vec<&str> = line.split('\t').collect();
            let bad = || format!("suite line {}: {line}", n + 2);
            if f.len() != 6 + width {
                return Err(bad());
            }
            Ok(Line {
                game_id: f[0].parse().map_err(|_| bad())?,
                kind: f[1].to_string(),
                seed: f[2].parse().map_err(|_| bad())?,
                seat: f[3].parse().ok().filter(|&s| s < 2).ok_or_else(bad)?,
                player: f[4].to_string(),
                opponent: f[5].to_string(),
                values: f[6..]
                    .iter()
                    .map(|v| v.parse::<f64>().map_err(|_| bad()))
                    .collect::<Result<_, _>>()?,
            })
        })
        .collect()
}

/// A start of the suite: map seed, the player's seat, the release met.
type Start = (u64, usize, String);

/// Plays `bot` in `seat` against `opponent` on the map of `seed`, up to
/// the start of the bot's ply of the last turn of [`TURNS`]; the bot's
/// values, or why there are none.
fn play(
    bot: &BotSpec,
    opponent: &BotSpec,
    settings: &BotSettings,
    seed: u64,
    seat: usize,
) -> Result<Values, String> {
    let map = generate(seed, ice_and_fire_referee::League::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let zones = Zones::of(&state);
    let mut players = [None, None];
    players[seat] = Some(LiveBot::spawn(bot, settings).map_err(|e| format!("bot: {e}"))?);
    players[1 - seat] =
        Some(LiveBot::spawn(opponent, settings).map_err(|e| format!("opponent: {e}"))?);
    let last = TURNS[TURNS.len() - 1];
    let mut states = Vec::new();
    while state.outcome().is_none() {
        let to_act = state.to_act();
        if to_act == seat {
            states.push(state.clone());
            if state.game_turn() == last {
                break;
            }
        }
        let Some(player) = players[to_act].as_mut() else {
            return Err("no player".to_string());
        };
        let mut input = String::new();
        let first = player.answers() == 0;
        if first {
            input.push_str(&state.initial_input(to_act));
        }
        input.push_str(&state.turn_input(to_act));
        let limit = Duration::from_secs(if first { 60 } else { 20 });
        let (lines, _) = player
            .ask(&input, 1, limit)
            .map_err(|e| format!("seat {to_act}, turn {}: {e}", state.game_turn()))?;
        let line = lines.first().map(String::as_str).unwrap_or("");
        let commands: Vec<_> = line
            .split(';')
            .filter_map(|part| crate::trace::parse(part.trim()))
            .collect();
        state.play_turn(&commands);
    }
    let refs: Vec<&State> = states.iter().collect();
    side_values(&refs, seat, &zones).ok_or_else(|| "the game ended before turn 12".to_string())
}

/// What `early-openings` needs.
pub struct OpeningsRequest {
    /// The suite's text.
    pub suite_text: String,
    pub bot: String,
    /// `RELEASE=COMMAND` for each release the suite's players met.
    pub opponents: Vec<String>,
    pub time_scale: f64,
    pub jobs: usize,
}

/// The mean of each value over `rows`.
fn mean(rows: &[&Values]) -> Values {
    let n = rows.len().max(1) as f64;
    let width = rows.first().map_or(0, |r| r.len());
    (0..width)
        .map(|i| rows.iter().map(|r| r[i]).sum::<f64>() / n)
        .collect()
}

/// The report of `early-openings`, as Markdown.
pub fn openings(request: &OpeningsRequest) -> Result<String, String> {
    let lines = read_suite(&request.suite_text)?;
    let bot = BotSpec::parse(&request.bot)?;
    let opponents: BTreeMap<String, BotSpec> = request
        .opponents
        .iter()
        .map(|o| BotSpec::parse(o).map(|spec| (spec.name.clone(), spec)))
        .collect::<Result<_, _>>()?;
    // The reference values of each start: the mean over its games.
    let mut by_start: BTreeMap<Start, Vec<&Line>> = BTreeMap::new();
    let mut skipped = 0;
    for l in &lines {
        if opponents.contains_key(&l.opponent) {
            by_start
                .entry((l.seed, l.seat, l.opponent.clone()))
                .or_default()
                .push(l);
        } else {
            skipped += 1;
        }
    }
    let starts: Vec<Start> = by_start.keys().cloned().collect();
    let settings = BotSettings {
        seed: 1,
        time_scale: request.time_scale,
        fixed_iters: None,
        show_stderr: false,
    };
    let jobs = request.jobs.clamp(1, 64);
    let size = starts.len().div_ceil(jobs).max(1);
    let played: Vec<(Start, Result<Values, String>)> = std::thread::scope(|scope| {
        let handles: Vec<_> = starts
            .chunks(size)
            .map(|chunk| {
                let (bot, opponents, settings) = (&bot, &opponents, &settings);
                scope.spawn(move || {
                    chunk
                        .iter()
                        .map(|start| {
                            let (seed, seat, ref release) = *start;
                            let result = play(bot, &opponents[release], settings, seed, seat);
                            (start.clone(), result)
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap_or_else(|p| std::panic::resume_unwind(p)))
            .collect()
    });

    // Group the starts by player and seat; a start whose games were played
    // by several players counts for each.
    let mut groups: BTreeMap<(String, usize), Vec<(Values, Values)>> = BTreeMap::new();
    let mut failed = Vec::new();
    for (start, result) in &played {
        let games = &by_start[start];
        match result {
            Ok(values) => {
                let mut by_player: BTreeMap<&str, Vec<&Values>> = BTreeMap::new();
                for g in games {
                    by_player.entry(&g.player).or_default().push(&g.values);
                }
                for (player, rows) in by_player {
                    groups
                        .entry((player.to_string(), start.1))
                        .or_default()
                        .push((mean(&rows), values.clone()));
                }
            }
            Err(e) => failed.push(format!(
                "seed {} seat {} vs {}: {e}",
                start.0, start.1, start.2
            )),
        }
    }

    let mut out = format!(
        "# Opening suite: {} against the top players\n\n{} starts played (map, seat and release met), from {} suite lines; {} lines left out (no binary for their release); {} starts failed.\n\nEach cell is the player's mean / the bot's mean over the starts, each start weighing once (the player's value on a start is the mean of its games there).\n",
        bot.name,
        played.len(),
        lines.len() - skipped,
        skipped,
        failed.len()
    );
    let col = |turn: u32, measure: usize| {
        let t = TURNS.iter().position(|&x| x == turn).unwrap_or(0);
        t * MEASURES.len() + measure
    };
    for (name, m) in MEASURES.iter().enumerate().map(|(i, (n, _))| (*n, i)) {
        let _ = writeln!(
            out,
            "\n## {name}\n\n| Player | Seat | Starts | Turn 5 | Turn 8 | Turn 10 | Turn 12 |\n| --- | --- | ---: | --- | --- | --- | --- |"
        );
        for ((player, seat), rows) in &groups {
            let refs: Vec<&Values> = rows.iter().map(|r| &r.0).collect();
            let bots: Vec<&Values> = rows.iter().map(|r| &r.1).collect();
            let (a, b) = (mean(&refs), mean(&bots));
            let cells: Vec<String> = [5, 8, 10, 12]
                .iter()
                .map(|&t| format!("{:.2} / {:.2}", a[col(t, m)], b[col(t, m)]))
                .collect();
            let _ = writeln!(
                out,
                "| {player} | {} | {} | {} |",
                ["red", "blue"][*seat],
                rows.len(),
                cells.join(" | ")
            );
        }
    }
    let _ = writeln!(
        out,
        "\n## Zasmu-likeness at turn 12, and active cells at turn 10\n\n| Player | Seat | Starts | Zasmu-likeness (player / bot) | Starts where the bot holds as many active cells at turn 10 |\n| --- | --- | ---: | --- | ---: |"
    );
    let zl = TURNS.len() * MEASURES.len();
    for ((player, seat), rows) in &groups {
        let refs: Vec<&Values> = rows.iter().map(|r| &r.0).collect();
        let bots: Vec<&Values> = rows.iter().map(|r| &r.1).collect();
        let (a, b) = (mean(&refs), mean(&bots));
        let level = rows
            .iter()
            .filter(|(r, v)| v[col(10, 0)] >= r[col(10, 0)])
            .count();
        let _ = writeln!(
            out,
            "| {player} | {} | {} | {:.2} / {:.2} | {:.0}% |",
            ["red", "blue"][*seat],
            rows.len(),
            a[zl],
            b[zl],
            100.0 * level as f64 / rows.len().max(1) as f64
        );
    }
    if !failed.is_empty() {
        let _ = writeln!(out, "\n## Failed starts\n");
        for f in failed.iter().take(20) {
            let _ = writeln!(out, "- {f}");
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests;
