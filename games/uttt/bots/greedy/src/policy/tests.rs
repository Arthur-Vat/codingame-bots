use cg_core::rng::Rng;

use super::*;

/// Every position of `games` random games, with the legal moves there.
fn random_positions(games: u64) -> Vec<(Board, Vec<Move>)> {
    let mut positions = Vec::new();
    let mut moves = MoveList::new();
    for seed in 0..games {
        let mut rng = Rng::new(seed);
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            board.legal_moves(&mut moves);
            positions.push((board, moves.to_vec()));
            board.play(*rng.pick(&moves).unwrap());
        }
    }
    positions
}

fn wins_game(board: &Board, mv: Move) -> bool {
    let mut after = *board;
    after.play(mv);
    after.status() == Status::Win(board.to_move())
}

/// Whether the opponent wins the game with `mv` or right after it (by
/// points when the board fills up, or by a line of small boards).
fn allows_a_game_win(board: &Board, mv: Move) -> bool {
    let mut after = *board;
    after.play(mv);
    if after.status() == Status::Win(1 - board.to_move()) {
        return true;
    }
    let mut replies = MoveList::new();
    after.legal_moves(&mut replies);
    replies.iter().any(|&reply| wins_game(&after, reply))
}

fn best_score(board: &Board, moves: &[Move]) -> i32 {
    moves.iter().map(|&mv| score(board, mv)).max().unwrap()
}

#[test]
fn a_winning_move_gets_the_top_score() {
    let mut checked = 0;
    for (board, moves) in random_positions(300) {
        for &mv in &moves {
            if wins_game(&board, mv) {
                assert_eq!(score(&board, mv), WIN_GAME);
                checked += 1;
            }
        }
    }
    assert!(checked > 50, "only {checked} winning moves seen");
}

#[test]
fn never_lets_the_opponent_win_when_it_can_avoid_it() {
    let mut checked = 0;
    for (board, moves) in random_positions(300) {
        if moves.iter().any(|&mv| wins_game(&board, mv)) {
            continue;
        }
        let safe: Vec<Move> = moves
            .iter()
            .copied()
            .filter(|&mv| !allows_a_game_win(&board, mv))
            .collect();
        if safe.is_empty() || safe.len() == moves.len() {
            continue;
        }
        checked += 1;
        let best = best_score(&board, &moves);
        for &mv in &moves {
            if score(&board, mv) == best {
                assert!(safe.contains(&mv), "{mv} lets the opponent win");
            }
        }
    }
    assert!(checked > 50, "only {checked} positions with a trap seen");
}

/// The position after `moves`, given as `(board, cell)`.
fn position(moves: &[(usize, usize)]) -> Board {
    let mut board = Board::new();
    for &(b, c) in moves {
        let mv = Move::new(b, c);
        assert!(board.is_legal(mv), "{mv}");
        board.play(mv);
    }
    board
}

/// Seat 0 holds cells 0 and 1 of board 1; seat 1 is about to move in
/// board 3, where cell 1 would send seat 0 to board 1.
const BEFORE_THE_THREAT: [(usize, usize); 5] = [(1, 0), (0, 1), (1, 1), (1, 5), (5, 3)];

#[test]
fn prefers_winning_a_small_board() {
    let mut moves = BEFORE_THE_THREAT.to_vec();
    moves.push((3, 1));
    let board = position(&moves);
    assert_eq!((board.to_move(), board.target()), (0, Some(1)));
    let win = Move::new(1, 2);
    let mut legal = MoveList::new();
    board.legal_moves(&mut legal);
    for &mv in legal.iter().filter(|&&mv| mv != win) {
        assert!(
            score(&board, win) > score(&board, mv) + WIN_BOARD / 2,
            "{mv}"
        );
    }
}

#[test]
fn avoids_sending_the_opponent_where_it_wins_a_board() {
    let board = position(&BEFORE_THE_THREAT);
    assert_eq!((board.to_move(), board.target()), (1, Some(3)));
    let gift = Move::new(3, 1);
    let mut legal = MoveList::new();
    board.legal_moves(&mut legal);
    for &mv in legal.iter().filter(|&&mv| mv != gift) {
        assert!(score(&board, gift) < score(&board, mv), "{mv}");
    }
}
