//! The early-game harness (E004): plays openings against each other, in
//! process, from both seats of many source maps, up to the middle game, and
//! reports the score there (`ice_and_fire_opening::score`) with its terms.
//!
//! ```sh
//! cargo run --release -p ice-and-fire-opening --example early -- \
//!   --ours econ,fill,rush --theirs econ,fill,rush --maps 500
//! ```
//!
//! Each map (the referee's Bronze map of a seed, from `--first-seed` on) is
//! played twice, our opening in each seat. A game stops at the middle game
//! (`ice_and_fire_opening::middle::reached`), or at the start of game turn
//! `--cap` if it never comes. There both sides are scored for us: our value
//! less theirs, at each horizon of `--horizons` (the first is the main one)
//! with `--lookahead` turns of banking in the exposure. An opening plays its
//! rules whatever the gap between the territories. The mean is over maps, each
//! map the mean of its two games, with a 95% interval.
//!
//! Openings: `econ` (`ice-and-fire-v003`'s), and E003's `fill` and `rush`.

mod rivals;

use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use ice_and_fire_engine::{Command, League, State};
use ice_and_fire_opening::score::{self, Settings, Terms};
use ice_and_fire_opening::{econ, middle};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

/// An opening the harness can play: one game at a time, from its first turn.
trait Player {
    fn turn(&mut self, state: &State) -> Vec<Command>;
}

/// An opening by rules: the same function every turn.
struct Rules(fn(&State) -> Vec<Command>);

impl Player for Rules {
    fn turn(&mut self, state: &State) -> Vec<Command> {
        (self.0)(state)
    }
}

/// A new player of the opening `name` for one game.
fn player(name: &str) -> Option<Box<dyn Player>> {
    Some(match name {
        "econ" => Box::new(Rules(econ::rules_turn)),
        "fill" => Box::new(Rules(rivals::fill)),
        "rush" => Box::new(Rules(rivals::rush)),
        _ => return None,
    })
}

struct Options {
    ours: Vec<String>,
    theirs: Vec<String>,
    maps: u64,
    first_seed: u64,
    threads: usize,
    lookahead: u32,
    horizons: Vec<i64>,
    cap: u32,
}

const USAGE: &str = "usage: early [--ours A,B] [--theirs C,D] [--maps N] [--first-seed S] \
[--threads T] [--lookahead J] [--horizons H1,H2] [--cap TURN]";

fn parse_args() -> Result<Options, String> {
    let mut options = Options {
        ours: vec!["econ".into()],
        theirs: vec!["econ".into(), "fill".into(), "rush".into()],
        maps: 200,
        first_seed: 1,
        threads: std::thread::available_parallelism().map_or(1, |n| n.get()),
        lookahead: score::LOOKAHEAD,
        horizons: vec![score::HORIZON],
        cap: 30,
    };
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let mut value = || args.next().ok_or_else(|| format!("{flag} needs a value"));
        let number = |text: String| {
            text.parse::<u64>()
                .map_err(|_| format!("{flag}: not a number: {text}"))
        };
        let list = |text: String| -> Vec<String> {
            text.split(',')
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect()
        };
        match flag.as_str() {
            "--ours" => options.ours = list(value()?),
            "--theirs" => options.theirs = list(value()?),
            "--maps" => options.maps = number(value()?)?,
            "--first-seed" => options.first_seed = number(value()?)?,
            "--threads" => options.threads = number(value()?)?.max(1) as usize,
            "--lookahead" => options.lookahead = number(value()?)? as u32,
            "--cap" => options.cap = number(value()?)? as u32,
            "--horizons" => {
                options.horizons = list(value()?)
                    .into_iter()
                    .map(|h| h.parse::<i64>().map_err(|_| format!("bad horizon {h}")))
                    .collect::<Result<_, _>>()?;
                if options.horizons.is_empty() {
                    return Err("--horizons needs at least one".into());
                }
            }
            "--help" | "-h" => return Err(USAGE.into()),
            _ => return Err(format!("unknown argument {flag}\n{USAGE}")),
        }
    }
    if options.maps == 0 {
        return Err("--maps must be at least 1".into());
    }
    for name in options.ours.iter().chain(&options.theirs) {
        if player(name).is_none() {
            return Err(format!("unknown opening {name}"));
        }
    }
    Ok(options)
}

/// One game's measure.
struct Game {
    seed: u64,
    /// Game turn of the middle game (or of the cap).
    turn: u32,
    capped: bool,
    /// Score for us at each horizon.
    scores: Vec<i64>,
    ours: Terms,
    theirs: Terms,
    /// Our answers' slowest time, ms.
    slowest_ms: f64,
}

fn start(seed: u64) -> State {
    let map = generate(seed, RefereeLeague::Bronze);
    State::new(&map.void, &map.mine_spots, League::Bronze)
}

fn play(ours: &str, theirs: &str, seed: u64, seat: usize, options: &Options) -> Game {
    let mut state = start(seed);
    let mut players = [player(ours), player(theirs)].map(Option::unwrap);
    if seat == 1 {
        players.swap(0, 1);
    }
    let mut slowest_ms: f64 = 0.0;
    let mut capped = false;
    while state.outcome().is_none() && !middle::reached(&state) {
        if state.game_turn() >= options.cap {
            capped = true;
            break;
        }
        let to_act = state.to_act();
        let clock = Instant::now();
        let commands = players[to_act].turn(&state);
        if to_act == seat {
            slowest_ms = slowest_ms.max(clock.elapsed().as_secs_f64() * 1000.0);
        }
        state.play_turn(&commands);
    }
    let settings = |horizon| Settings {
        horizon,
        lookahead: options.lookahead,
    };
    let main = settings(options.horizons[0]);
    let ours_terms = score::terms(&state, seat, main);
    let theirs_terms = score::terms(&state, 1 - seat, main);
    let mut scores = vec![ours_terms.value() - theirs_terms.value()];
    for &horizon in &options.horizons[1..] {
        scores.push(score::score(&state, seat, settings(horizon)));
    }
    Game {
        seed,
        turn: state.game_turn(),
        capped,
        scores,
        ours: ours_terms,
        theirs: theirs_terms,
        slowest_ms,
    }
}

/// Plays every map from both seats on `options.threads` threads.
fn run(ours: &str, theirs: &str, options: &Options) -> Vec<Game> {
    let jobs: Vec<(u64, usize)> = (0..options.maps)
        .flat_map(|i| [(options.first_seed + i, 0), (options.first_seed + i, 1)])
        .collect();
    let next = AtomicUsize::new(0);
    let games = Mutex::new(Vec::with_capacity(jobs.len()));
    std::thread::scope(|scope| {
        for _ in 0..options.threads {
            scope.spawn(|| loop {
                let index = next.fetch_add(1, Ordering::Relaxed);
                let Some(&(seed, seat)) = jobs.get(index) else {
                    break;
                };
                let game = play(ours, theirs, seed, seat, options);
                games.lock().unwrap_or_else(|e| e.into_inner()).push(game);
            });
        }
    });
    games.into_inner().unwrap_or_else(|e| e.into_inner())
}

/// Mean and half-width of the 95% interval of `values`.
fn mean_interval(values: &[f64]) -> (f64, f64) {
    let n = values.len() as f64;
    if n == 0.0 {
        return (0.0, 0.0);
    }
    let mean = values.iter().sum::<f64>() / n;
    if n < 2.0 {
        return (mean, 0.0);
    }
    let var = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1.0);
    (mean, 1.96 * (var / n).sqrt())
}

/// The per-map means (the two seats averaged) of `f`.
fn per_map(games: &[Game], f: impl Fn(&Game) -> f64) -> Vec<f64> {
    let mut by_seed: std::collections::BTreeMap<u64, (f64, u32)> = Default::default();
    for game in games {
        let entry = by_seed.entry(game.seed).or_default();
        entry.0 += f(game);
        entry.1 += 1;
    }
    by_seed
        .values()
        .map(|&(sum, n)| sum / f64::from(n))
        .collect()
}

fn report(ours: &str, theirs: &str, games: &[Game], options: &Options, seconds: f64) -> f64 {
    let n = games.len() as f64;
    let mean = |f: &dyn Fn(&Game) -> f64| games.iter().map(f).sum::<f64>() / n;
    let (score, half) = mean_interval(&per_map(games, |g| g.scores[0] as f64));
    println!(
        "{ours} vs {theirs}: {} maps, score {score:+.1} ± {half:.1} (H {})",
        games.len() / 2,
        options.horizons[0]
    );
    for (index, horizon) in options.horizons.iter().enumerate().skip(1) {
        let (s, h) = mean_interval(&per_map(games, |g| g.scores[index] as f64));
        println!("  at H {horizon}: {s:+.1} ± {h:.1}");
    }
    let capped = games.iter().filter(|g| g.capped).count();
    println!(
        "  middle game at turn {:.2} ({capped} games capped); our slowest answer {:.2} ms; {seconds:.1} s",
        mean(&|g| f64::from(g.turn)),
        games.iter().map(|g| g.slowest_ms).fold(0.0, f64::max)
    );
    let row = |name: &str, pick: &dyn Fn(&Game) -> Terms| {
        let t = |f: &dyn Fn(Terms) -> i64| mean(&|g| f(pick(g)) as f64);
        println!(
            "  {name:<6} value {:7.1} = gold {:6.1} + L×income {:6.1} + territory {:6.1} - exposure {:6.1} - debt {:5.1} - loss {:5.1}",
            t(&|x| x.value()),
            t(&|x| x.gold),
            t(&|x| x.income),
            t(&|x| x.territory),
            t(&|x| x.exposure),
            t(&|x| x.debt),
            t(&|x| x.loss),
        );
    };
    row("ours", &|g| g.ours);
    row("theirs", &|g| g.theirs);
    score
}

fn main() -> ExitCode {
    let options = match parse_args() {
        Ok(options) => options,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(2);
        }
    };
    let mut table: Vec<(String, Vec<f64>)> = Vec::new();
    for ours in &options.ours {
        let mut row = Vec::new();
        for theirs in &options.theirs {
            let clock = Instant::now();
            let games = run(ours, theirs, &options);
            row.push(report(
                ours,
                theirs,
                &games,
                &options,
                clock.elapsed().as_secs_f64(),
            ));
        }
        table.push((ours.clone(), row));
    }
    if table.len() * options.theirs.len() > 1 {
        println!("\n| Ours | {} | Mean |", options.theirs.join(" | "));
        println!("| --- |{} ---: |", " ---: |".repeat(options.theirs.len()));
        for (name, row) in &table {
            let cells: Vec<String> = row.iter().map(|s| format!("{s:+.1}")).collect();
            let mean = row.iter().sum::<f64>() / row.len() as f64;
            println!("| {name} | {} | {mean:+.1} |", cells.join(" | "));
        }
    }
    ExitCode::SUCCESS
}
