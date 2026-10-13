//! Tests of the analyses: the routes over the real analysis program of
//! Ultimate Tic-Tac-Toe, and the bookkeeping (limit, idle, stop) over
//! programs the tests drive line by line. No test waits for the real idle
//! time: the analyses read a clock the tests move.

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use cg_arena::live::ReplayError;
use cg_arena::record::RecordedAnswer;
use cg_arena::referee::{GameSetup, Referee};
use serde_json::{json, Value};
use studio_game::{GameInfo, HumanMove, StudioGame};
use uttt_studio::Uttt;

use crate::analysis::{Analyses, Clock, ANALYSIS_IDLE, MAX_ANALYSES};
use crate::server::{body_limit, MAX_BODY};
use crate::tests::{call, repo_root, TempDir};
use crate::State;

/// A clock the tests move.
#[derive(Clone)]
struct FakeClock {
    base: Instant,
    elapsed: Arc<Mutex<Duration>>,
}

impl FakeClock {
    fn new() -> FakeClock {
        FakeClock {
            base: Instant::now(),
            elapsed: Arc::new(Mutex::new(Duration::ZERO)),
        }
    }

    fn advance(&self, by: Duration) {
        *self.elapsed.lock().unwrap() += by;
    }

    fn clock(&self) -> Clock {
        let clock = self.clone();
        Arc::new(move || clock.base + *clock.elapsed.lock().unwrap())
    }
}

/// Waits for `condition`, polling every millisecond; the tests wait on state, not on time.
fn until(mut condition: impl FnMut() -> bool) {
    for _ in 0..20_000 {
        if condition() {
            return;
        }
        thread::sleep(Duration::from_millis(1));
    }
    panic!("the condition did not hold in time");
}

fn analyses(clock: &FakeClock) -> Analyses {
    Analyses::new(ANALYSIS_IDLE, clock.clock())
}

fn state_with(dir: &TempDir, games: Vec<Box<dyn StudioGame>>, analyses: Analyses) -> State {
    State::build(
        repo_root(),
        dir.path().join("web"),
        dir.path().join("data"),
        games,
        analyses,
    )
}

fn uttt_state(dir: &TempDir, clock: &FakeClock) -> State {
    state_with(dir, vec![Box::new(Uttt)], analyses(clock))
}

/// A game like Ultimate Tic-Tac-Toe, with no analysis.
struct NoAnalysis;

impl StudioGame for NoAnalysis {
    fn info(&self) -> GameInfo {
        GameInfo {
            id: "plain",
            name: "Plain",
        }
    }

    fn new_referee(&self, setup: &GameSetup) -> Box<dyn Referee> {
        Uttt.new_referee(setup)
    }

    fn opening_turns(&self, setup: &GameSetup) -> usize {
        Uttt.opening_turns(setup)
    }

    fn frames(
        &self,
        setup: &GameSetup,
        turns: &[Vec<RecordedAnswer>],
    ) -> Result<Vec<Value>, ReplayError> {
        Uttt.frames(setup, turns)
    }

    fn human_moves(
        &self,
        setup: &GameSetup,
        turns: &[Vec<RecordedAnswer>],
        seat: usize,
    ) -> Result<Vec<HumanMove>, ReplayError> {
        Uttt.human_moves(setup, turns, seat)
    }
}

/// A program the test drives: each line sent is emitted, and the answer of `emit` comes back.
struct Driver {
    lines: Sender<String>,
    answers: Receiver<bool>,
}

impl Driver {
    /// Emits `line` in the program's thread; what `emit` answered.
    fn send(&self, line: &str) -> bool {
        self.lines.send(line.to_string()).unwrap();
        self.answers.recv().unwrap()
    }
}

/// Starts an analysis whose program emits what the returned driver sends, until `emit` says stop
/// or the driver is dropped.
fn driven(analyses: &Analyses) -> (String, Driver) {
    let (lines, incoming) = channel::<String>();
    let (answer, answers) = channel::<bool>();
    let id = analyses.start(move |emit, _stop| {
        for line in incoming {
            let keep = emit(&line);
            let _ = answer.send(keep);
            if !keep {
                break;
            }
        }
    });
    (id, Driver { lines, answers })
}

const STEP: &str = r#"{"report":{"iterations":1000,"done":false,"stop":null}}"#;
const LAST: &str = r#"{"report":{"iterations":2000,"done":true,"stop":"iterations"}}"#;

fn small(extra: Value) -> Value {
    let mut request =
        json!({"game": "uttt", "moves": ["4 4"], "iterations": 2000, "report_every": 1000});
    request
        .as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    request
}

fn status_of(state: &State, id: &str) -> (u16, Value) {
    call(state, "GET", &format!("/api/analysis/{id}"), &Value::Null)
}

#[test]
fn the_games_list_says_which_games_have_an_analysis() {
    let dir = TempDir::new("analysis-games");
    let clock = FakeClock::new();
    let (status, games) = call(&uttt_state(&dir, &clock), "GET", "/api/games", &Value::Null);
    assert_eq!(status, 200);
    assert_eq!(games[0]["analysis"], true);
    let both = state_with(
        &dir,
        vec![Box::new(Uttt), Box::new(NoAnalysis)],
        analyses(&clock),
    );
    let (_, games) = call(&both, "GET", "/api/games", &Value::Null);
    assert_eq!(games[1]["id"], "plain");
    assert_eq!(games[1]["analysis"], false);
}

#[test]
fn a_game_without_analysis_reports_it_through_the_default() {
    let mut lines = Vec::new();
    NoAnalysis.analyse(
        "{}",
        &mut |line| {
            lines.push(line.to_string());
            true
        },
        &|| false,
    );
    assert_eq!(lines, [r#"{"error":"this game has no analysis"}"#]);
    assert!(!NoAnalysis.has_analysis());
}

#[test]
fn starting_an_analysis_validates_the_body() {
    let dir = TempDir::new("analysis-validation");
    let clock = FakeClock::new();
    let state = state_with(
        &dir,
        vec![Box::new(Uttt), Box::new(NoAnalysis)],
        analyses(&clock),
    );
    let post = |body: &[u8]| {
        let response = crate::handle(&state, "POST", "/api/analysis", body);
        let value: Value = serde_json::from_slice(&response.body).unwrap();
        (
            response.status,
            value["error"].as_str().unwrap().to_string(),
        )
    };
    let (status, error) = post(b"not json");
    assert_eq!(status, 400);
    assert!(error.starts_with("invalid request body: "), "{error}");
    let (status, error) = post(b"");
    assert_eq!(status, 400);
    assert!(error.starts_with("invalid request body: "), "{error}");
    for body in [&b"[]"[..], b"3", b"null", b"\"uttt\""] {
        let (status, error) = post(body);
        assert_eq!(status, 400, "{body:?}");
        assert_eq!(
            error,
            "invalid request body: the request must be a JSON object"
        );
    }
    for body in [
        r#"{"moves":["4 4"]}"#,
        r#"{"game":3,"moves":[]}"#,
        r#"{"game":null}"#,
    ] {
        let (status, error) = post(body.as_bytes());
        assert_eq!(status, 400, "{body}");
        assert_eq!(error, "invalid request body: game must be a string");
    }
    let (status, error) = post(br#"{"game":"chess","moves":[]}"#);
    assert_eq!((status, error.as_str()), (400, r#"unknown game "chess""#));
    let (status, error) = post(br#"{"game":"plain","moves":[]}"#);
    assert_eq!(
        (status, error.as_str()),
        (400, r#"the game "plain" has no analysis"#)
    );
    assert!(state.analyses().ids().is_empty());
    assert_eq!(body_limit("POST", "/api/analysis"), MAX_BODY);
    let (status, _) = call(&state, "PUT", "/api/analysis", &Value::Null);
    assert_eq!(status, 405);
    let (status, _) = call(&state, "POST", "/api/analysis/abc", &Value::Null);
    assert_eq!(status, 405);
}

#[test]
fn an_analysis_runs_to_a_done_report_and_is_kept_while_read() {
    let dir = TempDir::new("analysis-flow");
    let clock = FakeClock::new();
    let state = uttt_state(&dir, &clock);
    let (status, created) = call(&state, "POST", "/api/analysis", &small(json!({})));
    assert_eq!(status, 200, "{created}");
    let id = created["id"].as_str().unwrap().to_string();
    assert_eq!(id.len(), 16);
    until(|| status_of(&state, &id).1["status"] != "running");
    let (status, view) = status_of(&state, &id);
    assert_eq!(status, 200);
    assert_eq!(view["status"], "done");
    assert_eq!(view["error"], Value::Null);
    let report = &view["report"];
    assert_eq!(report["iterations"], 2000);
    assert_eq!(report["done"], true);
    assert_eq!(report["stop"], "iterations");
    assert_eq!(report["to_move"], 1);
    assert_eq!(report["moves"].as_array().unwrap().len(), 8);
    // A done analysis stays while it is read, however long that takes.
    for _ in 0..3 {
        clock.advance(Duration::from_secs(9));
        assert_eq!(status_of(&state, &id).1["status"], "done");
    }
    until(|| state.analyses().threads() == 0);
}

#[test]
fn the_programs_own_errors_come_back_through_get() {
    let dir = TempDir::new("analysis-errors");
    let clock = FakeClock::new();
    let state = uttt_state(&dir, &clock);
    for (request, message) in [
        (
            json!({"game": "uttt", "moves": ["9 9"]}),
            "move 1 (\"9 9\") is not legal",
        ),
        (json!({"game": "uttt", "moves": [], "iterations": 0}), ""),
        (json!({"game": "uttt"}), ""),
        (json!({"game": "uttt", "moves": [], "extra": 1}), ""),
    ] {
        let (status, created) = call(&state, "POST", "/api/analysis", &request);
        assert_eq!(status, 200, "{request}");
        let id = created["id"].as_str().unwrap();
        until(|| status_of(&state, id).1["status"] != "running");
        let (_, view) = status_of(&state, id);
        assert_eq!(view["status"], "error", "{request}");
        assert_eq!(view["report"], Value::Null);
        assert!(view["error"].as_str().unwrap().contains(message), "{view}");
        assert!(!view["error"].as_str().unwrap().is_empty());
    }
}

#[test]
fn unknown_ids_are_404_and_a_second_delete_too() {
    let dir = TempDir::new("analysis-unknown");
    let clock = FakeClock::new();
    let state = uttt_state(&dir, &clock);
    for id in [
        "zzz",
        "0123456789abcdef",
        "a%2Fb",
        "a b",
        "..",
        "%00",
        &"x".repeat(5000),
    ] {
        for method in ["GET", "DELETE"] {
            let (status, body) = call(&state, method, &format!("/api/analysis/{id}"), &Value::Null);
            assert_eq!(status, 404, "{method} {id}");
            assert_eq!(body, json!({"error": "unknown analysis"}));
        }
    }
    let (status, body) = call(&state, "GET", "/api/analysis/a/b", &Value::Null);
    assert_eq!((status, &body["error"]), (404, &json!("unknown route")));

    let (_, created) = call(
        &state,
        "POST",
        "/api/analysis",
        &small(json!({"iterations": 100_000_000, "report_every": 1000})),
    );
    let id = created["id"].as_str().unwrap().to_string();
    let (status, view) = status_of(&state, &id);
    assert_eq!(status, 200);
    assert!(view["status"] == "running" || view["status"] == "done");
    let path = format!("/api/analysis/{id}");
    let (status, body) = call(&state, "DELETE", &path, &Value::Null);
    assert_eq!((status, body), (200, json!({"deleted": true})));
    let (status, body) = call(&state, "DELETE", &path, &Value::Null);
    assert_eq!((status, body), (404, json!({"error": "unknown analysis"})));
    assert_eq!(status_of(&state, &id).0, 404);
    // The search ends at its next report: its thread is gone.
    until(|| state.analyses().threads() == 0);
}

#[test]
fn a_fifth_analysis_stops_and_removes_the_oldest() {
    let dir = TempDir::new("analysis-limit");
    let clock = FakeClock::new();
    let state = uttt_state(&dir, &clock);
    let long = small(json!({"iterations": 100_000_000, "report_every": 1000}));
    let ids: Vec<String> = (0..MAX_ANALYSES + 1)
        .map(|_| {
            let (status, created) = call(&state, "POST", "/api/analysis", &long);
            assert_eq!(status, 200);
            created["id"].as_str().unwrap().to_string()
        })
        .collect();
    assert_eq!(status_of(&state, &ids[0]).0, 404);
    for id in &ids[1..] {
        assert_eq!(status_of(&state, id).0, 200);
    }
    assert_eq!(state.analyses().ids(), ids[1..]);
    // The evicted search ends, the four others go on.
    until(|| state.analyses().threads() == MAX_ANALYSES);
    for id in &ids[1..] {
        let (status, _) = call(
            &state,
            "DELETE",
            &format!("/api/analysis/{id}"),
            &Value::Null,
        );
        assert_eq!(status, 200);
    }
    until(|| state.analyses().threads() == 0);
}

#[test]
fn a_report_is_kept_until_the_next_and_status_follows_the_lines() {
    let clock = FakeClock::new();
    let analyses = analyses(&clock);
    let (id, program) = driven(&analyses);
    // Before the first report: running, nothing reported.
    assert_eq!(
        analyses.read(&id).unwrap(),
        json!({"status": "running", "report": null, "error": null})
    );
    assert!(program.send(STEP));
    let view = analyses.read(&id).unwrap();
    assert_eq!(view["status"], "running");
    assert_eq!(view["report"]["iterations"], 1000);
    assert!(!program.send(LAST));
    let view = analyses.read(&id).unwrap();
    assert_eq!(view["status"], "done");
    assert_eq!(view["report"]["iterations"], 2000);
    assert_eq!(view["error"], Value::Null);
    drop(program);
    until(|| analyses.threads() == 0);
}

#[test]
fn an_error_line_ends_the_analysis_even_as_the_first_line() {
    let clock = FakeClock::new();
    let analyses = analyses(&clock);
    let (id, program) = driven(&analyses);
    assert!(!program.send(r#"{"error":"bad moves"}"#));
    assert_eq!(
        analyses.read(&id).unwrap(),
        json!({"status": "error", "report": null, "error": "bad moves"})
    );
    // An error after reports keeps the last report.
    let (id, program) = driven(&analyses);
    assert!(program.send(STEP));
    assert!(!program.send(r#"{"error":"late"}"#));
    let view = analyses.read(&id).unwrap();
    assert_eq!(
        (&view["status"], &view["error"]),
        (&json!("error"), &json!("late"))
    );
    assert_eq!(view["report"]["iterations"], 1000);
    // Lines that are not the protocol are an error too.
    for line in ["nonsense", "[]", r#"{"other":1}"#, r#"{"report":3}"#] {
        let (id, program) = driven(&analyses);
        assert!(!program.send(line), "{line}");
        let view = analyses.read(&id).unwrap();
        assert_eq!(view["status"], "error", "{line}");
        assert!(
            view["error"].as_str().unwrap().contains("unexpected line"),
            "{line}"
        );
    }
}

#[test]
fn a_program_that_crashes_or_stops_early_is_an_error() {
    let clock = FakeClock::new();
    let analyses = analyses(&clock);
    let id = analyses.start(|emit, _stop| {
        emit(STEP);
        panic!("boom");
    });
    until(|| analyses.read(&id).unwrap()["status"] == "error");
    let view = analyses.read(&id).unwrap();
    assert_eq!(view["error"], "the analysis program crashed");
    assert_eq!(view["report"]["iterations"], 1000);
    let id = analyses.start(|emit, _stop| {
        emit(STEP);
    });
    until(|| analyses.read(&id).unwrap()["status"] == "error");
    assert_eq!(
        analyses.read(&id).unwrap()["error"],
        "the analysis program ended without a final report"
    );
    until(|| analyses.threads() == 0);
}

#[test]
fn a_stopped_analysis_tells_the_program_to_stop() {
    let clock = FakeClock::new();
    let analyses = analyses(&clock);
    let (id, program) = driven(&analyses);
    assert!(program.send(STEP));
    assert!(analyses.remove(&id));
    assert!(!analyses.remove(&id));
    // The program notices at its next report, and its thread ends.
    assert!(!program.send(STEP));
    until(|| analyses.threads() == 0);
    assert!(analyses.read(&id).is_none());
}

#[test]
fn the_oldest_goes_when_a_fifth_starts() {
    let clock = FakeClock::new();
    let analyses = analyses(&clock);
    let programs: Vec<(String, Driver)> = (0..MAX_ANALYSES).map(|_| driven(&analyses)).collect();
    let (fifth, fifth_program) = driven(&analyses);
    let ids = analyses.ids();
    assert_eq!(ids.len(), MAX_ANALYSES);
    assert!(!ids.contains(&programs[0].0));
    assert_eq!(ids[MAX_ANALYSES - 1], fifth);
    // Only the oldest was told to stop.
    assert!(!programs[0].1.send(STEP));
    for (_, program) in &programs[1..] {
        assert!(program.send(STEP));
    }
    assert!(fifth_program.send(STEP));
    assert_eq!(analyses.threads(), MAX_ANALYSES);
}

#[test]
fn an_analysis_nobody_reads_is_removed_and_stopped() {
    let clock = FakeClock::new();
    let analyses = analyses(&clock);
    let (read, reading) = driven(&analyses);
    let (unread, unreading) = driven(&analyses);

    // Reading keeps one alive past ten seconds in all; the other expires.
    clock.advance(Duration::from_secs(6));
    assert!(analyses.read(&read).is_some());
    clock.advance(Duration::from_secs(4));
    // Exactly ten seconds without a read is not yet too long.
    assert!(unreading.send(STEP));
    assert_eq!(analyses.ids().len(), 2);
    clock.advance(Duration::from_millis(1));
    // The thread of the unread one sees it at its next report and ends.
    assert!(!unreading.send(STEP));
    assert_eq!(analyses.ids(), std::slice::from_ref(&read));
    assert!(analyses.read(&unread).is_none());
    assert!(reading.send(STEP));
    until(|| analyses.threads() == 1);

    // Any request sweeps: nothing is left after ten more seconds without a read.
    clock.advance(Duration::from_secs(11));
    assert!(analyses.read(&read).is_none());
    assert!(!reading.send(STEP));
    until(|| analyses.threads() == 0);
}

#[test]
fn an_idle_analysis_is_swept_by_the_next_request_even_without_reports() {
    let dir = TempDir::new("analysis-idle");
    let clock = FakeClock::new();
    let state = uttt_state(&dir, &clock);
    let (_, created) = call(&state, "POST", "/api/analysis", &small(json!({})));
    let id = created["id"].as_str().unwrap().to_string();
    until(|| status_of(&state, &id).1["status"] == "done");
    clock.advance(Duration::from_secs(10));
    assert_eq!(status_of(&state, &id).0, 200);
    clock.advance(Duration::from_secs(10) + Duration::from_millis(1));
    assert_eq!(status_of(&state, &id).0, 404);
    // A DELETE after the time is a 404 as well: it was already removed.
    let (_, created) = call(&state, "POST", "/api/analysis", &small(json!({})));
    let id = created["id"].as_str().unwrap().to_string();
    clock.advance(Duration::from_secs(11));
    let (status, _) = call(
        &state,
        "DELETE",
        &format!("/api/analysis/{id}"),
        &Value::Null,
    );
    assert_eq!(status, 404);
    until(|| state.analyses().threads() == 0);
}

#[test]
fn a_delete_racing_the_last_line_does_not_deadlock() {
    let clock = FakeClock::new();
    let analyses = analyses(&clock);
    for _ in 0..50 {
        let (id, program) = driven(&analyses);
        let remover = {
            let analyses = analyses.clone();
            let id = id.clone();
            thread::spawn(move || analyses.remove(&id))
        };
        program.send(LAST);
        remover.join().unwrap();
        let _ = analyses.read(&id);
        drop(program);
    }
    until(|| analyses.threads() == 0);
}

#[test]
fn a_stopped_search_ends_between_reports() {
    let clock = FakeClock::new();
    let analyses = analyses(&clock);
    // A program that thinks for ever between two reports, looking at `stop` as the search does.
    let id = analyses.start(|emit, stop| {
        emit(STEP);
        while !stop() {
            thread::yield_now();
        }
    });
    until(|| {
        analyses
            .read(&id)
            .is_some_and(|view| view["report"].is_object())
    });
    assert_eq!(analyses.threads(), 1);
    assert!(analyses.remove(&id));
    until(|| analyses.threads() == 0);
}

#[test]
fn an_unread_search_ends_between_reports() {
    let clock = FakeClock::new();
    let analyses = analyses(&clock);
    let id = analyses.start(|_emit, stop| {
        while !stop() {
            thread::yield_now();
        }
    });
    assert_eq!(analyses.ids(), std::slice::from_ref(&id));
    clock.advance(ANALYSIS_IDLE + Duration::from_millis(1));
    // No request, no report: the thread sees it by itself.
    until(|| analyses.threads() == 0);
    assert!(analyses.ids().is_empty());
}

#[test]
fn evicted_and_deleted_searches_end_even_with_huge_reports() {
    let dir = TempDir::new("analysis-huge");
    let clock = FakeClock::new();
    let state = uttt_state(&dir, &clock);
    // One report would come after ten million iterations, minutes away in a debug build.
    let long = small(json!({"iterations": 100_000_000, "report_every": 10_000_000}));
    let ids: Vec<String> = (0..2 * MAX_ANALYSES)
        .map(|_| {
            let (status, created) = call(&state, "POST", "/api/analysis", &long);
            assert_eq!(status, 200);
            created["id"].as_str().unwrap().to_string()
        })
        .collect();
    assert_eq!(state.analyses().ids(), ids[MAX_ANALYSES..]);
    // The four evicted ones end within a step of 1000 iterations: four threads are left.
    until(|| state.analyses().threads() == MAX_ANALYSES);
    for id in &ids[MAX_ANALYSES..] {
        let (status, _) = call(
            &state,
            "DELETE",
            &format!("/api/analysis/{id}"),
            &Value::Null,
        );
        assert_eq!(status, 200);
    }
    until(|| state.analyses().threads() == 0);
}
