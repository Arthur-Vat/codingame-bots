//! The fast engine must agree with the reference referee on every position
//! of many random games: same valid actions, same small-board winners, same
//! points, same player to move, same result.

use cg_arena::referee::Outcome;
use cg_core::rng::Rng;
use uttt_engine::{Board, Move, MoveList, Status};
use uttt_referee::UtttReferee;

/// Games compared, as required by the Phase 2 gate.
const GAMES: u64 = 10_000;

fn engine_actions(board: &Board, moves: &mut MoveList) -> Vec<(usize, usize)> {
    board.legal_moves(moves);
    let mut actions: Vec<(usize, usize)> = moves.iter().map(|mv| mv.row_col()).collect();
    actions.sort_unstable();
    actions
}

fn status_of(outcome: Option<Outcome>) -> Status {
    match outcome {
        None => Status::Ongoing,
        Some(Outcome::Win(seat)) => Status::Win(seat),
        Some(Outcome::Draw) => Status::Draw,
    }
}

#[test]
fn engine_and_referee_agree_on_10_000_random_games() {
    let mut rng = Rng::new(2026);
    let mut moves = MoveList::new();
    let mut results = [0u32; 3];
    let mut positions = 0u64;
    for seed in 0..GAMES {
        let mut referee = UtttReferee::new(seed);
        let mut board = Board::new();
        let mut history = Vec::new();
        loop {
            positions += 1;
            let context = || format!("game {seed}, after moves {history:?}");
            let mut expected: Vec<(usize, usize)> = referee.valid_actions().to_vec();
            expected.sort_unstable();
            assert_eq!(
                engine_actions(&board, &mut moves),
                expected,
                "{}",
                context()
            );
            assert_eq!(board.status(), status_of(referee.result()), "{}", context());
            assert_eq!(board.points(), referee.points(), "{}", context());
            for small in 0..9 {
                let referee_winner = referee.small_winner(small / 3, small % 3);
                assert_eq!(board.small_winner(small), referee_winner, "{}", context());
            }
            if board.status() != Status::Ongoing {
                break;
            }
            assert_eq!(board.to_move(), referee.to_move(), "{}", context());

            // Pick from the sorted list so the referee's shuffle does not matter.
            let (row, col) = expected[rng.below(expected.len() as u64) as usize];
            referee.play_cell((row, col)).unwrap();
            board.play(Move::from_row_col(row, col).unwrap());
            history.push((row, col));
        }
        match board.status() {
            Status::Win(seat) => results[seat] += 1,
            Status::Draw => results[2] += 1,
            Status::Ongoing => unreachable!(),
        }
    }
    // Random play reaches every kind of ending often.
    assert!(results.iter().all(|&count| count > 300), "{results:?}");
    assert!(positions > 400_000, "{positions}");
}
