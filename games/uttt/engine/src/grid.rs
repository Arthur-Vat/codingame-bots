//! 3×3 grids as 9-bit masks: bit `3 * row + col` is the cell at
//! `(row, col)`. Used for small boards and for the main board alike.

/// All nine cells.
pub const FULL: u16 = 0x1FF;

/// The 8 lines of a 3×3 grid: 3 rows, 3 columns, 2 diagonals.
pub const LINES: [u16; 8] = [
    0b000_000_111,
    0b000_111_000,
    0b111_000_000,
    0b001_001_001,
    0b010_010_010,
    0b100_100_100,
    0b100_010_001,
    0b001_010_100,
];

const fn line_table() -> [bool; 512] {
    let mut table = [false; 512];
    let mut mask = 0;
    while mask < 512 {
        let mut line = 0;
        while line < LINES.len() {
            if mask as u16 & LINES[line] == LINES[line] {
                table[mask] = true;
            }
            line += 1;
        }
        mask += 1;
    }
    table
}

/// `HAS_LINE[mask]` is true when `mask` contains a whole line.
static HAS_LINE: [bool; 512] = line_table();

/// Whether the cells in `mask` contain a whole line.
#[inline]
pub fn has_line(mask: u16) -> bool {
    HAS_LINE[usize::from(mask & FULL)]
}

/// Iterates over the cells set in `mask`, lowest first.
#[inline]
pub fn cells(mut mask: u16) -> impl Iterator<Item = usize> {
    std::iter::from_fn(move || {
        if mask == 0 {
            None
        } else {
            let cell = mask.trailing_zeros() as usize;
            mask &= mask - 1;
            Some(cell)
        }
    })
}

#[cfg(test)]
mod tests;
