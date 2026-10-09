//! The move models that `fit-patterns` compares (E016): which features of
//! a move each one weighs. A move's log-weight is the sum of one weight
//! from each table of its model; a model of one table is a plain lookup,
//! as E015's pattern policy is.
//!
//! Features that look past the move are computed on the position after
//! it, where the opponent is to move: exact, and cheap for a bot to compute
//! the same way.

use std::sync::OnceLock;

use uttt_engine::board::{
    symmetric_cell, BLOCKS, CLASSES, DESTINATIONS, PATTERNS, PATTERN_FEATURES, WINS_BOARD,
};
use uttt_engine::grid::{self, FULL};
use uttt_engine::{Board, Move, Status};

/// A model of the moves' log-weights.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Model {
    /// The playout policy's 32 move classes (E011).
    Classes,
    /// E015's pattern policy: the move's small-board pattern and cell, and
    /// 5 kinds of destination.
    Patterns,
    /// Pattern and cell with 7 kinds of destination: E015's, and whether
    /// the move lets the opponent win the game at once, in the board it is
    /// sent to or after a free choice.
    Destinations7,
    /// E015's features, with separate weights before and from the
    /// [`PHASE_PLY`]th move on.
    Phases2,
    /// E015's features, plus a weight for the pattern of the small board
    /// the opponent is sent to, seen by the opponent.
    DestinationPatterns,
    /// [`Model::Destinations7`], the destination's pattern, and a weight
    /// for the role of the move's own small board in the big board.
    Rich,
    /// [`Model::Destinations7`]; the destination's pattern with that
    /// board's role in the big board; and the move's pattern and cell with
    /// its own board's role: about 64,000 weights, to fill a bot's file.
    Large,
}

/// Every model, in the order of reports.
pub const ALL: [Model; 7] = [
    Model::Classes,
    Model::Patterns,
    Model::Destinations7,
    Model::Phases2,
    Model::DestinationPatterns,
    Model::Rich,
    Model::Large,
];

/// The move from which [`Model::Phases2`] uses its second set of weights:
/// marks on the whole board before the move.
pub const PHASE_PLY: u32 = 30;

/// Kinds of destination of [`Model::Destinations7`].
const DESTINATIONS_7: usize = 7;

/// Roles of a move's own small board for [`Model::Rich`]: whether winning
/// it would win the game for the player, for the opponent, or both, times
/// whether the move wins the board, blocks the opponent there, or neither.
const OWN_ROLES: usize = 12;

/// Roles of a small board in the big board: whether winning it would win
/// the game for one side, the other, both or neither ([`role`]).
const ROLES: usize = 4;

impl Model {
    /// The model's name on the command line and in reports.
    pub fn name(self) -> &'static str {
        match self {
            Model::Classes => "classes",
            Model::Patterns => "patterns",
            Model::Destinations7 => "destinations-7",
            Model::Phases2 => "phases-2",
            Model::DestinationPatterns => "destination-patterns",
            Model::Rich => "rich",
            Model::Large => "large",
        }
    }

    /// The model called `name`, if any.
    pub fn from_name(name: &str) -> Option<Model> {
        ALL.into_iter().find(|model| model.name() == name)
    }

    /// The size of each of the model's tables.
    pub fn tables(self) -> Vec<usize> {
        let cells = PATTERN_FEATURES / DESTINATIONS;
        match self {
            Model::Classes => vec![CLASSES],
            Model::Patterns => vec![PATTERN_FEATURES],
            Model::Destinations7 => vec![cells * DESTINATIONS_7],
            Model::Phases2 => vec![2 * PATTERN_FEATURES],
            Model::DestinationPatterns => vec![PATTERN_FEATURES, 1 + destination_patterns()],
            Model::Rich => vec![
                cells * DESTINATIONS_7,
                1 + destination_patterns(),
                OWN_ROLES,
            ],
            Model::Large => vec![
                cells * DESTINATIONS_7,
                1 + ROLES * destination_patterns(),
                cells * ROLES,
            ],
        }
    }

    /// For each table, how many interleaved kinds of weights it holds:
    /// destination kinds or board roles, which spread differently and are
    /// packed apart ([`cg_core::packed::encode_strided`]).
    pub fn strides(self) -> Vec<usize> {
        match self {
            Model::Classes => vec![1],
            Model::Patterns | Model::Phases2 => vec![DESTINATIONS],
            Model::Destinations7 => vec![DESTINATIONS_7],
            Model::DestinationPatterns => vec![DESTINATIONS, 1],
            Model::Rich => vec![DESTINATIONS_7, 1, 1],
            Model::Large => vec![DESTINATIONS_7, ROLES, ROLES],
        }
    }

    /// Weights of the model, all tables together.
    pub fn weights(self) -> usize {
        self.tables().iter().sum()
    }

    /// Appends to `out` the index of `mv`'s weight in each table, offset
    /// by the sizes of the tables before it. `mv` must be legal.
    pub fn features(self, board: &Board, mv: Move, out: &mut Vec<u32>) {
        let mut push = |offset: usize, index: usize| out.push((offset + index) as u32);
        match self {
            Model::Classes => push(0, board.move_class(mv)),
            Model::Patterns => push(0, board.pattern_feature(mv)),
            Model::Destinations7 => push(0, destinations_7(board, mv)),
            Model::Phases2 => {
                let late = usize::from(marks(board) >= PHASE_PLY);
                push(0, late * PATTERN_FEATURES + board.pattern_feature(mv));
            }
            Model::DestinationPatterns => {
                push(0, board.pattern_feature(mv));
                push(PATTERN_FEATURES, destination_pattern(board, mv));
            }
            Model::Rich => {
                let first = PATTERN_FEATURES / DESTINATIONS * DESTINATIONS_7;
                push(0, destinations_7(board, mv));
                push(first, destination_pattern(board, mv));
                push(first + 1 + destination_patterns(), own_role(board, mv));
            }
            Model::Large => {
                let first = PATTERN_FEATURES / DESTINATIONS * DESTINATIONS_7;
                let second = 1 + ROLES * destination_patterns();
                push(0, destinations_7(board, mv));
                push(first, destination_pattern_role(board, mv));
                let pair = board.pattern_feature(mv) / DESTINATIONS;
                push(
                    first + second,
                    pair * ROLES + role(board, board.to_move(), mv.board()),
                );
            }
        }
    }
}

/// The role of small board `small` in the big board for `seat`: 2 if
/// winning it would win the game for `seat`, plus 1 if for the other side.
fn role(board: &Board, seat: usize, small: usize) -> usize {
    let open = !board.closed_boards() & FULL;
    let decides = |seat: usize| {
        usize::from((grid::completing_cells(board.won_boards(seat)) & open) >> small & 1)
    };
    2 * decides(seat) + decides(1 - seat)
}

/// 0 when the move gives a free choice or ends the game; otherwise 1 plus
/// the canonical pattern of the small board the opponent is sent to, seen
/// by the opponent, times [`ROLES`], plus that board's role for the
/// opponent.
fn destination_pattern_role(board: &Board, mv: Move) -> usize {
    let pattern = destination_pattern(board, mv);
    if pattern == 0 {
        return 0;
    }
    let mut after = *board;
    after.play(mv);
    let target = after.target().expect("a destination pattern has a target");
    1 + (pattern - 1) * ROLES + role(&after, after.to_move(), target)
}

/// Marks on the whole board: the moves played so far.
fn marks(board: &Board) -> u32 {
    (0..9)
        .map(|small| board.cells(0, small).count_ones() + board.cells(1, small).count_ones())
        .sum()
}

/// E015's canonical pattern and cell, with one of 7 kinds of destination:
/// 0 for a free choice where the opponent cannot win the game at once, 6
/// for one where it can, 5 for a board where it can, and otherwise 1, plus
/// 1 if it can win that small board at once, plus 2 if the player could.
/// A move that ends the game counts as a safe free choice.
fn destinations_7(board: &Board, mv: Move) -> usize {
    let pair = board.pattern_feature(mv) / DESTINATIONS;
    let mut after = *board;
    after.play(mv);
    let kind = if after.status() != Status::Ongoing {
        0
    } else {
        let opponent_wins = after.game_winning_move().is_some();
        match after.target() {
            None => 6 * usize::from(opponent_wins),
            Some(_) if opponent_wins => 5,
            Some(target) => {
                let can_win = |seat: usize| usize::from((after.threat_boards(seat) >> target) & 1);
                1 + can_win(after.to_move()) + 2 * can_win(board.to_move())
            }
        }
    };
    pair * DESTINATIONS_7 + kind
}

/// The base-3 number with a 1 for each cell of `mask`.
fn base3(mask: u16) -> usize {
    (0..9)
        .filter(|&cell| mask & (1 << cell) != 0)
        .map(|cell| 3usize.pow(cell))
        .sum()
}

/// `ids[pattern]`: the number of the canonical form of an open small-board
/// pattern (a free cell, no line for either side) under the 8 symmetries,
/// from 0; `u32::MAX` for other patterns.
fn pattern_ids() -> &'static [u32] {
    static IDS: OnceLock<Vec<u32>> = OnceLock::new();
    IDS.get_or_init(|| {
        let mut ids = vec![u32::MAX; PATTERNS];
        let mut count = 0;
        for pattern in 0..PATTERNS {
            let mut digits = [0usize; 9];
            let mut rest = pattern;
            for digit in &mut digits {
                *digit = rest % 3;
                rest /= 3;
            }
            let mask = |state: usize| {
                (0..9)
                    .filter(|&cell| digits[cell] == state)
                    .fold(0u16, |mask, cell| mask | 1 << cell)
            };
            if mask(0) == 0 || grid::has_line(mask(1)) || grid::has_line(mask(2)) {
                continue;
            }
            let canonical = (0..8)
                .map(|symmetry| {
                    (0..9)
                        .map(|cell| {
                            digits[cell] * 3usize.pow(symmetric_cell(symmetry, cell) as u32)
                        })
                        .sum::<usize>()
                })
                .min()
                .expect("eight symmetries");
            // The canonical pattern comes first, so it has its number
            // already unless it is this one.
            ids[pattern] = if canonical == pattern {
                count += 1;
                count - 1
            } else {
                ids[canonical]
            };
        }
        ids
    })
}

/// Canonical open small-board patterns.
pub fn destination_patterns() -> usize {
    static COUNT: OnceLock<usize> = OnceLock::new();
    *COUNT.get_or_init(|| {
        pattern_ids()
            .iter()
            .filter(|&&id| id != u32::MAX)
            .map(|&id| id as usize + 1)
            .max()
            .unwrap_or(0)
    })
}

/// 0 when the move gives a free choice or ends the game; otherwise 1 plus
/// the canonical pattern of the small board the opponent is sent to, seen
/// by the opponent.
fn destination_pattern(board: &Board, mv: Move) -> usize {
    let mut after = *board;
    after.play(mv);
    if after.status() != Status::Ongoing {
        return 0;
    }
    match after.target() {
        None => 0,
        Some(target) => {
            let seat = after.to_move();
            let pattern =
                base3(after.cells(seat, target)) + 2 * base3(after.cells(1 - seat, target));
            let id = pattern_ids()[pattern];
            debug_assert_ne!(id, u32::MAX, "the target board is open");
            1 + id as usize
        }
    }
}

/// The role of `mv`'s own small board: 2 if winning it would win the game
/// for the player, plus 1 if for the opponent; times 3, plus 0 if the move
/// wins the board, 1 if it blocks the opponent's line there, else 2.
fn own_role(board: &Board, mv: Move) -> usize {
    let class = board.move_class(mv) as u8;
    let kind = if class & WINS_BOARD != 0 {
        0
    } else if class & BLOCKS != 0 {
        1
    } else {
        2
    };
    role(board, board.to_move(), mv.board()) * 3 + kind
}
