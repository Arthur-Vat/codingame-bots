use cg_core::rng::Rng;

use super::*;
use crate::board::Status;
use crate::moves::MoveList;

/// A value of `-range..=range`.
fn small(rng: &mut Rng, range: i64) -> i64 {
    rng.below(2 * range as u64 + 1) as i64 - range
}

/// The parts of a random network: first-layer weights in `-feature..=feature`
/// and biases in `-bias..=bias`, output weights in `-output..=output`.
struct Parts {
    feature_weights: Vec<i8>,
    feature_bias: Vec<i16>,
    output_weights: Vec<i8>,
    output_bias: [i32; BUCKETS],
}

fn random_parts(width: usize, seed: u64, feature: i64, bias: i64, output: i64) -> Parts {
    let mut rng = Rng::new(seed);
    let mut output_bias = [0; BUCKETS];
    for value in &mut output_bias {
        *value = small(&mut rng, 2_000) as i32;
    }
    Parts {
        feature_weights: (0..FEATURES * width)
            .map(|_| small(&mut rng, feature) as i8)
            .collect(),
        feature_bias: (0..width).map(|_| small(&mut rng, bias) as i16).collect(),
        output_weights: (0..BUCKETS * 2 * width)
            .map(|_| small(&mut rng, output) as i8)
            .collect(),
        output_bias,
    }
}

fn network<const N: usize>(parts: &Parts) -> Network<N> {
    Network::from_parts(
        &parts.feature_weights,
        &parts.feature_bias,
        &parts.output_weights,
        &parts.output_bias,
    )
    .unwrap()
}

/// A network with moderate weights, so that accumulators are neither all
/// clipped nor all inside the range.
fn random_network<const N: usize>(seed: u64) -> (Parts, Network<N>) {
    let parts = random_parts(N, seed, 20, 60, 40);
    let net = network(&parts);
    (parts, net)
}

/// Every position of `games` random games, with the move played from it.
fn random_games(games: u64) -> Vec<Vec<(Board, Move)>> {
    let mut moves = MoveList::new();
    (0..games)
        .map(|seed| {
            let mut rng = Rng::new(seed);
            let mut board = Board::new();
            let mut game = Vec::new();
            while board.status() == Status::Ongoing {
                board.legal_moves(&mut moves);
                let mv = *rng.pick(&moves).unwrap();
                game.push((board, mv));
                board.play(mv);
            }
            game
        })
        .collect()
}

fn features(board: &Board, seat: usize) -> Vec<u16> {
    let mut list = [0; MAX_ACTIVE];
    let len = active_features(board, seat, &mut list);
    list[..len].to_vec()
}

/// The logit that the float formula of the module documentation gives, in
/// `f64`, from the dequantized parts.
fn reference_logit<const N: usize>(
    parts: &Parts,
    accumulator: &Accumulator<N>,
    board: &Board,
) -> f64 {
    let width = N;
    let stm = board.to_move();
    let bucket = bucket(board);
    let mut logit = f64::from(parts.output_bias[bucket]) / f64::from(QA * QB);
    for (side, seat) in [stm, 1 - stm].into_iter().enumerate() {
        for j in 0..width {
            let a = (f64::from(accumulator.values[seat][j]) / f64::from(QA)).clamp(0.0, 1.0);
            let v = f64::from(parts.output_weights[bucket * 2 * width + side * width + j])
                / f64::from(QB);
            logit += a * a * v;
        }
    }
    logit
}

#[test]
fn constants_are_consistent() {
    assert_eq!(OUR_CELLS + 81, THEIR_CELLS);
    assert_eq!(THEIR_CELLS + 81, OUR_BOARDS);
    assert_eq!(OUR_BOARDS + 9, THEIR_BOARDS);
    assert_eq!(THEIR_BOARDS + 9, DRAWN_BOARDS);
    assert_eq!(DRAWN_BOARDS + 9, FEATURES);
    assert_eq!(BUCKETS, TARGETS * PHASES);
    assert_eq!(Network::<256>::BYTES, 64_388);
    assert_eq!(Network::<16>::BYTES, 132 + 251 * 16);
    assert_eq!(Network::<256>::BYTES_V1, 54_068);
    assert_eq!(Network::<16>::BYTES_V1, 52 + 211 * 16);
    assert_eq!(i32::from(MAX_BIAS), 32_767 - 90 * 127);
    let worst = 2 * MAX_WIDTH as i64 * i64::from(QA * QA) * 127;
    assert!(worst < i64::from(i32::MAX));
}

#[test]
fn start_position_has_no_features_and_a_free_bucket() {
    let board = Board::new();
    assert_eq!(features(&board, 0), Vec::<u16>::new());
    assert_eq!(features(&board, 1), Vec::<u16>::new());
    assert_eq!(bucket(&board), 9);
}

#[test]
fn bucket_is_the_target_board() {
    let mut board = Board::new();
    board.play(Move::new(4, 7));
    assert_eq!(bucket(&board), 7);
    board.play(Move::new(7, 0));
    assert_eq!(bucket(&board), 0);
}

#[test]
fn features_are_increasing_and_in_range() {
    for game in random_games(100) {
        for (board, _) in game {
            for seat in 0..2 {
                let list = features(&board, seat);
                assert!(list.len() <= MAX_ACTIVE);
                assert!(list.windows(2).all(|pair| pair[0] < pair[1]));
                assert!(list.iter().all(|&f| usize::from(f) < FEATURES));
                let marks: usize = (0..2)
                    .map(|s| {
                        (0..9)
                            .map(|b| board.cells(s, b).count_ones() as usize)
                            .sum::<usize>()
                    })
                    .sum();
                let cells = list
                    .iter()
                    .filter(|&&f| usize::from(f) < OUR_BOARDS)
                    .count();
                assert_eq!(cells, marks);
            }
        }
    }
}

#[test]
fn cells_are_seen_from_each_perspective() {
    let mut board = Board::new();
    board.play(Move::new(4, 7)); // seat 0
    board.play(Move::new(7, 2)); // seat 1
    assert_eq!(
        features(&board, 0),
        vec![
            (OUR_CELLS + 9 * 4 + 7) as u16,
            (THEIR_CELLS + 9 * 7 + 2) as u16
        ]
    );
    assert_eq!(
        features(&board, 1),
        vec![
            (OUR_CELLS + 9 * 7 + 2) as u16,
            (THEIR_CELLS + 9 * 4 + 7) as u16
        ]
    );
}

/// Seat 0 wins small board 0 with cells 1, 2 and 0, while seat 1 answers
/// with cell 0 of the board it is sent to, which sends seat 0 back to board 0.
fn won_board_position() -> Board {
    let mut board = Board::new();
    for (small, cell) in [(0, 1), (1, 0), (0, 2), (2, 0), (0, 0)] {
        board.play(Move::new(small, cell));
    }
    assert_eq!(board.small_winner(0), Some(0));
    board
}

#[test]
fn won_boards_are_seen_from_each_perspective() {
    let board = won_board_position();
    // Seat 1 is sent to board 0, which is closed: a free choice, in the
    // phase of one or two closed boards.
    assert_eq!(phase(&board), 1);
    assert_eq!(bucket(&board), 9 + TARGETS);
    assert!(features(&board, 0).contains(&(OUR_BOARDS as u16)));
    assert!(!features(&board, 0).contains(&(THEIR_BOARDS as u16)));
    assert!(features(&board, 1).contains(&(THEIR_BOARDS as u16)));
    assert!(!features(&board, 1).contains(&(OUR_BOARDS as u16)));
    // The cells of the closed board stay.
    assert!(features(&board, 0).contains(&(OUR_CELLS as u16)));
    assert!(features(&board, 1).contains(&(THEIR_CELLS as u16)));
}

#[test]
fn a_drawn_board_shows_as_drawn() {
    let mut found = 0;
    for game in random_games(300) {
        for (board, _) in game {
            for small in 0..9 {
                let drawn = board.is_closed(small) && board.small_winner(small).is_none();
                for seat in 0..2 {
                    let has = features(&board, seat).contains(&((DRAWN_BOARDS + small) as u16));
                    assert_eq!(has, drawn);
                }
                found += usize::from(drawn);
            }
        }
    }
    assert!(found > 0, "no drawn small board in 300 random games");
}

/// The board features that `mv`, played from `before`, adds for its mover.
fn mover_board_feature(before: &Board, mv: Move) -> Option<u16> {
    let mut list = [0; 2];
    let len = move_features(before, mv, before.to_move(), &mut list);
    (len == 2).then_some(list[1])
}

#[test]
fn a_line_in_the_ninth_cell_wins_the_board() {
    let mut seen = 0;
    for game in random_games(1000) {
        for (before, mv) in game {
            let small = mv.board();
            let marks = (before.cells(0, small) | before.cells(1, small)).count_ones();
            let mut after = before;
            after.play(mv);
            if marks != 8 || after.small_winner(small).is_none() {
                continue;
            }
            assert_eq!(
                mover_board_feature(&before, mv),
                Some((OUR_BOARDS + small) as u16)
            );
            seen += 1;
        }
    }
    assert!(
        seen > 0,
        "no board won in its ninth cell in 1,000 random games"
    );
}

#[test]
fn a_move_that_fills_a_board_and_ends_the_game_draws_the_board() {
    let mut seen = 0;
    for game in random_games(2000) {
        for (before, mv) in game {
            let small = mv.board();
            let mut after = before;
            after.play(mv);
            if after.status() == Status::Ongoing || after.small_winner(small).is_some() {
                continue;
            }
            if !after.is_closed(small) {
                continue;
            }
            assert_eq!(
                mover_board_feature(&before, mv),
                Some((DRAWN_BOARDS + small) as u16)
            );
            let mut list = [0; 2];
            assert_eq!(
                move_features(&before, mv, 1 - before.to_move(), &mut list),
                2
            );
            assert_eq!(usize::from(list[1]), DRAWN_BOARDS + small);
            seen += 1;
        }
    }
    assert!(
        seen > 0,
        "no game ended by filling a board in 2,000 random games"
    );
}

#[test]
fn move_features_are_the_difference_of_active_features() {
    for game in random_games(150) {
        for (before, mv) in game {
            let mut after = before;
            after.play(mv);
            for seat in 0..2 {
                let old = features(&before, seat);
                let new = features(&after, seat);
                let mut list = [0; 2];
                let len = move_features(&before, mv, seat, &mut list);
                assert!((1..=2).contains(&len));
                let mut added: Vec<u16> =
                    new.iter().copied().filter(|f| !old.contains(f)).collect();
                added.sort_unstable();
                assert_eq!(added, list[..len].to_vec());
                assert!(old.iter().all(|f| new.contains(f)), "a feature became 0");
                assert_eq!(new.len(), old.len() + len);
            }
        }
    }
}

#[test]
fn a_move_that_wins_the_game_still_gives_the_cell_and_the_board() {
    let mut seen = 0;
    for game in random_games(300) {
        for (before, mv) in game {
            let mut after = before;
            after.play(mv);
            if !matches!(after.status(), Status::Win(_)) || after.small_winner(mv.board()).is_none()
            {
                continue;
            }
            let mover = before.to_move();
            let mut list = [0; 2];
            assert_eq!(move_features(&before, mv, mover, &mut list), 2);
            assert_eq!(usize::from(list[0]), 9 * mv.board() + mv.cell());
            assert_eq!(usize::from(list[1]), OUR_BOARDS + mv.board());
            assert_eq!(move_features(&before, mv, 1 - mover, &mut list), 2);
            assert_eq!(
                usize::from(list[0]),
                THEIR_CELLS + 9 * mv.board() + mv.cell()
            );
            assert_eq!(usize::from(list[1]), THEIR_BOARDS + mv.board());
            seen += 1;
        }
    }
    assert!(
        seen > 0,
        "no game ended with a small board in 300 random games"
    );
}

fn check_incremental<const N: usize>(games: u64) {
    let (parts, net) = random_network::<N>(N as u64);
    for game in random_games(games) {
        let mut board = Board::new();
        let mut accumulator = net.refresh(&board);
        assert_eq!(accumulator, net.refresh(&Board::new()));
        for (before, mv) in game {
            assert_eq!(before, board);
            net.play(&mut accumulator, &mut board, mv);
            let mut expected = before;
            expected.play(mv);
            assert_eq!(board, expected);
            assert_eq!(
                accumulator,
                net.refresh(&board),
                "after {mv} from {before:?}"
            );
            if board.status() == Status::Ongoing {
                let logit = f64::from(net.logit(&accumulator, &board));
                // The integer division truncates by less than one unit of
                // 1 / (QA * QB), about 1.2e-4; the rest is f32 rounding.
                assert!((logit - reference_logit(&parts, &accumulator, &board)).abs() < 1e-3);
                let score = net.evaluate(&accumulator, &board);
                assert!((0.0..=1.0).contains(&score));
                assert!((f64::from(score) - 1.0 / (1.0 + (-logit).exp())).abs() < 1e-6);
                assert_eq!(score, net.evaluate_board(&board));
            }
        }
    }
}

#[test]
fn incremental_updates_equal_refresh_width_16() {
    check_incremental::<16>(200);
}

#[test]
fn incremental_updates_equal_refresh_width_256() {
    check_incremental::<256>(200);
}

#[test]
fn logit_matches_a_hand_computation() {
    // Width 1: all first-layer weights 0, bias 100, output weights 3 and 5.
    let mut parts = random_parts(1, 1, 0, 0, 0);
    parts.feature_bias = vec![100];
    parts.output_weights = vec![0; BUCKETS * 2];
    parts.output_weights[9 * 2] = 3;
    parts.output_weights[9 * 2 + 1] = 5;
    parts.output_bias = [0; BUCKETS];
    parts.output_bias[9] = 1_000;
    let net = network::<1>(&parts);
    let board = Board::new();
    let sum = 100 * 100 * 3 + 100 * 100 * 5;
    let expected = (sum / 127 + 1_000) as f32 / (127.0 * 64.0);
    assert_eq!(net.logit(&net.refresh(&board), &board), expected);
}

#[test]
fn bytes_round_trip() {
    let (parts, net) = random_network::<16>(3);
    let bytes = net.to_bytes();
    assert_eq!(bytes.len(), Network::<16>::BYTES);
    assert_eq!(&bytes[..8], MAGIC);
    assert_eq!(&bytes[8..12], &16u32.to_le_bytes());
    assert_eq!(Network::<16>::from_bytes(&bytes).unwrap(), net);
    assert_eq!(network::<16>(&parts), net);
    let (_, big) = random_network::<256>(4);
    assert_eq!(big.to_bytes().len(), 64_388);
    assert_eq!(Network::<256>::from_bytes(&big.to_bytes()).unwrap(), big);
}

#[test]
fn bytes_follow_the_layout() {
    let parts = random_parts(2, 9, 127, 21_337, 127);
    let net = network::<2>(&parts);
    let bytes = net.to_bytes();
    assert_eq!(bytes[12] as i8, parts.feature_weights[0]);
    let bias = 12 + FEATURES * 2;
    assert_eq!(
        i16::from_le_bytes([bytes[bias], bytes[bias + 1]]),
        parts.feature_bias[0]
    );
    let output = bias + 4;
    assert_eq!(bytes[output] as i8, parts.output_weights[0]);
    let last = bytes.len() - 4;
    assert_eq!(
        i32::from_le_bytes(bytes[last..].try_into().unwrap()),
        parts.output_bias[BUCKETS - 1]
    );
}

#[test]
fn from_bytes_rejects_malformed_input() {
    let (_, net) = random_network::<16>(3);
    let good = net.to_bytes();
    assert!(Network::<16>::from_bytes(&good).is_ok());

    assert!(Network::<16>::from_bytes(&[]).is_err());
    for len in [1, 7, 8, 11, 12, 13, good.len() - 1] {
        assert!(Network::<16>::from_bytes(&good[..len]).is_err(), "{len}");
    }
    let mut long = good.clone();
    long.push(0);
    assert!(Network::<16>::from_bytes(&long).is_err());

    let mut bad = good.clone();
    bad[0] ^= 1;
    assert!(Network::<16>::from_bytes(&bad).is_err());

    // Another width, both by the header and by the length.
    assert!(Network::<32>::from_bytes(&good).is_err());
    let mut bad = good.clone();
    bad[8] = 17;
    assert!(Network::<16>::from_bytes(&bad).is_err());
    let mut bad = good.clone();
    bad[8..12].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(Network::<16>::from_bytes(&bad).is_err());

    // A bias above MAX_BIAS, or its negative below -MAX_BIAS.
    let bias = 12 + FEATURES * 16;
    for value in [MAX_BIAS + 1, -MAX_BIAS - 1, i16::MIN, i16::MAX] {
        let mut bad = good.clone();
        bad[bias..bias + 2].copy_from_slice(&value.to_le_bytes());
        assert!(Network::<16>::from_bytes(&bad).is_err(), "{value}");
    }
    for value in [MAX_BIAS, -MAX_BIAS] {
        let mut bad = good.clone();
        bad[bias..bias + 2].copy_from_slice(&value.to_le_bytes());
        assert!(Network::<16>::from_bytes(&bad).is_ok(), "{value}");
    }

    // A weight of -128 in either layer.
    for at in [12, bias + 2 * 16] {
        let mut bad = good.clone();
        bad[at] = 0x80;
        assert!(Network::<16>::from_bytes(&bad).is_err(), "{at}");
    }
}

#[test]
fn from_bytes_never_panics_on_noise() {
    let (_, net) = random_network::<16>(3);
    let good = net.to_bytes();
    let mut rng = Rng::new(77);
    for _ in 0..300 {
        let mut bytes = good.clone();
        for _ in 0..1 + rng.below(4) {
            let at = rng.below(bytes.len() as u64) as usize;
            bytes[at] = rng.next_u64() as u8;
        }
        let _ = Network::<16>::from_bytes(&bytes);
        bytes.truncate(rng.below(bytes.len() as u64 + 1) as usize);
        let _ = Network::<16>::from_bytes(&bytes);
    }
}

#[test]
fn from_parts_rejects_wrong_parts() {
    let parts = random_parts(4, 1, 20, 60, 40);
    let build =
        |fw: &[i8], fb: &[i16], ow: &[i8]| Network::<4>::from_parts(fw, fb, ow, &parts.output_bias);
    assert!(build(
        &parts.feature_weights,
        &parts.feature_bias,
        &parts.output_weights
    )
    .is_ok());
    assert!(build(
        &parts.feature_weights[1..],
        &parts.feature_bias,
        &parts.output_weights
    )
    .is_err());
    assert!(build(
        &parts.feature_weights,
        &parts.feature_bias[1..],
        &parts.output_weights
    )
    .is_err());
    assert!(build(
        &parts.feature_weights,
        &parts.feature_bias,
        &parts.output_weights[1..]
    )
    .is_err());

    let empty = Network::<0>::from_parts(&[], &[], &[], &[0; BUCKETS]);
    assert!(empty.is_err());
    let wide = random_parts(MAX_WIDTH + 1, 1, 20, 60, 40);
    assert!(Network::<{ MAX_WIDTH + 1 }>::from_parts(
        &wide.feature_weights,
        &wide.feature_bias,
        &wide.output_weights,
        &wide.output_bias
    )
    .is_err());
    let widest = random_parts(MAX_WIDTH, 1, 20, 60, 40);
    assert!(Network::<MAX_WIDTH>::from_parts(
        &widest.feature_weights,
        &widest.feature_bias,
        &widest.output_weights,
        &widest.output_bias
    )
    .is_ok());
}

#[test]
fn an_extreme_network_does_not_overflow_in_random_games() {
    const N: usize = MAX_WIDTH;
    let parts = Parts {
        feature_weights: vec![127; FEATURES * N],
        feature_bias: vec![MAX_BIAS; N],
        output_weights: vec![127; BUCKETS * 2 * N],
        output_bias: [i32::MAX; BUCKETS],
    };
    let net = network::<N>(&parts);
    let negative = Parts {
        feature_weights: vec![-127; FEATURES * N],
        feature_bias: vec![-MAX_BIAS; N],
        output_weights: vec![-127; BUCKETS * 2 * N],
        output_bias: [i32::MIN; BUCKETS],
    };
    let low = network::<N>(&negative);
    for game in random_games(20) {
        let mut board = Board::new();
        let mut replay = Board::new();
        let mut accumulator = net.refresh(&board);
        let mut low_accumulator = low.refresh(&board);
        for (_, mv) in game {
            net.play(&mut accumulator, &mut board, mv);
            low.play(&mut low_accumulator, &mut replay, mv);
            assert_eq!(accumulator, net.refresh(&board));
            assert_eq!(low_accumulator, low.refresh(&replay));
            if board.status() == Status::Ongoing {
                assert!(net.logit(&accumulator, &board).is_finite());
                assert!(low.logit(&low_accumulator, &replay).is_finite());
                assert!(net.evaluate(&accumulator, &board) <= 1.0);
            }
        }
    }
}

#[test]
fn the_largest_possible_sum_fits_in_i32() {
    // Every accumulator at QA and every weight at 127, in the widest network.
    let acc = [[QA as i16; MAX_WIDTH]; 2];
    let weights = vec![127i16; MAX_WIDTH];
    let sum = squared_dot(&acc[0], &weights) + squared_dot(&acc[1], &weights);
    assert_eq!(i64::from(sum), 2 * MAX_WIDTH as i64 * 16_129 * 127);
}

#[test]
fn phase_counts_closed_boards() {
    let mut board = Board::new();
    assert_eq!(phase(&board), 0);
    assert_eq!(bucket(&board), 9);
    let mut closed = 0;
    let mut rng = Rng::new(5);
    let mut seen = [false; PHASES];
    while board.status() == Status::Ongoing {
        let count = board.closed_boards().count_ones();
        let expected = match count {
            0 => 0,
            1 | 2 => 1,
            _ => 2,
        };
        assert_eq!(phase(&board), expected, "{count} closed");
        assert_eq!(
            bucket(&board),
            board.target().unwrap_or(9) + TARGETS * expected
        );
        seen[expected] = true;
        closed = closed.max(count);
        let mv = board.random_move(&mut rng);
        board.play(mv);
    }
    assert!(closed >= 3, "the game closed only {closed} boards");
    assert_eq!(seen, [true; PHASES]);
}

#[test]
fn first_version_bytes_serve_every_phase() {
    // A network of the first version: one bucket per target.
    let parts = random_parts(16, 11, 20, 60, 40);
    let mut bytes = MAGIC_V1.to_vec();
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend(parts.feature_weights.iter().map(|&w| w as u8));
    for bias in &parts.feature_bias {
        bytes.extend_from_slice(&bias.to_le_bytes());
    }
    bytes.extend(
        parts.output_weights[..TARGETS * 2 * 16]
            .iter()
            .map(|&w| w as u8),
    );
    for bias in &parts.output_bias[..TARGETS] {
        bytes.extend_from_slice(&bias.to_le_bytes());
    }
    assert_eq!(bytes.len(), Network::<16>::BYTES_V1);
    let read = Network::<16>::from_bytes(&bytes).unwrap();

    // The same network with each target's bucket copied to every phase.
    let mut same = random_parts(16, 11, 20, 60, 40);
    same.output_weights = parts.output_weights[..TARGETS * 2 * 16].repeat(PHASES);
    for bucket in 0..BUCKETS {
        same.output_bias[bucket] = parts.output_bias[bucket % TARGETS];
    }
    assert_eq!(read, network::<16>(&same));

    // Its evaluations are those of a network that ignores the phase.
    for game in random_games(20) {
        for (board, _) in game {
            let accumulator = read.refresh(&board);
            let target = board.target().unwrap_or(9);
            let start = target * 2 * 16;
            let weights = &parts.output_weights[start..start + 2 * 16];
            let stm = board.to_move();
            let sum = squared_dot(&accumulator.values[stm], &widen(&weights[..16]))
                + squared_dot(&accumulator.values[1 - stm], &widen(&weights[16..]));
            let total = i64::from(sum / QA) + i64::from(parts.output_bias[target]);
            let expected = total as f32 / (QA * QB) as f32;
            assert_eq!(read.logit(&accumulator, &board), expected);
        }
    }

    // A first-version header with the second version's length, and the
    // reverse, are rejected.
    let mut wrong = bytes.clone();
    wrong[..8].copy_from_slice(MAGIC);
    assert!(Network::<16>::from_bytes(&wrong).is_err());
    let mut wrong = read.to_bytes();
    assert_eq!(&wrong[..8], MAGIC);
    wrong[..8].copy_from_slice(MAGIC_V1);
    assert!(Network::<16>::from_bytes(&wrong).is_err());
}

fn widen(weights: &[i8]) -> Vec<i16> {
    weights.iter().map(|&w| i16::from(w)).collect()
}
