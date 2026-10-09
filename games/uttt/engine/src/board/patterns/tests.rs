use super::*;
use crate::moves::MoveList;

/// Every legal move of `board`.
fn legal(board: &Board) -> Vec<Move> {
    let mut moves = MoveList::new();
    board.legal_moves(&mut moves);
    moves.to_vec()
}

#[test]
fn canonical_pattern_cells_are_counted_once_per_symmetry_class() {
    let ids = pattern_ids();
    assert_eq!(ids.len(), PATTERNS * 9);
    let most = ids.iter().copied().max().unwrap();
    assert_eq!(usize::from(most), PATTERN_CELLS - 1);
    // On the empty board, the 4 corners share an id, the 4 edges another,
    // and the centre a third.
    let empty = &ids[..9];
    assert!([0, 2, 6, 8].iter().all(|&cell| empty[cell] == empty[0]));
    assert!([1, 3, 5, 7].iter().all(|&cell| empty[cell] == empty[1]));
    assert_ne!(empty[0], empty[1]);
    assert_ne!(empty[0], empty[4]);
    assert_ne!(empty[1], empty[4]);
}

#[test]
fn symmetric_moves_of_symmetric_games_have_the_same_feature() {
    let mut rng = Rng::new(21);
    let mut checked = 0;
    for game in 0..80 {
        let symmetry = game % 8;
        let mut board = Board::new();
        let mut image = Board::new();
        while board.status() == Status::Ongoing {
            for mv in legal(&board) {
                let mirrored = Move::new(
                    symmetric_cell(symmetry, mv.board()),
                    symmetric_cell(symmetry, mv.cell()),
                );
                assert_eq!(
                    board.pattern_feature(mv),
                    image.pattern_feature(mirrored),
                    "symmetry {symmetry}, move {mv}"
                );
                assert!(board.pattern_feature(mv) < PATTERN_FEATURES);
                checked += 1;
            }
            let mv = *rng.pick(&legal(&board)).unwrap();
            board.play(mv);
            image.play(Move::new(
                symmetric_cell(symmetry, mv.board()),
                symmetric_cell(symmetry, mv.cell()),
            ));
        }
    }
    assert!(checked > 10_000);
}

#[test]
fn destinations_follow_free_choices_and_threats() {
    let mut rng = Rng::new(22);
    for _ in 0..100 {
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            let seat = board.to_move();
            for mv in legal(&board) {
                let destination = board.pattern_feature(mv) % DESTINATIONS;
                let mut next = board;
                next.play(mv);
                let free = next.status() == Status::Ongoing && next.target().is_none();
                if free {
                    assert_eq!(destination, 0, "{mv} gives a free choice");
                } else if next.status() == Status::Ongoing && mv.cell() != mv.board() {
                    let to = mv.cell();
                    let expected = 1
                        + usize::from(board.threats[1 - seat] & (1 << to) != 0)
                        + 2 * usize::from(board.threats[seat] & (1 << to) != 0);
                    assert_eq!(destination, expected, "{mv}");
                }
            }
            board.play(*rng.pick(&legal(&board)).unwrap());
        }
    }
}

#[test]
fn weights_survive_their_text_and_follow_the_log_weights() {
    let mut rng = Rng::new(23);
    let log_weights: Vec<f32> = (0..PATTERN_FEATURES)
        .map(|_| (rng.unit() * 16.0 - 8.0) as f32)
        .collect();
    let text = encode_pattern_weights(&log_weights);
    assert_eq!(text.len(), PATTERN_FEATURES);
    let decoded = PatternPolicy::decode(&text).unwrap();
    let rounded: Vec<f32> = log_weights
        .iter()
        .map(|&log| ((log * 4.0).round() / 4.0).clamp(-8.0, 7.75))
        .collect();
    assert_eq!(decoded, PatternPolicy::from_log_weights(&rounded).unwrap());
    // A quarter more in log-weight is about 1.28 times the weight.
    let pair = PatternPolicy::from_log_weights(
        &[vec![0.0, 0.25], vec![0.0; PATTERN_FEATURES - 2]].concat(),
    )
    .unwrap();
    let ratio = f64::from(pair.weights[1]) / f64::from(pair.weights[0]);
    assert!((ratio - 0.25f64.exp()).abs() < 1e-3, "{ratio}");
    assert!(PatternPolicy::decode("A!").is_err());
    assert!(PatternPolicy::decode("AB").is_err());
}

#[test]
fn draws_follow_the_weights_and_take_game_winning_moves() {
    let mut rng = Rng::new(24);
    // One feature, an empty board's corner sent to an empty board, weighs
    // e^4 times the others.
    let board = {
        let mut board = Board::new();
        board.play(Move::new(4, 0));
        board
    };
    let favourite = Move::new(0, 0);
    let mut log_weights = vec![0.0f32; PATTERN_FEATURES];
    log_weights[board.pattern_feature(favourite)] = 4.0;
    let policy = PatternPolicy::from_log_weights(&log_weights).unwrap();
    let mut weights = Vec::new();
    let moves = legal(&board);
    board.pattern_move_weights(&policy, &moves, &mut weights);
    let total: f32 = weights.iter().sum();
    let expected: f64 = moves
        .iter()
        .zip(&weights)
        .filter(|&(&mv, _)| board.pattern_feature(mv) == board.pattern_feature(favourite))
        .map(|(_, &w)| f64::from(w / total))
        .sum();
    let draws = 20_000;
    let hits = (0..draws)
        .filter(|_| {
            let mv = board.pattern_move(&policy, &mut rng);
            board.pattern_feature(mv) == board.pattern_feature(favourite)
        })
        .count();
    let share = hits as f64 / f64::from(draws);
    assert!(
        (share - expected).abs() < 0.02,
        "{share} against {expected}"
    );
    // Positions with a game-winning move always play it.
    let uniform = PatternPolicy::from_log_weights(&vec![0.0; PATTERN_FEATURES]).unwrap();
    for _ in 0..300 {
        let mut position = Board::new();
        while position.status() == Status::Ongoing {
            if let Some(winning) = position.game_winning_move() {
                let mv = position.pattern_move(&uniform, &mut rng);
                let mut next = position;
                next.play(mv);
                assert_eq!(next.status(), Status::Win(position.to_move()), "{winning}");
            }
            position.play(*rng.pick(&legal(&position)).unwrap());
        }
    }
}

#[test]
fn pattern_playouts_end_and_are_reproducible() {
    let policy = PatternPolicy::from_log_weights(&vec![0.0; PATTERN_FEATURES])
        .unwrap()
        .for_plies(16);
    for seed in 0..50 {
        let (mut a, mut b) = (Board::new(), Board::new());
        let status = a.pattern_playout(&policy, &mut Rng::new(seed));
        assert_ne!(status, Status::Ongoing);
        assert_eq!(b.pattern_playout(&policy, &mut Rng::new(seed)), status);
        assert_eq!(a, b);
    }
}
