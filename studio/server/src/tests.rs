//! Tests of the routes, without sockets and without bots.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use cg_arena::record::{PlayerKind, Record};
use serde_json::{json, Value};

use crate::server::{bind_address, request_allowed, BIND_ADDRESS};
use crate::speed::{iterations, median};
use crate::{handle, State};

/// A folder under the system's temporary one, removed on drop.
pub(crate) struct TempDir(PathBuf);

impl TempDir {
    pub(crate) fn new(name: &str) -> TempDir {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let dir = std::env::temp_dir().join(format!(
            "studio-test-{}-{}-{name}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        TempDir(dir)
    }

    pub(crate) fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

pub(crate) fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

pub(crate) fn state(web: &Path, data: &Path) -> State {
    State::new(repo_root(), web.to_path_buf(), data.to_path_buf())
}

/// A state with no front end and no data yet.
fn plain_state(dir: &TempDir) -> State {
    state(&dir.path().join("web"), &dir.path().join("data"))
}

pub(crate) fn call(state: &State, method: &str, path: &str, body: &Value) -> (u16, Value) {
    let bytes = if body.is_null() {
        Vec::new()
    } else {
        serde_json::to_vec(body).unwrap()
    };
    let response = handle(state, method, path, &bytes);
    let value = serde_json::from_slice(&response.body).unwrap_or(Value::Null);
    (response.status, value)
}

fn friends(state: &State) -> String {
    let (status, created) = call(
        state,
        "POST",
        "/api/sessions",
        &json!({
            "game": "uttt",
            "seed": 123,
            "opening_plies": 0,
            "seats": [
                {"kind": "human", "name": "Ann"},
                {"kind": "human"},
            ],
        }),
    );
    assert_eq!(status, 200, "{created}");
    created["id"].as_str().unwrap().to_string()
}

fn get(state: &State, id: &str) -> Value {
    let (status, session) = call(state, "GET", &format!("/api/sessions/{id}"), &Value::Null);
    assert_eq!(status, 200, "{session}");
    session
}

#[test]
fn the_games_list_has_uttt_and_its_releases_newest_first() {
    let dir = TempDir::new("games");
    let (status, games) = call(&plain_state(&dir), "GET", "/api/games", &Value::Null);
    assert_eq!(status, 200);
    let games = games.as_array().unwrap();
    assert_eq!(games.len(), 1);
    assert_eq!(games[0]["id"], "uttt");
    assert_eq!(games[0]["name"], "Ultimate Tic-Tac-Toe");
    let releases: Vec<&str> = games[0]["releases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|name| name.as_str().unwrap())
        .collect();
    assert!(releases.len() >= 10, "{releases:?}");
    assert_eq!(*releases.last().unwrap(), "uttt-v001");
    let mut sorted = releases.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    assert_eq!(releases, sorted);
    assert!(releases[0] > "uttt-v009");
}

#[test]
fn creating_a_session_validates_the_request() {
    let dir = TempDir::new("validation");
    let state = plain_state(&dir);
    let bot = |release: &str, think_ms: u64| json!({"kind": "bot", "release": release, "think_ms": think_ms, "mode": "fixed"});
    let human = json!({"kind": "human"});
    let cases = [
        (
            json!({"game": "chess", "seed": 1, "seats": [human, human]}),
            "unknown game",
        ),
        (
            json!({"game": "uttt", "seats": [human, human]}),
            "seed is required",
        ),
        (
            json!({"game": "uttt", "seed": 1, "seats": [human]}),
            "exactly 2 seats",
        ),
        (
            json!({"game": "uttt", "seed": 1, "seats": [human, bot("uttt-v999", 100)]}),
            "unknown release",
        ),
        (
            json!({"game": "uttt", "seed": 1, "seats": [human, bot("../uttt-v001", 100)]}),
            "unknown release",
        ),
        (
            json!({"game": "uttt", "seed": 1, "seats": [human, bot("uttt-v001", 9)]}),
            "think_ms",
        ),
        (
            json!({"game": "uttt", "seed": 1, "seats": [human, bot("uttt-v001", 10_001)]}),
            "think_ms",
        ),
        (
            json!({"game": "uttt", "seed": 1, "seats": [human,
                {"kind": "bot", "release": "uttt-v001", "think_ms": 100}]}),
            "mode",
        ),
        (
            json!({"game": "uttt", "seed": 1, "opening_plies": 4, "seats": [human, human]}),
            "opening_plies must be 0",
        ),
        (json!({"game": "uttt"}), "invalid request body"),
    ];
    for (body, expected) in cases {
        let (status, error) = call(&state, "POST", "/api/sessions", &body);
        assert_eq!(status, 400, "{body}");
        let message = error["error"].as_str().unwrap();
        assert!(message.contains(expected), "{message:?} for {body}");
    }
}

#[test]
fn a_friend_game_is_played_by_move_index() {
    let dir = TempDir::new("friends");
    let state = plain_state(&dir);
    let id = friends(&state);

    let session = get(&state, &id);
    assert_eq!(session["status"], "waiting_human");
    assert_eq!(session["to_act"], json!([0]));
    assert_eq!(session["seats"][0]["name"], "Ann");
    assert_eq!(session["seats"][1]["name"], "Player 2");
    assert_eq!(session["frames"].as_array().unwrap().len(), 1);
    assert_eq!(session["human_moves"].as_array().unwrap().len(), 81);
    assert_eq!(session["opening_turns"], 0);
    assert_eq!(session["result"], Value::Null);
    assert_eq!(session["progress"], Value::Null);

    let path = format!("/api/sessions/{id}/move");
    // Not seat 1's turn.
    let (status, _) = call(&state, "POST", &path, &json!({"seat": 1, "index": 0}));
    assert_eq!(status, 409);
    // Not a move.
    let (status, _) = call(&state, "POST", &path, &json!({"seat": 0, "index": 81}));
    assert_eq!(status, 409);

    let (status, session) = call(&state, "POST", &path, &json!({"seat": 0, "index": 3}));
    assert_eq!(status, 200, "{session}");
    assert_eq!(session["to_act"], json!([1]));
    assert_eq!(session["turns"].as_array().unwrap().len(), 1);
    assert_eq!(session["frames"].as_array().unwrap().len(), 2);
    let first_lines = session["turns"][0][0]["lines"].clone();
    let (status, session) = call(&state, "POST", &path, &json!({"seat": 1, "index": 0}));
    assert_eq!(status, 200, "{session}");
    assert_eq!(session["turns"].as_array().unwrap().len(), 2);

    let (status, record) = call(
        &state,
        "GET",
        &format!("/api/sessions/{id}/record"),
        &Value::Null,
    );
    assert_eq!(status, 200);
    let record: Record = serde_json::from_value(record).unwrap();
    assert_eq!(record.format, 1);
    assert_eq!(record.game, "uttt");
    assert_eq!(record.seed, 123);
    assert_eq!(record.source, "studio");
    assert_eq!(record.players[0].name, "Ann");
    assert_eq!(record.players[0].kind, PlayerKind::Human);
    assert_eq!(record.players[1].bot_seed, None);
    assert_eq!(record.turns.len(), 2);
    assert_eq!(json!(record.turns[0][0].lines), first_lines);
    assert_eq!(record.winner, None);

    // Takebacks.
    let takeback = format!("/api/sessions/{id}/takeback");
    let (status, _) = call(&state, "POST", &takeback, &json!({"turns": 3}));
    assert_eq!(status, 409);
    let (status, session) = call(&state, "POST", &takeback, &json!({"turns": 1}));
    assert_eq!(status, 200, "{session}");
    assert_eq!(session["turns"].as_array().unwrap().len(), 1);
    assert_eq!(session["to_act"], json!([1]));
    let (status, session) = call(&state, "POST", &takeback, &json!({"turns": 0}));
    assert_eq!(status, 200, "{session}");
    assert_eq!(session["turns"].as_array().unwrap().len(), 0);
    assert_eq!(session["to_act"], json!([0]));
    assert_eq!(session["status"], "waiting_human");

    // Resigning ends the game for the other seat.
    let end = format!("/api/sessions/{id}/end");
    let (status, _) = call(&state, "POST", &end, &json!({"seat": 0, "reason": "dance"}));
    assert_eq!(status, 400);
    let (status, session) = call(
        &state,
        "POST",
        &end,
        &json!({"seat": 0, "reason": "resign"}),
    );
    assert_eq!(status, 200, "{session}");
    assert_eq!(session["status"], "over");
    assert_eq!(session["result"]["winner"], 1);
    assert_eq!(session["result"]["end"]["kind"], "resigned");
    assert_eq!(session["result"]["end"]["seat"], 0);
    assert_eq!(session["to_act"], json!([]));
    assert_eq!(session["human_moves"], json!([]));
    let (status, _) = call(
        &state,
        "POST",
        &end,
        &json!({"seat": 1, "reason": "resign"}),
    );
    assert_eq!(status, 409);
    let (status, _) = call(&state, "POST", &path, &json!({"seat": 0, "index": 0}));
    assert_eq!(status, 409);
    let (_, record) = call(
        &state,
        "GET",
        &format!("/api/sessions/{id}/record"),
        &Value::Null,
    );
    assert_eq!(record["winner"], 1);

    // A flag falling is a timeout.
    let id = friends(&state);
    let (status, session) = call(
        &state,
        "POST",
        &format!("/api/sessions/{id}/end"),
        &json!({"seat": 1, "reason": "timeout"}),
    );
    assert_eq!(status, 200);
    assert_eq!(session["result"]["winner"], 0);
    assert_eq!(session["result"]["end"]["kind"], "timeout");
    assert_eq!(session["result"]["end"]["seat"], 1);

    // Deleting forgets the session.
    let (status, _) = call(
        &state,
        "DELETE",
        &format!("/api/sessions/{id}"),
        &Value::Null,
    );
    assert_eq!(status, 200);
    let (status, error) = call(&state, "GET", &format!("/api/sessions/{id}"), &Value::Null);
    assert_eq!(status, 404);
    assert!(error["error"].is_string());
}

#[test]
fn a_finished_game_is_over_with_its_result() {
    let dir = TempDir::new("finished");
    let state = plain_state(&dir);
    let id = friends(&state);
    let path = format!("/api/sessions/{id}/move");
    let mut session = get(&state, &id);
    let mut plies = 0;
    while session["status"] == "waiting_human" {
        let seat = session["to_act"][0].as_u64().unwrap();
        let (status, next) = call(&state, "POST", &path, &json!({"seat": seat, "index": 0}));
        assert_eq!(status, 200, "{next}");
        session = next;
        plies += 1;
        assert!(plies <= 81);
    }
    assert_eq!(session["status"], "over");
    assert_eq!(session["result"]["end"]["kind"], "finished");
    assert_eq!(session["to_act"], json!([]));
    assert_eq!(session["turns"].as_array().unwrap().len(), plies);
    // A takeback reopens a finished game.
    let (status, session) = call(
        &state,
        "POST",
        &format!("/api/sessions/{id}/takeback"),
        &json!({"turns": plies - 1}),
    );
    assert_eq!(status, 200, "{session}");
    assert_eq!(session["status"], "waiting_human");
    assert_eq!(session["result"], Value::Null);
}

#[test]
fn unknown_routes_are_json_404s() {
    let dir = TempDir::new("routes");
    let state = plain_state(&dir);
    for (method, path) in [
        ("GET", "/api"),
        ("GET", "/api/"),
        ("GET", "/api/nothing"),
        ("GET", "/api/sessions/zzz"),
        ("GET", "/api/sessions/zzz/record"),
        ("POST", "/api/sessions/zzz/move"),
        ("DELETE", "/api/sessions/zzz"),
    ] {
        let body = if method == "POST" {
            json!({"seat": 0, "index": 0})
        } else {
            Value::Null
        };
        let response = handle(&state, method, path, &serde_json::to_vec(&body).unwrap());
        assert_eq!(response.status, 404, "{method} {path}");
        assert_eq!(response.content_type, "application/json");
        let value: Value = serde_json::from_slice(&response.body).unwrap();
        assert!(value["error"].is_string(), "{method} {path}");
    }
    let (status, _) = call(&state, "PUT", "/api/games", &Value::Null);
    assert_eq!(status, 405);
}

#[test]
fn the_front_end_falls_back_to_index_html() {
    let dir = TempDir::new("web");
    let web = dir.path().join("web");
    fs::create_dir_all(web.join("assets")).unwrap();
    fs::write(web.join("index.html"), "<html>home</html>").unwrap();
    fs::write(web.join("assets/app.js"), "console.log(1)").unwrap();
    let state = state(&web, &dir.path().join("data"));

    let response = handle(&state, "GET", "/assets/app.js", b"");
    assert_eq!(response.status, 200);
    assert!(response.content_type.starts_with("text/javascript"));
    assert_eq!(response.body, b"console.log(1)");
    for path in [
        "/",
        "/index.html",
        "/game/uttt",
        "/assets/missing.js",
        "/../Cargo.toml",
        "/%2e%2e/x",
    ] {
        let response = handle(&state, "GET", path, b"");
        assert_eq!(response.status, 200, "{path}");
        assert_eq!(response.body, b"<html>home</html>", "{path}");
        assert!(response.content_type.starts_with("text/html"), "{path}");
    }
}

#[test]
fn a_missing_web_folder_says_how_to_build_it() {
    let dir = TempDir::new("noweb");
    let response = handle(&plain_state(&dir), "GET", "/", b"");
    assert_eq!(response.status, 200);
    let page = String::from_utf8(response.body).unwrap();
    assert!(page.contains("npm run build"));
    assert!(page.contains("studio/web"));
}

#[test]
fn the_server_binds_to_this_computer_only() {
    assert_eq!(BIND_ADDRESS, "127.0.0.1");
    assert_eq!(bind_address(8411), "127.0.0.1:8411");
}

#[test]
fn requests_must_come_from_the_studios_own_page() {
    assert!(request_allowed(8411, Some("127.0.0.1:8411"), None));
    assert!(request_allowed(
        8411,
        Some("localhost:8411"),
        Some("http://localhost:8411")
    ));
    assert!(!request_allowed(8411, Some("evil.example:8411"), None));
    assert!(!request_allowed(8411, None, None));
    assert!(!request_allowed(
        8411,
        Some("127.0.0.1:8411"),
        Some("http://evil.example")
    ));
}

#[test]
fn a_think_time_becomes_iterations() {
    assert_eq!(iterations(100, 12.5), 1250);
    assert_eq!(iterations(10, 0.0001), 1);
    assert_eq!(median(&mut [3.0, 1.0, 2.0]), 2.0);
    assert_eq!(median(&mut [4.0, 1.0, 2.0, 3.0]), 2.5);
    assert_eq!(median(&mut []), 0.0);
}
