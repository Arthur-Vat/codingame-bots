use super::*;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use uttt_engine::{Board, Move, MoveList};

fn lines(request: &str) -> Vec<String> {
    lines_stopping(request, |_| true)
}

/// The lines of the answer; `keep` says after each whether to go on.
fn lines_stopping(request: &str, mut keep: impl FnMut(usize) -> bool) -> Vec<String> {
    let mut out = Vec::new();
    analyse(request, &mut |line| {
        assert!(!line.contains('\n'), "a line holds no newline");
        out.push(line.to_string());
        keep(out.len())
    });
    out
}

fn json(line: &str) -> Value {
    serde_json::from_str(line).expect("a line is JSON")
}

fn report(line: &str) -> Value {
    json(line)["report"].clone()
}

fn error_of(request: &str) -> String {
    let out = lines(request);
    assert_eq!(out.len(), 1, "one line for {request}: {out:?}");
    let value = json(&out[0]);
    assert_eq!(value.as_object().expect("an object").len(), 1);
    value["error"]
        .as_str()
        .expect("an error message")
        .to_string()
}

/// A game played by picking, at each turn, a legal move by a small
/// deterministic generator: the moves as the protocol writes them, until
/// the game is over or `limit` moves.
fn game(mut state: u64, limit: usize) -> (Vec<String>, Board) {
    let mut board = Board::new();
    let mut texts = Vec::new();
    while board.status() == Status::Ongoing && texts.len() < limit {
        let mut legal = MoveList::new();
        board.legal_moves(&mut legal);
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let mv: Move = legal[(state >> 33) as usize % legal.len()];
        texts.push(mv.to_string());
        board.play(mv);
    }
    (texts, board)
}

fn request_with(moves: &[String], rest: &str) -> String {
    format!(
        "{{\"moves\":{}{rest}}}",
        serde_json::to_string(moves).unwrap()
    )
}

#[test]
fn the_settings_are_those_of_the_newest_release() {
    let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("../releases");
    let mut newest: Option<(u32, String)> = None;
    for entry in fs::read_dir(&folder).expect("the releases folder") {
        let name = entry.unwrap().file_name().into_string().unwrap();
        let number = name
            .strip_prefix("uttt-v")
            .and_then(|rest| rest.strip_suffix(".rs"))
            .and_then(|digits| digits.parse::<u32>().ok());
        if let Some(number) = number {
            if newest.as_ref().is_none_or(|(best, _)| number > *best) {
                newest = Some((number, name));
            }
        }
    }
    let (_, name) = newest.expect("a release");
    let release = fs::read_to_string(folder.join(&name))
        .unwrap()
        .replace("\r\n", "\n");
    for constant in [
        format!("const EXPLORATION: f64 = {:?};", EXPLORATION),
        format!("const PRIORS: bool = {PRIORS};"),
        format!("const PRIOR_WEIGHT: f64 = {:?};", PRIOR_WEIGHT),
        format!("const PLIES: u32 = {PLIES};"),
    ] {
        assert!(
            release.lines().any(|line| line == constant),
            "{name} has no line `{constant}`: update the analysis program to its settings"
        );
    }
    let start = "pub const PATTERN_TEXT: &str = \"\\\n";
    let from = release
        .find(start)
        .expect("the release holds the pattern text")
        + start.len();
    let length = release[from..].find("\";").expect("the pattern text ends");
    let text = release[from..from + length].replace("\\\n", "");
    assert_eq!(
        text,
        pattern_weights::PATTERN_TEXT,
        "{name}'s pattern weights differ"
    );
}

#[test]
fn the_start_position_gives_a_report_every_step() {
    let out = lines(r#"{"moves":[],"iterations":3000,"report_every":1000}"#);
    assert_eq!(out.len(), 3);
    for (index, line) in out.iter().enumerate() {
        let report = report(line);
        assert_eq!(report["iterations"], 1000 * (index as u64 + 1));
        assert_eq!(report["to_move"], 0);
        assert_eq!(report["moves"].as_array().unwrap().len(), 81);
        assert_eq!(report["done"], index == 2);
        assert_eq!(
            report["stop"],
            if index == 2 {
                json!("iterations")
            } else {
                Value::Null
            }
        );
    }
    let last = report(&out[2]);
    assert!(last["nodes"].as_u64().unwrap() > 3000);
}

#[test]
fn a_report_has_the_documented_shape() {
    let out = lines(r#"{"moves":["4 4","4 3"],"iterations":4000,"report_every":2000,"depth":3}"#);
    let report = report(&out[1]);
    assert_eq!(report["to_move"], 0);
    assert_eq!(report["proven"], Value::Null);
    let moves = report["moves"].as_array().unwrap();
    // "4 3" sent X to the empty small board 3: its 9 cells.
    let legal: Vec<&str> = moves.iter().map(|m| m["move"].as_str().unwrap()).collect();
    assert_eq!(legal.len(), 9, "{legal:?}");
    assert!(legal.iter().all(|text| text.split(' ').count() == 2));
    assert_eq!(report["wdl"], moves[0]["wdl"]);
    // After the first, by visits, most first.
    let visits: Vec<u64> = moves
        .iter()
        .map(|m| m["visits"].as_u64().unwrap())
        .collect();
    assert!(
        visits[1..].windows(2).all(|pair| pair[0] >= pair[1]),
        "{visits:?}"
    );
    let total: u64 = visits.iter().sum();
    assert!((3990..=4000).contains(&total), "{total}");
    for entry in moves {
        let wdl: Vec<f64> = entry["wdl"]
            .as_array()
            .unwrap()
            .iter()
            .map(|n| n.as_f64().unwrap())
            .collect();
        assert_eq!(wdl.len(), 3);
        assert!(wdl.iter().all(|n| (0.0..=1.0).contains(n)));
        // Four decimals at most, adding up to 1.
        let tenths: i64 = wdl.iter().map(|n| (n * 10_000.0).round() as i64).sum();
        assert_eq!(tenths, 10_000);
        assert!(wdl
            .iter()
            .all(|n| ((n * 10_000.0).round() / 10_000.0 - n).abs() < 1e-12));
        let line = entry["line"].as_array().unwrap();
        assert!(!line.is_empty() && line.len() <= 3);
        assert_eq!(line[0], entry["move"]);
    }
}

#[test]
fn the_same_request_gives_the_same_lines() {
    let request = r#"{"moves":["4 4","3 3"],"iterations":5000,"report_every":1000,"seed":"7"}"#;
    assert_eq!(lines(request), lines(request));
    let other = r#"{"moves":["4 4","3 3"],"iterations":5000,"report_every":1000,"seed":"8"}"#;
    assert_ne!(lines(request), lines(other));
}

#[test]
fn a_larger_report_every_than_the_iterations_gives_one_report() {
    let out = lines(r#"{"moves":[],"iterations":1500,"report_every":10000}"#);
    assert_eq!(out.len(), 1);
    let report = report(&out[0]);
    assert_eq!(report["iterations"], 1500);
    assert_eq!(report["done"], true);
    assert_eq!(report["stop"], "iterations");
}

#[test]
fn steps_do_not_change_the_search() {
    // The same iterations reported once or in steps give the same last line.
    let one = lines(r#"{"moves":["4 4"],"iterations":3000,"report_every":3000}"#);
    let many = lines(r#"{"moves":["4 4"],"iterations":3000,"report_every":1000}"#);
    assert_eq!(one.len(), 1);
    assert_eq!(many.len(), 3);
    assert_eq!(one[0], many[2]);
}

#[test]
fn emit_returning_false_stops_at_once() {
    let out = lines_stopping(
        r#"{"moves":[],"iterations":100000,"report_every":1000}"#,
        |count| count < 2,
    );
    assert_eq!(out.len(), 2);
    assert_eq!(report(&out[1])["done"], false);
    let out = lines_stopping(r#"{"moves":["9 9"]}"#, |_| false);
    assert_eq!(out.len(), 1);
}

#[test]
fn a_finished_game_gives_its_result_alone() {
    let mut seen = [false; 3];
    for state in 0..200 {
        let (texts, board) = game(state, 100);
        let expected = match board.status() {
            Status::Win(0) => (0, json!([1.0, 0.0, 0.0])),
            Status::Draw => (1, json!([0.0, 1.0, 0.0])),
            Status::Win(_) => (2, json!([0.0, 0.0, 1.0])),
            Status::Ongoing => continue,
        };
        seen[expected.0] = true;
        let out = lines(&request_with(&texts, ""));
        assert_eq!(out.len(), 1);
        let report = report(&out[0]);
        assert_eq!(report["wdl"], expected.1);
        assert_eq!(report["iterations"], 0);
        assert_eq!(report["nodes"], 0);
        assert_eq!(report["to_move"], Value::Null);
        assert_eq!(report["proven"], Value::Null);
        assert_eq!(report["moves"], json!([]));
        assert_eq!(report["done"], true);
        assert_eq!(report["stop"], "over");
        // A move after the end is an error.
        let mut more = texts.clone();
        more.push("4 4".to_string());
        let message = error_of(&request_with(&more, ""));
        assert!(message.contains("after the game is over"), "{message}");
    }
    assert!(
        seen[0] && seen[2],
        "random games end in wins for both seats: {seen:?}"
    );
}

#[test]
fn a_position_one_move_from_the_end_is_proven() {
    let mut found = 0;
    for state in 0..200 {
        let (texts, board) = game(state, 100);
        let Status::Win(winner) = board.status() else {
            continue;
        };
        let before = &texts[..texts.len() - 1];
        let out = lines(&request_with(
            before,
            r#","iterations":50000,"report_every":1000"#,
        ));
        let last = report(out.last().unwrap());
        assert_eq!(last["done"], true);
        assert_eq!(last["stop"], "proven");
        assert_eq!(last["proven"], winner);
        assert!(last["iterations"].as_u64().unwrap() < 50_000);
        assert_eq!(last["moves"][0]["proven"], winner);
        let wdl = if winner == 0 {
            json!([1.0, 0.0, 0.0])
        } else {
            json!([0.0, 0.0, 1.0])
        };
        assert_eq!(last["wdl"], wdl);
        found += 1;
        if found == 5 {
            return;
        }
    }
    panic!("no game found");
}

#[test]
fn a_position_with_one_legal_move_reports_it() {
    for state in 0..200 {
        let (texts, _) = game(state, 100);
        for length in 1..texts.len() {
            let mut board = Board::new();
            for text in &texts[..length] {
                let (row, col) = text.split_once(' ').unwrap();
                board.play(Move::from_row_col(row.parse().unwrap(), col.parse().unwrap()).unwrap());
            }
            let mut legal = MoveList::new();
            board.legal_moves(&mut legal);
            if legal.len() == 1 && board.status() == Status::Ongoing {
                let out = lines(&request_with(
                    &texts[..length],
                    r#","iterations":2000,"report_every":1000"#,
                ));
                let last = report(out.last().unwrap());
                assert_eq!(last["moves"].as_array().unwrap().len(), 1);
                assert_eq!(last["moves"][0]["move"], legal[0].to_string());
                assert_eq!(last["done"], true);
                return;
            }
        }
    }
    panic!("no position with one legal move");
}

#[test]
fn the_tree_stops_at_the_node_limit() {
    let request = Request::parse(r#"{"moves":[],"iterations":1000000,"report_every":1000}"#)
        .expect("a request");
    let mut out = Vec::new();
    search(
        &request,
        5_000,
        &mut |line| {
            out.push(line.to_string());
            true
        },
        &|| false,
    );
    let last = report(out.last().unwrap());
    assert_eq!(last["stop"], "nodes");
    assert_eq!(last["done"], true);
    assert!(last["nodes"].as_u64().unwrap() >= 5_000);
    assert!(last["iterations"].as_u64().unwrap() < 1_000_000);
    // Only the last report says why it stopped.
    for line in &out[..out.len() - 1] {
        assert_eq!(report(line)["stop"], Value::Null);
    }
}

#[test]
fn a_bad_request_gives_one_error_line() {
    let long = "x".repeat(300);
    let many = format!("{{\"moves\":[{}]}}", vec!["\"4 4\""; 201].join(","));
    let cases: Vec<(String, &str)> = vec![
        ("".into(), "invalid JSON"),
        ("{".into(), "invalid JSON"),
        ("not json".into(), "invalid JSON"),
        ("[]".into(), "must be a JSON object"),
        ("null".into(), "must be a JSON object"),
        ("{}".into(), "missing field \"moves\""),
        (r#"{"moves":"4 4"}"#.into(), "must be a list"),
        (r#"{"moves":[4]}"#.into(), "move 1 must be a string"),
        (
            r#"{"moves":["4 4",null]}"#.into(),
            "move 2 must be a string",
        ),
        (
            r#"{"moves":["9 9"]}"#.into(),
            "move 1 (\"9 9\") is not legal",
        ),
        (
            r#"{"moves":["4 4","9 9"]}"#.into(),
            "move 2 (\"9 9\") is not legal",
        ),
        (
            r#"{"moves":["4 4","4 4"]}"#.into(),
            "move 2 (\"4 4\") is not legal",
        ),
        (
            r#"{"moves":["4 4","0 0"]}"#.into(),
            "move 2 (\"0 0\") is not legal",
        ),
        (r#"{"moves":["4,4"]}"#.into(), "is not legal"),
        (r#"{"moves":["4  4"]}"#.into(), "is not legal"),
        (r#"{"moves":["-1 4"]}"#.into(), "is not legal"),
        (r#"{"moves":["04 4"]}"#.into(), "is not legal"),
        (r#"{"moves":[""]}"#.into(), "is not legal"),
        (format!("{{\"moves\":[\"{long}\"]}}"), "..."),
        (many, "at most 200"),
        (
            r#"{"moves":[],"extra":1}"#.into(),
            "unknown field \"extra\"",
        ),
        (r#"{"moves":[],"Moves":[]}"#.into(), "unknown field"),
        (
            r#"{"moves":[],"iterations":0}"#.into(),
            "\"iterations\" must be from 1 to 100000000",
        ),
        (
            r#"{"moves":[],"iterations":100000001}"#.into(),
            "\"iterations\"",
        ),
        (
            r#"{"moves":[],"iterations":-5}"#.into(),
            "\"iterations\" must be an integer",
        ),
        (
            r#"{"moves":[],"iterations":1.5}"#.into(),
            "\"iterations\" must be an integer",
        ),
        (
            r#"{"moves":[],"iterations":"1000"}"#.into(),
            "\"iterations\" must be an integer",
        ),
        (
            r#"{"moves":[],"iterations":null}"#.into(),
            "\"iterations\" must be an integer",
        ),
        (r#"{"moves":[],"iterations":1e30}"#.into(), "\"iterations\""),
        (
            r#"{"moves":[],"report_every":999}"#.into(),
            "\"report_every\" must be from 1000",
        ),
        (
            r#"{"moves":[],"report_every":10000001}"#.into(),
            "\"report_every\"",
        ),
        (
            r#"{"moves":[],"report_every":true}"#.into(),
            "\"report_every\" must be an integer",
        ),
        (
            r#"{"moves":[],"depth":0}"#.into(),
            "\"depth\" must be from 1 to 40",
        ),
        (r#"{"moves":[],"depth":41}"#.into(), "\"depth\""),
        (
            r#"{"moves":[],"depth":[3]}"#.into(),
            "\"depth\" must be an integer",
        ),
        (
            r#"{"moves":[],"seed":1}"#.into(),
            "\"seed\" must be a string",
        ),
        (
            r#"{"moves":[],"seed":""}"#.into(),
            "\"seed\" must be a string",
        ),
        (
            r#"{"moves":[],"seed":"-1"}"#.into(),
            "\"seed\" must be a string",
        ),
        (
            r#"{"moves":[],"seed":"+1"}"#.into(),
            "\"seed\" must be a string",
        ),
        (
            r#"{"moves":[],"seed":"1.5"}"#.into(),
            "\"seed\" must be a string",
        ),
        (
            r#"{"moves":[],"seed":"abc"}"#.into(),
            "\"seed\" must be a string",
        ),
        (
            r#"{"moves":[],"seed":"18446744073709551616"}"#.into(),
            "at most 18446744073709551615",
        ),
        (
            format!("{}{}", "[".repeat(10_000), "]".repeat(10_000)),
            "invalid JSON",
        ),
    ];
    for (request, expected) in cases {
        let message = error_of(&request);
        assert!(
            message.contains(expected),
            "for {:.60}: {message:?} lacks {expected:?}",
            request
        );
    }
    // The limits themselves are fine.
    let last = "18446744073709551615";
    for ok in [
        format!(r#"{{"moves":[],"iterations":1,"seed":"{last}"}}"#),
        r#"{"moves":[],"iterations":1,"report_every":1000,"depth":40,"seed":"0"}"#.to_string(),
        r#"{"moves":[],"iterations":1,"report_every":10000000,"depth":1}"#.to_string(),
    ] {
        let out = lines(&ok);
        assert_eq!(out.len(), 1, "{ok}");
        assert!(json(&out[0]).get("report").is_some(), "{ok}");
    }
}

#[test]
fn two_hundred_moves_are_accepted_but_a_game_is_shorter() {
    // 200 entries pass the count check; a game is at most 81 moves, so the
    // 82nd entry is after the end or not legal.
    let (texts, _) = game(3, 100);
    assert!(texts.len() <= 81);
    let mut more = texts.clone();
    more.resize(200, "4 4".to_string());
    let message = error_of(&request_with(&more, ""));
    assert!(message.contains("after the game is over"), "{message}");
}

#[test]
fn chances_are_exact_for_proven_moves_and_add_up_to_one() {
    let entry = |score, draws, proven| MoveReport {
        mv: Move::new(0, 0),
        visits: 10,
        score,
        draws,
        proven,
        line: Vec::new(),
    };
    assert_eq!(chances(&entry(1.0, 0.0, Some(true)), 0), [1.0, 0.0, 0.0]);
    assert_eq!(chances(&entry(1.0, 0.0, Some(true)), 1), [0.0, 0.0, 1.0]);
    assert_eq!(chances(&entry(0.0, 0.0, Some(false)), 0), [0.0, 0.0, 1.0]);
    assert_eq!(chances(&entry(0.0, 0.0, Some(false)), 1), [1.0, 0.0, 0.0]);
    // Score 0.6 with a fifth of draws: wins 0.5, loses 0.3.
    assert_eq!(chances(&entry(0.6, 0.2, None), 0), [0.5, 0.2, 0.3]);
    assert_eq!(chances(&entry(0.6, 0.2, None), 1), [0.3, 0.2, 0.5]);
    // Drift (more draws than the score allows) is clamped and renormalised.
    let drift = chances(&entry(0.1, 0.5, None), 0);
    assert_eq!(drift, [0.0, 0.4348, 0.5652]);
    assert_eq!(
        drift
            .iter()
            .map(|n| (n * 10_000.0).round() as i64)
            .sum::<i64>(),
        10_000
    );
    assert!(drift.iter().all(|n| *n >= 0.0));
}

#[test]
fn a_move_the_search_has_not_tried_has_no_wdl() {
    // One iteration tries one of the 81 moves.
    let out = lines(r#"{"moves":[],"iterations":1}"#);
    let report = report(&out[0]);
    let moves = report["moves"].as_array().unwrap();
    assert_eq!(moves.len(), 81);
    let untried: Vec<&Value> = moves.iter().filter(|m| m["visits"] == 0).collect();
    assert_eq!(untried.len(), 80);
    for entry in untried {
        assert_eq!(entry["wdl"], Value::Null);
        assert_eq!(entry["proven"], Value::Null);
        assert_eq!(entry["line"], json!([entry["move"]]));
    }
    // The position's chances are those of the first move, which was tried.
    assert_eq!(moves[0]["visits"], 1);
    assert_eq!(report["wdl"], moves[0]["wdl"]);
    assert!(report["wdl"].is_array());
}

#[test]
fn a_stop_ends_the_search_within_a_step_and_writes_no_line() {
    // One report would come after 10 million iterations.
    let request = r#"{"moves":[],"iterations":100000000,"report_every":10000000}"#;
    let asked = std::cell::Cell::new(0_u32);
    let mut out = Vec::new();
    analyse_with_stop(
        request,
        &mut |line| {
            out.push(line.to_string());
            true
        },
        &|| {
            asked.set(asked.get() + 1);
            // Say stop at the third look: 2 steps of 1000 iterations were run.
            asked.get() >= 3
        },
    );
    assert!(out.is_empty(), "{out:?}");
    assert_eq!(asked.get(), 3);
}

#[test]
fn a_stop_that_never_comes_gives_the_lines_of_analyse() {
    let request = r#"{"moves":["4 4"],"iterations":3000,"report_every":1000}"#;
    let mut with_stop = Vec::new();
    analyse_with_stop(
        request,
        &mut |line| {
            with_stop.push(line.to_string());
            true
        },
        &|| false,
    );
    assert_eq!(with_stop, lines(request));
}
