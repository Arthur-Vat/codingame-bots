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

const fn completing_table() -> [u16; 512] {
    let table = line_table();
    let mut completing = [0; 512];
    let mut mask = 0;
    while mask < 512 {
        let mut cell = 0;
        while cell < 9 {
            let with = mask | (1 << cell);
            if with != mask && table[with] {
                completing[mask] |= 1 << cell;
            }
            cell += 1;
        }
        mask += 1;
    }
    completing
}

/// `COMPLETING[mask]`: the cells outside `mask` that would give it a line.
static COMPLETING: [u16; 512] = completing_table();

/// The cells outside `mask` that, added to it, would make a whole line.
/// Whether those cells are free is up to the caller.
#[inline]
pub fn completing_cells(mask: u16) -> u16 {
    COMPLETING[usize::from(mask & FULL)]
}

const fn cell_tables() -> ([u8; 512], [[u8; 9]; 512]) {
    let mut counts = [0; 512];
    let mut nth = [[0; 9]; 512];
    let mut mask = 0;
    while mask < 512 {
        let mut cell = 0;
        while cell < 9 {
            if mask & (1 << cell) != 0 {
                nth[mask][counts[mask] as usize] = cell as u8;
                counts[mask] += 1;
            }
            cell += 1;
        }
        mask += 1;
    }
    (counts, nth)
}

/// `CELL_TABLES.0[mask]`: the cells in `mask`; `CELL_TABLES.1[mask][n]`:
/// its `n`-th cell from the lowest. Tables, because CodinGame's compiler
/// targets processors without a population-count instruction.
static CELL_TABLES: ([u8; 512], [[u8; 9]; 512]) = cell_tables();

/// The number of cells in `mask`.
#[inline]
pub fn count(mask: u16) -> u32 {
    u32::from(CELL_TABLES.0[usize::from(mask & FULL)])
}

/// The `n`-th cell of `mask`, counting from the lowest and from 0. `mask`
/// must have more than `n` cells.
#[inline]
pub fn nth_cell(mask: u16, n: u32) -> usize {
    debug_assert!(n < count(mask), "fewer than {} cells", n + 1);
    usize::from(CELL_TABLES.1[usize::from(mask & FULL)][n as usize])
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
