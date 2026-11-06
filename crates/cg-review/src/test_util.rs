//! A small made-up game for the tests.

use crate::profile::Profiling;
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
            none: None,
        }],
        lead: "cells".to_string(),
        plies,
        middle: None,
        endgame: None,
        moments: Vec::new(),
    }
}

/// A made-up game: the event "build" is a command starting with `B`, an
/// opening token is the command itself.
pub struct Toy;

impl Profiling for Toy {
    fn turns() -> Vec<u32> {
        vec![5, 10]
    }
    fn opening_plies() -> usize {
        3
    }
    fn key_metrics() -> Vec<&'static str> {
        vec!["cells"]
    }
    fn events(trace: &Trace, ply: usize) -> Vec<String> {
        trace.plies[ply]
            .commands
            .split(';')
            .filter(|c| c.starts_with('B'))
            .map(|_| "build".to_string())
            .collect()
    }
    fn opening(trace: &Trace, seat: usize, plies: usize) -> Vec<String> {
        trace
            .plies
            .iter()
            .filter(|p| p.to_act == seat)
            .take(plies)
            .map(|p| p.commands.clone())
            .collect()
    }
    fn lower_is_better(key: &str) -> bool {
        key == "debt"
    }
    fn compare_turns() -> Vec<u32> {
        vec![2, 3]
    }
    fn resource() -> &'static str {
        "cells"
    }
    /// Each command starting with `B` costs 2 and one starting with `T` 1.
    fn spending(trace: &Trace, ply: usize) -> Vec<(String, i64)> {
        let mut out = Vec::new();
        for (letter, name, cost) in [('B', "build", 2), ('T', "train", 1)] {
            let n = trace.plies[ply]
                .commands
                .split(';')
                .filter(|c| c.starts_with(letter))
                .count() as i64;
            if n > 0 {
                out.push((name.to_string(), n * cost));
            }
        }
        out
    }
}
