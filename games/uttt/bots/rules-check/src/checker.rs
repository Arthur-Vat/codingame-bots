//! Follows the game with the engine and compares it with CodinGame's input.

use std::collections::BTreeSet;

use uttt_engine::{Board, Move, MoveList, Status};

/// Most actions listed in one difference message.
const MAX_LISTED: usize = 12;

pub struct Checker {
    board: Board,
    /// Our seat: 0 if we moved first. Known from the first turn's input.
    seat: Option<usize>,
    /// False once a move the engine cannot play has been seen; the board is
    /// no longer trustworthy after that.
    in_sync: bool,
    turns: u32,
    differences: u32,
}

impl Checker {
    pub fn new() -> Self {
        Checker {
            board: Board::new(),
            seat: None,
            in_sync: true,
            turns: 0,
            differences: 0,
        }
    }

    pub fn turns(&self) -> u32 {
        self.turns
    }

    /// Turns on which CodinGame and the engine disagreed.
    pub fn differences(&self) -> u32 {
        self.differences
    }

    /// Checks a turn's input: the opponent's last action and the valid
    /// actions. Returns the lines to print.
    pub fn check_turn(&mut self, opponent: (i32, i32), actions: &[(i32, i32)]) -> Vec<String> {
        self.turns += 1;
        let mut problems = Vec::new();
        let first_of_game = opponent == (-1, -1);
        let seat = *self.seat.get_or_insert(usize::from(!first_of_game));
        if first_of_game && self.turns > 1 {
            problems.push("CodinGame sent -1 -1 after the first turn".to_string());
        }

        if !first_of_game && self.in_sync {
            match cell(opponent).filter(|mv| self.board.is_legal(*mv)) {
                Some(mv) => self.board.play(mv),
                None => {
                    problems.push(format!(
                        "the opponent played {} {}, which the engine says is not a valid action",
                        opponent.0, opponent.1
                    ));
                    self.in_sync = false;
                }
            }
        }

        if self.in_sync {
            if self.board.status() != Status::Ongoing {
                problems.push(format!(
                    "the engine says the game is over ({}), but CodinGame asks for a move",
                    describe(self.board.status(), seat, self.board.points())
                ));
            }
            let mut moves = MoveList::new();
            self.board.legal_moves(&mut moves);
            let engine: BTreeSet<(i32, i32)> = moves
                .iter()
                .map(|mv| {
                    let (row, col) = mv.row_col();
                    (row as i32, col as i32)
                })
                .collect();
            let codingame: BTreeSet<(i32, i32)> = actions.iter().copied().collect();
            if codingame.len() != actions.len() {
                problems.push("CodinGame listed an action twice".to_string());
            }
            if engine != codingame {
                problems.push(format!(
                    "valid actions differ. Only CodinGame lists: {}. Only the engine lists: {}",
                    list(codingame.difference(&engine)),
                    list(engine.difference(&codingame))
                ));
            }
        }

        if !problems.is_empty() {
            self.differences += 1;
        }
        let mut lines: Vec<String> = problems
            .iter()
            .map(|problem| format!("rules-check: DIFFERENCE on turn {}: {problem}", self.turns))
            .collect();
        let verdict = if problems.is_empty() {
            "same valid actions"
        } else {
            "DIFFERENT"
        };
        let sync = if self.in_sync {
            ""
        } else {
            " (no longer following the game)"
        };
        lines.push(format!(
            "rules-check: turn {}: {verdict}; {} turns checked, {} with differences{sync}",
            self.turns, self.turns, self.differences
        ));
        lines
    }

    /// Follows our own action. Returns the lines to print, if any.
    pub fn record_own_move(&mut self, action: (i32, i32)) -> Vec<String> {
        if !self.in_sync {
            return Vec::new();
        }
        match cell(action).filter(|mv| self.board.is_legal(*mv)) {
            Some(mv) => self.board.play(mv),
            None => {
                self.in_sync = false;
                return vec![format!(
                    "rules-check: my move {} {} is not valid for the engine; no longer following the game",
                    action.0, action.1
                )];
            }
        }
        let status = self.board.status();
        if status == Status::Ongoing {
            return Vec::new();
        }
        let seat = self.seat.unwrap_or(0);
        vec![format!(
            "rules-check: the engine says my move ends the game: {}. Check that CodinGame agrees.",
            describe(status, seat, self.board.points())
        )]
    }
}

/// The engine's move for CodinGame coordinates, if on the board.
fn cell((row, col): (i32, i32)) -> Option<Move> {
    let row = usize::try_from(row).ok()?;
    let col = usize::try_from(col).ok()?;
    Move::from_row_col(row, col)
}

/// The result from our point of view, with the points.
fn describe(status: Status, seat: usize, points: [u32; 2]) -> String {
    let (mine, theirs) = (points[seat], points[1 - seat]);
    let result = match status {
        Status::Ongoing => "still going",
        Status::Win(winner) if winner == seat => "I win",
        Status::Win(_) => "I lose",
        Status::Draw => "draw",
    };
    format!("{result}, small boards won {mine}-{theirs}")
}

/// At most `MAX_LISTED` actions as `r c` pairs, or "none".
fn list<'a>(actions: impl Iterator<Item = &'a (i32, i32)>) -> String {
    let actions: Vec<&(i32, i32)> = actions.collect();
    if actions.is_empty() {
        return "none".to_string();
    }
    let mut text: Vec<String> = actions
        .iter()
        .take(MAX_LISTED)
        .map(|(row, col)| format!("{row} {col}"))
        .collect();
    if actions.len() > MAX_LISTED {
        text.push(format!("and {} more", actions.len() - MAX_LISTED));
    }
    text.join(", ")
}

#[cfg(test)]
mod tests;
