use super::*;
use crate::moves::MoveList;

/// Every legal move of `board`.
fn legal(board: &Board) -> Vec<Move> {
    let mut moves = MoveList::new();
    board.legal_moves(&mut moves);
    moves.to_vec()
}

/// Whether the player to move, if it were `seat`, would win small board
/// `small` by playing `cell` there.
fn would_win(board: &Board, seat: usize, small: usize, cell: usize) -> bool {
    grid::has_line(board.marks[seat][small] | 1 << cell)
}

/// Positions from random games, each with the player to move.
fn positions() -> Vec<Board> {
    let mut rng = Rng::new(3);
    let mut all = Vec::new();
    for _ in 0..200 {
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            all.push(board);
            let moves = legal(&board);
            board.play(*rng.pick(&moves).unwrap());
        }
    }
    all
}

#[test]
fn move_classes_match_what_the_moves_do() {
    let mut seen = [0u32; CLASSES];
    for board in positions() {
        let seat = board.to_move();
        for mv in legal(&board) {
            let class = board.move_class(mv) as u8;
            seen[usize::from(class)] += 1;
            let (small, cell) = (mv.board(), mv.cell());
            let mut after = board;
            after.play(mv);
            assert_eq!(
                class & WINS_BOARD != 0,
                after.small_winner(small) == Some(seat),
                "{mv} in {board:?}"
            );
            assert_eq!(
                class & BLOCKS != 0,
                would_win(&board, 1 - seat, small, cell),
                "{mv} in {board:?}"
            );
            assert_eq!(class & CENTRE != 0, cell == 4);
            if after.status() == Status::Ongoing {
                assert_eq!(
                    class & GIVES_FREE_CHOICE != 0,
                    after.target().is_none(),
                    "{mv} in {board:?}"
                );
                // The opponent can win the small board it is sent to,
                // judged before the move unless the move changed it.
                if let Some(target) = after.target() {
                    if target != small {
                        let can_win = legal(&after)
                            .into_iter()
                            .any(|answer| would_win(&after, 1 - seat, target, answer.cell()));
                        assert_eq!(class & GIVES_BOARD != 0, can_win, "{mv} in {board:?}");
                    }
                } else {
                    assert_eq!(class & GIVES_BOARD, 0);
                }
            }
        }
    }
    // Every single feature shows up, and many combinations.
    for (feature, name) in FEATURE_NAMES.iter().enumerate() {
        let with: u32 = (0..CLASSES)
            .filter(|class| class & (1 << feature) != 0)
            .map(|class| seen[class])
            .sum();
        assert!(with > 100, "{name} seen {with} times");
    }
    assert!(
        seen.iter().filter(|&&count| count > 0).count() >= 20,
        "{seen:?}"
    );
}

#[test]
fn a_policy_move_wins_the_game_when_it_can_and_follows_the_weights_otherwise() {
    // Only moves that win their small board, then only the others.
    let only_wins = PlayoutPolicy::new(std::array::from_fn(|class| {
        u32::from(class as u8 & WINS_BOARD != 0)
    }));
    let mut rng = Rng::new(9);
    let (mut decisive, mut weighted) = (0, 0);
    for board in positions() {
        let moves = legal(&board);
        let mv = board.policy_move(&only_wins, &mut rng);
        assert!(moves.contains(&mv), "{mv} in {board:?}");
        let mut decisive_rng = rng.clone();
        if board.game_winning_cell().is_some() {
            assert_eq!(mv, board.decisive_move(&mut decisive_rng));
            decisive += 1;
        } else if moves
            .iter()
            .any(|&other| board.move_class(other) as u8 & WINS_BOARD != 0)
        {
            assert!(board.move_class(mv) as u8 & WINS_BOARD != 0, "{mv}");
            weighted += 1;
        }
    }
    assert!(decisive > 50 && weighted > 500, "{decisive} {weighted}");
}

#[test]
fn draws_follow_the_weights() {
    // A board where the player to move is sent to small board 0 with its
    // nine cells empty: weight 3 for the centre, 1 for the rest.
    let mut board = Board::new();
    board.play(Move::new(4, 0));
    let policy = PlayoutPolicy::new(std::array::from_fn(|class| {
        if class as u8 & CENTRE != 0 {
            3
        } else {
            1
        }
    }));
    let mut rng = Rng::new(1);
    let mut centre = 0;
    let draws = 11_000;
    for _ in 0..draws {
        if board.policy_move(&policy, &mut rng).cell() == 4 {
            centre += 1;
        }
    }
    // Expected 3 / 11 of the draws: 3,000, within about four deviations.
    assert!((2_800..3_200).contains(&centre), "{centre}");
}

#[test]
fn zero_weights_fall_back_to_random_moves_and_playouts_end() {
    let zero = PlayoutPolicy::new([0; CLASSES]);
    let uniform = PlayoutPolicy::uniform();
    let mut rng = Rng::new(4);
    for _ in 0..100 {
        for policy in [&zero, &uniform] {
            let mut board = Board::new();
            assert_ne!(board.policy_playout(policy, &mut rng), Status::Ongoing);
        }
    }
}

#[test]
fn draws_weigh_each_move_by_its_class() {
    // The draw's weights, read back from the running totals, are those of
    // each legal move's class.
    let policy = PlayoutPolicy::new(std::array::from_fn(|class| 1 + class as u32 * 7));
    for board in positions().into_iter().step_by(7) {
        let threats = board.threats[1 - board.to_move()];
        let boards = match board.target() {
            Some(small) => vec![small],
            None => (0..9).filter(|&small| !board.is_closed(small)).collect(),
        };
        for small in boards {
            let cumulative = board.cumulative_weights(small, threats, &policy);
            let mut before = 0;
            for (cell, &sum) in cumulative.iter().enumerate() {
                let weight = sum - before;
                before = sum;
                let mv = Move::new(small, cell);
                let expected = if board.is_legal(mv) {
                    policy.weights()[board.move_class(mv)]
                } else {
                    0
                };
                assert_eq!(weight, expected, "{mv} in {board:?}");
            }
        }
    }
}

#[test]
fn a_policy_for_some_plies_still_ends_games() {
    let policy = PlayoutPolicy::uniform().for_plies(3);
    let mut rng = Rng::new(8);
    for _ in 0..100 {
        let mut board = Board::new();
        assert_ne!(board.policy_playout(&policy, &mut rng), Status::Ongoing);
    }
}
