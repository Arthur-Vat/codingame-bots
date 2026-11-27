//! Approach to the centre (owner, 2026-11-27): where each side's units stand and move, by ring. One CSV row per side and game
//! turn (3 to 12), at the end of that side's ply. A cell's ring is half the
//! difference of its steps to the two HQs: 0 on the line even between them.
use cg_arena::record::Record;
use ice_and_fire_engine::{Bits, Command, League, State, CELLS, HQ, SIZE};
use ice_and_fire_referee::map::generate;
use std::collections::VecDeque;

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
fn bfs(void: Bits, start: usize) -> [i32; CELLS] {
    let mut d = [99; CELLS];
    let mut q = VecDeque::new();
    d[start] = 0;
    q.push_back(start);
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
fn main() {
    println!("game,seat,turn,u_mid,u_band,u_home,c_mid,c_band,c_home,toward,level,away,t_mid,t_band,t_home,units");
    for path in std::env::args().skip(1) {
        let record: Record = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        let game = std::path::Path::new(&path).file_stem().unwrap().to_string_lossy().to_string();
        let map = generate(record.seed, ice_and_fire_referee::League::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let void = state.map().void;
        let (h0, h1) = (bfs(void, HQ[0]), bfs(void, HQ[1]));
        let ring = |c: usize| (h0[c] - h1[c]).abs() / 2;
        let band = |r: i32| if r <= 1 { 0 } else if r <= 3 { 1 } else { 2 };
        for answers in &record.turns {
            if state.outcome().is_some() || state.game_turn() > 12 { break; }
            let s = state.to_act();
            let turn = state.game_turn();
            let line = answers.iter().find(|a| a.seat == s).and_then(|a| a.lines.first()).cloned().unwrap_or_default();
            let first_new = state.next_unit_id();
            let (mut toward, mut level, mut away) = (0, 0, 0);
            let mut tr = [0; 3];
            for part in line.split(';').map(str::trim).filter(|p| !p.is_empty() && !p.starts_with("MSG")) {
                let Some(cmd) = parse(part) else { continue };
                let from = if let Command::Move { id, .. } = cmd {
                    if id >= first_new { continue; }
                    match state.units(s).iter().find(|&c| state.unit_id(c) == Some(id)) { Some(c) => Some((id, c)), None => continue }
                } else { None };
                if !state.apply(cmd) { continue; }
                match cmd {
                    Command::Move { id, .. } => {
                        let (_, f) = from.unwrap();
                        let Some(t) = state.units(s).iter().find(|&c| state.unit_id(c) == Some(id)) else { continue };
                        if t == f { continue; }
                        match ring(t).cmp(&ring(f)) { std::cmp::Ordering::Less => toward += 1, std::cmp::Ordering::Equal => level += 1, _ => away += 1 }
                    }
                    Command::Train { x, y, .. } => tr[band(ring(usize::from(y) * SIZE + usize::from(x)))] += 1,
                    _ => {}
                }
            }
            if turn >= 3 {
                let mut u = [0; 3];
                for c in state.units(s).iter() { u[band(ring(c))] += 1; }
                let mut cc = [0; 3];
                for c in state.active(s).iter() { cc[band(ring(c))] += 1; }
                println!("{game},{s},{turn},{},{},{},{},{},{},{toward},{level},{away},{},{},{},{}", u[0], u[1], u[2], cc[0], cc[1], cc[2], tr[0], tr[1], tr[2], state.units(s).count());
            }
            state.end_turn();
        }
    }
}
