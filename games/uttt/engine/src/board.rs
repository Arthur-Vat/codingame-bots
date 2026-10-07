//! The position and the rules.

use cg_core::rng::Rng;

use crate::grid::{self, FULL};
use crate::moves::{Move, MoveList};

/// Whether the game goes on, and how it ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Status {
    Ongoing,
    /// The player in this seat won: 0 moved first.
    Win(usize),
    Draw,
}

/// Marker for "the player to move may choose any open small board".
const ANY_BOARD: u8 = 9;

/// An Ultimate Tic-Tac-Toe position. Small enough to copy freely.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    /// `marks[seat][board]`: the cells of a small board marked by a seat.
    marks: [[u16; 9]; 2],
    /// `won[seat]`: the small boards a seat has won.
    won: [u16; 2],
    /// Small boards that are won or full; no more moves there.
    closed: u16,
    /// The small board the player to move must play in, or `ANY_BOARD`.
    target: u8,
    to_move: u8,
    status: Status,
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl Board {
    /// The starting position: empty, seat 0 to move, anywhere.
    pub fn new() -> Self {
        Board {
            marks: [[0; 9]; 2],
            won: [0; 2],
            closed: 0,
            target: ANY_BOARD,
            to_move: 0,
            status: Status::Ongoing,
        }
    }

    /// The seat to move: 0 or 1.
    #[inline]
    pub fn to_move(&self) -> usize {
        usize::from(self.to_move)
    }

    #[inline]
    pub fn status(&self) -> Status {
        self.status
    }

    /// The seat that marked the cell of `mv`, if any.
    pub fn mark(&self, mv: Move) -> Option<usize> {
        let bit = 1 << mv.cell();
        (0..2).find(|&seat| self.marks[seat][mv.board()] & bit != 0)
    }

    /// The seat that won small board `board`, if any.
    pub fn small_winner(&self, board: usize) -> Option<usize> {
        (0..2).find(|&seat| self.won[seat] & (1 << board) != 0)
    }

    /// Small boards won by each seat: the tiebreak points.
    pub fn points(&self) -> [u32; 2] {
        [self.won[0].count_ones(), self.won[1].count_ones()]
    }

    /// The small board the player to move is sent to, or `None` when it may
    /// play in any open small board.
    pub fn target(&self) -> Option<usize> {
        (self.target != ANY_BOARD).then_some(usize::from(self.target))
    }

    /// Whether small board `board` is won or full.
    pub fn is_closed(&self, board: usize) -> bool {
        self.closed & (1 << board) != 0
    }

    /// Empty cells of `board`, as a 9-bit mask.
    #[inline]
    fn empty_cells(&self, board: usize) -> u16 {
        !(self.marks[0][board] | self.marks[1][board]) & FULL
    }

    /// Whether the player to move may play `mv`.
    pub fn is_legal(&self, mv: Move) -> bool {
        let board = mv.board();
        self.status == Status::Ongoing
            && !self.is_closed(board)
            && (self.target == ANY_BOARD || usize::from(self.target) == board)
            && self.empty_cells(board) & (1 << mv.cell()) != 0
    }

    /// Writes the legal moves into `moves`, by board then cell.
    pub fn legal_moves(&self, moves: &mut MoveList) {
        moves.clear();
        if self.status != Status::Ongoing {
            return;
        }
        if self.target != ANY_BOARD {
            self.push_board_moves(usize::from(self.target), moves);
        } else {
            for board in grid::cells(!self.closed & FULL) {
                self.push_board_moves(board, moves);
            }
        }
    }

    #[inline]
    fn push_board_moves(&self, board: usize, moves: &mut MoveList) {
        for cell in grid::cells(self.empty_cells(board)) {
            moves.push(Move::new(board, cell));
        }
    }

    /// Plays `mv` for the player to move. The move must be legal; this is
    /// checked in debug builds only, for speed.
    pub fn play(&mut self, mv: Move) {
        debug_assert!(self.is_legal(mv), "illegal move {mv} in {self:?}");
        let seat = usize::from(self.to_move);
        let (board, cell) = (mv.board(), mv.cell());
        self.marks[seat][board] |= 1 << cell;

        if grid::has_line(self.marks[seat][board]) {
            self.won[seat] |= 1 << board;
            self.closed |= 1 << board;
            if grid::has_line(self.won[seat]) {
                self.status = Status::Win(seat);
            }
        } else if self.empty_cells(board) == 0 {
            self.closed |= 1 << board;
        }

        self.target = if self.is_closed(cell) {
            ANY_BOARD
        } else {
            cell as u8
        };
        self.to_move ^= 1;

        if self.status == Status::Ongoing && self.closed == FULL {
            let [zero, one] = self.points();
            self.status = match zero.cmp(&one) {
                std::cmp::Ordering::Greater => Status::Win(0),
                std::cmp::Ordering::Less => Status::Win(1),
                std::cmp::Ordering::Equal => Status::Draw,
            };
        }
    }

    /// Plays uniformly random legal moves until the game ends, and returns
    /// how it ended.
    pub fn random_playout(&mut self, rng: &mut Rng) -> Status {
        let mut moves = MoveList::new();
        while self.status == Status::Ongoing {
            self.legal_moves(&mut moves);
            let mv = moves[rng.below(moves.len() as u64) as usize];
            self.play(mv);
        }
        self.status
    }
}

#[cfg(test)]
mod tests;
