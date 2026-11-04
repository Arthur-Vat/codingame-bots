use super::*;
use crate::runner::{EndReason, MatchRecord};

fn settings() -> SprtSettings {
    SprtSettings {
        elo0: 0.0,
        elo1: 10.0,
        alpha: 0.05,
        beta: 0.05,
    }
}

fn game(pair: u32, swapped: bool, candidate_points: f64) -> GameRecord {
    let first_seat = usize::from(swapped);
    let winner = if candidate_points == 1.0 {
        Some(first_seat)
    } else if candidate_points == 0.0 {
        Some(1 - first_seat)
    } else {
        None
    };
    GameRecord {
        pair,
        swapped,
        game: MatchRecord {
            seed: 1,
            seats: ["a".to_string(), "b".to_string()],
            winner,
            end: EndReason::Finished,
            turns: 1,
            max_answer_ms: [0.0; 2],
            mean_answer_ms: [0.0; 2],
            later_answer_ms: Default::default(),
            recorded_turns: Vec::new(),
        },
    }
}

#[test]
fn bounds_follow_the_error_rates() {
    let (lower, upper) = settings().bounds();
    assert!((lower - (-2.944_438_979)).abs() < 1e-6, "{lower}");
    assert!((upper - 2.944_438_979).abs() < 1e-6, "{upper}");
}

#[test]
fn score_and_elo_match() {
    assert_eq!(score_from_elo(0.0), 0.5);
    assert!((score_from_elo(400.0) - 10.0 / 11.0).abs() < 1e-12);
    assert!((score_from_elo(-400.0) - 1.0 / 11.0).abs() < 1e-12);
}

#[test]
fn llr_matches_a_hand_computation() {
    let pairs = [2, 5, 10, 30, 3];
    let n = 50.0;
    let mean: f64 = (5.0 * 0.25 + 10.0 * 0.5 + 30.0 * 0.75 + 3.0) / n;
    let variance = (2.0 * mean.powi(2)
        + 5.0 * (0.25 - mean).powi(2)
        + 10.0 * (0.5 - mean).powi(2)
        + 30.0 * (0.75 - mean).powi(2)
        + 3.0 * (1.0 - mean).powi(2))
        / n;
    let (s0, s1) = (0.5, score_from_elo(10.0));
    let expected = n * (s1 - s0) * (2.0 * mean - s0 - s1) / (2.0 * variance);
    let llr = settings().llr(&pairs);
    assert!((llr - expected).abs() < 1e-9, "{llr} vs {expected}");
}

#[test]
fn empty_cells_barely_change_the_llr() {
    let llr = settings().llr(&[0, 0, 10, 30, 0]);
    let nearly = settings().llr(&[1, 0, 1000, 3000, 0]) / 100.0;
    assert!((llr - nearly).abs() / nearly < 0.01, "{llr} vs {nearly}");
}

#[test]
fn even_results_drift_towards_h0_and_strong_ones_towards_h1() {
    let even = [10, 10, 60, 10, 10];
    assert!(settings().llr(&even) < 0.0);
    let strong = [0, 2, 30, 40, 28];
    assert!(strong.iter().sum::<u32>() >= MIN_PAIRS);
    assert_eq!(settings().verdict(&strong), Verdict::Accepted);
    let weak = [28, 40, 30, 2, 0];
    assert_eq!(settings().verdict(&weak), Verdict::Rejected);
    assert_eq!(settings().llr(&[0; 5]), 0.0);
}

#[test]
fn a_sweep_has_a_finite_llr() {
    let llr = settings().llr(&[0, 0, 0, 0, 3]);
    assert!(llr.is_finite() && llr > 0.0);
}

#[test]
fn never_decides_before_the_minimum_pairs() {
    let mut sweep = [0; 5];
    sweep[4] = MIN_PAIRS - 1;
    assert_eq!(settings().verdict(&sweep), Verdict::Continue);
    sweep[4] = MIN_PAIRS;
    assert_eq!(settings().verdict(&sweep), Verdict::Accepted);
}

#[test]
fn counts_pairs_in_order_whatever_order_games_finish_in() {
    let mut test = SequentialTest::new(settings());
    // Pair 1 completes before pair 0: nothing is counted yet.
    test.add(&game(1, false, 1.0));
    test.add(&game(1, true, 1.0));
    assert_eq!(test.counted_pairs(), 0);
    test.add(&game(0, true, 0.5));
    assert_eq!(test.counted_pairs(), 0);
    test.add(&game(0, false, 0.0));
    assert_eq!(test.counted_pairs(), 2);
    // Pair 0: 0.5 points; pair 1: 2 points.
    assert_eq!(test.pairs, [0, 1, 0, 0, 1]);
}

#[test]
fn stops_at_the_first_decisive_pair() {
    let mut test = SequentialTest::new(settings());
    let mut pair = 0;
    while test.verdict() == Verdict::Continue {
        test.add(&game(pair, false, 1.0));
        test.add(&game(pair, true, 1.0));
        pair += 1;
        assert!(pair < 1000);
    }
    assert_eq!(test.verdict(), Verdict::Accepted);
    assert_eq!(test.counted_pairs(), pair);
    assert_eq!(pair, MIN_PAIRS);
    // Later games do not change the verdict.
    test.add(&game(pair, false, 0.0));
    assert_eq!(test.verdict(), Verdict::Accepted);
}

#[test]
fn rejects_invalid_settings() {
    let mut bad = settings();
    bad.alpha = 0.0;
    assert!(bad.validate().is_err());
    let mut bad = settings();
    bad.elo1 = -5.0;
    assert!(bad.validate().is_err());
    assert!(settings().validate().is_ok());
}

/// Pair results of two bots `elo` apart, drawing 10% of games, with games
/// independent: a pentanomial distribution, as cumulative probabilities.
fn pair_distribution(elo: f64) -> [f64; 5] {
    let draw = 0.1;
    let win = score_from_elo(elo) - draw / 2.0;
    let game = [1.0 - win - draw, draw, win];
    let mut pair = [0.0; 5];
    for (first, p) in game.iter().enumerate() {
        for (second, q) in game.iter().enumerate() {
            pair[first + second] += p * q;
        }
    }
    let mut cumulative = [0.0; 5];
    let mut sum = 0.0;
    for (k, p) in pair.iter().enumerate() {
        sum += p;
        cumulative[k] = sum;
    }
    cumulative
}

/// The share of simulated tests accepting the candidate, for a candidate
/// `elo` stronger.
fn acceptance_rate(elo: f64, tests: u32, rng: &mut cg_core::rng::Rng) -> f64 {
    let cumulative = pair_distribution(elo);
    let mut accepted = 0;
    for _ in 0..tests {
        let mut pairs = [0u32; 5];
        let mut verdict = Verdict::Continue;
        for _ in 0..20_000 {
            let draw = rng.unit();
            let k = cumulative.iter().position(|&c| draw < c).unwrap_or(4);
            pairs[k] += 1;
            verdict = settings().verdict(&pairs);
            if verdict != Verdict::Continue {
                break;
            }
        }
        if verdict == Verdict::Accepted {
            accepted += 1;
        }
    }
    f64::from(accepted) / f64::from(tests)
}

#[test]
fn simulated_error_rates_stay_near_alpha_and_beta() {
    // 1000 simulated tests per hypothesis: the standard error of a 5% rate
    // is 0.7%, so 8% is a clear failure and not noise.
    let mut rng = cg_core::rng::Rng::new(2026);
    let false_positives = acceptance_rate(0.0, 1000, &mut rng);
    assert!(false_positives < 0.08, "accepted {false_positives} at elo0");
    let false_negatives = 1.0 - acceptance_rate(10.0, 1000, &mut rng);
    assert!(false_negatives < 0.08, "rejected {false_negatives} at elo1");
}

/// How many tests ended each way under one rule for when to look at the
/// evidence, and the pairs they played.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Outcomes {
    tests: u32,
    accepted: u32,
    rejected: u32,
    undecided: u32,
    pairs: u64,
}

impl Outcomes {
    fn share(&self, count: u32) -> f64 {
        f64::from(count) / f64::from(self.tests)
    }

    fn mean_pairs(&self) -> f64 {
        self.pairs as f64 / f64::from(self.tests)
    }
}

/// Simulates `tests` SPRTs on one candidate `elo` stronger. Every test draws
/// one stream of pair results and gives it to each rule in `look_every`:
/// the rule looks at the bounds after every `look_every[i]` pairs (1 is the
/// arena's own test, checking after every pair; `8 * B` is a test over 8
/// runners playing blocks of B pairs), and stops at `cap` pairs. Rules share
/// the streams, so their pairs played compare closely.
fn simulate_rules(
    elo: f64,
    tests: u32,
    look_every: &[u32],
    cap: u32,
    rng: &mut cg_core::rng::Rng,
) -> Vec<Outcomes> {
    let cumulative = pair_distribution(elo);
    let mut outcomes = vec![Outcomes::default(); look_every.len()];
    for _ in 0..tests {
        let mut pairs = [0u32; 5];
        let mut active = look_every.len();
        let mut decided = vec![false; look_every.len()];
        for played in 1..=cap {
            let draw = rng.unit();
            pairs[cumulative.iter().position(|&c| draw < c).unwrap_or(4)] += 1;
            for (rule, &every) in look_every.iter().enumerate() {
                if decided[rule] || played % every != 0 {
                    continue;
                }
                let verdict = settings().verdict(&pairs);
                if verdict != Verdict::Continue || played == cap {
                    decided[rule] = true;
                    active -= 1;
                    let found = &mut outcomes[rule];
                    found.pairs += u64::from(played);
                    match verdict {
                        Verdict::Accepted => found.accepted += 1,
                        Verdict::Rejected => found.rejected += 1,
                        Verdict::Continue => found.undecided += 1,
                    }
                }
            }
            if active == 0 {
                break;
            }
        }
        for found in &mut outcomes {
            found.tests += 1;
        }
    }
    outcomes
}

/// The block sizes compared: pairs a runner plays between two looks, with 8
/// runners (ADR 0049, decision 1).
const BLOCK_SIZES: [u32; 3] = [10, 25, 50];
const RUNNERS: u32 = 8;
const CAP: u32 = 8_000;

/// Runs the comparison of the arena's SPRT, looking after every pair, with
/// the same test looking once per round of 8 blocks of each size, at true
/// Elo 0, 5 and 10; returns the outcomes by Elo, the arena's own rule first.
fn compare_block_sizes(tests: u32, seed: u64) -> Vec<(f64, Vec<Outcomes>)> {
    let mut looks = vec![1];
    looks.extend(BLOCK_SIZES.map(|block| RUNNERS * block));
    let mut rng = cg_core::rng::Rng::new(seed);
    [0.0, 5.0, 10.0]
        .into_iter()
        .map(|elo| (elo, simulate_rules(elo, tests, &looks, CAP, &mut rng)))
        .collect()
}

fn print_comparison(results: &[(f64, Vec<Outcomes>)]) {
    println!("rule              elo  accepted rejected undecided  mean pairs");
    for (elo, rules) in results {
        for (rule, found) in rules.iter().enumerate() {
            let name = match rule {
                0 => "every pair".to_string(),
                n => format!("blocks of {}", BLOCK_SIZES[n - 1]),
            };
            println!(
                "{name:<16} {elo:>4}  {:>7.2}% {:>7.2}% {:>8.2}%  {:>10.0} ({:+.1}%)",
                100.0 * found.share(found.accepted),
                100.0 * found.share(found.rejected),
                100.0 * found.share(found.undecided),
                found.mean_pairs(),
                100.0 * (found.mean_pairs() / rules[0].mean_pairs() - 1.0)
            );
        }
    }
}

#[test]
fn checking_between_blocks_keeps_the_error_rates_and_the_pairs() {
    // 400 tests at each of three strengths: the standard error of a 5% rate
    // is 1.1%, so the bounds below tell a real loss from noise. The pairs are
    // compared test by test (shared streams), which is much tighter.
    let results = compare_block_sizes(400, 2026);
    print_comparison(&results);
    for (elo, rules) in &results {
        for (rule, found) in rules.iter().enumerate() {
            if *elo == 0.0 {
                assert!(
                    found.share(found.accepted) < 0.09,
                    "false accepts, rule {rule}"
                );
            }
            if *elo == 10.0 {
                assert!(
                    found.share(found.rejected) < 0.09,
                    "false rejects, rule {rule}"
                );
            }
            if rule > 0 {
                let extra = found.mean_pairs() / rules[0].mean_pairs() - 1.0;
                assert!(
                    (0.0..0.25).contains(&extra),
                    "blocks of {} play {extra:+.3} more pairs at elo {elo}",
                    BLOCK_SIZES[rule - 1]
                );
            }
        }
    }
}

/// The full comparison that chose the block size in `evaluation.env`:
/// `cargo test --release -p cg-arena -- --ignored --nocapture choose_the_block_size`.
#[test]
#[ignore = "takes a minute in release mode; its numbers are in games/ice-and-fire/evaluation.env"]
fn choose_the_block_size() {
    let results = compare_block_sizes(40_000, 49);
    print_comparison(&results);
}
