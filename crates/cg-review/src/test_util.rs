//! A small made-up game for the tests.

use crate::trace::{Metric, Ply, Trace};

/// A game of `cells` (each side's value at each ply, red then blue), red
/// to play at even plies, with one metric `cells` that is also the lead.
pub fn game(cells: &[(i64, i64)], winner: Option<usize>) -> Trace {
    let plies = cells
        .iter()
        .enumerate()
        .map(|(index, &(red, blue))| Ply {
            game_turn: index as u32 / 2 + 1,
            to_act: index % 2,
            board: vec!["R. ##".to_string(), " . B.".to_string()],
            metrics: [vec![red], vec![blue]],
            commands: if index % 2 == 0 {
                "WAIT".to_string()
            } else {
                String::new()
            },
            spent: 0,
        })
        .collect();
    Trace {
        name: "g1".to_string(),
        players: ["alpha".to_string(), "beta".to_string()],
        winner,
        end: "finished".to_string(),
        primer: vec!["Red is R, blue is B.".to_string()],
        metrics: vec![Metric {
            key: "cells".to_string(),
            meaning: "Cells held.".to_string(),
        }],
        lead: "cells".to_string(),
        plies,
        middle: None,
        endgame: None,
        moments: Vec::new(),
    }
}
