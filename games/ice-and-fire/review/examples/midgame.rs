//! Middle-game measure (owner, 2026-11-27): how each side uses its units and takes its free land in the
//! middle game. One CSV row per side and turn (game turns 6 to 30), at that
//! side's ply. For a cell c at the start of the ply of side s:
//! - dE(c): steps (over non-void cells) to the nearest enemy-owned cell;
//! - a free cell of s: neutral and nearer s's land than the enemy's; near
//!   when dE <= NEAR, far beyond.
use cg_arena::record::Record;
use ice_and_fire_engine::{Bits, Command, League, State, CELLS, SIZE};
use ice_and_fire_referee::map::generate;
use std::collections::{HashMap, HashSet, VecDeque};

const FROM: u32 = 6;
const UNTIL: u32 = 30;
const NEAR: i32 = 4;

fn parse(part: &str) -> Option<Command> {
    let w: Vec<&str> = part.split_whitespace().collect();
    let n = |i: usize| w.get(i).and_then(|s| s.parse::<u32>().ok());
    let b = |i: usize| n(i).and_then(|v| u8::try_from(v).ok());
    match *w.first()? {
        "MOVE" => Some(Command::Move { id: n(1)?, x: b(2)?, y: b(3)? }),
        "TRAIN" => Some(Command::Train { level: b(1)?, x: b(2)?, y: b(3)? }),
        "BUILD" => match *w.get(1)? {
            "MINE" => Some(Command::BuildMine { x: b(2)?, y: b(3)? }),
            "TOWER" => Some(Command::BuildTower { x: b(2)?, y: b(3)? }),
            _ => None,
        },
        "WAIT" => Some(Command::Wait),
        _ => None,
    }
}

fn neighbours(c: usize) -> impl Iterator<Item = usize> {
    let (x, y) = ((c % SIZE) as i32, (c / SIZE) as i32);
    [(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter().filter_map(move |(dx, dy)| {
        let (nx, ny) = (x + dx, y + dy);
        (nx >= 0 && ny >= 0 && nx < SIZE as i32 && ny < SIZE as i32).then(|| ny as usize * SIZE + nx as usize)
    })
}

fn bfs(void: Bits, from: Bits) -> [i32; CELLS] {
    let mut d = [99; CELLS];
    let mut q = VecDeque::new();
    for c in from.iter() {
        d[c] = 0;
        q.push_back(c);
    }
    while let Some(c) = q.pop_front() {
        for n in neighbours(c) {
            if !void.test(n) && d[n] > d[c] + 1 {
                d[n] = d[c] + 1;
                q.push_back(n);
            }
        }
    }
    d
}

fn unit_cell(s: &State, seat: usize, id: u32) -> Option<usize> {
    s.units(seat).iter().find(|&c| s.unit_id(c) == Some(id))
}

fn main() {
    println!("game,seat,turn,units,stuck,adj,front_units,far_units,free_near,free_far,free_next_to_units,moves,cap_near,cap_far,cap_enemy,inside,back,revisit,idle,train_near,train_far,train_enemy,cells,mines,gold");
    for path in std::env::args().skip(1) {
        let record: Record = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        let game = std::path::Path::new(&path).file_stem().unwrap().to_string_lossy().to_string();
        let map = generate(record.seed, ice_and_fire_referee::League::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let void = state.map().void;
        let mut visited: HashMap<(usize, u32), HashSet<usize>> = HashMap::new();
        for answers in &record.turns {
            if state.outcome().is_some() || state.game_turn() > UNTIL {
                break;
            }
            let s = state.to_act();
            let o = 1 - s;
            let turn = state.game_turn();
            let line = answers.iter().find(|a| a.seat == s).and_then(|a| a.lines.first()).cloned().unwrap_or_default();
            // every unit's current cell counts as visited
            for c in state.units(s).iter() {
                if let Some(id) = state.unit_id(c) {
                    visited.entry((s, id)).or_default().insert(c);
                }
            }
            let measure = turn >= FROM;
            let de = bfs(void, state.owned(o));
            let du = bfs(void, state.owned(s));
            let neutral = |st: &State, c: usize| !void.test(c) && !st.owned(0).test(c) && !st.owned(1).test(c);
            let units = state.units(s);
            let (mut stuck, mut adj, mut front_u, mut far_u) = (0, 0, 0, 0);
            let mut next_free: HashSet<usize> = HashSet::new();
            for c in units.iter() {
                let mut open = false;
                for n in neighbours(c) {
                    if neutral(&state, n) {
                        open = true;
                        next_free.insert(n);
                    }
                    if state.owned(o).test(n) {
                        open = true;
                    }
                    if units.test(n) {
                        adj += 1;
                    }
                }
                if !open { stuck += 1; }
                if de[c] <= 2 { front_u += 1; }
                if de[c] > NEAR { far_u += 1; }
            }
            let (mut free_near, mut free_far) = (0, 0);
            for c in 0..CELLS {
                if neutral(&state, c) && du[c] < de[c] {
                    if de[c] <= NEAR { free_near += 1 } else { free_far += 1 }
                }
            }
            let start_ids: Vec<u32> = units.iter().filter_map(|c| state.unit_id(c)).collect();
            let mut moved: HashSet<u32> = HashSet::new();
            let first_new = state.next_unit_id();
            let (mut moves, mut cap_near, mut cap_far, mut cap_enemy, mut inside, mut back, mut revisit) = (0, 0, 0, 0, 0, 0, 0);
            let (mut t_near, mut t_far, mut t_enemy) = (0, 0, 0);
            for part in line.split(';').map(str::trim).filter(|p| !p.is_empty() && !p.starts_with("MSG")) {
                let Some(cmd) = parse(part) else { continue };
                let (before_s, before_o) = (state.owned(s), state.owned(o));
                let src = match cmd {
                    Command::Move { id, .. } => {
                        if id >= first_new { continue; }
                        match unit_cell(&state, s, id) { Some(c) => Some((id, c)), None => continue }
                    }
                    _ => None,
                };
                if !state.apply(cmd) { continue; }
                match cmd {
                    Command::Move { id, .. } => {
                        let Some((_, from)) = src else { continue };
                        let Some(to) = unit_cell(&state, s, id) else { continue };
                        if to == from { continue; }
                        moved.insert(id);
                        moves += 1;
                        let seen = visited.entry((s, id)).or_default();
                        if seen.contains(&to) { revisit += 1; }
                        seen.insert(to);
                        if before_o.test(to) {
                            cap_enemy += 1;
                        } else if !before_s.test(to) {
                            if de[to] <= NEAR { cap_near += 1 } else { cap_far += 1 }
                        } else {
                            inside += 1;
                            if de[to] > de[from] { back += 1; }
                        }
                    }
                    Command::Train { x, y, .. } => {
                        let c = usize::from(y) * SIZE + usize::from(x);
                        if before_o.test(c) { t_enemy += 1 } else if !before_s.test(c) {
                            if de[c] <= NEAR { t_near += 1 } else { t_far += 1 }
                        }
                    }
                    _ => {}
                }
            }
            let idle = start_ids.iter().filter(|id| !moved.contains(id)).count();
            if measure {
                println!(
                    "{game},{s},{turn},{},{stuck},{adj},{front_u},{far_u},{free_near},{free_far},{},{moves},{cap_near},{cap_far},{cap_enemy},{inside},{back},{revisit},{idle},{t_near},{t_far},{t_enemy},{},{},{}",
                    units.count(), next_free.len(), state.active(s).count(), (state.mines() & state.active(s)).count(), state.gold(s)
                );
            }
            state.end_turn();
        }
    }
}
