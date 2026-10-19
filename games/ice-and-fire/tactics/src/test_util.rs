//! Helpers of the tests: positions written as CodinGame's text, and random
//! positions from the referee's maps.

use cg_core::rng::Rng;
use ice_and_fire_engine::{Bits, Command, League, State, SIZE};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

/// A Bronze position as the player in seat 0 (red, to act) sees it.
pub struct Position<'a> {
    /// The map, row by row: `#` void, `.` neutral, `O` and `o` the player's
    /// cells (active and inactive), `X` and `x` the opponent's.
    pub rows: [&'a str; 12],
    /// The player's gold, then the opponent's.
    pub gold: [i64; 2],
    /// Buildings other than the HQs: owner (0 the player, 1 the opponent),
    /// type (1 mine, 2 tower), x, y.
    pub buildings: &'a [(u8, u8, u8, u8)],
    /// Units: owner, id, level, x, y.
    pub units: &'a [(u8, u32, u8, u8, u8)],
}

impl Position<'_> {
    /// The state, as a bot reads it. The incomes come from the map.
    pub fn state(&self) -> State {
        let mut owned = [Bits::EMPTY; 2];
        let mut active = [Bits::EMPTY; 2];
        for (y, row) in self.rows.iter().enumerate() {
            assert_eq!(row.len(), SIZE, "row {y}: {row:?}");
            for (x, symbol) in row.bytes().enumerate() {
                let owner = match symbol {
                    b'O' | b'o' => 0,
                    b'X' | b'x' => 1,
                    _ => continue,
                };
                owned[owner].set(y * SIZE + x);
                if symbol.is_ascii_uppercase() {
                    active[owner].set(y * SIZE + x);
                }
            }
        }
        let mut income = [0i64; 2];
        for owner in 0..2u8 {
            let mut total = i64::from(active[usize::from(owner)].count());
            for &(who, kind, x, y) in self.buildings {
                let cell = usize::from(y) * SIZE + usize::from(x);
                if who == owner && kind == 1 && active[usize::from(owner)].test(cell) {
                    total += 4;
                }
            }
            for &(who, _, level, x, y) in self.units {
                let cell = usize::from(y) * SIZE + usize::from(x);
                if who == owner && active[usize::from(owner)].test(cell) {
                    total -= [1, 4, 20][usize::from(level) - 1];
                }
            }
            income[usize::from(owner)] = total;
        }
        let mut turn = format!(
            "{}\n{}\n{}\n{}\n",
            self.gold[0], income[0], self.gold[1], income[1]
        );
        for row in self.rows {
            turn.push_str(row);
            turn.push('\n');
        }
        let mut buildings = vec![(0, 0, 0, 0), (1, 0, 11, 11)];
        buildings.extend_from_slice(self.buildings);
        turn.push_str(&format!("{}\n", buildings.len()));
        for (owner, kind, x, y) in buildings {
            turn.push_str(&format!("{owner} {kind} {x} {y}\n"));
        }
        turn.push_str(&format!("{}\n", self.units.len()));
        for (owner, id, level, x, y) in self.units {
            turn.push_str(&format!("{owner} {id} {level} {x} {y}\n"));
        }
        State::read("0\n", &turn, League::Bronze, 10).expect("a valid position")
    }
}

/// The training cost of the units of `seat` on its active cells.
pub fn unit_value(state: &State, seat: usize) -> i64 {
    (state.units(seat) & state.active(seat))
        .iter()
        .filter_map(|cell| state.unit_level(cell))
        .map(|level| 10 * i64::from(level))
        .sum()
}

/// The league of the referee that is the engine's.
fn referee_league(league: League) -> RefereeLeague {
    match league {
        League::Wood3 => RefereeLeague::Wood3,
        League::Wood2 => RefereeLeague::Wood2,
        League::Wood1 => RefereeLeague::Wood1,
        League::Bronze => RefereeLeague::Bronze,
    }
}

/// The start of a game on the referee's map for `seed`.
pub fn start(seed: u64, league: League) -> State {
    let map = generate(seed, referee_league(league));
    State::new(&map.void, &map.mine_spots, league)
}

/// Positions of the game on `seed`'s map after `plies` random turns, one
/// after each of `plies`, in increasing order, until the game ends.
pub fn random_positions(seed: u64, league: League, plies: &[u32]) -> Vec<State> {
    let mut rng = Rng::new(seed ^ 0x07ac_71c5);
    let mut state = start(seed, league);
    let mut commands = Vec::new();
    let mut positions = Vec::new();
    let mut played = 0;
    for &until in plies {
        while played < until && state.outcome().is_none() {
            state.random_turn(&mut rng, &mut commands);
            state.play_turn(&commands);
            played += 1;
        }
        if state.outcome().is_some() {
            break;
        }
        positions.push(state.clone());
    }
    positions
}

/// [`ready_to_win`] after red takes blue's HQ in two trainings.
pub fn won_state() -> State {
    let mut state = ready_to_win();
    assert!(state.apply(Command::Train {
        level: 1,
        x: 11,
        y: 10
    }));
    assert!(state.apply(Command::Train {
        level: 1,
        x: 11,
        y: 11
    }));
    state
}

/// Red holds the top row and the right column down to (11, 9); blue's HQ is
/// alone at (11, 11), and (11, 10) is neutral. Red has 25 gold: two level 1
/// trainings take the HQ.
pub fn ready_to_win() -> State {
    Position {
        rows: [
            "OOOOOOOOOOOO",
            "###########O",
            "###########O",
            "###########O",
            "###########O",
            "###########O",
            "###########O",
            "###########O",
            "###########O",
            "###########O",
            "###########.",
            "###########X",
        ],
        gold: [25, 20],
        buildings: &[],
        units: &[],
    }
    .state()
}
