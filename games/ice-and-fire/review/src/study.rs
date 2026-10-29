//! The tower and threat study of a batch of records (`ice-and-fire-review
//! towers`): every tower built and what became of it, every turn's threat
//! to each HQ and how far it was from being affordable, and every turn's
//! cuts, against what was played.
//!
//! The chain to an HQ is the one the shared evaluation's pressure term
//! counts: `ice_and_fire_tactics::chain::cost_to` without moves, the
//! cheapest chain of trainings on the costs of the position (a tower's
//! protection is taken as lasting). Cuts are the trace's: the 8 best cuts of
//! up to 5 trainings, by the cells they take or cut off.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use ice_and_fire_engine::{Bits, Command, State, HQ, SIZE};
use ice_and_fire_tactics::chain::{cost_to, UNREACHABLE};

use crate::replay::Game;
use crate::trace::best_cut;

/// The lowest pressure term, as in the shared evaluation.
const MAX_PRESSURE: i64 = 200;
/// A large cut, as the trace's moments count one: at least this many cells.
pub const LARGE: i64 = 6;

/// The cost of the cheapest chain of trainings of `seat` to the enemy HQ,
/// as the pressure term counts it; `None` when no chain reaches it.
pub fn chain_cost(state: &State, seat: usize) -> Option<i64> {
    let cost = cost_to(state, seat, false, HQ[1 - seat]);
    (cost != UNREACHABLE).then_some(cost)
}

/// The turns until `gold` plus that many turns of `income` pays `cost`: 0
/// when `gold` pays it now, `None` when it never will at this income.
pub fn turns_to_afford(gold: i64, income: i64, cost: Option<i64>) -> Option<i64> {
    let cost = cost?;
    if gold >= cost {
        Some(0)
    } else if income <= 0 {
        None
    } else {
        Some((cost - gold + income - 1) / income)
    }
}

/// The pressure term of a side with `gold`, `income` and its chain `cost`.
pub fn pressure(gold: i64, income: i64, cost: Option<i64>) -> i64 {
    match cost {
        None => -MAX_PRESSURE,
        Some(cost) => (gold + income - cost).clamp(-MAX_PRESSURE, 0),
    }
}

/// The threat of the player to act on the enemy HQ at the start of its ply.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Threat {
    pub ply: usize,
    pub game_turn: u32,
    pub attacker: usize,
    pub gold: i64,
    pub income: i64,
    pub cost: Option<i64>,
    /// [`turns_to_afford`] of the chain.
    pub turns: Option<i64>,
    /// The attacker's turns from this one (0) to the one in which it took
    /// the HQ, if it did.
    pub taken_in: Option<i64>,
}

/// The cuts of the player to act at the start of its ply, and what it and
/// its next turns took.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CutTurn {
    pub ply: usize,
    pub game_turn: u32,
    pub attacker: usize,
    /// The most cells of the best cut, and its cost, with the gold of now,
    /// of one more income and of two (the position unchanged).
    pub cells: [i64; 3],
    pub cost: [i64; 3],
    /// The defender's active cells lost in the attacker's turn, and in its
    /// next two (`None` once the game is over).
    pub lost: [Option<i64>; 3],
}

/// A tower built, and what became of it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tower {
    pub ply: usize,
    pub game_turn: u32,
    pub builder: usize,
    pub cell: usize,
    /// The builder's gold and income at the start of its turn.
    pub gold: i64,
    pub income: i64,
    /// The enemy's gold and income when the tower was built.
    pub enemy_gold: i64,
    pub enemy_income: i64,
    /// The enemy's cheapest chain to the builder's HQ just before and just
    /// after the tower, and after the builder's whole turn.
    pub cost_before: Option<i64>,
    pub cost_after: Option<i64>,
    pub cost_turn_after: Option<i64>,
    /// The enemy's turns (its next one being 1) until its gold and income
    /// pay the chain of just before the tower.
    pub enemy_turns: Option<i64>,
    /// The enemy's turns after the build (its next one is 1) until: a protected cell
    /// the tower alone keeps from a training of level 1 or 2 is next to the
    /// enemy's territory at its turn, with the gold to train there (`block`);
    /// the same with an enemy unit of level 1 or 2 next to it that could
    /// otherwise step in (`unit_block`); the enemy has the gold of the chain
    /// before the tower at its turn (`afforded`); the tower is cut off from
    /// the HQ, or gone (`cut_off`, `gone`).
    pub block: Option<u32>,
    pub unit_block: Option<u32>,
    pub afforded: Option<u32>,
    pub cut_off: Option<u32>,
    pub gone: Option<u32>,
    /// Whether the builder won the game, and whether it lost its HQ.
    pub won: bool,
    pub hq_lost: bool,
}

/// What the study takes from one game.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GameStudy {
    pub name: String,
    pub players: [String; 2],
    pub winner: Option<usize>,
    pub hq_taken_by: Option<usize>,
    pub threats: Vec<Threat>,
    pub cuts: Vec<CutTurn>,
    pub towers: Vec<Tower>,
}

/// The cells next to `cell`.
fn near(cell: usize) -> Bits {
    Bits::cell(cell).neighbours()
}

/// The cells that the tower on `tower` of `seat` alone protects: its own
/// cell and its active neighbours, less those next to another active tower
/// of the seat (which would protect them anyway).
fn sole_cover(state: &State, seat: usize, tower: usize) -> Bits {
    let active = state.active(seat);
    let others = (state.towers() & active).and_not(Bits::cell(tower));
    let shared = others | others.neighbours();
    (Bits::cell(tower) | (near(tower) & active)).and_not(shared)
}

/// What the tower on `tower` of `seat` keeps the enemy, to act, from doing
/// at the start of its turn: a training of level 1 or 2 into a cell it alone
/// protects, and a step of a level 1 or 2 unit into one.
fn blocks(state: &State, seat: usize, tower: usize) -> (bool, bool) {
    let enemy = 1 - seat;
    let mut training = false;
    let mut step = false;
    let territory = state.active(enemy).neighbours();
    let gold = state.gold(enemy);
    for cell in sole_cover(state, seat, tower).iter() {
        // The lowest level that could enter without the tower.
        let needed = state.unit_level(cell).map_or(1, |level| level + 1);
        if needed >= 3 {
            continue;
        }
        if territory.test(cell) && gold >= 10 * i64::from(needed) {
            training = true;
        }
        for from in (near(cell) & state.units(enemy)).iter() {
            if state.unit_level(from).is_some_and(|l| l >= needed && l < 3) {
                step = true;
            }
        }
    }
    (training, step)
}

/// The towers built in `game`.
fn towers(game: &Game) -> Vec<Tower> {
    let mut out = Vec::new();
    let taken = game.hq_taken_by();
    for (ply, step) in game.steps.iter().enumerate() {
        let start = &step.state;
        let builder = start.to_act();
        let enemy = 1 - builder;
        let mut sim = start.clone();
        let first = out.len();
        for &command in &step.commands {
            let before = sim.clone();
            if !sim.apply(command) {
                continue;
            }
            let Command::BuildTower { x, y } = command else {
                continue;
            };
            let cost_before = chain_cost(&before, enemy);
            let (enemy_gold, enemy_income) = (sim.gold(enemy), sim.income(enemy));
            out.push(Tower {
                ply,
                game_turn: start.game_turn(),
                builder,
                cell: usize::from(y) * SIZE + usize::from(x),
                gold: start.gold(builder),
                income: start.income(builder),
                enemy_gold,
                enemy_income,
                cost_before,
                cost_after: chain_cost(&sim, enemy),
                cost_turn_after: None,
                // From the start of its next turn, which adds one income.
                enemy_turns: turns_to_afford(enemy_gold, enemy_income, cost_before)
                    .map(|n| n.max(1)),
                block: None,
                unit_block: None,
                afforded: None,
                cut_off: None,
                gone: None,
                won: game.record.winner == Some(builder),
                hq_lost: taken == Some(enemy),
            });
        }
        let after = game.steps.get(ply + 1).map_or(&game.end, |s| &s.state);
        for tower in &mut out[first..] {
            tower.cost_turn_after = chain_cost(after, enemy);
        }
    }
    for tower in &mut out {
        follow(game, tower);
    }
    out
}

/// What became of `tower` after the ply it was built in, counted in the
/// enemy's turns since then (its next one is 1).
fn follow(game: &Game, tower: &mut Tower) {
    let (seat, cell) = (tower.builder, tower.cell);
    let mut later = 0;
    for step in &game.steps[tower.ply + 1..] {
        let state = &step.state;
        let enemy_turn = state.to_act() != seat;
        later += u32::from(enemy_turn);
        if !(state.towers().test(cell) && state.owned(seat).test(cell)) {
            tower.gone = Some(later);
            return;
        }
        if !state.active(seat).test(cell) {
            tower.cut_off.get_or_insert(later);
            continue;
        }
        if enemy_turn {
            let (training, unit) = blocks(state, seat, cell);
            if training {
                tower.block.get_or_insert(later);
            }
            if unit {
                tower.unit_block.get_or_insert(later);
            }
            if tower
                .cost_before
                .is_some_and(|cost| state.gold(1 - seat) >= cost)
            {
                tower.afforded.get_or_insert(later);
            }
        }
    }
}

/// Studies one replayed game.
pub fn study(game: &Game) -> GameStudy {
    let taken = game.hq_taken_by();
    let last = game.steps.len().saturating_sub(1);
    let mut threats = Vec::new();
    let mut cut_turns = Vec::new();
    for (ply, step) in game.steps.iter().enumerate() {
        let state = &step.state;
        let attacker = state.to_act();
        let defender = 1 - attacker;
        let (gold, income) = (state.gold(attacker), state.income(attacker));
        let cost = chain_cost(state, attacker);
        threats.push(Threat {
            ply,
            game_turn: state.game_turn(),
            attacker,
            gold,
            income,
            cost,
            turns: turns_to_afford(gold, income, cost),
            taken_in: (taken == Some(attacker)).then(|| ((last - ply) / 2) as i64),
        });
        let budgets = [gold, gold + income.max(0), gold + 2 * income.max(0)];
        let mut cells = [0; 3];
        let mut costs = [0; 3];
        for (k, &budget) in budgets.iter().enumerate() {
            (cells[k], costs[k]) = if k > 0 && budget == budgets[k - 1] {
                (cells[k - 1], costs[k - 1])
            } else {
                best_cut(state, attacker, budget)
            };
        }
        let mut lost = [None; 3];
        for (k, slot) in lost.iter_mut().enumerate() {
            let at = ply + 2 * k;
            let Some(before) = game.steps.get(at) else {
                break;
            };
            let after = game.steps.get(at + 1).map_or(&game.end, |s| &s.state);
            *slot = Some(
                i64::from(before.state.active(defender).count())
                    - i64::from(after.active(defender).count()),
            );
        }
        cut_turns.push(CutTurn {
            ply,
            game_turn: state.game_turn(),
            attacker,
            cells,
            cost: costs,
            lost,
        });
    }
    GameStudy {
        name: game.name.clone(),
        players: [
            game.record.players[0].name.clone(),
            game.record.players[1].name.clone(),
        ],
        winner: game.record.winner,
        hq_taken_by: taken,
        threats,
        cuts: cut_turns,
        towers: towers(game),
    }
}

/// The value at quantile `q` (0 to 1) of `values`, `None` when empty.
fn quantile(values: &mut [i64], q: f64) -> Option<i64> {
    if values.is_empty() {
        return None;
    }
    values.sort_unstable();
    let index = ((values.len() - 1) as f64 * q).round() as usize;
    Some(values[index])
}

/// A median as text, `–` when there is none.
fn median_text(mut values: Vec<i64>) -> String {
    quantile(&mut values, 0.5).map_or("–".to_string(), |v| v.to_string())
}

/// `part` of `whole` as a percentage with one decimal, `–` when empty.
fn share(part: usize, whole: usize) -> String {
    if whole == 0 {
        "–".to_string()
    } else {
        format!("{:.1}%", 100.0 * part as f64 / whole as f64)
    }
}

/// A bucket of turns until affordable.
fn turns_bucket(turns: Option<i64>) -> &'static str {
    match turns {
        Some(0) => "0",
        Some(1) => "1",
        Some(2) => "2",
        Some(3) => "3",
        Some(4) => "4",
        Some(5..=6) => "5-6",
        Some(7..=9) => "7-9",
        Some(10..=14) => "10-14",
        Some(_) => "15+",
        None => "never",
    }
}

const TURN_BUCKETS: [&str; 10] = [
    "0", "1", "2", "3", "4", "5-6", "7-9", "10-14", "15+", "never",
];

/// The fit of `p(k) = p1 * r^(k - 1)` to the chance that a threat `k` turns
/// from affordable (1 to `max`) is carried out within the game, by maximum
/// likelihood over a grid: `(p1, r, log likelihood)`.
pub fn fit_geometric(counts: &BTreeMap<i64, (usize, usize)>, max: i64) -> Option<(f64, f64, f64)> {
    let mut best: Option<(f64, f64, f64)> = None;
    for pi in 1..=200 {
        let p1 = f64::from(pi) / 200.0;
        for ri in 1..=199 {
            let r = f64::from(ri) / 200.0;
            let mut ll = 0.0;
            for (&k, &(n, hit)) in counts.range(1..=max) {
                let p = (p1 * r.powi((k - 1) as i32)).clamp(1e-9, 1.0 - 1e-9);
                ll += hit as f64 * p.ln() + (n - hit) as f64 * (1.0 - p).ln();
            }
            if best.is_none_or(|(_, _, top)| ll > top) {
                best = Some((p1, r, ll));
            }
        }
    }
    best
}

/// The Markdown report of the study of `games`.
pub fn report(games: &[GameStudy]) -> String {
    let mut out = String::new();
    let n = games.len();
    let _ = writeln!(out, "# Towers and threats\n");
    let hq = games.iter().filter(|g| g.hq_taken_by.is_some()).count();
    let red = games.iter().filter(|g| g.winner == Some(0)).count();
    let _ = writeln!(
        out,
        "{n} games; red won {} ({}); {hq} ended by taking an HQ ({}).\n",
        red,
        share(red, n),
        share(hq, n)
    );
    towers_report(&mut out, games);
    threats_report(&mut out, games);
    cuts_report(&mut out, games);
    out
}

/// A row of the towers' table: its label and which towers it holds.
type Bucket = (&'static str, fn(&Tower) -> bool);

fn towers_report(out: &mut String, games: &[GameStudy]) {
    let n = games.len();
    let all: Vec<&Tower> = games.iter().flat_map(|g| &g.towers).collect();
    let _ = writeln!(out, "## Towers\n");
    let by_seat = |seat: usize| all.iter().filter(|t| t.builder == seat).count();
    let won = all.iter().filter(|t| t.won).count();
    let per_game = |k: usize| {
        if n == 0 {
            0.0
        } else {
            k as f64 / n as f64
        }
    };
    let none = games
        .iter()
        .flat_map(|g| [0, 1].map(|s| g.towers.iter().all(|t| t.builder != s)))
        .filter(|&b| b)
        .count();
    let _ = writeln!(
        out,
        "{} towers built, {:.2} a game: {:.2} by red, {:.2} by blue; {} by the side that won. {} of {} sides built none.\n",
        all.len(),
        per_game(all.len()),
        per_game(by_seat(0)),
        per_game(by_seat(1)),
        share(won, all.len()),
        none,
        2 * n
    );
    let mut turns: Vec<i64> = all.iter().map(|t| i64::from(t.game_turn)).collect();
    let q = |v: &mut Vec<i64>, p: f64| quantile(v, p).map_or("–".into(), |x| x.to_string());
    let _ = writeln!(
        out,
        "| When built and with what | Quartile 1 | Median | Quartile 3 |\n| --- | ---: | ---: | ---: |"
    );
    let mut row = |what: &str, mut values: Vec<i64>| {
        let _ = writeln!(
            out,
            "| {what} | {} | {} | {} |",
            q(&mut values, 0.25),
            q(&mut values, 0.5),
            q(&mut values, 0.75)
        );
    };
    row("Game turn", std::mem::take(&mut turns));
    row(
        "Builder's gold at the start of its turn",
        all.iter().map(|t| t.gold).collect(),
    );
    row("Builder's income", all.iter().map(|t| t.income).collect());
    row("Enemy's gold", all.iter().map(|t| t.enemy_gold).collect());
    row(
        "Enemy's income",
        all.iter().map(|t| t.enemy_income).collect(),
    );
    row(
        "Enemy's chain to the builder's HQ, just before",
        all.iter().filter_map(|t| t.cost_before).collect(),
    );
    row(
        "The same just after the tower",
        all.iter().filter_map(|t| t.cost_after).collect(),
    );
    row(
        "The same after the builder's whole turn",
        all.iter().filter_map(|t| t.cost_turn_after).collect(),
    );
    row(
        "The tower's raise of that chain",
        all.iter()
            .filter_map(|t| Some(t.cost_after? - t.cost_before?))
            .collect(),
    );
    row(
        "The tower's gain in the builder's pressure term (the enemy's pressure lowered)",
        all.iter()
            .map(|t| {
                pressure(t.enemy_gold, t.enemy_income, t.cost_before)
                    - pressure(t.enemy_gold, t.enemy_income, t.cost_after)
            })
            .collect(),
    );
    let _ = writeln!(
        out,
        "\nWhat became of the towers, by how far the enemy was from affording its chain to the builder's HQ just before the tower (its turns, the next one being 1). \"Blocked a training\": at a later enemy turn, a cell the tower alone protects was next to the enemy's territory, could otherwise be entered at level 1 or 2, and the enemy had the gold. \"Blocked a unit\": an enemy unit of level 1 or 2 stood next to such a cell and could otherwise have stepped in. \"Enemy afforded the old chain\": the enemy later had the gold of the chain as it was before the tower.\n"
    );
    let _ = writeln!(
        out,
        "| Enemy's turns to afford | Towers | Raised the chain | Blocked a training | Blocked a unit | Enemy afforded the old chain | Cut off | Gone | Builder won | Builder lost its HQ |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"
    );
    let buckets: [Bucket; 6] = [
        ("1", |t| t.enemy_turns == Some(1)),
        ("2", |t| t.enemy_turns == Some(2)),
        ("3-4", |t| matches!(t.enemy_turns, Some(3..=4))),
        ("5-9", |t| matches!(t.enemy_turns, Some(5..=9))),
        ("10+", |t| matches!(t.enemy_turns, Some(10..))),
        ("never", |t| t.enemy_turns.is_none()),
    ];
    let mut line = |label: &str, set: Vec<&&Tower>| {
        let k = set.len();
        let count = |f: &dyn Fn(&Tower) -> bool| share(set.iter().filter(|t| f(t)).count(), k);
        let _ = writeln!(
            out,
            "| {label} | {k} | {} | {} | {} | {} | {} | {} | {} | {} |",
            count(
                &|t| matches!((t.cost_before, t.cost_after), (Some(b), Some(a)) if a > b)
                    || (t.cost_before.is_some() && t.cost_after.is_none())
            ),
            count(&|t| t.block.is_some()),
            count(&|t| t.unit_block.is_some()),
            count(&|t| t.afforded.is_some()),
            count(&|t| t.cut_off.is_some()),
            count(&|t| t.gone.is_some()),
            count(&|t| t.won),
            count(&|t| t.hq_lost),
        );
    };
    for (label, test) in buckets {
        line(label, all.iter().filter(|t| test(t)).collect());
    }
    line("all", all.iter().collect());
    let _ = writeln!(
        out,
        "\nMedian enemy turns after the build (its next one is 1), among the towers it happened to: first blocked a training {}, first blocked a unit {}, the enemy afforded the old chain {}, cut off {}, gone {}.\n",
        median_text(all.iter().filter_map(|t| t.block.map(i64::from)).collect()),
        median_text(all.iter().filter_map(|t| t.unit_block.map(i64::from)).collect()),
        median_text(all.iter().filter_map(|t| t.afforded.map(i64::from)).collect()),
        median_text(all.iter().filter_map(|t| t.cut_off.map(i64::from)).collect()),
        median_text(all.iter().filter_map(|t| t.gone.map(i64::from)).collect()),
    );
    let idle = all
        .iter()
        .filter(|t| t.block.is_none() && t.unit_block.is_none())
        .count();
    let _ = writeln!(
        out,
        "{} of the towers never blocked a training or a unit ({}).\n",
        idle,
        share(idle, all.len())
    );
}

fn threats_report(out: &mut String, games: &[GameStudy]) {
    let rows: Vec<&Threat> = games.iter().flat_map(|g| &g.threats).collect();
    let _ = writeln!(out, "## Threats to the HQ\n");
    let _ = writeln!(
        out,
        "At the start of each ply, the turns until the player to act could pay its cheapest chain to the enemy HQ (0: now; its gold plus that many turns of its income at the time), against whether and when it took that HQ (its turns from this one, 0 being this one). A game that ends by taking an HQ is won by the side that took it, so \"HQ taken in the game\" is also the share of plies whose player went on to win that way.\n"
    );
    let _ = writeln!(
        out,
        "| Turns to afford | Plies | Median chain cost | HQ taken in the game | Within the turns to afford | Within those + 2 | Within 2 turns | Within 5 turns | Median turns until taken |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"
    );
    for bucket in TURN_BUCKETS {
        let set: Vec<&&Threat> = rows
            .iter()
            .filter(|t| turns_bucket(t.turns) == bucket)
            .collect();
        let k = set.len();
        let taken = set.iter().filter(|t| t.taken_in.is_some()).count();
        let within = |extra: i64| {
            set.iter()
                .filter(|t| match (t.taken_in, t.turns) {
                    (Some(at), Some(turns)) => at <= turns + extra,
                    _ => false,
                })
                .count()
        };
        let soon = |turns: i64| {
            set.iter()
                .filter(|t| t.taken_in.is_some_and(|at| at <= turns))
                .count()
        };
        let _ = writeln!(
            out,
            "| {bucket} | {k} | {} | {} | {} | {} | {} | {} | {} |",
            median_text(set.iter().filter_map(|t| t.cost).collect()),
            share(taken, k),
            share(within(0), k),
            share(within(2), k),
            share(soon(2), k),
            share(soon(5), k),
            median_text(set.iter().filter_map(|t| t.taken_in).collect()),
        );
    }
    // The chance by single turn, for the fit.
    // By single turn, three outcomes: taken in the game, within the turns
    // to afford plus 2, within 5 turns.
    let mut counts: [BTreeMap<i64, (usize, usize)>; 3] = Default::default();
    for t in &rows {
        if let Some(k) = t.turns {
            let hits = [
                t.taken_in.is_some(),
                t.taken_in.is_some_and(|at| at <= k + 2),
                t.taken_in.is_some_and(|at| at <= 5),
            ];
            for (map, hit) in counts.iter_mut().zip(hits) {
                let entry = map.entry(k.min(30)).or_default();
                entry.0 += 1;
                entry.1 += usize::from(hit);
            }
        }
    }
    let _ = writeln!(
        out,
        "\nBy single turn: the share of plies whose player took the HQ in the game, within the turns to afford plus 2, and within 5 turns, each also relative to a threat one turn away.\n"
    );
    let _ = writeln!(
        out,
        "| Turns to afford | Plies | Taken in the game | Relative | Within k + 2 | Relative | Within 5 turns | Relative |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"
    );
    let rate = |map: &BTreeMap<i64, (usize, usize)>, k: i64| {
        map.get(&k).map(|&(n, hit)| hit as f64 / n.max(1) as f64)
    };
    for &k in counts[0].keys().filter(|&&k| k <= 15) {
        let n = counts[0][&k].0;
        let _ = write!(out, "| {k} | {n} |");
        for map in &counts {
            let hit = map[&k].1;
            let rel = match (rate(map, k), rate(map, 1)) {
                (Some(p), Some(p1)) if p1 > 0.0 => format!("{:.2}", p / p1),
                _ => "–".to_string(),
            };
            let _ = write!(out, " {} | {rel} |", share(hit, n));
        }
        let _ = writeln!(out);
    }
    let _ = writeln!(out);
    for (map, what) in counts
        .iter()
        .zip(["in the game", "within k + 2 turns", "within 5 turns"])
    {
        if let Some((p, r, _)) = fit_geometric(map, 15) {
            let _ = writeln!(
                out,
                "Fit over 1 to 15 turns (maximum likelihood), taken {what}: P = {p:.3} x {r:.3}^(k - 1).\n"
            );
        }
    }
    // How far ahead the captures were seen.
    let _ = writeln!(
        out,
        "Before each HQ taken: the taker's turns to afford its chain, n of its turns before the capture.\n"
    );
    let _ = writeln!(
        out,
        "| Turns before the capture | Games | Median turns to afford | At most n | Never at that income |\n| --- | ---: | ---: | ---: | ---: |"
    );
    for back in 1..=6i64 {
        let set: Vec<&&Threat> = rows.iter().filter(|t| t.taken_in == Some(back)).collect();
        let k = set.len();
        let ok = set
            .iter()
            .filter(|t| t.turns.is_some_and(|x| x <= back))
            .count();
        let never = set.iter().filter(|t| t.turns.is_none()).count();
        let _ = writeln!(
            out,
            "| {back} | {k} | {} | {} | {} |",
            median_text(set.iter().filter_map(|t| t.turns).collect()),
            share(ok, k),
            share(never, k)
        );
    }
    let _ = writeln!(out);
}

fn cuts_report(out: &mut String, games: &[GameStudy]) {
    let rows: Vec<&CutTurn> = games.iter().flat_map(|g| &g.cuts).collect();
    let _ = writeln!(out, "## Cuts\n");
    let _ = writeln!(
        out,
        "At the start of each ply, the most enemy active cells the player to act could take or cut off with its gold (the 8 best cuts of up to 5 trainings), against the enemy's active cells lost in that turn.\n"
    );
    let _ = writeln!(
        out,
        "| Cells of the best cut | Plies | Median cost | Median cells lost | Lost at least those cells | Lost at least half |\n| --- | ---: | ---: | ---: | ---: | ---: |"
    );
    let buckets: [(&str, std::ops::RangeInclusive<i64>); 6] = [
        ("0", 0..=0),
        ("1-2", 1..=2),
        ("3-5", 3..=5),
        ("6-9", 6..=9),
        ("10-19", 10..=19),
        ("20+", 20..=i64::MAX),
    ];
    for (label, range) in buckets {
        let set: Vec<&&CutTurn> = rows
            .iter()
            .filter(|c| range.contains(&c.cells[0]))
            .collect();
        let k = set.len();
        let full = set
            .iter()
            .filter(|c| c.lost[0].is_some_and(|l| l >= c.cells[0]))
            .count();
        let half = set
            .iter()
            .filter(|c| c.lost[0].is_some_and(|l| 2 * l >= c.cells[0]))
            .count();
        let _ = writeln!(
            out,
            "| {label} | {k} | {} | {} | {} | {} |",
            median_text(
                set.iter()
                    .filter(|c| c.cells[0] > 0)
                    .map(|c| c.cost[0])
                    .collect()
            ),
            median_text(set.iter().filter_map(|c| c.lost[0]).collect()),
            share(full, k),
            share(half, k)
        );
    }
    let _ = writeln!(
        out,
        "\nLarge cuts ({LARGE} cells or more) first in reach with the gold of k more incomes (the position unchanged), and how often the enemy lost {LARGE} cells or more in one turn by then.\n"
    );
    let _ = writeln!(
        out,
        "| k | Plies | Lost 6+ in one turn within k turns | Within k + 1 |\n| --- | ---: | ---: | ---: |"
    );
    for k in 0..3 {
        let set: Vec<&&CutTurn> = rows
            .iter()
            .filter(|c| c.cells[k] >= LARGE && (k == 0 || c.cells[k - 1] < LARGE))
            .collect();
        let within = |upto: usize| {
            set.iter()
                .filter(|c| c.lost[..=upto.min(2)].iter().flatten().any(|&l| l >= LARGE))
                .count()
        };
        let _ = writeln!(
            out,
            "| {k} | {} | {} | {} |",
            set.len(),
            share(within(k), set.len()),
            if k < 2 {
                share(within(k + 1), set.len())
            } else {
                "–".to_string()
            }
        );
    }
    let _ = writeln!(out);
}

/// The study's rows as CSV files: (file name, text).
pub fn csv(games: &[GameStudy]) -> Vec<(&'static str, String)> {
    let opt = |v: Option<i64>| v.map_or(String::new(), |x| x.to_string());
    let optu = |v: Option<u32>| v.map_or(String::new(), |x| x.to_string());
    let mut towers = String::from("game,ply,game_turn,builder,x,y,gold,income,enemy_gold,enemy_income,cost_before,cost_after,cost_turn_after,enemy_turns,block,unit_block,afforded,cut_off,gone,won,hq_lost\n");
    let mut threats = String::from("game,ply,game_turn,attacker,gold,income,cost,turns,taken_in\n");
    let mut cuts = String::from(
        "game,ply,game_turn,attacker,cells0,cost0,cells1,cost1,cells2,cost2,lost0,lost1,lost2\n",
    );
    for g in games {
        for t in &g.towers {
            let _ = writeln!(
                towers,
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                g.name,
                t.ply,
                t.game_turn,
                t.builder,
                t.cell % SIZE,
                t.cell / SIZE,
                t.gold,
                t.income,
                t.enemy_gold,
                t.enemy_income,
                opt(t.cost_before),
                opt(t.cost_after),
                opt(t.cost_turn_after),
                opt(t.enemy_turns),
                optu(t.block),
                optu(t.unit_block),
                optu(t.afforded),
                optu(t.cut_off),
                optu(t.gone),
                u8::from(t.won),
                u8::from(t.hq_lost)
            );
        }
        for t in &g.threats {
            let _ = writeln!(
                threats,
                "{},{},{},{},{},{},{},{},{}",
                g.name,
                t.ply,
                t.game_turn,
                t.attacker,
                t.gold,
                t.income,
                opt(t.cost),
                opt(t.turns),
                opt(t.taken_in)
            );
        }
        for c in &g.cuts {
            let _ = writeln!(
                cuts,
                "{},{},{},{},{},{},{},{},{},{},{},{},{}",
                g.name,
                c.ply,
                c.game_turn,
                c.attacker,
                c.cells[0],
                c.cost[0],
                c.cells[1],
                c.cost[1],
                c.cells[2],
                c.cost[2],
                opt(c.lost[0]),
                opt(c.lost[1]),
                opt(c.lost[2])
            );
        }
    }
    vec![
        ("towers.csv", towers),
        ("threats.csv", threats),
        ("cuts.csv", cuts),
    ]
}

#[cfg(test)]
mod tests;
