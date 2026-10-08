use super::*;

fn sample() -> GameRecord {
    let moves = vec![Move::new(4, 4), Move::new(4, 0), Move::new(0, 8)];
    GameRecord {
        searched: vec![
            Searched {
                ply: 1,
                score: Some(0.625),
                visits: vec![(Move::new(4, 0), 7), (Move::new(4, 8), 70_000)],
            },
            Searched {
                ply: 2,
                score: Some(0.25),
                visits: vec![(Move::new(0, 8), 1)],
            },
            Searched {
                ply: 3,
                score: Some(1.0),
                visits: Vec::new(),
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
    assert_eq!(positions, vec![replay(1), replay(2), replay(3)]);
    assert_eq!(positions[0].target(), Some(4));
    assert_eq!(positions[1].target(), Some(0));
}

#[test]
fn files_of_the_first_version_are_still_read() {
    // Version 1 had no root score.
    let mut bytes = MAGIC_V1.to_vec();
    bytes.extend_from_slice(&[2, 40, 36, 1, 1, 1, 36]);
    bytes.extend_from_slice(&9u32.to_le_bytes());
    let games = read_games(&mut bytes.as_slice()).unwrap();
    let expected = GameRecord {
        moves: vec![Move::new(4, 4), Move::new(4, 0)],
        searched: vec![Searched {
            ply: 1,
            score: None,
            visits: vec![(Move::new(4, 0), 9)],
        }],
    };
    assert_eq!(games, vec![expected]);
}

#[test]
fn the_status_is_the_end_of_the_game() {
    assert_eq!(sample().status(), Status::Ongoing);
    let mut board = Board::new();
    let mut moves = Vec::new();
    let mut rng = cg_core::rng::Rng::new(5);
    while board.status() == Status::Ongoing {
        let mv = board.random_move(&mut rng);
        moves.push(mv);
        board.play(mv);
    }
    let game = GameRecord {
        moves,
        searched: Vec::new(),
    };
    assert_eq!(game.status(), board.status());
}
