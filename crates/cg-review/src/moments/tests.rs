use std::collections::BTreeMap;

use crate::profile::{GameMeta, Input, Options};
use crate::test_util::game;
use crate::trace::{Moment, Trace};

use super::*;

/// A made-up game: the decisive kinds are `cut` then `threat`, the exposure
/// is the `cells` metric.
struct Toy;

impl Profiling for Toy {
    fn turns() -> Vec<u32> {
        vec![5]
    }
    fn opening_plies() -> usize {
        1
    }
    fn key_metrics() -> Vec<&'static str> {
        vec!["cells"]
    }
    fn events(_: &Trace, _: usize) -> Vec<String> {
        Vec::new()
    }
    fn opening(_: &Trace, _: usize, _: usize) -> Vec<String> {
        Vec::new()
    }
}

impl Decisive for Toy {
    fn decisive_kinds() -> Vec<&'static str> {
        vec!["cut", "threat"]
    }
    fn exposure_metric() -> Option<&'static str> {
        Some("cells")
    }
    fn exposure_thresholds() -> Vec<i64> {
        vec![13, 15]
    }
}

fn moment(ply: usize, kind: &str, favours: usize) -> Moment {
    Moment {
        ply,
        kind: kind.to_string(),
        favours: Some(favours),
        text: format!("{kind} at {ply}"),
    }
}

/// Red (alpha) beats blue (beta, us) in a game of 8 plies; at ply i red has
/// 20 + i cells and blue 10 + i.
fn lost(id: u64, moments: Vec<Moment>) -> (GameMeta, Trace) {
    let cells: Vec<(i64, i64)> = (0..8).map(|i| (20 + i, 10 + i)).collect();
    let mut trace = game(&cells, Some(0));
    trace.name = id.to_string();
    trace.moments = moments;
    for (i, p) in trace.plies.iter_mut().enumerate() {
        p.board = vec![format!("board {i}")];
    }
    let meta = GameMeta {
        game_id: id,
        kind: "ranking".to_string(),
        batch: None,
        our_seat: Some(1),
    };
    (meta, trace)
}

fn run(games: Vec<(GameMeta, Trace)>) -> Findings {
    let input = Input {
        games,
        ranks: BTreeMap::from([("alpha".to_string(), 1)]),
        ..Input::default()
    };
    let options = Options {
        top: 1,
        players: Vec::new(),
    };
    analyze::<Toy>(&input, &options, 2)
}

#[test]
fn the_first_decisive_moment_against_us_is_found() {
    let moments = vec![
        moment(0, "other", 0),
        moment(2, "cut", 1),
        moment(6, "threat", 0),
        moment(4, "cut", 0),
    ];
    let f = run(vec![lost(1, moments)]);
    let found = f.losses[0].found.clone().expect("a moment");
    assert_eq!((found.kind.as_str(), found.ply, found.turn), ("cut", 4, 3));
    assert_eq!(found.before, ["board 4"]);
    assert_eq!(found.after, Some(vec!["board 5".to_string()]));
    assert_eq!(found.commands, "WAIT");
    assert!(f.losses[0].focus);
    assert_eq!(f.losses[0].rank, Some(1));
}

#[test]
fn exposure_and_metrics_are_read_after_the_last_own_plies() {
    let f = run(vec![lost(1, vec![moment(6, "cut", 0)])]);
    let found = f.losses[0].found.clone().expect("a moment");
    // Blue played plies 5, 3 and 1; the snapshots after them are plies 6,
    // 4 and 2, where blue's cells are 16, 14 and 12.
    assert_eq!(found.exposure, [Some(16), Some(14), Some(12)]);
    assert_eq!(found.ours, [Some(16)]);
    // Red's last ply is 4, read in ply 5: 25.
    assert_eq!(found.theirs, [Some(25)]);
}

#[test]
fn a_moment_on_the_first_ply_has_nothing_before_it() {
    let f = run(vec![lost(1, vec![moment(0, "threat", 0)])]);
    let found = f.losses[0].found.clone().expect("a moment");
    assert_eq!(found.exposure, [None, None, None]);
    assert_eq!((found.ours, found.theirs), (vec![None], vec![None]));
    let text = markdown(&f);
    assert!(text.contains("oldest first: - -> - -> -"), "{text}");
}

#[test]
fn a_moment_on_the_last_ply_has_no_board_after() {
    // The last ply of a game of 7 plies is the opponent's.
    let (meta, mut trace) = lost(1, vec![moment(6, "cut", 0)]);
    trace.plies.truncate(7);
    let f = run(vec![(meta, trace)]);
    let found = f.losses[0].found.clone().expect("a moment");
    assert_eq!(found.after, None);
    assert!(markdown(&f).contains("No board after"));
}

#[test]
fn a_loss_without_a_decisive_moment_is_counted() {
    let f = run(vec![
        lost(1, vec![moment(2, "cut", 1), moment(3, "other", 0)]),
        lost(2, vec![moment(4, "cut", 0)]),
    ]);
    assert!(f.losses[0].found.is_none());
    let text = markdown(&f);
    assert!(text.contains("1 losses have one and 1 have none"), "{text}");
    assert!(
        text.contains("| no decisive moment | 1 | - | 0 | - |"),
        "{text}"
    );
    assert!(text.contains("| cut | 1 | 3 | 0 | - |"), "{text}");
}

#[test]
fn draws_and_games_not_ours_are_left_out() {
    let (meta, mut drawn) = lost(1, vec![moment(4, "cut", 0)]);
    drawn.winner = None;
    let (mut other, trace) = lost(2, vec![moment(4, "cut", 0)]);
    other.our_seat = None;
    let f = run(vec![(meta, drawn), (other, trace)]);
    assert!(f.losses.is_empty());
    assert_eq!(f.games, 2);
}

#[test]
fn examples_are_limited_and_tables_are_narrow() {
    let games: Vec<_> = (1..=4)
        .map(|i| lost(i, vec![moment(4, "cut", 0)]))
        .collect();
    let text = markdown(&run(games));
    assert_eq!(text.matches("#### Game").count(), 2, "{text}");
    assert!(text.contains("### alpha (rank 1, 4 losses, 4 with a decisive moment)"));
    assert!(text.contains("```\nboard 4\n```"), "{text}");
    for line in text.lines().filter(|l| l.starts_with('|')) {
        assert!(line.matches(" | ").count() < 6, "{line}");
    }
    // Blue had two plies before ply 4: exposures 14 and 12, none for the third.
    assert!(
        text.contains(
            "| cut, losses, focus | 4 | 14 | 100% (4/4) [n = 4] | 0% (0/4) [n = 4] | - (0) |"
        ),
        "{text}"
    );
}

#[test]
fn wins_that_reached_the_turn_of_each_moment_are_the_control() {
    let (meta, mut won) = lost(9, Vec::new());
    won.winner = Some(1);
    // A win too short to reach turn 3 is left out of that moment's control.
    let (short_meta, mut short) = lost(10, Vec::new());
    short.winner = Some(1);
    short.plies.truncate(3);
    let f = run(vec![
        lost(1, vec![moment(4, "cut", 0)]),
        lost(2, vec![moment(2, "cut", 0)]),
        (meta, won),
        (short_meta, short),
    ]);
    assert_eq!(f.controls.len(), 1);
    let cases: Vec<(u64, u32)> = f.controls[0]
        .cases
        .iter()
        .map(|c| (c.game_id, c.turn))
        .collect();
    // Turn 3 (loss 1): the long win only. Turn 2 (loss 2): both wins.
    assert_eq!(cases, [(9, 3), (9, 2), (10, 2)]);
    assert_eq!(
        f.controls[0].exposures(None),
        [
            [Some(14), Some(12), None],
            [Some(12), None, None],
            [Some(12), None, None]
        ]
    );
    assert_eq!(f.controls[0].exposures(Some(false)).len(), 0);
    let text = markdown(&f);
    assert!(text.contains("### At or above 13"), "{text}");
    assert!(
        text.contains(
            "| cut, wins, focus | 3 | 12 | 33% (1/3) [n = 3] | 0% (0/1) [n = 1] | - (0) |"
        ),
        "{text}"
    );
    assert!(
        text.contains("| cut, losses, others | 0 | - | - (0) | - (0) | - (0) |"),
        "{text}"
    );
}

#[test]
fn a_moment_on_our_own_ply_is_not_decisive() {
    // Ply 3 and 5 are blue's (ours): a moment recorded there is left out.
    let f = run(vec![lost(
        1,
        vec![moment(3, "cut", 0), moment(5, "threat", 0)],
    )]);
    assert!(f.losses[0].found.is_none());
    let f = run(vec![lost(
        1,
        vec![moment(3, "cut", 0), moment(6, "threat", 0)],
    )]);
    let found = f.losses[0].found.clone().expect("a moment");
    assert_eq!((found.kind.as_str(), found.ply), ("threat", 6));
}

#[test]
fn files_hold_the_numbers_the_examples_and_a_readme() {
    let (meta, mut won) = lost(9, Vec::new());
    won.winner = Some(1);
    let f = run(vec![lost(1, vec![moment(4, "cut", 0)]), (meta, won)]);
    let files = files(&f);
    let names: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(
        names,
        ["moments.csv", "summary.csv", "examples.md", "README.md"]
    );
    let get = |name: &str| files.iter().find(|(n, _)| n == name).unwrap().1.clone();
    let moments = get("moments.csv");
    let lines: Vec<&str> = moments.lines().collect();
    assert_eq!(
        lines[0],
        "group,game_id,opponent,rank,focus,our_seat,kind,ply,turn,theirs_cells,ours_cells,exposure_1,exposure_2,exposure_3"
    );
    assert_eq!(lines[1], "loss,1,alpha,1,true,blue,cut,4,3,23,14,14,12,");
    assert_eq!(lines[2], "win_control,9,alpha,1,true,blue,cut,4,3,,,14,12,");
    let summary = get("summary.csv");
    assert!(
        summary.contains("losses_by_kind,focus,cut,,,count,1\n"),
        "{summary}"
    );
    assert!(
        summary.contains("exposure,losses_focus,cut,13,1,share_pct,100.0\n"),
        "{summary}"
    );
    assert!(
        summary.contains("exposure,wins_matched_focus,cut,15,1,at_or_above,0\n"),
        "{summary}"
    );
    assert!(get("examples.md").contains("board 4"));
    assert!(get("README.md").contains("exposure_1"));
}
