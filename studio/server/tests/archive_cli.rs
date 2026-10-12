//! Tests of the `studio-archive` program itself: arguments, exit status and
//! what it prints.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::{json, Value};
use studio::{handle, State};

/// A folder under the system's temporary one, removed on drop.
struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> TempDir {
        let dir = std::env::temp_dir().join(format!("studio-cli-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        TempDir(dir)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn run(args: &[&Path]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_studio-archive"))
        .args(args)
        .output()
        .unwrap()
}

fn call(state: &State, method: &str, path: &str, body: &Value) -> Value {
    let bytes = if body.is_null() {
        Vec::new()
    } else {
        serde_json::to_vec(body).unwrap()
    };
    let response = handle(state, method, path, &bytes);
    assert_eq!(response.status, 200, "{path}");
    serde_json::from_slice(&response.body).unwrap_or(Value::Null)
}

/// A game of two moves that seat 1 then resigned, as the studio exports it.
fn record(dir: &Path) -> Vec<u8> {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let state = State::new(
        repo.canonicalize().unwrap(),
        dir.join("web"),
        dir.join("data"),
    );
    let seats = json!([{"kind": "human", "name": "Ann"}, {"kind": "human", "name": "Bob"}]);
    let created = call(
        &state,
        "POST",
        "/api/sessions",
        &json!({"game": "uttt", "seed": 5, "seats": seats}),
    );
    let id = created["id"].as_str().unwrap();
    for ply in 0..2 {
        let body = json!({"seat": ply, "index": 0});
        call(&state, "POST", &format!("/api/sessions/{id}/move"), &body);
    }
    let end = json!({"seat": 1, "reason": "resign"});
    call(&state, "POST", &format!("/api/sessions/{id}/end"), &end);
    let record = call(
        &state,
        "GET",
        &format!("/api/sessions/{id}/record"),
        &Value::Null,
    );
    serde_json::to_vec(&record).unwrap()
}

const OUT: &str = "--out";

#[test]
fn success_prints_one_line_per_input_and_the_total() {
    let dir = TempDir::new("success");
    let (one, two) = (dir.0.join("one"), dir.0.join("two"));
    fs::create_dir_all(&one).unwrap();
    fs::create_dir_all(&two).unwrap();
    let bytes = record(&dir.0);
    fs::write(one.join("a.json"), &bytes).unwrap();
    fs::write(two.join("a.json"), &bytes).unwrap();
    let out = dir.0.join("out");

    let output = run(&[Path::new(OUT), &out, &one, &two]);
    assert_eq!(output.status.code(), Some(0));
    let expected = format!(
        "{}: 1 records, 1 new\n{}: 1 records, 0 new\n1 records written to {} (1 duplicates skipped)\n",
        one.display(),
        two.display(),
        out.display()
    );
    assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
    assert!(output.stderr.is_empty());
    assert!(out.join("index.json").is_file());
}

#[test]
fn failure_exits_1_names_the_file_and_writes_nothing() {
    let dir = TempDir::new("failure");
    let input = dir.0.join("in");
    fs::create_dir_all(&input).unwrap();
    fs::write(input.join("bad.json"), "{}").unwrap();
    let out = dir.0.join("out");

    let output = run(&[Path::new(OUT), &out, &input]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.starts_with(&format!("{}: ", input.join("bad.json").display())),
        "{stderr}"
    );
    assert!(!out.exists());
}

#[test]
fn bad_arguments_exit_2() {
    let dir = TempDir::new("arguments");
    let out = dir.0.join("out");
    assert_eq!(run(&[Path::new(OUT), &out]).status.code(), Some(2));
    assert_eq!(run(&[]).status.code(), Some(2));
}
