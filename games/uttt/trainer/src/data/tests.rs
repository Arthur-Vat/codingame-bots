use super::*;

fn sample() -> GameRecord {
    let moves = vec![Move::new(4, 4), Move::new(4, 0), Move::new(0, 8)];
    GameRecord {
        searched: vec![
            Searched {
                ply: 1,
                visits: vec![(Move::new(4, 0), 7), (Move::new(4, 8), 70_000)],
            },
            Searched {
                ply: 2,
                visits: vec![(Move::new(0, 8), 1)],
            },
        ],
        moves,
    }
}

#[test]
fn games_survive_a_round_trip() {
    let games = vec![sample(), GameRecord::default(), sample()];
    let mut bytes = Vec::new();
    write_header(&mut bytes).unwrap();
    for game in &games {
        write_game(&mut bytes, game).unwrap();
    }
    assert_eq!(read_games(&mut bytes.as_slice()).unwrap(), games);
}

#[test]
fn rejects_other_and_truncated_files() {
    assert!(read_games(&mut &b"not data"[..]).is_err());
    let mut bytes = Vec::new();
    write_header(&mut bytes).unwrap();
    write_game(&mut bytes, &sample()).unwrap();
    bytes.pop();
    assert!(read_games(&mut bytes.as_slice()).is_err());
}

#[test]
fn positions_replay_the_moves() {
    let game = sample();
    let positions: Vec<Board> = game.positions().map(|(board, _)| board).collect();
    let replay = |ply: usize| {
        let mut board = Board::new();
        for &mv in &game.moves[..ply] {
            board.play(mv);
        }
        board
    };
    assert_eq!(positions, vec![replay(1), replay(2)]);
    assert_eq!(positions[0].target(), Some(4));
    assert_eq!(positions[1].target(), Some(0));
}
