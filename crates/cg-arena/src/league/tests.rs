use super::*;

const LINE: &str = r#"{"a":"v019","b":"v018","pairs":100,"wins":61,"draws":98,"losses":41,"pair_points":[2,10,60,20,8],"faults":[0,1],"time_scale":1.0,"time_tolerance_ms":0,"opening_plies":4,"seed":1234567890123,"date":"2026-10-16","commit":"0123abc","cpu":"AMD EPYC 7763 64-Core Processor"}"#;

fn pairing(a: &str, b: &str, wins: u32, draws: u32, losses: u32) -> PairingResult {
    let pairs = (wins + draws + losses) / 2;
    PairingResult {
        a: a.to_string(),
        b: b.to_string(),
        pairs,
        wins,
        draws,
        losses,
        pair_points: [0, 0, pairs, 0, 0],
        faults: [0, 0],
        time_scale: 1.0,
        time_tolerance_ms: 0,
        opening_plies: 4,
        seed: 1,
        date: "2026-10-16".to_string(),
        commit: "unknown".to_string(),
        cpu: "unknown".to_string(),
    }
}

fn names(list: &[&str]) -> Vec<String> {
    list.iter().map(|name| name.to_string()).collect()
}

fn error_of(text: &str) -> String {
    parse_results(text, "results.jsonl").unwrap_err()
}

#[test]
fn a_line_round_trips_in_the_documented_key_order() {
    let parsed = parse_results(LINE, "f").unwrap();
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].a, "v019");
    assert_eq!(parsed[0].pair_points, [2, 10, 60, 20, 8]);
    assert_eq!(parsed[0].faults, [0, 1]);
    assert_eq!(parsed[0].seed, 1_234_567_890_123);
    assert_eq!(parsed[0].to_line(), LINE);
    assert_eq!(parse_results(&parsed[0].to_line(), "f").unwrap(), parsed);
}

#[test]
fn blank_lines_are_ignored_and_an_empty_file_is_fine() {
    assert!(parse_results("", "f").unwrap().is_empty());
    assert!(parse_results("\n  \n\n", "f").unwrap().is_empty());
    let text = format!("\n{LINE}\n   \n");
    assert_eq!(parse_results(&text, "f").unwrap().len(), 1);
}

#[test]
fn malformed_json_is_rejected_with_its_line_number() {
    let error = error_of(&format!("{LINE}\n{{not json\n"));
    assert!(error.starts_with("results.jsonl:2: "), "{error}");
}

#[test]
fn unknown_and_missing_keys_are_rejected() {
    let unknown = LINE.replace("\"cpu\"", "\"extra\":1,\"cpu\"");
    let error = error_of(&format!("\n{unknown}"));
    assert!(error.starts_with("results.jsonl:2: "), "{error}");
    assert!(error.contains("extra"), "{error}");
    let missing = LINE.replace("\"faults\":[0,1],", "");
    let error = error_of(&missing);
    assert!(error.starts_with("results.jsonl:1: "), "{error}");
    assert!(error.contains("faults"), "{error}");
}

#[test]
fn a_pairing_of_a_bot_with_itself_is_rejected() {
    let error = error_of(&LINE.replace("\"b\":\"v018\"", "\"b\":\"v019\""));
    assert!(error.starts_with("results.jsonl:1: "), "{error}");
    assert!(error.contains("itself"), "{error}");
}

#[test]
fn counts_that_do_not_add_up_are_rejected() {
    let games = LINE.replace("\"wins\":61", "\"wins\":62");
    let error = error_of(&format!("{LINE}\n{games}"));
    assert!(error.starts_with("results.jsonl:2: "), "{error}");
    assert!(error.contains("200 games"), "{error}");
    let points = LINE.replace("[2,10,60,20,8]", "[2,10,60,20,9]");
    let error = error_of(&points);
    assert!(error.starts_with("results.jsonl:1: "), "{error}");
    assert!(error.contains("pair_points"), "{error}");
}

#[test]
fn two_lines_for_the_same_pair_are_rejected_in_either_order() {
    let reversed = LINE.replace(
        "\"a\":\"v019\",\"b\":\"v018\"",
        "\"a\":\"v018\",\"b\":\"v019\"",
    );
    let error = error_of(&format!("{LINE}\n\n{reversed}\n"));
    assert!(error.starts_with("results.jsonl:3: "), "{error}");
    assert!(error.contains("line 1"), "{error}");
    let error = error_of(&format!("{LINE}\n{LINE}\n"));
    assert!(error.starts_with("results.jsonl:2: "), "{error}");
}

#[test]
fn a_missing_file_is_an_error() {
    let path = std::env::temp_dir().join(format!("cg-arena-missing-{}.jsonl", std::process::id()));
    let error = read_results(&path).unwrap_err();
    assert!(error.contains("cannot read"), "{error}");
}

#[test]
fn pairing_seed_is_deterministic_and_order_sensitive() {
    assert_eq!(
        pairing_seed(1, "v019", "v018"),
        pairing_seed(1, "v019", "v018")
    );
    assert_ne!(
        pairing_seed(1, "v019", "v018"),
        pairing_seed(1, "v018", "v019")
    );
    assert_ne!(
        pairing_seed(1, "v019", "v018"),
        pairing_seed(2, "v019", "v018")
    );
    // The 0 byte keeps "ab" + "c" apart from "a" + "bc".
    assert_ne!(pairing_seed(1, "ab", "c"), pairing_seed(1, "a", "bc"));
    // The canary plays other openings than the pairing, and other releases'.
    let canary = canary_seed(1, "v002", "v001", "v019");
    assert_eq!(canary, canary_seed(1, "v002", "v001", "v019"));
    assert_ne!(canary, pairing_seed(1, "v002", "v001"));
    assert_ne!(canary, canary_seed(1, "v002", "v001", "v020"));
}

#[test]
fn fnv1a_matches_the_published_vectors() {
    assert_eq!(fnv1a(b""), 0xcbf2_9ce4_8422_2325);
    assert_eq!(fnv1a(b"a"), 0xaf63_dc4c_8601_ec8c);
    assert_eq!(fnv1a(b"foobar"), 0x8594_4171_f739_67e8);
}

#[test]
fn dates_come_from_the_unix_time() {
    assert_eq!(date_from_unix(0), "1970-01-01");
    assert_eq!(date_from_unix(951_782_400), "2000-02-29");
    assert_eq!(date_from_unix(1_781_654_400), "2026-06-17");
    assert_eq!(date_from_unix(86_399), "1970-01-01");
    assert_eq!(date_from_unix(86_400), "1970-01-02");
    assert_eq!(today().len(), 10);
}

#[test]
fn the_cpu_is_the_first_model_name_line() {
    let text = "processor\t: 0\nvendor_id\t: X\nmodel name\t: AMD EPYC 7763 64-Core Processor \n\
                model name\t: other\n";
    assert_eq!(
        model_name(text).as_deref(),
        Some("AMD EPYC 7763 64-Core Processor")
    );
    assert_eq!(model_name("processor: 0\n"), None);
    assert!(!cpu_model().is_empty());
}

#[test]
fn ratings_follow_the_stored_pairings() {
    let pairings = [pairing("b", "a", 150, 0, 50), pairing("c", "b", 150, 0, 50)];
    let known = names(&["a", "b", "c", "d"]);
    let ratings = rate(&known, &pairings, "a").unwrap();
    assert_eq!(ratings[0].rating, 0.0);
    assert!(ratings[1].rating > 0.0 && ratings[2].rating > ratings[1].rating);
    assert!(ratings[2].is_linked());
    assert!(!ratings[3].is_linked(), "d played nobody");
    assert_eq!(ratings[1].games, 400);
    assert!(rate(&names(&["a"]), &pairings, "a").is_err());
    assert!(rate(&known, &pairings, "z").is_err());
}

#[test]
fn a_small_pool_is_played_in_full_previous_first() {
    let candidates = names(&["v001", "v002", "greedy", "random"]);
    let fixed = names(&["greedy", "v001"]);
    // 4 candidates <= 1 + 2 + 5.
    let all = all_opponents(&candidates, Some("v002"), &fixed, 5).unwrap();
    assert_eq!(all, ["v002", "greedy", "random", "v001"]);
    // Without a previous release the others follow in name order.
    let all = all_opponents(&candidates, None, &[], 4).unwrap();
    assert_eq!(all, ["greedy", "random", "v001", "v002"]);
    // One more than previous + fixed + nearest needs a choice.
    assert!(all_opponents(&candidates, Some("v002"), &fixed, 0).is_none());
    assert!(all_opponents(&candidates, None, &[], 3).is_none());
    // A previous release that is also fixed counts once.
    let fixed = names(&["v002", "greedy"]);
    assert!(all_opponents(&candidates, Some("v002"), &fixed, 2).is_some());
    assert!(all_opponents(&candidates, Some("v002"), &fixed, 1).is_none());
    assert_eq!(all_opponents(&[], None, &[], 0), Some(Vec::new()));
}

fn standing(name: &str, rating: f64, margin: f64) -> Rating {
    Rating {
        name: name.to_string(),
        rating,
        margin,
        games: 100,
    }
}

fn standings() -> Vec<Rating> {
    vec![
        standing("random", 0.0, 0.0),
        standing("greedy", 100.0, 20.0),
        standing("v001", 300.0, 20.0),
        standing("v002", 380.0, 20.0),
        standing("v003", 410.0, 20.0),
        standing("v004", 440.0, 20.0),
        standing("v005", 460.0, 20.0),
        standing("v006", 520.0, 20.0),
        standing("v007", 600.0, 20.0),
    ]
}

fn candidates() -> Vec<String> {
    standings().into_iter().map(|rating| rating.name).collect()
}

#[test]
fn the_nearest_bots_follow_the_previous_and_fixed_opponents() {
    let chosen = choose_opponents(
        &candidates(),
        Some("v007"),
        &names(&["greedy", "v001"]),
        2,
        &standings(),
        450.0,
    );
    // v004 (10 away) and v005 (10 away) are nearest to 450, tie by name.
    assert_eq!(chosen, ["v007", "greedy", "v001", "v004", "v005"]);
}

#[test]
fn a_fixed_opponent_among_the_nearest_gives_its_place_to_the_next() {
    // Nearest to 310 would be v001 (10), v002 (70), greedy (210), v003 (100).
    let chosen = choose_opponents(
        &candidates(),
        Some("v007"),
        &names(&["v001"]),
        2,
        &standings(),
        310.0,
    );
    assert_eq!(chosen, ["v007", "v001", "v002", "v003"]);
}

#[test]
fn a_previous_release_that_is_fixed_counts_once() {
    let chosen = choose_opponents(
        &candidates(),
        Some("v007"),
        &names(&["v007", "greedy"]),
        2,
        &standings(),
        450.0,
    );
    assert_eq!(chosen, ["v007", "greedy", "v004", "v005"]);
}

#[test]
fn equal_distances_are_broken_by_name_and_zero_nearest_adds_none() {
    let chosen = choose_opponents(
        &candidates(),
        None,
        &[],
        3,
        &standings(),
        // v002 (380) and v004 (440) are both 30 from 410.
        410.0,
    );
    assert_eq!(chosen, ["v003", "v002", "v004"]);
    let chosen = choose_opponents(
        &candidates(),
        Some("v007"),
        &names(&["greedy"]),
        0,
        &standings(),
        450.0,
    );
    assert_eq!(chosen, ["v007", "greedy"]);
}

#[test]
fn bots_not_linked_to_the_anchor_are_never_chosen() {
    let mut ratings = standings();
    ratings[4] = standing("v003", 410.0, f64::INFINITY);
    let chosen = choose_opponents(&candidates(), Some("v007"), &[], 1, &ratings, 410.0);
    assert_eq!(
        chosen,
        ["v007", "v002"],
        "v003 is unlinked and v004 farther"
    );
    // A candidate without any rating is excluded as well.
    let chosen = choose_opponents(&candidates(), None, &[], 2, &ratings[..3], 0.0);
    assert_eq!(chosen, ["random", "greedy"]);
}

#[test]
fn the_canary_interval_comes_from_the_pairs_of_both_runs() {
    let mut stored = pairing("a", "b", 100, 0, 0);
    // 100 pairs: 0, 1/4, 1/2, 3/4, 1 of a pair's score, in these numbers.
    stored.pairs = 100;
    stored.pair_points = [2, 10, 60, 20, 8];
    stored.wins = 61;
    stored.draws = 98;
    stored.losses = 41;
    // Stored mean (0*2 + .25*10 + .5*60 + .75*20 + 8) / 100 = 0.555 and
    // variance of the pair score 0.040725, so of the mean 0.00040725.
    let now = [0, 0, 10, 0, 0];
    let report = compare_canary(&stored, &now).unwrap();
    assert!((report.stored - 55.5).abs() < 1e-9);
    assert!((report.now - 50.0).abs() < 1e-9);
    assert!((report.difference + 5.5).abs() < 1e-9);
    assert_eq!(report.games_now, 20);
    // The replay's pairs all scored one point: no variance of its own.
    let expected = 100.0 * 1.96 * 0.000_407_25_f64.sqrt();
    assert!((report.interval - expected).abs() < 1e-9, "{report:?}");
    assert!((report.interval - 3.955).abs() < 1e-3);
    assert!(!report.within());
    assert_eq!(
        report.line("a", "b"),
        "canary a vs b: 50.0% of 20 games now, 55.5% stored, difference -5.5 points, \
         95% interval ±4.0: OUTSIDE"
    );
    let same = compare_canary(&stored, &stored.pair_points).unwrap();
    assert!(same.within());
    assert_eq!(same.difference, 0.0);
    assert!(same.line("a", "b").ends_with(": within"));
    // Two variances add: 2 * 0.00040725, sqrt * 1.96 = 5.594 points.
    assert!((same.interval - 100.0 * 1.96 * (2.0 * 0.000_407_25_f64).sqrt()).abs() < 1e-9);
    assert!(compare_canary(&stored, &[0; 5]).is_none());
}

#[test]
fn known_bots_are_sorted_and_without_repeats() {
    let pairings = [pairing("b", "a", 2, 0, 0), pairing("c", "a", 2, 0, 0)];
    assert_eq!(
        known_bots(&pairings, ["d", "a"]),
        names(&["a", "b", "c", "d"])
    );
}
