use super::*;

fn at(row: usize, col: usize) -> Move {
    Move::from_row_col(row, col).unwrap()
}

fn legal(board: &Board) -> Vec<Move> {
    let mut moves = MoveList::new();
    board.legal_moves(&mut moves);
    moves.to_vec()
}

/// A position built directly from marks, for tests. Winners, closed boards
/// and the status are derived from the marks with the rules spelled out.
fn position(marks: &[(Move, usize)], to_move: usize, target: Option<usize>) -> Board {
    let mut board = Board::new();
    for &(mv, seat) in marks {
        board.marks[seat][mv.board()] |= 1 << mv.cell();
    }
    for small in 0..9 {
        for seat in 0..2 {
            if grid::has_line(board.marks[seat][small]) {
                board.won[seat] |= 1 << small;
            }
        }
        if board.small_winner(small).is_some() || board.empty_cells(small) == 0 {
            board.closed |= 1 << small;
        }
    }
    board.to_move = to_move as u8;
    board.target = match target {
        Some(small) if !board.is_closed(small) => small as u8,
        _ => ANY_BOARD,
    };
    board.status = if let Some(seat) = (0..2).find(|&seat| grid::has_line(board.won[seat])) {
        Status::Win(seat)
    } else if board.closed == FULL {
        let [zero, one] = board.points();
        match zero.cmp(&one) {
            std::cmp::Ordering::Greater => Status::Win(0),
            std::cmp::Ordering::Less => Status::Win(1),
            std::cmp::Ordering::Equal => Status::Draw,
        }
    } else {
        Status::Ongoing
    };
    board
}

/// Marks winning small board `small` for `seat` with its top row.
fn won_board(small: usize, seat: usize) -> Vec<(Move, usize)> {
    (0..3).map(|cell| (Move::new(small, cell), seat)).collect()
}

/// Marks filling small board `small` with no line: X O X / X O O / O X X.
fn drawn_board(small: usize) -> Vec<(Move, usize)> {
    [0, 1, 0, 0, 1, 1, 1, 0, 0]
        .iter()
        .enumerate()
        .map(|(cell, &seat)| (Move::new(small, cell), seat))
        .collect()
}

#[test]
fn the_first_move_may_go_anywhere() {
    let board = Board::new();
    assert_eq!(legal(&board).len(), 81);
    assert_eq!(board.target(), None);
    assert_eq!(board.to_move(), 0);
    assert_eq!(board.status(), Status::Ongoing);
}

#[test]
fn a_move_sends_the_opponent_to_the_matching_small_board() {
    let mut board = Board::new();
    board.play(at(4, 4));
    assert_eq!(board.target(), Some(4));
    assert_eq!(board.to_move(), 1);
    assert_eq!(board.mark(at(4, 4)), Some(0));
    let moves = legal(&board);
    assert_eq!(moves.len(), 8);
    assert!(moves.iter().all(|mv| mv.board() == 4 && *mv != at(4, 4)));

    board.play(at(3, 5)); // top-right cell of the centre board
    assert_eq!(board.target(), Some(2));
    assert!(legal(&board).iter().all(|mv| mv.board() == 2));
}

#[test]
fn a_won_board_is_closed_and_frees_whoever_is_sent_there() {
    let mut marks = won_board(0, 0);
    marks.push((at(3, 0), 1));
    // (3, 0) is cell 0 of board 3: the player to move is sent to board 0.
    let board = position(&marks, 0, Some(0));
    assert_eq!(board.target(), None);
    let moves = legal(&board);
    assert_eq!(moves.len(), 81 - 9 - 1);
    assert!(moves.iter().all(|mv| mv.board() != 0));
    assert!(
        !board.is_legal(Move::new(0, 5)),
        "empty cell of a won board"
    );
}

#[test]
fn a_full_board_without_line_belongs_to_nobody() {
    let board = position(&drawn_board(8), 0, Some(8));
    assert!(board.is_closed(8));
    assert_eq!(board.small_winner(8), None);
    assert_eq!(board.points(), [0, 0]);
    assert_eq!(legal(&board).len(), 72);
}

#[test]
fn completing_a_board_by_filling_it_closes_it() {
    // Board 0 has 8 cells marked with no line; the last one fills it.
    let mut marks = drawn_board(0);
    let last = marks.pop().unwrap();
    let mut board = position(&marks, last.1, Some(0));
    assert_eq!(legal(&board), vec![last.0]);
    board.play(last.0);
    assert!(board.is_closed(0));
    assert_eq!(board.small_winner(0), None);
    // The last cell is cell 8, so the next player is sent to board 8.
    assert_eq!(board.target(), Some(8));
}

#[test]
fn winning_a_small_board_scores_and_three_in_a_row_wins_the_game() {
    let mut marks = won_board(0, 1);
    marks.extend(won_board(4, 1));
    marks.extend([(Move::new(8, 0), 1), (Move::new(8, 1), 1)]);
    let mut board = position(&marks, 1, Some(8));
    assert_eq!(board.points(), [0, 2]);
    board.play(Move::new(8, 2));
    assert_eq!(board.small_winner(8), Some(1));
    assert_eq!(board.status(), Status::Win(1));
    assert!(legal(&board).is_empty());
    assert!(!board.is_legal(Move::new(5, 5)));
}

/// Eight small boards won with no main line; board 8 is left to each test.
///
/// ```text
/// 0 1 0
/// 0 1 1
/// 1 0 .
/// ```
fn eight_boards_won() -> Vec<(Move, usize)> {
    [0, 1, 0, 0, 1, 1, 1, 0]
        .iter()
        .enumerate()
        .flat_map(|(small, &seat)| won_board(small, seat))
        .collect()
}

#[test]
fn when_every_board_is_closed_more_boards_wins() {
    let mut marks = eight_boards_won();
    marks.extend([(Move::new(8, 0), 0), (Move::new(8, 1), 0)]);
    let mut board = position(&marks, 0, Some(8));
    board.play(Move::new(8, 2));
    assert_eq!(board.points(), [5, 4]);
    assert_eq!(board.status(), Status::Win(0));
}

#[test]
fn when_every_board_is_closed_equal_points_is_a_draw() {
    let mut marks = eight_boards_won();
    let mut drawn = drawn_board(8);
    let last = drawn.pop().unwrap();
    marks.extend(drawn);
    let mut board = position(&marks, last.1, Some(8));
    board.play(last.0);
    assert_eq!(board.points(), [4, 4]);
    assert_eq!(board.status(), Status::Draw);
}

#[test]
fn random_games_keep_every_invariant() {
    let mut rng = Rng::new(11);
    let mut moves = MoveList::new();
    for _ in 0..2000 {
        let mut board = Board::new();
        let mut played = 0;
        while board.status() == Status::Ongoing {
            board.legal_moves(&mut moves);
            assert!(!moves.is_empty());
            // Exactly the listed moves are legal.
            let listed: std::collections::HashSet<Move> = moves.iter().copied().collect();
            for index in 0..81 {
                let mv = Move::new(index / 9, index % 9);
                assert_eq!(
                    board.is_legal(mv),
                    listed.contains(&mv),
                    "{mv} in {board:?}"
                );
            }
            for mv in moves.iter() {
                assert_eq!(board.mark(*mv), None);
                assert!(!board.is_closed(mv.board()));
                if let Some(target) = board.target() {
                    assert_eq!(mv.board(), target);
                }
            }
            let mv = moves[rng.below(moves.len() as u64) as usize];
            board.play(mv);
            played += 1;
            assert_eq!(board.mark(mv), Some(1 - board.to_move()));
            assert!(played <= 81);
        }
        let points = board.points();
        let won: u32 = (0..9)
            .filter(|&small| board.small_winner(small).is_some())
            .count() as u32;
        assert_eq!(points[0] + points[1], won);
        board.legal_moves(&mut moves);
        assert!(moves.is_empty());
    }
}

#[test]
fn random_playouts_end_and_are_reproducible() {
    let mut first = Rng::new(5);
    let mut second = Rng::new(5);
    for _ in 0..100 {
        let (mut a, mut b) = (Board::new(), Board::new());
        let result = a.random_playout(&mut first);
        assert_ne!(result, Status::Ongoing);
        assert_eq!(result, b.random_playout(&mut second));
        assert_eq!(a, b);
    }
}

#[test]
fn a_random_move_is_the_listed_move_the_same_draw_would_pick() {
    let mut moves = MoveList::new();
    for seed in 0..300 {
        let mut game_rng = Rng::new(seed);
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            board.legal_moves(&mut moves);
            let mut listed = game_rng.clone();
            let mut direct = game_rng.clone();
            let expected = moves[listed.below(moves.len() as u64) as usize];
            assert_eq!(board.random_move(&mut direct), expected, "seed {seed}");
            assert_eq!(listed.next_u64(), direct.next_u64(), "same draws");
            board.play(*game_rng.pick(&moves).unwrap());
        }
    }
}

/// Whether `mv` wins the game for the player to move with a line of small
/// boards.
fn wins_by_line(board: &Board, mv: Move) -> bool {
    let seat = board.to_move();
    let mut next = *board;
    next.play(mv);
    grid::has_line(next.won[seat])
}

#[test]
fn a_decisive_move_wins_when_it_can_and_is_random_otherwise() {
    let (mut wins, mut plain) = (0, 0);
    for seed in 0..300 {
        let mut game_rng = Rng::new(seed);
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            let moves = legal(&board);
            let mut draw = game_rng.clone();
            let chosen = board.decisive_move(&mut draw);
            assert!(moves.contains(&chosen), "{chosen} in {board:?}");
            let winning: Vec<Move> = moves
                .iter()
                .copied()
                .filter(|&mv| wins_by_line(&board, mv))
                .collect();
            if winning.is_empty() {
                let mut same = game_rng.clone();
                assert_eq!(chosen, board.random_move(&mut same));
                assert_eq!(draw.next_u64(), same.next_u64(), "same draws");
                plain += 1;
            } else {
                assert!(winning.contains(&chosen), "{chosen} in {board:?}");
                wins += 1;
            }
            board.play(*game_rng.pick(&moves).unwrap());
        }
    }
    assert!(wins > 500 && plain > 5000, "{wins} wins, {plain} plain");
}

#[test]
fn decisive_playouts_end_and_are_reproducible() {
    let mut first = Rng::new(9);
    let mut second = Rng::new(9);
    for _ in 0..100 {
        let (mut a, mut b) = (Board::new(), Board::new());
        let result = a.decisive_playout(&mut first);
        assert_ne!(result, Status::Ongoing);
        assert_eq!(result, b.decisive_playout(&mut second));
        assert_eq!(a, b);
    }
}
