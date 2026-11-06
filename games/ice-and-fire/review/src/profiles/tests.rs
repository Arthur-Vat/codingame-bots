use std::fs;

use crate::test_util::temp_dir;

use super::*;

const FINISHED: &str = include_str!("../../testdata/914386214.json");
const TIMEOUT: &str = include_str!("../../testdata/914333422.json");

/// A data directory of the two real logs, two lines without a log, a
/// malformed line, and two snapshots (the newest has Gabbek at #3).
fn data_dir(name: &str) -> PathBuf {
    let dir = temp_dir(name);
    fs::create_dir_all(dir.join("logs")).expect("logs");
    fs::create_dir_all(dir.join("leaderboard")).expect("leaderboard");
    fs::write(dir.join("logs/914386214.json"), FINISHED).expect("log");
    fs::write(dir.join("logs/914333422.json"), TIMEOUT).expect("log");
    let line = |id: u64, kind: &str, seat: u64, names: [&str; 2], batch: &str| {
        format!(
            "{{\"game_id\":{id},\"kind\":\"{kind}\",\"puzzle\":\"p\",\"our_seat\":{seat},\"players\":[{{\"seat\":0,\"name\":\"{}\"}},{{\"seat\":1,\"name\":\"{}\"}}],\"game_options\":\"seed=1\"{batch}}}\n",
            names[0], names[1]
        )
    };
    let index = [
        line(914386214, "ranking", 1, ["Gabbek", "Tarleya"], ""),
        line(
            914333422,
            "test",
            0,
            ["Tarleya", "Gabbek"],
            ",\"batch\":\"b1\"",
        ),
        line(5, "ranking", 0, ["Tarleya", "Nobody"], ""),
        line(6, "replay", 0, ["Tarleya", "Other"], ""),
        "not json\n".to_string(),
    ]
    .concat();
    fs::write(dir.join("index.jsonl"), index).expect("index");
    fs::write(
        dir.join("leaderboard/2026-11-06T000000Z.json"),
        "{\"rows\":[{\"rank\":1,\"nickname\":\"zasmu\"},{\"rank\":9,\"nickname\":\"Gabbek\"}]}",
    )
    .expect("old");
    fs::write(
        dir.join("leaderboard/2026-11-06T154110Z.json"),
        "{\"puzzle\":\"p\",\"fetched_at\":\"x\",\"rows\":[{\"rank\":1,\"nickname\":\"zasmu\"},{\"rank\":3,\"nickname\":\"Gabbek\"}]}",
    )
    .expect("snapshot");
    dir
}

fn request(dir: &Path) -> Request {
    Request {
        data: dir.to_path_buf(),
        me: "Tarleya".to_string(),
        kinds: vec![],
        batch: None,
        top: 10,
        players: vec![],
        leaderboard: None,
        jobs: 2,
    }
}

#[test]
fn the_report_covers_the_games_and_lists_those_left_out() {
    let dir = data_dir("profiles-all");
    let (text, json) = run(&request(&dir)).expect("a report");
    assert!(text.contains("## Sources"), "{text}");
    assert!(text.contains("2026-11-06T154110Z.json"), "{text}");
    // Games 5 and 6 have no log, and one line is not JSON.
    assert!(text.contains("- 5: no log at"), "{text}");
    assert!(text.contains("- 6: no log at"), "{text}");
    assert!(text.contains("index line 5"), "{text}");
    assert!(text.contains("| Gabbek | 3 | 2 [n = 2] |"), "{text}");
    assert!(text.contains("### Gabbek (rank 3, 2 games)"), "{text}");
    assert!(text.contains("train level 1"), "{text}");
    let value: serde_json::Value = serde_json::from_str(&json).expect("json");
    assert_eq!(value["opponents"][0]["games"], 2);
    assert_eq!(value["profiles"][0]["player"], "Gabbek");
    // We won the finished game; the timeout game was blue's fault.
    assert_eq!(value["opponents"][0]["ours"]["won"], 2);
}

#[test]
fn kinds_batch_and_players_select_games() {
    let dir = data_dir("profiles-select");
    let mut only = request(&dir);
    only.kinds = vec!["test".to_string()];
    only.batch = Some("b1".to_string());
    only.players = vec!["Gabbek".to_string()];
    let (text, _) = run(&only).expect("a report");
    assert!(text.contains("| test | b1 | 1 |"), "{text}");
    assert!(!text.contains("| ranking |"), "{text}");
    assert!(text.contains("(rank 3, 1 games)"), "{text}");
    assert!(text.contains("Few games"), "{text}");
    // A snapshot named on the command line wins over the newest.
    let other = dir.join("other.json");
    fs::write(&other, "{\"rows\":[{\"rank\":7,\"nickname\":\"Gabbek\"}]}").expect("file");
    only.leaderboard = Some(other);
    let (text, _) = run(&only).expect("a report");
    assert!(text.contains("(rank 7, 1 games)"), "{text}");
}

#[test]
fn a_missing_index_is_an_error_not_a_panic() {
    let dir = temp_dir("profiles-empty");
    assert!(run(&request(&dir)).is_err());
    fs::write(dir.join("index.jsonl"), "").expect("index");
    let (text, _) = run(&request(&dir)).expect("an empty report");
    assert!(text.contains("0 games used"), "{text}");
}

#[test]
fn a_malformed_log_is_left_out() {
    let dir = data_dir("profiles-bad");
    fs::write(dir.join("logs/914333422.json"), "{ nope").expect("log");
    let (text, _) = run(&request(&dir)).expect("a report");
    assert!(text.contains("- 914333422:"), "{text}");
    assert!(text.contains("1 games used"), "{text}");
}
