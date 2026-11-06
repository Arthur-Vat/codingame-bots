use super::*;

/// A game whose seed is `seed=N` and whose answers are every `stdout`.
struct Toy;

impl FromCodinGame for Toy {
    const GAME: &'static str = "toy";

    fn seed(options: &str) -> Result<u64, String> {
        options
            .lines()
            .find_map(|line| line.trim().strip_prefix("seed="))
            .ok_or_else(|| "no seed".to_string())?
            .parse()
            .map_err(|e| format!("bad seed: {e}"))
    }

    fn answers(log: &Log) -> Result<Vec<Vec<RecordedAnswer>>, String> {
        Ok(log
            .frames
            .iter()
            .filter_map(|f| {
                Some(vec![RecordedAnswer {
                    seat: f.agent?,
                    lines: vec![f.stdout.clone()?],
                    ms: 0.0,
                }])
            })
            .collect())
    }
}

fn log(text: &str) -> Log {
    serde_json::from_str(text).expect("a log")
}

const FINISHED: &str = r#"{"game_id": 42, "game_options": "seed=12\nwidth=30",
  "agents": [{"index": 1, "name": "B", "agent_id": -5}, {"index": 0, "name": "A", "agent_id": 7, "me": true}],
  "frames": [{"frame": 0}, {"frame": 1, "agent": 0, "stdout": "WAIT", "stderr": "x", "extra": 1},
             {"frame": 2, "agent": 0, "summary": "$0 did"}, {"frame": 3, "agent": 1, "stdout": "WAIT"}],
  "scores": [3.0, 1.0], "ranking": [0, 1], "tooltips": [], "metadata": {}}"#;

#[test]
fn a_log_becomes_a_record() {
    let record = to_record::<Toy>(&log(FINISHED), "ranking").expect("converts");
    assert_eq!(record.format, RECORD_FORMAT);
    assert_eq!(record.game, "toy");
    assert_eq!(record.seed, 12);
    assert_eq!(record.opening_plies, 0);
    assert_eq!(record.source, "codingame ranking game 42");
    assert_eq!(record.players[0].name, "A");
    assert_eq!(record.players[1].name, "B");
    assert_eq!(record.players[0].kind, PlayerKind::Bot);
    assert_eq!(record.players[0].bot_seed, None);
    assert_eq!(record.players[0].command, None);
    assert_eq!(record.players[1].time_scale, 1.0);
    assert_eq!(record.turns.len(), 2);
    assert_eq!(record.turns[1][0].seat, 1);
    assert_eq!(record.end, EndReason::Finished);
    assert_eq!(record.winner, Some(0));
    let origin = record.origin.expect("an origin");
    assert_eq!(
        (origin.site.as_str(), origin.game_id, origin.kind.as_str()),
        ("codingame", 42, "ranking")
    );
    assert_eq!(origin.agent_ids, [7, -5]);
}

#[test]
fn equal_scores_are_a_draw_and_a_missing_options_member_reaches_the_game() {
    let text = FINISHED.replace("[3.0, 1.0]", "[2.0, 2.0]");
    assert_eq!(
        to_record::<Toy>(&log(&text), "test")
            .expect("converts")
            .winner,
        None
    );
    let text = FINISHED.replace(r#""game_options": "seed=12\nwidth=30","#, "");
    let error = to_record::<Toy>(&log(&text), "test").expect_err("no seed");
    assert!(error.contains("42") && error.contains("no seed"), "{error}");
}

#[test]
fn a_timeout_in_a_summary_ends_the_game_by_timeout() {
    let text = FINISHED.replace(
        r#"{"frame": 3, "agent": 1, "stdout": "WAIT"}"#,
        r#"{"frame": 3, "agent": 0, "summary": "$1 timeout!"}"#,
    );
    let record = to_record::<Toy>(&log(&text), "ranking").expect("converts");
    assert_eq!(
        record.end,
        EndReason::Timeout {
            seat: 1,
            limit_ms: 0.0
        }
    );
    let text = FINISHED.replace(r#""ranking""#, r#""fail_cause": "TIMEOUT", "ranking""#);
    assert!(to_record::<Toy>(&log(&text), "ranking").is_err());
    let text = text.replace(
        r#"{"frame": 0}"#,
        r#"{"frame": 0, "agent": 1, "error": {"line": 3, "message": "boom"}}"#,
    );
    let record = to_record::<Toy>(&log(&text), "ranking").expect("converts");
    assert_eq!(
        record.end,
        EndReason::Timeout {
            seat: 1,
            limit_ms: 0.0
        }
    );
    let text = text.replace("TIMEOUT", "EXCEPTION");
    let record = to_record::<Toy>(&log(&text), "ranking").expect("converts");
    assert_eq!(
        record.end,
        EndReason::Crash {
            seat: 1,
            detail: "EXCEPTION".to_string()
        }
    );
}

#[test]
fn malformed_logs_are_errors_naming_the_game() {
    let three = FINISHED.replace(r#""frames""#, r#""agents2": 1, "frames""#);
    let mut parsed = log(&three);
    parsed.agents.push(Agent {
        index: 2,
        name: "C".to_string(),
        agent_id: Some(1),
    });
    let error = to_record::<Toy>(&parsed, "ranking").expect_err("three agents");
    assert!(error.contains("game 42"), "{error}");

    let mut parsed = log(FINISHED);
    parsed.agents[0].index = 0;
    assert!(to_record::<Toy>(&parsed, "ranking").is_err());

    let mut parsed = log(FINISHED);
    parsed.agents[0].agent_id = None;
    assert!(to_record::<Toy>(&parsed, "ranking").is_err());

    let mut parsed = log(FINISHED);
    parsed.scores.clear();
    assert!(to_record::<Toy>(&parsed, "ranking").is_err());

    let mut parsed = log(FINISHED);
    parsed.ranking = vec![5];
    assert!(to_record::<Toy>(&parsed, "ranking").is_err());

    assert!(serde_json::from_str::<Log>("{\"game_id\": ").is_err());
    assert!(serde_json::from_str::<Log>("{}").is_err());
}

#[test]
fn a_log_file_is_read_and_errors_name_the_path() {
    let dir = std::env::temp_dir().join(format!("cg-codingame-{}", std::process::id()));
    fs::create_dir_all(&dir).expect("a folder");
    let good = dir.join("42.json");
    fs::write(&good, FINISHED).expect("a file");
    assert_eq!(read_log(&good).expect("reads").game_id, 42);
    let bad = dir.join("bad.json");
    fs::write(&bad, "[").expect("a file");
    assert!(read_log(&bad).expect_err("bad").contains("bad.json"));
    assert!(read_log(&dir.join("missing.json")).is_err());
    fs::remove_dir_all(&dir).expect("cleaned up");
}
