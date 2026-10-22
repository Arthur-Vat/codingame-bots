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
//! less theirs, with the economy counted at each of `--turns` turns into the
//! middle game (the first is the main one). An opening plays its
//! rules whatever the gap between the territories. The mean is over maps, each
//! map the mean of its two games, with a 95% interval.
//!
//! Openings: `econ` (`ice-and-fire-v003`'s), and E003's `fill` and `rush`.

mod rivals;

use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use ice_and_fire_engine::{Command, League, State};
use ice_and_fire_opening::econ::Params;
use ice_and_fire_opening::planned::{Planned, Planner};
use ice_and_fire_opening::score::{self, Settings, Terms};
use ice_and_fire_opening::{econ, middle, rollout};
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

/// `econ` with other weights.
struct Econ(Params);

impl Player for Econ {
    fn turn(&mut self, state: &State) -> Vec<Command> {
        econ::rules_turn_with(state, &self.0)
    }
}

/// The weights of `econ` that a spec can set, by name.
const KNOBS: [&str; 11] = [
    "open",
    "contest",
    "contest_cap",
    "hq_div",
    "horizon",
    "min_turns_left",
    "safe_spot",
    "contact_offset",
    "contact_div",
    "after_contact",
    "compact",
];

fn knob<'a>(params: &'a mut Params, name: &str) -> Option<KnobRef<'a>> {
    Some(match name {
        "open" => KnobRef::I(&mut params.open),
        "contest" => KnobRef::I(&mut params.contest),
        "contest_cap" => KnobRef::U(&mut params.contest_cap),
        "hq_div" => KnobRef::I(&mut params.hq_div),
        "horizon" => KnobRef::I(&mut params.horizon),
        "min_turns_left" => KnobRef::I(&mut params.min_turns_left),
        "safe_spot" => KnobRef::U(&mut params.safe_spot),
        "contact_offset" => KnobRef::I(&mut params.contact_offset),
        "contact_div" => KnobRef::I(&mut params.contact_div),
        "after_contact" => KnobRef::I(&mut params.after_contact),
        "compact" => KnobRef::I(&mut params.compact),
        _ => return None,
    })
}

enum KnobRef<'a> {
    I(&'a mut i64),
    U(&'a mut u8),
}

impl KnobRef<'_> {
    fn get(&self) -> i64 {
        match self {
            KnobRef::I(v) => **v,
            KnobRef::U(v) => i64::from(**v),
        }
    }
    fn set(&mut self, value: i64) {
        match self {
            KnobRef::I(v) => **v = value,
            KnobRef::U(v) => **v = value.clamp(0, 255) as u8,
        }
    }
}

/// `econ` weights from a spec `econ/name=value/...`, the others V003's.
fn econ_params(spec: &str) -> Option<Params> {
    let mut parts = spec.split('/');
    if parts.next()? != "econ" {
        return None;
    }
    let mut params = Params::V003;
    for part in parts {
        let (name, value) = part.split_once('=')?;
        knob(&mut params, name)?.set(value.parse().ok()?);
    }
    Some(params)
}

/// The spec of `econ` with `params`, naming the weights that differ from V003.
fn econ_spec(params: &Params) -> String {
    let mut spec = String::from("econ");
    let mut base = Params::V003;
    let mut mine = *params;
    for name in KNOBS {
        let a = knob(&mut mine, name).map(|k| k.get());
        let b = knob(&mut base, name).map(|k| k.get());
        if a != b {
            spec.push_str(&format!("/{name}={}", a.unwrap_or_default()));
        }
    }
    spec
}

/// S2: on its first turn, the weights of a portfolio whose rollout against
/// the `econ` model scores best; then those weights.
struct Chooser {
    sets: Vec<Params>,
    chosen: Option<Params>,
}

impl Player for Chooser {
    fn turn(&mut self, state: &State) -> Vec<Command> {
        let params = *self.chosen.get_or_insert_with(|| {
            let index = rollout::choose(state, &self.sets, &Params::V003, Settings::DEFAULT, None);
            self.sets[index]
        });
        econ::rules_turn_with(state, &params)
    }
}

/// The weight sets of a portfolio file: one `econ/...` spec a line.
fn portfolio(path: &str) -> Option<Vec<Params>> {
    let text = std::fs::read_to_string(path).ok()?;
    let sets: Option<Vec<Params>> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(econ_params)
        .collect();
    sets.filter(|s| !s.is_empty())
}

/// A planner of the first turn, as the bot plays it ([`Planned`]): a plan
/// on the first turn within `--time-ms`, replayed while our side is as
/// predicted, else a new plan within `--replan-ms`.
struct Bot {
    planned: Planned,
    first_ms: u64,
    replan_ms: u64,
}

impl Player for Bot {
    fn turn(&mut self, state: &State) -> Vec<Command> {
        let ms = if self.planned.plans == 0 {
            self.first_ms
        } else {
            self.replan_ms
        };
        let now = Instant::now();
        let deadline = now + Duration::from_millis(ms * 4 / 5);
        self.planned.turn(state, Some(deadline))
    }
}

/// The planner of a name: `beam[:WIDTH]`, `choose` (S2 over the `--base`
/// sets) or `search` (S2, then hill climbing on the weights in the time).
fn planner(name: &str) -> Option<Planner> {
    let (kind, arg) = name.split_once(':').unwrap_or((name, ""));
    let arg: Option<usize> = arg.parse().ok();
    Some(match kind {
        "beam" => {
            let width = arg.unwrap_or(20).max(1);
            Planner::Beam {
                width,
                replan: (width / 4).max(4),
            }
        }
        "choose" => Planner::Choose,
        "search" => Planner::Search,
        // deep:TURNS:EVALS
        "deep" => {
            let mut parts = name.split(':').skip(1);
            let turns = parts.next().and_then(|t| t.parse().ok()).unwrap_or(3);
            let evals = parts.next().and_then(|e| e.parse().ok()).unwrap_or(200);
            Planner::Deep { turns, evals }
        }
        _ => return None,
    })
}

/// Settings of the planners, read once.
struct PlannerSettings {
    base: Vec<Params>,
    time_ms: u64,
    replan_ms: u64,
    /// How the planners score the middle game.
    score: Settings,
}

static PLANNER: std::sync::OnceLock<PlannerSettings> = std::sync::OnceLock::new();

/// A new player of the opening `name` for one game.
fn player(name: &str) -> Option<Box<dyn Player>> {
    if let Some(planner) = planner(name) {
        let settings = PLANNER.get_or_init(|| PlannerSettings {
            base: vec![Params::V003],
            time_ms: 600,
            replan_ms: 35,
            score: Settings::DEFAULT,
        });
        let mut planned = Planned::new(planner, settings.base.clone(), Params::V003);
        planned.settings = settings.score;
        return Some(Box::new(Bot {
            planned,
            first_ms: settings.time_ms,
            replan_ms: settings.replan_ms,
        }));
    }
    if let Some(path) = name.strip_prefix("s2=") {
        return Some(Box::new(Chooser {
            sets: portfolio(path)?,
            chosen: None,
        }));
    }
    Some(match name {
        "fill" => Box::new(Rules(rivals::fill)),
        "rush" => Box::new(Rules(rivals::rush)),
        _ => Box::new(Econ(econ_params(name)?)),
    })
}

struct Options {
    ours: Vec<String>,
    theirs: Vec<String>,
    maps: u64,
    first_seed: u64,
    threads: usize,
    turns: Vec<i64>,
    cap: u32,
    /// Tune `econ`'s weights from this spec instead of playing.
    tune: Option<String>,
    /// Build an S2 portfolio from the weight sets of this file.
    build_portfolio: Option<String>,
    /// Most sets of a portfolio.
    sets: usize,
}

const USAGE: &str = "usage: early [--ours A,B] [--theirs C,D] [--maps N] [--first-seed S] \
[--threads T] [--turns T1,T2] [--cap TURN] [--base SPEC[+SPEC...]] [--time-ms MS] [--replan-ms MS] [--plan-turns T] [--plan-cell G] [--plan-gold K] \
[--tune SPEC] [--build-portfolio FILE --sets N]";

fn parse_args() -> Result<Options, String> {
    let mut options = Options {
        ours: vec!["econ".into()],
        theirs: vec!["econ".into(), "fill".into(), "rush".into()],
        maps: 200,
        first_seed: 1,
        threads: std::thread::available_parallelism().map_or(1, |n| n.get()),
        turns: vec![score::TURNS],
        cap: 30,
        tune: None,
        build_portfolio: None,
        sets: 32,
    };
    let mut base: Option<String> = None;
    let mut time_ms = 600;
    let mut replan_ms = 35;
    let mut plan_score = Settings::DEFAULT;
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
            "--cap" => options.cap = number(value()?)? as u32,
            "--tune" => options.tune = Some(value()?),
            "--base" => base = Some(value()?),
            "--time-ms" => time_ms = number(value()?)?,
            "--replan-ms" => replan_ms = number(value()?)?,
            "--plan-turns" => plan_score.turns = number(value()?)? as i64,
            "--plan-cell" => plan_score.cell = number(value()?)? as i64,
            "--plan-gold" => plan_score.gold = number(value()?)? as i64,
            "--build-portfolio" => options.build_portfolio = Some(value()?),
            "--sets" => options.sets = number(value()?)?.max(1) as usize,
            "--turns" => {
                options.turns = list(value()?)
                    .into_iter()
                    .map(|t| {
                        t.parse::<i64>()
                            .map_err(|_| format!("bad number of turns {t}"))
                    })
                    .collect::<Result<_, _>>()?;
                if options.turns.is_empty() {
                    return Err("--turns needs at least one".into());
                }
            }
            "--help" | "-h" => return Err(USAGE.into()),
            _ => return Err(format!("unknown argument {flag}\n{USAGE}")),
        }
    }
    // The planners' base weights: one spec, or several separated by `+`.
    let sets: Option<Vec<Params>> = base
        .as_deref()
        .unwrap_or("econ")
        .split('+')
        .map(econ_params)
        .collect();
    let Some(sets) = sets else {
        return Err("bad --base".into());
    };
    let _ = PLANNER.set(PlannerSettings {
        base: sets,
        time_ms,
        replan_ms,
        score: plan_score,
    });
    if options.maps == 0 {
        return Err("--maps must be at least 1".into());
    }
    for name in options
        .ours
        .iter()
        .chain(&options.theirs)
        .chain(options.tune.iter())
    {
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
    /// Score for us at each number of turns.
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
    let settings = |turns| Settings {
        turns,
        ..Settings::DEFAULT
    };
    let main = settings(options.turns[0]);
    let ours_terms = score::terms(&state, seat, main);
    let theirs_terms = score::terms(&state, 1 - seat, main);
    let mut scores = vec![ours_terms.value() - theirs_terms.value()];
    for &turns in &options.turns[1..] {
        scores.push(score::score(&state, seat, settings(turns)));
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
        "{ours} vs {theirs}: {} maps, score {score:+.1} ± {half:.1} ({} turns)",
        games.len() / 2,
        options.turns[0]
    );
    for (index, turns) in options.turns.iter().enumerate().skip(1) {
        let (s, h) = mean_interval(&per_map(games, |g| g.scores[index] as f64));
        println!("  at {turns} turns: {s:+.1} ± {h:.1}");
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
            "  {name:<6} value {:7.1} = Voronoi {:6.1} + gold {:6.1} + turns × income {:6.1} - exposure {:6.1} - debt {:5.1} - loss {:5.1}",
            t(&|x| x.value()),
            t(&|x| x.voronoi),
            t(&|x| x.gold),
            t(&|x| x.income),
            t(&|x| x.exposure),
            t(&|x| x.debt),
            t(&|x| x.loss),
        );
    };
    row("ours", &|g| g.ours);
    row("theirs", &|g| g.theirs);
    score
}

/// The mean score of `ours` over `options.theirs`, each the mean over maps.
fn objective(ours: &str, options: &Options) -> f64 {
    let mut total = 0.0;
    for theirs in &options.theirs {
        let games = run(ours, theirs, options);
        total += games.iter().map(|g| g.scores[0] as f64).sum::<f64>() / games.len() as f64;
    }
    total / options.theirs.len() as f64
}

/// Values tried for a weight around its current value.
fn candidates(name: &str, current: i64) -> Vec<i64> {
    let (low, high, steps): (i64, i64, &[i64]) = match name {
        "hq_div" => {
            return [0, 1, 2, 4, 8]
                .into_iter()
                .filter(|&v| v != current)
                .collect()
        }
        "contest_cap" => (4, 20, &[-4, -2, 2, 4]),
        "horizon" => (8, 30, &[-4, -2, -1, 1, 2, 4]),
        "min_turns_left" => (1, 15, &[-3, -2, -1, 1, 2, 3]),
        "contact_div" => (1, 4, &[-1, 1]),
        "contact_offset" | "after_contact" | "safe_spot" => (0, 8, &[-2, -1, 1, 2]),
        _ => (0, 12, &[-2, -1, 1, 2]),
    };
    steps
        .iter()
        .map(|d| current + d)
        .filter(|v| (low..=high).contains(v))
        .collect()
}

/// Coordinate search over `econ`'s weights from `start`: each weight in turn
/// takes the value around it that raises the objective most, until a round
/// changes nothing. Prints each step.
fn tune(start: &str, options: &Options) {
    let Some(mut best) = econ_params(start) else {
        return;
    };
    let mut best_score = objective(&econ_spec(&best), options);
    println!("start {} : {best_score:+.2}", econ_spec(&best));
    for round in 1..=4 {
        let mut changed = false;
        for name in KNOBS {
            let current = knob(&mut best, name).map_or(0, |k| k.get());
            for value in candidates(name, current) {
                let mut trial = best;
                if let Some(mut k) = knob(&mut trial, name) {
                    k.set(value);
                }
                let score = objective(&econ_spec(&trial), options);
                // A change must gain at least a tenth of a gold.
                if score > best_score + 0.1 {
                    best = trial;
                    best_score = score;
                    changed = true;
                    println!(
                        "round {round}: {name}={value} -> {best_score:+.2}  ({})",
                        econ_spec(&best)
                    );
                }
            }
        }
        if !changed {
            break;
        }
    }
    println!("best {} : {best_score:+.2}", econ_spec(&best));
}

/// For one game (a map, our seat, an opponent): for each candidate weight
/// set, the rollout value the chooser sees on our first turn and the score
/// the set then gets.
fn candidate_table(
    sets: &[Params],
    theirs: &str,
    seed: u64,
    seat: usize,
    options: &Options,
) -> Vec<(i64, i64)> {
    let mut root = start(seed);
    if seat == 1 {
        let mut first = player(theirs).unwrap_or_else(|| Box::new(Rules(econ::rules_turn)));
        let commands = first.turn(&root);
        root.play_turn(&commands);
    }
    let main = Settings {
        turns: options.turns[0],
        ..Settings::DEFAULT
    };
    sets.iter()
        .map(|params| {
            let seen = rollout::value_of(&root, params, &Params::V003, Settings::DEFAULT);
            // The game from the root: we play `params`, they play `theirs`.
            let mut state = root.clone();
            let mut opponent = player(theirs).unwrap_or_else(|| Box::new(Rules(econ::rules_turn)));
            let mut first = true;
            while state.outcome().is_none() && state.game_turn() < options.cap {
                if !first && middle::reached(&state) {
                    break;
                }
                first = false;
                let commands = if state.to_act() == seat {
                    econ::rules_turn_with(&state, params)
                } else {
                    opponent.turn(&state)
                };
                state.play_turn(&commands);
            }
            (seen, score::score(&state, seat, main))
        })
        .collect()
}

/// Greedy portfolio: from the candidate sets of `path`, adds one at a time
/// the set that most raises the mean score of a chooser over the maps and
/// opponents, up to `options.sets` sets. Prints each step and the portfolio.
fn build_portfolio(path: &str, options: &Options) {
    let Some(sets) = portfolio(path) else {
        eprintln!("cannot read the candidates of {path}");
        return;
    };
    let jobs: Vec<(String, u64, usize)> = options
        .theirs
        .iter()
        .flat_map(|t| {
            (0..options.maps)
                .flat_map(move |i| [0, 1].map(|seat| (t.clone(), options.first_seed + i, seat)))
        })
        .collect();
    let next = AtomicUsize::new(0);
    let tables = Mutex::new(Vec::with_capacity(jobs.len()));
    std::thread::scope(|scope| {
        for _ in 0..options.threads {
            scope.spawn(|| loop {
                let index = next.fetch_add(1, Ordering::Relaxed);
                let Some((theirs, seed, seat)) = jobs.get(index) else {
                    break;
                };
                let table = candidate_table(&sets, theirs, *seed, *seat, options);
                tables.lock().unwrap_or_else(|e| e.into_inner()).push(table);
            });
        }
    });
    let tables = tables.into_inner().unwrap_or_else(|e| e.into_inner());
    let n = tables.len() as f64;
    // The mean score of a chooser over `chosen`.
    let mean = |chosen: &[usize]| {
        tables
            .iter()
            .map(|t| {
                let pick = chosen
                    .iter()
                    .copied()
                    .max_by_key(|&i| (t[i].0, std::cmp::Reverse(i)))
                    .unwrap_or(0);
                t[pick].1 as f64
            })
            .sum::<f64>()
            / n
    };
    for (i, set) in sets.iter().enumerate().take(3) {
        println!("set {i} alone: {:+.2}  {}", mean(&[i]), econ_spec(set));
    }
    let oracle = tables
        .iter()
        .map(|t| t.iter().map(|x| x.1).max().unwrap_or(0) as f64)
        .sum::<f64>()
        / n;
    println!("oracle over all {} sets: {oracle:+.2}", sets.len());
    let mut chosen: Vec<usize> = Vec::new();
    while chosen.len() < options.sets.min(sets.len()) {
        let best = (0..sets.len())
            .filter(|i| !chosen.contains(i))
            .map(|i| {
                let mut trial = chosen.clone();
                trial.push(i);
                (mean(&trial), i)
            })
            .max_by(|a, b| a.0.total_cmp(&b.0).then(b.1.cmp(&a.1)));
        let Some((value, index)) = best else {
            break;
        };
        chosen.push(index);
        println!(
            "{} sets: {value:+.2}  + {}",
            chosen.len(),
            econ_spec(&sets[index])
        );
    }
    println!("--- portfolio");
    for &i in &chosen {
        println!("{}", econ_spec(&sets[i]));
    }
}

fn main() -> ExitCode {
    let options = match parse_args() {
        Ok(options) => options,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(2);
        }
    };
    if let Some(path) = &options.build_portfolio {
        build_portfolio(path, &options);
        return ExitCode::SUCCESS;
    }
    if let Some(start) = &options.tune {
        tune(start, &options);
        return ExitCode::SUCCESS;
    }
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
