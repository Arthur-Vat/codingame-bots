//! Helpers of the tests: positions written as CodinGame's text, as in the
//! tactics crate's tests, and the start of the referee's maps.

use ice_and_fire_engine::{Bits, League, State, SIZE};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

/// A Bronze position at game turn 10 as the player in seat 0 (red, to act)
/// sees it.
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
        let mut active = [Bits::EMPTY; 2];
        for (y, row) in self.rows.iter().enumerate() {
            assert_eq!(row.len(), SIZE, "row {y}: {row:?}");
            for (x, symbol) in row.bytes().enumerate() {
                match symbol {
                    b'O' => active[0].set(y * SIZE + x),
                    b'X' => active[1].set(y * SIZE + x),
                    _ => {}
                }
            }
        }
        let mut income = [0i64; 2];
        for (owner, income) in income.iter_mut().enumerate() {
            let at = |x: u8, y: u8| usize::from(y) * SIZE + usize::from(x);
            *income = i64::from(active[owner].count());
            for &(who, kind, x, y) in self.buildings {
                if usize::from(who) == owner && kind == 1 && active[owner].test(at(x, y)) {
                    *income += 4;
                }
            }
            for &(who, _, level, x, y) in self.units {
                if usize::from(who) == owner && active[owner].test(at(x, y)) {
                    *income -= [1, 4, 20][usize::from(level) - 1];
                }
            }
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

/// The start of the game on the referee's Bronze map of `seed`.
pub fn start(seed: u64) -> State {
    let map = generate(seed, RefereeLeague::Bronze);
    State::new(&map.void, &map.mine_spots, League::Bronze)
}
