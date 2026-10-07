//! Moves and fixed-capacity move lists.

use std::fmt;
use std::ops::Deref;

/// A cell of the 9×9 board, stored as `9 * board + cell`: `board` is the
/// small board (0 to 8, row-major) and `cell` the position inside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Move(u8);

impl Move {
    /// The move in cell `cell` of small board `board`, both from 0 to 8.
    ///
    /// # Panics
    ///
    /// If either index is above 8.
    pub fn new(board: usize, cell: usize) -> Move {
        assert!(board < 9 && cell < 9, "no cell {cell} in board {board}");
        Move((9 * board + cell) as u8)
    }

    /// The move in cell `cell` of small board `board`, both from 0 to 8,
    /// checked in debug builds only, for speed.
    #[inline]
    pub(crate) fn new_unchecked(board: usize, cell: usize) -> Move {
        debug_assert!(board < 9 && cell < 9, "no cell {cell} in board {board}");
        Move((9 * board + cell) as u8)
    }

    /// The move at CodinGame coordinates `(row, col)`, or `None` outside the
    /// 9×9 board.
    pub fn from_row_col(row: usize, col: usize) -> Option<Move> {
        (row < 9 && col < 9).then(|| Move::new(3 * (row / 3) + col / 3, 3 * (row % 3) + col % 3))
    }

    /// The small board, 0 to 8.
    #[inline]
    pub fn board(self) -> usize {
        usize::from(self.0 / 9)
    }

    /// The cell inside the small board, 0 to 8. It is also the small board
    /// the opponent is sent to.
    #[inline]
    pub fn cell(self) -> usize {
        usize::from(self.0 % 9)
    }

    /// CodinGame coordinates `(row, col)`.
    pub fn row_col(self) -> (usize, usize) {
        let (board, cell) = (self.board(), self.cell());
        (3 * (board / 3) + cell / 3, 3 * (board % 3) + cell % 3)
    }
}

impl fmt::Display for Move {
    /// Formats the move as CodinGame expects it: `row col`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (row, col) = self.row_col();
        write!(f, "{row} {col}")
    }
}

/// Up to 81 moves, without allocation.
#[derive(Clone)]
pub struct MoveList {
    moves: [Move; 81],
    len: usize,
}

impl MoveList {
    pub fn new() -> Self {
        MoveList {
            moves: [Move(0); 81],
            len: 0,
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Adds a move. There is room for every cell of the board.
    #[inline]
    pub fn push(&mut self, mv: Move) {
        self.moves[self.len] = mv;
        self.len += 1;
    }
}

impl Default for MoveList {
    fn default() -> Self {
        MoveList::new()
    }
}

impl Deref for MoveList {
    type Target = [Move];

    fn deref(&self) -> &[Move] {
        &self.moves[..self.len]
    }
}

impl fmt::Debug for MoveList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

#[cfg(test)]
mod tests;
