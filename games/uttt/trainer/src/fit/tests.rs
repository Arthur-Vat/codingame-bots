use super::*;
use crate::data::Searched;
use uttt_engine::board::{CENTRE, WINS_BOARD};
use uttt_engine::{Board, Move};

/// An example with `moves` legal moves per class and visit shares.
fn example(moves: &[(usize, u16)], visits: &[(usize, f64)], best: usize) -> Example {
    let mut example = Example {
        moves: [0; CLASSES],
        visits: [0.0; CLASSES],
        best,
    };
    for &(k, n) in moves {
        example.moves[k] = n;
    }
    for &(k, share) in visits {
        example.visits[k] = share;
    }
    example
}

#[test]
fn the_gradient_matches_finite_differences() {
    let examples = vec![
        example(
            &[(0, 5), (1, 2), (16, 1)],
            &[(0, 0.2), (1, 0.5), (16, 0.3)],
            1,
        ),
        example(&[(0, 8), (4, 1)], &[(0, 0.9), (4, 0.1)], 0),
    ];
    let mut theta = [0.0; CLASSES];
    theta[1] = 0.7;
    theta[16] = -0.3;
    let (_, gradient) = loss_and_gradient(&theta, &examples);
    for k in [0, 1, 4, 16] {
        let h = 1e-6;
        let mut up = theta;
        up[k] += h;
        let mut down = theta;
        down[k] -= h;
        let numeric = (loss_and_gradient(&up, &examples).0 - loss_and_gradient(&down, &examples).0)
            / (2.0 * h);
        assert!(
            (numeric - gradient[k]).abs() < 1e-6,
            "class {k}: {numeric} vs {}",
            gradient[k]
        );
    }
}

#[test]
fn fitting_recovers_known_preferences() {
    // Searches give class 1 moves ten times the share of class 0 moves.
    let examples: Vec<Example> = (0..50)
        .map(|_| example(&[(0, 5), (1, 1)], &[(0, 5.0 / 15.0), (1, 10.0 / 15.0)], 1))
        .collect();
    let (theta, loss) = fit(&examples, 3_000, 0.0);
    assert!(
        ((theta[1] - theta[0]) - 10f64.ln()).abs() < 0.01,
        "{theta:?}"
    );
    let (uniform, _) = loss_and_gradient(&[0.0; CLASSES], &examples);
    assert!(loss < uniform);
    let weights = weights(&theta, 1.0, 10_000);
    assert_eq!(weights[1], 10_000);
    assert!((990..=1_010).contains(&weights[0]), "{}", weights[0]);
    // A temperature of 2 halves the log ratio.
    let flat = super::weights(&theta, 2.0, 10_000);
    assert!((3_150..=3_180).contains(&flat[0]), "{}", flat[0]);
}

#[test]
fn examples_count_classes_and_visit_shares() {
    let mut board = Board::new();
    board.play(Move::new(4, 0));
    let mut moves = Vec::new();
    let mut visits = Vec::new();
    for cell in 0..9 {
        moves.push(Move::new(0, cell));
        visits.push((Move::new(0, cell), if cell == 4 { 30 } else { 10 }));
    }
    let game = GameRecord {
        moves: vec![Move::new(4, 0), Move::new(0, 4)],
        searched: vec![Searched {
            ply: 1,
            score: None,
            visits,
        }],
    };
    let examples = examples(&[game]);
    assert_eq!(examples.len(), 1);
    let centre = board.move_class(Move::new(0, 4));
    assert_eq!(centre & usize::from(CENTRE), usize::from(CENTRE));
    assert_eq!(centre & usize::from(WINS_BOARD), 0);
    assert_eq!(examples[0].best, centre);
    assert!((examples[0].visits[centre] - 30.0 / 110.0).abs() < 1e-12);
    assert_eq!(
        examples[0].moves.iter().map(|&n| u32::from(n)).sum::<u32>(),
        9
    );
    let _ = moves;
}

#[test]
fn class_names_list_the_features() {
    assert_eq!(class_name(0), "none");
    assert_eq!(class_name(1 | 16), "wins board, centre");
    let source = weights_source(&[1; CLASSES], "test");
    assert!(source.contains("pub const PLAYOUT_WEIGHTS: [u32; 32] = ["));
}
