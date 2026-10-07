use super::*;

#[test]
fn converts_between_coordinates_and_board_cell() {
    for row in 0..9 {
        for col in 0..9 {
            let mv = Move::from_row_col(row, col).unwrap();
            assert_eq!(mv.row_col(), (row, col));
            assert_eq!(mv.board(), 3 * (row / 3) + col / 3);
            assert_eq!(mv.cell(), 3 * (row % 3) + col % 3);
        }
    }
    assert_eq!(Move::from_row_col(9, 0), None);
    assert_eq!(Move::from_row_col(0, 9), None);
    // Bottom-left cell of the centre board sends to the bottom-left board.
    let mv = Move::from_row_col(5, 3).unwrap();
    assert_eq!((mv.board(), mv.cell()), (4, 6));
    assert_eq!(mv.to_string(), "5 3");
}

#[test]
#[should_panic(expected = "no cell 9")]
fn new_rejects_out_of_range_indices() {
    Move::new(0, 9);
}

#[test]
fn move_lists_hold_up_to_81_moves() {
    let mut list = MoveList::new();
    assert!(list.is_empty());
    for index in 0..81 {
        list.push(Move::new(index / 9, index % 9));
    }
    assert_eq!(list.len(), 81);
    assert_eq!(list[80], Move::new(8, 8));
    list.clear();
    assert!(list.is_empty());
}
