//! Random turns, for playouts and as a source of commands for searches.

use cg_core::rng::Rng;

use crate::bits::{Bits, CELLS, SIZE};
use crate::command::Command;
use crate::state::{Outcome, State, HQ};

impl State {
    /// A plausible random turn for the player to act, in `out` (cleared
    /// first). Each unit that may move goes toward a random cell, toward the
    /// enemy HQ one time in four, or stays; then trainings at random levels
    /// on cells where TRAIN succeeds, then builds when affordable (the three
    /// parts are sometimes in another order). The commands are chosen on a
    /// scratch copy, so each sees the effects of the ones before it, and every
    /// command in `out` succeeds when applied in order to this state.
    pub fn random_turn(&self, rng: &mut Rng, out: &mut Vec<Command>) {
        out.clear();
        if self.outcome.is_some() {
            return;
        }
        let mut scratch = self.clone();
        let mut phases = [0u8, 1, 2];
        if rng.below(4) == 0 {
            rng.shuffle(&mut phases);
        }
        for phase in phases {
            match phase {
                0 => scratch.random_moves(rng, out),
                1 => scratch.random_trainings(rng, out),
                _ => scratch.random_builds(rng, out),
            }
            if scratch.outcome.is_some() {
                break;
            }
        }
    }

    /// Applies `command` and records it if it succeeded.
    fn try_command(&mut self, command: Command, out: &mut Vec<Command>) -> bool {
        let done = self.apply(command);
        if done {
            out.push(command);
        }
        done
    }

    fn random_moves(&mut self, rng: &mut Rng, out: &mut Vec<Command>) {
        let seat = self.to_act;
        let mut cells = [0u8; CELLS];
        let mut count = 0;
        for cell in (self.units[seat] & self.movable).iter() {
            cells[count] = cell as u8;
            count += 1;
        }
        rng.shuffle(&mut cells[..count]);
        for &cell in &cells[..count] {
            let cell = usize::from(cell);
            if rng.below(5) == 0 {
                continue;
            }
            let id = self.unit_id[cell];
            let target = if rng.below(4) == 0 {
                HQ[1 - seat]
            } else {
                let choice = rng.below(u64::from(self.map.valid.count())) as usize;
                self.map.valid.nth(choice).unwrap_or(cell)
            };
            let command = Command::Move {
                id,
                x: (target % SIZE) as u8,
                y: (target / SIZE) as u8,
            };
            if !self.try_command(command, out) {
                // Step to a random neighbour the unit may enter instead.
                let options =
                    self.enterable(seat, self.level_at(cell)) & Bits::cell(cell).neighbours();
                if let Some(step) = pick(options, rng) {
                    let command = Command::Move {
                        id,
                        x: (step % SIZE) as u8,
                        y: (step / SIZE) as u8,
                    };
                    self.try_command(command, out);
                }
            }
            if self.outcome.is_some() {
                return;
            }
        }
    }

    fn random_trainings(&mut self, rng: &mut Rng, out: &mut Vec<Command>) {
        let seat = self.to_act;
        for _ in 0..rng.below(5) {
            let mut level = match rng.below(10) {
                0..=5 => 1,
                6..=8 => 2,
                _ => 3,
            }
            .min(self.league.max_level());
            while level > 0 && self.gold[seat] < 10 * i64::from(level) {
                level -= 1;
            }
            if level == 0 {
                return;
            }
            let options = self.near_active(seat) & self.enterable(seat, level);
            // Most trainings extend the territory.
            let outward = options.and_not(self.own[seat]);
            let pool = if !outward.is_empty() && rng.below(4) != 0 {
                outward
            } else {
                options
            };
            let Some(cell) = pick(pool, rng) else {
                continue;
            };
            let command = Command::Train {
                level,
                x: (cell % SIZE) as u8,
                y: (cell / SIZE) as u8,
            };
            self.try_command(command, out);
            if self.outcome.is_some() {
                return;
            }
        }
    }

    fn random_builds(&mut self, rng: &mut Rng, out: &mut Vec<Command>) {
        let seat = self.to_act;
        for _ in 0..rng.below(3) {
            if self.gold[seat] < 15 {
                return;
            }
            let free = self.own[seat] & self.near_active(seat) & self.empty_cells();
            let mine =
                self.league.allows_mines() && (!self.league.allows_towers() || rng.below(3) != 0);
            let (options, command): (Bits, fn(u8, u8) -> Command) = if mine {
                (free & self.map.spots, |x, y| Command::BuildMine { x, y })
            } else if self.league.allows_towers() {
                (free.and_not(self.map.spots), |x, y| Command::BuildTower {
                    x,
                    y,
                })
            } else {
                return;
            };
            if let Some(cell) = pick(options, rng) {
                self.try_command(command((cell % SIZE) as u8, (cell / SIZE) as u8), out);
            }
        }
    }

    /// Plays random turns until the game ends, to its outcome. For playouts.
    pub fn random_playout(&mut self, rng: &mut Rng) -> Outcome {
        let mut turn = Vec::new();
        loop {
            if let Some(outcome) = self.outcome {
                return outcome;
            }
            self.random_turn(rng, &mut turn);
            self.play_turn(&turn);
        }
    }
}

/// A random cell of `cells`, if there is one.
fn pick(cells: Bits, rng: &mut Rng) -> Option<usize> {
    let count = cells.count();
    if count == 0 {
        return None;
    }
    cells.nth(rng.below(u64::from(count)) as usize)
}
