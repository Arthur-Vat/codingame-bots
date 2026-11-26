//! `towers`: for each tower of a bank's `played` lines (E090's study of
//! tower placement), why the search would offer it and what it changed.
//! One line a tower:
//!
//! ```text
//! NAME<TAB>PLY<TAB>TURN<TAB>x y<TAB>dist=D<TAB>left=0|1<TAB>guard=0|1<TAB>hq2=0|1<TAB>hq_now=0|1<TAB>threat=W/WO<TAB>cut=W/WO<TAB>best=B@x y
//! ```
//!
//! `dist` is the steps from the tower to the nearest enemy active cell or
//! unit before the turn; `left`, whether a unit moved off its cell earlier
//! in the turn (E070, E076); `guard`, whether the tower lowers the cells
//! the enemy can take at its next turn where it is built (E026); `hq2` and
//! `hq_now`, whether it raises the enemy's chain to our HQ while that chain
//! costs at most the enemy's gold plus two incomes, or its gold alone.
//! `threat` and `cut` compare the whole turn with and without the tower:
//! the cells the enemy can take, and the active cells the enemy's best cut
//! removes at its next turn. `best` is the least best cut over every cell
//! of ours where the tower could stand instead (at the same point of the
//! turn), and the first such cell.

use std::fmt::Write as _;

use ice_and_fire_engine::{Bits, Command, State, HQ, SIZE};
use ice_and_fire_tactics::chain::cost_to;
use ice_and_fire_tactics::cut::cuts;

use super::judge::commands_of;
use super::read_bank;
use crate::eval::threatened;

/// The active cells the enemy's best cut removes once `state`'s turn ends.
fn best_cut(state: &State) -> i64 {
    let mut next = state.clone();
    next.end_turn();
    let gold = next.gold(next.to_act());
    cuts(&next, 5, gold, 3)
        .iter()
        .map(|c| c.cells)
        .max()
        .unwrap_or(0)
        .max(0)
}

/// `state` after `commands`, skipping the one at `skip`.
fn after(state: &State, commands: &[Command], skip: Option<usize>) -> State {
    let mut s = state.clone();
    for (i, &c) in commands.iter().enumerate() {
        if Some(i) != skip {
            s.apply(c);
        }
    }
    s
}

pub fn run(args: &[String]) -> Result<String, String> {
    let path = args.first().ok_or("towers BANK")?;
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let mut out = String::new();
    for position in read_bank(&text)? {
        let Some(played) = &position.played else {
            continue;
        };
        let Ok(start) = position.state() else {
            continue;
        };
        let me = start.to_act();
        let enemy = 1 - me;
        let commands = commands_of(played);
        let theirs = start.active(enemy) | start.units(enemy);
        let map = start.map();
        let mut state = start.clone();
        let mut left = Bits::EMPTY;
        for (i, &command) in commands.iter().enumerate() {
            if let Command::BuildTower { x, y } = command {
                let cell = usize::from(y) * SIZE + usize::from(x);
                let dist = theirs
                    .iter()
                    .map(|c| map.distance(cell, c))
                    .min()
                    .unwrap_or(u8::MAX);
                let threats = threatened(&state, me);
                let chain = cost_to(&state, enemy, false, HQ[me]);
                let mut built = state.clone();
                let ok = built.apply(command);
                let guard = ok && threatened(&built, me) < threats;
                let raised = ok && cost_to(&built, enemy, false, HQ[me]) > chain;
                let hq2 = raised && chain <= state.gold(enemy) + 2 * state.income(enemy).max(0);
                let hq_now = raised && chain <= state.gold(enemy);
                let with = after(&start, &commands, None);
                let rest = &commands[i + 1..];
                let mut best = (i64::MAX, cell);
                let free = state.owned(me) & state.empty_cells();
                for other in free.and_not(map.spots).iter() {
                    let mut alt = state.clone();
                    let (ox, oy) = ((other % SIZE) as u8, (other / SIZE) as u8);
                    if !alt.apply(Command::BuildTower { x: ox, y: oy }) {
                        continue;
                    }
                    for &c in rest {
                        alt.apply(c);
                    }
                    let b = best_cut(&alt);
                    if b < best.0 {
                        best = (b, other);
                    }
                }
                let without = after(&start, &commands, Some(i));
                let _ = writeln!(
                    out,
                    "{}\t{}\t{}\t{x} {y}\t{}\t{}\t{}\t{}\t{}\t{}/{}\t{}/{}\t{}@{} {}",
                    position.name,
                    position.ply,
                    position.game_turn,
                    dist,
                    u8::from(left.test(cell)),
                    u8::from(guard),
                    u8::from(hq2),
                    u8::from(hq_now),
                    threatened(&with, me),
                    threatened(&without, me),
                    best_cut(&with),
                    best_cut(&without),
                    best.0,
                    best.1 % SIZE,
                    best.1 / SIZE,
                );
            }
            if let Command::Move { id, .. } = command {
                if let Some(from) = state
                    .units(me)
                    .iter()
                    .find(|&c| state.unit_id(c) == Some(id))
                {
                    left.set(from);
                }
            }
            state.apply(command);
        }
    }
    Ok(out)
}
