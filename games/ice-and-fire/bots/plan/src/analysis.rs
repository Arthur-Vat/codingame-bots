//! The plan bot's analysis mode, built with `--features analysis` only: the
//! bundler drops the lines that declare and call this module, so bots never
//! carry it.
//!
//! ```sh
//! cargo +1.90.0 run --release -p ice-and-fire-bot-plan --features analysis -- \
//!   bank --positions FILE [--tries 400] [--reps 200] [--out FILE.tsv]
//! ```
//!
//! `bank` reads a bank of positions (`ice-and-fire-review bank` writes one)
//! and measures, on each, the evaluation's cost in microseconds and the plan
//! the search chooses, with the whole evaluation and with one of its terms
//! removed ([`Variant`]). The search runs a fixed number of tries with no
//! deadline, so the result does not depend on the machine (ADR 0025). It
//! prints a Markdown report; `--out` also writes one line a position.
//!
//! A bank is text, one block a position:
//!
//! ```text
//! position NAME PLY GAME_TURN
//! <the initial input, as CodinGame sends it>
//! turn
//! <the turn's input of the player to act>
//! end
//! ```

#[cfg(test)]
mod tests;

use std::cell::Cell;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::hint::black_box;
use std::time::Instant;

use ice_and_fire_engine::{Command, League, State};
use ice_and_fire_tactics::eval::evaluate;

use crate::eval::eval;
use crate::search::{think, Limit};

/// The evaluation with one of its terms removed, for the whole search (both
/// our plans and the opponent's replies).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Variant {
    /// v009's evaluation, unchanged.
    Full,
    /// Without the pressure term: how far each side is from affording the
    /// chain to the other's HQ (the bot's own since E021,
    /// `crate::eval::pressure`).
    NoPressure,
    /// Without the static value of active towers, 10 each.
    NoTowers,
}

thread_local! {
    static VARIANT: Cell<Variant> = const { Cell::new(Variant::Full) };
}

/// Sets the variant the evaluation plays with on this thread.
pub fn set_variant(variant: Variant) {
    VARIANT.with(|v| v.set(variant));
}

/// The static value of an active tower in the shared evaluation.
pub const TOWER_VALUE: i64 = 10;

/// The active towers of `seat`.
pub fn towers(state: &State, seat: usize) -> i64 {
    i64::from((state.towers() & state.active(seat)).count())
}

/// The bot's pressure term of the score of `seat`, which replaces the shared
/// one (`crate::eval::pressure`, E021).
pub fn pressure_term(state: &State, seat: usize) -> i64 {
    crate::eval::pressure(state, seat) - crate::eval::pressure(state, 1 - seat)
}

/// The tower term of the score of `seat`.
pub fn tower_term(state: &State, seat: usize) -> i64 {
    TOWER_VALUE * (towers(state, seat) - towers(state, 1 - seat))
}

/// What the variant of this thread removes from the score of `seat` in a
/// game that is not over; `crate::eval::eval` subtracts it.
pub fn removed(state: &State, seat: usize) -> i64 {
    match VARIANT.with(Cell::get) {
        Variant::Full => 0,
        Variant::NoPressure => pressure_term(state, seat),
        Variant::NoTowers => tower_term(state, seat),
    }
}

/// One position of a bank.
pub struct Position {
    pub name: String,
    pub ply: usize,
    pub game_turn: u32,
    pub initial: String,
    pub turn: String,
}

impl Position {
    pub fn state(&self) -> Result<State, String> {
        State::read(&self.initial, &self.turn, League::Bronze, self.game_turn)
    }
}

/// Reads a bank (see the module's docs).
pub fn read_bank(text: &str) -> Result<Vec<Position>, String> {
    let mut out = Vec::new();
    let mut lines = text.lines().enumerate();
    while let Some((at, line)) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        let bad = || format!("line {}: expected `position NAME PLY GAME_TURN`", at + 1);
        if words.len() != 4 || words[0] != "position" {
            return Err(bad());
        }
        let ply = words[2].parse().map_err(|_| bad())?;
        let game_turn = words[3].parse().map_err(|_| bad())?;
        let mut part = |end: &str| -> Result<String, String> {
            let mut body = String::new();
            for (_, line) in lines.by_ref() {
                if line == end {
                    return Ok(body);
                }
                body.push_str(line);
                body.push('\n');
            }
            Err(format!("{}: no `{end}` line", words[1]))
        };
        let initial = part("turn")?;
        let turn = part("end")?;
        out.push(Position {
            name: words[1].to_string(),
            ply,
            game_turn,
            initial,
            turn,
        });
    }
    Ok(out)
}

/// The kind of a plan's first command.
pub fn kind(commands: &[Command]) -> &'static str {
    match commands.first() {
        None | Some(Command::Wait) => "nothing",
        Some(Command::Move { .. }) => "move",
        Some(Command::Train { level: 1, .. }) => "train 1",
        Some(Command::Train { level: 2, .. }) => "train 2",
        Some(Command::Train { .. }) => "train 3",
        Some(Command::BuildMine { .. }) => "mine",
        Some(Command::BuildTower { .. }) => "tower",
    }
}

/// The towers a plan builds.
fn towers_built(commands: &[Command]) -> usize {
    commands
        .iter()
        .filter(|c| matches!(c, Command::BuildTower { .. }))
        .count()
}

/// The plan the search chooses for the player to act in `position`, with
/// `variant` and `tries` tries, the opening being over (as it is by the
/// middle game).
pub fn plan(position: &Position, variant: Variant, tries: u64) -> Result<Vec<Command>, String> {
    let state = position.state()?;
    // The opening ends for good once the player can pay a chain to an enemy
    // cell: shown a copy of the position with ample gold, it is over.
    let rich_turn = match position.turn.split_once('\n') {
        Some((_, rest)) => format!("100000\n{rest}"),
        None => return Err(format!("{}: an empty turn", position.name)),
    };
    let rich = State::read(
        &position.initial,
        &rich_turn,
        League::Bronze,
        position.game_turn,
    )?;
    let mut opening = crate::opening();
    if opening.playing(&rich) {
        return Err(format!("{}: the opening does not end", position.name));
    }
    set_variant(variant);
    let limit = Limit {
        soft: None,
        hard: None,
        max_evals: tries,
    };
    let thought = think(&state, limit, &mut opening);
    set_variant(Variant::Full);
    Ok(thought.commands)
}

/// Mean microseconds of one call of `f`, over `reps` calls.
fn micros(reps: u32, mut f: impl FnMut() -> i64) -> f64 {
    let start = Instant::now();
    for _ in 0..reps {
        black_box(f());
    }
    start.elapsed().as_secs_f64() * 1e6 / f64::from(reps.max(1))
}

/// The median and the mean of `values`, 0 when empty.
fn median_mean(values: &mut [f64]) -> (f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0);
    }
    values.sort_by(f64::total_cmp);
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    (values[values.len() / 2], mean)
}

/// The searches of a position: a name, the variant and the tries as a
/// multiple of the bank's. The second is a control: v009 with twice the
/// tries, to show how much a plan changes from the search alone.
pub const RUNS: [(&str, Variant, u64); 4] = [
    ("v009", Variant::Full, 1),
    ("v009, twice the tries", Variant::Full, 2),
    ("no pressure", Variant::NoPressure, 1),
    ("no tower value", Variant::NoTowers, 1),
];

/// What the bank measured on one position.
pub struct Measure {
    /// Microseconds of the bot's evaluation, of the shared one, and of the
    /// two pressure terms, for the player to act.
    pub micros: [f64; 3],
    /// The pressure and tower terms of the player to act.
    pub terms: [i64; 2],
    /// The plan of each search of [`RUNS`].
    pub plans: Vec<Vec<Command>>,
}

/// Measures one position.
pub fn measure(position: &Position, tries: u64, reps: u32) -> Result<Measure, String> {
    let state = position.state()?;
    let me = state.to_act();
    let micros = [
        micros(reps, || eval(black_box(&state), me)),
        micros(reps, || evaluate(black_box(&state), me)),
        micros(reps, || pressure_term(black_box(&state), me)),
    ];
    let mut plans = Vec::new();
    for (_, variant, times) in RUNS {
        plans.push(plan(position, variant, tries * times)?);
    }
    Ok(Measure {
        micros,
        terms: [pressure_term(&state, me), tower_term(&state, me)],
        plans,
    })
}

/// The commands of a plan in a fixed order, to compare plans that differ
/// only in the order of their commands.
fn as_set(commands: &[Command]) -> Vec<String> {
    let mut words: Vec<String> = commands.iter().map(Command::to_text).collect();
    words.sort();
    words
}

/// The Markdown report of `measures`, `skipped` positions left out.
pub fn report(measures: &[Measure], skipped: usize, tries: u64, reps: u32) -> String {
    let mut out = String::new();
    let n = measures.len();
    let pct = |k: usize| {
        if n == 0 {
            0.0
        } else {
            100.0 * k as f64 / n as f64
        }
    };
    let _ = writeln!(out, "# Position bank\n");
    let _ = writeln!(
        out,
        "{n} positions ({skipped} left out), {tries} tries a search, {reps} calls a timing.\n"
    );
    let _ = writeln!(out, "## The evaluation's cost\n");
    let _ = writeln!(
        out,
        "| Function | Median µs | Mean µs |\n| --- | ---: | ---: |"
    );
    for (index, what) in [
        "The bot's evaluation (`eval`)",
        "The shared evaluation (`evaluate`)",
        "Its pressure term (both sides)",
    ]
    .iter()
    .enumerate()
    {
        let mut values: Vec<f64> = measures.iter().map(|m| m.micros[index]).collect();
        let (median, mean) = median_mean(&mut values);
        let _ = writeln!(out, "| {what} | {median:.2} | {mean:.2} |");
    }
    let nonzero = |index: usize| measures.iter().filter(|m| m.terms[index] != 0).count();
    let mean_abs = |index: usize| {
        if n == 0 {
            0.0
        } else {
            measures
                .iter()
                .map(|m| m.terms[index].abs() as f64)
                .sum::<f64>()
                / n as f64
        }
    };
    let _ = writeln!(out, "\n## The terms in the positions\n");
    let _ = writeln!(
        out,
        "| Term | Positions where it is not 0 | Mean absolute value |\n| --- | ---: | ---: |"
    );
    for (index, what) in ["Pressure", "Towers"].iter().enumerate() {
        let k = nonzero(index);
        let _ = writeln!(
            out,
            "| {what} | {k} ({:.1}%) | {:.1} |",
            pct(k),
            mean_abs(index)
        );
    }
    let _ = writeln!(out, "\n## The plans\n");
    let _ = writeln!(
        out,
        "| Search | First command differs from v009's | Plan differs | Plan differs as a set of commands | Plans with a tower | Towers in all plans |\n| --- | ---: | ---: | ---: | ---: | ---: |"
    );
    for (index, (name, _, _)) in RUNS.iter().enumerate() {
        let first = measures
            .iter()
            .filter(|m| m.plans[index].first() != m.plans[0].first())
            .count();
        let whole = measures
            .iter()
            .filter(|m| m.plans[index] != m.plans[0])
            .count();
        let set = measures
            .iter()
            .filter(|m| as_set(&m.plans[index]) != as_set(&m.plans[0]))
            .count();
        let with = measures
            .iter()
            .filter(|m| towers_built(&m.plans[index]) > 0)
            .count();
        let all: usize = measures.iter().map(|m| towers_built(&m.plans[index])).sum();
        let _ = writeln!(
            out,
            "| {name} | {first} ({:.1}%) | {whole} ({:.1}%) | {set} ({:.1}%) | {with} ({:.1}%) | {all} |",
            pct(first),
            pct(whole),
            pct(set),
            pct(with)
        );
    }
    for (index, (name, _, _)) in RUNS.iter().enumerate().skip(1) {
        let mut changes: BTreeMap<(&str, &str), usize> = BTreeMap::new();
        for m in measures {
            if m.plans[index].first() != m.plans[0].first() {
                *changes
                    .entry((kind(&m.plans[0]), kind(&m.plans[index])))
                    .or_default() += 1;
            }
        }
        let mut changes: Vec<_> = changes.into_iter().collect();
        changes.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let _ = writeln!(
            out,
            "\n### First commands that change with {name}\n\n| v009's first command | Becomes | Positions |\n| --- | --- | ---: |"
        );
        for ((from, to), count) in changes {
            let _ = writeln!(out, "| {from} | {to} | {count} |");
        }
    }
    out
}

/// One line of `--out`: the position, its timings and terms, and each
/// variant's plan.
fn line(position: &Position, m: &Measure) -> String {
    let mut text = format!(
        "{}\t{}\t{}\t{:.3}\t{:.3}\t{:.3}\t{}\t{}",
        position.name,
        position.ply,
        position.game_turn,
        m.micros[0],
        m.micros[1],
        m.micros[2],
        m.terms[0],
        m.terms[1]
    );
    for plan in &m.plans {
        let _ = write!(text, "\t{}", ice_and_fire_engine::answer_line(plan));
    }
    text
}

/// Runs `bank` with its arguments (after the word `bank`).
pub fn bank(args: &[String]) -> Result<String, String> {
    let mut positions = None;
    let mut tries = 400u64;
    let mut reps = 200u32;
    let mut out = None;
    let mut args = args.iter();
    while let Some(arg) = args.next() {
        let mut value = || {
            args.next()
                .cloned()
                .ok_or_else(|| format!("{arg} needs a value"))
        };
        match arg.as_str() {
            "--positions" => positions = Some(value()?),
            "--tries" => tries = value()?.parse().map_err(|_| "bad --tries")?,
            "--reps" => reps = value()?.parse().map_err(|_| "bad --reps")?,
            "--out" => out = Some(value()?),
            other => return Err(format!("unknown argument {other}")),
        }
    }
    let path = positions.ok_or("bank needs --positions FILE")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let bank = read_bank(&text)?;
    let mut measures = Vec::new();
    let mut lines = String::from(
        "name\tply\tgame_turn\teval_us\tevaluate_us\tpressure_us\tpressure\ttowers\tv009\tv009_twice\tno_pressure\tno_towers\n",
    );
    let mut skipped = 0;
    for (index, position) in bank.iter().enumerate() {
        match measure(position, tries, reps) {
            Ok(m) => {
                lines.push_str(&line(position, &m));
                lines.push('\n');
                measures.push(m);
            }
            Err(err) => {
                eprintln!("left out: {err}");
                skipped += 1;
            }
        }
        if (index + 1) % 500 == 0 {
            eprintln!("{} of {} positions", index + 1, bank.len());
        }
    }
    if let Some(file) = out {
        std::fs::write(&file, lines).map_err(|e| format!("{file}: {e}"))?;
    }
    Ok(report(&measures, skipped, tries, reps))
}

/// Runs the analysis mode and exits if the first argument names it
/// (`bank`); returns otherwise, and the bot plays.
pub fn main_if_asked() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) != Some("bank") {
        return;
    }
    match bank(&args[1..]) {
        Ok(text) => {
            print!("{text}");
            std::process::exit(0);
        }
        Err(err) => {
            eprintln!("plan bank: {err}");
            std::process::exit(2);
        }
    }
}
