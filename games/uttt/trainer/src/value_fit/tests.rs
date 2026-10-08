use uttt_engine::Move;

use uttt_engine::value::ValueNetwork;

use super::*;

/// The network of ADR 0017, which most tests use.
type Small = Shape<64, 16>;
const H: usize = 64;
const PARAMETERS: usize = Small::PARAMETERS;
const INPUT: usize = Small::INPUT;
const INPUT_BIAS: usize = Small::INPUT_BIAS;
const HIDDEN: usize = Small::HIDDEN;
const HIDDEN_BIAS: usize = Small::HIDDEN_BIAS;
const OUTPUT: usize = Small::OUTPUT;
const OUTPUT_BIAS: usize = Small::OUTPUT_BIAS;
use crate::data::Searched;

/// Every position of `games` random games.
fn random_positions(games: u64, seed: u64) -> Vec<Board> {
    let mut rng = Rng::new(seed);
    let mut positions = Vec::new();
    for _ in 0..games {
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            positions.push(board);
            board.play(board.random_move(&mut rng));
        }
    }
    positions
}

fn random_parameters(spread: f64, seed: u64) -> Vec<f32> {
    let mut rng = Rng::new(seed);
    (0..PARAMETERS)
        .map(|_| ((rng.unit() * 2.0 - 1.0) * spread) as f32)
        .collect()
}

/// The network's output, from 0 to 1, when the inputs in `active` are 1.
fn predict(parameters: &[f32], active: &[u16]) -> f32 {
    sigmoid(forward::<64, 16>(parameters, active).z)
}

fn inputs(board: &Board) -> Vec<u16> {
    let mut list = [0; MAX_ACTIVE];
    let count = active_inputs(board, &mut list);
    list[..count].to_vec()
}

#[test]
fn the_layout_and_forward_pass_match_the_engine() {
    assert_eq!(PARAMETERS, ValueNetwork::PARAMETERS);
    let parameters = random_parameters(0.3, 1);
    let network = ValueNetwork::from_parameters(&parameters).unwrap();
    for board in random_positions(20, 2) {
        let active = inputs(&board);
        let ours = predict(&parameters, &active);
        let engine = network.evaluate_inputs(&active);
        assert!((ours - engine).abs() < 1e-6, "{ours} against {engine}");
    }
}

/// Which hidden units are between their bounds, where the gradient flows.
fn linear_units(parameters: &[f32], active: &[u16]) -> (u64, u16) {
    let values = forward::<64, 16>(parameters, active);
    let first = values
        .first_pre
        .iter()
        .enumerate()
        .filter(|(_, &pre)| pre > 0.0 && pre < 1.0)
        .fold(0u64, |mask, (unit, _)| mask | 1 << unit);
    let second = values
        .second_pre
        .iter()
        .enumerate()
        .filter(|(_, &pre)| pre > 0.0 && pre < 1.0)
        .fold(0u16, |mask, (unit, _)| mask | 1 << unit);
    (first, second)
}

#[test]
fn gradients_match_finite_differences() {
    let parameters = random_parameters(0.2, 3);
    let boards = random_positions(3, 4);
    let mut checked = 0;
    for (index, board) in boards.iter().enumerate().step_by(7) {
        let active = inputs(board);
        let target = [0.0, 0.5, 1.0][index % 3];
        let mut gradient = vec![0.0; PARAMETERS];
        accumulate::<64, 16>(&parameters, &active, target, &mut gradient);
        let inactive = (0..INPUTS as u16)
            .find(|input| !active.contains(input))
            .unwrap();
        assert!(gradient[INPUT + usize::from(inactive) * H..][..H]
            .iter()
            .all(|&g| g == 0.0));
        let mut probes = vec![OUTPUT_BIAS, OUTPUT + 3, HIDDEN_BIAS + 5, INPUT_BIAS + 9];
        probes.extend((0..H).step_by(5).map(|unit| HIDDEN + 2 * H + unit));
        probes.extend(
            (0..H)
                .step_by(3)
                .map(|unit| INPUT + usize::from(active[0]) * H + unit),
        );
        let pattern = linear_units(&parameters, &active);
        for probe in probes {
            let step = 1e-3;
            let loss_at = |delta: f32| {
                let mut moved = parameters.clone();
                moved[probe] += delta;
                (
                    cross_entropy(forward::<64, 16>(&moved, &active).z, target),
                    linear_units(&moved, &active),
                )
            };
            let ((up, up_pattern), (down, down_pattern)) = (loss_at(step), loss_at(-step));
            // A step across a unit's bound bends the loss: not comparable.
            if up_pattern != pattern || down_pattern != pattern {
                continue;
            }
            let numeric = (up - down) / (2.0 * f64::from(step));
            let analytic = f64::from(gradient[probe]);
            assert!(
                (numeric - analytic).abs() < 2e-3 + 0.02 * analytic.abs(),
                "parameter {probe}: {analytic} against {numeric}"
            );
            checked += 1;
        }
    }
    assert!(checked > 100, "only {checked} parameters checked");
}

#[test]
fn symmetries_map_inputs_as_in_the_symmetric_game() {
    let symmetries = input_symmetries();
    assert_eq!(symmetries.len(), 8);
    for map in &symmetries {
        let mut images: Vec<u16> = map.to_vec();
        images.sort_unstable();
        assert!(images
            .iter()
            .enumerate()
            .all(|(index, &image)| usize::from(image) == index));
    }
    let mut rng = Rng::new(6);
    for game in 0..40 {
        let view = game % 8;
        let mut board = Board::new();
        let mut image = Board::new();
        while board.status() == Status::Ongoing {
            let mut mapped: Vec<u16> = inputs(&board)
                .iter()
                .map(|&input| symmetries[view][usize::from(input)])
                .collect();
            mapped.sort_unstable();
            assert_eq!(mapped, inputs(&image), "symmetry {view}");
            let mv = board.random_move(&mut rng);
            board.play(mv);
            image.play(Move::new(
                symmetric_cell(view, mv.board()),
                symmetric_cell(view, mv.cell()),
            ));
            assert_eq!(
                board.status() == Status::Ongoing,
                image.status() == Status::Ongoing
            );
        }
    }
}

#[test]
fn rounded_weights_decode_in_the_engine_as_the_trainer_rounded_them() {
    let parameters = random_parameters(0.7, 8);
    let (rounded, text) = quantize::<64, 16>(&parameters);
    let decoded = ValueNetwork::decode(&text).unwrap();
    assert!(decoded == ValueNetwork::from_parameters(&rounded).unwrap());
    let mut at = 0;
    for count in ValueNetwork::GROUPS {
        let group = at..at + count;
        let largest = parameters[group.clone()]
            .iter()
            .fold(0.0f32, |max, value| max.max(value.abs()));
        for index in group {
            assert!((rounded[index] - parameters[index]).abs() <= largest / 254.0 * 1.001);
        }
        at += count;
    }
}

#[test]
fn base64_matches_the_standard() {
    assert_eq!(base64(b"Man"), "TWFu");
    assert_eq!(base64(b"Ma"), "TWE=");
    assert_eq!(base64(b"M"), "TQ==");
    assert_eq!(base64(b""), "");
}

#[test]
fn the_weights_source_holds_the_text_in_continued_lines() {
    let text = "AbC+/9".repeat(50);
    let source = weights_source(&text, "a test", (64, 16));
    assert!(source.starts_with("//! Generated by `uttt-trainer fit-value`: a test.\n"));
    let start = source.find("= \"\\\n").unwrap() + 5;
    let end = source.rfind("\";").unwrap();
    // A line continuation drops the backslash, the line break and the next
    // line's leading spaces, of which there are none.
    let literal = source[start..end].replace("\\\n", "");
    assert_eq!(literal, text);
    assert!(source.lines().all(|line| line.len() <= 100));
}

/// A game record of a random game, with every position searched and
/// root scores that vary.
fn random_record(seed: u64) -> GameRecord {
    let mut rng = Rng::new(seed);
    let mut board = Board::new();
    let mut record = GameRecord::default();
    while board.status() == Status::Ongoing {
        record.searched.push(Searched {
            ply: record.moves.len() as u8,
            score: Some(rng.unit() as f32),
            visits: Vec::new(),
        });
        let mv = board.random_move(&mut rng);
        record.moves.push(mv);
        board.play(mv);
    }
    record
}

/// Every example of `examples`, built.
fn all(examples: &impl Examples) -> Vec<Example> {
    (0..examples.len())
        .map(|index| examples.get(index))
        .collect()
}

#[test]
fn positions_hold_out_whole_games_and_score_the_side_to_move() {
    let games: Vec<GameRecord> = (0..40).map(random_record).collect();
    let mut split = Split::default();
    // Added in two parts, as from two files: game indices run on.
    split.add(&games[..13]);
    split.add(&games[13..]);
    assert_eq!(split.games, 40);
    let mut expected = (Vec::new(), Vec::new());
    for (index, game) in games.iter().enumerate() {
        let set = if index % 20 == 19 {
            &mut expected.1
        } else {
            &mut expected.0
        };
        for (board, searched) in game.positions() {
            if board.game_winning_move().is_some() {
                continue;
            }
            let result = match game.status() {
                Status::Win(seat) if seat == board.to_move() => 1.0,
                Status::Win(_) => 0.0,
                _ => 0.5,
            };
            set.push((board, result, searched.score.unwrap()));
        }
    }
    for (stored, expected) in [(&split.fitted, &expected.0), (&split.held_out, &expected.1)] {
        let built = all(stored);
        assert_eq!(built.len(), expected.len());
        for (index, (example, (board, result, score))) in built.iter().zip(expected).enumerate() {
            assert_eq!(example.board, *board);
            assert_eq!(example.result, *result);
            assert_eq!(stored.result(index), *result);
            assert!(
                (example.score - score).abs() < 1e-4,
                "{} against {score}",
                example.score
            );
        }
    }
    assert!(!split.held_out.entries.is_empty());
    assert_eq!(std::mem::size_of::<Entry>(), 8);
}

#[test]
fn training_learns_a_simple_rule() {
    // The "result" is whether the side to move has a threat: a rule the
    // inputs show directly.
    let label = |board: &Board| {
        if board.threat_boards(board.to_move()) != 0 {
            1.0
        } else {
            0.0
        }
    };
    let make = |seed| -> Vec<Example> {
        random_positions(120, seed)
            .into_iter()
            .filter(|board| board.game_winning_move().is_none())
            .map(|board| Example {
                result: label(&board),
                score: label(&board),
                board,
            })
            .collect()
    };
    let (fitted, held_out) = (make(10), make(11));
    let start = initial_parameters::<64, 16>(12);
    let before = measure::<64, 16>(&start, &held_out[..], 2);
    let mut epochs = Vec::new();
    let settings = Training {
        epochs: 4,
        batch: 64,
        rate: 0.01,
        penalty: 0.0,
        target: Target::Result,
        threads: 2,
        seed: 13,
    };
    let trained = train::<64, 16>(start, &fitted[..], &held_out[..], &settings, |epoch| {
        epochs.push(*epoch)
    });
    let after = measure::<64, 16>(&trained, &held_out[..], 2);
    assert_eq!(epochs.len(), 4);
    assert_eq!(epochs[3].held_out, after);
    assert!(
        after.cross_entropy < 0.5 * before.cross_entropy,
        "{before:?} then {after:?}"
    );
    assert!(after.squared_error < 0.05, "{after:?}");
}

#[test]
fn more_playouts_predict_results_better() {
    let weights =
        crate::selfplay::read_policy_weights(include_str!("../../../bots/mcts/src/weights.rs"))
            .unwrap();
    let policy = PlayoutPolicy::new(weights).for_plies(16);
    let games: Vec<GameRecord> = (100..140).map(random_record).collect();
    let mut split = Split::default();
    split.add(&games);
    let positions = spread(&split.fitted, 300);
    assert!(
        positions.len() > 150 && positions.len() <= 300,
        "{}",
        positions.len()
    );
    let counts = [1, 4, 16];
    let errors = playout_errors(&positions, &policy, &counts, 5, 2);
    assert!(errors[0] > errors[1] && errors[1] > errors[2], "{errors:?}");
    assert_eq!(errors, playout_errors(&positions, &policy, &counts, 5, 2));
}

#[test]
fn worth_is_read_between_playout_counts() {
    let counts = [1, 2, 4];
    let errors = [0.3, 0.25, 0.18];
    assert_eq!(
        worth_in_playouts(0.35, &counts, &errors),
        "less than one playout"
    );
    assert_eq!(
        worth_in_playouts(0.2, &counts, &errors),
        "between 2 and 4 playouts"
    );
    assert_eq!(
        worth_in_playouts(0.28, &counts, &errors),
        "between 1 and 2 playouts"
    );
    assert_eq!(
        worth_in_playouts(0.1, &counts, &errors),
        "more than 4 playouts"
    );
}

#[test]
fn the_larger_network_matches_the_engine_and_rounds_the_same() {
    type Large = Shape<128, 32>;
    assert_eq!(Large::PARAMETERS, Network::<128, 32>::PARAMETERS);
    let parameters: Vec<f32> = {
        let mut rng = Rng::new(40);
        (0..Large::PARAMETERS)
            .map(|_| ((rng.unit() * 2.0 - 1.0) * 0.2) as f32)
            .collect()
    };
    let network = Network::<128, 32>::from_parameters(&parameters).unwrap();
    for board in random_positions(10, 41) {
        let active = inputs(&board);
        let ours = sigmoid(forward::<128, 32>(&parameters, &active).z);
        let engine = network.evaluate_inputs(&active);
        assert!((ours - engine).abs() < 1e-6, "{ours} against {engine}");
    }
    let (rounded, text) = quantize::<128, 32>(&parameters);
    assert!(
        Network::<128, 32>::decode(&text).unwrap()
            == Network::<128, 32>::from_parameters(&rounded).unwrap()
    );
    // One step of training moves the loss down on a single position.
    let board = random_positions(1, 42)[20];
    let active = inputs(&board);
    let mut gradient = vec![0.0; Large::PARAMETERS];
    let before = accumulate::<128, 32>(&parameters, &active, 1.0, &mut gradient);
    let stepped: Vec<f32> = parameters
        .iter()
        .zip(&gradient)
        .map(|(value, g)| value - 0.01 * g)
        .collect();
    let after = cross_entropy(forward::<128, 32>(&stepped, &active).z, 1.0);
    assert!(after < before, "{before} then {after}");
}
