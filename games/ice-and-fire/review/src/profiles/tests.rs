use std::fs;

use crate::test_util::temp_dir;

use cg_arena::codingame::to_record;

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
        line(7, "mystery", 0, ["Tarleya", "Other"], ""),
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
        release: None,
        our_seat: None,
        top: 10,
        players: vec![],
        leaderboard: None,
        jobs: 2,
    }
}

/// The text of the file `name` among `files`.
fn file(files: &[(String, String)], name: &str) -> String {
    files
        .iter()
        .find(|(n, _)| n == name)
        .map(|f| f.1.clone())
        .unwrap_or_else(|| panic!("no file {name}"))
}

#[test]
fn the_report_covers_the_games_and_lists_those_left_out() {
    let dir = data_dir("profiles-all");
    let (json, files) = run(&request(&dir)).expect("a report");
    assert!(file(&files, "README.md").contains("2026-11-06T154110Z.json"));
    // Games 5 and 6 have no log, and one line is not JSON.
    let left = file(&files, "left_out.csv");
    assert!(left.contains("\n5,no log at"), "{left}");
    assert!(left.contains("\n6,no log at"), "{left}");
    assert!(left.contains(",index line 5"), "{left}");
    assert!(file(&files, "opponents.csv").contains("\nGabbek,3,2,2,0,0,"));
    assert!(file(&files, "profile_results.csv").contains("\nGabbek,3,2,"));
    assert!(file(&files, "profile_events.csv").contains(",train level 1,"));
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
    let (_, files) = run(&only).expect("a report");
    let sources = file(&files, "sources.csv");
    assert!(sources.contains("\ntest,b1,1\n"), "{sources}");
    assert!(!sources.contains("ranking"), "{sources}");
    assert!(file(&files, "profile_results.csv").contains("\nGabbek,3,1,"));
    // A snapshot named on the command line wins over the newest.
    let other = dir.join("other.json");
    fs::write(&other, "{\"rows\":[{\"rank\":7,\"nickname\":\"Gabbek\"}]}").expect("file");
    only.leaderboard = Some(other);
    let (_, files) = run(&only).expect("a report");
    assert!(file(&files, "profile_results.csv").contains("\nGabbek,7,1,"));
}

#[test]
fn a_missing_index_is_an_error_not_a_panic() {
    let dir = temp_dir("profiles-empty");
    assert!(run(&request(&dir)).is_err());
    fs::write(dir.join("index.jsonl"), "").expect("index");
    let (_, files) = run(&request(&dir)).expect("an empty report");
    assert_eq!(file(&files, "sources.csv").lines().count(), 1);
    assert!(file(&files, "signals.md").contains("Nothing to compare"));
}

#[test]
fn a_malformed_log_is_left_out() {
    let dir = data_dir("profiles-bad");
    fs::write(dir.join("logs/914333422.json"), "{ nope").expect("log");
    let (_, files) = run(&request(&dir)).expect("a report");
    assert!(file(&files, "left_out.csv").contains("\n914333422,"));
    assert!(file(&files, "sources.csv").contains("\nranking,,1\n"));
}

#[test]
fn an_unknown_kind_is_left_out_and_a_wrong_name_is_warned_of() {
    let dir = data_dir("profiles-kind");
    let mut wrong = request(&dir);
    wrong.me = "Someone".to_string();
    let (_, files) = run(&wrong).expect("a report");
    assert!(file(&files, "left_out.csv").contains("\n7,\"unknown kind \"\"mystery\"\""));
    assert!(file(&files, "README.md").contains("**Warning:** --me \"Someone\" differs"));
    let (_, files) = run(&request(&dir)).expect("a report");
    let readme = file(&files, "README.md");
    assert!(!readme.contains("**Warning:**"), "{readme}");
    assert!(readme.contains("commands carried out"), "{readme}");
}

#[test]
fn refused_commands_are_dropped_and_the_game_replays_the_same() {
    let log: cg_arena::codingame::Log = serde_json::from_str(FINISHED).expect("a log");
    let record = to_record::<IceAndFire>(&log, "ranking").expect("converts");
    // Red's first answer, with two commands the rules refuse: a level 3 unit
    // far from red's cells, and a tower on a cell red does not own.
    let mut sent = record.clone();
    let first = &mut sent.turns[0][0].lines[0];
    assert_eq!(first, "TRAIN 1 1 0;TRAIN 1 2 0");
    first.push_str(";TRAIN 3 11 11;BUILD TOWER 5 5");
    let kept = carried_out_record(&sent);
    assert_eq!(kept.turns[0][0].lines[0], "TRAIN 1 1 0;TRAIN 1 2 0");
    // The real record already holds only what was carried out, or refused
    // commands that change nothing: either way the game replays the same.
    for (a, b) in [(&sent, &kept), (&record, &carried_out_record(&record))] {
        let (a, b) = (replay("a", a.clone()), replay("b", b.clone()));
        assert_eq!(a.steps.len(), b.steps.len());
        assert_eq!(a.end.outcome(), b.end.outcome());
        for seat in 0..2 {
            assert_eq!(a.end.gold(seat), b.end.gold(seat));
            assert_eq!(a.end.owned(seat), b.end.owned(seat));
            assert_eq!(a.end.unit_count(seat), b.end.unit_count(seat));
        }
    }
    // The trace counts only what was carried out.
    let trace = crate::trace::trace("t", &kept);
    assert_eq!(trace.plies[0].commands, "TRAIN 1 1 0;TRAIN 1 2 0");
}

#[test]
fn the_files_have_charts_data_tables_and_a_readme_and_repeat() {
    let dir = data_dir("profiles-charts");
    let (_, files) = run(&request(&dir)).expect("a report");
    let names: Vec<&str> = files.iter().map(|f| f.0.as_str()).collect();
    for name in [
        "README.md",
        "signals.md",
        "separation.csv",
        "spending.csv",
        "turns-all-top-players.csv",
        "turns-our-wins.csv",
        "wins-vs-losses-cells.svg",
        "top-vs-us-event-train-level-1.svg",
    ] {
        assert!(names.contains(&name), "{name}");
    }
    let unique: std::collections::BTreeSet<&&str> = names.iter().collect();
    assert_eq!(unique.len(), names.len(), "a file written twice");
    let charts = names.iter().filter(|n| n.ends_with(".svg")).count();
    assert!(file(&files, "README.md").contains(&format!("{charts} SVG files")));
    assert!(files
        .iter()
        .filter(|f| f.0.ends_with(".svg"))
        .all(|f| f.1.starts_with("<svg ")));
    // The same data gives the same files.
    let (_, again) = run(&request(&dir)).expect("a report");
    assert_eq!(files, again);
}

#[test]
fn the_moments_cover_our_losses_and_list_those_left_out() {
    let dir = data_dir("moments-all");
    let text = run_moments(&request(&dir), 3, None).expect("a report");
    assert!(
        text.starts_with("# Decisive moments of our losses"),
        "{text}"
    );
    assert!(text.contains("- 5: no log at"), "{text}");
    assert!(text.contains("## Summary"), "{text}");
    assert!(text.contains("| no decisive moment |"), "{text}");
    assert!(text.contains("## Our exposure before the moment"), "{text}");
    assert!(text.contains("## Examples"), "{text}");
    for line in text.lines().filter(|l| l.starts_with('|')) {
        assert!(line.matches(" | ").count() < 6, "{line}");
    }
}

#[test]
fn the_moments_of_an_empty_selection_say_so() {
    let dir = data_dir("moments-none");
    let mut selection = request(&dir);
    selection.batch = Some("nothing".to_string());
    let text = run_moments(&selection, 3, None).expect("a report");
    assert!(text.contains("We lost 0 of 0 games"), "{text}");
}

/// The index of `data_dir` with the agent ids, the days and the releases
/// that `scripts/codingame.sh` writes, and snapshots of two days.
fn versioned_dir(name: &str) -> PathBuf {
    let dir = data_dir(name);
    let line = |id: u64,
                kind: &str,
                seat: u64,
                names: [&str; 2],
                agents: [u64; 2],
                day: &str,
                release: &str| {
        // Test games have no end time, only the day they were fetched.
        let ended = if kind == "test" {
            format!("\"ended_at\":null,\"fetched_at\":\"{day}T10:00:00Z\"")
        } else {
            format!("\"ended_at\":\"{day}T10:00:00.000Z\"")
        };
        format!(
            "{{\"game_id\":{id},\"kind\":\"{kind}\",\"puzzle\":\"p\",\"our_seat\":{seat},{ended},\"players\":[{{\"seat\":0,\"name\":\"{}\",\"agent_id\":{}}},{{\"seat\":1,\"name\":\"{}\",\"agent_id\":{}}}],\"game_options\":\"seed=1\",\"release\":\"{release}\"}}\n",
            names[0], agents[0], names[1], agents[1]
        )
    };
    let index = [
        line(
            914386214,
            "ranking",
            1,
            ["Gabbek", "Tarleya"],
            [100, 1],
            "2026-11-01",
            "v1",
        ),
        line(
            914333422,
            "test",
            0,
            ["Tarleya", "Gabbek"],
            [1, 101],
            "2026-11-02",
            "v2",
        ),
    ]
    .concat();
    fs::write(dir.join("index.jsonl"), index).expect("index");
    let snapshot = |agent: u64| {
        format!("{{\"rows\":[{{\"rank\":1,\"nickname\":\"zasmu\",\"agent_id\":9}},{{\"rank\":3,\"nickname\":\"Gabbek\",\"agent_id\":{agent}}}]}}")
    };
    fs::write(
        dir.join("leaderboard/2026-11-05T000000Z.json"),
        snapshot(100),
    )
    .expect("snapshot");
    fs::write(
        dir.join("leaderboard/2026-11-06T154110Z.json"),
        snapshot(101),
    )
    .expect("snapshot");
    dir
}

#[test]
fn a_player_with_two_agent_ids_is_split_into_versions() {
    let dir = versioned_dir("profiles-versions");
    let (json, files) = run(&request(&dir)).expect("a report");
    assert_eq!(
        file(&files, "profile_versions.csv"),
        "player,agent_id,games,won,lost,drawn,first_seen,last_seen\nGabbek,100,1,0,1,0,2026-11-01,2026-11-01\nGabbek,101,1,0,1,0,2026-11-02,2026-11-02\n"
    );
    // The snapshots of the 5th and 6th: Gabbek went from 100 to 101, zasmu
    // stayed.
    let versions = file(&files, "versions.csv");
    assert!(
        versions.contains("\nGabbek,2026-11-05,2026-11-05T000000Z.json,3,100,false\n"),
        "{versions}"
    );
    assert!(
        versions.contains("\nGabbek,2026-11-06,2026-11-06T154110Z.json,3,101,true\n"),
        "{versions}"
    );
    assert!(
        versions.contains("\nzasmu,2026-11-06,2026-11-06T154110Z.json,1,9,false\n"),
        "{versions}"
    );
    let signals = file(&files, "signals.md");
    assert!(
        signals.contains("Gabbek changed its agent id from 100 (2026-11-05) to 101 (2026-11-06)."),
        "{signals}"
    );
    assert!(
        signals.contains("Gabbek was met with 2 agent ids"),
        "{signals}"
    );
    let value: serde_json::Value = serde_json::from_str(&json).expect("json");
    assert_eq!(value["versions"]["changes"][0]["to_agent"], 101);
    assert_eq!(value["profiles"][0]["versions"][1]["agent_id"], 101);
}

#[test]
fn a_release_keeps_only_our_games_played_by_it() {
    let dir = versioned_dir("profiles-release");
    let mut only = request(&dir);
    only.release = Some("v2".to_string());
    let (_, files) = run(&only).expect("a report");
    let sources = file(&files, "sources.csv");
    assert!(
        sources.contains("\ntest,,1\n") && !sources.contains("ranking"),
        "{sources}"
    );
    assert!(file(&files, "README.md").contains("the release v2"));
    assert_eq!(
        file(&files, "profile_versions.csv"),
        "player,agent_id,games,won,lost,drawn,first_seen,last_seen\nGabbek,101,1,0,1,0,2026-11-02,2026-11-02\n"
    );
    only.release = Some("v3".to_string());
    let (_, files) = run(&only).expect("an empty report");
    assert_eq!(file(&files, "sources.csv").lines().count(), 1);
}

#[test]
fn a_seat_keeps_only_the_games_we_played_in_it() {
    // The ranking game has us in seat 1, the test game in seat 0.
    let dir = versioned_dir("profiles-seat");
    let mut only = request(&dir);
    only.our_seat = Some(1);
    let (_, files) = run(&only).expect("a report");
    let sources = file(&files, "sources.csv");
    assert!(
        sources.contains("\nranking,,1\n") && !sources.contains("test"),
        "{sources}"
    );
    assert!(file(&files, "README.md").contains("we sat in seat 1"));
    only.our_seat = Some(0);
    let (_, files) = run(&only).expect("a report");
    let sources = file(&files, "sources.csv");
    assert!(
        sources.contains("\ntest,,1\n") && !sources.contains("ranking"),
        "{sources}"
    );
}

#[test]
fn the_report_has_the_economy_numbers_of_the_real_games() {
    let dir = data_dir("profiles-economy");
    let (json, files) = run(&request(&dir)).expect("a report");
    let exchange = file(&files, "economy_exchange.csv");
    assert!(exchange.contains("\ntop players,early game,"), "{exchange}");
    assert!(
        exchange.contains("\n\"us, same games\",early game,"),
        "{exchange}"
    );
    assert!(file(&files, "economy_mix.csv").contains(",army,"));
    assert!(file(&files, "economy_tempo.csv").contains("\ntop players,2,"));
    assert!(file(&files, "economy_lead.csv").contains("\nmaterial,5,"));
    assert!(file(&files, "economy_curves.csv").contains("\neconomy-top-vs-us,exchange,"));
    assert!(files
        .iter()
        .any(|f| f.0 == "economy-top-vs-us-exchange.svg"));
    assert!(files
        .iter()
        .any(|f| f.0 == "economy-wins-vs-losses-cells-per-gold.svg"));
    assert!(file(&files, "signals.md").contains("## Economy or fighting"));
    let value: serde_json::Value = serde_json::from_str(&json).expect("json");
    assert_eq!(value["economy"]["decisive"][0], "large cut");
}
