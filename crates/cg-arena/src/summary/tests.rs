use super::*;
use crate::runner::MatchRecord;

fn game(pair: u32, swapped: bool, winner: Option<usize>, end: EndReason) -> GameRecord {
    let names = if swapped { ["b", "a"] } else { ["a", "b"] };
    GameRecord {
        pair,
        swapped,
        game: MatchRecord {
            seed: 1,
            seats: names.map(str::to_string),
            winner,
            end,
            turns: 10,
            max_answer_ms: [1.0, 2.0],
            mean_answer_ms: [0.5, 0.5],
        },
    }
}

fn summary() -> Summary {
    Summary::new(["a".to_string(), "b".to_string()])
}

#[test]
fn counts_results_from_the_first_bots_point_of_view() {
    let mut s = summary();
    s.add(&game(0, false, Some(0), EndReason::Finished)); // a wins in seat 0
    s.add(&game(0, true, Some(0), EndReason::Finished)); // b wins in seat 0
    s.add(&game(1, false, None, EndReason::Finished));
    s.add(&game(1, true, Some(1), EndReason::Finished)); // a wins in seat 1
    assert_eq!((s.wins, s.draws, s.losses), (2, 1, 1));
    assert_eq!(s.score(), Some(0.625));
    // Pair 0: 1 + 0 = 1 point; pair 1: 0.5 + 1 = 1.5 points.
    assert_eq!(s.pairs, [0, 0, 1, 1, 0]);
    // Seat 0 was a in game 0 and b in game 1 of each pair.
    assert_eq!(s.max_answer_ms, [2.0, 2.0]);
}

#[test]
fn attributes_faults_to_the_bot_in_the_faulty_seat() {
    let mut s = summary();
    let timeout = EndReason::Timeout {
        seat: 0,
        limit_ms: 100.0,
    };
    s.add(&game(0, true, Some(1), timeout)); // b sat in seat 0
    let crash = EndReason::Crash {
        seat: 0,
        detail: String::new(),
    };
    s.add(&game(1, false, Some(1), crash)); // a sat in seat 0
    let aborted = EndReason::Aborted {
        reason: String::new(),
    };
    s.add(&game(2, false, None, aborted));
    assert_eq!(s.faults[1].timeouts, 1);
    assert_eq!(s.faults[0].crashes, 1);
    assert_eq!(s.total_faults(), 2);
    assert_eq!(s.aborted, 1);
}

#[test]
fn estimates_elo_from_pairs() {
    let mut s = summary();
    assert_eq!(s.elo(), None);
    // 30 pairs where a scores 1.5 of 2 and 10 pairs where it scores 1.
    s.pairs = [0, 0, 10, 30, 0];
    let estimate = s.elo().unwrap();
    // Mean pair score 0.6875 -> about +137 Elo.
    assert!((estimate.elo - elo_from_score(0.6875)).abs() < 1e-9);
    assert!((estimate.elo - 137.0).abs() < 1.0, "{estimate:?}");
    assert!(estimate.low < estimate.elo && estimate.elo < estimate.high);
    // A sweep has no finite estimate.
    s.pairs = [0, 0, 0, 0, 5];
    assert_eq!(s.elo(), None);
}

#[test]
fn elo_of_even_score_is_zero() {
    assert!(elo_from_score(0.5).abs() < 1e-12);
    assert!(elo_from_score(0.75) > 190.0 && elo_from_score(0.75) < 191.0);
}

#[test]
fn displays_a_readable_report() {
    let mut s = summary();
    s.add(&game(0, false, Some(0), EndReason::Finished));
    let text = s.to_string();
    assert!(text.contains("a vs b: 1 games"), "{text}");
    assert!(
        text.contains("1 wins, 0 draws, 0 losses (score 100.0%)"),
        "{text}"
    );
}
