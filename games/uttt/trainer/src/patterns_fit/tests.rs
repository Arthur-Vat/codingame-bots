use uttt_engine::{Board, Move, MoveList, Status};

use uttt_engine::board::{CLASSES, PATTERN_CELLS, PATTERN_FEATURES};

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
    use uttt_engine::board::symmetric_cell;
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
fn the_weights_source_holds_the_text() {
    let text = "Ab+/".repeat(60);
    let source = weights_source(Model::Patterns, &text, "a test");
    assert!(source.starts_with("//! Generated by `uttt-trainer fit-patterns`: a test.\n"));
    let start = source.find("= \"\\\n").unwrap() + 5;
    let end = source.rfind("\";").unwrap();
    assert_eq!(source[start..end].replace("\\\n", ""), text);
    let read = crate::selfplay::read_pattern_text(&source).unwrap();
    assert_eq!(read, text);
    let other = weights_source(Model::Rich, &text, "a test");
    assert!(other.contains("`rich` model"));
    assert!(other.contains("pub const PACKED_TEXT: &str"));
    assert!(crate::selfplay::read_pattern_text(&other).is_err());
}

#[test]
fn packed_weights_come_back_table_by_table() {
    let mut rng = Rng::new(5);
    for model in ALL {
        let theta: Vec<f32> = (0..model.weights())
            .map(|i| {
                if i % 3 == 0 {
                    (rng.below(40) as f32 - 20.0) / 4.0
                } else {
                    0.0
                }
            })
            .collect();
        let quantized = quantize(&theta, 0.5);
        assert!(quantized.iter().all(|&q| q < 64));
        let mut rest = packed_text(model, &quantized);
        let mut decoded = Vec::new();
        for (size, stride) in model.tables().into_iter().zip(model.strides()) {
            let (table, after) = cg_core::packed::decode_strided(&rest, stride).unwrap();
            assert_eq!(table.len(), size, "{model:?}");
            decoded.extend(table);
            rest = after.to_string();
        }
        assert_eq!(rest, "");
        assert_eq!(decoded, quantized, "{model:?}");
    }
    // Twice the log-weight at half the temperature; the ends clamp.
    assert_eq!(
        quantize(&[1.0, -1.0, 100.0, -100.0], 0.5),
        vec![40, 24, 63, 0]
    );
}

#[test]
fn the_destination_models_are_the_engines_context_policy() {
    use uttt_engine::board::DESTINATION_WEIGHTS;
    use uttt_engine::ContextPolicy;
    assert_eq!(
        Model::DestinationRoles.tables(),
        vec![PATTERN_FEATURES, DESTINATION_WEIGHTS]
    );
    let one =
        ContextPolicy::new(vec![32; PATTERN_FEATURES], vec![32; DESTINATION_WEIGHTS]).unwrap();
    let two = ContextPolicy::new(
        vec![32; 2 * PATTERN_FEATURES],
        vec![32; DESTINATION_WEIGHTS],
    )
    .unwrap();
    let mut rng = Rng::new(12);
    let mut moves = MoveList::new();
    let mut features = Vec::new();
    for _ in 0..60 {
        let mut board = Board::new();
        while board.status() == Status::Ongoing {
            board.legal_moves(&mut moves);
            for &mv in moves.iter() {
                for (model, policy, offset) in [
                    (Model::DestinationRoles, &one, PATTERN_FEATURES),
                    (Model::PhasesDestinations, &two, 2 * PATTERN_FEATURES),
                ] {
                    features.clear();
                    model.features(&board, mv, &mut features);
                    let [first, second] = board.context_features(policy, mv);
                    let expected = vec![first as u32, (offset + second) as u32];
                    assert_eq!(features, expected, "{model:?}, move {mv}");
                }
            }
            let mv = *rng.pick(&moves).unwrap();
            board.play(mv);
        }
    }
}
