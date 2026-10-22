//! The opening benchmark: replays arena game records through the engine and
//! measures how each bot spent the first turns, up to the moment the two
//! territories meet.
//!
//! ```sh
//! target/release/ice-and-fire-arena match --bot a=... --bot b=... \
//!   --pairs 100 --records DIR
//! cargo run --release -p ice-and-fire-arena --example opening -- DIR
//! ```
//!
//! For every game it rebuilds the start of each player's turn with the
//! engine (which the parity tests hold to the referee) and the commands the
//! record holds. **Contact** is the first start of a player's turn at which
//! the two territories are at most 2 cells apart, so that a training on the
//! cell between them touches the enemy. At a bot's first start of turn in
//! contact, it reports the bot's active cells, income, gold, units, and its
//! share of the map by Voronoi (each cell to the territory nearer to it,
//! halves on a tie). After that turn, at the opponent's start, it reports
//! the bot's **exposure**: the opponent's best cut (the tactics' cut finder,
//! up to 4 trainings within its gold) in cells and unit value removed, and
//! the opponent's **chain margin**, its gold minus the cheapest chain of
//! trainings to the bot's HQ (lethal when at least 0). It also counts the
//! bot's wipe-outs (its gold could not pay its upkeep), its trainings of
//! level 2 or 3 before contact, and the game's result.
//!
//! At the **middle game** (`ice_and_fire_opening::middle`: the first start of
//! a player's turn whose gold pays a chain of trainings to an enemy cell) it
//! scores both sides with `ice_and_fire_opening::score` (E004) and reports
//! each bot's mean score and terms there, and how often the sign of the score
//! agrees with the game's winner. With `OPENING_CSV=FILE`, it also writes one
//! line per game and seat: the bot, the result, the middle game's turn, the
//! score and the terms of each side.
//!
//! Records whose games end before contact count in the results only.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use cg_arena::record::Record;
use ice_and_fire_engine::{Bits, Command, League, State, HQ};
use ice_and_fire_opening::middle;
use ice_and_fire_opening::score::{self, Settings, Terms};
use ice_and_fire_referee::map::generate;
use ice_and_fire_tactics::chain::{Reach, UNREACHABLE};
use ice_and_fire_tactics::cut::cuts;

/// Game turns measured at fixed points as well as at contact.
const CHECKPOINTS: [u32; 3] = [5, 8, 12];

/// One command of an answer line, as the referee reads it; `MSG` and
/// anything unknown are skipped (the arena would have ended the game on an
/// unknown command).
fn parse(part: &str) -> Option<Command> {
    let words: Vec<&str> = part.split_whitespace().collect();
    let num = |i: usize| words.get(i).and_then(|w| w.parse::<u32>().ok());
    let small = |i: usize| num(i).and_then(|n| u8::try_from(n).ok());
    match words.first().copied()? {
        "MOVE" => Some(Command::Move {
            id: num(1)?,
            x: small(2)?,
            y: small(3)?,
        }),
        "TRAIN" => Some(Command::Train {
            level: small(1)?,
            x: small(2)?,
            y: small(3)?,
        }),
        "BUILD" => {
            let (x, y) = (small(2)?, small(3)?);
            match words.get(1).copied()? {
                "MINE" => Some(Command::BuildMine { x, y }),
                "TOWER" => Some(Command::BuildTower { x, y }),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Distances over non-void cells from the cells of `from`.
fn distances(state: &State, from: Bits) -> [u8; 144] {
    let mut dist = [u8::MAX; 144];
    let mut frontier = from;
    let mut seen = from;
    let mut d = 0u8;
    while !frontier.is_empty() {
        for cell in frontier.iter() {
            dist[cell] = d;
        }
        let next = (frontier.neighbours() & state.map().valid).and_not(seen);
        seen |= next;
        frontier = next;
        d = d.saturating_add(1);
    }
    dist
}

/// The gap between the two territories: the fewest steps from a cell of
/// one to a cell of the other.
fn gap(state: &State) -> u8 {
    let dist = distances(state, state.owned(0));
    state
        .owned(1)
        .iter()
        .map(|c| dist[c])
        .min()
        .unwrap_or(u8::MAX)
}

/// Twice the cells nearer to `seat`'s territory than to the other's (a tie
/// counts once).
fn voronoi2(state: &State, seat: usize) -> i64 {
    let mine = distances(state, state.owned(seat));
    let theirs = distances(state, state.owned(1 - seat));
    state
        .map()
        .valid
        .iter()
        .map(|c| match mine[c].cmp(&theirs[c]) {
            std::cmp::Ordering::Less => 2,
            std::cmp::Ordering::Equal => 1,
            std::cmp::Ordering::Greater => 0,
        })
        .sum()
}

/// A bot's numbers at its first start of turn in contact.
#[derive(Default, Clone, Copy)]
struct AtContact {
    turn: f64,
    cells: f64,
    income: f64,
    gold: f64,
    units: f64,
    mines: f64,
    voronoi: f64,
    /// Cells the opponent's best cut removes, after the bot's turn.
    cut_cells: f64,
    /// Unit value (10 per level) the opponent's best cut removes.
    cut_units: f64,
    /// The opponent's gold minus its cheapest chain to the bot's HQ.
    margin: f64,
    lethal: f64,
}

impl AtContact {
    fn add(&mut self, o: &AtContact) {
        self.turn += o.turn;
        self.cells += o.cells;
        self.income += o.income;
        self.gold += o.gold;
        self.units += o.units;
        self.mines += o.mines;
        self.voronoi += o.voronoi;
        self.cut_cells += o.cut_cells;
        self.cut_units += o.cut_units;
        self.margin += o.margin;
        self.lethal += o.lethal;
    }
}

#[derive(Default)]
struct Totals {
    games: u64,
    score: f64,
    contacts: u64,
    at: AtContact,
    /// Sums of (cells, income, gold) at each checkpoint, and their count.
    checkpoints: [(f64, f64, f64, u64); CHECKPOINTS.len()],
    wipes_early: u64,
    wipes: u64,
    high_level: u64,
    game_turns: f64,
    /// Games that reached the middle game, and the sums there of the bot's
    /// score, its terms and the opponent's terms.
    middles: u64,
    middle_score: f64,
    middle_turn: f64,
    ours: [f64; 6],
    theirs: [f64; 6],
    /// Games won or lost with a score of the winner's sign (draws and scores of
    /// 0 aside), and games won or lost.
    agree: u64,
    decided: u64,
}

/// The terms as numbers, in the order of the table.
fn term_list(t: &Terms) -> [f64; 6] {
    [t.voronoi, t.gold, t.income, t.exposure, t.debt, t.loss].map(|x| x as f64)
}

fn margin(state: &State) -> i64 {
    let me = state.to_act();
    let cost = Reach::new(state, me, false).cost[HQ[1 - me]];
    if cost == UNREACHABLE {
        -999
    } else {
        state.gold(me) - cost
    }
}

fn analyse(
    record: &Record,
    totals: &mut BTreeMap<String, Totals>,
    csv: &mut String,
) -> Result<(), String> {
    let map = generate(record.seed, ice_and_fire_referee::League::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let names = [
        record.players[0].name.clone(),
        record.players[1].name.clone(),
    ];
    let mut contact: [Option<AtContact>; 2] = [None, None];
    // Gold at the end of each seat's last turn, to see a wipe-out.
    let mut end_gold: [Option<i64>; 2] = [None, None];
    let mut units_before: [u32; 2] = [0, 0];
    let mut seen_checkpoint = [[false; CHECKPOINTS.len()]; 2];
    let mut wipes = [[0u64; 2]; 2];
    let mut high = [0u64; 2];
    // The seat whose exposure is measured at this start of turn.
    let mut pending: Option<usize> = None;
    // The state at the middle game.
    let mut at_middle: Option<State> = None;

    for answers in &record.turns {
        if state.outcome().is_some() {
            break;
        }
        let seat = state.to_act();
        let turn = state.game_turn();
        if at_middle.is_none() && middle::reached(&state) {
            at_middle = Some(state.clone());
        }
        // A wipe-out: the gold left last turn plus the income could not pay.
        if let Some(g) = end_gold[seat] {
            if units_before[seat] > 0 && g + state.income(seat) < 0 {
                wipes[seat][usize::from(turn > 20)] += 1;
            }
        }
        if let Some(other) = pending.take() {
            debug_assert_eq!(other, 1 - seat);
            let best = cuts(&state, 4, state.gold(seat), 1);
            let at = contact[other].as_mut().ok_or("no contact")?;
            if let Some(cut) = best.first() {
                at.cut_cells = cut.cells as f64;
                at.cut_units = cut.units as f64;
            }
            let m = margin(&state);
            at.margin = m.max(-200) as f64;
            at.lethal = f64::from(u8::from(m >= 0));
        }
        let all = &mut totals.entry(names[seat].clone()).or_default();
        for (i, &point) in CHECKPOINTS.iter().enumerate() {
            if turn == point && !seen_checkpoint[seat][i] {
                seen_checkpoint[seat][i] = true;
                let c = &mut all.checkpoints[i];
                c.0 += f64::from(state.active(seat).count());
                c.1 += state.income(seat) as f64;
                c.2 += state.gold(seat) as f64;
                c.3 += 1;
            }
        }
        if contact[seat].is_none() && gap(&state) <= 2 {
            contact[seat] = Some(AtContact {
                turn: f64::from(turn),
                cells: f64::from(state.active(seat).count()),
                income: state.income(seat) as f64,
                gold: state.gold(seat) as f64,
                units: state.unit_count(seat) as f64,
                mines: f64::from((state.mines() & state.owned(seat)).count()),
                voronoi: voronoi2(&state, seat) as f64 / 2.0,
                ..AtContact::default()
            });
            pending = Some(seat);
        }

        let line = answers
            .iter()
            .find(|a| a.seat == seat)
            .and_then(|a| a.lines.first())
            .cloned()
            .unwrap_or_default();
        let commands: Vec<Command> = line.split(';').filter_map(|p| parse(p.trim())).collect();
        for &command in &commands {
            if let Command::Train { level, .. } = command {
                if level > 1 && contact[seat].is_none() {
                    high[seat] += 1;
                }
            }
        }
        // As `play_turn`, but with the gold seen before the end of the turn.
        let mut scratch = state.clone();
        let first_new = scratch.next_unit_id();
        for &command in &commands {
            if let Command::Move { id, .. } = command {
                if id >= first_new {
                    continue;
                }
            }
            scratch.apply(command);
            if scratch.outcome().is_some() {
                break;
            }
        }
        end_gold[seat] = Some(scratch.gold(seat));
        units_before[seat] = scratch.unit_count(seat) as u32;
        state.play_turn(&commands);
    }

    let score = match record.winner {
        Some(0) => [1.0, 0.0],
        Some(_) => [0.0, 1.0],
        None => [0.5, 0.5],
    };
    let last_turn = f64::from(state.game_turn());
    for seat in 0..2 {
        let t = totals.entry(names[seat].clone()).or_default();
        t.games += 1;
        t.score += score[seat];
        t.game_turns += last_turn;
        t.wipes_early += wipes[seat][0];
        t.wipes += wipes[seat][0] + wipes[seat][1];
        t.high_level += high[seat];
        if let Some(at) = contact[seat] {
            t.contacts += 1;
            t.at.add(&at);
        }
        if let Some(mid) = &at_middle {
            let ours = score::terms(mid, seat, Settings::DEFAULT);
            let theirs = score::terms(mid, 1 - seat, Settings::DEFAULT);
            let value = ours.value() - theirs.value();
            t.middles += 1;
            t.middle_score += value as f64;
            t.middle_turn += f64::from(mid.game_turn());
            for (sum, x) in t.ours.iter_mut().zip(term_list(&ours)) {
                *sum += x;
            }
            for (sum, x) in t.theirs.iter_mut().zip(term_list(&theirs)) {
                *sum += x;
            }
            if score[seat] != 0.5 && value != 0 {
                t.decided += 1;
                if (value > 0) == (score[seat] == 1.0) {
                    t.agree += 1;
                }
            }
            let list = |x: &Terms| term_list(x).map(|v| format!("{v}")).join(",");
            let _ = writeln!(
                csv,
                "{},{},{},{},{},{}",
                names[seat],
                score[seat],
                mid.game_turn(),
                value,
                list(&ours),
                list(&theirs)
            );
        }
    }
    Ok(())
}

/// Facts of the maps of seeds 1 to `n` that the opening depends on.
fn maps(n: u64) {
    let (mut cells, mut spots, mut hq_gap, mut half) = (0.0, 0.0, 0.0, 0.0);
    let (mut near_spots, mut nearest_spot) = (0.0, 0.0);
    for seed in 1..=n {
        let map = generate(seed, ice_and_fire_referee::League::Bronze);
        let state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let valid = state.map().valid;
        cells += f64::from(valid.count());
        spots += f64::from(state.map().spots.count());
        hq_gap += f64::from(state.map().distance(HQ[0], HQ[1]));
        half += voronoi2(&state, 0) as f64 / 2.0;
        // Red's mine spots on its side of the map, and the nearest past (1, 0).
        let mine: Vec<u8> = state
            .map()
            .spots
            .iter()
            .filter(|&c| state.map().distance(HQ[0], c) < state.map().distance(HQ[1], c))
            .map(|c| state.map().distance(HQ[0], c))
            .collect();
        near_spots += mine.len() as f64;
        let mut far: Vec<u8> = mine.iter().copied().filter(|&d| d > 1).collect();
        far.sort_unstable();
        nearest_spot += f64::from(far.first().copied().unwrap_or(0));
    }
    let n = n as f64;
    println!("maps {n}: non-void cells {:.1}, mine spots {:.1} ({:.1} on red's side), HQ to HQ {:.1} steps, red's Voronoi half {:.1} cells, red's nearest spot past (1, 0) {:.1} steps from its HQ",
        cells / n, spots / n, near_spots / n, hq_gap / n, half / n, nearest_spot / n);
}

fn main() -> ExitCode {
    let Some(dir) = std::env::args().nth(1) else {
        eprintln!("usage: opening RECORDS_DIR | opening --maps N");
        return ExitCode::from(2);
    };
    if dir == "--maps" {
        maps(
            std::env::args()
                .nth(2)
                .and_then(|n| n.parse().ok())
                .unwrap_or(1000),
        );
        return ExitCode::SUCCESS;
    }
    let mut totals: BTreeMap<String, Totals> = BTreeMap::new();
    let mut files: Vec<_> = match fs::read_dir(Path::new(&dir)) {
        Ok(entries) => entries.filter_map(Result::ok).map(|e| e.path()).collect(),
        Err(err) => {
            eprintln!("opening: cannot read {dir}: {err}");
            return ExitCode::FAILURE;
        }
    };
    files.sort();
    let mut csv = String::from("bot,result,turn,score,voronoi,gold,income,exposure,debt,loss,their_voronoi,their_gold,their_income,their_exposure,their_debt,their_loss\n");
    for file in files
        .iter()
        .filter(|f| f.extension().is_some_and(|e| e == "json"))
    {
        let parsed = fs::read_to_string(file)
            .map_err(|e| e.to_string())
            .and_then(|text| serde_json::from_str::<Record>(&text).map_err(|e| e.to_string()));
        match parsed {
            Ok(record) => {
                if let Err(err) = analyse(&record, &mut totals, &mut csv) {
                    eprintln!("opening: {}: {err}", file.display());
                }
            }
            Err(err) => eprintln!("opening: {}: {err}", file.display()),
        }
    }

    println!(
        "| Bot | Games | Score | Mean game turns | Wipe-outs (turns 1-20, all) | Level 2-3 trainings before contact |"
    );
    println!("| --- | ---: | ---: | ---: | ---: | ---: |");
    for (name, t) in &totals {
        let g = t.games.max(1) as f64;
        println!(
            "| {name} | {} | {:.1}% | {:.1} | {:.2}, {:.2} | {:.2} |",
            t.games,
            100.0 * t.score / g,
            t.game_turns / g,
            t.wipes_early as f64 / g,
            t.wipes as f64 / g,
            t.high_level as f64 / g,
        );
    }
    println!();
    println!(
        "At the bot's first start of turn in contact (means), and its exposure after that turn:"
    );
    println!();
    println!("| Bot | Contacts | Turn | Cells | Income | Gold | Units | Mines | Voronoi cells | Cut: cells | Cut: unit value | Enemy chain margin | Lethal for the enemy |");
    println!("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");
    for (name, t) in &totals {
        let n = t.contacts.max(1) as f64;
        let a = &t.at;
        println!(
            "| {name} | {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.2} | {:.1} | {:.1} | {:.1} | {:.0} | {:.1}% |",
            t.contacts,
            a.turn / n,
            a.cells / n,
            a.income / n,
            a.gold / n,
            a.units / n,
            a.mines / n,
            a.voronoi / n,
            a.cut_cells / n,
            a.cut_units / n,
            a.margin / n,
            100.0 * a.lethal / n,
        );
    }
    println!();
    println!("Cells, income and gold at the start of game turns {CHECKPOINTS:?} (means over the games that reached them):");
    println!();
    let heads: Vec<String> = CHECKPOINTS.iter().map(|t| format!("Turn {t}")).collect();
    println!("| Bot | {} |", heads.join(" | "));
    println!("| --- |{}", " ---: |".repeat(CHECKPOINTS.len()));
    for (name, t) in &totals {
        let cols: Vec<String> = t
            .checkpoints
            .iter()
            .map(|&(c, i, g, n)| {
                let n = n.max(1) as f64;
                format!("{:.1}, {:.1}, {:.1}", c / n, i / n, g / n)
            })
            .collect();
        println!("| {name} | {} |", cols.join(" | "));
    }
    println!();
    println!("At the middle game (means; the score is the bot's value less the opponent's, the terms are the bot's, then the opponent's):");
    println!();
    println!("| Bot | Games | Turn | Score | Sign agrees with the winner | Voronoi | Gold | 3 × income | Exposure | Debt | Loss | Opponent: Voronoi, gold, 3 × income, exposure |");
    println!("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |");
    for (name, t) in &totals {
        let n = t.middles.max(1) as f64;
        let o = t.ours.map(|x| x / n);
        let e = t.theirs.map(|x| x / n);
        println!(
            "| {name} | {} | {:.1} | {:+.1} | {:.1}% | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1}, {:.1}, {:.1}, {:.1} |",
            t.middles,
            t.middle_turn / n,
            t.middle_score / n,
            100.0 * t.agree as f64 / t.decided.max(1) as f64,
            o[0], o[1], o[2], o[3], o[4], o[5], e[0], e[1], e[2], e[3],
        );
    }
    if let Ok(path) = std::env::var("OPENING_CSV") {
        if let Err(err) = fs::write(&path, csv) {
            eprintln!("opening: cannot write {path}: {err}");
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
