use cg_search::{Budget, Mcts};

use super::*;
use crate::board::PATTERN_FEATURES;
use crate::nnue::{BUCKETS, FEATURES};

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
    let (mut checked, mut proven) = (0, 0);
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
                assert_eq!(result.proven, Some(true), "seed {seed}: {result:?}");
                let mut next = board;
                next.play(result.best);
                assert_eq!(minimax(&next), -1, "seed {seed}: {result:?}");
                proven += 1;
            }
            -1 => {
                assert_eq!(result.proven, Some(false), "seed {seed}: {result:?}");
                proven += 1;
            }
            _ => assert_eq!(result.proven, None, "seed {seed}: draws are not proven"),
        }
    }
    assert!(
        checked > 100 && proven > 50,
        "{checked} checked, {proven} proven"
    );
}

/// A position searched with a random network of width 16 and a uniform
/// pattern policy.
fn nnue_board(board: Board) -> NnueBoard<16> {
    static NETWORK: std::sync::OnceLock<Network<16>> = std::sync::OnceLock::new();
    static POLICY: std::sync::OnceLock<PatternPolicy> = std::sync::OnceLock::new();
    let network = NETWORK.get_or_init(|| {
        let mut rng = Rng::new(5);
        let mut small = |range: u64| rng.below(2 * range + 1) as i64 - range as i64;
        let feature_weights: Vec<i8> = (0..FEATURES * 16).map(|_| small(20) as i8).collect();
        let feature_bias: Vec<i16> = (0..16).map(|_| small(60) as i16).collect();
        let output_weights: Vec<i8> = (0..BUCKETS * 32).map(|_| small(40) as i8).collect();
        let mut output_bias = [0; BUCKETS];
        for bias in &mut output_bias {
            *bias = small(2_000) as i32;
        }
        Network::from_parts(
            &feature_weights,
            &feature_bias,
            &output_weights,
            &output_bias,
        )
        .unwrap()
    });
    let policy = POLICY
        .get_or_init(|| PatternPolicy::from_log_weights(&vec![0.0; PATTERN_FEATURES]).unwrap());
    NnueBoard {
        board,
        network,
        policy,
    }
}

#[test]
fn nnue_estimates_are_chances_seen_from_seat_0() {
    let mut rng = Rng::new(11);
    let mut list = MoveList::new();
    let mut checked = 0;
    for _ in 0..30 {
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            let position = nnue_board(board);
            let estimate = position.estimate();
            assert!((0.0..=1.0).contains(&estimate), "{estimate}");
            // The same position, with the roles swapped, is the other view.
            let value = if board.game_winning_move().is_some() {
                1.0
            } else {
                f64::from(position.network.evaluate_board(&board))
            };
            let expected = if board.to_move() == 0 {
                value
            } else {
                1.0 - value
            };
            assert_eq!(estimate, expected);
            assert_eq!(position.clone().playout(&mut rng), estimate);
            checked += 1;
            Board::legal_moves(&board, &mut list);
            board.play(*rng.pick(&list).unwrap());
        }
    }
    assert!(checked > 500);
}

#[test]
fn nnue_views_of_the_two_seats_are_complementary() {
    let mut rng = Rng::new(12);
    let mut list = MoveList::new();
    let (mut zero, mut one) = (0, 0);
    let mut board = Board::new();
    while board.status() == Status::Ongoing {
        let position = nnue_board(board);
        let seat = board.to_move();
        // Seat 0's view is the estimate, seat 1's its complement; the side
        // to move's own view is the network's output, a win being 1.
        let views = [position.estimate(), 1.0 - position.estimate()];
        assert_eq!(views[0] + views[1], 1.0);
        let network = f64::from(position.network.evaluate_board(&board));
        let expected = if board.game_winning_move().is_some() {
            1.0
        } else {
            network
        };
        assert_eq!(views[seat], expected);
        if seat == 0 {
            zero += 1;
        } else {
            one += 1;
        }
        Board::legal_moves(&board, &mut list);
        board.play(*rng.pick(&list).unwrap());
    }
    assert!(zero > 5 && one > 5);
}

#[test]
fn nnue_scores_a_game_winning_move_as_a_win_for_the_side_to_move() {
    let positions = positions_with_a_winning_move(40);
    let mut checked = 0;
    // Wins on points, when the last open board closes, are not looked for.
    for (board, _) in positions {
        let Some(winning) = board.game_winning_move() else {
            continue;
        };
        let mut next = board;
        next.play(winning);
        assert_eq!(next.status(), Status::Win(board.to_move()));
        checked += 1;
        let estimate = nnue_board(board).estimate();
        let expected = if board.to_move() == 0 { 1.0 } else { 0.0 };
        assert_eq!(estimate, expected);
    }
    assert!(checked > 20, "only {checked} positions");
}

#[test]
fn nnue_positions_are_equal_when_their_boards_are() {
    let mut board = Board::new();
    let a = nnue_board(board);
    assert!(a == nnue_board(board));
    board.play(Move::new(4, 4));
    assert!(a != nnue_board(board));
}

#[test]
fn a_search_with_the_nnue_returns_a_legal_move() {
    let mut rng = Rng::new(13);
    let mut list = MoveList::new();
    let mut mcts = Mcts::<NnueBoard<16>>::new(0.5, 3);
    mcts.priors = true;
    let mut board = Board::new();
    while board.status() == Status::Ongoing {
        Board::legal_moves(&board, &mut list);
        let result = mcts.search(&nnue_board(board), &list, Budget::Iterations(300));
        assert!(list.contains(&result.best), "{result:?}");
        board.play(result.best);
        // A random reply keeps the tree's reuse in play.
        if board.status() == Status::Ongoing {
            Board::legal_moves(&board, &mut list);
            board.play(*rng.pick(&list).unwrap());
        }
    }
}

#[test]
fn nnue_child_scores_are_the_children_estimates() {
    let mut rng = Rng::new(14);
    let mut list = MoveList::new();
    let mut scores = Vec::new();
    let (mut estimated, mut over) = (0, 0);
    for _ in 0..30 {
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            Board::legal_moves(&board, &mut list);
            let position = nnue_board(board);
            position.child_scores(&list, &mut rng, &mut scores);
            assert_eq!(scores.len(), list.len());
            for (&mv, &(score, ended)) in list.iter().zip(&scores) {
                let mut child = position;
                child.play(mv);
                match child.score() {
                    Some(exact) => {
                        assert_eq!((score, ended), (exact, true));
                        over += 1;
                    }
                    None => {
                        // Incremental updates give the scratch estimate exactly.
                        assert_eq!((score, ended), (child.estimate(), false));
                        estimated += 1;
                    }
                }
            }
            board.play(*rng.pick(&list).unwrap());
        }
    }
    assert!(
        estimated > 5_000 && over > 20,
        "{estimated} estimated, {over} over"
    );
}

/// A search that expands all children at once, judging them by their
/// minimax values only.
fn jacekmax(seed: u64) -> Mcts<NnueBoard<16>> {
    let mut mcts = Mcts::new(0.3, seed);
    mcts.expand_all = true;
    mcts.minimax_weight = 1.0;
    mcts
}

#[test]
fn jacekmax_proofs_agree_with_exhaustive_search_in_endgames() {
    let mut list = MoveList::new();
    let (mut checked, mut proven) = (0, 0);
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
        let result = jacekmax(seed).search(&nnue_board(board), &list, Budget::Iterations(20_000));
        checked += 1;
        match exact {
            1 => {
                assert_eq!(result.proven, Some(true), "seed {seed}: {result:?}");
                let mut next = board;
                next.play(result.best);
                assert_eq!(minimax(&next), -1, "seed {seed}: {result:?}");
                proven += 1;
            }
            -1 => {
                assert_eq!(result.proven, Some(false), "seed {seed}: {result:?}");
                proven += 1;
            }
            _ => assert_eq!(result.proven, None, "seed {seed}: draws are not proven"),
        }
    }
    assert!(
        checked > 100 && proven > 50,
        "{checked} checked, {proven} proven"
    );
}

#[test]
fn a_jacekmax_search_with_the_nnue_returns_a_legal_move() {
    let mut rng = Rng::new(15);
    let mut list = MoveList::new();
    let mut mcts = jacekmax(4);
    let mut board = Board::new();
    while board.status() == Status::Ongoing {
        Board::legal_moves(&board, &mut list);
        let result = mcts.search(&nnue_board(board), &list, Budget::Iterations(300));
        assert!(list.contains(&result.best), "{result:?}");
        assert!((0.0..=1.0).contains(&result.expected_score), "{result:?}");
        board.play(result.best);
        if board.status() == Status::Ongoing {
            Board::legal_moves(&board, &mut list);
            board.play(*rng.pick(&list).unwrap());
        }
    }
}
