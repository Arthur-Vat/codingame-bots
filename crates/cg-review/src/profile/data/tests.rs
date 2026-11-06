use crate::profile::{analyze, data_files, GameMeta, Input, Options};
use crate::test_util::{game, Toy};

use super::*;

/// A game of two turns in which we (blue) hold 4 and build in our first
/// ply (the toy's event "build"), against red's 9.
fn built(id: u64, won: bool) -> (GameMeta, Trace) {
    let mut trace = game(&[(9, 4); 4], Some(usize::from(won)));
    trace.plies[1].commands = "B1".to_string();
    let meta = GameMeta {
        game_id: id,
        kind: "ranking".to_string(),
        batch: None,
        our_seat: Some(1),
    };
    (meta, trace)
}

fn input(wins: u64, losses: u64) -> Input {
    let games = (0..wins)
        .map(|i| built(i, true))
        .chain((0..losses).map(|i| built(100 + i, false)))
        .collect();
    Input {
        games,
        ..Input::default()
    }
}

fn options(players: &[&str]) -> Options {
    Options {
        top: 0,
        players: players.iter().map(|p| (*p).to_string()).collect(),
    }
}

fn file(files: &[(String, String)], name: &str) -> String {
    files
        .iter()
        .find(|(n, _)| n == name)
        .map(|f| f.1.clone())
        .unwrap_or_else(|| panic!("no file {name}"))
}

#[test]
fn quartiles_interpolate_between_values() {
    let s = stats(&[4.0, 1.0, 3.0, 2.0]).expect("stats");
    assert_eq!(s.n, 4);
    assert_eq!((s.median, s.p25, s.p75, s.mean), (2.5, 1.75, 3.25, 2.5));
    let one = stats(&[7.0]).expect("stats");
    assert_eq!((one.median, one.p25, one.p75), (7.0, 7.0, 7.0));
    assert_eq!(stats(&[]), None);
}

#[test]
fn numbers_and_fields_are_written_plainly() {
    assert_eq!(num(2.0), "2");
    assert_eq!(num(0.123_456), "0.123");
    assert_eq!(num(-0.000_1), "0");
    assert_eq!(num(12.5), "12.5");
    assert_eq!(field("plain"), "plain");
    assert_eq!(field("a,b"), "\"a,b\"");
    assert_eq!(field("say \"hi\""), "\"say \"\"hi\"\"\"");
}

#[test]
fn the_data_files_hold_a_row_for_each_metric_event_and_turn_with_enough_games() {
    let files = data_files::<Toy>(&input(3, 2), &options(&["alpha"]));
    let names: Vec<&str> = files.iter().map(|f| f.0.as_str()).collect();
    assert_eq!(
        names,
        [
            "turns-alpha.csv",
            "turns-all-top-players.csv",
            "turns-our-wins.csv",
            "turns-our-losses.csv"
        ]
    );
    let wins = file(&files, "turns-our-wins.csv");
    assert!(wins.starts_with("group,side,kind,name,turn,n,median,p25,p75,mean\n"));
    assert!(
        wins.contains("\nour-wins,us,metric,cells,1,3,4,4,4,4\n"),
        "{wins}"
    );
    assert!(
        wins.contains("\nour-wins,us,metric,cells,2,3,4,4,4,4\n"),
        "{wins}"
    );
    // The event: done at turn 1 in every game, not at turn 2.
    assert!(
        wins.contains("\nour-wins,us,event,build,1,3,1,1,1,1\n"),
        "{wins}"
    );
    assert!(
        wins.contains("\nour-wins,us,event,build,2,3,0,0,0,0\n"),
        "{wins}"
    );
    assert!(!wins.contains(",3,3,"), "no turn 3");
    // Two losses are fewer than 3 games: only the header.
    assert_eq!(file(&files, "turns-our-losses.csv").lines().count(), 1);
    // The pair of groups lists both sides.
    let all = file(&files, "turns-all-top-players.csv");
    assert!(
        all.contains("\nall-top-players,us,metric,cells,1,5,4,4,4,4\n"),
        "{all}"
    );
    assert!(
        all.contains("\nall-top-players,top players,metric,cells,1,5,9,9,9,9\n"),
        "{all}"
    );
    // The same input gives the same files.
    assert_eq!(files, data_files::<Toy>(&input(3, 2), &options(&["alpha"])));
}

#[test]
fn events_and_player_sets_are_curves_too() {
    let a = analyze::<Toy>(&input(6, 6), &options(&["alpha"]));
    let events = &a.curves.wins_vs_losses.events;
    assert_eq!(events.len(), 1);
    assert!(events[0].rate);
    assert_eq!(events[0].key, "build");
    assert_eq!(events[0].lines[0][0].mean, Some(1.0));
    assert_eq!(events[0].lines[0][1].mean, Some(0.0));
    // One set for the focus player, with the metric that tells wins from losses.
    assert_eq!(a.curves.players.len(), 1);
    let set = &a.curves.players[0];
    assert_eq!(set.slug, "player-alpha");
    assert_eq!(set.labels, ["us", "alpha"]);
    assert_eq!(set.games, [12, 12]);
    assert_eq!(set.curves.len(), 1);
    assert!(set.events.is_empty());
    // The data groups, for the report to list.
    let slugs: Vec<&str> = a.data.iter().map(|g| g.slug.as_str()).collect();
    assert_eq!(
        slugs,
        ["alpha", "all-top-players", "our-wins", "our-losses"]
    );
    assert_eq!(a.data[0].games, 12);
}

#[test]
fn a_game_without_us_is_in_no_group() {
    let mut game = built(1, true);
    game.0.our_seat = None;
    let files = data_files::<Toy>(
        &Input {
            games: vec![game],
            ..Input::default()
        },
        &options(&["alpha"]),
    );
    assert!(files.iter().all(|f| f.1.lines().count() == 1));
}
