//! Reference implementation of CodinGame's Ultimate Tic-Tac-Toe rules, as
//! described in `games/uttt/RULES.md`.
//!
//! It is written to be obviously correct rather than fast: the arena uses it
//! to judge matches, and faster engines used inside bots are checked against
//! it.

use std::time::Duration;

use cg_arena::referee::{Answer, InvalidAnswer, Outcome, Referee, TimeLimits};
use cg_core::rng::Rng;

/// A cell as `(row, col)`, both from 0 to 8.
pub type Cell = (usize, usize);

/// The 8 lines of a 3×3 grid, as `(row, col)` triples.
const LINES: [[(usize, usize); 3]; 8] = [
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    [(0, 0), (1, 1), (2, 2)],
    [(0, 2), (1, 1), (2, 0)],
];

/// The player who owns a whole line of `grid`, if any.
fn line_owner(grid: &[[Option<usize>; 3]; 3]) -> Option<usize> {
    LINES.iter().find_map(|line| {
        let [a, b, c] = line.map(|(row, col)| grid[row][col]);
        a.filter(|_| a == b && b == c)
    })
}

/// The state of an Ultimate Tic-Tac-Toe game, with the referee's protocol.
#[derive(Clone, Debug)]
pub struct UtttReferee {
    /// The seat that marked each cell.
    marks: [[Option<usize>; 9]; 9],
    /// The winner of each small board, indexed by board row and column.
    small_winners: [[Option<usize>; 3]; 3],
    /// Small boards won by each seat.
    points: [u32; 2],
    /// Set when a seat aligns three small boards.
    main_winner: Option<usize>,
    to_move: usize,
    last: Option<Cell>,
    /// Valid actions for the player to move, in the (shuffled) order sent.
    valid: Vec<Cell>,
    /// Shuffles the valid actions, like CodinGame does.
    rng: Rng,
    /// Actions played so far.
    moves: usize,
    /// Actions imposed at the start of the game (see [`UtttReferee::with_opening`]).
    opening: Vec<Cell>,
}

/// Mixed into the game seed to draw the opening, so the opening and the
/// order of valid actions are independent.
const OPENING_SALT: u64 = 0x0BE7_1736_0F3A_11CE;

impl UtttReferee {
    /// A new game. `seed` only decides the order of the valid actions.
    pub fn new(seed: u64) -> Self {
        let mut game = UtttReferee {
            marks: [[None; 9]; 9],
            small_winners: [[None; 3]; 3],
            points: [0; 2],
            main_winner: None,
            to_move: 0,
            last: None,
            valid: Vec::new(),
            rng: Rng::new(seed),
            moves: 0,
            opening: Vec::new(),
        };
        game.update_valid_actions();
        game
    }

    /// A new game whose first `plies` actions are drawn at random from
    /// `seed`, to start the bots from varied positions. During those turns
    /// the valid-action list holds only the imposed action, so a bot that
    /// picks from the list plays it. Not a CodinGame rule: the arena uses it
    /// to test bots on more positions.
    ///
    /// The same seed gives the same opening. An opening never ends the game:
    /// it is cut short before an action that would.
    pub fn with_opening(seed: u64, plies: u32) -> Self {
        let mut rng = Rng::new(seed ^ OPENING_SALT);
        let mut scratch = UtttReferee::new(seed ^ OPENING_SALT);
        let mut opening = Vec::new();
        for _ in 0..plies {
            let quiet: Vec<Cell> = scratch
                .valid
                .iter()
                .copied()
                .filter(|&cell| {
                    let mut next = scratch.clone();
                    next.play_cell(cell).is_ok() && next.result().is_none()
                })
                .collect();
            let Some(&cell) = rng.pick(&quiet) else {
                break;
            };
            scratch
                .play_cell(cell)
                .expect("the opening plays valid actions");
            opening.push(cell);
        }
        let mut game = UtttReferee::new(seed);
        game.opening = opening;
        game.update_valid_actions();
        game
    }

    /// The actions imposed at the start of the game.
    pub fn opening(&self) -> &[Cell] {
        &self.opening
    }

    /// The valid actions for the player to move, in the order sent.
    pub fn valid_actions(&self) -> &[Cell] {
        &self.valid
    }

    /// The seat to move.
    pub fn to_move(&self) -> usize {
        self.to_move
    }

    /// The last action played, if any.
    pub fn last_action(&self) -> Option<Cell> {
        self.last
    }

    /// Small boards won by each seat.
    pub fn points(&self) -> [u32; 2] {
        self.points
    }

    /// The seat that marked `cell`, if any.
    pub fn mark(&self, (row, col): Cell) -> Option<usize> {
        self.marks[row][col]
    }

    /// The winner of the small board at `(board_row, board_col)`, if any.
    pub fn small_winner(&self, board_row: usize, board_col: usize) -> Option<usize> {
        self.small_winners[board_row][board_col]
    }

    /// The result once the game is over.
    pub fn result(&self) -> Option<Outcome> {
        if let Some(seat) = self.main_winner {
            return Some(Outcome::Win(seat));
        }
        if !self.valid.is_empty() {
            return None;
        }
        let [zero, one] = self.points;
        Some(match zero.cmp(&one) {
            std::cmp::Ordering::Greater => Outcome::Win(0),
            std::cmp::Ordering::Less => Outcome::Win(1),
            std::cmp::Ordering::Equal => Outcome::Draw,
        })
    }

    /// Plays `cell` for the player to move.
    pub fn play_cell(&mut self, cell: Cell) -> Result<(), String> {
        if self.result().is_some() {
            return Err("the game is over".to_string());
        }
        if !self.valid.contains(&cell) {
            return Err(format!("{} {} is not a valid action", cell.0, cell.1));
        }
        let (row, col) = cell;
        let seat = self.to_move;
        self.marks[row][col] = Some(seat);

        let (board_row, board_col) = (row / 3, col / 3);
        if self.board_winner(board_row, board_col) == Some(seat) {
            self.small_winners[board_row][board_col] = Some(seat);
            self.points[seat] += 1;
            self.main_winner = line_owner(&self.small_winners);
        }

        self.last = Some(cell);
        self.to_move = 1 - seat;
        self.moves += 1;
        self.update_valid_actions();
        Ok(())
    }

    /// The owner of a line inside the small board, from its marks.
    fn board_winner(&self, board_row: usize, board_col: usize) -> Option<usize> {
        let mut grid = [[None; 3]; 3];
        for (r, grid_row) in grid.iter_mut().enumerate() {
            for (c, mark) in grid_row.iter_mut().enumerate() {
                *mark = self.marks[3 * board_row + r][3 * board_col + c];
            }
        }
        line_owner(&grid)
    }

    /// The empty cells of a small board that is still open: neither won nor
    /// full. A closed board has none.
    fn open_cells(&self, board_row: usize, board_col: usize) -> Vec<Cell> {
        if self.small_winners[board_row][board_col].is_some() {
            return Vec::new();
        }
        let mut cells = Vec::new();
        for row in 3 * board_row..3 * board_row + 3 {
            for col in 3 * board_col..3 * board_col + 3 {
                if self.marks[row][col].is_none() {
                    cells.push((row, col));
                }
            }
        }
        cells
    }

    fn update_valid_actions(&mut self) {
        self.valid.clear();
        if self.main_winner.is_some() {
            return;
        }
        if let Some((row, col)) = self.last {
            // Sent to the small board at the cell's position in its board.
            self.valid = self.open_cells(row % 3, col % 3);
        }
        if self.valid.is_empty() {
            for board_row in 0..3 {
                for board_col in 0..3 {
                    let cells = self.open_cells(board_row, board_col);
                    self.valid.extend(cells);
                }
            }
        }
        self.rng.shuffle(&mut self.valid);
        if let Some(&imposed) = self.opening.get(self.moves) {
            debug_assert!(self.valid.contains(&imposed));
            self.valid = vec![imposed];
        }
    }
}

/// Parses an answer the way CodinGame does: split on single spaces, the
/// first two parts are integers, the rest is ignored.
fn parse_answer(line: &str) -> Option<(i64, i64)> {
    let mut parts = line.split(' ');
    let row = parts.next()?.parse().ok()?;
    let col = parts.next()?.parse().ok()?;
    Some((row, col))
}

impl Referee for UtttReferee {
    fn time_limits(&self) -> TimeLimits {
        TimeLimits {
            first_answer: Duration::from_millis(1000),
            later_answers: Duration::from_millis(100),
        }
    }

    fn players_to_act(&self) -> Vec<usize> {
        if self.result().is_some() {
            Vec::new()
        } else {
            vec![self.to_move]
        }
    }

    fn turn_input(&self, _seat: usize) -> String {
        let (row, col) = self
            .last
            .map_or((-1, -1), |(row, col)| (row as i64, col as i64));
        let mut input = format!("{row} {col}\n{}\n", self.valid.len());
        for (row, col) in &self.valid {
            input.push_str(&format!("{row} {col}\n"));
        }
        input
    }

    fn play(&mut self, answers: &[Answer]) -> Result<(), InvalidAnswer> {
        let answer = answers
            .first()
            .expect("the arena sends one answer per turn");
        let invalid = |reason: String| InvalidAnswer {
            seat: answer.seat,
            reason,
        };
        let line = answer.lines.first().map_or("", String::as_str);
        let (row, col) = parse_answer(line)
            .ok_or_else(|| invalid(format!("expected `row col`, got {line:?}")))?;
        let cell = match (usize::try_from(row), usize::try_from(col)) {
            (Ok(row), Ok(col)) => (row, col),
            _ => return Err(invalid(format!("{row} {col} is not a valid action"))),
        };
        self.play_cell(cell).map_err(invalid)
    }

    fn outcome(&self) -> Option<Outcome> {
        self.result()
    }
}

#[cfg(test)]
mod tests;
