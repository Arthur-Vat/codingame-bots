use super::*;

#[test]
fn every_line_is_three_cells_in_a_row() {
    for line in LINES {
        assert_eq!(line.count_ones(), 3);
        let cells: Vec<(usize, usize)> = cells(line).map(|cell| (cell / 3, cell % 3)).collect();
        let same_row = cells.iter().all(|&(row, _)| row == cells[0].0);
        let same_col = cells.iter().all(|&(_, col)| col == cells[0].1);
        let diagonal = cells.iter().all(|&(row, col)| row == col);
        let anti_diagonal = cells.iter().all(|&(row, col)| row + col == 2);
        assert!(
            same_row || same_col || diagonal || anti_diagonal,
            "{line:09b}"
        );
    }
}

#[test]
fn has_line_matches_a_direct_check() {
    for mask in 0..512u16 {
        let direct = LINES.iter().any(|&line| (mask & line).count_ones() == 3);
        assert_eq!(has_line(mask), direct, "{mask:09b}");
    }
    // Bits above the ninth are ignored.
    assert!(has_line(0b1_000_000_000_111 & 0x7FFF));
    assert!(!has_line(0b1110_0000_0000));
}

#[test]
fn iterates_cells_in_order() {
    assert_eq!(cells(0).count(), 0);
    assert_eq!(cells(0b100_010_001).collect::<Vec<_>>(), vec![0, 4, 8]);
    assert_eq!(cells(FULL).count(), 9);
}

#[test]
fn nth_cell_counts_set_bits_from_the_lowest() {
    let mask = 0b1_0010_0110;
    let all: Vec<usize> = cells(mask).collect();
    for (n, &cell) in all.iter().enumerate() {
        assert_eq!(nth_cell(mask, n as u32), cell);
    }
    assert_eq!(nth_cell(FULL, 8), 8);
}

#[test]
fn count_matches_the_number_of_set_bits() {
    for mask in 0..512u16 {
        assert_eq!(count(mask), mask.count_ones(), "{mask:#b}");
    }
}
