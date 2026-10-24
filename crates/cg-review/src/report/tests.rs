use std::collections::BTreeMap;

use super::{curves, first_telling, report};
use crate::summary::{Game, Row};
use crate::test_util::game;
use crate::trace::Trace;

/// The batch game of `trace`.
fn batch_game(trace: &Trace) -> Game {
    Game {
        summary: trace.summary(),
        rows: trace
            .plies
            .iter()
            .map(|p| Row {
                to_act: p.to_act,
                values: p.metrics.clone(),
            })
            .collect(),
    }
}

/// `n` games of 2 turns between alpha (red) and beta (blue), red winning
/// with more cells from turn 2 in all of them.
fn batch(n: usize) -> Vec<Game> {
    (0..n)
        .map(|_| batch_game(&game(&[(1, 1), (1, 1), (5, 2), (5, 2)], Some(0))))
        .collect()
}

#[test]
fn a_metric_tells_the_winner_from_the_turn_it_separates_the_sides() {
    let games = batch(40);
    let c = curves(&games);
    assert_eq!(c.bots, ["alpha", "beta"]);
    assert_eq!(c.metrics, ["cells"]);
    let turn1 = &c.points[0][0];
    assert_eq!((turn1.games, turn1.higher, turn1.equal), (40, 0, 40));
    // Ties count half: no side is ahead.
    assert_eq!(turn1.higher_share(), Some(50));
    assert_eq!(turn1.strength(), 0);
    let turn2 = &c.points[0][1];
    assert_eq!((turn2.higher, turn2.lower), (40, 0));
    assert_eq!(turn2.higher_share(), Some(100));
    assert_eq!(turn2.strength(), 50);
    assert_eq!(turn2.winner_mean, Some(5.0));
    assert_eq!(turn2.loser_mean, Some(2.0));
    assert_eq!(turn2.bot_means, vec![Some(5.0), Some(2.0)]);
    // The games are over at turn 3.
    assert_eq!(c.points[0][2].games, 0);
    assert_eq!(first_telling(&c.points[0]), Some((2, 100)));
    // Too few games: it tells nothing.
    assert_eq!(first_telling(&curves(&batch(10)).points[0]), None);
}

#[test]
fn draws_count_for_the_bots_means_but_not_for_the_winner() {
    let mut games = batch(30);
    games.push(batch_game(&game(&[(9, 9), (9, 9)], None)));
    let c = curves(&games);
    assert_eq!(c.points[0][0].games, 30);
    assert_eq!(c.points[0][0].bot_means[0], Some(39.0 / 31.0));
}

#[test]
fn a_none_value_is_left_out() {
    let mut trace = game(&[(999, 1), (999, 1), (3, 1), (3, 1)], Some(1));
    trace.metrics[0].none = Some(999);
    let c = curves(&[batch_game(&trace)]);
    assert_eq!(c.points[0][0].games, 0);
    assert_eq!(c.points[0][0].bot_means, vec![None, Some(1.0)]);
    assert_eq!(c.points[0][1].games, 1);
}

#[test]
fn the_report_has_the_results_the_telling_turns_and_the_bots_compared() {
    let meanings = BTreeMap::from([("cells".to_string(), "Cells held.".to_string())]);
    let text = report(&batch(40), &meanings);
    assert!(text.starts_with("# Metrics report\n\n40 games, 40 decided; 1 metrics a side"));
    assert!(text.contains("| alpha | 40 | 40 | 40 | 0 |"));
    assert!(text.contains("| beta | 40 | 0 | 0 | 0 |"));
    assert!(text.contains("Median length: 2 turns."));
    assert!(text.contains("| cells | 2 (100%) |"));
    assert!(text.contains("| Cells held. |"));
    assert!(text.contains("## alpha against beta"));
}

/// A game of `turns` turns between `red` and `blue`, `winner` winning,
/// red's cells `red_cells(turn)` and blue's 10 at each turn.
fn long_game(turns: usize, players: [&str; 2], winner: usize, red_cells: fn(usize) -> i64) -> Game {
    let cells: Vec<(i64, i64)> = (0..2 * turns)
        .map(|ply| (red_cells(ply / 2 + 1), 10))
        .collect();
    let mut trace = game(&cells, Some(winner));
    trace.players = [players[0].to_string(), players[1].to_string()];
    batch_game(&trace)
}

#[test]
fn turns_before_the_end_line_games_up_on_their_last_turn() {
    // Red has 5 cells at its last turn, blue 2; one turn before, 1 and 1.
    let games = [batch_game(&game(
        &[(1, 1), (1, 1), (5, 2), (5, 2)],
        Some(0),
    ))];
    let refs: Vec<&Game> = games.iter().collect();
    let metrics = vec!["cells".to_string()];
    let bots = vec!["alpha".to_string(), "beta".to_string()];
    let end = super::points_of(&refs, &metrics, &bots, super::Align::End);
    assert_eq!(end[0][0].turn, 0);
    assert_eq!(end[0][0].winner_mean, Some(5.0));
    assert_eq!(end[0][1].winner_mean, Some(1.0));
    assert_eq!(end[0][2].games, 0);
    let start = super::points_of(&refs, &metrics, &bots, super::Align::Start);
    assert_eq!(start[0][0].turn, 1);
    assert_eq!(start[0][0].winner_mean, Some(1.0));
}

#[test]
fn the_slices_cut_by_length_seats_winner_and_lead() {
    // Lengths 10, 12 and 14 turns; alpha red in the first two. Red leads at
    // turn 10 in the first and third games (11 cells against 10), blue in
    // the second (9 against 10).
    let games = [
        long_game(10, ["alpha", "beta"], 0, |_| 11),
        long_game(12, ["alpha", "beta"], 0, |_| 9),
        long_game(14, ["beta", "alpha"], 1, |_| 11),
    ];
    let bots = super::bots_of(&games);
    let s = super::slices(&games, &bots);
    let get = |name: &str| s.iter().find(|x| x.name == name).map(|x| x.games.clone());
    assert_eq!(get("All games"), Some(vec![0, 1, 2]));
    // One game a third, by rank.
    assert_eq!(get("Short (10 turns)"), Some(vec![0]));
    assert_eq!(get("Regular (12 turns)"), Some(vec![1]));
    assert_eq!(get("Long (14 turns)"), Some(vec![2]));
    assert_eq!(get("alpha red, beta blue"), Some(vec![0, 1]));
    assert_eq!(get("beta red, alpha blue"), Some(vec![2]));
    assert_eq!(get("alpha won"), Some(vec![0, 1, 2]));
    assert_eq!(get("beta won"), Some(vec![]));
    // The test games' lead metric is cells.
    assert_eq!(get("Leader at turn 10 won"), Some(vec![0]));
    assert_eq!(get("Leader at turn 10 lost"), Some(vec![1, 2]));
    let all = super::sliced(&games);
    assert_eq!(all.slices.len(), s.len());
    assert_eq!(all.slices[0].start[0].len(), super::MAX_TURN);
}

#[test]
fn the_report_has_its_slices() {
    let meanings = BTreeMap::new();
    let text = report(&batch(40), &meanings);
    assert!(text.contains("## How the batch splits"));
    assert!(text.contains("| All games | 40 | 40 | 0 | 2 |"));
    assert!(text.contains("## When each metric tells the winner, by slice"));
    assert!(text.contains("## 5 turns before the end, by slice"));
    assert!(text.contains("## 1 turn before the end, by slice"));
    assert!(text.contains("## alpha red, beta blue: the two bots at turn 10"));
}
