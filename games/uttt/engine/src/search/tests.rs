use cg_search::{Budget, Mcts};

use crate::value::ValueNetwork;

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

/// A network with random weights, for the searches below.
fn random_network(seed: u64) -> &'static ValueNetwork {
    let mut rng = Rng::new(seed);
    let parameters: Vec<f32> = (0..ValueNetwork::PARAMETERS)
        .map(|_| ((rng.unit() * 2.0 - 1.0) * 0.3) as f32)
        .collect();
    Box::leak(Box::new(
        ValueNetwork::from_parameters(&parameters).unwrap(),
    ))
}

#[test]
fn value_boards_score_wins_in_one_and_ask_the_network_otherwise() {
    let network = random_network(17);
    // Wins on points, when the last open board closes, are left to the
    // network, like any position without a line to complete.
    let positions: Vec<Board> = positions_with_a_winning_move(20)
        .into_iter()
        .map(|(board, _)| board)
        .filter(|board| board.game_winning_move().is_some())
        .collect();
    assert!(positions.len() > 10, "only {} positions", positions.len());
    for board in positions {
        let estimate = ValueBoard { board, network }.estimate();
        assert_eq!(estimate, if board.to_move() == 0 { 1.0 } else { 0.0 });
    }
    let mut rng = Rng::new(4);
    let mut list = MoveList::new();
    let mut asked = 0;
    for _ in 0..20 {
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            if board.game_winning_move().is_none() {
                let value = f64::from(network.evaluate(&board));
                let seat_0 = if board.to_move() == 0 {
                    value
                } else {
                    1.0 - value
                };
                let mut position = ValueBoard { board, network };
                assert_eq!(position.estimate(), seat_0);
                assert_eq!(Game::playout(&mut position, &mut rng), seat_0);
                asked += 1;
            }
            Board::legal_moves(&board, &mut list);
            board.play(*rng.pick(&list).unwrap());
        }
    }
    assert!(asked > 500);
}

#[test]
fn mcts_searches_value_boards() {
    let network = random_network(23);
    let mut mcts = Mcts::new(0.5, 1);
    for (board, moves) in positions_with_a_winning_move(10) {
        let result = mcts.search(
            &ValueBoard { board, network },
            &moves,
            Budget::Iterations(500),
        );
        let mut next = board;
        next.play(result.best);
        assert_eq!(next.status(), Status::Win(board.to_move()), "{result:?}");
    }
    let board = Board::new();
    let mut moves = Vec::new();
    Game::legal_moves(&board, &mut moves);
    let result = Mcts::new(0.5, 2).search(
        &ValueBoard { board, network },
        &moves,
        Budget::Iterations(2_000),
    );
    assert_eq!(result.iterations, 2_000);
    assert!(board.is_legal(result.best));
}

#[test]
fn mix_boards_weigh_the_network_and_a_playout() {
    let network = random_network(29);
    let policy: &'static PlayoutPolicy = Box::leak(Box::new(PlayoutPolicy::uniform()));
    let mut rng = Rng::new(8);
    let mut list = MoveList::new();
    let mut checked = 0;
    for _ in 0..10 {
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            let estimate = ValueBoard { board, network }.estimate();
            let mix = |share| MixBoard {
                board,
                network,
                policy,
                share,
            };
            if board.game_winning_move().is_some() {
                assert_eq!(Game::playout(&mut mix(0.0), &mut rng), estimate);
            } else {
                assert_eq!(Game::playout(&mut mix(1.0), &mut rng), estimate);
                let played = Game::playout(&mut mix(0.0), &mut rng);
                assert!([0.0, 0.5, 1.0].contains(&played), "{played}");
                let half = Game::playout(&mut mix(0.5), &mut rng);
                let rest = half - 0.5 * estimate;
                assert!(
                    [0.0, 0.25, 0.5].iter().any(|&r| (rest - r).abs() < 1e-12),
                    "{half} with {estimate}"
                );
                checked += 1;
            }
            Board::legal_moves(&board, &mut list);
            board.play(*rng.pick(&list).unwrap());
        }
    }
    assert!(checked > 200);
}
