use std::time::Duration;

use super::*;
use crate::referee::{Answer, GameSetup, InvalidAnswer, Outcome, Referee, TimeLimits};

/// Seat 0 wins when the seed is even, seat 1 when it is odd, after one turn.
struct SeedDecides {
    seed: u64,
    played: bool,
}

impl Referee for SeedDecides {
    fn time_limits(&self) -> TimeLimits {
        TimeLimits {
            first_answer: Duration::from_secs(2),
            later_answers: Duration::from_secs(1),
        }
    }

    fn players_to_act(&self) -> Vec<usize> {
        vec![0]
    }

    fn turn_input(&self, _seat: usize) -> String {
        "go\n".to_string()
    }

    fn play(&mut self, _answers: &[Answer]) -> Result<(), InvalidAnswer> {
        self.played = true;
        Ok(())
    }

    fn outcome(&self) -> Option<Outcome> {
        self.played
            .then_some(Outcome::Win((self.seed % 2) as usize))
    }
}

fn echo_bot(name: &str) -> BotSpec {
    BotSpec {
        name: name.to_string(),
        program: "sh".to_string(),
        args: vec![
            "-c".to_string(),
            "while read l; do echo x; done".to_string(),
        ],
    }
}

fn tournament(pairs: u32, jobs: usize) -> Tournament {
    Tournament {
        bots: [echo_bot("a"), echo_bot("b")],
        pairs,
        seed: 3,
        opening_plies: 0,
        jobs,
        options: MatchOptions::default(),
    }
}

fn new_referee(setup: &GameSetup) -> Box<dyn Referee> {
    Box::new(SeedDecides {
        seed: setup.seed,
        played: false,
    })
}

#[test]
fn plays_every_pair_twice_with_seats_swapped() {
    let mut games = Vec::new();
    run(&tournament(5, 3), &new_referee, |game| {
        games.push(game.clone());
        Flow::Continue
    })
    .unwrap();
    assert_eq!(games.len(), 10);
    games.sort_by_key(|game| (game.pair, game.swapped));
    for pair in games.chunks(2) {
        let (normal, swapped) = (&pair[0], &pair[1]);
        assert_eq!(normal.pair, swapped.pair);
        assert!(!normal.swapped && swapped.swapped);
        assert_eq!(normal.game.seed, swapped.game.seed);
        assert_eq!(normal.game.seed, pair_seed(3, normal.pair));
        assert_eq!(normal.game.seats, ["a".to_string(), "b".to_string()]);
        assert_eq!(swapped.game.seats, ["b".to_string(), "a".to_string()]);
        // Same seed, so the same seat wins both games: one win each.
        assert_eq!(normal.game.winner, swapped.game.winner);
    }
}

#[test]
fn stops_at_the_first_error() {
    let mut broken = tournament(50, 2);
    broken.bots[1] = BotSpec::parse("ghost=/definitely/not/here").unwrap();
    let mut played = 0;
    let error = run(&broken, &new_referee, |_| {
        played += 1;
        Flow::Continue
    })
    .unwrap_err();
    assert!(error.to_string().contains("ghost"), "{error}");
    assert!(played < 100);
}

#[test]
fn stop_ends_the_tournament_early() {
    let mut reported = 0;
    run(&tournament(500, 4), &new_referee, |_| {
        reported += 1;
        if reported == 3 {
            Flow::Stop
        } else {
            Flow::Continue
        }
    })
    .unwrap();
    assert_eq!(reported, 3, "no game is reported after Stop");
}

#[test]
fn passes_the_opening_to_the_referee() {
    let mut with_opening = tournament(2, 1);
    with_opening.opening_plies = 7;
    let seen = std::sync::Mutex::new(Vec::new());
    let factory = |setup: &GameSetup| -> Box<dyn Referee> {
        seen.lock().unwrap().push(*setup);
        new_referee(setup)
    };
    run(&with_opening, &factory, |_| Flow::Continue).unwrap();
    let seen = seen.into_inner().unwrap();
    assert_eq!(seen.len(), 4);
    assert!(seen.iter().all(|setup| setup.opening_plies == 7));
    assert_eq!(seen[0].seed, pair_seed(3, 0));
}

#[test]
fn pair_seeds_differ() {
    let seeds: std::collections::HashSet<u64> = (0..1000).map(|pair| pair_seed(1, pair)).collect();
    assert_eq!(seeds.len(), 1000);
}
