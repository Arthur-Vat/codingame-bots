use std::collections::HashSet;

use super::*;
use crate::runner::{EndReason, MatchRecord};
use crate::sprt::MIN_PAIRS;
use crate::tournament::{pair_seed, shard_seed};

const CANDIDATE: &str = "cand";
const BASELINE: &str = "base";

fn settings() -> SprtSettings {
    SprtSettings {
        elo0: 0.0,
        elo1: 10.0,
        alpha: 0.05,
        beta: 0.05,
    }
}

fn test(round_blocks: usize, max_pairs: u32) -> BlockTest {
    BlockTest {
        settings: settings(),
        round_blocks,
        max_pairs,
        names: [CANDIDATE.to_string(), BASELINE.to_string()],
        faults: None,
    }
}

/// One game: the candidate gets `points` (0, 0.5 or 1); `end` says how it ended.
fn game(pair: u32, swapped: bool, points: f64, end: EndReason) -> GameRecord {
    let candidate_seat = usize::from(swapped);
    let winner = if points == 1.0 {
        Some(candidate_seat)
    } else if points == 0.0 {
        Some(1 - candidate_seat)
    } else {
        None
    };
    let mut seats = [CANDIDATE.to_string(), BASELINE.to_string()];
    if swapped {
        seats.reverse();
    }
    GameRecord {
        pair,
        swapped,
        game: MatchRecord {
            seed: 1,
            seats,
            winner,
            end,
            turns: 1,
            max_answer_ms: [1.0; 2],
            mean_answer_ms: [1.0; 2],
            later_answer_ms: Default::default(),
            recorded_turns: Vec::new(),
        },
    }
}

/// A block whose pairs give the candidate these points (0, 0.5, 1, 1.5, 2).
fn block(pairs: &[f64]) -> Vec<GameRecord> {
    let mut games = Vec::new();
    for (pair, points) in pairs.iter().enumerate() {
        let (first, second) = if *points == 2.0 {
            (1.0, 1.0)
        } else if *points == 1.5 {
            (1.0, 0.5)
        } else if *points == 1.0 {
            (0.5, 0.5)
        } else if *points == 0.5 {
            (0.5, 0.0)
        } else {
            (0.0, 0.0)
        };
        games.push(game(pair as u32, false, first, EndReason::Finished));
        games.push(game(pair as u32, true, second, EndReason::Finished));
    }
    games
}

/// A block of `copies` times 25 pairs with even results: [2, 3, 15, 3, 2].
fn even_block(copies: usize) -> Vec<GameRecord> {
    let mut pairs = Vec::new();
    for _ in 0..copies {
        pairs.extend([0.0; 2]);
        pairs.extend([0.5; 3]);
        pairs.extend([1.0; 15]);
        pairs.extend([1.5; 3]);
        pairs.extend([2.0; 2]);
    }
    block(&pairs)
}

fn blocks_of(pairs: f64, count: usize, size: usize) -> Vec<Vec<GameRecord>> {
    (0..count).map(|_| block(&vec![pairs; size])).collect()
}

#[test]
fn counts_a_round_of_blocks_in_order_and_accepts_a_sweep() {
    // 15 pairs a block, two blocks a round: the first round has MIN_PAIRS.
    assert_eq!(MIN_PAIRS, 30);
    let checked = test(2, 8000).check(&blocks_of(2.0, 6, 15)).unwrap();
    assert_eq!(checked.outcome, Outcome::Accepted);
    assert_eq!(checked.counted_pairs(), 30);
    assert_eq!(checked.counted_blocks, 2);
    assert_eq!(checked.left_out_blocks, 4);
    assert_eq!(checked.summary.pairs, [0, 0, 0, 0, 30]);
    assert!(checked.llr > 2.9);
}

#[test]
fn rejects_a_clearly_weaker_candidate() {
    let checked = test(2, 8000).check(&blocks_of(0.0, 4, 15)).unwrap();
    assert_eq!(checked.outcome, Outcome::Rejected);
    assert_eq!(checked.counted_pairs(), 30);
}

#[test]
fn waits_for_the_minimum_pairs() {
    let checked = test(2, 8000).check(&blocks_of(2.0, 2, 10)).unwrap();
    assert_eq!(checked.outcome, Outcome::Continue);
    assert_eq!(checked.counted_pairs(), 20);
}

#[test]
fn a_round_that_is_not_whole_is_left_out() {
    // Three blocks of a round of two: the third waits for its round.
    let checked = test(2, 8000).check(&blocks_of(2.0, 3, 10)).unwrap();
    assert_eq!(checked.outcome, Outcome::Continue);
    assert_eq!(checked.counted_blocks, 2);
    assert_eq!(checked.left_out_blocks, 1);
    assert_eq!(checked.counted_pairs(), 20);
    let none = test(2, 8000).check(&blocks_of(2.0, 1, 10)).unwrap();
    assert_eq!((none.outcome, none.counted_pairs()), (Outcome::Continue, 0));
}

#[test]
fn stops_at_the_cap_when_undecided() {
    let blocks: Vec<_> = (0..8).map(|_| even_block(1)).collect();
    let capped = test(2, 100).check(&blocks).unwrap();
    assert_eq!(capped.outcome, Outcome::Capped);
    // The cap is checked after a whole round: 4 rounds of 50 pairs make 200,
    // but the second round already reaches 100.
    assert_eq!(capped.counted_pairs(), 100);
    assert_eq!(capped.counted_blocks, 4);
    assert_eq!(capped.left_out_blocks, 4);
    let open = test(2, 1000).check(&blocks).unwrap();
    assert_eq!(open.outcome, Outcome::Continue);
    assert_eq!(open.counted_pairs(), 200);
}

#[test]
fn the_deciding_round_does_not_depend_on_how_many_blocks_come_with_it() {
    let all = blocks_of(2.0, 8, 10);
    let at_once = test(2, 8000).check(&all).unwrap();
    for count in (2..=8).step_by(2) {
        let partial = test(2, 8000).check(&all[..count]).unwrap();
        if partial.outcome == Outcome::Accepted {
            assert_eq!(partial.counted_pairs(), at_once.counted_pairs());
            assert_eq!(partial.summary.pairs, at_once.summary.pairs);
        }
    }
    // 2 blocks of 10 pairs make 20, 4 make 40: accepted after the second round.
    assert_eq!(at_once.counted_blocks, 4);
    assert_eq!(at_once.counted_pairs(), 40);
}

#[test]
fn the_order_of_games_in_a_file_does_not_matter() {
    let mut shuffled = block(&[2.0; 30]);
    shuffled.reverse();
    let checked = test(1, 8000).check(&[shuffled]).unwrap();
    assert_eq!(checked.outcome, Outcome::Accepted);
    assert_eq!(checked.summary.pairs, [0, 0, 0, 0, 30]);
}

#[test]
fn a_candidate_fault_stops_the_test_at_once() {
    let mut checked_test = test(1, 8000);
    checked_test.faults = Some(FaultCheck {
        bot: 0,
        max_timeout_rate: 0.01,
    });
    // One crash of the candidate in the first block of 100 games.
    let mut crashed = block(&[1.0; 50]);
    crashed[0] = game(
        0,
        false,
        0.0,
        EndReason::Crash {
            seat: 0,
            detail: "gone".to_string(),
        },
    );
    let result = checked_test.check(&[crashed, block(&[2.0; 50])]).unwrap();
    assert_eq!(result.outcome, Outcome::Fault);
    assert_eq!(result.counted_blocks, 1);
    assert_eq!(result.summary.faults[0].crashes, 1);
}

#[test]
fn timeouts_are_tolerated_up_to_the_rate_of_the_games_counted() {
    let mut checked_test = test(1, 8000);
    checked_test.faults = Some(FaultCheck {
        bot: 0,
        max_timeout_rate: 0.01,
    });
    let timeout = |pair| {
        game(
            pair,
            false,
            0.0,
            EndReason::Timeout {
                seat: 0,
                limit_ms: 50.0,
            },
        )
    };
    // 100 games allow 1 timeout; 2 are too many.
    let mut one = even_block(2);
    one[0] = timeout(0);
    assert_eq!(
        checked_test.check(&[one.clone()]).unwrap().outcome,
        Outcome::Continue
    );
    let mut two = one;
    two[2] = timeout(1);
    assert_eq!(
        checked_test.check(&[two.clone()]).unwrap().outcome,
        Outcome::Fault
    );
    // The rate is checked after each round, so more games later do not
    // excuse the first round.
    let more = vec![two, even_block(2)];
    let late = checked_test.check(&more).unwrap();
    assert_eq!((late.outcome, late.counted_blocks), (Outcome::Fault, 1));
    // One timeout in each of two blocks stays within 1% of the games.
    let mut spread = even_block(2);
    spread[0] = timeout(0);
    let mut later = even_block(2);
    later[2] = timeout(1);
    assert_eq!(
        checked_test.check(&[spread, later]).unwrap().outcome,
        Outcome::Continue
    );
}

#[test]
fn the_baseline_may_fault_when_only_the_candidate_is_checked() {
    let mut checked_test = test(1, 8000);
    checked_test.faults = Some(FaultCheck {
        bot: 0,
        max_timeout_rate: 0.0,
    });
    let mut games = block(&[1.0; 20]);
    games[0] = game(
        0,
        false,
        1.0,
        EndReason::Crash {
            seat: 1,
            detail: "gone".to_string(),
        },
    );
    let result = checked_test.check(&[games]).unwrap();
    assert_eq!(result.outcome, Outcome::Continue);
    assert_eq!(result.summary.faults[1].crashes, 1);
    checked_test.faults = Some(FaultCheck {
        bot: 1,
        max_timeout_rate: 0.0,
    });
    let mut games = block(&[1.0; 20]);
    games[0] = game(
        0,
        false,
        1.0,
        EndReason::Crash {
            seat: 1,
            detail: "gone".to_string(),
        },
    );
    assert_eq!(
        checked_test.check(&[games]).unwrap().outcome,
        Outcome::Fault
    );
}

#[test]
fn refuses_blocks_that_are_not_complete_pairs_of_the_two_bots() {
    let mut half = block(&[1.0; 5]);
    half.pop();
    let err = test(1, 100).check(&[half]).unwrap_err();
    assert!(
        err.contains("block 1") && err.contains("not complete"),
        "{err}"
    );

    let mut twice = block(&[1.0; 5]);
    twice[1] = twice[0].clone();
    let err = test(1, 100).check(&[twice]).unwrap_err();
    assert!(err.contains("same bot in seat 0"), "{err}");

    let mut other = block(&[1.0; 5]);
    other[0].game.seats[1] = "third".to_string();
    let err = test(1, 100).check(&[block(&[1.0; 5]), other]).unwrap_err();
    assert!(err.contains("block 2") && err.contains("third"), "{err}");

    assert!(test(0, 100).check(&[]).is_err());
}

#[test]
fn reads_the_games_a_match_wrote() {
    let games = block(&[2.0, 0.5, 1.0]);
    let dir = std::env::temp_dir().join(format!("cg-arena-blocks-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("block.jsonl");
    let mut text = String::new();
    for record in &games {
        text.push_str(&serde_json::to_string(record).unwrap());
        text.push_str("\n\n");
    }
    std::fs::write(&path, text).unwrap();
    let read = read_block(&path).unwrap();
    assert_eq!(read, games);
    std::fs::write(&path, "{\"pair\": 0}\n").unwrap();
    let err = read_block(&path).unwrap_err();
    assert!(err.contains("line 1"), "{err}");
    assert!(read_block(&dir.join("missing.jsonl")).is_err());
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn every_shard_has_its_own_seed_and_so_its_own_games() {
    let mut seeds = HashSet::new();
    let mut games = HashSet::new();
    for shard in 0..8 {
        assert!(seeds.insert(shard_seed(1, shard)), "{shard}");
        for pair in 0..2_000 {
            let seed = pair_seed(shard_seed(1, shard), pair);
            assert!(games.insert(seed), "shard {shard} pair {pair}");
        }
    }
    assert_eq!(shard_seed(1, 3), shard_seed(1, 3));
    assert_ne!(shard_seed(1, 3), shard_seed(2, 3));
    assert_ne!(shard_seed(1, 3), shard_seed(1, 4));
}
