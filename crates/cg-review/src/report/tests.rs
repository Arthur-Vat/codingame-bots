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
