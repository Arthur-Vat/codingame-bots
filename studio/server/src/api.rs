//! The routes. [`handle`] takes a request already read from the socket and
//! returns the response to write, so everything here is testable without a
//! network.

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{json, Value};
use studio_game::StudioGame;

use cg_arena::record::RecordedAnswer;
use cg_arena::referee::SEATS;
use cg_arena::runner::EndReason;
use studio_core::request::{self, CreateRequest};
use studio_core::session::GameState;

use crate::history;
use crate::runner::{spawn_drive, spawn_launch};
use crate::session::{other, GameResult, Mode, Progress, SeatConfig, Session, Status};
use crate::state::{lock, SharedSession, State};
use crate::web;

/// A response to write.
#[derive(Clone, Debug)]
pub struct Response {
    pub status: u16,
    pub content_type: String,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: u16, content_type: &str, body: Vec<u8>) -> Response {
        Response {
            status,
            content_type: content_type.to_string(),
            body,
        }
    }

    pub fn json(status: u16, value: &Value) -> Response {
        Response::new(
            status,
            "application/json",
            serde_json::to_vec(value).unwrap_or_default(),
        )
    }

    pub fn error(status: u16, message: &str) -> Response {
        Response::json(status, &json!({ "error": message }))
    }
}

/// Answers one request. `path` may end with a query string (`?a=b&c=d`).
pub fn handle(state: &State, method: &str, path: &str, body: &[u8]) -> Response {
    let (path, query) = path.split_once('?').unwrap_or((path, ""));
    let method = if method == "HEAD" { "GET" } else { method };
    match path.strip_prefix("/api") {
        Some("") => Response::error(404, "unknown route"),
        Some(rest) if rest.starts_with('/') => api(state, method, &rest[1..], query, body),
        _ if method == "GET" => web::serve(state.web_dir(), path),
        _ => Response::error(405, "only GET serves files"),
    }
}

fn api(state: &State, method: &str, path: &str, query: &str, body: &[u8]) -> Response {
    let segments: Vec<&str> = path.split('/').collect();
    match (method, segments.as_slice()) {
        ("GET", ["games"]) => list_games(state),
        ("POST", ["sessions"]) => create_session(state, body),
        ("GET", ["sessions", id]) => {
            with_session(state, id, |_, s, game| Response::json(200, &s.view(game)))
        }
        ("DELETE", ["sessions", id]) => {
            if state.remove(id) {
                Response::json(200, &json!({ "deleted": true }))
            } else {
                unknown_session()
            }
        }
        ("POST", ["sessions", id, "move"]) => post_move(state, id, body),
        ("POST", ["sessions", id, "takeback"]) => post_takeback(state, id, body),
        ("POST", ["sessions", id, "end"]) => post_end(state, id, body),
        ("GET", ["sessions", id, "record"]) => with_session(state, id, |_, s, _| {
            Response::json(
                200,
                &serde_json::to_value(s.record()).unwrap_or(Value::Null),
            )
        }),
        ("POST", ["sessions", id, "save"]) => history::save_session(state, id),
        ("GET", ["history"]) => history::list(state, query),
        ("POST", ["history"]) => history::save(state, body),
        ("GET", ["history", id]) => history::get(state, id),
        ("DELETE", ["history", id]) => history::delete(state, id),
        ("GET", ["history", id, "view"]) => history::view(state, id),
        ("POST", ["view"]) => history::view_body(state, body),
        ("POST", ["filter"]) => history::filter_rows(query, body),
        (_, ["games"])
        | (_, ["history"])
        | (_, ["history", _])
        | (_, ["history", _, "view"])
        | (_, ["view"])
        | (_, ["filter"])
        | (_, ["sessions"])
        | (_, ["sessions", _])
        | (_, ["sessions", _, "move" | "takeback" | "end" | "record" | "save"]) => {
            Response::error(405, "method not allowed")
        }
        _ => Response::error(404, "unknown route"),
    }
}

fn unknown_session() -> Response {
    Response::error(404, "unknown session")
}

fn parse<T: DeserializeOwned>(body: &[u8]) -> Result<T, Response> {
    serde_json::from_slice(body)
        .map_err(|error| Response::error(400, &format!("invalid request body: {error}")))
}

/// Runs `action` on the session `id` with its lock held.
fn with_session(
    state: &State,
    id: &str,
    action: impl FnOnce(&SharedSession, &mut Session, &dyn StudioGame) -> Response,
) -> Response {
    let Some(shared) = state.session(id) else {
        return unknown_session();
    };
    let mut session = lock(&shared);
    let Some(game) = state.game(&session.core.game) else {
        return Response::error(500, "the session's game is not registered");
    };
    action(&shared, &mut session, game)
}

fn list_games(state: &State) -> Response {
    let games: Vec<Value> = state
        .games()
        .iter()
        .map(|game| {
            let info = game.info();
            json!({
                "id": info.id,
                "name": info.name,
                "releases": state.releases().list(info.id),
            })
        })
        .collect();
    Response::json(200, &Value::Array(games))
}

fn create_session(state: &State, body: &[u8]) -> Response {
    let request: CreateRequest = match parse(body) {
        Ok(request) => request,
        Err(response) => return response,
    };
    let game = state.game(&request.game);
    let game_id = request.game.clone();
    let new = match request::validate(request, game.is_some(), || state.releases().list(&game_id)) {
        Ok(new) => new,
        Err(message) => return Response::error(400, &message),
    };
    let Some(game) = game else {
        return Response::error(500, "the game is not registered");
    };
    let any_bot = new.seats.iter().any(SeatConfig::is_bot);
    let core = GameState::new(game, new.seed, new.opening_plies, new.seats);
    let mut session = Session::new(state.new_id(), core);
    if any_bot {
        session.status = Status::Compiling;
    } else {
        session.settle(game);
    }
    let id = session.id.clone();
    let shared = state.insert(session);
    if any_bot {
        let generation = lock(&shared).generation;
        spawn_launch(state, &shared, generation);
    }
    Response::json(200, &json!({ "id": id }))
}

#[derive(Deserialize)]
struct MoveRequest {
    seat: usize,
    index: usize,
}

fn post_move(state: &State, id: &str, body: &[u8]) -> Response {
    let request: MoveRequest = match parse(body) {
        Ok(request) => request,
        Err(response) => return response,
    };
    with_session(state, id, |shared, s, game| {
        if s.status != Status::WaitingHuman {
            return Response::error(
                409,
                &format!("not waiting for a move (status {})", s.status.as_str()),
            );
        }
        let (live, valid) = s.core.position(game);
        if live.to_act() != [request.seat]
            || !matches!(
                s.core.seats.get(request.seat),
                Some(SeatConfig::Human { .. })
            )
        {
            return Response::error(409, "it is not that human's turn");
        }
        let setup = s.core.setup();
        let moves = match game.human_moves(&setup, &s.core.turns[..valid], request.seat) {
            Ok(moves) => moves,
            Err(error) => return Response::error(409, &error.to_string()),
        };
        let Some(chosen) = moves.get(request.index) else {
            return Response::error(
                409,
                &format!(
                    "index {} is not one of the {} moves",
                    request.index,
                    moves.len()
                ),
            );
        };
        s.apply_answer(
            game,
            RecordedAnswer {
                seat: request.seat,
                lines: chosen.lines.clone(),
                ms: 0.0,
            },
        );
        if s.settle(game) {
            spawn_drive(state, shared, s.generation);
        }
        Response::json(200, &s.view(game))
    })
}

#[derive(Deserialize)]
struct TakebackRequest {
    turns: usize,
}

fn post_takeback(state: &State, id: &str, body: &[u8]) -> Response {
    let request: TakebackRequest = match parse(body) {
        Ok(request) => request,
        Err(response) => return response,
    };
    with_session(state, id, |shared, s, game| {
        if !matches!(
            s.status,
            Status::WaitingHuman | Status::BotThinking | Status::Over
        ) {
            return Response::error(
                409,
                &format!("no takeback now (status {})", s.status.as_str()),
            );
        }
        if s.core.has_bot_in_mode(Mode::Realtime) {
            return Response::error(
                409,
                "takebacks are unavailable against a bot playing in real time",
            );
        }
        let (_, valid) = s.core.position(game);
        if request.turns < s.core.opening_turns || request.turns > valid {
            return Response::error(
                409,
                &format!(
                    "turns must be from {} to {valid}, got {}",
                    s.core.opening_turns, request.turns
                ),
            );
        }
        s.stop();
        s.core.turns.truncate(request.turns);
        s.core.result = None;
        s.error = None;
        s.progress = None;
        if s.core.seats.iter().any(SeatConfig::is_bot) {
            s.status = Status::Rewinding;
            s.progress = Some(Progress {
                done: 0,
                total: Some(s.core.bot_answers()),
            });
            spawn_launch(state, shared, s.generation);
        } else {
            s.settle(game);
        }
        Response::json(200, &s.view(game))
    })
}

#[derive(Deserialize)]
struct EndRequest {
    seat: usize,
    reason: String,
}

fn post_end(state: &State, id: &str, body: &[u8]) -> Response {
    let request: EndRequest = match parse(body) {
        Ok(request) => request,
        Err(response) => return response,
    };
    with_session(state, id, |_, s, game| {
        if request.seat >= SEATS {
            return Response::error(400, "seat must be 0 or 1");
        }
        let seat = request.seat;
        let end = match request.reason.as_str() {
            "resign" => EndReason::Resigned { seat },
            "timeout" => EndReason::Timeout {
                seat,
                limit_ms: 0.0,
            },
            _ => return Response::error(400, "reason must be \"resign\" or \"timeout\""),
        };
        if !matches!(s.core.seats[seat], SeatConfig::Human { .. }) {
            return Response::error(409, "only a human seat can resign or run out of time");
        }
        if matches!(s.status, Status::Over | Status::Failed) {
            return Response::error(409, "the game is already over");
        }
        s.finish(GameResult {
            winner: Some(other(seat)),
            end,
        });
        Response::json(200, &s.view(game))
    })
}
