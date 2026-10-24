use std::collections::BTreeMap;

use std::fs;

use super::{load, parse_csv, parse_notes, summarize, Game, Note, SUMMARY_SUFFIX};
use crate::detect::large_drops;
use crate::test_util::game;
use crate::text::metrics_csv;

fn batch_game(cells: &[(i64, i64)], winner: Option<usize>, name: &str) -> Game {
    let mut trace = game(cells, winner);
    trace.name = name.to_string();
    trace.middle = Some(2);
    trace.moments = large_drops(&trace, "cells", 30, 6, "large cut");
    let summary = trace.summary();
    let rows = parse_csv(&metrics_csv(&trace), &summary.metrics).expect("the CSV reads back");
    Game { summary, rows }
}

#[test]
fn the_csv_reads_back_and_bad_files_are_refused() {
    let trace = game(&[(1, 2), (3, 4)], None);
    let keys = vec!["cells".to_string()];
    let rows = parse_csv(&metrics_csv(&trace), &keys).expect("reads");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[1].to_act, 1);
    assert_eq!(rows[1].values, [vec![3], vec![4]]);
    assert!(parse_csv("ply,turn\n", &keys).is_err());
    assert!(parse_csv(
        "ply,turn,to_act,phase,spent,red_cells,blue_cells\n0,1,green,early,0,1,2\n",
        &keys
    )
    .is_err());
}

#[test]
fn the_notes_are_the_last_json_block() {
    let review = "Text.\n```json\n[]\n```\nMore.\n```json\n[{\"ply\": 3, \"side\": \"blue\", \"verdict\": \"bad\", \"note\": \"left a neck\"}]\n```\n";
    let notes = parse_notes(review).expect("reads");
    assert_eq!(
        notes,
        vec![Note {
            ply: 3,
            side: "blue".to_string(),
            verdict: "bad".to_string(),
            note: "left a neck".to_string(),
        }]
    );
    assert!(parse_notes("no block").is_err());
    assert!(parse_notes("```json\n[{\"ply\": 1}]\n```").is_err());
}

#[test]
fn the_summary_counts_winners_moments_and_judgments() {
    // Red wins the first game after cutting blue at ply 2; blue wins the
    // second, level at the middle game.
    let games = vec![
        batch_game(
            &[(10, 20), (10, 20), (11, 20), (11, 12), (13, 12)],
            Some(0),
            "a",
        ),
        batch_game(
            &[(10, 10), (10, 10), (10, 10), (10, 12), (9, 14)],
            Some(1),
            "b",
        ),
    ];
    let mut notes = BTreeMap::new();
    notes.insert(
        "a".to_string(),
        vec![
            // Red's cut: its lead goes from -9 to +1.
            Note {
                ply: 2,
                side: "red".to_string(),
                verdict: "good".to_string(),
                note: String::new(),
            },
            // Blue's ply 1 judged bad: its lead falls from 10 to 1.
            Note {
                ply: 1,
                side: "blue".to_string(),
                verdict: "bad".to_string(),
                note: String::new(),
            },
            Note {
                ply: 1,
                side: "green".to_string(),
                verdict: "bad".to_string(),
                note: String::new(),
            },
            // Blue did not play ply 2.
            Note {
                ply: 2,
                side: "blue".to_string(),
                verdict: "bad".to_string(),
                note: String::new(),
            },
            // Red's last turn: no next turn shows its effect.
            Note {
                ply: 4,
                side: "red".to_string(),
                verdict: "good".to_string(),
                note: String::new(),
            },
        ],
    );
    let text = summarize(&games, &notes, &["x.review.md: no block".to_string()]);
    assert!(
        text.contains("1 judgments of a play in a side's last turn"),
        "{text}"
    );
    assert!(
        text.contains("2 games, 2 decided; 3.0 turns on average."),
        "{text}"
    );
    assert!(text.contains("| alpha | 1 | 0 | 0 |"), "{text}");
    assert!(text.contains("| beta | 0 | 1 | 0 |"), "{text}");
    // At the middle game (ply 2): red behind by 9 in game a and level in b.
    assert!(
        text.contains("### At the start of the middle game (2 games)"),
        "{text}"
    );
    assert!(text.contains("| cells | 0% (0) | 1 | 1 | -4.5 |"), "{text}");
    assert!(
        text.contains("| large cut | 1 | 1 | 100% (1 of 1) |"),
        "{text}"
    );
    assert!(text.contains("| good | all | 1 | +10.0 | 100% |"), "{text}");
    assert!(text.contains("| bad | all | 1 | -9.0 | 100% |"), "{text}");
    assert!(
        text.contains(
            "### Five turns into the middle game (0 games)\n\nNo decided game reached it."
        ),
        "{text}"
    );
    assert!(text.contains("2 judgments named no known side"), "{text}");
    assert!(text.contains("- x.review.md: no block"), "{text}");
}

#[test]
fn a_value_meaning_none_is_left_out_of_the_mean() {
    let mut games = vec![
        batch_game(
            &[(10, 20), (10, 20), (11, 20), (11, 12), (13, 12)],
            Some(0),
            "a",
        ),
        batch_game(
            &[(10, 10), (10, 10), (999, 10), (10, 12), (9, 14)],
            Some(1),
            "b",
        ),
    ];
    for game in &mut games {
        game.summary.none.insert("cells".to_string(), 999);
    }
    let text = summarize(&games, &BTreeMap::new(), &[]);
    // Game b's middle game has red at 999: only game a counts.
    assert!(text.contains("| cells | 0% (0) | 1 | 0 | -9.0 |"), "{text}");
}

#[test]
fn unreadable_files_are_left_out_and_reported() {
    let dir = std::env::temp_dir().join(format!("cg-review-load-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("a scratch directory");
    let mut trace = crate::test_util::game(&[(1, 2), (3, 4)], Some(0));
    trace.name = "good".to_string();
    let summary = serde_json::to_string(&trace.summary()).expect("serializes");
    fs::write(dir.join(format!("good{SUMMARY_SUFFIX}")), &summary).expect("writes");
    fs::write(dir.join("good.csv"), crate::text::metrics_csv(&trace)).expect("writes");
    fs::write(dir.join(format!("bad{SUMMARY_SUFFIX}")), "{}").expect("writes");
    fs::write(dir.join(format!("lonely{SUMMARY_SUFFIX}")), &summary).expect("writes");
    // A record, not a summary: ignored.
    fs::write(dir.join("record.json"), "{}").expect("writes");
    let (games, notes, problems) = load(&dir).expect("reads the directory");
    fs::remove_dir_all(&dir).expect("cleans up");
    assert_eq!(games.len(), 1);
    assert_eq!(games[0].summary.name, "good");
    assert!(notes.is_empty());
    assert_eq!(problems.len(), 2, "{problems:?}");
    assert!(
        problems[0].starts_with("bad.summary.json: "),
        "{problems:?}"
    );
    assert!(problems[1].starts_with("lonely.csv: "), "{problems:?}");
}
