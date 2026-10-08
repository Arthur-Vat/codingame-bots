use uttt_engine::MoveList;

use super::*;

#[test]
fn games_are_reproducible_and_record_searched_positions() {
    let settings = SelfPlay {
        iterations: 300,
        opening_plies: 4,
        exploration: 0.5,
        policy: None,
        record_visits: true,
    };
    let game = settings.play(11);
    assert_eq!(game, settings.play(11));
    assert_ne!(game, settings.play(12));
    let mut board = Board::new();
    for &mv in &game.moves {
        assert!(board.is_legal(mv));
        board.play(mv);
    }
    assert_ne!(board.status(), Status::Ongoing);
    assert!(!game.searched.is_empty());
    for (position, searched) in game.positions() {
        assert!(searched.ply >= 4, "openings are not searched");
        assert!(position.game_winning_move().is_none());
        let score = searched.score.unwrap();
        assert!((0.0..=1.0).contains(&score), "{score}");
        let mut legal = MoveList::new();
        position.legal_moves(&mut legal);
        assert_eq!(searched.visits.len(), legal.len());
        assert!(searched.visits.iter().all(|(mv, _)| legal.contains(mv)));
        // The move played is the most visited one.
        let best = searched.visits.iter().max_by_key(|(_, n)| *n).unwrap().0;
        assert_eq!(best, game.moves[usize::from(searched.ply)]);
    }
}

/// `uttt-v008`'s weights, as the Train workflow reads them.
const BOT_WEIGHTS: &str = include_str!("../../../bots/mcts/src/weights.rs");

#[test]
fn the_bot_weights_are_read_from_its_source() {
    let weights = read_policy_weights(BOT_WEIGHTS).unwrap();
    assert_eq!(weights[0], 800);
    assert_eq!(weights[1], 10_000);
    assert_eq!(weights[31], 1070);
    assert!(read_policy_weights("no weights here").is_err());
    let short = "pub const PLAYOUT_WEIGHTS: [u32; 32] = [1, 2, 3];";
    assert_eq!(
        read_policy_weights(short).unwrap_err(),
        "3 weights instead of 32"
    );
}

#[test]
fn games_with_the_bots_playouts_can_skip_visits() {
    let weights = read_policy_weights(BOT_WEIGHTS).unwrap();
    let policy: &'static PlayoutPolicy =
        Box::leak(Box::new(PlayoutPolicy::new(weights).for_plies(16)));
    let settings = SelfPlay {
        iterations: 300,
        opening_plies: 6,
        exploration: 0.5,
        policy: Some(policy),
        record_visits: false,
    };
    let game = settings.play(3);
    assert_eq!(game, settings.play(3));
    assert_ne!(game.status(), Status::Ongoing);
    assert!(!game.searched.is_empty());
    for searched in &game.searched {
        assert!(searched.visits.is_empty());
        assert!((0.0..=1.0).contains(&searched.score.unwrap()));
    }
}
