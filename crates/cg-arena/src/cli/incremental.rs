//! `league --results`: the incremental league of ADR 0038. It reads the
//! stored pairings, plays a new release against opponents it chooses, and
//! appends each new pairing to a file in the results file's format. Also
//! `replay`, which plays stored pairings again with their stored seeds
//! (ADR 0048), and `play`, which plays the lines of a night's plan
//! (ADR 0049).

use super::*;
use crate::night;

/// Runs `league` with `--results`. `bots` are the parsed `--bot` options.
pub(super) fn run_incremental_league<F: RefereeFactory>(
    args: &LeagueArgs,
    bots: &[BotSpec],
    game: &str,
    new_referee: &F,
) -> Result<bool, String> {
    let (Some(results_path), Some(new), Some(new_results_path)) =
        (&args.results, &args.new, &args.new_results)
    else {
        return Err("--results needs --new and --new-results".to_string());
    };
    let stored = league::read_results(results_path)?;
    let names: Vec<String> = bots.iter().map(|bot| bot.name.clone()).collect();
    check_unique(&names)?;
    check_fault_name(&args.common, &names)?;
    if !names.contains(new) {
        return Err(format!("--new {new:?} is not one of the bots"));
    }
    let pool = league::known_bots(&stored, names.iter().map(String::as_str));
    let anchor = args.anchor.clone().unwrap_or_else(|| names[0].clone());
    if !pool.contains(&anchor) {
        return Err(format!(
            "--anchor {anchor:?} is not one of the bots or in the results file"
        ));
    }
    let candidates: Vec<String> = pool.iter().filter(|name| *name != new).cloned().collect();
    let previous = args.previous.as_deref();
    // An empty value (`--fixed-opponents ""`) means no fixed opponents.
    let fixed: Vec<String> = args
        .fixed_opponents
        .iter()
        .filter(|name| !name.is_empty())
        .cloned()
        .collect();
    for (option, name) in previous.iter().map(|name| ("--previous", *name)).chain(
        fixed
            .iter()
            .map(|name| ("--fixed-opponents", name.as_str())),
    ) {
        if name == new {
            return Err(format!("{option} {name:?} is the release being rated"));
        }
        if !candidates.iter().any(|known| known == name) {
            return Err(format!("{option} {name:?} is not a known bot"));
        }
    }
    let canary = match &args.canary {
        Some(spec) => Some(find_canary(spec, &stored, bots)?),
        None => None,
    };
    // The canary is compared with every stored line of its pair, combined.
    let combined = canary
        .as_ref()
        .and_then(|first| league::combined_pair_points(&stored, &first.a, &first.b));
    // Every opponent that may be chosen needs a command, whatever the choice.
    check_playable(bots, &candidates)?;
    let already = stored.iter().any(|pairing| pairing.involves(new));

    if args.plan {
        plan(
            args,
            &stored,
            &pool,
            &candidates,
            &fixed,
            &anchor,
            canary.as_ref(),
        )?;
        return Ok(true);
    }
    check_common(&args.common)?;
    if same_file(results_path, new_results_path) {
        return Err("--new-results must not be the --results file".to_string());
    }
    let sink = File::create(new_results_path)
        .map_err(|err| format!("cannot create {}: {err}", new_results_path.display()))?;

    let mut run = Run {
        common: &args.common,
        pairs: args.pairs,
        commit: args.commit.clone(),
        speed: args.speed,
        bots,
        new_referee,
        output: Output::create(args.common.out.as_deref())?,
        records: Records::create(&args.common, game, "arena league")?,
        sink,
        sink_path: new_results_path,
        cpu: league::cpu_model(),
        played: Vec::new(),
        faults: HashMap::new(),
        games: HashMap::new(),
    };
    let mut opponents: Vec<String> = Vec::new();
    if already {
        println!("{new} already has stored pairings: none played");
    } else {
        match league::all_opponents(&candidates, previous, &fixed, args.nearest) {
            Some(all) => {
                for opponent in &all {
                    run.play_pairing(new, opponent)?;
                }
                opponents = all;
            }
            None => {
                if let Some(previous) = previous {
                    run.play_pairing(new, previous)?;
                }
                let pairings: Vec<PairingResult> =
                    stored.iter().chain(&run.played).cloned().collect();
                let ratings = league::rate(&pool, &pairings, &anchor)?;
                let estimate = rating_of(&ratings, new).unwrap_or(0.0);
                let chosen = league::choose_opponents(
                    &candidates,
                    previous,
                    &fixed,
                    args.nearest,
                    &ratings,
                    estimate,
                );
                let later = &chosen[run.played.len()..];
                for opponent in later {
                    run.play_pairing(new, opponent)?;
                }
                opponents = chosen;
            }
        }
    }
    if let (Some(first), Some(combined)) = (&canary, &combined) {
        run.play_canary(first, combined, new)?;
    }
    let Run {
        output,
        records,
        played,
        faults,
        games,
        ..
    } = run;
    output.finish()?;
    let records_result = records.finish();

    if !opponents.is_empty() {
        println!("Opponents of {new}: {}", opponents.join(", "));
    }
    let pairings: Vec<PairingResult> = stored.iter().chain(&played).cloned().collect();
    let table = league::rate(&pool, &pairings, &anchor)?;
    print_ratings(&table, &anchor);
    let faults: Vec<Faults> = names
        .iter()
        .map(|name| faults.get(name).copied().unwrap_or_default())
        .collect();
    let games: Vec<u32> = names
        .iter()
        .map(|name| games.get(name).copied().unwrap_or(0))
        .collect();
    let passed = faults_pass(&args.common, &names, &faults, &games);
    records_result.map(|()| passed)
}

/// Runs `replay`: plays each `--pairing` of the results file again, with
/// its stored seed, and appends it to `--new-results`.
pub(super) fn run_replay<F: RefereeFactory>(
    args: &ReplayArgs,
    game: &str,
    new_referee: &F,
) -> Result<bool, String> {
    let bots = args
        .bots
        .iter()
        .map(|spec| BotSpec::parse(spec))
        .collect::<Result<Vec<_>, _>>()?;
    let names: Vec<String> = bots.iter().map(|bot| bot.name.clone()).collect();
    check_unique(&names)?;
    check_fault_name(&args.common, &names)?;
    if args.pairs == Some(0) {
        return Err("--pairs must be at least 1".to_string());
    }
    let stored = league::read_results(&args.results)?;
    let mut chosen: Vec<PairingResult> = Vec::new();
    let mut given: Vec<(String, String)> = Vec::new();
    for spec in &args.pairings {
        let lines = find_pairings("--pairing", spec, &stored, &bots)?;
        let key = unordered(&lines[0].a, &lines[0].b);
        if given.contains(&key) {
            return Err(format!("--pairing {spec:?} is given twice"));
        }
        given.push(key);
        chosen.extend(lines);
    }
    check_common(&args.common)?;
    if same_file(&args.results, &args.new_results) {
        return Err("--new-results must not be the --results file".to_string());
    }
    let sink = File::create(&args.new_results)
        .map_err(|err| format!("cannot create {}: {err}", args.new_results.display()))?;
    let mut run = Run {
        common: &args.common,
        pairs: 0,
        commit: args.commit.clone(),
        speed: args.speed,
        bots: &bots,
        new_referee,
        output: Output::create(args.common.out.as_deref())?,
        records: Records::create(&args.common, game, "arena replay")?,
        sink,
        sink_path: &args.new_results,
        cpu: league::cpu_model(),
        played: Vec::new(),
        faults: HashMap::new(),
        games: HashMap::new(),
    };
    for pairing in &chosen {
        run.replay_pairing(pairing, args.pairs.unwrap_or(pairing.pairs))?;
    }
    let Run {
        output,
        records,
        faults,
        games,
        ..
    } = run;
    output.finish()?;
    let records_result = records.finish();
    let faults: Vec<Faults> = names
        .iter()
        .map(|name| faults.get(name).copied().unwrap_or_default())
        .collect();
    let games: Vec<u32> = names
        .iter()
        .map(|name| games.get(name).copied().unwrap_or(0))
        .collect();
    let passed = faults_pass(&args.common, &names, &faults, &games);
    records_result.map(|()| passed)
}

/// Runs `play`: plays the lines of a plan (those of `--shard` when given),
/// in file order, and appends each finished pairing to `--new-results`.
pub(super) fn run_play<F: RefereeFactory>(
    args: &PlayArgs,
    game: &str,
    new_referee: &F,
) -> Result<bool, String> {
    let bots = args
        .bots
        .iter()
        .map(|spec| BotSpec::parse(spec))
        .collect::<Result<Vec<_>, _>>()?;
    let names: Vec<String> = bots.iter().map(|bot| bot.name.clone()).collect();
    check_unique(&names)?;
    check_fault_name(&args.common, &names)?;
    let text = fs::read_to_string(&args.plan)
        .map_err(|err| format!("cannot read {}: {err}", args.plan.display()))?;
    let lines = night::parse_plan(&text, &args.plan.display().to_string())?;
    if let Some(shard) = args.shard {
        if !lines.iter().any(|line| line.shard == shard) {
            return Err(format!(
                "--shard {shard} matches no line of {}",
                args.plan.display()
            ));
        }
    }
    let chosen: Vec<&night::PlanLine> = lines
        .iter()
        .filter(|line| args.shard.is_none_or(|shard| line.shard == shard))
        .collect();
    for line in &chosen {
        for name in [&line.a, &line.b] {
            if !names.contains(name) {
                return Err(format!(
                    "{name:?} is in the plan but has no --bot, so it cannot be played"
                ));
            }
        }
    }
    check_common(&args.common)?;
    if same_file(&args.plan, &args.new_results) {
        return Err("--new-results must not be the --plan file".to_string());
    }
    let sink = File::create(&args.new_results)
        .map_err(|err| format!("cannot create {}: {err}", args.new_results.display()))?;
    let mut run = Run {
        common: &args.common,
        pairs: 0,
        commit: args.commit.clone(),
        speed: args.speed,
        bots: &bots,
        new_referee,
        output: Output::create(args.common.out.as_deref())?,
        records: Records::create(&args.common, game, "arena play")?,
        sink,
        sink_path: &args.new_results,
        cpu: league::cpu_model(),
        played: Vec::new(),
        faults: HashMap::new(),
        games: HashMap::new(),
    };
    println!("Playing {} of {} plan lines", chosen.len(), lines.len());
    for line in chosen {
        run.write_pairing(&line.a, &line.b, line.pairs, line.seed)?;
    }
    let Run {
        output,
        records,
        faults,
        games,
        ..
    } = run;
    output.finish()?;
    let records_result = records.finish();
    let faults: Vec<Faults> = names
        .iter()
        .map(|name| faults.get(name).copied().unwrap_or_default())
        .collect();
    let games: Vec<u32> = names
        .iter()
        .map(|name| games.get(name).copied().unwrap_or(0))
        .collect();
    let passed = faults_pass(&args.common, &names, &faults, &games);
    records_result.map(|()| passed)
}

/// The rating of `name`, when it is linked to the anchor.
fn rating_of(ratings: &[Rating], name: &str) -> Option<f64> {
    ratings
        .iter()
        .find(|rating| rating.name == name && rating.is_linked())
        .map(|rating| rating.rating)
}

/// Errors on the first of `names` that has no `--bot` to play with.
fn check_playable(bots: &[BotSpec], names: &[String]) -> Result<(), String> {
    for name in names {
        if !bots.iter().any(|bot| bot.name == *name) {
            return Err(format!(
                "{name:?} is in the results file but has no --bot; give a --bot for every bot of \
                 the file"
            ));
        }
    }
    Ok(())
}

/// The first stored pairing `A,B` names, in either order, which both need a
/// `--bot`.
pub(super) fn find_canary(
    spec: &str,
    stored: &[PairingResult],
    bots: &[BotSpec],
) -> Result<PairingResult, String> {
    let mut lines = find_pairings("--canary", spec, stored, bots)?;
    Ok(lines.swap_remove(0))
}

/// The two names in order, so that a pair is the same in either order.
fn unordered(x: &str, y: &str) -> (String, String) {
    if x <= y {
        (x.to_string(), y.to_string())
    } else {
        (y.to_string(), x.to_string())
    }
}

/// Every stored line of the pairing that `spec`, `A,B` in either order,
/// names for `option`, in file order; both bots need a `--bot`. At least one.
fn find_pairings(
    option: &str,
    spec: &str,
    stored: &[PairingResult],
    bots: &[BotSpec],
) -> Result<Vec<PairingResult>, String> {
    let (x, y) = spec
        .split_once(',')
        .filter(|(x, y)| !x.is_empty() && !y.is_empty() && !y.contains(','))
        .ok_or_else(|| format!("{option} {spec:?} must look like A,B"))?;
    let lines: Vec<PairingResult> = stored
        .iter()
        .filter(|pairing| (pairing.a == x && pairing.b == y) || (pairing.a == y && pairing.b == x))
        .cloned()
        .collect();
    let first = lines
        .first()
        .ok_or_else(|| format!("{option}: the results file has no pairing of {x} and {y}"))?;
    if lines.iter().any(|pairing| pairing.pairs == 0) {
        return Err(format!("{option}: a pairing of {x} and {y} has no pairs"));
    }
    check_playable(bots, &[first.a.clone(), first.b.clone()])?;
    Ok(lines)
}

/// Whether two paths are the same existing file.
fn same_file(first: &Path, second: &Path) -> bool {
    first == second
        || matches!(
            (fs::canonicalize(first), fs::canonicalize(second)),
            (Ok(x), Ok(y)) if x == y
        )
}

/// `--plan`: prints the opponents the run would choose, and the canary.
fn plan(
    args: &LeagueArgs,
    stored: &[PairingResult],
    pool: &[String],
    candidates: &[String],
    fixed: &[String],
    anchor: &str,
    canary: Option<&PairingResult>,
) -> Result<(), String> {
    let new = args.new.as_deref().unwrap_or_default();
    let previous = args.previous.as_deref();
    if stored.iter().any(|pairing| pairing.involves(new)) {
        println!("{new} already has stored pairings: none played");
    } else {
        match league::all_opponents(candidates, previous, fixed, args.nearest) {
            Some(all) => {
                println!("Opponents of {new}: {}", all.join(", "));
            }
            None => {
                let ratings = league::rate(pool, stored, anchor)?;
                let (estimate, source) = match previous {
                    Some(previous) => (
                        rating_of(&ratings, previous).unwrap_or(0.0),
                        format!("{previous}'s rating"),
                    ),
                    None => (0.0, "the anchor's rating, 0".to_string()),
                };
                let chosen = league::choose_opponents(
                    candidates,
                    previous,
                    fixed,
                    args.nearest,
                    &ratings,
                    estimate,
                );
                println!(
                    "Opponents of {new}, estimated from {source}; the real choice follows the \
                     first pairing: {}",
                    chosen.join(", ")
                );
            }
        }
    }
    if let Some(canary) = canary {
        println!("Canary: {} vs {}", canary.a, canary.b);
    }
    Ok(())
}

/// The games of an incremental league run or a replay.
struct Run<'a, F> {
    common: &'a CommonArgs,
    /// Pairs of each new pairing.
    pairs: u32,
    /// Written to each new line.
    commit: Option<String>,
    speed: Option<f64>,
    bots: &'a [BotSpec],
    new_referee: &'a F,
    output: Output,
    records: Records,
    /// The file the new pairings are appended to.
    sink: File,
    sink_path: &'a Path,
    cpu: String,
    /// The new pairings finished so far.
    played: Vec<PairingResult>,
    /// Faults and games by bot in this run, the canary included.
    faults: HashMap<String, Faults>,
    games: HashMap<String, u32>,
}

impl<F: RefereeFactory> Run<'_, F> {
    fn bot(&self, name: &str) -> Result<BotSpec, String> {
        self.bots
            .iter()
            .find(|bot| bot.name == name)
            .cloned()
            .ok_or_else(|| format!("{name:?} has no --bot, so it cannot be played"))
    }

    /// Plays `pairs` pairs of `a` against `b` with `seed` and prints the
    /// result line, tagging the game records with `tag`.
    fn play_summary(
        &mut self,
        a: &str,
        b: &str,
        pairs: u32,
        seed: u64,
        tag: &str,
    ) -> Result<(Summary, Tournament), String> {
        let mut tournament = tournament([self.bot(a)?, self.bot(b)?], pairs, self.common)?;
        tournament.seed = seed;
        let mut summary = Summary::new([a.to_string(), b.to_string()]);
        self.records.start_matchup(tag);
        play(
            &tournament,
            self.new_referee,
            &mut self.output,
            &mut self.records,
            |game| {
                summary.add(game);
                Flow::Continue
            },
        )?;
        print_pairing(&summary);
        for (name, found) in [a, b].into_iter().zip(summary.faults) {
            let total = self.faults.entry(name.to_string()).or_default();
            total.timeouts += found.timeouts;
            total.crashes += found.crashes;
            total.invalid += found.invalid;
            *self.games.entry(name.to_string()).or_default() += summary.games();
        }
        Ok((summary, tournament))
    }

    /// Plays the new release `new` against `opponent` and appends the
    /// pairing to the new results file.
    fn play_pairing(&mut self, new: &str, opponent: &str) -> Result<(), String> {
        let seed = league::pairing_seed(self.common.seed, new, opponent);
        self.write_pairing(new, opponent, self.pairs, seed)
    }

    /// Plays the stored pairing `stored` again with its seed, `pairs` pairs,
    /// and appends it to the new results file.
    fn replay_pairing(&mut self, stored: &PairingResult, pairs: u32) -> Result<(), String> {
        self.write_pairing(&stored.a, &stored.b, pairs, stored.seed)
    }

    /// Plays `pairs` pairs of `a` against `b` with `seed` and appends the
    /// pairing to the new results file.
    fn write_pairing(&mut self, a: &str, b: &str, pairs: u32, seed: u64) -> Result<(), String> {
        let (summary, tournament) = self.play_summary(a, b, pairs, seed, &format!("{a}-vs-{b}"))?;
        let common = self.common;
        let pairing = PairingResult {
            a: a.to_string(),
            b: b.to_string(),
            pairs: summary.pairs.iter().sum(),
            wins: summary.wins,
            draws: summary.draws,
            losses: summary.losses,
            pair_points: summary.pairs,
            faults: [summary.faults[0].total(), summary.faults[1].total()],
            time_scale: common.time_scale,
            time_tolerance_ms: common.time_tolerance_ms,
            opening_plies: tournament.opening_plies,
            seed: tournament.seed,
            date: league::today(),
            commit: self.commit.clone().unwrap_or_else(|| "unknown".to_string()),
            cpu: self.cpu.clone(),
            speed: self.speed,
        };
        pairing.validate()?;
        writeln!(self.sink, "{}", pairing.to_line())
            .and_then(|()| self.sink.flush())
            .map_err(|err| format!("cannot write {}: {err}", self.sink_path.display()))?;
        self.played.push(pairing);
        Ok(())
    }

    /// Replays the stored pairing `first` (the first line of its pair) with
    /// the canary seed of the release `new` and compares it with the pair
    /// points of all the pair's stored lines, `combined`, seen from the
    /// first line's `a`. The replay is never written to the new results file.
    fn play_canary(
        &mut self,
        first: &PairingResult,
        combined: &[u32; 5],
        new: &str,
    ) -> Result<(), String> {
        let (a, b) = (first.a.as_str(), first.b.as_str());
        let seed = league::canary_seed(self.common.seed, a, b, new);
        let (summary, _) =
            self.play_summary(a, b, first.pairs, seed, &format!("canary-{a}-vs-{b}"))?;
        let report = league::compare_pair_points(combined, &summary.pairs)
            .ok_or_else(|| format!("the canary of {a} and {b} has no complete pair"))?;
        println!("{}", report.line(a, b));
        if !report.within() {
            println!(
                "warning: canary {a} vs {b} scored outside its stored interval: the runner's \
                 speed may have changed"
            );
        }
        Ok(())
    }
}
