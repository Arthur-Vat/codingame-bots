//! A Code of Ice and Fire's adapter for the studio
//! ([ADR 0025](../../../../docs/adr/0025-studio.md),
//! [ADR 0039](../../../../docs/adr/0039-ice-and-fire-maps-and-studio.md)).
//!
//! It holds no rules. Positions come from replaying [`IceAndFireReferee`]
//! (Bronze) and reading its protocol text, and what changed in a turn comes
//! from comparing two positions. The game has no human play: bots only
//! ([`StudioGame::human_play`]).
//!
//! Seat 0 is red (HQ at x 0, y 0, plays first), seat 1 is blue (HQ at x 11,
//! y 11).
//!
//! # Frames
//!
//! Frame 0 is the start (red to act). Frame `i` (`i >= 1`) is the position
//! after turn `i`; one turn is one seat's whole answer line. There is no
//! opening: the referee ignores opening plies, so after `n` turns there are
//! `1 + n` frames.
//!
//! Each frame is a JSON object. Coordinates are `x` (column, 0 to 11) and `y`
//! (row, 0 to 11); grids are 12 rows (`y`) of 12 entries (`x`):
//!
//! - `void`: 12 x 12 booleans, true for a void cell;
//! - `owner`: 12 x 12, each `null` (neutral or void), `0` or `1`;
//! - `active`: 12 x 12 booleans, true for an owned cell connected to its
//!   owner's HQ, false for neutral, void and inactive cells;
//! - `mine_spots`: `[[x, y], ...]`, in the order of the referee's initial
//!   input;
//! - `buildings`: `[{"owner": 0|1, "kind": "hq"|"mine"|"tower", "x", "y"}, ...]`,
//!   sorted by `y` then `x`;
//! - `units`: `[{"id", "owner": 0|1, "level": 1|2|3, "x", "y"}, ...]`, sorted
//!   by `id`;
//! - `gold`: `[g0, g1]`, `income`: `[i0, i1]`, `military`: `[m0, m1]` (the
//!   referee's `military_power`);
//! - `turn`: the game turn, 1 to 100 ([`IceAndFireReferee::game_turn`]);
//! - `to_move`: `0`, `1`, or `null` when the game is over;
//! - `last`: `null` in frame 0; otherwise an object:
//!   - `seat`: `0` or `1`, the seat that answered;
//!   - `answer`: the answer line as sent;
//!   - `moved`: `[{"id", "from": [x, y], "to": [x, y]}, ...]`;
//!   - `trained`: `[{"id", "level", "x", "y"}, ...]`;
//!   - `built`: `[{"kind": "mine"|"tower", "x", "y"}, ...]`;
//!   - `removed`: `[{"id", "owner", "level", "x", "y"}, ...]`;
//!
//!   what changed between the previous frame and this one, found by comparing
//!   the two (units by id, buildings by cell), not by reading the rules.
//!   `moved`, `trained` and `removed` are sorted by `id`, `built` by `y` then
//!   `x`;
//! - `result`: `null`, or `{"winner": 0 | 1 | null, "end": "hq" | "turns"}`:
//!   `"hq"` when the loser's HQ cell is owned by the winner, else `"turns"`
//!   (100 game turns played; `winner` is `null` for a draw).
//!
//! The state is read from the referee's text for seat 0 (`turn_input(0)` and
//! `initial_input(0)`), with its public `military_power` and `game_turn`.
//! Turns that break the rules are an error with the index of the turn; the
//! referee's own text is trusted.

use cg_arena::live::{LiveGame, ReplayError};
use cg_arena::record::RecordedAnswer;
use cg_arena::referee::{Answer, GameSetup, Outcome, Referee};
use ice_and_fire_referee::{IceAndFireReferee, SIZE};
use serde_json::{json, Value};
use studio_game::{live_game, GameInfo, HumanMove, StudioGame};

/// The studio's A Code of Ice and Fire.
pub struct IceAndFire;

/// A building, as the referee's input lists it (`owner` is the seat, since the
/// text is read from seat 0's view).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Building {
    owner: usize,
    kind: &'static str,
    x: usize,
    y: usize,
}

/// A unit, as the referee's input lists it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Unit {
    id: u32,
    owner: usize,
    level: u8,
    x: usize,
    y: usize,
}

/// A position, read from the referee's text for seat 0.
#[derive(Clone, Debug)]
struct Position {
    void: [[bool; SIZE]; SIZE],
    owner: [[Option<usize>; SIZE]; SIZE],
    active: [[bool; SIZE]; SIZE],
    /// Sorted by `y` then `x`.
    buildings: Vec<Building>,
    /// Sorted by id.
    units: Vec<Unit>,
    gold: [i64; 2],
    income: [i64; 2],
}

/// The next field of a line of the referee's text, as a number.
fn number<T: std::str::FromStr>(fields: &mut std::str::SplitWhitespace<'_>) -> T {
    fields
        .next()
        .and_then(|field| field.parse().ok())
        .expect("the referee sends the numbers its protocol describes")
}

impl Position {
    /// The position of `game`, from its turn input for seat 0, whose "own" is
    /// seat 0 and "opponent" seat 1.
    fn read(game: &IceAndFireReferee) -> Position {
        let input = game.turn_input(0);
        let mut lines = input.lines();
        let mut line = || lines.next().expect("the referee sends every line");
        let mut scalar = || -> i64 { line().parse().expect("the referee sends a number") };
        let (gold0, income0, gold1, income1) = (scalar(), scalar(), scalar(), scalar());

        let mut void = [[false; SIZE]; SIZE];
        let mut owner = [[None; SIZE]; SIZE];
        let mut active = [[false; SIZE]; SIZE];
        for y in 0..SIZE {
            let row = line();
            assert_eq!(row.chars().count(), SIZE, "the referee sends a full row");
            for (x, cell) in row.chars().enumerate() {
                void[y][x] = cell == '#';
                owner[y][x] = match cell {
                    'O' | 'o' => Some(0),
                    'X' | 'x' => Some(1),
                    '#' | '.' => None,
                    other => unreachable!("the referee sends no cell {other:?}"),
                };
                active[y][x] = cell == 'O' || cell == 'X';
            }
        }

        let count: usize = line().parse().expect("the referee counts the buildings");
        let mut buildings: Vec<Building> = (0..count)
            .map(|_| {
                let mut fields = line().split_whitespace();
                let owner = number(&mut fields);
                let kind = match number::<u8>(&mut fields) {
                    0 => "hq",
                    1 => "mine",
                    2 => "tower",
                    other => unreachable!("the referee sends no building type {other}"),
                };
                Building {
                    owner,
                    kind,
                    x: number(&mut fields),
                    y: number(&mut fields),
                }
            })
            .collect();
        buildings.sort_unstable_by_key(|building| (building.y, building.x));

        let count: usize = line().parse().expect("the referee counts the units");
        let mut units: Vec<Unit> = (0..count)
            .map(|_| {
                let mut fields = line().split_whitespace();
                Unit {
                    owner: number(&mut fields),
                    id: number(&mut fields),
                    level: number(&mut fields),
                    x: number(&mut fields),
                    y: number(&mut fields),
                }
            })
            .collect();
        units.sort_unstable_by_key(|unit| unit.id);

        Position {
            void,
            owner,
            active,
            buildings,
            units,
            gold: [gold0, gold1],
            income: [income0, income1],
        }
    }
}

/// The mine spots of a game, from the referee's initial input: their count,
/// then `x y` lines.
fn mine_spots(game: &IceAndFireReferee) -> Vec<(usize, usize)> {
    let input = game.initial_input(0);
    let mut lines = input.lines();
    let count: usize = lines
        .next()
        .and_then(|line| line.parse().ok())
        .expect("the referee sends the number of mine spots");
    lines
        .take(count)
        .map(|line| {
            let mut fields = line.split_whitespace();
            (number(&mut fields), number(&mut fields))
        })
        .collect()
}

/// What changed from `before` to `after` in the turn `seat` played with
/// `answer`: units compared by id, buildings by cell.
fn last_json(seat: usize, answer: &str, before: &Position, after: &Position) -> Value {
    let mut moved = Vec::new();
    let mut trained = Vec::new();
    for unit in &after.units {
        match before.units.iter().find(|old| old.id == unit.id) {
            Some(old) if (old.x, old.y) != (unit.x, unit.y) => moved.push(json!({
                "id": unit.id,
                "from": [old.x, old.y],
                "to": [unit.x, unit.y],
            })),
            Some(_) => {}
            None => trained.push(json!({
                "id": unit.id,
                "level": unit.level,
                "x": unit.x,
                "y": unit.y,
            })),
        }
    }
    let removed: Vec<Value> = before
        .units
        .iter()
        .filter(|old| !after.units.iter().any(|unit| unit.id == old.id))
        .map(|old| {
            json!({
                "id": old.id,
                "owner": old.owner,
                "level": old.level,
                "x": old.x,
                "y": old.y,
            })
        })
        .collect();
    let built: Vec<Value> = after
        .buildings
        .iter()
        .filter(|building| building.kind != "hq" && !before.buildings.contains(building))
        .map(|building| json!({ "kind": building.kind, "x": building.x, "y": building.y }))
        .collect();
    json!({
        "seat": seat,
        "answer": answer,
        "moved": moved,
        "trained": trained,
        "built": built,
        "removed": removed,
    })
}

/// A grid of `SIZE` rows of `SIZE` cells, as JSON, each cell turned by `value`.
fn grid<T: Copy>(cells: &[[T; SIZE]; SIZE], value: impl Fn(T) -> Value) -> Value {
    Value::Array(
        cells
            .iter()
            .map(|row| Value::Array(row.iter().map(|&cell| value(cell)).collect()))
            .collect(),
    )
}

/// The cell of a seat's HQ: red's is `(0, 0)`, blue's `(11, 11)`.
fn hq_cell(seat: usize) -> (usize, usize) {
    if seat == 0 {
        (0, 0)
    } else {
        (SIZE - 1, SIZE - 1)
    }
}

/// The result of a finished game; `end` is `"hq"` when the loser's HQ cell
/// is owned by the winner.
fn result_json(outcome: Outcome, position: &Position) -> Value {
    match outcome {
        Outcome::Win(winner) => {
            let (x, y) = hq_cell(1 - winner);
            let end = if position.owner[y][x] == Some(winner) {
                "hq"
            } else {
                "turns"
            };
            json!({ "winner": winner, "end": end })
        }
        Outcome::Draw => json!({ "winner": Value::Null, "end": "turns" }),
    }
}

/// The frame of the position `game` is in, read as `position`; `last` is the
/// frame's `last` field.
fn frame(
    game: &IceAndFireReferee,
    position: &Position,
    mine_spots: &[(usize, usize)],
    last: Value,
) -> Value {
    let outcome = game.outcome();
    let buildings: Vec<Value> = position
        .buildings
        .iter()
        .map(|b| json!({ "owner": b.owner, "kind": b.kind, "x": b.x, "y": b.y }))
        .collect();
    let units: Vec<Value> = position
        .units
        .iter()
        .map(|u| json!({ "id": u.id, "owner": u.owner, "level": u.level, "x": u.x, "y": u.y }))
        .collect();
    json!({
        "void": grid(&position.void, |void| json!(void)),
        "owner": grid(&position.owner, |owner| owner.map_or(Value::Null, |seat| json!(seat))),
        "active": grid(&position.active, |active| json!(active)),
        "mine_spots": mine_spots.iter().map(|&(x, y)| json!([x, y])).collect::<Vec<_>>(),
        "buildings": buildings,
        "units": units,
        "gold": position.gold,
        "income": position.income,
        "military": [game.military_power(0), game.military_power(1)],
        "turn": game.game_turn(),
        "to_move": if outcome.is_some() {
            Value::Null
        } else {
            game.players_to_act().first().map_or(Value::Null, |&seat| json!(seat))
        },
        "last": last,
        "result": outcome.map(|outcome| result_json(outcome, position)),
    })
}

impl StudioGame for IceAndFire {
    fn info(&self) -> GameInfo {
        GameInfo {
            id: "ice-and-fire",
            name: "A Code of Ice and Fire",
        }
    }

    fn new_referee(&self, setup: &GameSetup) -> Box<dyn Referee> {
        Box::new(IceAndFireReferee::new(setup))
    }

    fn baseline_bots(&self) -> &'static [&'static str] {
        &[
            "cutter", "guard", "chain", "bronze", "wood1", "wood2", "wood3", "random",
        ]
    }

    fn opening_turns(&self, _setup: &GameSetup) -> usize {
        0
    }

    fn frames(
        &self,
        setup: &GameSetup,
        turns: &[Vec<RecordedAnswer>],
    ) -> Result<Vec<Value>, ReplayError> {
        // `checked` judges the turns (seats, number of answers, rules); the
        // concrete referee follows it to read the position.
        let mut checked = LiveGame::new(self.new_referee(setup), *setup);
        let mut game = IceAndFireReferee::new(setup);
        let spots = mine_spots(&game);
        let mut position = Position::read(&game);
        let mut frames = vec![frame(&game, &position, &spots, Value::Null)];
        for (index, turn) in turns.iter().enumerate() {
            checked.play(turn.clone()).map_err(|invalid| ReplayError {
                turn: index,
                invalid,
            })?;
            let answers: Vec<Answer> = turn
                .iter()
                .map(|answer| Answer {
                    seat: answer.seat,
                    lines: answer.lines.clone(),
                })
                .collect();
            game.play(&answers)
                .expect("the live game accepted the turn");
            let first = turn.first().expect("the live game accepted a turn");
            let answer = first.lines.first().map_or("", String::as_str);
            let after = Position::read(&game);
            let last = last_json(first.seat, answer, &position, &after);
            frames.push(frame(&game, &after, &spots, last));
            position = after;
        }
        Ok(frames)
    }

    fn human_moves(
        &self,
        setup: &GameSetup,
        turns: &[Vec<RecordedAnswer>],
        _seat: usize,
    ) -> Result<Vec<HumanMove>, ReplayError> {
        // The turns are still judged, so that a broken record is an error.
        live_game(self, *setup, turns)?;
        Ok(Vec::new())
    }

    fn human_play(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests;
