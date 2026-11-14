//! `codingame-surprise`: where our bot was overconfident in its CodinGame
//! games: the score its search expected for the turn it played, against what
//! the opponent's real reply did.
//!
//! Every ply of ours in the games of a data directory (read as
//! `codingame-report` reads them) is written as a bank block with our
//! answer (`played`) and the opponent's next answer (`reply`), the blocks
//! are split over `--jobs` runs of the plan bot's analysis build
//! (`surprise --positions FILE --tries N`), and its lines are joined with
//! what the reply did on the board, which this command measures with the
//! engine. Plies of the opening, which the bot's search does not play, and
//! plies its search answers with a lethal plan, are left out by the bot.
//!
//! The **surprise** of a ply is `actual − expected`, in the evaluation's
//! units (gold 1, income 3, an active cell 4, a unit 10 a level, a tower
//! 10; `ice_and_fire_tactics::eval`): below zero, the opponent found a
//! reply worse for us than every reply the search modeled.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as Process;

use ice_and_fire_engine::{answer_line, Bits, Command, Outcome, State};

use crate::bank::block_reply;
use crate::profiles::{newest_snapshot, prepare, read_index, read_ranks, Request as Base};
use crate::replay::{replay, Game};
use crate::trace::hq_cost;
use ice_and_fire_opening::econ::distances;
use ice_and_fire_opening::middle;
use ice_and_fire_tactics::cut::cuts;
use ice_and_fire_tactics::eval::WIN;

/// What `codingame-surprise` is asked.
pub struct Request {
    pub data: PathBuf,
    pub me: String,
    /// The plan bot's analysis build.
    pub judge: PathBuf,
    /// Only our games played by this release (the index's `release`).
    pub release: Option<String>,
    pub top: usize,
    pub tries: u64,
    /// At most this many plies, spread over the whole list.
    pub limit: Option<usize>,
    pub jobs: usize,
    pub leaderboard: Option<PathBuf>,
    pub out: PathBuf,
}

/// A surprise at or below this is large: about three level-1 units, twelve
/// active cells, or 50 gold.
pub const LARGE: i64 = -50;
/// Surprises are capped here in means and quantiles.
pub const CAP: i64 = 1000;
/// A value beyond this is the evaluation's HQ term, not material: the
/// evaluation counts `WIN / 2` against the side to act when the opponent
/// can pay a chain of trainings to its HQ, and `WIN` for a game won.
pub const THREAT: i64 = WIN / 4;
/// The cut finder as the search runs it for replies: chains of up to 5
/// trainings, the 3 best (`CUT_LENGTH`, `REPLY_CUTS` in the plan bot).
pub const NARROW: (usize, usize) = (5, 3);
/// The cut finder wider: up to 8 trainings, the 20 best.
pub const WIDE: (usize, usize) = (8, 20);

/// What kind of ply a row is, by its expected and actual values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    /// Both values are material: the surprise is measured.
    Normal,
    /// The search expected no chain to our HQ, and the reply left one the
    /// opponent can pay (or took the HQ).
    ThreatMissed,
    /// The search expected the threat on our HQ, and it came.
    ThreatSeen,
    /// The search expected a threat on our HQ that did not come.
    ThreatAvoided,
    /// The values are beyond material for another reason (a threat of ours
    /// on the enemy HQ).
    Other,
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Kind::Normal => "normal: both values material",
            Kind::ThreatMissed => "HQ threat not foreseen",
            Kind::ThreatSeen => "HQ threat foreseen, and it came",
            Kind::ThreatAvoided => "HQ threat foreseen, it did not come",
            Kind::Other => "other (our threat on the enemy HQ)",
        }
    }
}

/// What the bot printed for one ply.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Judged {
    pub name: String,
    pub ply: usize,
    pub expected: i64,
    pub actual: i64,
    pub pass: i64,
    pub modeled: usize,
    pub modeled_best: i64,
    pub same_reply: bool,
}

/// Parses one line of the bot's `surprise`.
pub fn parse_line(line: &str) -> Result<Judged, String> {
    let parts: Vec<&str> = line.split('\t').collect();
    let bad = || format!("unreadable line: {line}");
    if parts.len() != 8 {
        return Err(bad());
    }
    let field = |i: usize, key: &str| -> Result<&str, String> {
        parts[i]
            .strip_prefix(key)
            .and_then(|r| r.strip_prefix('='))
            .ok_or_else(bad)
    };
    let num =
        |i: usize, key: &str| -> Result<i64, String> { field(i, key)?.parse().map_err(|_| bad()) };
    Ok(Judged {
        name: parts[0].to_string(),
        ply: parts[1].parse().map_err(|_| bad())?,
        expected: num(2, "expected")?,
        actual: num(3, "actual")?,
        pass: num(4, "pass")?,
        modeled: field(5, "modeled")?.parse().map_err(|_| bad())?,
        modeled_best: num(6, "modeled_best")?,
        same_reply: field(7, "same_reply")? == "yes",
    })
}

/// What the opponent's reply did, measured on the board.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Effect {
    /// Our active cells at the end of our turn that the reply took.
    pub taken: i64,
    /// Our active cells at the end of our turn that the reply cut off.
    pub cut: i64,
    /// `taken` plus `cut` when the reply is played without its moves (its
    /// trainings and buildings alone, in order): how much of the damage the
    /// trainings did without the units' steps.
    pub without_moves: i64,
    /// The most cells (taken or cut off) of a cut the cut finder finds from
    /// the reply's start with the opponent's gold: as the search models
    /// replies ([`NARROW`]), and wider ([`WIDE`]).
    pub finder: [i64; 2],
    /// The training cost (10 a level) of our units it killed or cut off.
    pub units_lost: i64,
    /// The reply's trainings by level.
    pub trainings: [i64; 3],
    pub moves: i64,
    pub towers: i64,
    pub mines: i64,
    /// Gold the reply spent.
    pub spent: i64,
    /// The reply took our HQ.
    pub hq: bool,
    /// After the reply, at our start of turn: the opponent's gold less the
    /// cost of its cheapest chain of trainings to our HQ (above zero: it can
    /// pay it at its next turn, before its income).
    pub hq_margin: Option<i64>,
    /// At our next start of turn, ours then the opponent's: gold, income,
    /// active cells, and the training cost of units on active cells.
    pub material: [[i64; 4]; 2],
}

fn unit_value(state: &State, cells: Bits) -> i64 {
    cells
        .iter()
        .filter_map(|c| state.unit_level(c))
        .map(|l| 10 * i64::from(l))
        .sum()
}

/// What `reply`, played from `start` (the opponent to act, after our turn),
/// did to `me`.
pub fn effect(start: &State, reply: &[Command], me: usize) -> Effect {
    let them = 1 - me;
    let mut state = start.clone();
    let active = start.active(me);
    let units = start.units(me) & active;
    let gold = start.gold(them);
    let mut e = Effect::default();
    for &command in reply {
        if state.outcome().is_some() {
            break;
        }
        if !state.apply(command) {
            continue;
        }
        match command {
            Command::Train { level, .. } => {
                e.trainings[usize::from(level.clamp(1, 3)) - 1] += 1;
            }
            Command::Move { .. } => e.moves += 1,
            Command::BuildTower { .. } => e.towers += 1,
            Command::BuildMine { .. } => e.mines += 1,
            Command::Wait => {}
        }
    }
    e.spent = gold - state.gold(them);
    e.hq = state.outcome() == Some(Outcome::Win(them));
    let taken = active & state.owned(them);
    e.taken = i64::from(taken.count());
    e.cut = i64::from(
        active
            .and_not(state.owned(them))
            .and_not(state.active(me))
            .count(),
    );
    let alive = state.units(me) & state.active(me);
    e.units_lost = unit_value(start, units.and_not(alive));
    let mut alone = start.clone();
    for &command in reply {
        if alone.outcome().is_some() {
            break;
        }
        if !matches!(command, Command::Move { .. }) {
            alone.apply(command);
        }
    }
    e.without_moves = i64::from(active.and_not(alone.active(me)).count());
    for (slot, (length, limit)) in [NARROW, WIDE].into_iter().enumerate() {
        e.finder[slot] = cuts(start, length, gold, limit)
            .iter()
            .map(|c| c.cells)
            .max()
            .unwrap_or(0);
    }
    if !e.hq {
        let mut next = state.clone();
        next.end_turn();
        if next.outcome().is_none() {
            e.hq_margin = hq_cost(&next, them).map(|cost| next.gold(them) - cost);
        }
        for (i, seat) in [me, them].into_iter().enumerate() {
            let active = next.active(seat);
            e.material[i] = [
                next.gold(seat),
                next.income(seat),
                i64::from(active.count()),
                unit_value(&next, next.units(seat) & active),
            ];
        }
    }
    e
}

/// The shape of the threat a reply carried out, for the study of the
/// replies the search missed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Shape {
    /// Of our active cells the reply removed (taken or cut off), those we
    /// did not own at the start of our turn: land just taken.
    pub new_lost: i64,
    /// Our active cells, at the end of our turn, whose capture alone cuts
    /// others off from our HQ (joints), and the most cells one cuts off.
    pub joints: i64,
    pub joint_max: i64,
    /// The most cells cut off by one capture among the cells the reply took
    /// (0 when it took no joint).
    pub joint_hit: i64,
    /// The most steps from the enemy's land, before the reply, to a cell it
    /// removed: how deep it reached.
    pub depth: i64,
    /// The fewest steps from our HQ to a cell it removed.
    pub hq_steps: i64,
    /// Gold at the reply's start: ours (left after our turn) and the
    /// opponent's.
    pub our_gold: i64,
    pub opp_gold: i64,
    /// Our towers on active cells at the end of our turn, and those on the
    /// cells the reply removed.
    pub towers: i64,
    pub towers_lost: i64,
    /// The reply's moves that stepped onto a cell of ours.
    pub move_captures: i64,
}

/// The shape of `reply`, played from `start` (the opponent to act after our
/// turn), against `me`, whose turn started at `before`.
pub fn shape(before: &State, start: &State, reply: &[Command], me: usize) -> Shape {
    let them = 1 - me;
    let mut state = start.clone();
    let mut out = Shape {
        our_gold: start.gold(me),
        opp_gold: start.gold(them),
        ..Shape::default()
    };
    for &command in reply {
        if state.outcome().is_some() {
            break;
        }
        // A move goes one step toward its target: it is counted by the
        // cell the unit lands on.
        let units = state.units(them);
        let accepted = state.apply(command);
        if accepted && matches!(command, Command::Move { .. }) {
            let landed = state.units(them).and_not(units);
            out.move_captures += i64::from((landed & start.owned(me)).count());
        }
    }
    let active = start.active(me);
    let removed = active.and_not(state.active(me));
    out.new_lost = i64::from(removed.and_not(before.owned(me)).count());
    let severed = ice_and_fire_opening::solo::severed(start, me);
    for cell in active.iter() {
        if severed[cell] > 0 {
            out.joints += 1;
            out.joint_max = out.joint_max.max(severed[cell]);
        }
    }
    for cell in (removed & state.owned(them)).iter() {
        out.joint_hit = out.joint_hit.max(severed[cell]);
    }
    let from_enemy = distances(start, start.owned(them));
    let from_hq = distances(start, Bits::cell(ice_and_fire_engine::HQ[me]));
    out.hq_steps = removed
        .iter()
        .map(|c| i64::from(from_hq[c]))
        .min()
        .unwrap_or(-1);
    out.depth = removed
        .iter()
        .map(|c| i64::from(from_enemy[c]))
        .max()
        .unwrap_or(-1);
    let towers = start.towers() & active;
    out.towers = i64::from(towers.count());
    out.towers_lost = i64::from((towers & removed).count());
    out
}

/// One of our plies, with what the bot said and what the reply did.
#[derive(Clone, Debug)]
pub struct Row {
    pub game_id: u64,
    pub ply: usize,
    pub turn: u32,
    pub seat: usize,
    pub opponent: String,
    /// The opponent is in the top N of the snapshot.
    pub top: bool,
    /// We won the game.
    pub won: bool,
    /// Our turns left in the game after this one.
    pub turns_left: u32,
    pub judged: Judged,
    pub effect: Effect,
    pub shape: Shape,
    pub reply: String,
}

/// Why the search's modeled replies did not hold a reply's damage, as far
/// as the board shows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cause {
    /// The reply removed none of our cells.
    NoCell,
    /// Without its moves, the reply removes less than half the cells.
    Moves,
    /// Its trainings alone remove them, and the cut finder as the search
    /// runs it finds a cut of at least as many cells.
    Found,
    /// Only the wider cut finder ([`WIDE`]) finds as many.
    Wider,
    /// Neither finds as many.
    Neither,
}

impl Cause {
    pub fn name(self) -> &'static str {
        match self {
            Cause::NoCell => "no cell of ours removed",
            Cause::Moves => "moves needed (without them, under half the cells)",
            Cause::Found => "trainings alone; the search's cut finder finds as many cells",
            Cause::Wider => "trainings alone; only the wider cut finder finds as many",
            Cause::Neither => "trainings alone; neither finder finds as many",
        }
    }
}

impl Row {
    pub fn kind(&self) -> Kind {
        let (e, a) = (self.judged.expected, self.judged.actual);
        match (e <= -THREAT, a <= -THREAT) {
            (false, true) => Kind::ThreatMissed,
            (true, true) => Kind::ThreatSeen,
            (true, false) => Kind::ThreatAvoided,
            (false, false) if e.abs() < THREAT && a.abs() < THREAT => Kind::Normal,
            (false, false) => Kind::Other,
        }
    }

    pub fn cause(&self) -> Cause {
        let e = &self.effect;
        let removed = e.taken + e.cut;
        if removed == 0 {
            Cause::NoCell
        } else if 2 * e.without_moves < removed {
            Cause::Moves
        } else if e.finder[0] >= removed {
            Cause::Found
        } else if e.finder[1] >= removed {
            Cause::Wider
        } else {
            Cause::Neither
        }
    }

    pub fn surprise(&self) -> i64 {
        (self.judged.actual - self.judged.expected).clamp(-CAP, CAP)
    }
}

/// Runs the bot on one chunk of blocks.
fn run_chunk(judge: &Path, file: &Path, tries: u64) -> Result<(Vec<Judged>, usize), String> {
    let result = Process::new(judge)
        .arg("surprise")
        .arg("--positions")
        .arg(file)
        .arg("--tries")
        .arg(tries.to_string())
        .output()
        .map_err(|e| format!("{}: {e}", judge.display()))?;
    if !result.status.success() {
        return Err(format!(
            "the bot failed on {}: {}: {}",
            file.display(),
            result.status,
            String::from_utf8_lossy(&result.stderr)
                .lines()
                .last()
                .unwrap_or("")
        ));
    }
    let left = String::from_utf8_lossy(&result.stderr)
        .lines()
        .filter(|l| l.starts_with("left out"))
        .count();
    let mut out = Vec::new();
    for line in String::from_utf8_lossy(&result.stdout).lines() {
        if !line.trim().is_empty() {
            out.push(parse_line(line)?);
        }
    }
    Ok((out, left))
}

/// A ply waiting for the bot.
struct Pending {
    game: usize,
    ply: usize,
    block: String,
}

/// Runs the study and writes `rows.csv` and `README.md` (the report) to
/// `out`.
pub fn run(request: &Request) -> Result<String, String> {
    let base = Base {
        data: request.data.clone(),
        me: request.me.clone(),
        kinds: vec![],
        batch: None,
        release: request.release.clone(),
        our_seat: None,
        top: request.top,
        players: vec![],
        leaderboard: request.leaderboard.clone(),
        jobs: request.jobs,
    };
    let (entries, left_index) = read_index(&request.data.join("index.jsonl"), &base)?;
    let ranks = match request
        .leaderboard
        .clone()
        .or_else(|| newest_snapshot(&request.data))
    {
        Some(path) => read_ranks(&path)?,
        None => BTreeMap::new(),
    };
    let mut games: Vec<(u64, usize, Game)> = Vec::new();
    let mut unusable = left_index.len();
    for entry in &entries {
        match prepare(&request.data, entry, &request.me) {
            Ok((meta, record)) => match meta.our_seat {
                Some(seat) => games.push((
                    meta.game_id,
                    seat,
                    replay(&meta.game_id.to_string(), record),
                )),
                None => unusable += 1,
            },
            Err(_) => unusable += 1,
        }
    }
    let mut pending = Vec::new();
    for (index, (_, seat, game)) in games.iter().enumerate() {
        // From the start of the middle game: the bot plays the opening by
        // rules, not by its search.
        let Some(from) = game.steps.iter().position(|s| middle::reached(&s.state)) else {
            continue;
        };
        for ply in from..game.steps.len() {
            if game.steps[ply].state.to_act() != *seat {
                continue;
            }
            if let Some(block) = block_reply(game, ply) {
                pending.push(Pending {
                    game: index,
                    ply,
                    block,
                });
            }
        }
    }
    if let Some(limit) = request.limit.filter(|&l| l < pending.len()) {
        let step = pending.len() as f64 / limit as f64;
        pending = (0..limit)
            .filter_map(|i| {
                pending.get((i as f64 * step) as usize).map(|p| Pending {
                    game: p.game,
                    ply: p.ply,
                    block: p.block.clone(),
                })
            })
            .collect();
    }
    eprintln!(
        "judging {} plies of ours in {} games ({unusable} games unusable)",
        pending.len(),
        games.len()
    );
    let jobs = request.jobs.clamp(1, 64).min(pending.len().max(1));
    let dir = std::env::temp_dir().join(format!("ice-and-fire-surprise-{}", std::process::id()));
    fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut files = Vec::new();
    for job in 0..jobs {
        let text: String = pending
            .iter()
            .skip(job)
            .step_by(jobs)
            .map(|p| p.block.as_str())
            .collect();
        let file = dir.join(format!("chunk-{job}.bank"));
        fs::write(&file, text).map_err(|e| format!("{}: {e}", file.display()))?;
        files.push(file);
    }
    let results: Vec<Result<(Vec<Judged>, usize), String>> = std::thread::scope(|scope| {
        let handles: Vec<_> = files
            .iter()
            .map(|file| scope.spawn(move || run_chunk(&request.judge, file, request.tries)))
            .collect();
        handles
            .into_iter()
            .map(|h| {
                h.join()
                    .unwrap_or_else(|_| Err("a thread panicked".to_string()))
            })
            .collect()
    });
    let _ = fs::remove_dir_all(&dir);
    let mut by_key: HashMap<(String, usize), Judged> = HashMap::new();
    let mut left_by_bot = 0;
    for result in results {
        let (judged, left) = result?;
        left_by_bot += left;
        for j in judged {
            by_key.insert((j.name.clone(), j.ply), j);
        }
    }
    let mut rows = Vec::new();
    for p in &pending {
        let (game_id, seat, game) = &games[p.game];
        let Some(judged) = by_key.get(&(game.name.clone(), p.ply)) else {
            continue;
        };
        let start = &game.steps[p.ply + 1].state;
        let reply = &game.steps[p.ply + 1].commands;
        let opponent = game.record.players[1 - seat].name.clone();
        let top = ranks
            .get(&opponent)
            .is_some_and(|&r| r as usize <= request.top);
        let last_turn = game.steps.last().map_or(0, |s| s.state.game_turn());
        let turn = game.steps[p.ply].state.game_turn();
        rows.push(Row {
            game_id: *game_id,
            ply: p.ply,
            turn,
            seat: *seat,
            opponent,
            top,
            won: game.end.outcome() == Some(Outcome::Win(*seat)),
            turns_left: last_turn.saturating_sub(turn),
            judged: judged.clone(),
            effect: effect(start, reply, *seat),
            shape: shape(&game.steps[p.ply].state, start, reply, *seat),
            reply: answer_line(reply),
        });
    }
    fs::create_dir_all(&request.out).map_err(|e| format!("{}: {e}", request.out.display()))?;
    let csv = rows_csv(&rows);
    let report = report(&rows, request, games.len(), unusable, left_by_bot);
    for (name, text) in [("rows.csv", &csv), ("README.md", &report)] {
        let path = request.out.join(name);
        fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
    }
    Ok(format!(
        "wrote rows.csv ({} plies) and README.md to {}\n",
        rows.len(),
        request.out.display()
    ))
}

/// One row a ply.
pub fn rows_csv(rows: &[Row]) -> String {
    let mut out = String::from("game_id,ply,turn,seat,opponent,top,won,turns_left,expected,actual,pass,surprise,modeled,modeled_best,same_reply,taken,cut,units_lost,train1,train2,train3,moves,towers,mines,spent,hq,hq_margin,without_moves,finder,finder_wide,gold,income,cells,units,opp_gold,opp_income,opp_cells,opp_units,new_lost,joints,joint_max,joint_hit,depth,hq_steps,gold_left,opp_gold_before,towers_held,towers_lost,move_captures,reply\n");
    for r in rows {
        let e = &r.effect;
        let j = &r.judged;
        let _ = writeln!(
            out,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},\"{}\"",
            r.game_id,
            r.ply,
            r.turn,
            r.seat,
            r.opponent,
            r.top,
            r.won,
            r.turns_left,
            j.expected,
            j.actual,
            j.pass,
            r.surprise(),
            j.modeled,
            j.modeled_best,
            j.same_reply,
            e.taken,
            e.cut,
            e.units_lost,
            e.trainings[0],
            e.trainings[1],
            e.trainings[2],
            e.moves,
            e.towers,
            e.mines,
            e.spent,
            e.hq,
            e.hq_margin.map_or(String::new(), |m| m.to_string()),
            e.without_moves,
            e.finder[0],
            e.finder[1],
            e.material
                .iter()
                .flatten()
                .map(i64::to_string)
                .collect::<Vec<_>>()
                .join(","),
            {
                let h = &r.shape;
                [
                    h.new_lost,
                    h.joints,
                    h.joint_max,
                    h.joint_hit,
                    h.depth,
                    h.hq_steps,
                    h.our_gold,
                    h.opp_gold,
                    h.towers,
                    h.towers_lost,
                    h.move_captures,
                ]
                .map(|v| v.to_string())
                .join(",")
            },
            r.reply.replace('"', "'")
        );
    }
    out
}

fn mean(values: impl Iterator<Item = i64>) -> String {
    let (sum, n) = values.fold((0i64, 0i64), |(s, n), v| (s + v, n + 1));
    if n == 0 {
        "-".to_string()
    } else {
        format!("{:.1}", sum as f64 / n as f64)
    }
}

fn share(part: usize, all: usize) -> String {
    if all == 0 {
        "-".to_string()
    } else {
        format!("{:.0}%", 100.0 * part as f64 / all as f64)
    }
}

/// The value at quantile `q` of sorted `values`.
fn quantile(sorted: &[i64], q: f64) -> i64 {
    if sorted.is_empty() {
        return 0;
    }
    sorted[((sorted.len() - 1) as f64 * q).round() as usize]
}

fn bucket(turn: u32) -> &'static str {
    match turn {
        0..=10 => "up to 10",
        11..=15 => "11 to 15",
        16..=20 => "16 to 20",
        21..=30 => "21 to 30",
        _ => "31 on",
    }
}

/// The report, as Markdown.
pub fn report(
    rows: &[Row],
    request: &Request,
    games: usize,
    unusable: usize,
    left: usize,
) -> String {
    let mut md = String::new();
    let _ = writeln!(
        md,
        "# Overconfidence of the search: expected against actual\n\nGames of `{}`{}: {games} games read ({unusable} unusable), {} of our plies judged by the plan bot's search ({} tries; {left} left out by the bot: opening or lethal plies). **Surprise** = actual − expected, in the evaluation's units (an active cell 4, income 3, gold 1, a unit 10 a level), capped at ±{CAP}; *large* is {LARGE} or less.\n",
        request.data.display(),
        request
            .release
            .as_deref()
            .map_or(String::new(), |r| format!(", release {r}")),
        rows.len(),
        request.tries
    );
    let _ = writeln!(md, "## Kinds of plies\n\nThe evaluation counts half a win against us when, at our start of turn, the opponent can pay a chain of trainings to our HQ; plies where either value holds that term are counted here, not in the surprise.\n\n| Kind | Plies | Share | Lost games among them |\n| --- | ---: | ---: | ---: |");
    let mut kinds: BTreeMap<Kind, Vec<&Row>> = BTreeMap::new();
    for r in rows {
        kinds.entry(r.kind()).or_default().push(r);
    }
    for (kind, g) in &kinds {
        let _ = writeln!(
            md,
            "| {} | {} | {} | {} |",
            kind.name(),
            g.len(),
            share(g.len(), rows.len()),
            share(g.iter().filter(|r| !r.won).count(), g.len())
        );
    }
    let _ = writeln!(md);
    let all = rows;
    let normal: Vec<Row> = rows
        .iter()
        .filter(|r| r.kind() == Kind::Normal)
        .cloned()
        .collect();
    let rows = &normal[..];
    let mut s: Vec<i64> = rows.iter().map(Row::surprise).collect();
    s.sort_unstable();
    let n = rows.len();
    let count = |f: &dyn Fn(&Row) -> bool| rows.iter().filter(|r| f(r)).count();
    let _ = writeln!(md, "## Distribution (normal plies)\n\n| Plies | Below expected | ≤ −20 | ≤ −50 | ≤ −100 | ≤ −200 | ≥ +50 | 5% | 25% | median | 75% | 95% |\n| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");
    let _ = writeln!(
        md,
        "| {n} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
        share(count(&|r| r.surprise() < 0), n),
        share(count(&|r| r.surprise() <= -20), n),
        share(count(&|r| r.surprise() <= -50), n),
        share(count(&|r| r.surprise() <= -100), n),
        share(count(&|r| r.surprise() <= -200), n),
        share(count(&|r| r.surprise() >= 50), n),
        quantile(&s, 0.05),
        quantile(&s, 0.25),
        quantile(&s, 0.5),
        quantile(&s, 0.75),
        quantile(&s, 0.95)
    );
    let _ = writeln!(md, "## By game turn and seat\n\n| Seat | Turns | Plies | Mean surprise | ≤ −50 | Mean expected − pass (the damage modeled) | Mean actual − pass (the damage done) |\n| ---: | --- | ---: | ---: | ---: | ---: | ---: |");
    let mut groups: BTreeMap<(usize, &str), Vec<&Row>> = BTreeMap::new();
    for r in rows {
        groups.entry((r.seat, bucket(r.turn))).or_default().push(r);
    }
    for ((seat, b), g) in &groups {
        let _ = writeln!(
            md,
            "| {seat} | {b} | {} | {} | {} | {} | {} |",
            g.len(),
            mean(g.iter().map(|r| r.surprise())),
            share(g.iter().filter(|r| r.surprise() <= LARGE).count(), g.len()),
            mean(
                g.iter()
                    .map(|r| (r.judged.expected - r.judged.pass).clamp(-CAP, CAP))
            ),
            mean(
                g.iter()
                    .map(|r| (r.judged.actual - r.judged.pass).clamp(-CAP, CAP))
            )
        );
    }
    let _ = writeln!(md, "\n## What the large surprises were\n\nMeans per ply; *cells taken* and *cut off* are our active cells at the end of our turn that the reply took or cut from our HQ; *units lost* their training cost; *HQ margin* the opponent's gold less its cheapest chain to our HQ at our next start of turn (above 0: it can pay it next turn).\n\n| Plies | Count | Cells taken | Cells cut off | Units lost | Train 1 | Train 2 | Train 3 | Moves | Towers | Gold spent | Some cut | Real reply among the modeled | Replies modeled | HQ margin ≥ 0 after |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");
    type Filter = Box<dyn Fn(&Row) -> bool>;
    let sets: [(&str, Filter); 3] = [
        ("large (≤ −50)", Box::new(|r: &Row| r.surprise() <= LARGE)),
        (
            "small (−50 to 0)",
            Box::new(|r: &Row| r.surprise() > LARGE && r.surprise() < 0),
        ),
        ("none (≥ 0)", Box::new(|r: &Row| r.surprise() >= 0)),
    ];
    for (name, f) in &sets {
        let g: Vec<&Row> = rows.iter().filter(|r| f(r)).collect();
        let _ = writeln!(
            md,
            "| {name} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            g.len(),
            mean(g.iter().map(|r| r.effect.taken)),
            mean(g.iter().map(|r| r.effect.cut)),
            mean(g.iter().map(|r| r.effect.units_lost)),
            mean(g.iter().map(|r| r.effect.trainings[0])),
            mean(g.iter().map(|r| r.effect.trainings[1])),
            mean(g.iter().map(|r| r.effect.trainings[2])),
            mean(g.iter().map(|r| r.effect.moves)),
            mean(g.iter().map(|r| r.effect.towers)),
            mean(g.iter().map(|r| r.effect.spent)),
            share(g.iter().filter(|r| r.effect.cut > 0).count(), g.len()),
            share(g.iter().filter(|r| r.judged.same_reply).count(), g.len()),
            mean(g.iter().map(|r| r.judged.modeled as i64)),
            share(
                g.iter()
                    .filter(|r| r.effect.hq_margin.is_some_and(|m| m >= 0))
                    .count(),
                g.len()
            )
        );
    }
    let _ = writeln!(md, "\n## Why the modeled replies missed it\n\nThe search's cut finder searches chains of trainings only ({} trainings at most, its {} best cuts); the wider finder {} and {}. The causes are read on the board, in this order.\n\n| Cause | Large (≤ −50) | Small (−50 to 0) | None (≥ 0) |\n| --- | ---: | ---: | ---: |", NARROW.0, NARROW.1, WIDE.0, WIDE.1);
    let causes = [
        Cause::NoCell,
        Cause::Moves,
        Cause::Found,
        Cause::Wider,
        Cause::Neither,
    ];
    for cause in causes {
        let mut cells = Vec::new();
        for (_, f) in &sets {
            let g: Vec<&Row> = rows.iter().filter(|r| f(r)).collect();
            cells.push(share(
                g.iter().filter(|r| r.cause() == cause).count(),
                g.len(),
            ));
        }
        let _ = writeln!(
            md,
            "| {} | {} | {} | {} |",
            cause.name(),
            cells[0],
            cells[1],
            cells[2]
        );
    }
    // Games: a large surprise and the result.
    let mut per_game: BTreeMap<u64, (bool, Option<u32>, u32)> = BTreeMap::new();
    for r in all {
        let e = per_game
            .entry(r.game_id)
            .or_insert((r.won, None, r.turn + r.turns_left));
        let large = match r.kind() {
            Kind::Normal => r.surprise() <= LARGE,
            Kind::ThreatMissed => true,
            _ => false,
        };
        if large && e.1.is_none_or(|t| r.turn < t) {
            e.1 = Some(r.turn);
        }
    }
    let with: Vec<_> = per_game.values().filter(|g| g.1.is_some()).collect();
    let without: Vec<_> = per_game.values().filter(|g| g.1.is_none()).collect();
    let lost = |v: &[&(bool, Option<u32>, u32)]| v.iter().filter(|g| !g.0).count();
    let _ = writeln!(
        md,
        "\n## Large surprises and the result\n\nA game has a large surprise when one of our plies is normal with a surprise of {LARGE} or less, or misses a threat on our HQ.\n\n| Games | Count | Lost | Median turns from the first large surprise to the end |\n| --- | ---: | ---: | ---: |\n| with a large surprise | {} | {} | {} |\n| without | {} | {} | - |\n",
        with.len(),
        share(lost(&with), with.len()),
        {
            let mut d: Vec<i64> = with
                .iter()
                .filter(|g| !g.0)
                .filter_map(|g| g.1.map(|t| i64::from(g.2) - i64::from(t)))
                .collect();
            d.sort_unstable();
            if d.is_empty() {
                "-".to_string()
            } else {
                format!("{} (losses)", quantile(&d, 0.5))
            }
        },
        without.len(),
        share(lost(&without), without.len())
    );
    let missed: Vec<&Row> = all
        .iter()
        .filter(|r| r.kind() == Kind::ThreatMissed)
        .collect();
    if !missed.is_empty() {
        let _ = writeln!(md, "## HQ threats not foreseen\n\n| Game | Turn | Seat | Opponent | Expected | HQ taken | HQ margin after | Taken | Cut off | Units lost | Spent | Reply |\n| ---: | ---: | ---: | --- | ---: | --- | ---: | ---: | ---: | ---: | ---: | --- |");
        for r in &missed {
            let reply: String = r.reply.chars().take(120).collect();
            let _ = writeln!(
                md,
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | `{reply}` |",
                r.game_id,
                r.turn,
                r.seat,
                r.opponent,
                r.judged.expected,
                if r.effect.hq { "yes" } else { "no" },
                r.effect
                    .hq_margin
                    .map_or("-".to_string(), |m| m.to_string()),
                r.effect.taken,
                r.effect.cut,
                r.effect.units_lost,
                r.effect.spent
            );
        }
        let _ = writeln!(md);
    }
    let _ = writeln!(md, "## The largest surprises\n\n| Game | Turn | Seat | Opponent | Expected | Actual | Pass | Taken | Cut off | Units lost | Spent | Reply |\n| ---: | ---: | ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |");
    let mut worst: Vec<&Row> = rows.iter().collect();
    worst.sort_by_key(|r| (r.surprise(), r.game_id, r.ply));
    for r in worst.iter().take(25) {
        let reply: String = r.reply.chars().take(120).collect();
        let _ = writeln!(
            md,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | `{reply}` |",
            r.game_id,
            r.turn,
            r.seat,
            r.opponent,
            r.judged.expected,
            r.judged.actual,
            r.judged.pass,
            r.effect.taken,
            r.effect.cut,
            r.effect.units_lost,
            r.effect.spent
        );
    }
    md
}

#[cfg(test)]
mod tests;
