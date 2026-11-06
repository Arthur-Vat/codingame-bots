use crate::profile::{analyze, GameMeta, Input, Options};
use crate::test_util::{game, Toy};

use super::*;

/// A game of `turns` turns where we (blue) have `blue(i)` at ply `i`, the
/// value read at turn `t` being `blue(2 t)` (the next ply shows what our
/// play left), or `blue(2 t - 1)` at the last turn.
fn ours(id: u64, turns: usize, blue: impl Fn(usize) -> i64, won: bool) -> (GameMeta, Trace) {
    let cells: Vec<(i64, i64)> = (0..2 * turns).map(|i| (7, blue(i))).collect();
    let mut trace = game(&cells, Some(if won { 1 } else { 0 }));
    trace.name = id.to_string();
    let meta = GameMeta {
        game_id: id,
        kind: "ranking".to_string(),
        batch: None,
        our_seat: Some(1),
    };
    (meta, trace)
}

fn run(games: Vec<(GameMeta, Trace)>, players: &[&str]) -> Curves {
    let input = Input {
        games,
        ..Input::default()
    };
    analyze::<Toy>(
        &input,
        &Options {
            top: 0,
            players: players.iter().map(|p| (*p).to_string()).collect(),
        },
    )
    .curves
}

#[test]
fn wins_and_losses_separate_from_the_turn_the_wins_pull_ahead() {
    // Wins: 0 at ply 2 (read at turn 1), 5 from ply 4 on. Losses: always 0.
    let wins = (0..5).map(|i| ours(i, 3, |p| if p >= 4 { 5 } else { 0 }, true));
    let losses = (5..10).map(|i| ours(i, 3, |_| 0, false));
    let curves = run(wins.chain(losses).collect(), &[]);
    let set = &curves.wins_vs_losses;
    assert_eq!(set.labels, ["our wins", "our losses"]);
    assert_eq!(set.games, [5, 5]);
    assert_eq!(set.last_turn, 3);
    let c = &set.curves[0];
    assert_eq!(c.key, "cells");
    assert_eq!(c.meaning, "Cells held.");
    assert_eq!(c.auc, [Some(0.5), Some(1.0), Some(1.0)]);
    assert_eq!(c.separates_from, Some(2));
    let medians: Vec<Option<f64>> = c.lines[0].iter().map(|p| p.median).collect();
    assert_eq!(medians, [Some(0.0), Some(5.0), Some(5.0)]);
    assert_eq!(c.lines[1][2].games, 5);
    // The turns are compared in the order of the ranking.
    let ranked = set.compare(&[2, 3, 9]);
    assert_eq!(ranked.rows[0].at.len(), 2, "turn 9 is not reached");
    assert_eq!(ranked.rows[0].score, Some(1.0));
    assert_eq!(ranked.rows[0].at[1].first, Some(5.0));
}

#[test]
fn the_last_turn_is_the_last_that_half_of_the_games_reach() {
    // One win of 3 turns and three losses of 1 turn: turn 2 has 1 of 4.
    let mut games = vec![ours(1, 3, |_| 1, true)];
    games.extend((2..5).map(|i| ours(i, 1, |_| 1, false)));
    let set = run(games, &[]).wins_vs_losses;
    assert_eq!(set.last_turn, 1);
    // Two of four reach turn 2 and 3: kept.
    let games = vec![
        ours(1, 3, |_| 1, true),
        ours(2, 3, |_| 1, false),
        ours(3, 1, |_| 1, false),
        ours(4, 1, |_| 1, true),
    ];
    assert_eq!(run(games, &[]).wins_vs_losses.last_turn, 3);
}

#[test]
fn no_win_or_no_loss_leaves_the_auc_empty() {
    let games: Vec<_> = (0..6).map(|i| ours(i, 2, |_| 3, false)).collect();
    let set = run(games, &[]).wins_vs_losses;
    assert_eq!(set.games, [0, 6]);
    let c = &set.curves[0];
    assert!(c.auc.iter().all(Option::is_none));
    assert_eq!(c.separates_from, None);
    assert_eq!(c.lines[0][0].median, None);
    assert_eq!(c.lines[1][0].median, Some(3.0));
    assert_eq!(set.compare(&[1, 2]).rows[0].score, None);
    // Not a single game: empty set, no panic.
    let empty = run(Vec::new(), &[]);
    assert_eq!(empty.wins_vs_losses.last_turn, 0);
    assert!(empty.top_vs_us.curves.is_empty());
}

#[test]
fn small_groups_have_no_auc() {
    let few = [1.0, 2.0, 3.0, 4.0];
    let many = [1.0; 6];
    assert_eq!(auc(&few, &many), None);
    assert_eq!(auc(&many, &few), None);
    let five = [5.0; 5];
    assert_eq!(auc(&five, &many), Some(1.0));
    assert_eq!(auc(&many, &five), Some(0.0));
    assert_eq!(auc(&many, &[1.0; 5]), Some(0.5));
    assert_eq!(auc(&[1.0, 3.0, 4.0, 5.0, 7.0], &[4.0; 5]), Some(0.5));
}

#[test]
fn less_is_better_reads_the_auc_the_other_way() {
    assert_eq!(oriented(0.8, false), 0.8);
    assert!((oriented(0.8, true) - 0.2).abs() < 1e-12);
}

#[test]
fn the_groups_separate_for_good_from_the_last_unbroken_run() {
    let s = |v: &[Option<f64>], either| separates_from(v, either);
    // Apart from turn 3 on; the dip at turn 2 ends the run.
    let a = [Some(0.9), Some(0.5), Some(0.8), Some(0.75)];
    assert_eq!(s(&a, false), Some(3));
    // A turn without an AUC does not break it.
    let b = [Some(0.8), None, Some(0.9)];
    assert_eq!(s(&b, false), Some(1));
    // Not apart at the end.
    assert_eq!(s(&[Some(0.9), Some(0.6)], false), None);
    assert_eq!(s(&[], false), None);
    assert_eq!(s(&[None, None], true), None);
    // The wrong way only counts when both ways do.
    let c = [Some(0.2), Some(0.1), Some(0.25)];
    assert_eq!(s(&c, false), None);
    assert_eq!(s(&c, true), Some(1));
    // Mixed ways: the last decides, the other way breaks the run.
    let d = [Some(0.1), Some(0.8), Some(0.9)];
    assert_eq!(s(&d, true), Some(2));
    assert_eq!(s(&[Some(0.7), Some(0.3)], true), Some(2));
}

#[test]
fn the_top_players_are_compared_with_us_in_their_games() {
    // Red (alpha, a focus player) holds 7 all game; we hold 3 and win.
    let games: Vec<_> = (0..5).map(|i| ours(i, 2, |_| 3, true)).collect();
    let curves = run(games, &["alpha"]);
    let set = &curves.top_vs_us;
    assert_eq!(set.labels, ["us", "top players"]);
    assert_eq!(set.games, [5, 5]);
    assert!(set.either);
    let c = &set.curves[0];
    assert_eq!(c.lines[0][0].median, Some(3.0));
    assert_eq!(c.lines[1][0].median, Some(7.0));
    // We are never better: AUC 0, apart in the other direction from turn 1.
    assert_eq!(c.auc, [Some(0.0), Some(0.0)]);
    assert_eq!(c.separates_from, Some(1));
    // Without a focus player there is nothing to compare.
    let none = run(vec![ours(1, 2, |_| 3, true)], &[]);
    assert_eq!(none.top_vs_us.games, [0, 0]);
}

#[test]
fn the_ranking_puts_the_strongest_first_and_missing_scores_last() {
    let curve = |key: &str, auc: [Option<f64>; 2]| Curve {
        key: key.to_string(),
        meaning: String::new(),
        lower_is_better: false,
        rate: false,
        lines: [
            vec![Point::new(1, 6, Some(1.0)), Point::new(2, 6, Some(1.0))],
            vec![Point::new(1, 6, Some(1.0)), Point::new(2, 6, Some(1.0))],
        ],
        auc: auc.to_vec(),
        separates_from: None,
    };
    let mut set = CurveSet {
        slug: "s".to_string(),
        labels: ["a".to_string(), "b".to_string()],
        games: [6, 6],
        last_turn: 2,
        either: false,
        events: Vec::new(),
        curves: vec![
            curve("none", [None, None]),
            curve("weak", [Some(0.5), Some(0.6)]),
            curve("strong", [Some(0.8), Some(0.9)]),
            curve("reversed", [Some(0.1), Some(0.1)]),
        ],
    };
    let keys = |set: &CurveSet| -> Vec<String> {
        set.compare(&[1, 2])
            .rows
            .into_iter()
            .map(|r| r.key)
            .collect()
    };
    assert_eq!(keys(&set), ["strong", "weak", "reversed", "none"]);
    // When either side counts, the reversed one is strong.
    set.either = true;
    assert_eq!(keys(&set), ["reversed", "strong", "weak", "none"]);
}
