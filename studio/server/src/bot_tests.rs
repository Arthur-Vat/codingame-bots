//! Tests with real bots: `uttt-v001` and the baseline bots are compiled once
//! for the whole test binary, into a folder shared by the tests.

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
    create_game(state, "uttt", seats)
}

fn create_game(state: &State, game: &str, seats: Value) -> String {
    let (status, created) = call(
        state,
        "POST",
        "/api/sessions",
        &json!({"game": game, "seed": 7, "seats": seats}),
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
    a_computer_game_replays_after_a_takeback(RELEASE);
}

#[test]
fn a_baseline_bot_answers_and_replays_after_a_takeback() {
    a_computer_game_replays_after_a_takeback("uttt-greedy");
}

fn a_computer_game_replays_after_a_takeback(bot: &str) {
    let started = Instant::now();
    let state = bot_state();
    let id = create(
        &state,
        json!([
            {"kind": "human", "name": "Me"},
            {"kind": "bot", "release": bot, "think_ms": 10, "mode": "fixed"},
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
    assert_eq!(record["players"][1]["command"], bot);
    assert_eq!(record["players"][1]["time_scale"], 1.0);
    assert!(record["players"][1]["fixed_iters"].as_u64().unwrap() >= 1);
    assert!(record["players"][1]["bot_seed"].is_u64());
    assert_eq!(record["players"][0]["kind"], "human");
    assert_eq!(record["turns"].as_array().unwrap().len(), 4);

    // Only a human can resign or run out of time.
    let (status, _) = call(
        &state,
        "POST",
        &format!("/api/sessions/{id}/end"),
        &json!({"seat": 1, "reason": "resign"}),
    );
    assert_eq!(status, 409);

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
    let bot = json!({"kind": "bot", "release": RELEASE, "think_ms": 100, "mode": "realtime"});
    let id = create(&state, json!([bot, bot]));
    let session = wait_for(&state, &id, "the end of the game", |s| {
        s["status"] == "over"
    });
    assert!(session["result"].is_object(), "{session}");
    assert_eq!(session["to_act"], json!([]));
    let (_, record) = call(
        &state,
        "GET",
        &format!("/api/sessions/{id}/record"),
        &Value::Null,
    );
    assert_eq!(record["players"][0]["time_scale"], 1.0);
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

/// The turns of a game of `bot` against itself in "fixed" mode, played to
/// the end: the lines of every answer.
fn fixed_game(state: &State, game: &str, bot: &str) -> Value {
    let seat = json!({"kind": "bot", "release": bot, "think_ms": 10, "mode": "fixed"});
    let id = create_game(state, game, json!([seat, seat]));
    let session = wait_for(state, &id, "the end of the game", |s| s["status"] == "over");
    assert!(session["result"].is_object(), "{session}");
    let turns = session["turns"].clone();
    let lines: Vec<Value> = turns
        .as_array()
        .unwrap()
        .iter()
        .map(|turn| {
            let answers = turn.as_array().unwrap();
            json!(answers
                .iter()
                .map(|answer| answer["lines"].clone())
                .collect::<Vec<_>>())
        })
        .collect();
    assert!(!lines.is_empty(), "{session}");
    call(
        state,
        "DELETE",
        &format!("/api/sessions/{id}"),
        &Value::Null,
    );
    json!({"lines": lines, "winner": session["result"]["winner"], "end": session["result"]["end"]})
}

/// Every baseline bot of `game` plays the same game twice from its seed
/// (ADR 0040), as `scripts/check-determinism.sh` checks the releases.
fn baselines_replay_identically(game: &str) {
    let state = bot_state();
    let baselines = state.game(game).map(studio_game::baseline_names).unwrap();
    assert!(!baselines.is_empty());
    for bot in baselines {
        let first = fixed_game(&state, game, &bot);
        let second = fixed_game(&state, game, &bot);
        assert_eq!(first, second, "{bot} does not replay identically");
    }
}

#[test]
fn the_baseline_bots_of_uttt_replay_identically() {
    baselines_replay_identically("uttt");
}

#[test]
fn the_baseline_bots_of_ice_and_fire_replay_identically() {
    baselines_replay_identically("ice-and-fire");
}

#[test]
fn a_game_without_release_plays_baseline_bots_to_the_end() {
    let state = bot_state();
    let bronze = json!({"kind": "bot", "release": "ice-and-fire-bronze", "think_ms": 100, "mode": "realtime"});
    let random = json!({"kind": "bot", "release": "ice-and-fire-random", "think_ms": 100, "mode": "realtime"});
    let id = create_game(&state, "ice-and-fire", json!([bronze, random]));
    let session = wait_for(&state, &id, "the end of the game", |s| {
        s["status"] == "over"
    });
    assert!(session["result"].is_object(), "{session}");
    let (_, record) = call(
        &state,
        "GET",
        &format!("/api/sessions/{id}/record"),
        &Value::Null,
    );
    assert_eq!(record["game"], "ice-and-fire");
    assert_eq!(record["players"][0]["command"], "ice-and-fire-bronze");
    assert_eq!(record["players"][1]["command"], "ice-and-fire-random");
}

#[test]
fn a_name_that_is_not_a_bot_of_the_game_is_refused_before_any_path_is_made() {
    let state = bot_state();
    for name in [
        "../uttt-v001",
        "uttt-greedy/../../x",
        "ice-and-fire-bronze",
        "greedy",
    ] {
        let bot = json!({"kind": "bot", "release": name, "think_ms": 10, "mode": "fixed"});
        let (status, error) = call(
            &state,
            "POST",
            "/api/sessions",
            &json!({"game": "uttt", "seed": 7, "seats": [{"kind": "human"}, bot]}),
        );
        assert_eq!(status, 400, "{name}");
        assert_eq!(error["error"], format!("unknown release {name:?}"));
    }
}
