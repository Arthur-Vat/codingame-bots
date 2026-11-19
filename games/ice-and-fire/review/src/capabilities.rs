//! Capability tests (stage A of the curriculum in
//! [research/codingame/2026-11-18-zasmu-strategy.md]): positions from the
//! CodinGame games, each with one decision that a capability decides, and a
//! property of the answer that says whether a bot made it
//! (`games/ice-and-fire/capabilities/README.md`).
//!
//! `codingame-capability-suite` draws the positions from a data directory of
//! `scripts/codingame.sh` and writes the suite, one test a line;
//! `codingame-capabilities` puts each bot in the mover's seat of each test,
//! feeds it the game's inputs from the first turn up to the position, and
//! reports the share of the tests it passes, by capability, beside the
//! answers the players actually sent.
//!
//! P is the side to act and O its opponent. A chain is the cheapest chain
//! of trainings to the enemy HQ ([`ice_and_fire_tactics::chain`]), its
//! side's units stepping first. Its cost and a side's gold and income are
//! those of the state, so "in k turns" means gold plus k incomes as they
//! stand.
//!
//! - **finish**: P can pay its chain now. Passes if the answer takes O's HQ.
//! - **save-k** (k = 1, 2): P cannot pay its chain now but can with k turns
//!   of income, the fewest that do, and O cannot pay its own at its next
//!   turn. Passes if, after the answer, P's gold plus k incomes still pays
//!   its chain recomputed on the new state, or the answer took O's HQ.
//! - **defend-k** (k = 1, 2): O can pay its chain at its k-th next turn, the
//!   first that does, and P cannot pay its own now. Passes if, after the
//!   answer, O's gold (its income of the next turn included) plus k - 1
//!   incomes no longer pays O's chain recomputed on the new state, or the
//!   answer took O's HQ.
//! - **wall**: the reference player (zasmu) was P and built a tower on or
//!   next to one of its frontier cells, and P cannot pay its chain now.
//!   Passes if, after the answer, O can take no more of P's active cells
//!   with the gold of its next turn than after the reference's own answer,
//!   or the answer took O's HQ.
//!
//! [research/codingame/2026-11-18-zasmu-strategy.md]: ../../research/codingame/2026-11-18-zasmu-strategy.md

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use cg_arena::live::{BotSettings, LiveBot};
use cg_arena::runner::BotSpec;
use cg_core::rng::Rng;
use ice_and_fire_engine::{Bits, Command, Outcome, State, HQ, SIZE};
use ice_and_fire_tactics::chain::{cost_to_moving, reach_within_moving, UNREACHABLE};

use crate::profiles::{prepare, read_index, Entry, Request as Base};
use crate::replay::{replay, Game};

/// One capability a test checks.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Capability {
    Finish,
    Save(u8),
    Defend(u8),
    Wall,
}

/// The capabilities in the report's order.
pub const ALL: [Capability; 6] = [
    Capability::Finish,
    Capability::Save(1),
    Capability::Save(2),
    Capability::Defend(1),
    Capability::Defend(2),
    Capability::Wall,
];

/// The most turns of income a save or defend test counts.
const MAX_TURNS: i64 = 2;

impl Capability {
    /// The capability's name in the suite and the report.
    pub fn name(self) -> String {
        match self {
            Capability::Finish => "finish".to_string(),
            Capability::Save(k) => format!("save-{k}"),
            Capability::Defend(k) => format!("defend-{k}"),
            Capability::Wall => "wall".to_string(),
        }
    }

    /// The capability named `name`.
    pub fn parse(name: &str) -> Option<Capability> {
        ALL.into_iter().find(|c| c.name() == name)
    }
}

/// The cost of `seat`'s cheapest chain to the enemy HQ, all its units
/// stepping first (they may all move at its next turn, or at this one when
/// it is to act at the start of its ply).
fn chain_cost(state: &State, seat: usize) -> i64 {
    cost_to_moving(state, seat, state.units(seat), HQ[1 - seat])
}

/// The fewest turns `t` in `from..=MAX_TURNS` such that `gold + t * income`
/// pays `cost`.
fn turns_to_pay(gold: i64, income: i64, cost: i64, from: i64) -> Option<i64> {
    if cost == UNREACHABLE {
        return None;
    }
    (from..=MAX_TURNS).find(|&t| gold + t * income >= cost)
}

/// `seat`'s active cells next to a cell of the other side.
fn frontier(state: &State, seat: usize) -> Bits {
    state.active(seat) & state.owned(1 - seat).neighbours()
}

/// Whether `commands` build a tower on or next to one of `seat`'s frontier
/// cells in `state`.
fn builds_wall(state: &State, seat: usize, commands: &[Command]) -> bool {
    let front = frontier(state, seat);
    commands.iter().any(|command| match *command {
        Command::BuildTower { x, y } if usize::from(x) < SIZE && usize::from(y) < SIZE => {
            let cell = usize::from(y) * SIZE + usize::from(x);
            !((Bits::cell(cell) | Bits::cell(cell).neighbours()) & front).is_empty()
        }
        _ => false,
    })
}

/// The capabilities that the position `state` tests, for its player to act
/// who answered `commands`: `reference` says whether that player is the
/// reference player, whose answers alone make wall tests.
pub fn classify(state: &State, commands: &[Command], reference: bool) -> Vec<Capability> {
    if state.outcome().is_some() {
        return Vec::new();
    }
    let p = state.to_act();
    let o = 1 - p;
    let mut out = Vec::new();
    let mine = turns_to_pay(state.gold(p), state.income(p), chain_cost(state, p), 0);
    // O's gold at its next turn is its gold plus one income.
    let theirs = turns_to_pay(state.gold(o), state.income(o), chain_cost(state, o), 1);
    match mine {
        Some(0) => out.push(Capability::Finish),
        Some(k) if theirs != Some(1) => out.push(Capability::Save(k as u8)),
        _ => {}
    }
    if mine != Some(0) {
        if let Some(k) = theirs {
            out.push(Capability::Defend(k as u8));
        }
    }
    if reference && mine != Some(0) && builds_wall(state, p, commands) {
        out.push(Capability::Wall);
    }
    out
}

/// The state after the player to act of `state` answered `commands`.
pub fn after(state: &State, commands: &[Command]) -> State {
    let mut next = state.clone();
    next.play_turn(commands);
    next
}

/// The active cells of `seat` that the other side can take with the gold of
/// its next turn, in `next`, the state after `seat`'s answer (the other
/// side to act, its income paid).
pub fn exposure(next: &State, seat: usize) -> u32 {
    let o = 1 - seat;
    let reach = reach_within_moving(next, o, next.units(o), next.gold(o));
    (reach & next.active(seat)).count()
}

/// Whether the answer that led from `state` to `next` passes `capability`;
/// `reference` is the state after the reference player's answer, for the
/// wall.
pub fn passes(capability: Capability, state: &State, next: &State, reference: &State) -> bool {
    let p = state.to_act();
    let o = 1 - p;
    if next.outcome() == Some(Outcome::Win(p)) {
        return true;
    }
    match capability {
        Capability::Finish => false,
        Capability::Save(k) => {
            let cost = chain_cost(next, p);
            cost != UNREACHABLE && next.gold(p) + i64::from(k) * next.income(p) >= cost
        }
        Capability::Defend(k) => {
            let cost = chain_cost(next, o);
            cost == UNREACHABLE || next.gold(o) + (i64::from(k) - 1) * next.income(o) < cost
        }
        Capability::Wall => exposure(next, p) <= exposure(reference, p),
    }
}

/// One test of the suite.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Test {
    pub capability: Capability,
    pub game_id: u64,
    /// The index of the ply in the replayed game, from 0.
    pub ply: usize,
    /// The game turn of the ply, for readers.
    pub turn: u32,
    /// The player who answered in the game.
    pub mover: String,
}

/// The suite's first line.
const HEADER: &str = "capability\tgame_id\tply\tturn\tmover";

/// The suite's text: a comment, the header, then one test a line.
pub fn suite_text(comment: &str, tests: &[Test]) -> String {
    let mut out = String::new();
    for line in comment.lines() {
        let _ = writeln!(out, "# {line}");
    }
    let _ = writeln!(out, "{HEADER}");
    for t in tests {
        let _ = writeln!(
            out,
            "{}\t{}\t{}\t{}\t{}",
            t.capability.name(),
            t.game_id,
            t.ply,
            t.turn,
            t.mover
        );
    }
    out
}

/// The tests of a suite's text.
pub fn read_suite(text: &str) -> Result<Vec<Test>, String> {
    let mut tests = Vec::new();
    for (number, line) in text.lines().enumerate() {
        if line.starts_with('#') || line.trim().is_empty() || line == HEADER {
            continue;
        }
        let bad = |what: &str| format!("suite line {}: {what}: {line:?}", number + 1);
        let fields: Vec<&str> = line.split('\t').collect();
        let [capability, game_id, ply, turn, mover] = fields[..] else {
            return Err(bad("not 5 fields separated by tabs"));
        };
        tests.push(Test {
            capability: Capability::parse(capability).ok_or_else(|| bad("unknown capability"))?,
            game_id: game_id.parse().map_err(|_| bad("bad game id"))?,
            ply: ply.parse().map_err(|_| bad("bad ply"))?,
            turn: turn.parse().map_err(|_| bad("bad turn"))?,
            mover: mover.to_string(),
        });
    }
    Ok(tests)
}

/// The tests of one game: every ply and the capabilities it tests.
fn candidates(game: &Game, game_id: u64, reference: &str) -> Vec<Test> {
    let mut out = Vec::new();
    for (ply, step) in game.steps.iter().enumerate() {
        let mover = &game.record.players[step.state.to_act()].name;
        for capability in classify(&step.state, &step.commands, mover == reference) {
            out.push(Test {
                capability,
                game_id,
                ply,
                turn: step.state.game_turn(),
                mover: mover.clone(),
            });
        }
    }
    out
}

/// Draws, for each capability, up to `per` of `candidates` at random with
/// `seed`, at most `per_game` from one game, and returns them sorted.
pub fn draw(mut candidates: Vec<Test>, per: usize, per_game: usize, seed: u64) -> Vec<Test> {
    candidates.sort();
    let mut rng = Rng::new(seed);
    rng.shuffle(&mut candidates);
    let mut taken: BTreeMap<Capability, usize> = BTreeMap::new();
    let mut by_game: BTreeMap<(Capability, u64), usize> = BTreeMap::new();
    let mut out = Vec::new();
    for test in candidates {
        let n = taken.entry(test.capability).or_default();
        let g = by_game.entry((test.capability, test.game_id)).or_default();
        if *n < per && *g < per_game {
            *n += 1;
            *g += 1;
            out.push(test);
        }
    }
    out.sort();
    out
}

/// The games of the index that pass `prepare`, replayed, on `jobs` threads,
/// with the number of games left out.
fn load_games(base: &Base, entries: &[Entry]) -> (Vec<(u64, Game)>, usize) {
    let jobs = base.jobs.clamp(1, 64);
    let size = entries.len().div_ceil(jobs).max(1);
    let mut slots: Vec<Option<(u64, Game)>> = Vec::new();
    slots.resize_with(entries.len(), || None);
    std::thread::scope(|scope| {
        for (chunk, out) in entries.chunks(size).zip(slots.chunks_mut(size)) {
            scope.spawn(move || {
                for (entry, slot) in chunk.iter().zip(out.iter_mut()) {
                    if let Ok((meta, record)) = prepare(&base.data, entry, &base.me) {
                        let id = meta.game_id;
                        *slot = Some((id, replay(&id.to_string(), record)));
                    }
                }
            });
        }
    });
    let left_out = slots.iter().filter(|s| s.is_none()).count();
    (slots.into_iter().flatten().collect(), left_out)
}

/// What `codingame-capability-suite` is asked.
pub struct SuiteRequest {
    pub base: Base,
    /// The reference player, whose answers alone make wall tests.
    pub reference: String,
    pub per: usize,
    pub per_game: usize,
    pub seed: u64,
    /// Only the positions this player answered, if set.
    pub mover: Option<String>,
}

/// The suite drawn from the data directory, as text, and a summary of the
/// candidates by capability for stderr.
pub fn run_suite(request: &SuiteRequest) -> Result<(String, String), String> {
    let base = &request.base;
    let (entries, _) = read_index(&base.data.join("index.jsonl"), base)?;
    let (games, left_out) = load_games(base, &entries);
    let mut all = Vec::new();
    for (id, game) in &games {
        all.extend(
            candidates(game, *id, &request.reference)
                .into_iter()
                .filter(|t| request.mover.as_ref().is_none_or(|m| *m == t.mover)),
        );
    }
    let mut counts: BTreeMap<Capability, usize> = BTreeMap::new();
    for t in &all {
        *counts.entry(t.capability).or_default() += 1;
    }
    let tests = draw(all, request.per, request.per_game, request.seed);
    let mut summary = format!(
        "{} games replayed, {left_out} left out; candidates by capability:",
        games.len()
    );
    for c in ALL {
        let _ = write!(
            summary,
            " {} {}",
            c.name(),
            counts.get(&c).copied().unwrap_or(0)
        );
    }
    let comment = format!(
        "Capability tests of A Code of Ice and Fire (games/ice-and-fire/capabilities/README.md).\n\
         Drawn by `ice-and-fire-review codingame-capability-suite` from {} games of the data branch:\n\
         up to {} tests a capability, at most {} a game, seed {}, wall reference {}{}.",
        games.len(),
        request.per,
        request.per_game,
        request.seed,
        request.reference,
        request
            .mover
            .as_ref()
            .map_or(String::new(), |m| format!(", only positions answered by {m}"))
    );
    Ok((suite_text(&comment, &tests), summary))
}

/// What `codingame-capabilities` is asked.
pub struct Request {
    pub base: Base,
    pub suite: PathBuf,
    /// The bots, `NAME=COMMAND`.
    pub bots: Vec<String>,
    /// The bots' `CG_FIXED_ITERS`, or none (their own clock) when 0.
    pub tries: u64,
    /// The reference player, as the suite was drawn.
    pub reference: String,
    /// Also write one row a test and bot as CSV here.
    pub csv: Option<PathBuf>,
}

/// A bot's answers in the tests of one game and seat, in ply order, or why
/// it gave none.
fn ask_bot(
    spec: &BotSpec,
    settings: &BotSettings,
    game: &Game,
    seat: usize,
    plies: &[usize],
) -> Result<Vec<Vec<Command>>, String> {
    let Some(&last) = plies.iter().max() else {
        return Ok(Vec::new());
    };
    let mut bot = LiveBot::spawn(spec, settings).map_err(|e| format!("cannot start: {e}"))?;
    let mut answers = Vec::new();
    let mut first = true;
    for (ply, step) in game.steps.iter().enumerate().take(last + 1) {
        if step.state.to_act() != seat {
            continue;
        }
        let mut input = String::new();
        if first {
            input.push_str(&step.state.initial_input(seat));
        }
        input.push_str(&step.state.turn_input(seat));
        let limit = Duration::from_secs(if first { 60 } else { 20 });
        first = false;
        let (lines, _) = bot
            .ask(&input, 1, limit)
            .map_err(|e| format!("ply {ply}: {e}"))?;
        if plies.contains(&ply) {
            let line = lines.first().map(String::as_str).unwrap_or("");
            answers.push(
                line.split(';')
                    .filter_map(|part| crate::trace::parse(part.trim()))
                    .collect(),
            );
        }
    }
    Ok(answers)
}

/// A test's index in the suite and the bot's answer to it, or why there is
/// none.
type Answered = (usize, Result<Vec<Command>, String>);

/// A bot's answers to the tests of each group (the indices of the tests of
/// one game and seat), the groups shared out over `jobs` threads.
fn answer_groups(
    spec: &BotSpec,
    settings: &BotSettings,
    games: &BTreeMap<u64, Game>,
    tests: &[Test],
    groups: &[((u64, usize), Vec<usize>)],
    jobs: usize,
) -> Vec<Answered> {
    let size = groups.len().div_ceil(jobs).max(1);
    std::thread::scope(|scope| {
        let handles: Vec<_> = groups
            .chunks(size)
            .map(|chunk| {
                scope.spawn(move || {
                    let mut out = Vec::new();
                    for ((game_id, seat), indices) in chunk {
                        let mut plies: Vec<usize> = indices.iter().map(|&i| tests[i].ply).collect();
                        plies.sort_unstable();
                        plies.dedup();
                        match ask_bot(spec, settings, &games[game_id], *seat, &plies) {
                            Ok(answers) => {
                                let by_ply: BTreeMap<usize, Vec<Command>> =
                                    plies.into_iter().zip(answers).collect();
                                for &i in indices {
                                    out.push((
                                        i,
                                        by_ply
                                            .get(&tests[i].ply)
                                            .cloned()
                                            .ok_or_else(|| "no answer".to_string()),
                                    ));
                                }
                            }
                            Err(e) => out.extend(indices.iter().map(|&i| (i, Err(e.clone())))),
                        }
                    }
                    out
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| {
                h.join()
                    .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
            })
            .collect()
    })
}

/// The outcome of one test for one answerer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Verdict {
    Pass,
    Fail,
    /// The bot did not answer.
    Error,
}

/// The report of `codingame-capabilities`, as Markdown.
pub fn run(request: &Request) -> Result<String, String> {
    let text = fs::read_to_string(&request.suite)
        .map_err(|e| format!("{}: {e}", request.suite.display()))?;
    let tests = read_suite(&text)?;
    let specs: Vec<BotSpec> = request
        .bots
        .iter()
        .map(|b| BotSpec::parse(b))
        .collect::<Result<_, _>>()?;
    let base = &request.base;
    let (entries, _) = read_index(&base.data.join("index.jsonl"), base)?;
    let wanted: std::collections::BTreeSet<u64> = tests.iter().map(|t| t.game_id).collect();
    let entries: Vec<Entry> = entries
        .into_iter()
        .filter(|e| wanted.contains(&e.meta.game_id))
        .collect();
    let (games, _) = load_games(base, &entries);
    let games: BTreeMap<u64, Game> = games.into_iter().collect();

    // Check that each test still stands, and find its reference answer.
    for t in &tests {
        let game = games
            .get(&t.game_id)
            .ok_or_else(|| format!("game {} of the suite is not in the data", t.game_id))?;
        let step = game
            .steps
            .get(t.ply)
            .ok_or_else(|| format!("game {} has no ply {}", t.game_id, t.ply))?;
        let mover = &game.record.players[step.state.to_act()].name;
        let tested = classify(&step.state, &step.commands, *mover == request.reference);
        if *mover != t.mover || !tested.contains(&t.capability) {
            return Err(format!(
                "game {} ply {}: the data no longer gives a {} test by {}",
                t.game_id,
                t.ply,
                t.capability.name(),
                t.mover
            ));
        }
    }

    // The players' own answers.
    let mover_verdicts: Vec<Verdict> = tests
        .iter()
        .map(|t| {
            let step = &games[&t.game_id].steps[t.ply];
            let next = after(&step.state, &step.commands);
            verdict(passes(t.capability, &step.state, &next, &next))
        })
        .collect();

    // Each bot, one process per game and seat, the groups shared out over
    // the jobs.
    let mut groups: BTreeMap<(u64, usize), Vec<usize>> = BTreeMap::new();
    for (i, t) in tests.iter().enumerate() {
        let seat = games[&t.game_id].steps[t.ply].state.to_act();
        groups.entry((t.game_id, seat)).or_default().push(i);
    }
    let groups: Vec<((u64, usize), Vec<usize>)> = groups.into_iter().collect();
    let settings = BotSettings {
        seed: 1,
        time_scale: 1.0,
        fixed_iters: (request.tries > 0).then_some(request.tries),
        show_stderr: false,
    };
    let mut bot_verdicts: Vec<Vec<Verdict>> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    for spec in &specs {
        let mut verdicts = vec![Verdict::Error; tests.len()];
        let jobs = base.jobs.clamp(1, 64);
        let results = answer_groups(spec, &settings, &games, &tests, &groups, jobs);
        for (i, answer) in results {
            let t = &tests[i];
            let step = &games[&t.game_id].steps[t.ply];
            verdicts[i] = match answer {
                Ok(commands) => {
                    let next = after(&step.state, &commands);
                    let reference = after(&step.state, &step.commands);
                    verdict(passes(t.capability, &step.state, &next, &reference))
                }
                Err(e) => {
                    errors.push(format!(
                        "{}: game {} ply {}: {e}",
                        spec.name, t.game_id, t.ply
                    ));
                    Verdict::Error
                }
            };
        }
        bot_verdicts.push(verdicts);
    }
    errors.sort();
    errors.dedup();

    if let Some(path) = &request.csv {
        fs::write(path, csv(&tests, &specs, &mover_verdicts, &bot_verdicts))
            .map_err(|e| format!("{}: {e}", path.display()))?;
    }
    Ok(report(
        request,
        &tests,
        &specs,
        &mover_verdicts,
        &bot_verdicts,
        &errors,
    ))
}

fn verdict(pass: bool) -> Verdict {
    if pass {
        Verdict::Pass
    } else {
        Verdict::Fail
    }
}

/// One row a test: its fields, the mover's verdict and each bot's (1 pass,
/// 0 fail, empty when the bot did not answer).
fn csv(tests: &[Test], specs: &[BotSpec], movers: &[Verdict], bots: &[Vec<Verdict>]) -> String {
    let cell = |v: Verdict| match v {
        Verdict::Pass => "1",
        Verdict::Fail => "0",
        Verdict::Error => "",
    };
    let mut out = String::from("capability,game_id,ply,turn,mover,mover_pass");
    for spec in specs {
        let _ = write!(out, ",{}", spec.name);
    }
    out.push('\n');
    for (i, t) in tests.iter().enumerate() {
        let _ = write!(
            out,
            "{},{},{},{},{},{}",
            t.capability.name(),
            t.game_id,
            t.ply,
            t.turn,
            t.mover,
            cell(movers[i])
        );
        for verdicts in bots {
            let _ = write!(out, ",{}", cell(verdicts[i]));
        }
        out.push('\n');
    }
    out
}

/// `pass` of `all` as "P% (pass/all)", or "-" when `all` is 0.
fn rate(pass: usize, all: usize) -> String {
    if all == 0 {
        "-".to_string()
    } else {
        format!("{:.0}% ({pass}/{all})", 100.0 * pass as f64 / all as f64)
    }
}

/// The passes and the tests among `indices` of `verdicts`.
fn tally(verdicts: &[Verdict], indices: &[usize]) -> (usize, usize) {
    let pass = indices
        .iter()
        .filter(|&&i| verdicts[i] == Verdict::Pass)
        .count();
    (pass, indices.len())
}

fn report(
    request: &Request,
    tests: &[Test],
    specs: &[BotSpec],
    movers: &[Verdict],
    bots: &[Vec<Verdict>],
    errors: &[String],
) -> String {
    let me = &request.base.me;
    let reference = &request.reference;
    let mut out = String::from("# Capability tests\n\n");
    let _ = writeln!(
        out,
        "Suite `{}`: {} tests from {} games. Bots answered {}; each in the mover's seat, fed the game's inputs from its first turn. \"Played\" is the answers the players sent in the game: all of them, then those of {reference}, of {me} and of the others.\n",
        request.suite.display(),
        tests.len(),
        tests
            .iter()
            .map(|t| t.game_id)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        if request.tries > 0 {
            format!("with `CG_FIXED_ITERS={}`", request.tries)
        } else {
            "on their own clock".to_string()
        }
    );
    let _ = write!(out, "| Capability | Tests |");
    for spec in specs {
        let _ = write!(out, " {} |", spec.name);
    }
    let _ = writeln!(
        out,
        " Played, all | Played by {reference} | Played by {me} | Played by others |"
    );
    let _ = write!(out, "| --- | ---: |");
    for _ in specs {
        out.push_str(" ---: |");
    }
    out.push_str(" ---: | ---: | ---: | ---: |\n");
    for c in ALL {
        let indices: Vec<usize> = (0..tests.len())
            .filter(|&i| tests[i].capability == c)
            .collect();
        if indices.is_empty() {
            continue;
        }
        let _ = write!(out, "| {} | {} |", c.name(), indices.len());
        for verdicts in bots {
            let (pass, all) = tally(verdicts, &indices);
            let _ = write!(out, " {} |", rate(pass, all));
        }
        let by = |keep: &dyn Fn(&str) -> bool| -> String {
            let chosen: Vec<usize> = indices
                .iter()
                .copied()
                .filter(|&i| keep(&tests[i].mover))
                .collect();
            let (pass, all) = tally(movers, &chosen);
            rate(pass, all)
        };
        let _ = writeln!(
            out,
            " {} | {} | {} | {} |",
            by(&|_| true),
            by(&|m| m == reference),
            by(&|m| m == me),
            by(&|m| m != reference && m != me)
        );
    }
    let _ = writeln!(
        out,
        "\nA wall test compares the answer with {reference}'s own, which passes by construction."
    );
    if !errors.is_empty() {
        let _ = writeln!(
            out,
            "\n## Bots that did not answer\n\nThose tests count as failed.\n"
        );
        for e in errors.iter().take(20) {
            let _ = writeln!(out, "- {e}");
        }
        if errors.len() > 20 {
            let _ = writeln!(out, "- and {} more", errors.len() - 20);
        }
    }
    out
}

#[cfg(test)]
mod tests;
