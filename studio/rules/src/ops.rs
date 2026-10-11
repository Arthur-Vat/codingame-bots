//! The operations of the protocol, one function each.

use cg_arena::live::{LiveGame, ReplayError};
use cg_arena::record::RecordedAnswer;
use cg_arena::referee::SEATS;
use cg_arena::runner::bot_seed;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{json, Value};
use studio_core::games;
use studio_core::history::{self, Filter, Row};
use studio_core::request::{self, CreateRequest};
use studio_core::session::{GameState, Next, SEVERAL_SEATS};
use studio_game::StudioGame;

use crate::wire::{to_state, StateWire};

/// Runs the operation `op` of `request`.
pub fn run(op: &str, request: &Value) -> Result<Value, String> {
    match op {
        "games" => Ok(list_games()),
        "new" => new(args(request)?),
        "settle" => settle(args(request)?),
        "play" => play(args(request)?),
        "replay" => replay(args(request)?),
        "record" => record(args(request)?),
        "check" => check(args(request)?),
        "view" => view(args(request)?),
        "filter" => filter(args(request)?),
        _ => Err(format!("unknown op {op:?}")),
    }
}

/// The fields of `request` an operation reads.
fn args<T: DeserializeOwned>(request: &Value) -> Result<T, String> {
    T::deserialize(request).map_err(|error| format!("invalid request: {error}"))
}

/// A checked state and its game, from the registered `games`.
fn state_of(
    wire: StateWire,
    games: &[Box<dyn StudioGame>],
) -> Result<(GameState, &dyn StudioGame), String> {
    let game =
        games::find(games, &wire.game).ok_or_else(|| format!("unknown game {:?}", wire.game))?;
    Ok((to_state(wire, game)?, game))
}

fn state_json(state: &GameState) -> Value {
    json!({
        "game": state.game,
        "seed": state.seed,
        "opening_plies": state.opening_plies,
        "seats": (0..SEATS).map(|seat| state.seat_json(seat)).collect::<Vec<_>>(),
        "turns": state.turns_json(),
        "result": state.result_json(),
    })
}

fn list_games() -> Value {
    let games: Vec<Value> = games::registered()
        .iter()
        .map(|game| {
            let info = game.info();
            json!({ "id": info.id, "name": info.name })
        })
        .collect();
    Value::Array(games)
}

#[derive(Deserialize)]
struct NewArgs {
    request: CreateRequest,
    releases: Vec<String>,
}

fn new(args: NewArgs) -> Result<Value, String> {
    let games = games::registered();
    let game = games::find(&games, &args.request.game);
    let new = request::validate(args.request, game.is_some(), || args.releases)?;
    let game = game.ok_or_else(|| format!("unknown game {:?}", new.game))?;
    let state = GameState::new(game, new.seed, new.opening_plies, new.seats);
    Ok(state_json(&state))
}

#[derive(Deserialize)]
struct StateArgs {
    state: StateWire,
}

/// Who acts next, with what a bot needs to be asked.
fn next_json(state: &GameState, live: &LiveGame, next: Next) -> Value {
    match next {
        Next::Over => json!({ "kind": "over" }),
        Next::Human { seat } => json!({ "kind": "human", "seat": seat }),
        Next::Bot { seat } => json!({
            "kind": "bot",
            "seat": seat,
            "input": live.input_for(seat),
            "answer_lines": live.answer_lines(seat),
            "limit_ms": state.limit_for(seat, live).as_secs_f64() * 1000.0,
            "bot_seed": bot_seed(state.seed, seat, state.seats[seat].name()).to_string(),
        }),
        Next::Unsupported => json!({ "kind": "unsupported", "error": SEVERAL_SEATS }),
    }
}

fn settle(args: StateArgs) -> Result<Value, String> {
    let games = games::registered();
    let (mut state, game) = state_of(args.state, &games)?;
    let (live, valid) = state.position(game);
    let next = state.settle(game);
    let over = state.result.is_some() || next == Next::Unsupported;
    let to_act = if over { Vec::new() } else { live.to_act() };
    let human_moves = match next {
        Next::Human { seat } => state.human_moves(game, valid, seat),
        _ => Vec::new(),
    };
    Ok(json!({
        "opening_turns": state.opening_turns,
        "valid_turns": valid,
        "to_act": to_act,
        "frames": state.frames(game, valid),
        "human_moves": human_moves,
        "result": state.result_json(),
        "next": next_json(&state, &live, next),
    }))
}

#[derive(Deserialize)]
struct PlayArgs {
    state: StateWire,
    answer: RecordedAnswer,
}

fn play(args: PlayArgs) -> Result<Value, String> {
    let games = games::registered();
    let (mut state, game) = state_of(args.state, &games)?;
    if state.result.is_some() {
        return Err("the game is already over".to_string());
    }
    let seat = args.answer.seat;
    let (live, _) = state.position(game);
    if live.to_act() != [seat] {
        return Err(format!("it is not seat {seat}'s turn"));
    }
    state.apply_answer(game, args.answer);
    Ok(json!({
        "turns": state.turns_json(),
        "result": state.result_json(),
    }))
}

#[derive(Deserialize)]
struct ReplayArgs {
    state: StateWire,
    seat: usize,
}

/// What a restarted bot of `seat` is asked, as `resync_bot_with` walks the
/// game: for each turn where the seat answered, its input and the lines it
/// must answer again.
fn replay(args: ReplayArgs) -> Result<Value, String> {
    if args.seat >= SEATS {
        return Err(format!("seat {} is not a seat", args.seat));
    }
    let games = games::registered();
    let (state, game) = state_of(args.state, &games)?;
    let mut live = LiveGame::new(game.new_referee(&state.setup()), state.setup());
    let mut asked = Vec::new();
    for (index, turn) in state.turns.iter().enumerate() {
        if let Some(recorded) = turn.iter().find(|answer| answer.seat == args.seat) {
            asked.push(json!({
                "input": live.input_for(args.seat),
                "answer_lines": live.answer_lines(args.seat),
                "lines": recorded.lines,
            }));
        }
        live.play(turn.clone()).map_err(|invalid| {
            ReplayError {
                turn: index,
                invalid,
            }
            .to_string()
        })?;
    }
    Ok(Value::Array(asked))
}

#[derive(Deserialize)]
struct RecordArgs {
    state: StateWire,
    unix_time: u64,
}

fn record(args: RecordArgs) -> Result<Value, String> {
    let games = games::registered();
    let (state, _) = state_of(args.state, &games)?;
    // Written as the server writes it: through a `Value`, so that the keys
    // come out in the same order.
    let record = serde_json::to_value(state.record(args.unix_time))
        .and_then(|value| serde_json::to_string(&value))
        .map_err(|error| error.to_string())?;
    Ok(json!({ "record_text": record }))
}

#[derive(Deserialize)]
struct RecordTextArgs {
    record_text: String,
}

fn check(args: RecordTextArgs) -> Result<Value, String> {
    let record = history::parse_record(args.record_text.as_bytes())?;
    let games = games::registered();
    history::checked(games::find(&games, &record.game), &record)?;
    let id = history::record_id(&record)?;
    let row = Row::new(&id, &record);
    Ok(json!({ "id": id, "row": row }))
}

fn view(args: RecordTextArgs) -> Result<Value, String> {
    let record = history::parse_record(args.record_text.as_bytes())?;
    let games = games::registered();
    history::view(games::find(&games, &record.game), record)
}

#[derive(Deserialize)]
struct FilterArgs {
    query: String,
    rows: Vec<Row>,
}

fn filter(args: FilterArgs) -> Result<Value, String> {
    let filter = Filter::parse(&args.query)?;
    serde_json::to_value(filter.select(args.rows)).map_err(|error| error.to_string())
}
