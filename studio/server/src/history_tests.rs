//! Tests of the saved-games routes, without sockets and without bots.

use std::fs;
use std::path::PathBuf;

use serde_json::{json, Value};

use crate::tests::{call, state, TempDir};
use crate::State;

/// 2024-03-01 00:00:00 UTC.
const MARCH_1: u64 = 1_709_251_200;
const DAY: u64 = 86_400;

fn new_state(dir: &TempDir) -> State {
    state(&dir.path().join("web"), &dir.path().join("data"))
}

fn history_dir(dir: &TempDir) -> PathBuf {
    dir.path().join("data/history")
}

/// The record of a friend game of `plies` moves (always the first legal
/// move) that seat 1 then resigned.
fn friend_record(state: &State, seed: u64, plies: usize) -> Value {
    let (status, created) = call(
        state,
        "POST",
        "/api/sessions",
        &json!({"game": "uttt", "seed": seed, "seats": [
            {"kind": "human", "name": "Ann"}, {"kind": "human", "name": "Bob"}]}),
    );
    assert_eq!(status, 200, "{created}");
    let id = created["id"].as_str().unwrap();
    for ply in 0..plies {
        let (status, body) = call(
            state,
            "POST",
            &format!("/api/sessions/{id}/move"),
            &json!({"seat": ply % 2, "index": 0}),
        );
        assert_eq!(status, 200, "{body}");
    }
    let (status, body) = call(
        state,
        "POST",
        &format!("/api/sessions/{id}/end"),
        &json!({"seat": 1, "reason": "resign"}),
    );
    assert_eq!(status, 200, "{body}");
    let (status, record) = call(
        state,
        "GET",
        &format!("/api/sessions/{id}/record"),
        &Value::Null,
    );
    assert_eq!(status, 200);
    record
}

fn save(state: &State, record: &Value) -> (u16, Value) {
    call(state, "POST", "/api/history", record)
}

/// A record changed to look like another game: time, source and who played.
fn variant(record: &Value, unix_time: u64, source: &str, players: [&str; 2]) -> Value {
    let mut record = record.clone();
    record["unix_time"] = json!(unix_time);
    record["source"] = json!(source);
    for (seat, name) in players.iter().enumerate() {
        record["players"][seat]["name"] = json!(name);
    }
    record
}

fn ids(state: &State, query: &str) -> Vec<String> {
    let (status, list) = call(state, "GET", &format!("/api/history{query}"), &Value::Null);
    assert_eq!(status, 200, "{list}");
    list.as_array()
        .unwrap()
        .iter()
        .map(|item| item["id"].as_str().unwrap().to_string())
        .collect()
}

fn files(dir: &TempDir) -> usize {
    fs::read_dir(history_dir(dir)).map_or(0, |entries| entries.count())
}

#[test]
fn a_saved_game_is_listed_viewed_exported_and_deleted() {
    let dir = TempDir::new("history-basic");
    let state = new_state(&dir);
    assert!(ids(&state, "").is_empty());

    let record = friend_record(&state, 7, 3);
    let (status, saved) = save(&state, &record);
    assert_eq!(status, 200, "{saved}");
    assert_eq!(saved["duplicate"], false);
    let id = saved["id"].as_str().unwrap().to_string();
    let unix_time = record["unix_time"].as_u64().unwrap();
    let prefix = format!("uttt-{unix_time}-");
    assert!(id.starts_with(&prefix), "{id}");
    assert_eq!(id.len(), prefix.len() + 12);
    assert!(history_dir(&dir).join(format!("{id}.json")).is_file());

    let (_, list) = call(&state, "GET", "/api/history", &Value::Null);
    assert_eq!(
        list,
        json!([{
            "id": id, "game": "uttt", "unix_time": unix_time,
            "players": ["Ann", "Bob"], "winner": 0, "end": "resigned",
            "turns": 3, "source": "studio",
        }])
    );

    let (status, exported) = call(&state, "GET", &format!("/api/history/{id}"), &Value::Null);
    assert_eq!(status, 200);
    assert_eq!(exported, record);

    let (status, view) = call(
        &state,
        "GET",
        &format!("/api/history/{id}/view"),
        &Value::Null,
    );
    assert_eq!(status, 200, "{view}");
    let (status, unsaved) = call(&state, "POST", "/api/view", &record);
    assert_eq!(status, 200, "{unsaved}");
    assert_eq!(view, unsaved);
    assert_eq!(view["record"], record);
    assert_eq!(view["frames"].as_array().unwrap().len(), 4);
    assert_eq!(view["opening_turns"], 0);
    assert_eq!(view["shown_turns"], 3);
    // Viewing does not save.
    assert_eq!(files(&dir), 1);

    let path = format!("/api/history/{id}");
    let (status, _) = call(&state, "DELETE", &path, &Value::Null);
    assert_eq!(status, 204);
    assert!(ids(&state, "").is_empty());
    let (status, _) = call(&state, "DELETE", &path, &Value::Null);
    assert_eq!(status, 404);
    let (status, _) = call(&state, "GET", &path, &Value::Null);
    assert_eq!(status, 404);
}

#[test]
fn the_same_record_is_saved_once() {
    let dir = TempDir::new("history-duplicate");
    let state = new_state(&dir);
    let record = friend_record(&state, 1, 2);
    let (_, first) = save(&state, &record);
    let (status, second) = save(&state, &record);
    assert_eq!(status, 200);
    assert_eq!(first["duplicate"], false);
    assert_eq!(second["duplicate"], true);
    assert_eq!(first["id"], second["id"]);
    assert_eq!(files(&dir), 1);
    // Another game is not a duplicate.
    let other = variant(&friend_record(&state, 2, 2), 1, "studio", ["Ann", "Bob"]);
    let (_, third) = save(&state, &other);
    assert_eq!(third["duplicate"], false);
    assert_eq!(files(&dir), 2);
}

#[test]
fn a_session_saves_its_own_record() {
    let dir = TempDir::new("history-session");
    let state = new_state(&dir);
    let (_, created) = call(
        &state,
        "POST",
        "/api/sessions",
        &json!({"game": "uttt", "seed": 5, "seats": [{"kind": "human"}, {"kind": "human"}]}),
    );
    let id = created["id"].as_str().unwrap();
    let (status, saved) = call(
        &state,
        "POST",
        &format!("/api/sessions/{id}/save"),
        &Value::Null,
    );
    assert_eq!(status, 200, "{saved}");
    assert_eq!(saved["duplicate"], false);
    let (_, list) = call(&state, "GET", "/api/history", &Value::Null);
    assert_eq!(list[0]["id"], saved["id"]);
    assert_eq!(list[0]["end"], "aborted");
    let (status, _) = call(&state, "POST", "/api/sessions/nope/save", &Value::Null);
    assert_eq!(status, 404);
}

#[test]
fn the_listing_filters_and_sorts() {
    let dir = TempDir::new("history-filters");
    let state = new_state(&dir);
    let base = friend_record(&state, 3, 3);

    // Seat 0 won, an arena match, at noon on March 1st.
    let mut x_win = variant(
        &base,
        MARCH_1 + DAY / 2,
        "arena match",
        ["uttt-v010", "uttt-v009"],
    );
    x_win["end"] = json!({"kind": "finished"});
    // Seat 1 won, studio, on the last second of March 1st.
    let mut o_win = variant(&base, MARCH_1 + DAY - 1, "studio", ["Ann", "uttt-v010"]);
    o_win["end"] = json!({"kind": "finished"});
    o_win["winner"] = json!(1);
    // A draw, studio, on the first second of March 2nd.
    let mut draw = variant(&base, MARCH_1 + DAY, "studio", ["Ann", "Bob"]);
    draw["end"] = json!({"kind": "finished"});
    draw["winner"] = Value::Null;
    // Seat 1 timed out, an arena run, on the last second of February 29th.
    let mut fault = variant(&base, MARCH_1 - 1, "arena sprt", ["uttt-v008", "uttt-v009"]);
    fault["end"] = json!({"kind": "timeout", "seat": 1, "limit_ms": 100.0});
    // An aborted game counts as a draw.
    let mut aborted = variant(&base, MARCH_1 + 5, "studio", ["Ann", "Bob"]);
    aborted["end"] = json!({"kind": "aborted", "reason": "unfinished"});
    aborted["winner"] = Value::Null;

    let mut saved = Vec::new();
    for record in [&x_win, &o_win, &draw, &fault, &aborted] {
        let (status, body) = save(&state, record);
        assert_eq!(status, 200, "{body}");
        saved.push(body["id"].as_str().unwrap().to_string());
    }
    let [x_win, o_win, draw, fault, aborted] = <[String; 5]>::try_from(saved).unwrap();
    let listed = |query: &str, expected: &[&String]| {
        let expected: Vec<String> = expected.iter().map(|id| (*id).clone()).collect();
        assert_eq!(ids(&state, query), expected, "{query}");
    };

    // Newest first.
    listed("", &[&draw, &o_win, &x_win, &aborted, &fault]);
    listed("?game=uttt", &[&draw, &o_win, &x_win, &aborted, &fault]);
    listed("?game=chess", &[]);
    // Empty values are not filters.
    listed("?game=&result=", &[&draw, &o_win, &x_win, &aborted, &fault]);

    listed("?release=uttt-v010", &[&o_win, &x_win]);
    listed("?release=uttt-v009", &[&x_win, &fault]);
    listed("?release=uttt-v001", &[]);

    listed("?result=x", &[&x_win, &fault]);
    listed("?result=o", &[&o_win]);
    listed("?result=draw", &[&draw, &aborted]);
    listed("?result=fault", &[&fault]);

    listed("?source=studio", &[&draw, &o_win, &aborted]);
    listed("?source=arena", &[&x_win, &fault]);

    // Dates are days in UTC, both ends included.
    listed("?from=2024-03-01", &[&draw, &o_win, &x_win, &aborted]);
    listed("?from=2024-03-02", &[&draw]);
    listed("?from=2024-03-03", &[]);
    listed("?to=2024-03-01", &[&o_win, &x_win, &aborted, &fault]);
    listed("?to=2024-02-29", &[&fault]);
    listed("?to=2024-02-28", &[]);
    listed(
        "?from=2024-03-01&to=2024-03-01",
        &[&o_win, &x_win, &aborted],
    );

    // Filters combine.
    listed("?release=uttt-v009&result=fault&source=arena", &[&fault]);
    listed("?release=uttt-v010&result=draw", &[]);

    for query in [
        "?result=won",
        "?source=web",
        "?from=yesterday",
        "?from=2024-3-1",
        "?to=2024-02-30",
        "?to=2023-02-29",
        "?from=2024-13-01",
    ] {
        let (status, error) = call(&state, "GET", &format!("/api/history{query}"), &Value::Null);
        assert_eq!(status, 400, "{query}: {error}");
    }
}

#[test]
fn a_record_that_does_not_replay_is_refused() {
    let dir = TempDir::new("history-invalid");
    let state = new_state(&dir);
    let record = friend_record(&state, 4, 4);

    let mut format_2 = record.clone();
    format_2["format"] = json!(2);
    let (status, error) = save(&state, &format_2);
    assert_eq!(status, 400);
    assert!(
        error["error"].as_str().unwrap().contains("format"),
        "{error}"
    );

    let mut unknown_game = record.clone();
    unknown_game["game"] = json!("chess");
    let (status, error) = save(&state, &unknown_game);
    assert_eq!(status, 400);
    assert!(
        error["error"].as_str().unwrap().contains("chess"),
        "{error}"
    );

    let mut three_players = record.clone();
    let player = three_players["players"][0].clone();
    three_players["players"]
        .as_array_mut()
        .unwrap()
        .push(player);
    assert_eq!(save(&state, &three_players).0, 400);

    let mut bad_winner = record.clone();
    bad_winner["winner"] = json!(2);
    assert_eq!(save(&state, &bad_winner).0, 400);

    assert_eq!(
        call(&state, "POST", "/api/history", &json!({"format": 1})).0,
        400
    );
    assert_eq!(save(&state, &Value::Null).0, 400);

    // A bad turn in the middle names the turn.
    let mut middle = record.clone();
    middle["turns"][2][0]["lines"] = json!(["not a move"]);
    for (status, error) in [
        save(&state, &middle),
        call(&state, "POST", "/api/view", &middle),
    ] {
        assert_eq!(status, 400);
        assert!(
            error["error"].as_str().unwrap().contains("turn 2"),
            "{error}"
        );
    }

    // A bad last turn is only forgiven when the game ended by an invalid
    // answer.
    let mut last = record.clone();
    last["turns"].as_array_mut().unwrap().push(json!([
        {"seat": 0, "lines": ["not a move"], "ms": 1.0}
    ]));
    let (status, error) = save(&state, &last);
    assert_eq!(status, 400);
    assert!(
        error["error"].as_str().unwrap().contains("turn 4"),
        "{error}"
    );

    assert_eq!(files(&dir), 0);
}

#[test]
fn an_arena_game_ended_by_an_invalid_last_turn_is_accepted() {
    let dir = TempDir::new("history-arena-invalid");
    let state = new_state(&dir);
    let mut record = variant(
        &friend_record(&state, 6, 4),
        MARCH_1,
        "arena match",
        ["uttt-v010", "uttt-v009"],
    );
    // Seat 0 acts after four turns, and answers nonsense.
    record["turns"].as_array_mut().unwrap().push(json!([
        {"seat": 0, "lines": ["not a move"], "ms": 1.0}
    ]));
    record["end"] = json!({"kind": "invalid", "seat": 0, "reason": "not a move"});
    record["winner"] = json!(1);

    let (status, saved) = save(&state, &record);
    assert_eq!(status, 200, "{saved}");
    let id = saved["id"].as_str().unwrap();
    let (status, view) = call(
        &state,
        "GET",
        &format!("/api/history/{id}/view"),
        &Value::Null,
    );
    assert_eq!(status, 200, "{view}");
    assert_eq!(view["record"]["turns"].as_array().unwrap().len(), 5);
    assert_eq!(view["shown_turns"], 4);
    assert_eq!(view["frames"].as_array().unwrap().len(), 5);
    let (_, list) = call(&state, "GET", "/api/history?result=fault", &Value::Null);
    assert_eq!(list[0]["end"], "invalid");
    assert_eq!(list[0]["turns"], 5);

    // An invalid answer in the middle is still refused.
    let mut middle = record.clone();
    middle["turns"][1][0]["lines"] = json!(["not a move"]);
    assert_eq!(save(&state, &middle).0, 400);
}

#[test]
fn ids_cannot_leave_the_history_folder() {
    let dir = TempDir::new("history-ids");
    let state = new_state(&dir);
    let record = friend_record(&state, 8, 1);
    save(&state, &record);
    fs::write(dir.path().join("data/secret.json"), "{}").unwrap();
    for id in ["../secret", "..%2Fsecret", "..", "A", "a_b", "a.b", "x%00"] {
        for (method, suffix) in [("GET", ""), ("DELETE", ""), ("GET", "/view")] {
            let (status, _) = call(
                &state,
                method,
                &format!("/api/history/{id}{suffix}"),
                &Value::Null,
            );
            assert!(status == 400 || status == 404, "{method} {id}: {status}");
        }
    }
    assert!(dir.path().join("data/secret.json").is_file());
    assert_eq!(files(&dir), 1);
    // Spelled as one segment, the id is refused by its characters.
    let (status, _) = call(&state, "GET", "/api/history/a.b", &Value::Null);
    assert_eq!(status, 400);
}

#[test]
fn unreadable_files_are_skipped_by_the_listing() {
    let dir = TempDir::new("history-corrupt");
    let state = new_state(&dir);
    let record = friend_record(&state, 9, 2);
    let (_, saved) = save(&state, &record);
    let history = history_dir(&dir);
    fs::write(history.join("uttt-1-aaaaaaaaaaaa.json"), "{ not json").unwrap();
    fs::write(history.join("uttt-2-bbbbbbbbbbbb.json"), "{\"format\": 1}").unwrap();
    fs::write(history.join("notes.txt"), "hello").unwrap();
    fs::write(history.join("Not An Id.json"), "{}").unwrap();
    assert_eq!(ids(&state, ""), [saved["id"].as_str().unwrap()]);
    // A corrupt file is reported, not hidden, when asked for by id.
    let (status, _) = call(
        &state,
        "GET",
        "/api/history/uttt-1-aaaaaaaaaaaa",
        &Value::Null,
    );
    assert_eq!(status, 500);
}

#[test]
fn only_the_documented_methods_are_allowed() {
    let dir = TempDir::new("history-methods");
    let state = new_state(&dir);
    for (method, path) in [
        ("PUT", "/api/history"),
        ("DELETE", "/api/history"),
        ("POST", "/api/history/abc"),
        ("POST", "/api/history/abc/view"),
        ("GET", "/api/view"),
        ("GET", "/api/sessions/abc/save"),
    ] {
        let (status, _) = call(&state, method, path, &Value::Null);
        assert_eq!(status, 405, "{method} {path}");
    }
}
