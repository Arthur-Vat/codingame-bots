//! Perfect play for plain 3×3 tic-tac-toe.
//!
//! Positions are pairs of 9-bit masks: the cells of the player to move and
//! the cells of its opponent. The whole game tree has fewer than 6,000
//! positions, so the solver simply remembers every value it computes.

use std::collections::HashMap;

use uttt_engine::grid::{cells, has_line, FULL};

/// What a position is worth to one player, from 0 (loss) to 1 (win).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Value {
    /// The score the player is sure to get, whatever the opponent does.
    pub guaranteed: f64,
    /// The average score against an opponent playing uniformly at random.
    pub expected: f64,
}

const WIN: Value = Value {
    guaranteed: 1.0,
    expected: 1.0,
};
const DRAW: Value = Value {
    guaranteed: 0.5,
    expected: 0.5,
};
const LOSS: Value = Value {
    guaranteed: 0.0,
    expected: 0.0,
};

impl Value {
    /// Never trade guaranteed score for expected score.
    fn better_than(self, other: Value) -> bool {
        self.guaranteed > other.guaranteed
            || (self.guaranteed == other.guaranteed && self.expected > other.expected + 1e-12)
    }
}

/// Remembers the best move and value of every position it has solved.
#[derive(Default)]
pub struct Solver {
    memo: HashMap<(u16, u16), (Value, usize)>,
}

impl Solver {
    pub fn new() -> Self {
        Solver::default()
    }

    /// The best cell (0 to 8, row-major) for the player to move, who owns
    /// `mine` while the opponent owns `theirs`. Best means the highest
    /// guaranteed score, then the highest expected score against random play.
    /// `None` when the game is already over.
    pub fn best_move(&mut self, mine: u16, theirs: u16) -> Option<usize> {
        if has_line(mine) || has_line(theirs) || (mine | theirs) & FULL == FULL {
            return None;
        }
        Some(self.solve(mine, theirs).1)
    }

    /// The value of the position for the player to move.
    #[cfg(test)]
    pub fn value(&mut self, mine: u16, theirs: u16) -> Value {
        self.solve(mine, theirs).0
    }

    /// Value and best cell for the player to move. The game is not over.
    fn solve(&mut self, mine: u16, theirs: u16) -> (Value, usize) {
        if let Some(&known) = self.memo.get(&(mine, theirs)) {
            return known;
        }
        let mut best: Option<(Value, usize)> = None;
        for cell in cells(!(mine | theirs) & FULL) {
            let after = mine | 1 << cell;
            let value = if has_line(after) {
                WIN
            } else if (after | theirs) & FULL == FULL {
                DRAW
            } else {
                self.opponent_moves(after, theirs)
            };
            if best.is_none_or(|(best_value, _)| value.better_than(best_value)) {
                best = Some((value, cell));
            }
        }
        let solved = best.expect("a game that is not over has an empty cell");
        self.memo.insert((mine, theirs), solved);
        solved
    }

    /// Our value when the opponent, owning `theirs`, is to move.
    fn opponent_moves(&mut self, mine: u16, theirs: u16) -> Value {
        let empty = !(mine | theirs) & FULL;
        let mut guaranteed: f64 = 1.0;
        let mut total = 0.0;
        for cell in cells(empty) {
            let after = theirs | 1 << cell;
            let value = if has_line(after) {
                LOSS
            } else if (mine | after) & FULL == FULL {
                DRAW
            } else {
                // Our turn again, with the same perspective.
                self.solve(mine, after).0
            };
            guaranteed = guaranteed.min(value.guaranteed);
            total += value.expected;
        }
        Value {
            guaranteed,
            expected: total / f64::from(empty.count_ones()),
        }
    }
}

#[cfg(test)]
mod tests;
