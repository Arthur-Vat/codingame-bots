//! Tests of `studio-archive`'s library, on records played through the
//! server's routes and written to temporary folders.

use std::fs;
use std::path::{Path, PathBuf};

use cg_arena::record::Record;
use serde_json::{json, Value};
use studio_core::history::{parse_record, record_id};

use crate::archive::{build, InputSummary, Options, Summary};
use crate::history_tests::{finished_record, friend_record, ice_and_fire_record};
use crate::tests::{state, TempDir};

/// A game that resigned after `plies` moves, played at `unix_time`.
fn record(dir: &TempDir, seed: u64, plies: usize, unix_time: u64) -> Value {
    let state = state(&dir.path().join("web"), &dir.path().join("data"));
    let mut record = friend_record(&state, seed, plies);
    record["unix_time"] = json!(unix_time);
    record
}

/// Writes `record` as `path` under `root`, creating folders; the text is
/// indented, so that the archive's compact form is a change.
fn put(root: &Path, path: &str, record: &Value) {
    put_text(root, path, &serde_json::to_string_pretty(record).unwrap());
}

fn put_text(root: &Path, path: &str, text: &str) {
    let path = root.join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

fn id_of(record: &Value) -> String {
    let record: Record = parse_record(&serde_json::to_vec(record).unwrap()).unwrap();
    record_id(&record).unwrap()
}

fn options(out: &Path, game: Option<&str>, inputs: &[&Path]) -> Options {
    Options {
        out: out.to_path_buf(),
        game: game.map(str::to_string),
        inputs: inputs.iter().map(|input| input.to_path_buf()).collect(),
    }
}

fn index_ids(out: &Path) -> Vec<String> {
    let index: Value = serde_json::from_slice(&fs::read(out.join("index.json")).unwrap()).unwrap();
    index["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["id"].as_str().unwrap().to_string())
        .collect()
}

fn record_files(out: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(out.join("records"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    names
}

fn summary(input: &Path, records: usize, new: usize) -> InputSummary {
    InputSummary {
        input: input.to_path_buf(),
        records,
        new,
    }
}

#[test]
fn overlapping_inputs_keep_each_game_once_and_the_first_input_wins() {
    let dir = TempDir::new("archive-overlap");
    let (a, b, c) = (
        record(&dir, 1, 2, 1000),
        record(&dir, 2, 3, 2000),
        record(&dir, 3, 4, 3000),
    );
    let old = dir.path().join("old");
    let new = dir.path().join("new");
    put(&old, "a.json", &a);
    put(&old, "b.json", &b);
    // The same game twice in one folder, under another name.
    put(&old, "b-copy.json", &b);
    put(&new, "b.json", &b);
    put(&new, "c.json", &c);
    let out = dir.path().join("out");

    let built = build(&options(&out, None, &[&old, &new])).unwrap();
    assert_eq!(
        built,
        Summary {
            inputs: vec![summary(&old, 3, 2), summary(&new, 2, 1)],
            written: 3,
            duplicates: 2,
        }
    );
    let mut ids = [id_of(&a), id_of(&b), id_of(&c)];
    ids.sort();
    let names: Vec<String> = ids.iter().map(|id| format!("{id}.json")).collect();
    assert_eq!(record_files(&out), names);
    assert_eq!(index_ids(&out), vec![id_of(&c), id_of(&b), id_of(&a)]);
}

#[test]
fn the_order_of_the_inputs_decides_which_input_counts_a_record_as_new() {
    let dir = TempDir::new("archive-order");
    let a = record(&dir, 1, 2, 1000);
    let (one, two) = (dir.path().join("one"), dir.path().join("two"));
    put(&one, "a.json", &a);
    put(&two, "a.json", &a);
    let built = build(&options(&dir.path().join("out"), None, &[&two, &one])).unwrap();
    assert_eq!(built.inputs, vec![summary(&two, 1, 1), summary(&one, 1, 0)]);
}

#[test]
fn folders_are_searched_and_index_dot_files_and_other_files_are_ignored() {
    let dir = TempDir::new("archive-walk");
    let (a, b, c) = (
        record(&dir, 1, 2, 1000),
        record(&dir, 2, 3, 2000),
        record(&dir, 3, 4, 3000),
    );
    let input = dir.path().join("in");
    put(&input, "a.json", &a);
    put(&input, "deep/er/b.json", &b);
    put(&input, "deep/c.json", &c);
    // Not read, although they would fail: an archive's own index, dot files,
    // other extensions.
    put_text(&input, "index.json", "not json");
    put_text(&input, "deep/index.json", "{\"format\":1,\"rows\":[]}");
    put_text(&input, ".hidden.json", "not json");
    put_text(&input, "deep/.tmp-x.json", "not json");
    put_text(&input, "notes.txt", "not json");
    put_text(&input, "a.json.bak", "not json");
    let out = dir.path().join("out");

    let built = build(&options(&out, None, &[&input])).unwrap();
    assert_eq!(built.written, 3);
    assert_eq!(built.inputs, vec![summary(&input, 3, 3)]);
}

#[cfg(unix)]
#[test]
fn symbolic_links_are_skipped() {
    let dir = TempDir::new("archive-links");
    let a = record(&dir, 1, 2, 1000);
    let outside = dir.path().join("outside");
    put(&outside, "a.json", &a);
    let input = dir.path().join("in");
    fs::create_dir_all(&input).unwrap();
    std::os::unix::fs::symlink(outside.join("a.json"), input.join("link.json")).unwrap();
    std::os::unix::fs::symlink(&outside, input.join("folder")).unwrap();
    let built = build(&options(&dir.path().join("out"), None, &[&input])).unwrap();
    assert_eq!(built.written, 0);
}

#[test]
fn every_bad_file_is_named_and_nothing_is_written() {
    let dir = TempDir::new("archive-bad");
    let good = record(&dir, 1, 2, 1000);
    let mut unknown = record(&dir, 2, 3, 2000);
    unknown["game"] = json!("chess");
    // Seat 1 resigned, so seat 0 won.
    let mut wrong = record(&dir, 3, 4, 3000);
    wrong["winner"] = json!(1);
    let input = dir.path().join("in");
    put(&input, "good.json", &good);
    put_text(&input, "not-a-record.json", "{\"hello\": 1}");
    put(&input, "nested/unknown.json", &unknown);
    put(&input, "nested/wrong.json", &wrong);
    let other = dir.path().join("other");
    put_text(&other, "empty.json", "");
    let out = dir.path().join("out");

    let errors = build(&options(&out, None, &[&input, &other])).unwrap_err();
    assert_eq!(errors.len(), 4, "{errors:?}");
    let reason = |file: &str| {
        errors
            .iter()
            .find(|line| line.contains(file))
            .unwrap_or_else(|| panic!("{file} is not named in {errors:?}"))
            .clone()
    };
    for (path, reason_part) in [
        (input.join("not-a-record.json"), "invalid game record"),
        (input.join("nested").join("unknown.json"), "unknown game"),
        (input.join("nested").join("wrong.json"), "winner"),
        (other.join("empty.json"), "invalid game record"),
    ] {
        let line = reason(&path.display().to_string());
        assert!(
            line.starts_with(&format!("{}: ", path.display())) && line.contains(reason_part),
            "{line}"
        );
    }
    assert!(!out.exists());
}

#[test]
fn a_game_option_refuses_records_of_other_games() {
    let dir = TempDir::new("archive-game");
    let (a, b) = (record(&dir, 1, 2, 1000), record(&dir, 2, 3, 2000));
    let input = dir.path().join("in");
    put(&input, "a.json", &a);
    put(&input, "b.json", &b);
    let out = dir.path().join("out");

    let errors = build(&options(&out, Some("chess"), &[&input])).unwrap_err();
    assert_eq!(errors.len(), 2, "{errors:?}");
    assert!(
        errors.iter().all(|line| line.contains("--game")),
        "{errors:?}"
    );
    assert!(!out.exists());

    let built = build(&options(&out, Some("uttt"), &[&input])).unwrap();
    assert_eq!(built.written, 2);
}

#[test]
fn an_ice_and_fire_arena_record_is_archived_with_the_uttt_ones() {
    let dir = TempDir::new("archive-ice-and-fire");
    let uttt = record(&dir, 1, 2, 1000);
    let ice_and_fire = ice_and_fire_record(&["WAIT"; 200], json!({"kind": "finished"}), json!(1));
    let input = dir.path().join("in");
    put(&input, "uttt.json", &uttt);
    put(&input, "ice-and-fire/a.json", &ice_and_fire);
    let out = dir.path().join("out");

    // `--game` keeps the games apart.
    let errors = build(&options(&out, Some("uttt"), &[&input])).unwrap_err();
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains("--game"), "{errors:?}");
    assert!(!out.exists());
    let only = dir.path().join("only");
    put(&only, "a.json", &ice_and_fire);
    let built = build(&options(&out, Some("ice-and-fire"), &[&only])).unwrap();
    assert_eq!(built.written, 1);
    assert_eq!(index_ids(&out), [id_of(&ice_and_fire)]);

    let both = dir.path().join("both");
    let built = build(&options(&both, None, &[&input])).unwrap();
    assert_eq!(built.written, 2);
}

#[test]
fn a_missing_or_misplaced_input_is_an_error_and_nothing_is_written() {
    let dir = TempDir::new("archive-missing");
    let a = record(&dir, 1, 2, 1000);
    let input = dir.path().join("in");
    put(&input, "a.json", &a);
    let missing = dir.path().join("missing");
    let file = input.join("a.json");
    let out = dir.path().join("out");

    let errors = build(&options(&out, None, &[&input, &missing, &file])).unwrap_err();
    assert_eq!(errors.len(), 2, "{errors:?}");
    assert!(errors[0].starts_with(&format!("{}: ", missing.display())));
    assert!(errors[1].starts_with(&format!("{}: ", file.display())));
    assert!(!out.exists());
}

#[test]
fn an_output_folder_that_is_not_empty_is_refused() {
    let dir = TempDir::new("archive-out");
    let a = record(&dir, 1, 2, 1000);
    let input = dir.path().join("in");
    put(&input, "a.json", &a);

    let full = dir.path().join("full");
    put_text(&full, "keep.txt", "mine");
    let errors = build(&options(&full, None, &[&input])).unwrap_err();
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(errors[0].contains("not empty"), "{errors:?}");
    assert_eq!(fs::read_dir(&full).unwrap().count(), 1);
    assert_eq!(fs::read_to_string(full.join("keep.txt")).unwrap(), "mine");

    let file: PathBuf = dir.path().join("file");
    fs::write(&file, "x").unwrap();
    assert!(build(&options(&file, None, &[&input])).is_err());

    // An empty folder is fine, and no temporary folder is left next to it.
    let empty = dir.path().join("empty");
    fs::create_dir_all(&empty).unwrap();
    assert_eq!(build(&options(&empty, None, &[&input])).unwrap().written, 1);
    assert!(empty.join("index.json").is_file());
    for entry in fs::read_dir(dir.path()).unwrap() {
        let name = entry.unwrap().file_name().into_string().unwrap();
        assert!(!name.starts_with('.'), "{name} was left behind");
    }
}

#[test]
fn the_output_problem_is_listed_with_the_bad_files() {
    let dir = TempDir::new("archive-all-problems");
    let input = dir.path().join("in");
    put_text(&input, "bad.json", "nope");
    let full = dir.path().join("full");
    put_text(&full, "keep.txt", "mine");
    let errors = build(&options(
        &full,
        None,
        &[&input, &dir.path().join("missing")],
    ))
    .unwrap_err();
    assert_eq!(errors.len(), 3, "{errors:?}");
    assert!(errors[0].contains("not empty"), "{errors:?}");
    assert!(errors[1].contains("bad.json"), "{errors:?}");
    assert!(errors[2].contains("missing"), "{errors:?}");
    assert_eq!(fs::read_dir(&full).unwrap().count(), 1);
}

#[test]
fn an_output_that_cannot_be_created_leaves_nothing_behind() {
    let dir = TempDir::new("archive-no-room");
    let a = record(&dir, 1, 2, 1000);
    let input = dir.path().join("in");
    put(&input, "a.json", &a);
    // A file where the output's parent folder should be.
    let blocker = dir.path().join("blocker");
    fs::write(&blocker, "x").unwrap();
    let errors = build(&options(&blocker.join("out"), None, &[&input])).unwrap_err();
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert!(blocker.is_file());
}

#[test]
fn no_records_give_the_empty_index_and_an_empty_records_folder() {
    let dir = TempDir::new("archive-empty");
    let input = dir.path().join("in");
    fs::create_dir_all(&input).unwrap();
    put_text(&input, "readme.txt", "nothing here");
    let out = dir.path().join("deeper").join("out");

    let built = build(&options(&out, None, &[&input])).unwrap();
    assert_eq!(built.written, 0);
    assert_eq!(built.duplicates, 0);
    assert_eq!(
        fs::read_to_string(out.join("index.json")).unwrap(),
        "{\"format\":1,\"rows\":[]}"
    );
    assert!(out.join("records").is_dir());
    assert!(record_files(&out).is_empty());
}

#[test]
fn the_index_lists_newest_first_and_ties_by_id_last_first() {
    let dir = TempDir::new("archive-index");
    let middle = record(&dir, 1, 2, 2000);
    let oldest = record(&dir, 2, 3, 1000);
    let newest = record(&dir, 3, 4, 3000);
    let tie_a = record(&dir, 4, 5, 2500);
    let tie_b = record(&dir, 5, 6, 2500);
    let input = dir.path().join("in");
    for (name, record) in [
        ("1", &middle),
        ("2", &oldest),
        ("3", &newest),
        ("4", &tie_a),
        ("5", &tie_b),
    ] {
        put(&input, &format!("{name}.json"), record);
    }
    let out = dir.path().join("out");
    build(&options(&out, None, &[&input])).unwrap();

    let mut ties = vec![id_of(&tie_a), id_of(&tie_b)];
    ties.sort();
    ties.reverse();
    let mut expected = vec![id_of(&newest)];
    expected.extend(ties);
    expected.push(id_of(&middle));
    expected.push(id_of(&oldest));
    assert_eq!(index_ids(&out), expected);
}

#[test]
fn files_are_the_compact_records_named_by_id_and_the_index_has_the_documented_shape() {
    let dir = TempDir::new("archive-bytes");
    let resigned = record(&dir, 1, 2, 1000);
    let state = state(&dir.path().join("web"), &dir.path().join("data"));
    let mut finished = finished_record(&state, 9);
    finished["unix_time"] = json!(2000);
    let input = dir.path().join("in");
    put(&input, "resigned.json", &resigned);
    put(&input, "finished.json", &finished);
    let out = dir.path().join("out");
    build(&options(&out, None, &[&input])).unwrap();

    for value in [&resigned, &finished] {
        let parsed = parse_record(&serde_json::to_vec(value).unwrap()).unwrap();
        let id = record_id(&parsed).unwrap();
        let bytes = fs::read(out.join("records").join(format!("{id}.json"))).unwrap();
        assert_eq!(bytes, serde_json::to_vec(&parsed).unwrap());
        assert!(!bytes.contains(&b'\n'));
    }
    let text = fs::read_to_string(out.join("index.json")).unwrap();
    let id = id_of(&finished);
    assert!(
        text.starts_with(&format!(
            "{{\"format\":1,\"rows\":[{{\"id\":\"{id}\",\"game\":\"uttt\",\"unix_time\":2000,\"players\":["
        )),
        "{text}"
    );
    assert!(!text.contains('\n') && !text.contains(": "));
    let index: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(index["rows"][0]["end"], "finished");
    assert_eq!(index["rows"][1]["end"], "resigned");
}

#[test]
fn building_from_an_archive_gives_the_same_archive() {
    let dir = TempDir::new("archive-idempotent");
    let input = dir.path().join("in");
    for (seed, plies) in [(1, 2), (2, 3), (3, 4)] {
        put(
            &input,
            &format!("{seed}.json"),
            &record(&dir, seed, plies, 1000 + seed),
        );
    }
    let first = dir.path().join("first");
    let second = dir.path().join("second");
    build(&options(&first, None, &[&input])).unwrap();
    // The archive's folder is itself an input: `index.json` is skipped and
    // `records/` is searched.
    let built = build(&options(&second, None, &[&first])).unwrap();
    assert_eq!(built.written, 3);
    assert_eq!(built.duplicates, 0);

    assert_eq!(
        fs::read(first.join("index.json")).unwrap(),
        fs::read(second.join("index.json")).unwrap()
    );
    assert_eq!(record_files(&first), record_files(&second));
    for name in record_files(&first) {
        assert_eq!(
            fs::read(first.join("records").join(&name)).unwrap(),
            fs::read(second.join("records").join(&name)).unwrap()
        );
    }
}
