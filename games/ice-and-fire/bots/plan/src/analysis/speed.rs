//! `speed`: the time of the plan search alone on a bank of positions, at a
//! fixed number of tries, so that a change meant only to make the search
//! faster can be timed and checked to answer exactly as before.
//!
//! Each position's search runs `--reps` times with the same tries and no
//! deadline; the fastest run counts, since a busy machine only slows runs
//! down. The answers do not depend on the machine (ADR 0025): `--answers`
//! writes them, a first line with the tries then one line a position (a
//! position left out keeps its line, saying why), and `--expect` compares
//! them with a file written so before, the run failing if any line differs,
//! the tries included.

use std::fmt::Write as _;
use std::time::Instant;

use ice_and_fire_engine::answer_line;

use super::{read_bank, ready, Position};
use crate::search::{think, Limit};

/// What the search did on one position.
pub struct Timed {
    /// The fastest run, in microseconds.
    pub micros: f64,
    /// Plans scored by our climbs (the tries).
    pub evals: u64,
    /// Climbs started.
    pub climbs: usize,
    /// The answer, as the bot prints it.
    pub answer: String,
}

/// Times the search on `position` with `tries` tries, the fastest of
/// `reps` runs.
pub fn time(position: &Position, tries: u64, reps: u32) -> Result<Timed, String> {
    let mut best: Option<Timed> = None;
    for _ in 0..reps.max(1) {
        let (state, mut opening) = ready(position)?;
        let limit = Limit {
            soft: None,
            hard: None,
            max_evals: tries,
        };
        let start = Instant::now();
        let thought = think(&state, limit, &mut opening);
        let micros = start.elapsed().as_secs_f64() * 1e6;
        if best.as_ref().is_none_or(|b| micros < b.micros) {
            best = Some(Timed {
                micros,
                evals: thought.evals,
                climbs: thought.climbs,
                answer: answer_line(&thought.commands),
            });
        }
    }
    best.ok_or_else(|| "no run".to_string())
}

/// One line of `--answers`: the position's name, its ply and the answer.
pub fn answer(position: &Position, timed: &Timed) -> String {
    format!("{}\t{}\t{}", position.name, position.ply, timed.answer)
}

/// The Markdown report of `timed` (with their positions), `skipped`
/// positions left out, and the lines of `--expect` that differ.
pub fn report(
    timed: &[(&Position, Timed)],
    skipped: usize,
    tries: u64,
    reps: u32,
    differ: Option<&[String]>,
) -> String {
    let mut out = String::new();
    let n = timed.len();
    let total_us: f64 = timed.iter().fold(0.0, |sum, (_, t)| sum + t.micros);
    let evals: u64 = timed.iter().map(|(_, t)| t.evals).sum();
    let climbs: usize = timed.iter().map(|(_, t)| t.climbs).sum();
    let mut each: Vec<f64> = timed.iter().map(|(_, t)| t.micros / 1000.0).collect();
    each.sort_by(f64::total_cmp);
    let median = each.get(n / 2).copied().unwrap_or(0.0);
    let mean = if n == 0 {
        0.0
    } else {
        total_us / 1000.0 / n as f64
    };
    let per_ms = if total_us > 0.0 {
        evals as f64 / (total_us / 1000.0)
    } else {
        0.0
    };
    let _ = writeln!(out, "# Search speed\n");
    let _ = writeln!(
        out,
        "{n} positions ({skipped} left out), {tries} tries a search, the fastest of {reps} runs.\n"
    );
    let _ = writeln!(out, "| Measure | Value |\n| --- | ---: |");
    let _ = writeln!(out, "| All positions, ms | {:.1} |", total_us / 1000.0);
    let _ = writeln!(out, "| A position, median ms | {median:.2} |");
    let _ = writeln!(out, "| A position, mean ms | {mean:.2} |");
    let _ = writeln!(out, "| Tries a ms | {per_ms:.1} |");
    let _ = writeln!(
        out,
        "| Climbs a position, mean | {:.1} |",
        if n == 0 {
            0.0
        } else {
            climbs as f64 / n as f64
        }
    );
    if let Some(differ) = differ {
        if differ.is_empty() {
            let _ = writeln!(out, "\nAnswers: all {n} as expected.");
        } else {
            let _ = writeln!(
                out,
                "\nAnswers: {} of {n} differ from those expected; the first:\n",
                differ.len()
            );
            for line in differ.iter().take(5) {
                let _ = writeln!(out, "- {line}");
            }
        }
    }
    out
}

/// The lines of `got` that differ from `expected`, line by line, and the
/// lines one has and the other lacks.
pub fn compare(got: &[String], expected: &str) -> Vec<String> {
    let expected: Vec<&str> = expected.lines().collect();
    let mut differ = Vec::new();
    for (index, line) in got.iter().enumerate() {
        match expected.get(index) {
            Some(want) if *want == line.as_str() => {}
            Some(want) => differ.push(format!("line {}: `{line}`, expected `{want}`", index + 1)),
            None => differ.push(format!("line {}: `{line}`, not expected", index + 1)),
        }
    }
    for (index, want) in expected.iter().enumerate().skip(got.len()) {
        differ.push(format!("line {}: missing, expected `{want}`", index + 1));
    }
    differ
}

/// Runs `speed` with its arguments (after the word `speed`): the report,
/// and whether the answers are as expected (true without `--expect`).
pub fn run(args: &[String]) -> Result<(String, bool), String> {
    let mut positions = None;
    let mut tries = 600u64;
    let mut reps = 3u32;
    let mut answers = None;
    let mut expect = None;
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
            "--answers" => answers = Some(value()?),
            "--expect" => expect = Some(value()?),
            other => return Err(format!("unknown argument {other}")),
        }
    }
    if tries == 0 {
        return Err("--tries must be at least 1".into());
    }
    let path = positions.ok_or("speed needs --positions FILE")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let expected = match &expect {
        Some(file) => Some(std::fs::read_to_string(file).map_err(|e| format!("{file}: {e}"))?),
        None => None,
    };
    let bank = read_bank(&text)?;
    let mut timed = Vec::new();
    let mut lines = vec![format!("# tries {tries}")];
    let mut skipped = 0;
    for position in &bank {
        match time(position, tries, reps) {
            Ok(t) => {
                lines.push(answer(position, &t));
                timed.push((position, t));
            }
            Err(err) => {
                eprintln!("left out: {err}");
                lines.push(format!("{}\t{}\tleft out", position.name, position.ply));
                skipped += 1;
            }
        }
    }
    if let Some(file) = answers {
        let mut body = lines.join("\n");
        body.push('\n');
        std::fs::write(&file, body).map_err(|e| format!("{file}: {e}"))?;
    }
    let differ = expected.map(|e| compare(&lines, &e));
    let same = differ.as_ref().is_none_or(Vec::is_empty);
    Ok((
        report(&timed, skipped, tries, reps, differ.as_deref()),
        same,
    ))
}
