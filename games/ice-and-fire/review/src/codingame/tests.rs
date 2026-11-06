use cg_arena::record::PlayerKind;

use crate::test_util::temp_dir;

use super::*;

/// A real log: red lost its HQ at answer 42.
const FINISHED: &str = include_str!("../../testdata/914386214.json");
/// A real log: blue timed out on its first turn.
const TIMEOUT: &str = include_str!("../../testdata/914333422.json");

fn log(text: &str) -> Log {
    serde_json::from_str(text).expect("a log")
}

fn convert(text: &str) -> (Log, Record) {
    let log = log(text);
    let record = to_record::<IceAndFire>(&log, "ranking").expect("converts");
    (log, record)
}

fn details(checked: &Checked) -> Vec<(Option<usize>, u8)> {
    checked.findings.iter().map(|f| (f.ply, f.check)).collect()
}

#[test]
fn the_seed_is_the_signed_number_of_the_options() {
    let seed = IceAndFire::seed;
    assert_eq!(seed("seed=12\nwidth=30"), Ok(12));
    assert_eq!(seed("width=30\n  seed=7  \n"), Ok(7));
    assert_eq!(
        seed("seed=-474809778509893000"),
        Ok((-474_809_778_509_893_000_i64) as u64)
    );
    assert_eq!(
        seed("seed=4994961277120056000"),
        Ok(4_994_961_277_120_056_000)
    );
    assert!(seed("").is_err());
    assert!(seed("width=30").is_err());
    assert!(seed("seed=abc").is_err());
    assert!(seed("seed=").is_err());
    assert!(seed("seed=99999999999999999999").is_err());
}

fn frames(text: &str) -> Log {
    log(&format!(r#"{{"game_id": 1, "frames": [{text}]}}"#))
}

#[test]
fn the_answers_are_the_stdout_frames_in_order_and_alternate_from_agent_0() {
    let turns = IceAndFire::answers(&frames(
        r#"{"frame": 0},
        {"frame": 1, "agent": 0, "stdout": "TRAIN 1 1 0;TRAIN 1 2 0"},
        {"frame": 2, "agent": 0, "summary": "$0 trained a unit in (1, 0)"},
        {"frame": 3, "agent": 1, "stdout": "WAIT"}"#,
    ))
    .expect("answers");
    assert_eq!(turns.len(), 2);
    assert_eq!(turns[0][0].seat, 0);
    assert_eq!(
        turns[0][0].lines,
        vec!["TRAIN 1 1 0;TRAIN 1 2 0".to_string()]
    );
    assert_eq!(turns[0][0].ms, 0.0);
    assert_eq!(turns[1][0].seat, 1);
    assert_eq!(IceAndFire::answers(&frames(r#"{"frame": 0}"#)), Ok(vec![]));
}

#[test]
fn answers_that_do_not_alternate_or_have_no_agent_are_refused() {
    let error = IceAndFire::answers(&frames(
        r#"{"frame": 1, "agent": 0, "stdout": "WAIT"},
        {"frame": 2, "agent": 0, "stdout": "WAIT"}"#,
    ))
    .expect_err("two answers of agent 0");
    assert!(error.contains("answer 2"), "{error}");
    assert!(IceAndFire::answers(&frames(r#"{"frame": 1, "agent": 1, "stdout": "WAIT"}"#)).is_err());
    assert!(IceAndFire::answers(&frames(r#"{"frame": 1, "stdout": "WAIT"}"#)).is_err());
}

#[test]
fn the_late_stdout_of_a_timeout_is_not_an_answer() {
    let turns = IceAndFire::answers(&frames(
        r#"{"frame": 1, "agent": 0, "stdout": "WAIT"},
        {"frame": 2, "agent": 1, "summary": "$1 timeout!", "stdout": "M"}"#,
    ))
    .expect("answers");
    assert_eq!(turns.len(), 1);
}

#[test]
fn a_finished_log_becomes_a_record_that_agrees() {
    let (log, record) = convert(FINISHED);
    assert_eq!(record.game, "ice-and-fire");
    assert_eq!(record.seed, 4_994_961_277_120_056_000);
    assert_eq!(record.turns.len(), 42);
    assert_eq!(record.players[1].name, "Tarleya");
    assert_eq!(record.players[0].kind, PlayerKind::Bot);
    assert_eq!(record.end, EndReason::Finished);
    assert_eq!(record.winner, Some(1));
    let origin = record.origin.clone().expect("an origin");
    assert_eq!(origin.game_id, 914_386_214);
    assert_eq!(origin.agent_ids, [5_313_748, 6_833_063]);

    let checked = check(&log, &record);
    assert_eq!(details(&checked), vec![]);
    assert_eq!(checked.plies, 42);
    assert!(checked.accepted > 0);
}

#[test]
fn a_timeout_log_becomes_a_record_that_agrees() {
    let (log, record) = convert(TIMEOUT);
    assert_eq!(
        record.end,
        EndReason::Timeout {
            seat: 1,
            limit_ms: 0.0
        }
    );
    assert_eq!(record.winner, Some(0));
    assert_eq!(record.turns.len(), 1);
    assert_eq!(details(&check(&log, &record)), vec![]);
}

#[test]
fn a_refused_command_where_codingame_shows_an_effect_is_found_by_check_4() {
    let (log, mut record) = convert(FINISHED);
    // Red's first answer trains at (1, 0) and (2, 0); (11, 11) is blue's HQ.
    record.turns[0][0].lines = vec!["TRAIN 1 1 0;TRAIN 1 11 11".to_string()];
    let checked = check(&log, &record);
    assert!(
        checked
            .findings
            .iter()
            .any(|f| f.check == 4 && f.ply == Some(1)),
        "{:?}",
        checked.findings
    );
}

#[test]
fn a_swapped_winner_is_found_by_check_2() {
    let (log, mut record) = convert(FINISHED);
    record.winner = Some(0);
    assert_eq!(details(&check(&log, &record)), vec![(None, 2)]);
    record.winner = None;
    assert_eq!(details(&check(&log, &record)), vec![(None, 2)]);

    let (log, mut record) = convert(TIMEOUT);
    record.winner = Some(1);
    assert_eq!(details(&check(&log, &record)), vec![(None, 2)]);
    record.winner = Some(0);
    record.end = EndReason::Timeout {
        seat: 0,
        limit_ms: 0.0,
    };
    assert!(check(&log, &record).findings.iter().any(|f| f.check == 2));
}

#[test]
fn answers_left_over_when_our_game_is_over_are_found_by_check_1() {
    let (log, mut record) = convert(FINISHED);
    let extra = record.turns[0].clone();
    record.turns.push(extra);
    let checked = check(&log, &record);
    assert!(checked
        .findings
        .iter()
        .any(|f| f.check == 1 && f.ply == Some(43)));
    // A game cut short is not over for us.
    let (log, mut record) = convert(FINISHED);
    record.turns.truncate(10);
    assert!(check(&log, &record).findings.iter().any(|f| f.check == 2));
}

#[test]
fn commands_the_parser_does_not_read_are_found_by_check_3_but_wait_and_msg_are_not() {
    let (log, mut record) = convert(FINISHED);
    record.turns[0][0].lines = vec!["TRAIN 1 1 0;TRAIN 1 2 0;WAIT;MSG hi there;;".to_string()];
    assert_eq!(details(&check(&log, &record)), vec![]);
    record.turns[0][0].lines = vec!["TRAIN 1 1 0;TRAIN 1 2 0;FLY 1 2;MSG".to_string()];
    let checked = check(&log, &record);
    let found: Vec<&str> = checked.findings.iter().map(|f| f.detail.as_str()).collect();
    assert_eq!(
        found.iter().filter(|d| d.starts_with("not read")).count(),
        2,
        "{found:?}"
    );
    // An empty answer is an empty part, which the rules do not read either.
    record.turns[0][0].lines = vec![String::new()];
    assert!(check(&log, &record).findings.iter().any(|f| f.check == 3));
}

#[test]
fn the_parts_of_an_answer_drop_the_empty_ones_at_the_end_only() {
    assert_eq!(parts("A;B;"), vec!["A", "B"]);
    assert_eq!(parts("A;;B;;;"), vec!["A", "", "B"]);
    assert_eq!(parts(" A ; B "), vec!["A", "B"]);
    assert_eq!(parts(""), vec![""]);
    assert_eq!(parts("A; "), vec!["A", ""]);
}

#[test]
fn effects_are_counted_by_the_seat_of_the_text_over_all_the_frames_of_a_ply() {
    let log = frames(
        r#"{"frame": 1, "agent": 0, "stdout": "A", "summary": "$0: Invalid action (invalid id) MOVE 9999 6 7"},
        {"frame": 2, "agent": 1, "summary": "$0 moved 1 to (1, 1)"},
        {"frame": 3, "agent": 0, "summary": "$0: Unit 3 stayed still (no nearest cell in range) MOVE 3 5 5\n$0 trained a unit in (1, 2)\n$1 moved 2 to (3, 3)"},
        {"frame": 4, "agent": 1, "stdout": "B"},
        {"frame": 5, "agent": 0, "summary": "$1 built a MINE in (10, 10)"},
        {"frame": 6, "agent": 0, "summary": "$0 timeout!"}"#,
    );
    assert_eq!(
        effects(&log),
        vec![
            Effects {
                done: 2,
                refused: 2
            },
            Effects {
                done: 1,
                refused: 0
            }
        ]
    );
}

#[test]
fn a_batch_is_written_checked_and_reported_with_its_conversion_errors() {
    let dir = temp_dir("codingame");
    let logs = dir.join("logs");
    fs::create_dir_all(&logs).expect("a folder");
    let mut paths = Vec::new();
    for (name, text) in [
        ("914386214.json", FINISHED.to_string()),
        ("914333422.json", TIMEOUT.to_string()),
        (
            "no-seed.json",
            TIMEOUT.replace("seed=-474809778509893000", "width=3"),
        ),
        ("broken.json", "{".to_string()),
    ] {
        let path = logs.join(name);
        fs::write(&path, text).expect("a log");
        paths.push(path);
    }
    let out = dir.join("records");
    let (text, clean) = run(&paths, &out, "ranking").expect("runs");
    assert!(!clean);
    assert!(text.contains("| 4 | 2 | 2 | 0 |"), "{text}");
    assert!(text.contains("## Conversion errors"), "{text}");
    assert!(
        text.contains("no-seed.json") && text.contains("broken.json"),
        "{text}"
    );
    assert!(!text.contains("## Disagreements"), "{text}");
    let written = fs::read_to_string(out.join("914386214.json")).expect("a record");
    let record: Record = serde_json::from_str(&written).expect("a record");
    assert_eq!(record.format, 2);
    assert_eq!(record.source, "codingame ranking game 914386214");

    let (_, clean) = run(&paths[..2], &out, "ranking").expect("runs");
    assert!(clean);

    let copy = logs.join("copy.json");
    fs::write(&copy, FINISHED).expect("a log");
    let (text, clean) = run(&[paths[0].clone(), copy], &out, "ranking").expect("runs");
    assert!(!clean);
    assert!(text.contains("duplicate of game 914386214"), "{text}");
    assert!(text.contains("| 2 | 1 | 1 | 0 |"), "{text}");
    fs::remove_dir_all(&dir).expect("cleaned up");
}
