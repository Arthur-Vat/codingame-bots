use std::time::Duration;

use super::*;
use crate::referee::{Answer, InvalidAnswer, Outcome, TimeLimits};

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
        jobs,
        options: MatchOptions::default(),
    }
}

fn new_referee(seed: u64) -> Box<dyn Referee> {
    Box::new(SeedDecides {
        seed,
        played: false,
    })
}

#[test]
fn plays_every_pair_twice_with_seats_swapped() {
    let mut games = Vec::new();
    run(&tournament(5, 3), &new_referee, |game| {
        games.push(game.clone())
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
    let error = run(&broken, &new_referee, |_| played += 1).unwrap_err();
    assert!(error.to_string().contains("ghost"), "{error}");
    assert!(played < 100);
}

#[test]
fn pair_seeds_differ() {
    let seeds: std::collections::HashSet<u64> = (0..1000).map(|pair| pair_seed(1, pair)).collect();
    assert_eq!(seeds.len(), 1000);
}
