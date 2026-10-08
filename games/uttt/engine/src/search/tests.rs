use cg_search::{Budget, Mcts, Outcome};

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

/// The exact value of `board` for the player to move: 1 for a win, 0 for
/// a draw, -1 for a loss, by trying every line of play.
fn minimax(board: &Board) -> i32 {
    match board.status() {
        Status::Win(seat) if seat == board.to_move() => 1,
        Status::Win(_) => -1,
        Status::Draw => 0,
        Status::Ongoing => {
            let mut moves = MoveList::new();
            Board::legal_moves(board, &mut moves);
            let mut best = -1;
            for &mv in moves.iter() {
                let mut next = *board;
                next.play(mv);
                best = best.max(-minimax(&next));
                if best == 1 {
                    break;
                }
            }
            best
        }
    }
}

/// Empty cells left in small boards that are still open.
fn empty_open_cells(board: &Board) -> usize {
    (0..9)
        .filter(|&small| !board.is_closed(small))
        .map(|small| {
            (0..9)
                .filter(|&cell| board.mark(Move::new(small, cell)).is_none())
                .count()
        })
        .sum()
}

#[test]
fn proofs_agree_with_exhaustive_search_in_endgames() {
    let mut list = MoveList::new();
    let (mut checked, mut proven, mut draws, mut proven_draws) = (0, 0, 0, 0);
    for seed in 0..400 {
        let mut rng = Rng::new(seed);
        let mut board = Board::new();
        while board.status() == Status::Ongoing && empty_open_cells(&board) > 7 {
            Board::legal_moves(&board, &mut list);
            board.play(*rng.pick(&list).unwrap());
        }
        if board.status() != Status::Ongoing {
            continue;
        }
        Board::legal_moves(&board, &mut list);
        let exact = minimax(&board);
        let result = Mcts::new(1.0, seed).search(&board, &list, Budget::Iterations(20_000));
        checked += 1;
        match exact {
            1 => {
                // A win is proven, and the move played keeps it.
                assert_eq!(result.proven, Some(Outcome::Win), "seed {seed}: {result:?}");
                let mut next = board;
                next.play(result.best);
                assert_eq!(minimax(&next), -1, "seed {seed}: {result:?}");
                proven += 1;
            }
            -1 => {
                assert_eq!(
                    result.proven,
                    Some(Outcome::Loss),
                    "seed {seed}: {result:?}"
                );
                proven += 1;
            }
            _ => {
                // A draw is proven only once every other answer is too,
                // and a search starves those of visits next to a proven
                // draw; when it is proven, it is right. The move played
                // keeps the draw.
                if let Some(outcome) = result.proven {
                    assert_eq!(outcome, Outcome::Draw, "seed {seed}: {result:?}");
                    proven_draws += 1;
                }
                let mut next = board;
                next.play(result.best);
                assert_eq!(minimax(&next), 0, "seed {seed}: {result:?}");
                draws += 1;
            }
        }
    }
    eprintln!("{checked} checked, {proven} won or lost, {proven_draws} of {draws} draws proven");
    assert!(
        checked > 100 && proven + draws == checked && draws > 5 && proven_draws > 0,
        "{checked} checked, {proven} won or lost, {proven_draws} of {draws} draws proven"
    );
}
