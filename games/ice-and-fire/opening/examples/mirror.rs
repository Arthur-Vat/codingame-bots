//! Does the opening favour one flank by its orientation (E082)? On each referee map and on the same map mirrored across the
//! HQs' diagonal (x and y swapped), both sides play the bot's opening
//! (v032's settings, untimed) for 8 turns; we count red's active cells in
//! the column flank (x − y ≤ −4: straight down from red's HQ) and in the
//! row flank (x − y ≥ 4). An unbiased opening takes, on the mirror, the
//! mirror of what it took on the map. The opening is the plan bot's with
//! `econ::BALANCED_TIES` as compiled.
//!
//! ```sh
//! cargo run --release -p ice-and-fire-opening --example mirror -- [MAPS]
//! ```

use ice_and_fire_engine::{League, State, CELLS, SIZE};
use ice_and_fire_opening::econ::Params;
use ice_and_fire_opening::planned::{Planned, Planner};
use ice_and_fire_opening::solo::{self, Decay, Objective, Order};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

const OBJECTIVE: Objective = Objective {
    cells: 3,
    income: 1,
    gold: 1,
    cut: 2,
    joints: 2,
    tips: 2,
    reach: 6,
    voronoi: 1,
    chokes: 0,
    assign: false,
    roles: None,
    order: Order::Central(Decay::Linear(7)),
};

fn opening() -> Planned {
    let sets = solo::SETS
        .iter()
        .map(|&p| Params {
            safe_spot: u8::MAX,
            crowd: 2,
            horizon: 18,
            after_contact: 0,
            ..p
        })
        .collect();
    Planned::new(
        Planner::Solo {
            turns: 9,
            width: 8,
            objective: OBJECTIVE,
            replan_from: 0,
        },
        sets,
        Params::V003,
    )
}

/// Red's active cells in the column flank and in the row flank after 8
/// turns of both openings on the map `void`, `spots`.
fn play(void: &[bool; CELLS], spots: &[(usize, usize)]) -> (i64, i64) {
    let mut state = State::new(void, spots, League::Bronze);
    let mut openings = [opening(), opening()];
    while state.game_turn() <= 8 && state.outcome().is_none() {
        let seat = state.to_act();
        let commands = openings[seat].turn(&state, None);
        state.play_turn(&commands);
    }
    let (mut col, mut row) = (0, 0);
    for c in state.active(0).iter() {
        let (x, y) = ((c % SIZE) as i64, (c / SIZE) as i64);
        if x - y <= -4 {
            col += 1;
        } else if x - y >= 4 {
            row += 1;
        }
    }
    (col, row)
}

fn main() {
    let n: u64 = std::env::args()
        .nth(1)
        .and_then(|a| a.parse().ok())
        .unwrap_or(100);
    let (mut a, mut b, mut c, mut d) = (0i64, 0i64, 0i64, 0i64);
    let mut differ = 0;
    for seed in 1..=n {
        let map = generate(seed, RefereeLeague::Bronze);
        let mut t = [false; CELLS];
        for (i, v) in t.iter_mut().enumerate() {
            let (x, y) = (i % SIZE, i / SIZE);
            *v = map.void[x * SIZE + y];
        }
        let mut ts: Vec<(usize, usize)> = map.mine_spots.iter().map(|&(x, y)| (y, x)).collect();
        ts.sort_by_key(|&(x, y)| (y, x));
        let (col, row) = play(&map.void, &map.mine_spots);
        let (tcol, trow) = play(&t, &ts);
        // On the mirror, the column flank is the map's row flank.
        a += col;
        b += row;
        c += tcol;
        d += trow;
        if (col, row) != (trow, tcol) {
            differ += 1;
        }
    }
    println!(
        "{n} maps: red's column flank {a}, row flank {b}; on the mirrors column {c}, row {d}; maps whose mirror is not mirrored: {differ}"
    );
    println!(
        "column flank over both: {} cells, row flank {} cells",
        a + c,
        b + d
    );
}
