use super::*;

#[test]
fn games_are_reproducible_and_record_searched_positions() {
    let settings = SelfPlay {
        iterations: 300,
        opening_plies: 4,
        exploration: 0.5,
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
        let mut legal = MoveList::new();
        position.legal_moves(&mut legal);
        assert_eq!(searched.visits.len(), legal.len());
        assert!(searched.visits.iter().all(|(mv, _)| legal.contains(mv)));
        // The move played is the most visited one.
        let best = searched.visits.iter().max_by_key(|(_, n)| *n).unwrap().0;
        assert_eq!(best, game.moves[usize::from(searched.ply)]);
    }
}
