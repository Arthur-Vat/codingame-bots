//! A set of the 144 cells of the map, as a bitboard.
//!
//! Cell `y * 12 + x` is bit `cell % 48` of limb `cell / 48`: three limbs of
//! 48 bits, each holding four whole rows, so that shifting by a row or a
//! column is a few word operations. Bits above 143 are always 0.

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

/// Cells per row and per column.
pub const SIZE: usize = 12;
/// Number of cells.
pub const CELLS: usize = SIZE * SIZE;

/// Bits per limb: four rows.
const LIMB: usize = 4 * SIZE;
/// The 48 low bits.
const MASK: u64 = (1 << LIMB) - 1;
/// Column 0 of the four rows of a limb.
const COLUMN_0: u64 = 1 | 1 << 12 | 1 << 24 | 1 << 36;
/// Column 11 of the four rows of a limb.
const COLUMN_11: u64 = COLUMN_0 << 11;
/// The first row of a limb.
const ROW_0: u64 = (1 << SIZE) - 1;

/// A set of cells.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Bits([u64; 3]);

impl Bits {
    /// No cell.
    pub const EMPTY: Bits = Bits([0; 3]);
    /// Every cell of the map.
    pub const ALL: Bits = Bits([MASK; 3]);

    /// The set of one cell.
    #[inline]
    pub fn cell(cell: usize) -> Bits {
        let mut bits = Bits::EMPTY;
        bits.set(cell);
        bits
    }

    /// Whether `cell` is in the set.
    #[inline]
    pub fn test(self, cell: usize) -> bool {
        (self.0[cell / LIMB] >> (cell % LIMB)) & 1 != 0
    }

    /// Adds `cell`.
    #[inline]
    pub fn set(&mut self, cell: usize) {
        self.0[cell / LIMB] |= 1 << (cell % LIMB);
    }

    /// Removes `cell`.
    #[inline]
    pub fn clear(&mut self, cell: usize) {
        self.0[cell / LIMB] &= !(1 << (cell % LIMB));
    }

    /// Whether the set is empty.
    #[inline]
    pub fn is_empty(self) -> bool {
        self.0[0] | self.0[1] | self.0[2] == 0
    }

    /// The number of cells.
    #[inline]
    pub fn count(self) -> u32 {
        self.0[0].count_ones() + self.0[1].count_ones() + self.0[2].count_ones()
    }

    /// The cells of `self` that are not in `other`.
    #[inline]
    pub fn and_not(self, other: Bits) -> Bits {
        Bits([
            self.0[0] & !other.0[0],
            self.0[1] & !other.0[1],
            self.0[2] & !other.0[2],
        ])
    }

    /// The cells in order of index.
    #[inline]
    pub fn iter(self) -> BitIter {
        BitIter {
            limbs: self.0,
            at: 0,
        }
    }

    /// The cell with `n` cells before it (counting from 0), if there is one.
    pub fn nth(self, mut n: usize) -> Option<usize> {
        for (index, &limb) in self.0.iter().enumerate() {
            let count = limb.count_ones() as usize;
            if n < count {
                let mut rest = limb;
                for _ in 0..n {
                    rest &= rest - 1;
                }
                return Some(index * LIMB + rest.trailing_zeros() as usize);
            }
            n -= count;
        }
        None
    }

    /// Each cell moved one row up (`y - 1`); cells in row 0 leave the map.
    #[inline]
    pub fn shift_up(self) -> Bits {
        let [a, b, c] = self.0;
        Bits([
            (a >> SIZE) | ((b & ROW_0) << (LIMB - SIZE)),
            (b >> SIZE) | ((c & ROW_0) << (LIMB - SIZE)),
            c >> SIZE,
        ])
    }

    /// Each cell moved one row down (`y + 1`); cells in row 11 leave the map.
    #[inline]
    pub fn shift_down(self) -> Bits {
        let [a, b, c] = self.0;
        Bits([
            (a << SIZE) & MASK,
            ((b << SIZE) & MASK) | (a >> (LIMB - SIZE)),
            ((c << SIZE) & MASK) | (b >> (LIMB - SIZE)),
        ])
    }

    /// Each cell moved one column left (`x - 1`); cells in column 0 leave.
    #[inline]
    pub fn shift_left(self) -> Bits {
        let [a, b, c] = self.0;
        Bits([
            (a >> 1) & !COLUMN_11 & MASK,
            (b >> 1) & !COLUMN_11 & MASK,
            (c >> 1) & !COLUMN_11 & MASK,
        ])
    }

    /// Each cell moved one column right (`x + 1`); cells in column 11 leave.
    #[inline]
    pub fn shift_right(self) -> Bits {
        let [a, b, c] = self.0;
        Bits([
            (a << 1) & !COLUMN_0 & MASK,
            (b << 1) & !COLUMN_0 & MASK,
            (c << 1) & !COLUMN_0 & MASK,
        ])
    }

    /// The cells that are a neighbour (sharing a side) of a cell of the set.
    /// Void cells are not excluded, and a cell of the set is in the result
    /// when another cell of the set is next to it.
    #[inline]
    pub fn neighbours(self) -> Bits {
        self.shift_up() | self.shift_down() | self.shift_left() | self.shift_right()
    }
}

impl BitAnd for Bits {
    type Output = Bits;

    #[inline]
    fn bitand(self, other: Bits) -> Bits {
        Bits([
            self.0[0] & other.0[0],
            self.0[1] & other.0[1],
            self.0[2] & other.0[2],
        ])
    }
}

impl BitAndAssign for Bits {
    #[inline]
    fn bitand_assign(&mut self, other: Bits) {
        *self = *self & other;
    }
}

impl BitOr for Bits {
    type Output = Bits;

    #[inline]
    fn bitor(self, other: Bits) -> Bits {
        Bits([
            self.0[0] | other.0[0],
            self.0[1] | other.0[1],
            self.0[2] | other.0[2],
        ])
    }
}

impl BitOrAssign for Bits {
    #[inline]
    fn bitor_assign(&mut self, other: Bits) {
        *self = *self | other;
    }
}

impl Not for Bits {
    type Output = Bits;

    /// The cells of the map that are not in the set.
    #[inline]
    fn not(self) -> Bits {
        Bits::ALL.and_not(self)
    }
}

/// The cells of a [`Bits`], lowest index first.
#[derive(Clone, Debug)]
pub struct BitIter {
    limbs: [u64; 3],
    at: usize,
}

impl Iterator for BitIter {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        while self.at < 3 {
            let limb = self.limbs[self.at];
            if limb != 0 {
                self.limbs[self.at] = limb & (limb - 1);
                return Some(self.at * LIMB + limb.trailing_zeros() as usize);
            }
            self.at += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests;
