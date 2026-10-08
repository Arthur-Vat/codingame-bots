//! The position and the rules.

use cg_core::rng::Rng;

use crate::grid::{self, FULL};
use crate::moves::{Move, MoveList};

mod policy;

pub use policy::{
    PlayoutPolicy, BLOCKS, CENTRE, CLASSES, FEATURES, FEATURE_NAMES, GIVES_BOARD,
    GIVES_FREE_CHOICE, WINS_BOARD,
};

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
    /// `threats[seat]`: the open small boards where `seat` has a free cell
    /// that would win the board, kept up to date move by move.
    threats: [u16; 2],
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
            threats: [0; 2],
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
    #[inline(always)]
    pub fn play(&mut self, mv: Move) {
        debug_assert!(self.is_legal(mv), "illegal move {mv} in {self:?}");
        self.play_at(mv.board(), mv.cell());
    }

    /// Plays in `cell` of small board `board`: [`play`](Board::play)
    /// without encoding the move, for playouts.
    #[inline(always)]
    fn play_at(&mut self, board: usize, cell: usize) {
        let seat = usize::from(self.to_move);
        let marks = self.marks[seat][board] | (1 << cell);
        self.marks[seat][board] = marks;

        if grid::has_line(marks) {
            self.won[seat] |= 1 << board;
            self.closed |= 1 << board;
            if grid::has_line(self.won[seat]) {
                self.status = Status::Win(seat);
            } else if self.closed == FULL {
                self.status = self.status_on_points();
            }
        } else if marks | self.marks[1 - seat][board] == FULL {
            self.closed |= 1 << board;
            if self.closed == FULL {
                self.status = self.status_on_points();
            }
        }

        // Only this small board changed: the mover may have made a threat
        // there, blocked the opponent's, or closed the board.
        let bit = 1 << board;
        let open = self.closed & bit == 0;
        let other = self.marks[1 - seat][board];
        let empty = !(marks | other) & FULL;
        let mine = open && grid::completing_cells(marks) & empty != 0;
        let theirs = open && grid::completing_cells(other) & empty != 0;
        self.threats[seat] = (self.threats[seat] & !bit) | (u16::from(mine) << board);
        self.threats[1 - seat] = (self.threats[1 - seat] & !bit) | (u16::from(theirs) << board);

        self.target = if self.is_closed(cell) {
            ANY_BOARD
        } else {
            cell as u8
        };
        self.to_move ^= 1;
    }

    /// How the game ends once every small board is closed with no line of
    /// small boards: more small boards wins, equal is a draw.
    fn status_on_points(&self) -> Status {
        let [zero, one] = self.points();
        match zero.cmp(&one) {
            std::cmp::Ordering::Greater => Status::Win(0),
            std::cmp::Ordering::Less => Status::Win(1),
            std::cmp::Ordering::Equal => Status::Draw,
        }
    }

    /// A uniformly random legal move, chosen without listing the moves. It
    /// draws the same number from `rng` as picking from
    /// [`legal_moves`](Board::legal_moves), and picks the same move for it.
    /// The game must go on.
    #[inline(always)]
    pub fn random_move(&self, rng: &mut Rng) -> Move {
        let (board, cell) = self.random_cell(rng);
        Move::new_unchecked(board, cell)
    }

    /// [`random_move`](Board::random_move) as a small board and a cell.
    #[inline(always)]
    fn random_cell(&self, rng: &mut Rng) -> (usize, usize) {
        debug_assert_eq!(self.status, Status::Ongoing);
        if self.target != ANY_BOARD {
            let board = usize::from(self.target);
            let empty = self.empty_cells(board);
            let index = rng.below(u64::from(grid::count(empty))) as u32;
            return (board, grid::nth_cell(empty, index));
        }
        let open = !self.closed & FULL;
        let total: u32 = grid::cells(open)
            .map(|board| grid::count(self.empty_cells(board)))
            .sum();
        let mut index = rng.below(u64::from(total)) as u32;
        for board in grid::cells(open) {
            let empty = self.empty_cells(board);
            let count = grid::count(empty);
            if index < count {
                return (board, grid::nth_cell(empty, index));
            }
            index -= count;
        }
        unreachable!("a game that goes on has a legal move")
    }

    /// Plays uniformly random legal moves until the game ends, and returns
    /// how it ended.
    pub fn random_playout(&mut self, rng: &mut Rng) -> Status {
        while self.status == Status::Ongoing {
            let (board, cell) = self.random_cell(rng);
            self.play_at(board, cell);
        }
        self.status
    }

    /// The small boards among `boards` where `seat` could win the whole
    /// game at once: open boards that would complete a line of small boards
    /// for `seat`, with a free cell that completes a line of cells there.
    /// Whether `seat` may play there is up to the caller.
    #[inline]
    fn game_winning_boards(&self, seat: usize, boards: u16) -> u16 {
        grid::completing_cells(self.won[seat]) & boards & self.threats[seat]
    }

    /// A move for playouts that looks one move ahead ("decisive moves"): a
    /// move that wins the game with a line of small boards if there is one,
    /// else the same uniformly random move as
    /// [`random_move`](Board::random_move). Wins on points, when the last
    /// open board closes, are not looked for. The game must go on.
    #[inline]
    pub fn decisive_move(&self, rng: &mut Rng) -> Move {
        let (board, cell) = self.decisive_cell(rng);
        Move::new_unchecked(board, cell)
    }

    /// [`decisive_move`](Board::decisive_move) as a small board and a cell.
    #[inline(always)]
    fn decisive_cell(&self, rng: &mut Rng) -> (usize, usize) {
        match self.game_winning_cell() {
            Some(winning) => winning,
            None => self.random_cell(rng),
        }
    }

    /// A legal move that wins the game at once with a line of small
    /// boards, if there is one: the move playouts play first.
    pub fn game_winning_move(&self) -> Option<Move> {
        self.game_winning_cell()
            .map(|(board, cell)| Move::new_unchecked(board, cell))
    }

    /// The first legal move, as a small board and a cell, that wins the game
    /// with a line of small boards, if there is one.
    #[inline(always)]
    fn game_winning_cell(&self) -> Option<(usize, usize)> {
        let seat = self.to_move();
        let allowed = if self.target == ANY_BOARD {
            FULL
        } else {
            1 << self.target
        };
        let winning = self.game_winning_boards(seat, allowed);
        if winning == 0 {
            return None;
        }
        let board = winning.trailing_zeros() as usize;
        let cells = grid::completing_cells(self.marks[seat][board]) & self.empty_cells(board);
        Some((board, cells.trailing_zeros() as usize))
    }

    /// Plays [`decisive_move`](Board::decisive_move)s until the game ends,
    /// and returns how it ended.
    pub fn decisive_playout(&mut self, rng: &mut Rng) -> Status {
        while self.status == Status::Ongoing {
            let (board, cell) = self.decisive_cell(rng);
            self.play_at(board, cell);
        }
        self.status
    }
}

#[cfg(test)]
mod tests;
