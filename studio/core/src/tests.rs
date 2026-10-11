//! Tests of the logic shared by the server and the browser's program.

use cg_arena::record::RecordedAnswer;
use cg_arena::runner::EndReason;
use serde_json::json;
use studio_game::StudioGame;

use crate::games;
use crate::history::{self, day_start, percent_decode, record_id, Filter, Row};
use crate::request::{self, CreateRequest};
use crate::session::{GameState, Mode, Next, SeatConfig};
use crate::speed::{iterations, median};

fn uttt() -> Box<dyn StudioGame> {
    games::registered().remove(0)
}

fn humans() -> [SeatConfig; 2] {
    [
        SeatConfig::Human {
            name: "Ann".to_string(),
        },
        SeatConfig::Human {
            name: "Bob".to_string(),
        },
    ]
}

/// Plays the first legal move of whoever must act, until the game ends.
fn play_out(state: &mut GameState, game: &dyn StudioGame) {
    loop {
        match state.settle(game) {
            Next::Human { seat } => {
                let (_, valid) = state.position(game);
                let moves = state.human_moves(game, valid, seat);
                let answer = RecordedAnswer {
                    seat,
                    lines: moves[0].lines.clone(),
                    ms: 0.0,
                };
                state.apply_answer(game, answer);
            }
            Next::Over => return,
            other => panic!("unexpected {other:?}"),
        }
    }
}

fn request(body: serde_json::Value) -> CreateRequest {
    serde_json::from_value(body).unwrap()
}

#[test]
fn a_think_time_becomes_iterations() {
    assert_eq!(iterations(100, 12.5), 1250);
    assert_eq!(iterations(10, 0.0001), 1);
    assert_eq!(median(&mut [3.0, 1.0, 2.0]), 2.0);
    assert_eq!(median(&mut [4.0, 1.0, 2.0, 3.0]), 2.5);
    assert_eq!(median(&mut []), 0.0);
}

#[test]
fn a_request_is_validated_with_the_servers_messages() {
    let check = |body: serde_json::Value| {
        request::validate(request(body), true, || vec!["uttt-v001".to_string()])
            .map(|_| ())
            .unwrap_err()
    };
    let bot = json!({"kind": "bot", "release": "uttt-v001", "think_ms": 100, "mode": "fixed"});
    let human = json!({"kind": "human"});
    assert_eq!(
        request::validate(
            request(json!({"game": "x", "seed": 1, "seats": []})),
            false,
            Vec::new
        )
        .unwrap_err(),
        "unknown game \"x\""
    );
    assert_eq!(
        check(json!({"game": "uttt", "seats": []})),
        "seed is required"
    );
    assert_eq!(
        check(json!({"game": "uttt", "seed": 1, "opening_plies": 82, "seats": []})),
        "opening_plies is too large"
    );
    assert!(
        check(json!({"game": "uttt", "seed": 1, "opening_plies": 2, "seats": []}))
            .starts_with("openings are not supported")
    );
    assert_eq!(
        check(json!({"game": "uttt", "seed": 1, "seats": [human]})),
        "seats must hold exactly 2 seats"
    );
    assert_eq!(
        check(
            json!({"game": "uttt", "seed": 1, "seats": [human, {"kind": "bot", "release": "nope", "think_ms": 100, "mode": "fixed"}]})
        ),
        "unknown release \"nope\""
    );
    assert_eq!(
        check(
            json!({"game": "uttt", "seed": 1, "seats": [human, {"kind": "bot", "release": "uttt-v001", "think_ms": 5, "mode": "fixed"}]})
        ),
        "think_ms must be a number from 10 to 10000"
    );
    assert_eq!(
        check(
            json!({"game": "uttt", "seed": 1, "seats": [human, {"kind": "bot", "release": "uttt-v001", "think_ms": 100}]})
        ),
        "mode must be \"fixed\" or \"realtime\""
    );
    let new = request::validate(
        request(
            json!({"game": "uttt", "seed": 7, "seats": [{"kind": "human", "name": "  "}, bot]}),
        ),
        true,
        || vec!["uttt-v001".to_string()],
    )
    .unwrap();
    assert_eq!(new.seats[0].name(), "Player 1");
    assert_eq!(new.seats[1].name(), "uttt-v001");
}

#[test]
fn a_game_is_settled_played_and_recorded() {
    let game = uttt();
    let mut state = GameState::new(game.as_ref(), 5, 0, humans());
    assert_eq!(state.settle(game.as_ref()), Next::Human { seat: 0 });
    play_out(&mut state, game.as_ref());
    let result = state.result.clone().unwrap();
    assert_eq!(result.end, EndReason::Finished);
    let record = state.record(1_700_000_000);
    assert_eq!(record.unix_time, 1_700_000_000);
    assert_eq!(record.turns, state.turns);
    history::checked(Some(game.as_ref()), &record).unwrap();
}

#[test]
fn an_answer_the_rules_reject_loses_the_game() {
    let game = uttt();
    let mut state = GameState::new(game.as_ref(), 5, 0, humans());
    let answer = RecordedAnswer {
        seat: 0,
        lines: vec!["9 9".to_string()],
        ms: 1.0,
    };
    state.apply_answer(game.as_ref(), answer);
    let result = state.result.clone().unwrap();
    assert_eq!(result.winner, Some(1));
    assert!(matches!(result.end, EndReason::Invalid { seat: 0, .. }));
    assert_eq!(state.turns.len(), 1);
    // The invalid turn is kept in the record, which still checks.
    history::checked(Some(game.as_ref()), &state.record(1)).unwrap();
}

#[test]
fn a_bot_seat_has_a_time_limit() {
    let game = uttt();
    let seats = [
        SeatConfig::Bot {
            release: "uttt-v001".to_string(),
            think_ms: 100,
            mode: Mode::Fixed,
        },
        SeatConfig::Bot {
            release: "uttt-v001".to_string(),
            think_ms: 200,
            mode: Mode::Realtime,
        },
    ];
    let state = GameState::new(game.as_ref(), 5, 0, seats);
    let (live, _) = state.position(game.as_ref());
    assert_eq!(state.limit_for(0, &live).as_millis(), 10_500);
    assert!(state.limit_for(1, &live).as_millis() > 50);
    assert_eq!(state.bot_answers(), 0);
}

#[test]
fn a_query_is_decoded_and_dates_are_read() {
    assert_eq!(percent_decode("a+b%20c").unwrap(), "a b c");
    assert!(percent_decode("%zz").is_err());
    assert_eq!(day_start("1970-01-02").unwrap(), 86_400);
    assert_eq!(day_start("2024-02-29").unwrap(), 1_709_164_800);
    assert!(day_start("2023-02-29").is_err());
    assert!(day_start("2023-2-9").is_err());
    assert!(Filter::parse("result=bogus").is_err());
    assert!(Filter::parse("source=bogus").is_err());
    assert!(Filter::parse("from=yesterday").is_err());
}

#[test]
fn rows_are_filtered_and_listed_newest_first() {
    let row = |id: &str, unix_time: u64, winner: Option<usize>, end: &str, source: &str| Row {
        id: id.to_string(),
        game: "uttt".to_string(),
        unix_time,
        players: ["uttt-v001".to_string(), "Ann".to_string()],
        winner,
        end: end.to_string(),
        turns: 3,
        source: source.to_string(),
    };
    let rows = vec![
        row("a", 100, Some(0), "finished", "studio"),
        row("b", 200, None, "finished", "arena match"),
        row("c", 200, Some(1), "timeout", "studio"),
    ];
    let ids = |query: &str| -> Vec<String> {
        Filter::parse(query)
            .unwrap()
            .select(rows.clone())
            .into_iter()
            .map(|row| row.id)
            .collect()
    };
    assert_eq!(ids(""), ["c", "b", "a"]);
    assert_eq!(ids("result=x"), ["a"]);
    assert_eq!(ids("result=draw"), ["b"]);
    assert_eq!(ids("result=fault"), ["c"]);
    assert_eq!(ids("source=arena"), ["b"]);
    assert_eq!(ids("release=Ann&source=studio"), ["c", "a"]);
    assert_eq!(ids("release=nobody"), Vec::<String>::new());
    assert_eq!(ids("from=1970-01-02"), Vec::<String>::new());
    assert_eq!(ids("to=1970-01-01"), ["c", "b", "a"]);
}

#[test]
fn a_record_has_a_stable_id() {
    let game = uttt();
    let mut state = GameState::new(game.as_ref(), 5, 0, humans());
    play_out(&mut state, game.as_ref());
    let record = state.record(1_700_000_000);
    let id = record_id(&record).unwrap();
    assert!(id.starts_with("uttt-1700000000-"), "{id}");
    assert_eq!(id.len(), "uttt-1700000000-".len() + 12);
    assert_eq!(record_id(&record).unwrap(), id);
}
