//! Early growth measure (owner, 2026-11-27): where each side's new cells go in the first
//! turns. One CSV row per side and turn, at the end of that side's ply.
//!
//! Distances are steps over non-void cells. For a cell c at the start of the
//! ply of side s (o the other side):
//! - axis(c) = dist(c, HQ of s) - dist(c, HQ of o): negative on s's half,
//!   0 on the middle line;
//! - margin(c) = dist(c, land of o) - dist(c, land of s): how much nearer s
//!   is than o; large = deep on s's side (land s would get anyway), small =
//!   contested.
use cg_arena::record::Record;
use ice_and_fire_engine::{Bits, Command, League, State, CELLS, HQ, SIZE};
use ice_and_fire_referee::map::generate;
use std::collections::VecDeque;

const TURNS: u32 = 12;
const RESERVE_TURN: u32 = 8;

fn parse(part: &str) -> Option<Command> {
    let w: Vec<&str> = part.split_whitespace().collect();
    let n = |i: usize| w.get(i).and_then(|s| s.parse::<u32>().ok());
    let b = |i: usize| n(i).and_then(|v| u8::try_from(v).ok());
    match *w.first()? {
        "MOVE" => Some(Command::Move {
            id: n(1)?,
            x: b(2)?,
            y: b(3)?,
        }),
        "TRAIN" => Some(Command::Train {
            level: b(1)?,
            x: b(2)?,
            y: b(3)?,
        }),
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
    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .filter_map(move |(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            (nx >= 0 && ny >= 0 && nx < SIZE as i32 && ny < SIZE as i32)
                .then(|| ny as usize * SIZE + nx as usize)
        })
}

fn bfs(void: Bits, from: impl Iterator<Item = usize>) -> [i32; CELLS] {
    let mut d = [i32::MAX / 2; CELLS];
    let mut q = VecDeque::new();
    for c in from {
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

/// Sector of a cell across the HQ-to-HQ diagonal: 0 the flank of (11, 0),
/// 1 the middle, 2 the flank of (0, 11).
fn sector(c: usize) -> usize {
    let lat = (c % SIZE) as i32 - (c / SIZE) as i32;
    if lat >= 4 {
        0
    } else if lat <= -4 {
        2
    } else {
        1
    }
}

/// Per sector: cells nearer s's land than o's, and nearer either side.
fn voronoi_sectors(s: &State, void: Bits, seat: usize) -> [(u32, u32); 3] {
    let a = bfs(void, s.owned(seat).iter());
    let b = bfs(void, s.owned(1 - seat).iter());
    let mut out = [(0, 0); 3];
    for c in 0..CELLS {
        if void.test(c) || a[c] >= i32::MAX / 2 {
            continue;
        }
        let k = sector(c);
        if a[c] < b[c] {
            out[k].0 += 1;
        }
        if a[c] != b[c] {
            out[k].1 += 1;
        }
    }
    out
}

/// Cells nearer s's land than o's, and nearer either side.
fn voronoi(s: &State, void: Bits, seat: usize) -> (u32, u32) {
    let a = bfs(void, s.owned(seat).iter());
    let b = bfs(void, s.owned(1 - seat).iter());
    let (mut mine, mut any) = (0, 0);
    for c in 0..CELLS {
        if void.test(c) || a[c] >= i32::MAX / 2 {
            continue;
        }
        if a[c] < b[c] {
            mine += 1;
        }
        if a[c] != b[c] {
            any += 1;
        }
    }
    (mine, any)
}

fn unit_cell(s: &State, seat: usize, id: u32) -> Option<usize> {
    s.units(seat).iter().find(|&c| s.unit_id(c) == Some(id))
}

fn main() {
    println!("game,seat,turn,new_neutral,new_enemy,axis_sum,margin_sum,contest_new,deep_new,mid_new,move_caps,train_caps,backfill_moves,free_unused,units_start,vor_mine,vor_any,owned,max_axis,lat_span,v0m,v0a,v1m,v1a,v2m,v2a,ft,fm,mt,mm,uf,ut,o0,o1,o2");
    for path in std::env::args().skip(1) {
        let record: Record =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        let game = std::path::Path::new(&path)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let map = generate(record.seed, ice_and_fire_referee::League::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let void = state.map().void;
        let hq = [
            bfs(void, std::iter::once(HQ[0])),
            bfs(void, std::iter::once(HQ[1])),
        ];
        let mut reserve: [Option<Bits>; 2] = [None, None];
        for answers in &record.turns {
            if state.outcome().is_none() && state.game_turn() > TURNS {
                // the empty cells each side had nearer to it at RESERVE_TURN,
                // and who owns them now (both sides have played TURNS turns)
                for (seat, r) in reserve.iter().enumerate() {
                    if let Some(r) = r {
                        let (mut own, mut lost) = (0, 0);
                        for c in r.iter() {
                            if state.owned(seat).test(c) {
                                own += 1;
                            }
                            if state.owned(1 - seat).test(c) {
                                lost += 1;
                            }
                        }
                        eprintln!("{game},{seat},{},{own},{lost}", r.count());
                    }
                }
            }
            if state.outcome().is_some() || state.game_turn() > TURNS {
                break;
            }
            let s = state.to_act();
            let o = 1 - s;
            let turn = state.game_turn();
            let d_s = bfs(void, state.owned(s).iter());
            let d_o = bfs(void, state.owned(o).iter());
            let axis = |c: usize| hq[s][c] - hq[o][c];
            let margin = |c: usize| d_o[c].min(99) - d_s[c].min(99);
            let units_start = state.units(s).count();
            // units that start next to a free (neutral) cell
            let free_next: Vec<(u32, usize)> = state
                .units(s)
                .iter()
                .filter(|&c| {
                    neighbours(c).any(|n| {
                        !void.test(n) && !state.owned(0).test(n) && !state.owned(1).test(n)
                    })
                })
                .filter_map(|c| state.unit_id(c).map(|id| (id, c)))
                .collect();
            let mut captured_by: Vec<u32> = Vec::new();
            // neutral captures on a flank / in the middle, by training / by move
            let (mut ft, mut fm, mut mt, mut mm) = (0, 0, 0, 0);
            let (
                mut nn,
                mut ne,
                mut axs,
                mut mgs,
                mut contest,
                mut deep,
                mut mid,
                mut mc,
                mut tc,
                mut bf,
            ) = (0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
            let line = answers
                .iter()
                .find(|a| a.seat == s)
                .and_then(|a| a.lines.first())
                .cloned()
                .unwrap_or_default();
            let first_new = state.next_unit_id();
            for part in line
                .split(';')
                .map(str::trim)
                .filter(|p| !p.is_empty() && !p.starts_with("MSG"))
            {
                let Some(cmd) = parse(part) else { continue };
                if let Command::Move { id, .. } = cmd {
                    if id >= first_new || unit_cell(&state, s, id).is_none() {
                        continue;
                    }
                }
                let before_s = state.owned(s);
                let before_o = state.owned(o);
                let src = if let Command::Move { id, .. } = cmd {
                    unit_cell(&state, s, id)
                } else {
                    None
                };
                if !state.apply(cmd) {
                    continue;
                }
                let new = state.owned(s).and_not(before_s);
                for c in new.iter() {
                    if before_o.test(c) {
                        ne += 1;
                        continue;
                    }
                    nn += 1;
                    axs += axis(c);
                    let m = margin(c);
                    mgs += m;
                    if m <= 2 {
                        contest += 1;
                    }
                    if m >= 5 {
                        deep += 1;
                    }
                    if axis(c).abs() <= 1 {
                        mid += 1;
                    }
                    let flank = sector(c) != 1;
                    match cmd {
                        Command::Train { .. } => {
                            if flank {
                                ft += 1
                            } else {
                                mt += 1
                            }
                        }
                        Command::Move { .. } => {
                            if flank {
                                fm += 1
                            } else {
                                mm += 1
                            }
                        }
                        _ => {}
                    }
                    match cmd {
                        Command::Move { id, .. } => {
                            mc += 1;
                            captured_by.push(id);
                            // a free neighbour of the source at least 2 nearer the contest
                            if let Some(src) = src {
                                let best = neighbours(src)
                                    .filter(|&n| {
                                        !void.test(n) && !before_s.test(n) && !before_o.test(n)
                                    })
                                    .map(margin)
                                    .min();
                                if best.is_some_and(|b| m - b >= 2) {
                                    bf += 1;
                                }
                            }
                        }
                        Command::Train { .. } => tc += 1,
                        _ => {}
                    }
                }
            }
            let free_unused = free_next
                .iter()
                .filter(|(id, _)| !captured_by.contains(id))
                .count();
            let (vm, va) = voronoi(&state, void, s);
            if turn == RESERVE_TURN {
                let a = bfs(void, state.owned(s).iter());
                let b = bfs(void, state.owned(o).iter());
                let mut r = Bits::default();
                for c in 0..CELLS {
                    if !void.test(c)
                        && !state.owned(0).test(c)
                        && !state.owned(1).test(c)
                        && a[c] < b[c]
                    {
                        r.set(c);
                    }
                }
                reserve[s] = Some(r);
            }
            let max_axis = state.owned(s).iter().map(axis).max().unwrap_or(0);
            let lats: Vec<i32> = state
                .owned(s)
                .iter()
                .map(|c| (c % SIZE) as i32 - (c / SIZE) as i32)
                .collect();
            let span = lats.iter().max().unwrap_or(&0) - lats.iter().min().unwrap_or(&0);
            let v = voronoi_sectors(&state, void, s);
            let uf = state.units(s).iter().filter(|&c| sector(c) != 1).count();
            let ut = state.units(s).count();
            let mut os = [0; 3];
            for c in state.owned(s).iter() {
                os[sector(c)] += 1;
            }
            println!(
                "{game},{s},{turn},{nn},{ne},{axs},{mgs},{contest},{deep},{mid},{mc},{tc},{bf},{free_unused},{units_start},{vm},{va},{},{max_axis},{span},{},{},{},{},{},{},{ft},{fm},{mt},{mm},{uf},{ut},{},{},{}",
                state.owned(s).count(), v[0].0, v[0].1, v[1].0, v[1].1, v[2].0, v[2].1, os[0], os[1], os[2]
            );
            state.end_turn();
        }
    }
}
