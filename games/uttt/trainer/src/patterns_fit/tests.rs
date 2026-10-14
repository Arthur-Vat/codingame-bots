use uttt_engine::{Board, Move, MoveList, Status};

use uttt_policy::{
    encode_pattern_bytes, encode_pattern_weights, PatternPolicy, CLASSES, PATTERN_CELLS,
    PATTERN_FEATURES,
};

use super::*;
use crate::data::Searched;

/// A game of random moves where every position's visits favour the
/// centre cell of its board when it is free, and are even otherwise.
fn centre_loving_game(seed: u64) -> GameRecord {
    let mut rng = Rng::new(seed);
    let mut board = Board::new();
    let mut record = GameRecord::default();
    let mut moves = MoveList::new();
    while board.status() == Status::Ongoing {
        board.legal_moves(&mut moves);
        let visits = moves
            .iter()
            .map(|&mv| (mv, if mv.cell() == 4 { 50 } else { 5 }))
            .collect();
        record.searched.push(Searched {
            ply: record.moves.len() as u8,
            score: Some(0.5),
            visits,
        });
        let mv: Move = *rng.pick(&moves).unwrap();
        record.moves.push(mv);
        board.play(mv);
    }
    record
}

#[test]
fn examples_store_every_move_and_hold_out_whole_games() {
    let games: Vec<GameRecord> = (0..40).map(centre_loving_game).collect();
    for model in ALL {
        let mut split = Split::new(model);
        split.add(&games[..7]);
        split.add(&games[7..]);
        assert_eq!(split.fitted.games + split.held_out.games, 40);
        assert_eq!(split.held_out.games, 2);
        let tables = model.tables();
        for examples in [&split.fitted, &split.held_out] {
            assert_eq!(examples.per_move, tables.len());
            assert_eq!(examples.features.len(), examples.moves() * tables.len());
            for position in 0..examples.positions() {
                let total: f32 = examples.shares[examples.range(position)].iter().sum();
                assert!((total - 1.0).abs() < 1e-4);
            }
            // Each feature falls in its own table.
            for index in 0..examples.moves() {
                let mut offset = 0;
                for (&feature, &size) in examples.of(index).iter().zip(&tables) {
                    let feature = feature as usize;
                    assert!(
                        (offset..offset + size).contains(&feature),
                        "{model:?}: {feature} outside {offset}..{}",
                        offset + size
                    );
                    offset += size;
                }
            }
        }
    }
}

#[test]
fn symmetric_moves_of_symmetric_games_have_the_same_features() {
    use uttt_policy::symmetric_cell;
    let mirror = |symmetry: usize, mv: Move| {
        Move::new(
            symmetric_cell(symmetry, mv.board()),
            symmetric_cell(symmetry, mv.cell()),
        )
    };
    let mut rng = Rng::new(8);
    let mut moves = MoveList::new();
    let (mut features, mut images) = (Vec::new(), Vec::new());
    let mut checked = 0;
    for game in 0..40 {
        let symmetry = game % 8;
        let (mut board, mut image) = (Board::new(), Board::new());
        while board.status() == Status::Ongoing {
            board.legal_moves(&mut moves);
            for &mv in moves.iter() {
                for model in ALL {
                    features.clear();
                    images.clear();
                    model.features(&board, mv, &mut features);
                    model.features(&image, mirror(symmetry, mv), &mut images);
                    assert_eq!(
                        features, images,
                        "{model:?}, symmetry {symmetry}, move {mv}"
                    );
                }
                checked += 1;
            }
            let mv = *rng.pick(&moves).unwrap();
            board.play(mv);
            image.play(mirror(symmetry, mv));
        }
    }
    assert!(checked > 5_000);
}

#[test]
fn models_are_named_and_sized() {
    for model in ALL {
        assert_eq!(Model::from_name(model.name()), Some(model));
        assert_eq!(model.weights(), model.tables().iter().sum::<usize>());
    }
    assert_eq!(Model::from_name("none"), None);
    assert_eq!(Model::Patterns.weights(), PATTERN_FEATURES);
    assert_eq!(Model::Classes.weights(), CLASSES);
    // Open small-board patterns up to symmetry: fewer than all 3^9
    // patterns over 8, more than the open pattern-cell pairs over 9.
    let patterns = models::destination_patterns();
    assert!(
        patterns > PATTERN_CELLS / 9 && patterns < 19_683 / 8 + 100,
        "{patterns}"
    );
}

#[test]
fn fitting_learns_what_the_visits_prefer() {
    let games: Vec<GameRecord> = (0..200).map(centre_loving_game).collect();
    let settings = Fitting {
        epochs: 3,
        batch: 64,
        rate: 0.05,
        penalty: 0.0,
        seed: 3,
    };
    for model in ALL {
        let mut split = Split::new(model);
        split.add(&games);
        let uniform = measure(&vec![0.0; model.weights()], &split.held_out);
        let mut reports = 0;
        let theta = fit(model.weights(), &split.fitted, &settings, |_, _| {
            reports += 1
        });
        assert_eq!(reports, 3);
        let fitted = measure(&theta, &split.held_out);
        // Every model can tell the centre cell: the classes by a feature,
        // the patterns by the cell.
        assert!(
            fitted.cross_entropy < uniform.cross_entropy - 0.05,
            "{model:?}: {fitted:?} against {uniform:?}"
        );
        assert!(
            fitted.best_move > uniform.best_move + 0.1,
            "{model:?}: {fitted:?} against {uniform:?}"
        );
    }
}

#[test]
fn the_weights_source_holds_the_packed_weights() {
    let mut rng = Rng::new(7);
    let log_weights: Vec<f32> = (0..PATTERN_FEATURES)
        .map(|_| (rng.unit() * 16.0 - 8.0) as f32)
        .collect();
    let bytes = encode_pattern_bytes(&log_weights);
    let source = weights_source(Model::Patterns, &bytes, "a test").unwrap();
    assert!(source.starts_with(
        "// Written by cg-packer from `uttt-trainer fit-patterns`, a test: 19707 bytes in "
    ));
    assert!(source.contains("pub const PATTERN_PACKED: &str = \""));
    // The bot's reader and the analysis program's find the same policy.
    let expected = PatternPolicy::decode(&encode_pattern_weights(&log_weights)).unwrap();
    let read = crate::selfplay::read_pattern_policy(&source).unwrap();
    assert_eq!(read, expected);
    // Another model's module holds its own number of weights, which no pattern
    // policy has.
    let other = weights_source(Model::Rich, &bytes[..127], "a test").unwrap();
    assert!(other.contains("`rich` model"));
    assert!(crate::selfplay::read_pattern_policy(&other).is_err());
    assert!(weights_source(Model::Patterns, &bytes, "a\ntest")
        .unwrap()
        .starts_with("// Written by cg-packer from `uttt-trainer fit-patterns`, a test: "));
}

#[test]
fn entropy_counts_fewer_characters_for_skewed_text() {
    assert_eq!(entropy_characters(&"A".repeat(600)), 0);
    // Two digits equally often: one bit each, six to a character.
    assert_eq!(entropy_characters(&"AB".repeat(300)), 100);
    let even: String = (0..640)
        .map(|i| {
            char::from(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"[i % 64])
        })
        .collect();
    assert_eq!(entropy_characters(&even), 640);
}
