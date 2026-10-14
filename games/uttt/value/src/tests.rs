use cg_core::rng::Rng;

use cg_search::{Budget, Mcts};
use uttt_engine::grid;
use uttt_engine::{Move, MoveList, Status};

use super::*;

/// Every position of `games` random games.
fn random_positions(games: u64) -> Vec<Board> {
    let mut positions = Vec::new();
    let mut moves = MoveList::new();
    for seed in 0..games {
        let mut rng = Rng::new(seed);
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            positions.push(board);
            board.legal_moves(&mut moves);
            board.play(*rng.pick(&moves).unwrap());
        }
    }
    positions
}

/// The active inputs of `board`, worked out one cell at a time from the
/// module's documentation.
fn reference_inputs(board: &Board) -> Vec<u16> {
    let me = board.to_move();
    let them = 1 - me;
    let mut inputs = Vec::new();
    for (seat, offset) in [(me, 0), (them, 81)] {
        for small in (0..9).filter(|&small| !board.is_closed(small)) {
            for cell in 0..9 {
                if board.mark(Move::new(small, cell)) == Some(seat) {
                    inputs.push(offset + 9 * small + cell);
                }
            }
        }
    }
    for small in (0..9).filter(|&small| board.is_closed(small)) {
        let status = match board.small_winner(small) {
            Some(seat) if seat == me => 0,
            Some(_) => 1,
            None => 2,
        };
        inputs.push(162 + 3 * small + status);
    }
    inputs.push(189 + board.target().unwrap_or(9));
    for (seat, offset) in [(me, 199), (them, 208)] {
        for small in (0..9).filter(|&small| !board.is_closed(small)) {
            let marks = board.cells(seat, small);
            let wins = (0..9).any(|cell| {
                board.mark(Move::new(small, cell)).is_none() && grid::has_line(marks | 1 << cell)
            });
            if wins {
                inputs.push(offset + small);
            }
        }
    }
    inputs.into_iter().map(|input| input as u16).collect()
}

#[test]
fn active_inputs_describe_the_position_from_the_side_to_move() {
    let mut list = [0; MAX_ACTIVE];
    let positions = random_positions(300);
    assert!(positions.len() > 10_000);
    for board in positions {
        let count = active_inputs(&board, &mut list);
        assert_eq!(list[..count], reference_inputs(&board)[..], "{board:?}");
        assert!(list[..count].windows(2).all(|pair| pair[0] < pair[1]));
        assert!(list[..count]
            .iter()
            .all(|&input| usize::from(input) < INPUTS));
    }
}

/// Parameters drawn uniformly from `-spread..spread`.
fn random_parameters(count: usize, spread: f64, seed: u64) -> Vec<f32> {
    let mut rng = Rng::new(seed);
    (0..count)
        .map(|_| ((rng.unit() * 2.0 - 1.0) * spread) as f32)
        .collect()
}

/// The network of ADR 0017 computed plainly, in `f64`, from its parameters
/// in the documented order.
fn plain_forward(parameters: &[f32], active: &[u16]) -> f64 {
    const H: usize = 64;
    let value = |index: usize| f64::from(parameters[index]);
    let (input, input_bias) = (0, INPUTS * H);
    let (hidden, hidden_bias) = (input_bias + H, input_bias + H + 16 * H);
    let (output, output_bias) = (hidden_bias + 16, hidden_bias + 32);
    let first: Vec<f64> = (0..H)
        .map(|unit| {
            let sum: f64 = active
                .iter()
                .map(|&row| value(input + usize::from(row) * H + unit))
                .sum();
            (value(input_bias + unit) + sum).clamp(0.0, 1.0)
        })
        .collect();
    let second: Vec<f64> = (0..16)
        .map(|unit| {
            let sum: f64 = first
                .iter()
                .enumerate()
                .map(|(from, x)| x * value(hidden + unit * H + from))
                .sum();
            (value(hidden_bias + unit) + sum).clamp(0.0, 1.0)
        })
        .collect();
    let z = value(output_bias)
        + second
            .iter()
            .enumerate()
            .map(|(from, x)| x * value(output + from))
            .sum::<f64>();
    1.0 / (1.0 + (-z).exp())
}

#[test]
fn the_network_computes_the_documented_function() {
    assert_eq!(ValueNetwork::PARAMETERS, 15_009);
    assert_eq!(
        ValueNetwork::GROUPS.iter().sum::<usize>(),
        ValueNetwork::PARAMETERS
    );
    let parameters = random_parameters(ValueNetwork::PARAMETERS, 0.3, 5);
    let network = ValueNetwork::from_parameters(&parameters).unwrap();
    let mut list = [0; MAX_ACTIVE];
    let (mut low, mut high) = (1.0f32, 0.0f32);
    for board in random_positions(30) {
        let count = active_inputs(&board, &mut list);
        let value = network.evaluate(&board);
        let plain = plain_forward(&parameters, &list[..count]);
        assert!(
            (f64::from(value) - plain).abs() < 1e-5,
            "{value} against {plain}"
        );
        assert!(value > 0.0 && value < 1.0);
        low = low.min(value);
        high = high.max(value);
    }
    // The test means something only if the positions give different values.
    assert!(high - low > 0.1, "values from {low} to {high}");
}

#[test]
fn a_wrong_number_of_parameters_is_refused() {
    let error = ValueNetwork::from_parameters(&[0.0; 10]).unwrap_err();
    assert_eq!(error, "10 parameters instead of 15009");
}

/// Standard base64 with padding, as `uttt-trainer` writes it.
fn encode_base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut text = String::new();
    for chunk in bytes.chunks(3) {
        let mut word = [0u8; 3];
        word[..chunk.len()].copy_from_slice(chunk);
        let bits = u32::from(word[0]) << 16 | u32::from(word[1]) << 8 | u32::from(word[2]);
        for index in 0..4 {
            if index <= chunk.len() {
                text.push(char::from(
                    ALPHABET[(bits >> (18 - 6 * index)) as usize & 63],
                ));
            } else {
                text.push('=');
            }
        }
    }
    text
}

#[test]
fn base64_decodes_with_and_without_padding() {
    assert_eq!(base64("TWFu").unwrap(), b"Man");
    assert_eq!(base64("TWE=").unwrap(), b"Ma");
    assert_eq!(base64("TQ==").unwrap(), b"M");
    assert_eq!(base64("").unwrap(), b"");
    assert_eq!(base64("TW!u").unwrap_err(), "'!' is not base64");
    let bytes: Vec<u8> = (0..=255).collect();
    assert_eq!(base64(&encode_base64(&bytes)).unwrap(), bytes);
}

#[test]
fn decoding_multiplies_each_byte_by_its_group_scale() {
    let mut rng = Rng::new(9);
    let scales = [0.01f32, 0.02, 0.005, 0.03, 0.1, 0.25];
    let mut bytes = Vec::new();
    let mut parameters = Vec::new();
    for (count, scale) in ValueNetwork::GROUPS.into_iter().zip(scales) {
        bytes.extend_from_slice(&scale.to_le_bytes());
        for _ in 0..count {
            let value = (rng.below(255) as i16 - 127) as i8;
            bytes.push(value as u8);
            parameters.push(f32::from(value) * scale);
        }
    }
    let decoded = ValueNetwork::decode(&encode_base64(&bytes)).unwrap();
    let expected = ValueNetwork::from_parameters(&parameters).unwrap();
    assert!(decoded == expected, "decoded parameters differ");

    let error = ValueNetwork::decode(&encode_base64(&bytes[1..])).unwrap_err();
    assert_eq!(
        error,
        format!("{} bytes instead of {}", bytes.len() - 1, bytes.len())
    );
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
