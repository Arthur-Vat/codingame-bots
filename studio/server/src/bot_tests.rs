//! Tests with real bots: `uttt-v001` is compiled once for the whole test
//! binary, into a folder shared by the tests.

use std::sync::OnceLock;
use std::thread::sleep;
use std::time::{Duration, Instant};

use serde_json::{json, Value};

use crate::tests::{call, repo_root, state, TempDir};
use crate::State;

const RELEASE: &str = "uttt-v001";

/// One state for all the tests, so that the release is compiled and
/// measured once. Its data folder is leaked on purpose: it must outlive
/// every test of the binary.
fn bot_state() -> State {
    static STATE: OnceLock<State> = OnceLock::new();
    STATE
        .get_or_init(|| {
            let data = TempDir::new("bots");
            let state = state(&repo_root().join("no-web"), data.path());
            std::mem::forget(data);
            state
        })
        .clone()
}

/// Polls the session until `done` holds or the deadline passes.
fn wait_for(state: &State, id: &str, what: &str, done: impl Fn(&Value) -> bool) -> Value {
    let deadline = Instant::now() + Duration::from_secs(180);
    loop {
        let (status, session) = call(state, "GET", &format!("/api/sessions/{id}"), &Value::Null);
        assert_eq!(status, 200, "{session}");
        assert_ne!(session["status"], "failed", "{session}");
        if done(&session) {
            return session;
        }
        assert!(
            Instant::now() < deadline,
            "timeout waiting for {what}: {session}"
        );
        sleep(Duration::from_millis(20));
    }
}

fn turns(session: &Value) -> usize {
    session["turns"].as_array().unwrap().len()
}

fn create(state: &State, seats: Value) -> String {
    let (status, created) = call(
        state,
        "POST",
        "/api/sessions",
        &json!({"game": "uttt", "seed": 7, "seats": seats}),
    );
    assert_eq!(status, 200, "{created}");
    created["id"].as_str().unwrap().to_string()
}

/// Plays the human move that answers `lines`.
fn play_lines(state: &State, id: &str, session: &Value, lines: &Value) -> Value {
    let index = session["human_moves"]
        .as_array()
        .unwrap()
        .iter()
        .position(|candidate| &candidate["lines"] == lines)
        .expect("the move is legal");
    let (status, next) = call(
        state,
        "POST",
        &format!("/api/sessions/{id}/move"),
        &json!({"seat": 0, "index": index}),
    );
    assert_eq!(status, 200, "{next}");
    next
}

#[test]
fn a_computer_game_answers_and_replays_after_a_takeback() {
    let started = Instant::now();
    let state = bot_state();
    let id = create(
        &state,
        json!([
            {"kind": "human", "name": "Me"},
            {"kind": "bot", "release": RELEASE, "think_ms": 10, "mode": "fixed"},
        ]),
    );
    let session = wait_for(&state, &id, "the bot to be ready", |s| {
        s["status"] == "waiting_human"
    });
    assert!(session["seats"][1]["fixed_iters"].as_u64().unwrap() >= 1);
    eprintln!("bot ready after {:?}", started.elapsed());

    // Two moves of mine, two answers of the bot.
    let mut session = session;
    for expected in [2, 4] {
        let lines = session["human_moves"][0]["lines"].clone();
        play_lines(&state, &id, &session, &lines);
        session = wait_for(&state, &id, "the bot's answer", |s| {
            turns(s) == expected && s["status"] == "waiting_human"
        });
    }
    let my_second_move = session["turns"][2][0]["lines"].clone();
    let bots_second_answer = session["turns"][3][0]["lines"].clone();

    // A takeback of two turns restarts the bot and replays it.
    let (status, back) = call(
        &state,
        "POST",
        &format!("/api/sessions/{id}/takeback"),
        &json!({"turns": 2}),
    );
    assert_eq!(status, 200, "{back}");
    assert_eq!(back["status"], "rewinding");
    assert_eq!(back["progress"]["total"], 1);
    let session = wait_for(&state, &id, "the replay", |s| {
        s["status"] == "waiting_human"
    });
    assert_eq!(turns(&session), 2);
    assert_eq!(session["progress"], Value::Null);

    // The same move gets the same answer.
    play_lines(&state, &id, &session, &my_second_move);
    let session = wait_for(&state, &id, "the same answer", |s| {
        turns(s) == 4 && s["status"] == "waiting_human"
    });
    assert_eq!(session["turns"][3][0]["lines"], bots_second_answer);

    // The record names the bot and its settings.
    let (status, record) = call(
        &state,
        "GET",
        &format!("/api/sessions/{id}/record"),
        &Value::Null,
    );
    assert_eq!(status, 200);
    assert_eq!(record["players"][1]["kind"], "bot");
    assert_eq!(record["players"][1]["command"], RELEASE);
    assert_eq!(record["players"][1]["time_scale"], 1.0);
    assert!(record["players"][1]["fixed_iters"].as_u64().unwrap() >= 1);
    assert!(record["players"][1]["bot_seed"].is_u64());
    assert_eq!(record["players"][0]["kind"], "human");
    assert_eq!(record["turns"].as_array().unwrap().len(), 4);

    call(
        &state,
        "DELETE",
        &format!("/api/sessions/{id}"),
        &Value::Null,
    );
    eprintln!("computer game took {:?}", started.elapsed());
}

#[test]
fn a_bot_against_a_bot_plays_to_the_end() {
    let started = Instant::now();
    let state = bot_state();
    let bot = json!({"kind": "bot", "release": RELEASE, "think_ms": 20, "mode": "realtime"});
    let id = create(&state, json!([bot, bot]));
    let session = wait_for(&state, &id, "the end of the game", |s| {
        s["status"] == "over"
    });
    assert!(session["result"].is_object(), "{session}");
    assert!(turns(&session) >= 5);
    assert_eq!(session["to_act"], json!([]));
    let (_, record) = call(
        &state,
        "GET",
        &format!("/api/sessions/{id}/record"),
        &Value::Null,
    );
    assert_eq!(record["players"][0]["time_scale"], 0.2);
    assert_eq!(record["players"][0]["fixed_iters"], Value::Null);
    assert_eq!(record["end"], session["result"]["end"]);

    // No takeback against bots in real time.
    let (status, _) = call(
        &state,
        "POST",
        &format!("/api/sessions/{id}/takeback"),
        &json!({"turns": 0}),
    );
    assert_eq!(status, 409);
    eprintln!("bot against bot took {:?}", started.elapsed());
}
