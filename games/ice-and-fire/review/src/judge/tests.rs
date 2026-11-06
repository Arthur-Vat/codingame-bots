use std::os::unix::fs::PermissionsExt;

use crate::bank::{block, block_played};
use crate::test_util::{random_record, temp_dir};

use super::*;

const FINISHED: &str = include_str!("../../testdata/914386214.json");
const TIMEOUT: &str = include_str!("../../testdata/914333422.json");

/// A data directory of the two real logs (Gabbek against Tarleya) and a
/// snapshot with Gabbek at #3.
fn data_dir(name: &str) -> PathBuf {
    let dir = temp_dir(name);
    fs::create_dir_all(dir.join("logs")).expect("logs");
    fs::create_dir_all(dir.join("leaderboard")).expect("leaderboard");
    fs::write(dir.join("logs/914386214.json"), FINISHED).expect("log");
    fs::write(dir.join("logs/914333422.json"), TIMEOUT).expect("log");
    let line = |id: u64, kind: &str, seat: u64, names: [&str; 2]| {
        format!(
            "{{\"game_id\":{id},\"kind\":\"{kind}\",\"puzzle\":\"p\",\"our_seat\":{seat},\"players\":[{{\"seat\":0,\"name\":\"{}\"}},{{\"seat\":1,\"name\":\"{}\"}}],\"game_options\":\"seed=1\"}}\n",
            names[0], names[1]
        )
    };
    let index = [
        line(914386214, "ranking", 1, ["Gabbek", "Tarleya"]),
        line(914333422, "test", 0, ["Tarleya", "Gabbek"]),
        line(5, "ranking", 0, ["Tarleya", "Nobody"]),
    ]
    .concat();
    fs::write(dir.join("index.jsonl"), index).expect("index");
    fs::write(
        dir.join("leaderboard/2026-11-06T154110Z.json"),
        "{\"rows\":[{\"rank\":1,\"nickname\":\"zasmu\"},{\"rank\":3,\"nickname\":\"Gabbek\"}]}",
    )
    .expect("snapshot");
    dir
}

/// A script that stands for the plan bot: `body` runs with the bank file as
/// `$3` (`judge --positions FILE --tries N`).
fn stub(dir: &Path, body: &str) -> PathBuf {
    let file = dir.join("stub.sh");
    fs::write(&file, format!("#!/bin/sh\n{body}\n")).expect("script");
    fs::set_permissions(&file, fs::Permissions::from_mode(0o755)).expect("mode");
    file
}

const PRINT_ALL: &str = "grep '^position ' \"$3\" | while read -r _ name ply _; do printf '%s\\t%s\\tours=WAIT\\tours_score=10\\tplayed_score=4\\tsame=no\\n' \"$name\" \"$ply\"; done";

fn request(dir: &Path, judge: PathBuf) -> Request {
    Request {
        data: dir.to_path_buf(),
        me: "Tarleya".to_string(),
        judge,
        players: vec![],
        top: 10,
        tries: 7,
        limit: None,
        jobs: 2,
        leaderboard: None,
        out: dir.join("out"),
    }
}

/// Runs the request and reads back a file of its folder.
fn read(request: &Request, file: &str) -> String {
    fs::read_to_string(request.out.join(file)).expect(file)
}

#[test]
fn a_played_block_is_the_block_with_the_answer_before_end() {
    let game = replay("g", random_record(3, ["a", "b"]));
    let plain = block(&game, 5);
    let played = block_played(&game, 5);
    let answer = ice_and_fire_engine::answer_line(&game.steps[5].commands);
    assert_eq!(
        played,
        format!(
            "{}played {answer}\nend\n",
            plain.strip_suffix("end\n").unwrap()
        )
    );
}

#[test]
fn the_judges_lines_are_parsed_or_refused() {
    let ok = parse_output(
        "g1\t4\tours=TRAIN 1 2 2;MOVE 3 4 5\tours_score=-12\tplayed_score=30\tsame=no",
    )
    .expect("a line");
    assert_eq!(
        ok,
        Output {
            name: "g1".to_string(),
            ply: 4,
            ours: "TRAIN 1 2 2;MOVE 3 4 5".to_string(),
            ours_score: -12,
            played_score: 30,
        }
    );
    for bad in [
        "",
        "g1\t4",
        "g1\tx\tours=WAIT\tours_score=1\tplayed_score=1\tsame=no",
        "g1\t4\tWAIT\tours_score=1\tplayed_score=1\tsame=no",
        "g1\t4\tours=WAIT\tours_score=1\tplayed_score=-\tsame=-",
    ] {
        assert!(parse_output(bad).is_err(), "{bad:?}");
    }
}

#[test]
fn moves_compare_as_sets_then_by_first_command() {
    let ours = "TRAIN 1 2 2;MOVE 3 4 5";
    assert_eq!(
        Hooks::compare("MOVE 3 4 5;TRAIN 1 2 2", ours),
        MoveMatch::Same
    );
    assert_eq!(Hooks::compare("TRAIN 1 2 2", ours), MoveMatch::First);
    assert_eq!(Hooks::compare("MOVE 3 4 5", ours), MoveMatch::Different);
    assert_eq!(Hooks::compare("WAIT", "WAIT"), MoveMatch::Same);
    assert_eq!(Hooks::compare("WAIT", ours), MoveMatch::Different);
    assert_eq!(Hooks::gap_threshold(), GAP_THRESHOLD);
}

#[test]
fn the_report_judges_every_ply_of_the_chosen_players() {
    let dir = data_dir("judge-all");
    let judge = stub(&dir, PRINT_ALL);
    let asked = request(&dir, judge);
    run(&asked).expect("files");
    let readme = read(&asked, "README.md");
    assert!(readme.contains("players judged: Gabbek"), "{readme}");
    assert!(readme.contains("2 games used"), "{readme}");
    // Every judged ply has gap 10 - 4 = 6, and Gabbek is #3.
    let csv = read(&asked, "judgements.csv");
    let rows: Vec<&str> = csv.lines().skip(1).collect();
    assert!(rows.len() > 10, "{csv}");
    for row in &rows {
        let f: Vec<&str> = row.split(',').collect();
        assert_eq!(f[1], "Gabbek", "{row}");
        assert_eq!(f[2], "3", "{row}");
        assert_eq!(f[7..11], ["WAIT", "4", "10", "6"], "{row}");
    }
    let summary = read(&asked, "summary.csv");
    assert!(summary.contains("Gabbek,all,median_gap,6.0"), "{summary}");
    assert!(
        summary.contains("Gabbek,all,gap_over_threshold,0"),
        "{summary}"
    );
    let worst = read(&asked, "largest-gaps.md");
    assert!(worst.contains("- played (4):"), "{worst}");
    assert!(worst.contains("- ours (10): `WAIT`"), "{worst}");
    assert!(!worst.contains("Tarleya"), "{worst}");
}

#[test]
fn players_and_limit_select_the_plies() {
    let dir = data_dir("judge-select");
    let judge = stub(&dir, PRINT_ALL);
    let mut asked = request(&dir, judge);
    asked.players = vec!["Tarleya".to_string()];
    asked.limit = Some(5);
    run(&asked).expect("files");
    let readme = read(&asked, "README.md");
    assert!(readme.contains("5 of "), "{readme}");
    assert!(readme.contains("5 judged"), "{readme}");
    let summary = read(&asked, "summary.csv");
    assert!(summary.contains("Tarleya,all,plies,5"), "{summary}");
    assert!(!summary.contains("Gabbek"), "{summary}");
}

#[test]
fn a_failing_judge_is_an_error_naming_the_chunk() {
    let dir = data_dir("judge-fail");
    let judge = stub(&dir, "echo boom >&2; exit 3");
    let err = run(&request(&dir, judge)).expect_err("fails");
    assert!(err.contains("chunk-"), "{err}");
    assert!(err.contains("boom"), "{err}");
    let err = run(&request(&dir, dir.join("missing"))).expect_err("no program");
    assert!(err.contains("missing"), "{err}");
}

#[test]
fn malformed_lines_are_reported_and_their_plies_skipped() {
    let dir = data_dir("judge-bad");
    let judge = stub(
        &dir,
        "echo garbage; echo '1\t2\tours=WAIT\tours_score=1\tplayed_score=-\tsame=-'",
    );
    let asked = request(&dir, judge);
    run(&asked).expect("files");
    let readme = read(&asked, "README.md");
    assert!(
        readme.contains("malformed judge lines were skipped"),
        "{readme}"
    );
    assert!(readme.contains("0 judged"), "{readme}");
    assert_eq!(read(&asked, "judgements.csv").lines().count(), 1);
}

#[test]
fn without_players_or_a_snapshot_there_is_nothing_to_judge() {
    let dir = data_dir("judge-none");
    fs::remove_dir_all(dir.join("leaderboard")).expect("remove");
    let judge = stub(&dir, PRINT_ALL);
    let err = run(&request(&dir, judge)).expect_err("nobody");
    assert!(err.contains("no player to judge"), "{err}");
}

#[test]
fn plies_are_spread_evenly() {
    let all: Vec<usize> = (0..10).collect();
    assert_eq!(spread(all.clone(), Some(5)), vec![0, 2, 4, 6, 8]);
    assert_eq!(spread(all.clone(), Some(20)), all);
    assert_eq!(spread(all.clone(), None), all);
    assert_eq!(spread(all, Some(0)), Vec::<usize>::new());
}
