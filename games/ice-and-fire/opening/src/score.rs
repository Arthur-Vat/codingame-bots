//! A hand-built score of a position at the end of the opening (E004), from
//! the early game's research (`games/ice-and-fire/research/early-game.md`),
//! never fitted to games. The owner's direction (2026-10-22): "something like
//! Voronoi first, but then take into account economy", with "the time
//! horizon for the score evaluation in the middle game, not long after the
//! early game".
//!
//! The value of a side, in gold, is
//!
//! - its **Voronoi share** × [`CELL`]: the cells nearer its active cells
//!   than the enemy's (steps over non-void cells; its own active cells
//!   included, a tie half), each worth a training, the price of taking a
//!   free cell. It is the 3rd-placed player's first criterion for his
//!   opening search, and the map a side will hold once the opening is over;
//! - its **economy** [`TURNS`] turns into the middle game: gold +
//!   [`TURNS`] × income, the gold it holds then if nothing changes. A few
//!   turns, since the first fights begin at the middle game;
//! - less its **exposure**: what the enemy's best cut on its next turn would
//!   take, with the enemy's gold then (gold, plus its income if it is not to
//!   act): the cells it takes or cuts off × [`CELL`], the value of the units
//!   it kills or cuts off, and [`TURNS`] × 4 per mine; [`LOSS`] if the cut
//!   takes the HQ;
//! - less the value of all its units if its gold plus income is below 0
//!   (**debt**: they die at its next start of turn);
//! - less [`LOSS`] if the enemy can pay a chain of trainings to its HQ
//!   (**loss**), with its gold if it is to act, else its gold plus income;
//!   the exposure then counts 0, so that the loss counts once.
//!
//! The score of a state for a seat is its value less the enemy's.

use ice_and_fire_engine::{Bits, Outcome, State, CELLS, HQ};
use ice_and_fire_tactics::chain::Reach;
use ice_and_fire_tactics::cut::cuts;

/// The worth of a cell of Voronoi share, in gold: a training.
pub const CELL: i64 = 10;
/// Turns into the middle game at which the economy is counted.
pub const TURNS: i64 = 3;
/// The value of a lost game, in gold: more than anything else in the score.
pub const LOSS: i64 = 1_000;
/// The longest cut the exposure looks at, in trainings.
const CUT_LENGTH: usize = 5;
/// The enemy's cuts that [`exposure`] weighs, the cut finder's best first.
const CUTS_KEPT: usize = 8;

/// The score's setting: turns into the middle game at which the economy is
/// counted ([`TURNS`] for the score of E004; others to check that a ranking
/// does not depend on it).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    pub turns: i64,
}

impl Settings {
    /// The score of E004.
    pub const DEFAULT: Settings = Settings { turns: TURNS };
}

/// The terms of a side's value, in gold.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Terms {
    /// Voronoi share × [`CELL`].
    pub voronoi: i64,
    pub gold: i64,
    /// Turns × income.
    pub income: i64,
    pub exposure: i64,
    pub debt: i64,
    pub loss: i64,
}

impl Terms {
    /// The side's value: its map and economy, less what it may lose.
    pub fn value(&self) -> i64 {
        self.voronoi + self.gold + self.income - self.exposure - self.debt - self.loss
    }
}

/// The training cost (10 a level) of the units of `seat` on its active cells.
pub fn unit_value(state: &State, seat: usize) -> i64 {
    (state.units(seat) & state.active(seat))
        .iter()
        .filter_map(|cell| state.unit_level(cell))
        .map(|level| 10 * i64::from(level))
        .sum()
}

/// The score of `state` for `seat`: its value less the enemy's.
pub fn score(state: &State, seat: usize, settings: Settings) -> i64 {
    terms(state, seat, settings).value() - terms(state, 1 - seat, settings).value()
}

/// The terms of the value of `seat` in `state`. A finished game has only the
/// loss term, for the loser.
pub fn terms(state: &State, seat: usize, settings: Settings) -> Terms {
    match state.outcome() {
        Some(Outcome::Win(winner)) if winner != seat => {
            return Terms {
                loss: LOSS,
                ..Terms::default()
            }
        }
        Some(_) => return Terms::default(),
        None => {}
    }
    let gold = state.gold(seat);
    let income = state.income(seat);
    let lost = threatened(state, seat);
    Terms {
        voronoi: CELL * voronoi_halves(state, seat) / 2,
        gold,
        income: settings.turns * income,
        exposure: if lost {
            0
        } else {
            exposure(state, seat, settings)
        },
        debt: if gold + income < 0 {
            unit_value(state, seat)
        } else {
            0
        },
        loss: if lost { LOSS } else { 0 },
    }
}

/// Steps over non-void cells from the cells of `from`; `u8::MAX` where there
/// is no path.
fn steps(state: &State, from: Bits) -> [u8; CELLS] {
    let valid = state.map().valid;
    let mut dist = [u8::MAX; CELLS];
    let mut frontier = from & valid;
    let mut seen = frontier;
    let mut d = 0u8;
    while !frontier.is_empty() {
        for cell in frontier.iter() {
            dist[cell] = d;
        }
        let next = (frontier.neighbours() & valid).and_not(seen);
        seen |= next;
        frontier = next;
        d = d.saturating_add(1);
    }
    dist
}

/// Twice the Voronoi share of `seat`: 2 for each non-void cell nearer its
/// active cells than the enemy's, 1 for a tie; cells neither side reaches
/// count for no one.
pub fn voronoi_halves(state: &State, seat: usize) -> i64 {
    let ours = steps(state, state.active(seat));
    let theirs = steps(state, state.active(1 - seat));
    state
        .map()
        .valid
        .iter()
        .map(|cell| match ours[cell].cmp(&theirs[cell]) {
            _ if ours[cell] == u8::MAX => 0,
            std::cmp::Ordering::Less => 2,
            std::cmp::Ordering::Equal => 1,
            std::cmp::Ordering::Greater => 0,
        })
        .sum()
}

/// The enemy's budget on its next turn: its gold if it is to act, else its
/// gold plus income.
fn enemy_budget(state: &State, enemy: usize) -> i64 {
    state.gold(enemy)
        + if state.to_act() == enemy {
            0
        } else {
            state.income(enemy)
        }
}

/// Whether the enemy of `seat` can pay a chain of trainings to its HQ on its
/// next turn (its units' steps free if it is to act).
pub fn threatened(state: &State, seat: usize) -> bool {
    let enemy = 1 - seat;
    let to_act = state.to_act() == enemy;
    Reach::new(state, enemy, to_act).cost[HQ[seat]] <= enemy_budget(state, enemy)
}

/// What the enemy's best cut on its next turn takes from `victim` (see the
/// module's docs). When the victim is to act, its turn passes first; the
/// enemy's budget is then its gold after its start of turn.
pub fn exposure(state: &State, victim: usize, settings: Settings) -> i64 {
    if state.outcome().is_some() {
        return 0;
    }
    let mut next = state.clone();
    if next.to_act() == victim {
        next.end_turn();
        if next.outcome().is_some() {
            return 0;
        }
    }
    let enemy = 1 - victim;
    let mines = |s: &State| i64::from((s.mines() & s.active(victim)).count());
    let mines_before = mines(&next);
    let mut worst = 0;
    for cut in cuts(&next, CUT_LENGTH, next.gold(enemy), CUTS_KEPT) {
        let mut after = next.clone();
        for &command in &cut.commands {
            after.apply(command);
        }
        let loss = if after.outcome() == Some(Outcome::Win(enemy)) {
            LOSS
        } else {
            CELL * cut.cells + cut.units + 4 * settings.turns * (mines_before - mines(&after))
        };
        worst = worst.max(loss);
    }
    worst
}

#[cfg(test)]
mod tests;
