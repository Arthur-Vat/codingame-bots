//! Tests of the protocol, through [`handle`].

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use cg_arena::live::LiveGame;
use cg_arena::record::RecordedAnswer;
use cg_arena::runner::bot_seed;
use serde_json::{json, Value};
use studio_core::games;
use studio_core::history::record_id;
use studio_core::session::{GameState, Next, SeatConfig};

use crate::handle;

/// The `ok` of the answer to `request`; panics with the error otherwise.
fn ok(request: Value) -> Value {
    let answer = call(&request);
    match answer.get("ok") {
        Some(value) => value.clone(),
        None => panic!("{request} gave {answer}"),
    }
}

/// The `error` of the answer to `request`; panics if it succeeded.
fn error(request: Value) -> String {
    let answer = call(&request);
    match answer.get("error").and_then(Value::as_str) {
        Some(message) => message.to_string(),
        None => panic!("{request} gave {answer}"),
    }
}

fn call(request: &Value) -> Value {
    let text = handle(&request.to_string());
    let answer: Value = serde_json::from_str(&text).unwrap();
    let keys = answer.as_object().unwrap().len();
    assert_eq!(keys, 1, "exactly one of ok and error: {text}");
    answer
}

fn releases() -> Value {
    json!(["uttt-v010", "uttt-v001"])
}

fn two_humans() -> Value {
    json!({"game": "uttt", "seed": 123, "seats": [{"kind": "human", "name": " Ann "}, {"kind": "human"}]})
}

fn new_game(request: Value) -> Value {
    ok(json!({"op": "new", "request": request, "releases": releases()}))
}

fn bot(release: &str, think_ms: u64, mode: &str) -> Value {
    json!({"kind": "bot", "release": release, "think_ms": think_ms, "mode": mode})
}

/// Plays a whole game of first legal moves through `settle` and `play`.
/// Returns the final state and the last answer to `settle`.
fn play_through(mut state: Value) -> (Value, Value) {
    loop {
        let settled = ok(json!({"op": "settle", "state": state}));
        let next = &settled["next"];
        match next["kind"].as_str().unwrap() {
            "over" => {
                state["result"] = settled["result"].clone();
                return (state, settled);
            }
            "human" => {
                let moves = settled["human_moves"].as_array().unwrap();
                let answer = json!({"seat": next["seat"], "lines": moves[0]["lines"], "ms": 0.0});
                let played = ok(json!({"op": "play", "state": state, "answer": answer}));
                state["turns"] = played["turns"].clone();
                state["result"] = played["result"].clone();
            }
            other => panic!("unexpected {other}"),
        }
    }
}

#[test]
fn the_games_are_listed() {
    assert_eq!(
        ok(json!({"op": "games"})),
        json!([{"id": "uttt", "name": "Ultimate Tic-Tac-Toe"}])
    );
}

#[test]
fn a_new_game_has_no_turns_and_no_result() {
    let state = new_game(json!({"game": "uttt", "seed": 123, "seats": [
        {"kind": "human", "name": " Ann "}, bot("uttt-v010", 100, "fixed")]}));
    assert_eq!(state["game"], "uttt");
    assert_eq!(state["seed"], 123);
    assert_eq!(state["opening_plies"], 0);
    assert_eq!(state["turns"], json!([]));
    assert_eq!(state["result"], Value::Null);
    assert_eq!(state["seats"][0], json!({"kind": "human", "name": "Ann"}));
    assert_eq!(
        state["seats"][1],
        json!({"kind": "bot", "name": "uttt-v010", "release": "uttt-v010",
               "think_ms": 100, "mode": "fixed", "fixed_iters": null})
    );
    assert_eq!(new_game(two_humans())["seats"][1]["name"], "Player 2");
}

#[test]
fn a_new_game_is_validated_like_a_session() {
    let fail =
        |request: Value| error(json!({"op": "new", "request": request, "releases": releases()}));
    let human = json!({"kind": "human"});
    assert_eq!(
        fail(json!({"game": "chess", "seed": 1, "seats": []})),
        "unknown game \"chess\""
    );
    assert_eq!(
        fail(json!({"game": "uttt", "seats": []})),
        "seed is required"
    );
    assert!(
        fail(json!({"game": "uttt", "seed": 1, "opening_plies": 4, "seats": []}))
            .starts_with("openings are not supported")
    );
    assert_eq!(
        fail(json!({"game": "uttt", "seed": 1, "seats": [human]})),
        "seats must hold exactly 2 seats"
    );
    assert_eq!(
        fail(json!({"game": "uttt", "seed": 1, "seats": [human, bot("uttt-v999", 100, "fixed")]})),
        "unknown release \"uttt-v999\""
    );
    assert_eq!(
        fail(
            json!({"game": "uttt", "seed": 1, "seats": [human, bot("uttt-v001", 10_001, "fixed")]})
        ),
        "think_ms must be a number from 10 to 10000"
    );
    // A mode that is neither fixed nor realtime is refused when the body is
    // read, as the server does.
    assert!(fail(
        json!({"game": "uttt", "seed": 1, "seats": [human, bot("uttt-v001", 100, "slow")]})
    )
    .contains("unknown variant"));
    assert!(error(json!({"op": "new", "request": two_humans()})).starts_with("invalid request"));
}

#[test]
fn settle_says_who_acts_and_what_the_moves_are() {
    let state = new_game(two_humans());
    let settled = ok(json!({"op": "settle", "state": state}));
    assert_eq!(settled["next"], json!({"kind": "human", "seat": 0}));
    assert_eq!(settled["to_act"], json!([0]));
    assert_eq!(settled["opening_turns"], 0);
    assert_eq!(settled["valid_turns"], 0);
    assert_eq!(settled["result"], Value::Null);
    assert_eq!(settled["frames"].as_array().unwrap().len(), 1);
    assert_eq!(settled["human_moves"].as_array().unwrap().len(), 81);
}

#[test]
fn settle_gives_a_bot_what_it_is_asked() {
    let state = new_game(
        json!({"game": "uttt", "seed": 18_446_744_073_709_551_000_u64, "seats": [
            bot("uttt-v010", 100, "fixed"), bot("uttt-v001", 200, "realtime")]}),
    );
    let settled = ok(json!({"op": "settle", "state": state}));
    let next = &settled["next"];
    assert_eq!(next["kind"], "bot");
    assert_eq!(next["seat"], 0);
    assert_eq!(next["limit_ms"], 10_500.0);
    assert_eq!(next["answer_lines"], 1);
    assert_eq!(settled["human_moves"], json!([]));
    let games = games::registered();
    let setup = cg_arena::referee::GameSetup {
        seed: 18_446_744_073_709_551_000,
        opening_plies: 0,
    };
    let live = LiveGame::new(games[0].new_referee(&setup), setup);
    assert_eq!(next["input"], live.input_for(0));
    // The seed is a decimal string, whole.
    let expected = bot_seed(18_446_744_073_709_551_000, 0, "uttt-v010").to_string();
    assert_eq!(next["bot_seed"], Value::String(expected.clone()));
    assert!(next["bot_seed"].is_string());
    // Every bot seat's seed, before it is asked, for a restart.
    assert_eq!(settled["bot_seeds"][0], Value::String(expected));
    assert_eq!(settled["bot_seeds"].as_array().map(Vec::len), Some(2));
}

#[test]
fn settle_ends_a_finished_game_and_skips_an_invalid_last_turn() {
    let (state, settled) = play_through(new_game(two_humans()));
    assert_eq!(settled["next"], json!({"kind": "over"}));
    assert_eq!(settled["to_act"], json!([]));
    assert_eq!(settled["human_moves"], json!([]));
    assert_eq!(settled["result"]["end"], json!({"kind": "finished"}));
    // The state given with no result: settle finds it.
    let mut open = state.clone();
    open["result"] = Value::Null;
    let again = ok(json!({"op": "settle", "state": open}));
    assert_eq!(again["result"], settled["result"]);
    let turns = state["turns"].as_array().unwrap().len();
    assert_eq!(again["valid_turns"], turns);
    assert_eq!(again["frames"].as_array().unwrap().len(), turns + 1);
    // A last turn the rules reject is not counted.
    let mut broken = new_game(two_humans());
    broken["turns"] = json!([[{"seat": 0, "lines": ["9 9"], "ms": 0.0}]]);
    let settled = ok(json!({"op": "settle", "state": broken}));
    assert_eq!(settled["valid_turns"], 0);
    assert_eq!(settled["frames"].as_array().unwrap().len(), 1);
}

#[test]
fn play_appends_answers_and_loses_on_an_invalid_one() {
    let state = new_game(two_humans());
    let settled = ok(json!({"op": "settle", "state": state}));
    let lines = settled["human_moves"][0]["lines"].clone();
    let answer = json!({"seat": 0, "lines": lines, "ms": 12.5});
    let played = ok(json!({"op": "play", "state": state, "answer": answer}));
    assert_eq!(played["turns"], json!([[answer]]));
    assert_eq!(played["result"], Value::Null);

    let bad = json!({"seat": 0, "lines": ["9 9"], "ms": 1.0});
    let played = ok(json!({"op": "play", "state": state, "answer": bad}));
    assert_eq!(played["turns"], json!([[bad]]));
    assert_eq!(played["result"]["winner"], 1);
    assert_eq!(played["result"]["end"]["kind"], "invalid");
    assert_eq!(played["result"]["end"]["seat"], 0);
}

#[test]
fn play_refuses_an_answer_out_of_turn_or_after_the_end() {
    let state = new_game(two_humans());
    let answer = json!({"seat": 1, "lines": ["0 0"], "ms": 0.0});
    assert_eq!(
        error(json!({"op": "play", "state": state, "answer": answer})),
        "it is not seat 1's turn"
    );
    let (finished, _) = play_through(state);
    let answer = json!({"seat": 0, "lines": ["0 0"], "ms": 0.0});
    assert_eq!(
        error(json!({"op": "play", "state": finished, "answer": answer})),
        "the game is already over"
    );
}

#[test]
fn play_refuses_a_state_whose_turns_break_the_rules_without_a_result() {
    let mut state = new_game(two_humans());
    state["turns"] = json!([[{"seat": 0, "lines": ["99 99"], "ms": 0.0}]]);
    let answer = json!({"seat": 0, "lines": ["1 1"], "ms": 0.0});
    assert_eq!(
        error(json!({"op": "play", "state": state, "answer": answer})),
        "turn 0 breaks the rules, and the game has no result"
    );
}

#[test]
fn a_game_played_through_the_protocol_matches_the_core() {
    let (state, settled) = play_through(new_game(two_humans()));

    let games = games::registered();
    let game = games[0].as_ref();
    let seats = [
        SeatConfig::Human {
            name: "Ann".to_string(),
        },
        SeatConfig::Human {
            name: "Player 2".to_string(),
        },
    ];
    let mut core = GameState::new(game, 123, 0, seats);
    loop {
        match core.settle(game) {
            Next::Human { seat } => {
                let (_, valid) = core.position(game);
                let lines = core.human_moves(game, valid, seat)[0].lines.clone();
                core.apply_answer(
                    game,
                    RecordedAnswer {
                        seat,
                        lines,
                        ms: 0.0,
                    },
                );
            }
            Next::Over => break,
            other => panic!("unexpected {other:?}"),
        }
    }
    let (_, valid) = core.position(game);
    assert_eq!(settled["frames"], json!(core.frames(game, valid)));
    assert_eq!(settled["result"], core.result_json());
    assert_eq!(state["turns"], json!(core.turns_json()));

    let text = ok(json!({"op": "record", "state": state, "unix_time": 1_700_000_000_u64}));
    let expected =
        serde_json::to_string(&serde_json::to_value(core.record(1_700_000_000)).unwrap()).unwrap();
    assert_eq!(text["record_text"], Value::String(expected));
}

#[test]
fn replay_gives_a_bot_the_inputs_it_saw() {
    let mut state = new_game(two_humans());
    for _ in 0..4 {
        let settled = ok(json!({"op": "settle", "state": state}));
        let seat = settled["next"]["seat"].clone();
        let lines = settled["human_moves"][0]["lines"].clone();
        let answer = json!({"seat": seat, "lines": lines, "ms": 0.0});
        let played = ok(json!({"op": "play", "state": state, "answer": answer}));
        state["turns"] = played["turns"].clone();
    }
    let asked = ok(json!({"op": "replay", "state": state, "seat": 1}));
    let asked = asked.as_array().unwrap();
    assert_eq!(asked.len(), 2);

    let games = games::registered();
    let setup = cg_arena::referee::GameSetup {
        seed: 123,
        opening_plies: 0,
    };
    let mut live = LiveGame::new(games[0].new_referee(&setup), setup);
    let mut expected = Vec::new();
    for turn in state["turns"].as_array().unwrap() {
        let answers: Vec<RecordedAnswer> = serde_json::from_value(turn.clone()).unwrap();
        if answers[0].seat == 1 {
            expected.push(json!({
                "input": live.input_for(1),
                "answer_lines": live.answer_lines(1),
                "lines": answers[0].lines,
            }));
        }
        live.play(answers).unwrap();
    }
    assert_eq!(asked, &expected);
    // The first input carries the initial input, the second does not.
    assert!(asked[0]["input"].as_str().unwrap().len() > asked[1]["input"].as_str().unwrap().len());

    assert_eq!(
        ok(json!({"op": "replay", "state": new_game(two_humans()), "seat": 0})),
        json!([])
    );
}

#[test]
fn replay_fails_when_the_turns_break_the_rules() {
    let mut state = new_game(two_humans());
    state["turns"] = json!([[{"seat": 0, "lines": ["9 9"], "ms": 0.0}]]);
    let message = error(json!({"op": "replay", "state": state, "seat": 0}));
    assert!(
        message.starts_with("turn 0: the answers of seat 0 break the rules"),
        "{message}"
    );
    assert_eq!(
        error(json!({"op": "replay", "state": new_game(two_humans()), "seat": 2})),
        "seat 2 is not a seat"
    );
}

/// A finished game as a record text, with a seed no `f64` holds.
fn record_text(seed: u64, unix_time: u64, source: &str) -> String {
    let mut state = new_game(json!({"game": "uttt", "seed": seed, "seats": [
        {"kind": "human", "name": "Ann"}, {"kind": "human", "name": "Bob"}]}));
    // JSON numbers above 2^53 survive the Rust side: the state is built
    // with serde_json, not with a JavaScript number.
    assert_eq!(state["seed"].as_u64(), Some(seed));
    state = play_through(state).0;
    let text = ok(json!({"op": "record", "state": state, "unix_time": unix_time}));
    let mut record: Value = serde_json::from_str(text["record_text"].as_str().unwrap()).unwrap();
    record["source"] = json!(source);
    serde_json::to_string(&record).unwrap()
}

#[test]
fn a_record_is_checked_and_viewed() {
    let text = record_text(5, 1_700_000_000, "studio");
    let checked = ok(json!({"op": "check", "record_text": text}));
    let id = checked["id"].as_str().unwrap();
    assert!(id.starts_with("uttt-1700000000-"), "{id}");
    assert_eq!(checked["row"]["id"], id);
    assert_eq!(checked["row"]["game"], "uttt");
    assert_eq!(checked["row"]["players"], json!(["Ann", "Bob"]));
    assert_eq!(checked["row"]["end"], "finished");
    assert_eq!(checked["row"]["source"], "studio");
    assert_eq!(checked["row"]["unix_time"], 1_700_000_000_u64);

    let view = ok(json!({"op": "view", "record_text": text}));
    let turns = view["record"]["turns"].as_array().unwrap().len();
    assert_eq!(checked["row"]["turns"], turns);
    assert_eq!(view["frames"].as_array().unwrap().len(), turns + 1);
    assert_eq!(view["opening_turns"], 0);
    assert_eq!(view["shown_turns"], turns);
}

#[test]
fn a_record_is_refused_with_the_servers_messages() {
    let text = record_text(5, 1_700_000_000, "studio");
    let with = |key: &str, value: Value| {
        let mut record: Value = serde_json::from_str(&text).unwrap();
        record[key] = value;
        record.to_string()
    };
    let fail = |text: String| error(json!({"op": "check", "record_text": text}));
    assert!(fail("{".to_string()).starts_with("invalid game record: "));
    assert!(fail(with("format", json!(2))).starts_with("unsupported record format 2"));
    assert_eq!(fail(with("game", json!("chess"))), "unknown game \"chess\"");
    assert!(fail(with("winner", json!(1))).starts_with("the referee ends the game with winner"));
    assert_eq!(
        fail(with("winner", json!(7))),
        "the record's winner is not a seat"
    );
    assert!(fail(with("turns", json!([]))).starts_with("the record says the game finished"));
    assert!(
        error(json!({"op": "view", "record_text": with("format", json!(9))}))
            .starts_with("unsupported record format 9")
    );
}

#[test]
fn an_invalid_last_turn_is_accepted_when_the_game_ended_by_it() {
    let mut state = new_game(two_humans());
    let bad = json!({"seat": 0, "lines": ["9 9"], "ms": 0.0});
    let played = ok(json!({"op": "play", "state": state, "answer": bad}));
    state["turns"] = played["turns"].clone();
    state["result"] = played["result"].clone();
    let text = ok(json!({"op": "record", "state": state, "unix_time": 5}));
    let text = text["record_text"].clone();
    let view = ok(json!({"op": "view", "record_text": text}));
    assert_eq!(view["shown_turns"], 0);
    assert_eq!(view["frames"].as_array().unwrap().len(), 1);
    let checked = ok(json!({"op": "check", "record_text": text}));
    assert_eq!(checked["row"]["end"], "invalid");
    assert_eq!(checked["row"]["winner"], 1);
}

/// A server state on a fresh data folder, for comparing with its routes.
struct Server {
    state: studio::State,
    dir: PathBuf,
}

impl Server {
    fn new() -> Server {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let dir = std::env::temp_dir().join(format!(
            "studio-rules-test-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .unwrap();
        Server {
            state: studio::State::new(repo, dir.join("web"), dir.join("data")),
            dir,
        }
    }

    fn call(&self, method: &str, path: &str, body: &[u8]) -> (u16, Value) {
        let response = studio::handle(&self.state, method, path, body);
        let value = serde_json::from_slice(&response.body).unwrap_or(Value::Null);
        (response.status, value)
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

#[test]
fn a_record_with_a_big_seed_keeps_it_and_gets_the_servers_id() {
    let seed = 18_446_744_073_709_551_557_u64;
    assert_ne!(seed as f64 as u64, seed);
    let text = record_text(seed, 1_700_000_001, "studio");
    let checked = ok(json!({"op": "check", "record_text": text}));
    let view = ok(json!({"op": "view", "record_text": text}));
    assert_eq!(view["record"]["seed"].as_u64(), Some(seed));
    // The record the rules program wrote holds the seed digit for digit.
    assert!(text.contains(&format!("\"seed\":{seed}")));

    let server = Server::new();
    let (status, saved) = server.call("POST", "/api/history", text.as_bytes());
    assert_eq!(status, 200, "{saved}");
    assert_eq!(saved["id"], checked["id"]);
    let record = studio_core::history::parse_record(text.as_bytes()).unwrap();
    assert_eq!(record_id(&record).unwrap(), saved["id"].as_str().unwrap());
}

#[test]
fn filter_agrees_with_the_servers_list() {
    let server = Server::new();
    let day = 86_400_u64;
    let base = 1_700_000_000_u64 / day * day;
    let texts = [
        record_text(1, base + 10, "studio"),
        record_text(2, base + 10, "arena match"),
        record_text(3, base + day + 5, "studio"),
        record_text(4, base - day, "arena match"),
    ];
    let mut rows = Vec::new();
    for text in &texts {
        let checked = ok(json!({"op": "check", "record_text": text}));
        let (status, saved) = server.call("POST", "/api/history", text.as_bytes());
        assert_eq!(status, 200, "{saved}");
        assert_eq!(saved["id"], checked["id"]);
        rows.push(checked["row"].clone());
    }
    let queries = [
        "",
        "game=uttt",
        "game=chess",
        "release=Ann",
        "release=Nobody",
        "result=x",
        "result=o",
        "result=draw",
        "result=fault",
        "result=unfinished",
        "source=studio",
        "source=arena",
        "from=2023-11-15&to=2023-11-15",
        "to=2023-11-14",
        "release=A%6En&source=studio",
        "release=A+nn",
        "bogus=1&&source=arena",
        "result=",
        "result=bogus",
        "source=bogus",
        "from=2023-13-01",
        "release=%zz",
    ];
    for query in queries {
        let (status, listed) = server.call("GET", &format!("/api/history?{query}"), b"");
        let answer = call(&json!({"op": "filter", "query": query, "rows": rows}));
        if status == 200 {
            assert_eq!(answer["ok"], listed, "query {query:?}");
        } else {
            assert_eq!(answer["error"], listed["error"], "query {query:?}");
        }
    }
    // The filter of a request body answers like the server's `POST /api/filter`, errors included.
    let body = serde_json::to_string(&rows).unwrap();
    let negative = r#"[{"id":"x","game":"uttt","unix_time":-1,"players":["a","b"],"winner":null,"end":"finished","turns":1,"source":"s"}]"#;
    let bodies = [
        body.as_str(),
        "[]",
        "",
        "not json",
        r#"{"rows": []}"#,
        "[1]",
        r#"[{"id": "x"}]"#,
        negative,
    ];
    for query in [
        "",
        "result=x",
        "release=A+nn",
        "result=bogus",
        "release=%zz",
    ] {
        for body in bodies {
            let (status, listed) =
                server.call("POST", &format!("/api/filter?{query}"), body.as_bytes());
            let answer = call(&json!({"op": "filter_body", "query": query, "body": body}));
            if status == 200 {
                assert_eq!(answer["ok"], listed, "query {query:?}, body {body:?}");
            } else {
                assert_eq!(
                    answer["error"], listed["error"],
                    "query {query:?}, body {body:?}"
                );
            }
        }
    }
    // The order is newest first, and two records of one second are told
    // apart by their ids, last first.
    let all = ok(json!({"op": "filter", "query": "", "rows": rows}));
    let times: Vec<u64> = all
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["unix_time"].as_u64().unwrap())
        .collect();
    assert!(times.windows(2).all(|pair| pair[0] >= pair[1]), "{times:?}");
}

#[test]
fn a_bad_request_is_an_error_and_never_a_panic() {
    let state = new_game(two_humans());
    let mut bad_seat = state.clone();
    bad_seat["turns"] = json!([[{"seat": 7, "lines": [], "ms": 0.0}]]);
    let mut bad_think = new_game(json!({"game": "uttt", "seed": 1, "seats": [
        bot("uttt-v001", 100, "fixed"), {"kind": "human"}]}));
    bad_think["seats"][0]["think_ms"] = json!(5);
    let mut bad_result = state.clone();
    bad_result["result"] = json!({"winner": 5, "end": {"kind": "finished"}});
    let mut unknown_game = state.clone();
    unknown_game["game"] = json!("chess");
    let mut one_seat = state.clone();
    one_seat["seats"] = json!([{"kind": "human", "name": "A"}]);
    let mut huge_plies = state.clone();
    huge_plies["opening_plies"] = json!(1000);
    let mut wrong_lines = state.clone();
    wrong_lines["turns"] = json!([[{"seat": 0, "lines": [1, 2], "ms": 0.0}]]);
    let mut garbage_lines = state.clone();
    garbage_lines["turns"] = json!([
        [{"seat": 0, "lines": ["\u{0}\n\n-1 -1 99999999999999999999"], "ms": 0.0}],
        [{"seat": 1, "lines": [], "ms": 0.0}, {"seat": 0, "lines": [], "ms": 0.0}],
    ]);

    let requests = [
        "".to_string(),
        "not json".to_string(),
        "{".to_string(),
        "[]".to_string(),
        "null".to_string(),
        "42".to_string(),
        "{}".to_string(),
        r#"{"op":5}"#.to_string(),
        r#"{"op":"nope"}"#.to_string(),
        r#"{"op":"new"}"#.to_string(),
        r#"{"op":"new","request":5,"releases":[]}"#.to_string(),
        r#"{"op":"new","request":{},"releases":[]}"#.to_string(),
        r#"{"op":"settle"}"#.to_string(),
        r#"{"op":"settle","state":{}}"#.to_string(),
        r#"{"op":"settle","state":null}"#.to_string(),
        r#"{"op":"play","state":{},"answer":{}}"#.to_string(),
        r#"{"op":"replay","state":{},"seat":-1}"#.to_string(),
        r#"{"op":"record","state":{},"unix_time":"x"}"#.to_string(),
        r#"{"op":"check"}"#.to_string(),
        r#"{"op":"check","record_text":5}"#.to_string(),
        r#"{"op":"check","record_text":""}"#.to_string(),
        r#"{"op":"check","record_text":"{\"format\":1}"}"#.to_string(),
        r#"{"op":"view","record_text":"[1,2"}"#.to_string(),
        r#"{"op":"filter","query":"a=b"}"#.to_string(),
        r#"{"op":"filter","query":5,"rows":[]}"#.to_string(),
        r#"{"op":"filter","query":"","rows":[{"id":1}]}"#.to_string(),
        r#"{"op":"filter_body","query":"a=b"}"#.to_string(),
        r#"{"op":"filter_body","query":5,"body":""}"#.to_string(),
        r#"{"op":"filter_body","query":"","body":"[{\"id\":1}]"}"#.to_string(),
        json!({"op": "settle", "state": bad_seat}).to_string(),
        json!({"op": "settle", "state": bad_think}).to_string(),
        json!({"op": "settle", "state": bad_result}).to_string(),
        json!({"op": "settle", "state": unknown_game}).to_string(),
        json!({"op": "settle", "state": one_seat}).to_string(),
        json!({"op": "settle", "state": huge_plies}).to_string(),
        json!({"op": "settle", "state": wrong_lines}).to_string(),
        json!({"op": "settle", "state": garbage_lines}).to_string(),
        json!({"op": "replay", "state": bad_seat, "seat": 0}).to_string(),
    ];
    for request in requests {
        let text = handle(&request);
        let answer: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
        let object = answer
            .as_object()
            .unwrap_or_else(|| panic!("{request} gave {text}"));
        assert_eq!(object.len(), 1, "{request} gave {text}");
        if let Some(message) = object.get("error") {
            assert!(message.is_string(), "{request} gave {text}");
        }
    }
    assert_eq!(error(json!({"op": "nope"})), "unknown op \"nope\"");
    assert!(error(json!({"op": "settle", "state": bad_seat})).contains("seat 7 is not a seat"));
    assert_eq!(
        error(json!({"op": "settle", "state": bad_think})),
        "think_ms must be a number from 10 to 10000"
    );
    assert_eq!(
        error(json!({"op": "settle", "state": unknown_game})),
        "unknown game \"chess\""
    );
}
