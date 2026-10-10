use super::*;

fn answer(seat: usize, line: &str, ms: f64) -> RecordedAnswer {
    RecordedAnswer {
        seat,
        lines: vec![line.to_string()],
        ms,
    }
}

fn played(winner: Option<usize>, end: EndReason) -> MatchRecord {
    MatchRecord {
        seed: 42,
        seats: ["a".to_string(), "b".to_string()],
        winner,
        end,
        turns: 2,
        max_answer_ms: [12.5, 8.0],
        mean_answer_ms: [12.5, 8.0],
        later_answer_ms: Default::default(),
        recorded_turns: vec![vec![answer(0, "4 4", 12.5)], vec![answer(1, "3 3", 8.0)]],
    }
}

fn game(swapped: bool, winner: Option<usize>) -> GameRecord {
    GameRecord {
        pair: 0,
        swapped,
        game: played(winner, EndReason::Finished),
    }
}

fn fault_game() -> GameRecord {
    GameRecord {
        pair: 0,
        swapped: false,
        game: played(
            Some(1),
            EndReason::Timeout {
                seat: 0,
                limit_ms: 100.0,
            },
        ),
    }
}

/// How many of `games` the sampler keeps.
fn kept(sampler: &mut Sampler, games: &[GameRecord]) -> usize {
    games.iter().filter(|game| sampler.keep(game)).count()
}

#[test]
fn without_a_limit_every_game_is_kept() {
    let mut sampler = Sampler::new(None);
    let games: Vec<GameRecord> = (0..50).map(|_| game(false, Some(0))).collect();
    assert_eq!(kept(&mut sampler, &games), 50);
}

#[test]
fn a_limit_caps_each_result_at_a_third_rounded_up() {
    let mut sampler = Sampler::new(Some(10));
    let wins: Vec<GameRecord> = (0..6).map(|_| game(false, Some(0))).collect();
    assert_eq!(kept(&mut sampler, &wins), 4, "ceil(10 / 3) wins");
    let losses: Vec<GameRecord> = (0..6).map(|_| game(false, Some(1))).collect();
    assert_eq!(kept(&mut sampler, &losses), 4);
    let draws: Vec<GameRecord> = (0..6).map(|_| game(false, None)).collect();
    assert_eq!(kept(&mut sampler, &draws), 2, "10 in total");
    assert!(!sampler.keep(&game(false, Some(0))));
    assert!(!sampler.keep(&game(false, None)));
}

#[test]
fn results_are_those_of_the_first_bot_when_seats_are_swapped() {
    let mut sampler = Sampler::new(Some(3));
    // Seat 1 wins a swapped game: the first bot wins. Each result allows one.
    assert!(sampler.keep(&game(true, Some(1))));
    assert!(!sampler.keep(&game(false, Some(0))), "a second win");
    assert!(sampler.keep(&game(true, Some(0))), "a loss");
    assert!(!sampler.keep(&game(false, Some(1))), "a second loss");
    assert!(sampler.keep(&game(true, None)), "a draw");
}

#[test]
fn a_limit_of_zero_keeps_only_faults() {
    let mut sampler = Sampler::new(Some(0));
    assert!(!sampler.keep(&game(false, Some(0))));
    assert!(!sampler.keep(&game(false, None)));
    assert!(sampler.keep(&fault_game()));
}

#[test]
fn faults_are_kept_beyond_the_limit_and_do_not_use_it() {
    let mut sampler = Sampler::new(Some(3));
    for _ in 0..10 {
        assert!(sampler.keep(&fault_game()));
    }
    assert!(sampler.keep(&game(false, Some(0))));
    assert!(sampler.keep(&game(false, Some(1))));
    assert!(sampler.keep(&game(false, None)));
    assert!(!sampler.keep(&game(false, None)));
}

#[test]
fn a_record_round_trips_through_json() {
    let options = MatchOptions {
        time_scale: 2.0,
        ..MatchOptions::default()
    };
    let record = Record::from_match(
        "uttt",
        "arena match",
        4,
        &played(Some(0), EndReason::Finished),
        &options,
    );
    assert_eq!(record.format, 1);
    assert_eq!(RECORD_FORMAT, 1);
    assert_eq!(record.players[1].name, "b");
    assert_eq!(record.players[1].kind, PlayerKind::Bot);
    assert_eq!(record.players[1].bot_seed, Some(bot_seed(42, 1, "b")));
    assert_eq!(record.players[0].time_scale, 2.0);
    assert_eq!(record.turns.len(), 2);

    let json = serde_json::to_string(&record).unwrap();
    assert!(json.contains("\"kind\":\"bot\""), "{json}");
    assert!(json.contains("\"end\":{\"kind\":\"finished\"}"), "{json}");
    let back: Record = serde_json::from_str(&json).unwrap();
    assert_eq!(back, record);
}

#[test]
fn a_fault_ending_round_trips_through_json() {
    let end = EndReason::Invalid {
        seat: 1,
        reason: "no".to_string(),
    };
    let json = serde_json::to_string(&end).unwrap();
    assert_eq!(serde_json::from_str::<EndReason>(&json).unwrap(), end);
}

#[test]
fn reads_the_fixed_iteration_count_like_the_bots() {
    assert_eq!(fixed_iters(Some("500")), Some(500));
    assert_eq!(fixed_iters(Some(" 7\n")), Some(7));
    assert_eq!(fixed_iters(Some("0")), None);
    assert_eq!(fixed_iters(Some("many")), None);
    assert_eq!(fixed_iters(None), None);
}
