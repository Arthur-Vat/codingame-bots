use cg_arena::codingame::{to_record, Log};
use cg_review::profile::Profiling;
use cg_review::trace::{Moment, Trace};

use super::*;
use crate::codingame::IceAndFire as Conversion;
use crate::trace::{metrics, trace};

/// A real log: red (Gabbek) lost its HQ at answer 42 to blue (Tarleya).
const FINISHED: &str = include_str!("../../testdata/914386214.json");

fn real() -> Trace {
    let log: Log = serde_json::from_str(FINISHED).expect("a log");
    let record = to_record::<Conversion>(&log, "ranking").expect("converts");
    trace("914386214", &record)
}

#[test]
fn the_key_metrics_exist_in_the_trace() {
    let keys: Vec<String> = metrics().into_iter().map(|m| m.key).collect();
    let wanted = IceAndFire::key_metrics();
    assert_eq!(wanted.len(), 10);
    assert_eq!(IceAndFire::turns(), [5, 10, 15, 20, 30]);
    assert_eq!(IceAndFire::opening_plies(), 3);
    for key in wanted {
        assert!(keys.iter().any(|k| k == key), "{key}");
    }
}

#[test]
fn the_second_seat_is_mirrored_by_the_half_turn() {
    assert_eq!(mirrored(0, 1, 0), (1, 0));
    assert_eq!(mirrored(1, 10, 11), (1, 0));
    assert_eq!(mirrored(1, 0, 0), (11, 11));
    assert_eq!(mirrored(1, 9, 8), (2, 3));
}

#[test]
fn openings_compare_across_seats() {
    let trace = real();
    // Red answered `TRAIN 1 1 0;TRAIN 1 2 0`, then moved to (1, 1) and (2, 1).
    assert_eq!(
        IceAndFire::opening(&trace, 0, 3)[..2],
        ["T1(1,0) T1(2,0)", "M(1,1) M(2,1)"]
    );
    // Blue answered `TRAIN 1 10 10;TRAIN 1 10 11`: mirrored, (1, 1) and (1, 0).
    assert_eq!(IceAndFire::opening(&trace, 1, 3)[..1], ["T1(1,1) T1(1,0)"]);
    assert_eq!(IceAndFire::opening(&trace, 1, 3).len(), 3);
}

#[test]
fn events_follow_the_commands() {
    let trace = real();
    assert_eq!(
        IceAndFire::events(&trace, 0),
        ["train level 1", "train level 1"]
    );
    assert!(IceAndFire::events(&trace, 2).is_empty());
    // The seventh answer trains a level 1 unit after two moves.
    assert_eq!(IceAndFire::events(&trace, 6), ["train level 1"]);
    assert!(IceAndFire::events(&trace, 10_000).is_empty());
}

#[test]
fn buildings_and_large_cuts_are_events() {
    let mut trace = real();
    trace.plies[2].commands = "BUILD MINE 1 0;BUILD TOWER 2 2;MOVE 1 1 1;TRAIN 3 0 1".to_string();
    assert_eq!(
        IceAndFire::events(&trace, 2),
        ["mine", "tower", "train level 3"]
    );
    let moment = |favours| Moment {
        ply: 2,
        kind: "large cut".to_string(),
        favours,
        text: String::new(),
    };
    // Caused by the player to act (red, ply 2 is red's): an event.
    trace.moments = vec![moment(Some(0))];
    assert_eq!(
        IceAndFire::events(&trace, 2).last().map(String::as_str),
        Some("large cut")
    );
    // A cut that red suffered during its own turn is not its event.
    trace.moments = vec![moment(Some(1))];
    assert!(!IceAndFire::events(&trace, 2).contains(&"large cut".to_string()));
}

#[test]
fn spending_counts_the_costs_of_the_commands_carried_out() {
    let mut trace = real();
    trace.plies[2].commands =
        "BUILD MINE 1 0;BUILD TOWER 2 2;MOVE 1 1 1;TRAIN 3 0 1;TRAIN 1 0 2;TRAIN 1 0 3;WAIT"
            .to_string();
    assert_eq!(
        IceAndFire::spending(&trace, 2),
        [
            ("mine".to_string(), 15),
            ("tower".to_string(), 15),
            ("train level 1".to_string(), 20),
            ("train level 3".to_string(), 30),
        ]
    );
    // A level 2 unit costs 20; a ply of moves, or past the end, nothing.
    trace.plies[2].commands = "TRAIN 2 0 1;MOVE 1 2 2".to_string();
    assert_eq!(
        IceAndFire::spending(&trace, 2),
        [("train level 2".to_string(), 20)]
    );
    trace.plies[2].commands = "MOVE 1 2 2".to_string();
    assert!(IceAndFire::spending(&trace, 2).is_empty());
    assert!(IceAndFire::spending(&trace, 10_000).is_empty());
}

#[test]
fn what_counts_as_better_and_where_it_is_compared() {
    for key in ["exposed30", "hq_gap", "upkeep", "cut_off", "units_lost"] {
        assert!(IceAndFire::lower_is_better(key), "{key}");
    }
    for key in ["gold", "income", "cells", "army", "mines", "material"] {
        assert!(!IceAndFire::lower_is_better(key), "{key}");
    }
    let keys: Vec<String> = metrics().into_iter().map(|m| m.key).collect();
    assert!(keys.iter().any(|k| k == IceAndFire::resource()));
    assert_eq!(IceAndFire::compare_turns(), [10, 15]);
    for key in super::LOWER_IS_BETTER {
        assert!(keys.iter().any(|k| k == key), "{key}");
    }
}

/// The hook's total is what the trace says the player spent (its gold before
/// the ply less its gold after), on every ply of a real game.
#[test]
fn the_spending_matches_the_gold_the_engine_takes() {
    let trace = real();
    for (ply, p) in trace.plies.iter().enumerate() {
        let total: i64 = IceAndFire::spending(&trace, ply)
            .iter()
            .map(|(_, cost)| cost)
            .sum();
        assert_eq!(total, p.spent, "ply {ply}: {}", p.commands);
    }
}

#[test]
fn the_economy_hooks_name_metrics_and_categories_that_exist() {
    let keys: Vec<String> = metrics().into_iter().map(|m| m.key).collect();
    let hooks = IceAndFire::economy().expect("hooks");
    let mut wanted = vec![
        hooks.balance,
        hooks.killed,
        hooks.lost,
        hooks.upkeep,
        hooks.gross_income,
        hooks.contact,
        hooks.income,
    ];
    for e in &hooks.efficiency {
        wanted.extend(&e.gained);
    }
    for key in wanted {
        assert!(keys.iter().any(|k| k == key), "{key}");
    }
    // Every category of the spending hook has a role.
    let t = real();
    for ply in 0..t.plies.len() {
        for (category, _) in IceAndFire::spending(&t, ply) {
            assert!(
                hooks
                    .roles
                    .iter()
                    .any(|r| r.categories.contains(&category.as_str())),
                "{category}"
            );
        }
    }
    let role = |name: &str| {
        hooks
            .roles
            .iter()
            .find(|r| r.categories.contains(&name))
            .map(|r| r.name)
    };
    assert_eq!(role("mine"), Some("economy"));
    assert_eq!(role("train level 2"), Some("army"));
    assert_eq!(role("tower"), Some("defence"));
    assert_eq!(hooks.decisive, ["large cut", "HQ threat", "debt"]);
}

#[test]
fn the_spending_a_side_names_in_an_efficiency_is_what_it_paid() {
    // Over a real game, the cumulative spending on every category equals the
    // gold the plies spent (`Ply::spent`).
    let t = real();
    let hooks = IceAndFire::economy().expect("hooks");
    let all = &hooks.efficiency[0].spent;
    for seat in 0..2 {
        let named: i64 = (0..t.plies.len())
            .filter(|&i| t.plies[i].to_act == seat)
            .flat_map(|i| IceAndFire::spending(&t, i))
            .filter(|(c, _)| all.contains(&c.as_str()))
            .map(|(_, a)| a)
            .sum();
        let spent: i64 = t
            .plies
            .iter()
            .filter(|p| p.to_act == seat)
            .map(|p| p.spent)
            .sum();
        assert_eq!(named, spent);
    }
}
