//! The cell-value study (step 0): do per-cell values computed from a state
//! predict what happens next in real games?
//!
//! ```sh
//! cargo run --release -p ice-and-fire-arena --example cells -- OUT.json RECORD.json...
//! ```
//!
//! The records are CodinGame games converted by `ice-and-fire-review
//! codingame`. For every ply, with A the player to act and D the other:
//!
//! - **Fights.** For each active cell c of D, its *price*: the cheapest
//!   chain of trainings by which A takes c (`Reach::new(state, A, true)`,
//!   A's movable units stepping first), or takes any cell whose loss cuts c
//!   off from D's HQ, whichever is cheaper. The *margin* is the price minus
//!   A's gold (its income is already paid at the start of its ply). The
//!   outcome: whether c is still an active cell of D after A's ply. The same
//!   for the direct price alone (no cuts) and for the walking steps from A's
//!   active land, as baselines.
//! - **The early game.** At game turns 2, 4, 6, 8 and 10 (red to act), for
//!   each neutral cell, the *contest margin* m = (steps from blue's active
//!   land) − (steps from red's active land), and who holds the cell after
//!   game turn 12.
//! - **Games.** Per game: the winner, how and when it ended, both sides'
//!   active cells after game turn 12, and the cells each side then holds
//!   among those contested at game turn 4 (|m| ≤ 1).
//!
//! It writes one JSON file of histograms and per-game rows, which
//! `games/ice-and-fire/research/codingame/2026-11-27-cell-values/analyse.py`
//! reads.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use cg_arena::record::Record;
use ice_and_fire_engine::{Bits, Command, League, Outcome, State, CELLS, HQ};
use ice_and_fire_referee::map::generate;
use ice_and_fire_tactics::chain::{Reach, UNREACHABLE};

/// A histogram: key → [cells, cells lost].
type Hist = BTreeMap<String, [u64; 2]>;

/// The command parser of the review crate, copied: `MOVE id x y`, `TRAIN
/// level x y`, `BUILD MINE|TOWER x y`, `WAIT`.
fn parse(part: &str) -> Option<Command> {
    let words: Vec<&str> = part.split(' ').collect();
    let n = |i: usize| -> Option<u32> { words.get(i)?.parse().ok() };
    match *words.first()? {
        "WAIT" => Some(Command::Wait),
        "MOVE" if words.len() == 4 => Some(Command::Move {
            id: n(1)?,
            x: u8::try_from(n(2)?).ok()?,
            y: u8::try_from(n(3)?).ok()?,
        }),
        "TRAIN" if words.len() == 4 => Some(Command::Train {
            level: u8::try_from(n(1)?).ok()?,
            x: u8::try_from(n(2)?).ok()?,
            y: u8::try_from(n(3)?).ok()?,
        }),
        "BUILD" if words.len() == 4 => {
            let (x, y) = (u8::try_from(n(2)?).ok()?, u8::try_from(n(3)?).ok()?);
            match words[1] {
                "MINE" => Some(Command::BuildMine { x, y }),
                "TOWER" => Some(Command::BuildTower { x, y }),
                _ => None,
            }
        }
        _ => None,
    }
}

/// The cells of `own` joined to `hq` through `own`.
fn flood(own: Bits, hq: usize) -> Bits {
    if !own.test(hq) {
        return Bits::EMPTY;
    }
    let mut region = Bits::cell(hq);
    loop {
        let next = (region | region.neighbours()) & own;
        if next == region {
            return region;
        }
        region = next;
    }
}

/// Walking steps over `valid` cells from `from`; 255 where unreachable.
fn steps(valid: Bits, from: Bits) -> [u8; CELLS] {
    let mut dist = [u8::MAX; CELLS];
    let mut frontier = from & valid;
    let mut seen = frontier;
    let mut d = 0u8;
    while !frontier.is_empty() {
        for cell in frontier.iter() {
            dist[cell] = d;
        }
        frontier = frontier.neighbours() & valid & !seen;
        seen |= frontier;
        d = d.saturating_add(1);
    }
    dist
}

fn phase(turn: u32) -> &'static str {
    match turn {
        0..=6 => "t01-06",
        7..=12 => "t07-12",
        13..=20 => "t13-20",
        21..=40 => "t21-40",
        _ => "t41+",
    }
}

fn margin_bin(price: i64, gold: i64) -> String {
    if price == UNREACHABLE {
        return "none".to_string();
    }
    let m = (price - gold).clamp(-300, 300);
    format!("{:+04}", m.div_euclid(10) * 10)
}

#[derive(Default)]
struct Out {
    fights: Hist,
    early: BTreeMap<String, [u64; 3]>,
    games: Vec<serde_json::Value>,
    lost_kind: BTreeMap<String, [u64; 2]>,
}

fn add(h: &mut Hist, key: String, lost: bool) {
    let e = h.entry(key).or_insert([0, 0]);
    e[0] += 1;
    e[1] += u64::from(lost);
}

fn study(path: &PathBuf, out: &mut Out) -> Result<(), String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let record: Record = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let Some(origin) = &record.origin else {
        return Ok(());
    };
    // In test games our bot is always seat 0.
    if origin.kind != "test" {
        return Ok(());
    }
    let us = 0usize;
    let map = generate(record.seed, ice_and_fire_referee::League::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let valid = state.map().valid;
    let mut early_rows: Vec<(u32, usize, i32)> = Vec::new();
    let mut contested4 = Bits::EMPTY;
    let mut at12: Option<State> = None;
    let mut last_turn = 0;
    for answers in &record.turns {
        if state.outcome().is_some() {
            break;
        }
        let a = state.to_act();
        let d = 1 - a;
        let turn = state.game_turn();
        last_turn = turn;
        if a == 0 && turn == 13 && at12.is_none() {
            at12 = Some(state.clone());
        }
        // The early game, from red's starts of turn.
        if a == 0 && matches!(turn, 2 | 4 | 6 | 8 | 10) {
            let red = steps(valid, state.active(0));
            let blue = steps(valid, state.active(1));
            let neutral = valid.and_not(state.owned(0) | state.owned(1));
            for c in neutral.iter() {
                if red[c] == u8::MAX || blue[c] == u8::MAX {
                    continue;
                }
                let m = i32::from(blue[c]) - i32::from(red[c]);
                early_rows.push((turn, c, m));
                if turn == 4 && m.abs() <= 1 {
                    contested4.set(c);
                }
            }
        }
        let line = answers
            .iter()
            .find(|x| x.seat == a)
            .and_then(|x| x.lines.first())
            .cloned()
            .unwrap_or_default();
        let commands: Vec<Command> = line.split(';').filter_map(|p| parse(p.trim())).collect();
        let before = state.clone();
        state.play_turn(&commands);
        // Fights: D's active cells before A's ply, and what became of them.
        let reach = Reach::new(&before, a, true);
        let gold = before.gold(a);
        let mine = before.active(d);
        let mut price = [UNREACHABLE; CELLS];
        for c in mine.iter() {
            price[c] = reach.cost[c];
        }
        for cut in mine.iter() {
            if cut == HQ[d] || reach.cost[cut] == UNREACHABLE {
                continue;
            }
            let severed = mine.and_not(flood(mine.and_not(Bits::cell(cut)), HQ[d]));
            for c in severed.iter() {
                price[c] = price[c].min(reach.cost[cut]);
            }
        }
        let walk = steps(valid, before.active(a));
        let after = flood(state.owned(d), HQ[d]);
        let who = if a == us { "us" } else { "them" };
        let ph = phase(turn);
        for c in mine.iter() {
            let lost = !after.test(c);
            add(
                &mut out.fights,
                format!("{ph}|{who}|cut|{}", margin_bin(price[c], gold)),
                lost,
            );
            add(
                &mut out.fights,
                format!("{ph}|{who}|direct|{}", margin_bin(reach.cost[c], gold)),
                lost,
            );
            add(
                &mut out.fights,
                format!("{ph}|{who}|steps|{:02}", walk[c].min(20)),
                lost,
            );
            if lost {
                let taken = state.owned(a).test(c);
                let e = out.lost_kind.entry(format!("{ph}|{who}")).or_insert([0, 0]);
                e[usize::from(!taken)] += 1;
            }
        }
    }
    let snap = at12.unwrap_or_else(|| state.clone());
    let owner = |c: usize| -> usize {
        if snap.owned(0).test(c) {
            0
        } else if snap.owned(1).test(c) {
            1
        } else {
            2
        }
    };
    for (turn, c, m) in early_rows {
        let e = out
            .early
            .entry(format!("t{turn:02}|{:+03}", m.clamp(-10, 10)))
            .or_insert([0; 3]);
        e[owner(c)] += 1;
    }
    let end = match state.outcome() {
        Some(Outcome::Win(w)) if state.owned(w).test(HQ[1 - w]) => "hq",
        Some(_) if last_turn >= 100 => "turn100",
        Some(_) => "other",
        None => "unfinished",
    };
    out.games.push(serde_json::json!({
        "game": origin.game_id,
        "opponent": record.players[1].name,
        "winner": record.winner,
        "end": end,
        "turns": last_turn,
        "active12": [snap.active(0).count(), snap.active(1).count()],
        "contested12": [(snap.owned(0) & contested4).count(), (snap.owned(1) & contested4).count()],
        "contested": contested4.count(),
        "gold12": [snap.gold(0), snap.gold(1)],
        "income12": [snap.income(0), snap.income(1)],
    }));
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some((out_path, records)) = args.split_first() else {
        eprintln!("usage: cells OUT.json RECORD.json...");
        std::process::exit(2);
    };
    let paths: Vec<PathBuf> = records.iter().map(PathBuf::from).collect();
    let jobs = std::thread::available_parallelism().map_or(4, |n| n.get());
    let size = paths.len().div_ceil(jobs).max(1);
    let parts: Vec<Out> = std::thread::scope(|scope| {
        let handles: Vec<_> = paths
            .chunks(size)
            .map(|chunk| {
                scope.spawn(move || {
                    let mut out = Out::default();
                    for p in chunk {
                        if let Err(e) = study(p, &mut out) {
                            eprintln!("{}: {e}", p.display());
                        }
                    }
                    out
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let mut all = Out::default();
    for part in parts {
        for (k, v) in part.fights {
            let e = all.fights.entry(k).or_insert([0, 0]);
            e[0] += v[0];
            e[1] += v[1];
        }
        for (k, v) in part.lost_kind {
            let e = all.lost_kind.entry(k).or_insert([0, 0]);
            e[0] += v[0];
            e[1] += v[1];
        }
        for (k, v) in part.early {
            let e = all.early.entry(k).or_insert([0; 3]);
            for i in 0..3 {
                e[i] += v[i];
            }
        }
        all.games.extend(part.games);
    }
    let json = serde_json::json!({
        "fights": all.fights,
        "lost_kind": all.lost_kind,
        "early": all.early,
        "games": all.games,
    });
    fs::write(out_path, serde_json::to_string(&json).unwrap()).unwrap();
    eprintln!("{} games", all.games.len());
}
