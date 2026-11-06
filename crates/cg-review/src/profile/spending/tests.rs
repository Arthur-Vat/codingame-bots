use crate::profile::{analyze, Input, Options};
use crate::test_util::{game, Toy};

use super::*;

/// A game between alpha (red) and us (blue): red holds 10 and plays `red`
/// in its first ply, blue holds 20 and plays `blue` in its first ply; the
/// second pair of plies is "WAIT" and "".
fn played(id: u64, red: &str, blue: &str, won: bool) -> (GameMeta, Trace) {
    let cells = [(10, 20), (10, 20), (10, 20), (10, 20)];
    let mut trace = game(&cells, Some(usize::from(won)));
    trace.plies[0].commands = red.to_string();
    trace.plies[1].commands = blue.to_string();
    let meta = GameMeta {
        game_id: id,
        kind: "ranking".to_string(),
        batch: None,
        our_seat: Some(1),
    };
    (meta, trace)
}

fn run(games: Vec<(GameMeta, Trace)>) -> Spending {
    let input = Input {
        games,
        ..Input::default()
    };
    analyze::<Toy>(
        &input,
        &Options {
            top: 0,
            players: vec!["alpha".to_string()],
        },
    )
    .spending
}

#[test]
fn spending_is_averaged_per_own_turn_by_category_and_group() {
    // Red builds once (2) in its first turn; blue builds twice and trains
    // once (5) in its first turn and nothing in its second.
    let s = run(vec![
        played(1, "B1", "B1;B2;T1;x", true),
        played(2, "x", "x", false),
    ]);
    assert_eq!(s.categories, ["build", "train"]);
    let names: Vec<&str> = s.groups.iter().map(|g| g.label.as_str()).collect();
    assert_eq!(
        names,
        ["top players", "us, same games", "our wins", "our losses"]
    );
    let (top, us, wins, losses) = (&s.groups[0], &s.groups[1], &s.groups[2], &s.groups[3]);
    assert_eq!(
        (top.games, us.games, wins.games, losses.games),
        (2, 2, 1, 1)
    );
    let early = |g: &SpendGroup| g.by_phase[0].clone();
    assert_eq!(early(top).plies, 4);
    // Red: 2 gold in one of four own turns, so 0.5 on builds.
    assert_eq!(early(top).spent, [0.5, 0.0]);
    assert_eq!(early(us).spent, [1.0, 0.25]);
    assert_eq!(early(wins).spent, [2.0, 0.5]);
    assert_eq!(early(losses).spent, [0.0, 0.0]);
    // The middle game and the endgame were never reached.
    assert_eq!(top.by_phase[1].plies, 0);
    assert_eq!(top.by_phase[1].kept, None);
}

#[test]
fn the_kept_share_is_the_mean_of_each_turn_s_share_left() {
    // Blue holds 20: spends 5 in turn 1 (75% kept), nothing in turn 2
    // (100%): 87.5. Red holds 10: spends 2 (80%), then nothing: 90.
    let s = run(vec![played(1, "B1", "B1;B2;T1", true)]);
    assert_eq!(s.groups[1].by_phase[0].kept, Some(87.5));
    assert_eq!(s.groups[0].by_phase[0].kept, Some(90.0));
    // Each focus player apart, in the games against us.
    assert_eq!(s.players.len(), 1);
    assert_eq!(s.players[0].label, "alpha");
    assert_eq!(s.players[0].games, 1);
    assert_eq!(s.players[0].by_phase[0].kept, Some(90.0));
}

#[test]
fn nothing_to_spend_or_no_games_is_said_with_empty_tables() {
    let s = run(vec![played(1, "x", "x", true)]);
    assert!(s.categories.is_empty());
    assert_eq!(s.groups[0].by_phase[0].kept, Some(100.0));
    let none = run(Vec::new());
    assert!(none.categories.is_empty());
    assert_eq!(none.groups[0].games, 0);
    assert_eq!(none.players[0].games, 0);
    assert_eq!(none.players[0].by_phase[0].kept, None);
}
