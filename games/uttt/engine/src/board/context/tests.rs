use super::*;
use crate::board::symmetric_cell;
use crate::moves::MoveList;

/// Every legal move of `board`.
fn legal(board: &Board) -> Vec<Move> {
    let mut moves = MoveList::new();
    board.legal_moves(&mut moves);
    moves.to_vec()
}

/// The base-3 number with a 1 for each cell of `mask`.
fn base3(mask: u16) -> usize {
    (0..9)
        .filter(|&cell| mask & (1 << cell) != 0)
        .map(|cell| 3usize.pow(cell))
        .sum()
}

/// The features of `mv` computed the slow way, by playing it: the
/// definition the masks and the cache must match.
fn reference(board: &Board, mv: Move) -> [usize; 2] {
    let first = board.pattern_feature(mv);
    let mut after = *board;
    after.play(mv);
    let second = match after.target() {
        Some(target) if after.status() == Status::Ongoing => {
            let seat = after.to_move();
            let pattern =
                base3(after.cells(seat, target)) + 2 * base3(after.cells(1 - seat, target));
            let open = !after.closed_boards() & FULL;
            let decides = |side: usize| {
                usize::from((grid::completing_cells(after.won_boards(side)) & open) >> target & 1)
            };
            1 + usize::from(open_pattern_ids()[pattern]) * ROLES
                + 2 * decides(seat)
                + decides(1 - seat)
        }
        _ => 0,
    };
    [first, second]
}

/// A policy with every weight drawn at random.
fn random_policy(seed: u64) -> ContextPolicy {
    let mut rng = Rng::new(seed);
    let mut table = |size: usize| (0..size).map(|_| rng.below(64) as u8).collect::<Vec<u8>>();
    let patterns = table(PATTERN_FEATURES);
    ContextPolicy::new(patterns, table(DESTINATION_WEIGHTS)).unwrap()
}

#[test]
fn features_and_cached_weights_match_the_moves_played_out() {
    let mut rng = Rng::new(31);
    let mut checked = 0;
    let mut free = 0;
    for game in 0..300 {
        let policy = random_policy(game);
        let table = weights();
        let mut board = Board::new();
        let mut cache = Destinations::new(&board, &policy);
        while board.status() == Status::Ongoing {
            for mv in legal(&board) {
                let features = board.context_features(mv);
                assert_eq!(features, reference(&board, mv), "move {mv} in {board:?}");
                assert!(features[0] < policy.patterns.len() && features[1] < DESTINATION_WEIGHTS);
                free += usize::from(features[1] == 0);
                checked += 1;
            }
            let boards = match board.target() {
                Some(target) => 1 << target,
                None => !board.closed & FULL,
            };
            for small in grid::cells(boards) {
                let fresh = board.context_weights(&policy, small, None);
                let cached = board.context_weights(&policy, small, Some(&cache));
                assert_eq!(fresh, cached, "board {small} in {board:?}");
                let cumulative = board.context_cumulative(&policy, small, &cache);
                let mut total = 0.0;
                for cell in 0..9 {
                    total += fresh[cell];
                    assert!(
                        (cumulative[cell] - total).abs() <= total * 1e-5,
                        "cell {cell} of board {small}: {cumulative:?} against {fresh:?}"
                    );
                }
                for cell in grid::cells(board.empty_cells(small)) {
                    let [first, second] = reference(&board, Move::new(small, cell));
                    let q = usize::from(policy.patterns[first])
                        + usize::from(policy.destinations[second]);
                    assert_eq!(fresh[cell], table[q]);
                }
            }
            let mv = board.random_move(&mut rng);
            board.play(mv);
            cache.update(&board, &policy, mv.board());
        }
    }
    assert!(checked > 100_000, "{checked}");
    assert!(free > 1000, "{free} free choices");
}

#[test]
fn symmetric_moves_of_symmetric_games_have_the_same_features() {
    let mirror = |symmetry: usize, mv: Move| {
        Move::new(
            symmetric_cell(symmetry, mv.board()),
            symmetric_cell(symmetry, mv.cell()),
        )
    };
    let mut rng = Rng::new(9);
    for game in 0..40 {
        let symmetry = game % 8;
        let (mut board, mut image) = (Board::new(), Board::new());
        while board.status() == Status::Ongoing {
            for mv in legal(&board) {
                assert_eq!(
                    board.context_features(mv),
                    image.context_features(mirror(symmetry, mv))
                );
            }
            let mv = board.random_move(&mut rng);
            board.play(mv);
            image.play(mirror(symmetry, mv));
        }
    }
}

#[test]
fn packed_tables_decode_and_wrong_sizes_fail() {
    let policy = random_policy(3);
    let text = packed::encode_strided(&policy.patterns, DESTINATIONS)
        + &packed::encode_strided(&policy.destinations, ROLES);
    assert_eq!(ContextPolicy::decode(&text).unwrap(), policy);
    assert!(ContextPolicy::decode(&(text.clone() + "A")).is_err());
    assert!(ContextPolicy::new(vec![32; 10], vec![32; DESTINATION_WEIGHTS]).is_err());
    assert!(ContextPolicy::new(vec![32; PATTERN_FEATURES], vec![32; 10]).is_err());
    assert!(ContextPolicy::new(vec![64; PATTERN_FEATURES], vec![32; DESTINATION_WEIGHTS]).is_err());
}

#[test]
fn draws_follow_the_weights_and_take_game_winning_moves() {
    let policy = random_policy(5);
    let mut rng = Rng::new(17);
    let mut positions = 0;
    for _ in 0..40 {
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            let moves = legal(&board);
            if let Some(winning) = board.game_winning_move() {
                assert_eq!(board.context_move(&policy, &mut rng), winning);
            } else if positions < 30 && board.target().is_none() == (positions % 2 == 0) {
                // Compare frequencies with the priors on a few positions,
                // free choices and targets alike.
                positions += 1;
                let mut weights = Vec::new();
                board.context_move_weights(&policy, &moves, &mut weights);
                let total: f32 = weights.iter().sum();
                let draws = 20_000;
                let mut counts = vec![0u32; moves.len()];
                for _ in 0..draws {
                    let mv = board.context_move(&policy, &mut rng);
                    counts[moves.iter().position(|&m| m == mv).unwrap()] += 1;
                }
                for (count, weight) in counts.iter().zip(&weights) {
                    let expected = f64::from(weight / total) * f64::from(draws);
                    let spread = 5.0 * expected.sqrt() + 5.0;
                    assert!(
                        (f64::from(*count) - expected).abs() < spread,
                        "{count} draws for {expected:.1} expected"
                    );
                }
            }
            let mv = *rng.pick(&moves).unwrap();
            board.play(mv);
        }
    }
    assert_eq!(positions, 30);
}

#[test]
fn playouts_end_and_are_reproducible() {
    let policy = random_policy(6).for_plies(16);
    for seed in 0..50 {
        let (mut first, mut second) = (Board::new(), Board::new());
        let a = first.context_playout(&policy, &mut Rng::new(seed));
        let b = second.context_playout(&policy, &mut Rng::new(seed));
        assert_ne!(a, Status::Ongoing);
        assert_eq!((a, first), (b, second));
    }
}

#[test]
fn playout_draws_follow_the_weights() {
    let policy = random_policy(15);
    let mut rng = Rng::new(19);
    let mut positions = 0;
    for _ in 0..40 {
        let mut board = Board::new();
        let mut cache = Destinations::new(&board, &policy);
        while board.status() == Status::Ongoing {
            let moves = legal(&board);
            if board.game_winning_move().is_none()
                && positions < 30
                && board.target().is_none() == (positions % 2 == 0)
            {
                positions += 1;
                let mut weights = Vec::new();
                board.context_move_weights(&policy, &moves, &mut weights);
                let total: f32 = weights.iter().sum();
                let draws = 20_000;
                let mut counts = vec![0u32; moves.len()];
                for _ in 0..draws {
                    let (small, cell) = board.context_playout_cell(&policy, &cache, &mut rng);
                    let mv = Move::new(small, cell);
                    counts[moves.iter().position(|&m| m == mv).expect("a legal move")] += 1;
                }
                for (count, weight) in counts.iter().zip(&weights) {
                    let expected = f64::from(weight / total) * f64::from(draws);
                    let spread = 5.0 * expected.sqrt() + 5.0;
                    assert!(
                        (f64::from(*count) - expected).abs() < spread,
                        "{count} draws for {expected:.1} expected"
                    );
                }
            }
            let mv = *rng.pick(&moves).unwrap();
            board.play(mv);
            cache.update(&board, &policy, mv.board());
        }
    }
    assert_eq!(positions, 30);
}
