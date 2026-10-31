//! `plan` and `merge`, the nightly league's commands that play nothing
//! (ADR 0049); `play` is in the incremental league's module, since it uses
//! its `Run`.

use super::*;
use crate::league::Seeds;
use crate::night;

/// Runs `plan`: writes tonight's pairings to `--out` and reports them.
pub(super) fn run_plan(args: &PlanArgs) -> Result<bool, String> {
    let stored = league::read_results(&args.results)?;
    // An empty value (`--fixed-opponents ""`) means none.
    let names = |list: &[String]| -> Vec<String> {
        list.iter()
            .filter(|name| !name.is_empty())
            .cloned()
            .collect()
    };
    let settings = night::Settings {
        pool: names(&args.pool),
        releases: names(&args.releases),
        anchor: args.anchor.clone(),
        fixed: names(&args.fixed_opponents),
        nearest: args.nearest,
        new_pairs: args.new_pairs,
        budget_pairs: args.budget_pairs,
        block_pairs: args.block_pairs,
        top: args.top,
        target_margin: args.target_margin,
        low_share: args.low_share,
        night: args.night.clone(),
        seed: args.seed,
        shards: args.shards,
    };
    let plan = night::plan(&stored, &settings)?;
    let mut text = String::new();
    for line in &plan.lines {
        text.push_str(&line.to_line());
        text.push('\n');
    }
    write_atomically(&args.out, text.as_bytes())?;
    for line in &plan.report {
        println!("{line}");
    }
    Ok(true)
}

/// Runs `merge`: the stored lines, then the added ones, to `--out`.
pub(super) fn run_merge(args: &MergeArgs) -> Result<bool, String> {
    for line in merge(args)? {
        println!("{line}");
    }
    Ok(true)
}

/// Writes the merged file and returns what to print: the drift reports, then
/// the summary line. Nothing is written on an error.
pub(super) fn merge(args: &MergeArgs) -> Result<Vec<String>, String> {
    let mut seeds = Seeds::default();
    let (stored_text, stored) = read_lines(&args.results, &mut seeds)?;
    let mut text = lines_of(&stored_text);
    let mut added: Vec<PairingResult> = Vec::new();
    for path in &args.add {
        let (added_text, lines) = read_lines(path, &mut seeds)?;
        text.push_str(&lines_of(&added_text));
        added.extend(lines);
    }
    let mut report = Vec::new();
    for line in &added {
        if let Some(drift) = league::drift(&stored, line) {
            report.extend(drift.drift_lines(&line.a, &line.b));
        }
    }
    let total = stored.len() + added.len();
    write_atomically(&args.out, text.as_bytes())?;
    report.push(format!(
        "merged {} new lines into {} ({total} lines in all)",
        added.len(),
        args.out.display()
    ));
    Ok(report)
}

/// A results file's text and its parsed lines, the seeds checked against
/// those of the files read before.
fn read_lines(path: &Path, seeds: &mut Seeds) -> Result<(String, Vec<PairingResult>), String> {
    let text =
        fs::read_to_string(path).map_err(|err| format!("cannot read {}: {err}", path.display()))?;
    let lines = league::parse_results_with(&text, &path.display().to_string(), seeds)?;
    Ok((text, lines))
}

/// The non-blank lines of `text` as written, each ending with a newline.
fn lines_of(text: &str) -> String {
    let mut kept = String::new();
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        kept.push_str(line);
        kept.push('\n');
    }
    kept
}

/// Writes `bytes` to `path` through a temporary file in the same folder and
/// a rename, so that a failure leaves `path` as it was.
fn write_atomically(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let folder = match path.parent() {
        Some(folder) if !folder.as_os_str().is_empty() => folder,
        _ => Path::new("."),
    };
    let name = path
        .file_name()
        .ok_or_else(|| format!("{} is not a file name", path.display()))?
        .to_string_lossy();
    let temporary = folder.join(format!(".{name}.{}.tmp", std::process::id()));
    let written = File::create(&temporary)
        .and_then(|mut file| file.write_all(bytes).and_then(|()| file.sync_all()))
        .and_then(|()| fs::rename(&temporary, path));
    written.map_err(|err| {
        let _ = fs::remove_file(&temporary);
        format!("cannot write {}: {err}", path.display())
    })
}
