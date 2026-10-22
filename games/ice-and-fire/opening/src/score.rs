//! A hand-built score of a state for comparing openings at the middle game
//! (E004). It is never fitted to games: each term is a count of gold that a
//! side has, will earn or may lose by the horizon, game turn [`HORIZON`].
//!
//! With `L` = max(1, horizon − game turn) turns left, the value of a side is
//!
//! - its **gold**;
//! - **L × its income**;
//! - the **territory** it can still take: each free cell nearer its active
//!   cells than the enemy's (through free cells; a tie counts half), sorted
//!   by its distance `d`; the `k`-th cell is taken at turn `τ = max(d, k)`
//!   and then earns `L − τ`, if positive. One cell a turn, whatever the
//!   units: from the middle game on the units fight, and the plan approved
//!   for E004 (`n` units taking `n` cells a turn, `τ = max(d, ceil(k / n))`)
//!   failed its check against games: it ranked E003's `fill` above v003's
//!   `econ` by 42 gold at the middle game of 400 games that `fill` lost
//!   (−16 Elo), its territory term tracking the result least of all terms
//!   (`games/ice-and-fire/research/early-game.md`, "The middle-game score");
//! - less its **exposure**: what the enemy could take from it by banking for
//!   1 to [`Settings::lookahead`] of its turns (see [`exposure`]);
//! - less the value of all its units if its gold plus income is below 0
//!   (**debt**: they die at its next start of turn);
//! - less [`LOSS`] if the enemy can pay a chain of trainings to its HQ
//!   (**loss**), with its gold if it is to act, else its gold plus income.
//!
//! The score of a state for a seat is its value less the enemy's.

use ice_and_fire_engine::state::BUILD_COST;
use ice_and_fire_engine::{Bits, Command, Outcome, State, CELLS, HQ, SIZE};
use ice_and_fire_tactics::chain::Reach;
use ice_and_fire_tactics::cut::cuts;

/// The game turn at which the score counts gold, fixed in advance (E004).
pub const HORIZON: i64 = 20;
/// Enemy turns of banking that [`exposure`] looks at in a full score.
pub const LOOKAHEAD: u32 = 3;
/// The value of a lost game, in gold: more than a side can gather by the
/// horizon.
pub const LOSS: i64 = 1_000;
/// Trainings of the enemy's cuts after one turn of banking; one more for each
/// further turn.
const CUT_LENGTH: usize = 4;
/// The enemy's cuts that [`exposure`] weighs, the cut finder's best first.
const CUTS_KEPT: usize = 8;

/// The score's two settings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    /// The game turn by which gold is counted.
    pub horizon: i64,
    /// Most enemy turns of banking in the exposure; 0 leaves it out.
    pub lookahead: u32,
}

impl Settings {
    /// The score that ranks openings: banking up to 3 turns.
    pub const FULL: Settings = Settings {
        horizon: HORIZON,
        lookahead: LOOKAHEAD,
    };
    /// The cheaper score of searches: one turn of banking.
    pub const SEARCH: Settings = Settings {
        horizon: HORIZON,
        lookahead: 1,
    };
}

/// The terms of a side's value, in gold.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Terms {
    pub gold: i64,
    /// L × income.
    pub income: i64,
    pub territory: i64,
    pub exposure: i64,
    pub debt: i64,
    pub loss: i64,
}

impl Terms {
    /// The side's value: what it has and will earn, less what it may lose.
    pub fn value(&self) -> i64 {
        self.gold + self.income + self.territory - self.exposure - self.debt - self.loss
    }
}

/// Turns left until the horizon, at least 1.
pub fn turns_left(state: &State, horizon: i64) -> i64 {
    (horizon - i64::from(state.game_turn())).max(1)
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
    let left = turns_left(state, settings.horizon);
    let gold = state.gold(seat);
    let income = state.income(seat);
    Terms {
        gold,
        income: left * income,
        territory: territory(state, seat, left),
        exposure: exposure(state, seat, settings),
        debt: if gold + income < 0 {
            unit_value(state, seat)
        } else {
            0
        },
        loss: if threatened(state, seat) { LOSS } else { 0 },
    }
}

/// Steps from the cells of `from` through the cells of `through`;
/// `u8::MAX` where there is no such path.
fn steps(from: Bits, through: Bits) -> [u8; CELLS] {
    let mut dist = [u8::MAX; CELLS];
    let mut frontier = from;
    let mut seen = from;
    let mut d = 0u8;
    while !frontier.is_empty() {
        for cell in frontier.iter() {
            dist[cell] = d;
        }
        let next = (frontier.neighbours() & through).and_not(seen);
        seen |= next;
        frontier = next;
        d = d.saturating_add(1);
    }
    dist
}

/// The territory term of `seat` with `left` turns to the horizon (see the
/// module's docs): its free cells, sorted by distance `d`, the `k`-th taken
/// on turn `max(d, k)` and then earning `left − max(d, k)`, a tie half.
pub fn territory(state: &State, seat: usize, left: i64) -> i64 {
    let free = state.map().valid.and_not(state.owned(0) | state.owned(1));
    let ours = steps(state.active(seat), free);
    let theirs = steps(state.active(1 - seat), free);
    // (distance, weight in halves)
    let mut cells: Vec<(u8, i64)> = Vec::new();
    for cell in free.iter() {
        let (d, e) = (ours[cell], theirs[cell]);
        if d == u8::MAX {
            continue;
        }
        if d < e {
            cells.push((d, 2));
        } else if d == e {
            cells.push((d, 1));
        }
    }
    cells.sort_unstable();
    let mut halves = 0;
    for (index, &(d, weight)) in cells.iter().enumerate() {
        let k = index as i64 + 1;
        halves += weight * (left - i64::from(d).max(k)).max(0);
    }
    halves / 2
}

/// Whether the enemy of `seat` can pay a chain of trainings to its HQ: with
/// its gold if it is to act (its units' steps free), else with its gold plus
/// income.
pub fn threatened(state: &State, seat: usize) -> bool {
    let enemy = 1 - seat;
    let to_act = state.to_act() == enemy;
    let budget = state.gold(enemy) + if to_act { 0 } else { state.income(enemy) };
    Reach::new(state, enemy, to_act).cost[HQ[seat]] <= budget
}

/// What the enemy of `victim` could take from it after banking.
///
/// From `state`, both sides pass; for the enemy's `j`-th turn from now (`j`
/// from 1 to the lookahead) the enemy plays its worst cut for the victim:
/// up to 4 + `j` trainings within its gold, from the tactics' cut finder,
/// which prices standing towers. Before it, on its last turn, the victim may
/// build up to `j` towers within its gold, each where it most lowers the
/// loss, greedily, against the cells of the enemy's worst cut. The loss is
/// the cells the cut takes or cuts off × (L − j) + the value of the units it
/// kills or cuts off + 4 (L − j) for each mine + 15 for each tower built;
/// [`LOSS`] if the cut takes the HQ. The exposure is the largest loss over
/// `j`, at least 0.
pub fn exposure(state: &State, victim: usize, settings: Settings) -> i64 {
    if state.outcome().is_some() {
        return 0;
    }
    let left = turns_left(state, settings.horizon);
    let enemy = 1 - victim;
    let mut worst = 0;
    let mut sim = state.clone();
    // The victim's last turn before the enemy's next one, if any.
    let mut before: Option<State> = None;
    let mut j = 0;
    while j < settings.lookahead && sim.outcome().is_none() {
        if sim.to_act() == enemy {
            j += 1;
            let loss = match before.take() {
                Some(turn) => defended(&turn, j, left),
                None => attack(&sim, j, left).0,
            };
            worst = worst.max(loss);
        } else {
            before = Some(sim.clone());
        }
        sim.end_turn();
    }
    worst
}

/// The loss of the victim, to act in `turn`, to the enemy's worst cut on its
/// `j`-th turn, after up to `j` towers built in `turn` greedily.
fn defended(turn: &State, j: u32, left: i64) -> i64 {
    let victim = turn.to_act();
    let mut current = turn.clone();
    let mut next = current.clone();
    next.end_turn();
    let (mut loss, mut cells) = attack(&next, j, left);
    let mut towers = 0;
    while towers < i64::from(j) && loss > 0 {
        let near = cells | cells.neighbours();
        let options =
            (near & current.active(victim) & current.empty_cells()).and_not(current.map().spots);
        let mut best: Option<(i64, Bits, State)> = None;
        for cell in options.iter() {
            let mut trial = current.clone();
            let command = Command::BuildTower {
                x: (cell % SIZE) as u8,
                y: (cell / SIZE) as u8,
            };
            if !trial.apply(command) {
                continue;
            }
            let mut next = trial.clone();
            next.end_turn();
            let (after, cut) = attack(&next, j, left);
            if best.as_ref().is_none_or(|(top, _, _)| after < *top) {
                best = Some((after, cut, trial));
            }
        }
        match best {
            Some((after, cut, trial)) if after + BUILD_COST < loss => {
                loss = after;
                cells = cut;
                current = trial;
                towers += 1;
            }
            _ => break,
        }
    }
    loss + BUILD_COST * towers
}

/// The enemy's worst cut for the victim in `state`, the enemy to act on its
/// `j`-th turn of the exposure: the loss and the cells it trains on.
fn attack(state: &State, j: u32, left: i64) -> (i64, Bits) {
    let enemy = state.to_act();
    let victim = 1 - enemy;
    let weight = (left - i64::from(j)).max(0);
    let mines = |s: &State| i64::from((s.mines() & s.active(victim)).count());
    let mines_before = mines(state);
    let mut worst = (0, Bits::EMPTY);
    let length = CUT_LENGTH + j as usize;
    for cut in cuts(state, length, state.gold(enemy), CUTS_KEPT) {
        let mut after = state.clone();
        let mut cells = Bits::EMPTY;
        for &command in &cut.commands {
            after.apply(command);
            if let Command::Train { x, y, .. } = command {
                cells.set(usize::from(y) * SIZE + usize::from(x));
            }
        }
        let loss = if after.outcome() == Some(Outcome::Win(enemy)) {
            LOSS
        } else {
            cut.cells * weight + cut.units + 4 * weight * (mines_before - mines(&after))
        };
        if loss > worst.0 {
            worst = (loss, cells);
        }
    }
    worst
}

#[cfg(test)]
mod tests;
