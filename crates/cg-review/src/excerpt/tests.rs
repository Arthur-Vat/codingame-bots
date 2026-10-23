use std::fs;

use super::{
    excerpt_summary, excerpt_text, games_row, parse_excerpt_notes, parse_games, primer_text,
    window, GAMES_HEADER,
};
use crate::test_util::game;
use crate::trace::{Moment, Trace};

fn moment(ply: usize, kind: &str, favours: usize) -> Moment {
    Moment {
        ply,
        kind: kind.to_string(),
        favours: Some(favours),
        text: format!("{kind} at {ply}"),
    }
}

/// A game of 12 plies, red winning.
fn twelve() -> Trace {
    let cells: Vec<(i64, i64)> = (0..12).map(|i| (i, 12 - i)).collect();
    game(&cells, Some(0))
}

#[test]
fn the_window_ends_with_the_play_that_caused_a_large_cut() {
    let mut trace = twelve();
    trace.moments = vec![moment(7, "large cut", 0)];
    let w = window(&trace, 3).unwrap();
    assert_eq!((w.from, w.last), (2, 7));
    assert_eq!(w.moment.unwrap().ply, 7);
}

#[test]
fn a_lead_change_ends_the_window_one_ply_before_it_shows() {
    let mut trace = twelve();
    trace.moments = vec![moment(6, "lead change", 1)];
    let w = window(&trace, 3).unwrap();
    assert_eq!((w.from, w.last), (0, 5));
}

#[test]
fn a_large_cut_is_preferred_to_an_earlier_lead_change() {
    let mut trace = twelve();
    trace.moments = vec![
        moment(4, "lead change", 1),
        moment(5, "swing", 0),
        moment(9, "large cut", 0),
    ];
    let w = window(&trace, 2).unwrap();
    assert_eq!((w.from, w.last), (6, 9));
    assert_eq!(w.moment.unwrap().kind, "large cut");
}

#[test]
fn without_a_decisive_moment_the_window_is_the_game_end() {
    let mut trace = twelve();
    trace.moments = vec![moment(5, "HQ threat", 0)];
    let w = window(&trace, 3).unwrap();
    assert_eq!((w.from, w.last), (6, 11));
    assert!(w.moment.is_none());
    assert!(window(&game(&[], None), 3).is_none());
}

#[test]
fn the_excerpt_shows_the_window_then_the_position_after_it() {
    let mut trace = twelve();
    trace.moments = vec![moment(7, "large cut", 0)];
    let text = excerpt_text(&trace, 3);
    assert!(text.starts_with("# Excerpt g1: red alpha against blue beta\n"));
    assert!(text.contains("**Result:** alpha (red) won; finished; 12 plies."));
    assert!(text.contains(
        "**Decisive moment:** Ply 7 · turn 4 · blue to play · early game (large cut, favours red): large cut at 7."
    ));
    assert!(text.contains("**Shown:** plies 2 to 7, each side's last 3 turns"));
    assert!(!text.contains("### Ply 1 ·"));
    assert!(text.contains("### Ply 2 ·"));
    assert!(text.contains("### Ply 7 ·"));
    let after = &text[text.find("## The position after it").unwrap()..];
    assert!(after.contains("### Ply 8 ·"));
    assert!(!after.contains("played"));
    assert!(!text.contains("### Ply 9 ·"));
    // The primer is not repeated in each excerpt.
    assert!(!text.contains("Red is R"));
    assert!(primer_text(&trace).contains("- Red is R, blue is B."));
    assert!(primer_text(&trace).contains("| cells | Cells held. |"));
}

#[test]
fn a_games_row_reads_back() {
    let mut trace = twelve();
    trace.moments = vec![moment(7, "large cut", 0)];
    let row = games_row(&trace, 3);
    assert_eq!(row, "g1,alpha,beta,red,12,large cut,7,red,2,7");
    let quiet = games_row(&game(&[(1, 1), (1, 2)], None), 3);
    assert_eq!(quiet, "g1,alpha,beta,draw,2,,,,0,1");
    let rows = parse_games(&format!("{GAMES_HEADER}\n{row}\n{quiet}\n")).unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].winner, Some(0));
    assert_eq!(rows[0].favours, Some(0));
    assert_eq!(rows[1].winner, None);
    assert_eq!(rows[1].moment, "");
    assert!(parse_games("game,red\n").is_err());
}

#[test]
fn notes_are_the_last_json_block() {
    let text = "Some words.\n```json\n[]\n```\nMore.\n```json\n[{\"game\":\"g1\",\"side\":\"blue\",\
                \"verdict\":\"bad\",\"ply\":7,\"category\":\"open link\",\"note\":\"Left (5,4) empty.\"}]\n```\n";
    let notes = parse_excerpt_notes(text).unwrap();
    assert_eq!(notes.len(), 1);
    assert_eq!(notes[0].category, "open link");
    assert!(parse_excerpt_notes("no block").is_err());
    assert!(parse_excerpt_notes("```json\n[").is_err());
}

#[test]
fn the_counts_name_the_bot_of_each_judgment_and_the_problems() {
    let dir = std::env::temp_dir().join(format!("cg-review-excerpts-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("games.csv"),
        format!(
            "{GAMES_HEADER}\ng1,alpha,beta,red,12,large cut,7,red,2,7\n\
             g2,beta,alpha,red,10,lead change,6,blue,0,5\ng3,alpha,beta,blue,8,,,,2,7\n"
        ),
    )
    .unwrap();
    fs::write(
        dir.join("notes-01.md"),
        "```json\n[\
         {\"game\":\"g1\",\"side\":\"blue\",\"verdict\":\"bad\",\"ply\":7,\"category\":\"open link\",\"note\":\"a\"},\
         {\"game\":\"g2\",\"side\":\"red\",\"verdict\":\"bad\",\"ply\":5,\"category\":\"open link\",\"note\":\"b\"},\
         {\"game\":\"g2\",\"side\":\"blue\",\"verdict\":\"good\",\"ply\":4,\"category\":\"cut\",\"note\":\"c\"},\
         {\"game\":\"g9\",\"side\":\"red\",\"verdict\":\"bad\",\"ply\":1,\"category\":\"other\",\"note\":\"d\"}\
         ]\n```\n",
    )
    .unwrap();
    fs::write(dir.join("notes-02.md"), "no json here").unwrap();
    let text = excerpt_summary(&dir).unwrap();
    fs::remove_dir_all(&dir).unwrap();
    assert!(text.contains("3 games, 4 notes from 2 files."));
    assert!(text.contains("| alpha | 3 | 1 | 1 | 0 |"));
    assert!(text.contains("| beta | 3 | 2 | 1 | 1 |"));
    assert!(text.contains("| large cut | 1 | 100% |"));
    assert!(text.contains("| lead change | 1 | 0% |"));
    assert!(text.contains("| none (last plies) | 1 | 0% |"));
    // g1's blue is beta; g2's red is beta and its blue alpha.
    assert!(text.contains("| beta | bad | open link | 2 |"));
    assert!(text.contains("| alpha | good | cut | 1 |"));
    assert!(text.contains("notes-02.md: no fenced json block"));
    assert!(text.contains("unknown game `g9`"));
    assert!(text.contains("1 games have no note: g3"));
}
