use cg_search::{Budget, Mcts};

use super::*;

#[test]
fn scores_follow_the_status() {
    assert_eq!(score(Status::Ongoing), None);
    assert_eq!(score(Status::Win(0)), Some(1.0));
    assert_eq!(score(Status::Win(1)), Some(0.0));
    assert_eq!(score(Status::Draw), Some(0.5));
}

#[test]
fn the_game_view_matches_the_board() {
    let mut rng = Rng::new(3);
    let mut board = Board::new();
    let mut list = MoveList::new();
    let mut moves = Vec::new();
    while board.status() == Status::Ongoing {
        Board::legal_moves(&board, &mut list);
        Game::legal_moves(&board, &mut moves);
        assert_eq!(moves, list.to_vec());
        assert_eq!(Game::to_move(&board), board.to_move());
        let mv = *rng.pick(&moves).unwrap();
        Game::play(&mut board, mv);
    }
    assert!(Game::score(&board).is_some());
}

/// Every position of random games where the player to move can win the
/// game at once, with its legal moves.
fn positions_with_a_winning_move(games: u64) -> Vec<(Board, Vec<Move>)> {
    let mut found = Vec::new();
    let mut list = MoveList::new();
    for seed in 0..games {
        let mut rng = Rng::new(seed);
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            Board::legal_moves(&board, &mut list);
            let wins = list.iter().any(|&mv| {
                let mut next = board;
                next.play(mv);
                next.status() == Status::Win(board.to_move())
            });
            if wins {
                found.push((board, list.to_vec()));
            }
            board.play(*rng.pick(&list).unwrap());
        }
    }
    found
}

#[test]
fn mcts_takes_a_winning_move() {
    let positions = positions_with_a_winning_move(40);
    assert!(positions.len() > 20, "only {} positions", positions.len());
    let mut mcts = Mcts::new(1.0, 1);
    for (board, moves) in positions {
        let result = mcts.search(&board, &moves, Budget::Iterations(2_000));
        let mut next = board;
        next.play(result.best);
        assert_eq!(next.status(), Status::Win(board.to_move()), "{result:?}");
    }
}
