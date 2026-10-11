//! Ultimate Tic-Tac-Toe's adapter for the studio
//! ([ADR 0025](../../../../docs/adr/0025-studio.md)).
//!
//! It holds no rules. Positions come from replaying [`UtttReferee`], and a
//! human's legal moves from the referee's turn input.
//!
//! # Frames
//!
//! Each frame is a JSON object:
//!
//! - `cells`: 9 rows of 9, each `null`, `0` or `1` (the seat that marked it);
//! - `small`: 3 rows of 3 small boards, each `null`, `0`, `1`, or `"draw"`
//!   for a full board without a winner;
//! - `last`: `[row, col]` of the last action, or `null`;
//! - `playable`: the valid actions as `[row, col]`, sorted; empty when the
//!   game is over;
//! - `to_move`: `0` or `1`, `null` when the game is over;
//! - `points`: `[p0, p1]`, the small boards won by each seat;
//! - `result`: `null`, or `{"winner": 0 | 1 | null}` (`null` for a draw).
//!
//! The opening plies the referee imposes are part of the start: frame 0
//! shows the position after the whole opening. A game with an opening holds
//! those plies as its first turns (the referee's only valid action is the
//! imposed one), and they add no frame. So after `n` turns there are
//! `1 + n.saturating_sub(opening)` frames, where `opening` is
//! [`opening_turns`](StudioGame::opening_turns): the number of imposed plies,
//! which can be fewer than `opening_plies` when the referee cuts the opening
//! short. A human is asked for no move during the opening, and an invalid
//! turn inside it is an error, not a frame.

use cg_arena::live::{LiveGame, ReplayError};
use cg_arena::record::RecordedAnswer;
use cg_arena::referee::{Answer, GameSetup, Outcome, Referee};
use serde_json::{json, Value};
use studio_game::{live_game, GameInfo, HumanMove, StudioGame};
use uttt_referee::{Cell, UtttReferee};

/// The studio's Ultimate Tic-Tac-Toe.
pub struct Uttt;

fn referee(setup: &GameSetup) -> UtttReferee {
    UtttReferee::with_opening(setup.seed, setup.opening_plies)
}

fn seat_json(seat: Option<usize>) -> Value {
    seat.map_or(Value::Null, |seat| json!(seat))
}

/// The state of the small board at `(board_row, board_col)`: its winner, or
/// `"draw"` when it is closed without one.
fn small_json(game: &UtttReferee, board_row: usize, board_col: usize) -> Value {
    match game.small_winner(board_row, board_col) {
        Some(seat) => json!(seat),
        None if game.small_closed(board_row, board_col) => json!("draw"),
        None => Value::Null,
    }
}

/// The frame of a position.
fn frame(game: &UtttReferee) -> Value {
    let cells: Vec<Vec<Value>> = (0..9)
        .map(|row| (0..9).map(|col| seat_json(game.mark((row, col)))).collect())
        .collect();
    let small: Vec<Vec<Value>> = (0..3)
        .map(|board_row| {
            (0..3)
                .map(|board_col| small_json(game, board_row, board_col))
                .collect()
        })
        .collect();
    let result = game.result();
    let mut playable: Vec<Cell> = if result.is_some() {
        Vec::new()
    } else {
        game.valid_actions().to_vec()
    };
    playable.sort_unstable();
    let playable: Vec<Value> = playable
        .iter()
        .map(|&(row, col)| json!([row, col]))
        .collect();
    json!({
        "cells": cells,
        "small": small,
        "last": game.last_action().map(|(row, col)| json!([row, col])),
        "playable": playable,
        "to_move": if result.is_some() { Value::Null } else { json!(game.to_move()) },
        "points": game.points(),
        "result": result.map(|outcome| json!({
            "winner": match outcome {
                Outcome::Win(seat) => json!(seat),
                Outcome::Draw => Value::Null,
            },
        })),
    })
}

/// The valid actions in a turn input: after the opponent's last action line
/// and the count come that many `row col` lines.
fn valid_cells(input: &str) -> Vec<Cell> {
    let mut lines = input.lines().skip(1);
    let count: usize = lines
        .next()
        .and_then(|line| line.parse().ok())
        .expect("the referee sends the number of valid actions");
    lines
        .take(count)
        .map(|line| {
            let (row, col) = line
                .split_once(' ')
                .expect("the referee sends `row col` lines");
            (
                row.parse().expect("the referee sends a row number"),
                col.parse().expect("the referee sends a column number"),
            )
        })
        .collect()
}

impl StudioGame for Uttt {
    fn info(&self) -> GameInfo {
        GameInfo {
            id: "uttt",
            name: "Ultimate Tic-Tac-Toe",
        }
    }

    fn new_referee(&self, setup: &GameSetup) -> Box<dyn Referee> {
        Box::new(referee(setup))
    }

    fn opening_turns(&self, setup: &GameSetup) -> usize {
        // The referee may cut the opening short of `opening_plies`.
        referee(setup).opening().len()
    }

    fn frames(
        &self,
        setup: &GameSetup,
        turns: &[Vec<RecordedAnswer>],
    ) -> Result<Vec<Value>, ReplayError> {
        let mut after_opening = referee(setup);
        let opening = after_opening.opening().to_vec();
        for &cell in &opening {
            after_opening
                .play_cell(cell)
                .expect("the opening plays valid actions");
        }

        // `checked` judges the turns (seats, number of answers, rules); the
        // concrete referee follows it to read the position.
        let mut checked = LiveGame::new(self.new_referee(setup), *setup);
        let mut game = referee(setup);
        let mut frames = vec![frame(&after_opening)];
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
            if index >= opening.len() {
                frames.push(frame(&game));
            }
        }
        Ok(frames)
    }

    fn human_moves(
        &self,
        setup: &GameSetup,
        turns: &[Vec<RecordedAnswer>],
        seat: usize,
    ) -> Result<Vec<HumanMove>, ReplayError> {
        let game = live_game(self, *setup, turns)?;
        if turns.len() < self.opening_turns(setup) || !game.to_act().contains(&seat) {
            return Ok(Vec::new());
        }
        let mut cells = valid_cells(&game.input_for(seat));
        cells.sort_unstable();
        Ok(cells
            .into_iter()
            .map(|(row, col)| HumanMove {
                action: json!({ "row": row, "col": col }),
                lines: vec![format!("{row} {col}")],
            })
            .collect())
    }
}

#[cfg(test)]
mod tests;
