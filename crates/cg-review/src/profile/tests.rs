use crate::test_util::{game, Toy};
use crate::trace::{Moment, Trace};

use super::*;

/// A game between `alpha` (red) and `beta` (blue); `cells` are the values
/// at each ply, and `commands` the red and blue answers of turn 1 and 2.
fn toy(id: u64, cells: &[(i64, i64)], winner: Option<usize>, red: &[&str]) -> (GameMeta, Trace) {
    let mut trace = game(cells, winner);
    trace.name = id.to_string();
    for (i, c) in red.iter().enumerate() {
        trace.plies[2 * i].commands = (*c).to_string();
    }
    let meta = GameMeta {
        game_id: id,
        kind: "ranking".to_string(),
        batch: None,
        our_seat: Some(1),
    };
    (meta, trace)
}

fn ranks() -> BTreeMap<String, u32> {
    BTreeMap::from([("alpha".to_string(), 2), ("gamma".to_string(), 1)])
}

/// Ten plies (5 turns) where blue (us) is always below red.
fn below(n: i64) -> Vec<(i64, i64)> {
    (0..10).map(|i| (n + i, n + i - 2)).collect()
}

#[test]
fn the_opponents_table_counts_our_results_by_seat() {
    let mut loss = toy(1, &below(5), Some(0), &["B1", "x", "x"]);
    loss.1.end = "red took the HQ".to_string();
    let win = toy(2, &below(5), Some(1), &["x"]);
    // A game we are not in, and one against a third player we have not ranked.
    let mut other = toy(3, &below(5), Some(0), &["x"]);
    other.0.our_seat = None;
    let mut third = toy(4, &below(5), Some(0), &["x"]);
    third.1.players[0] = "delta".to_string();
    let input = Input {
        games: vec![loss, win, other, third],
        ranks: ranks(),
        ..Input::default()
    };
    let a = analyze::<Toy>(
        &input,
        &Options {
            top: 5,
            players: vec![],
        },
    );
    assert_eq!(a.opponents.len(), 2);
    let alpha = &a.opponents[0];
    assert_eq!(alpha.player, "alpha");
    assert_eq!(alpha.rank, Some(2));
    assert_eq!(alpha.games, 2);
    assert_eq!((alpha.ours.won, alpha.ours.lost), (1, 1));
    assert_eq!(alpha.by_seat[1].games(), 2);
    assert_eq!(alpha.by_seat[0].games(), 0);
    assert_eq!(alpha.median_turns, Some(5.0));
    assert_eq!(alpha.ends["red took the HQ"], 1);
    // Unranked last.
    assert_eq!(a.opponents[1].player, "delta");
    assert_eq!(a.opponents[1].rank, None);
    // The game we are not in is listed on its own.
    assert!(a.games.iter().any(|l| l.label.ends_with("(not ours)")));
    assert_eq!(a.weaknesses.losses, 2);
}

#[test]
fn a_profile_has_events_and_openings() {
    let input = Input {
        games: vec![
            toy(1, &below(5), Some(0), &["B1;B2", "x", "B3"]),
            toy(2, &below(5), Some(0), &["B1;B2", "x", "y"]),
            toy(3, &below(5), Some(1), &["z", "x", "y"]),
        ],
        ranks: ranks(),
        ..Input::default()
    };
    let a = analyze::<Toy>(
        &input,
        &Options {
            top: 1,
            players: vec![],
        },
    );
    // The top 1 of the snapshot with a game: gamma has none, alpha is next.
    assert_eq!(a.profiles.len(), 1);
    let p = &a.profiles[0];
    assert_eq!(p.player, "alpha");
    assert_eq!(p.games, 3);
    assert!(!p.few_games);
    assert_eq!((p.vs_us[0].won, p.vs_us[0].lost), (2, 1));
    let build = &p.events[0];
    assert_eq!(build.name, "build");
    assert_eq!((build.theirs_in, build.games), (2, 3));
    assert_eq!(build.first_turn, Some(1.0));
    assert_eq!(build.mean_theirs, Some(5.0 / 3.0));
    assert_eq!(build.ours_in, 0);
    assert_eq!(p.openings[0].opening, "B1;B2 / x / B3");
    assert_eq!((p.openings[0].games, p.openings[0].wins), (1, 1));
    assert_eq!(p.openings.len(), 3);
    // Turn 5 is the last turn, read at the last ply (9): red 14, blue 12.
    let cells = &p.metrics[0];
    let at5 = cells.at_turn.iter().find(|c| c.turn == 5).expect("turn 5");
    assert_eq!(
        (at5.games, at5.theirs, at5.ours),
        (3, Some(14.0), Some(12.0))
    );
    // Turn 10 is not reached by half of the games: left out, and said so.
    assert!(cells.at_turn.iter().all(|c| c.turn <= 5));
    assert_eq!(p.turns_dropped, [10]);
    assert_eq!(p.opening_plies, 3);
    assert_eq!(p.wins.wins, 2);
}

#[test]
fn moments_are_counted_in_wins_and_losses() {
    let mut won = toy(1, &below(5), Some(0), &["x"]);
    won.1.moments.push(Moment {
        ply: 4,
        kind: "swing".to_string(),
        favours: Some(0),
        text: String::new(),
    });
    let lost = toy(2, &below(5), Some(1), &["x"]);
    let input = Input {
        games: vec![won, lost],
        ranks: ranks(),
        ..Input::default()
    };
    let a = analyze::<Toy>(
        &input,
        &Options {
            top: 5,
            players: vec![],
        },
    );
    let p = &a.profiles[0];
    let them = p.moments.iter().find(|m| m.favours == "them").expect("row");
    assert_eq!(them.in_wins, Share { with: 1, games: 1 });
    assert_eq!(them.in_losses, Share { with: 0, games: 1 });
    assert_eq!(them.median_turn, Some(3.0));
    assert_eq!(p.wins.first_moments[0].kind, "swing");
    // It favoured them in the game we lost: it is against us.
    assert_eq!(a.weaknesses.against_us.len(), 1);
    assert_eq!(a.weaknesses.against_us[0].by_phase, [1, 0, 0]);
}

#[test]
fn stays_below_finds_the_turn_from_which_it_never_recovers() {
    // Each side is read right after its own ply: blue is ahead at turns 1
    // and 2 (4 against 1 and 3), behind from turn 3.
    let cells = [
        (1, 5),
        (1, 5),
        (3, 4),
        (3, 4),
        (5, 4),
        (5, 4),
        (6, 4),
        (6, 4),
    ];
    let trace = game(&cells, Some(0));
    assert_eq!(stays_below(&trace, 1, "cells"), Some(3));
    assert_eq!(stays_below(&trace, 0, "cells"), None);
    assert_eq!(stays_below(&trace, 1, "nothing"), None);
    // Ahead at the last turn: not below.
    let ahead = game(&[(1, 2), (1, 2)], Some(1));
    assert_eq!(stays_below(&ahead, 1, "cells"), None);
}

#[test]
fn the_weaknesses_list_the_earliest_falls() {
    let games = (0..7)
        .map(|i| {
            let mut cells = below(5);
            // Blue is ahead before turn `i`, so the fall comes later in later games.
            for cell in cells.iter_mut().take((2 * i).min(9)) {
                *cell = (1, 9);
            }
            toy(10 + i as u64, &cells, Some(0), &["x"])
        })
        .collect();
    let input = Input {
        games,
        ranks: ranks(),
        ..Input::default()
    };
    let a = analyze::<Toy>(
        &input,
        &Options {
            top: 5,
            players: vec![],
        },
    );
    let w = &a.weaknesses;
    assert_eq!(w.losses, 7);
    assert_eq!(w.earliest.len(), EARLIEST_SHOWN);
    assert_eq!(w.earliest[0].game_id, 10);
    assert_eq!(w.earliest[0].turn, 1);
    assert_eq!(w.earliest[1].turn, 2);
    assert_eq!(w.earliest[2].turn, 3);
    assert_eq!(w.stays_below[0].below, 7);
    assert_eq!(w.lead, "cells");
}

#[test]
fn small_samples_and_edge_cases_are_flagged() {
    // One game, one player, no loss for us; ending at turn 1.
    let win = toy(1, &[(5, 3), (5, 3)], Some(1), &["x"]);
    let input = Input {
        games: vec![win],
        ranks: ranks(),
        snapshot: Some("s.json".to_string()),
        left_out: vec![LeftOut {
            game_id: Some(9),
            why: "no log".to_string(),
        }],
        warnings: vec!["a name differs".to_string()],
        notes: vec!["a note".to_string()],
    };
    let a = analyze::<Toy>(
        &input,
        &Options {
            top: 5,
            players: vec![],
        },
    );
    assert!(a.profiles[0].few_games);
    assert_eq!(a.weaknesses.losses, 0);
    let files = report_files(&a, 0);
    let readme = file(&files, "README.md");
    assert!(readme.contains("**Warning:** a name differs"), "{readme}");
    assert!(readme.contains("a note"), "{readme}");
    assert!(file(&files, "left_out.csv").contains("9,no log"));
    // The player has one game against us.
    assert!(file(&files, "profile_results.csv").contains("alpha,2,1,"));
    assert!(file(&files, "weaknesses_stays_below.csv").starts_with("metric,losses,"));
    assert_eq!(files.len(), 20);
    // A named player without a game is shown as such, and the JSON is valid.
    let named = analyze::<Toy>(
        &input,
        &Options {
            top: 0,
            players: vec!["nobody".to_string()],
        },
    );
    assert_eq!(named.profiles[0].games, 0);
    assert!(file(&report_files(&named, 0), "profile_results.csv").contains("nobody,,0,0,0,0,0"));
    let json = serde_json::to_string(&named).expect("json");
    assert!(json.contains("\"profiles\""));
}

#[test]
fn a_game_we_are_not_in_counts_for_the_profile_only() {
    let mut other = toy(1, &below(5), Some(0), &["B1"]);
    other.0.our_seat = None;
    let input = Input {
        games: vec![other],
        ranks: ranks(),
        ..Input::default()
    };
    let a = analyze::<Toy>(
        &input,
        &Options {
            top: 5,
            players: vec![],
        },
    );
    assert!(a.opponents.is_empty());
    assert_eq!(a.profiles.len(), 1);
    assert_eq!(a.profiles[0].games, 1);
    assert_eq!(a.profiles[0].vs_us[0].games(), 0);
    assert_eq!(a.weaknesses.losses, 0);
}

#[test]
fn each_side_is_read_right_after_its_own_ply() {
    // Blue (seat 1) is below red at the start of each of its own plies
    // (3 against 5, 5 against 7, 7 against 9: its income comes after), but
    // equal once it has played: the snapshot of the ply after its own.
    let cells = [(5, 5), (5, 3), (7, 5), (7, 5), (9, 7), (9, 7), (11, 9)];
    let trace = game(&cells, Some(0));
    let index = trace.metric("cells").expect("cells");
    assert_eq!(value_at(&trace, 0, 1, index), Some(5));
    assert_eq!(value_at(&trace, 1, 1, index), Some(5));
    assert_eq!(value_at(&trace, 0, 3, index), Some(9));
    assert_eq!(value_at(&trace, 1, 3, index), Some(9));
    // Red's last ply is read at its own snapshot; blue has no turn 4.
    assert_eq!(value_at(&trace, 0, 4, index), Some(11));
    assert_eq!(value_at(&trace, 1, 4, index), None);
    // So blue is never "behind from turn 1".
    assert_eq!(stays_below(&trace, 1, "cells"), None);
}

/// Five wins and five losses of ours, in which we are `blue_win` and
/// `blue_loss` all game against red's 9.
fn won_and_lost(blue_win: i64, blue_loss: i64) -> Input {
    let mut games = Vec::new();
    for i in 0..5 {
        games.push(toy(i, &[(9, blue_win); 10], Some(1), &["x"]));
        games.push(toy(10 + i, &[(9, blue_loss); 10], Some(0), &["x"]));
    }
    Input {
        games,
        ranks: ranks(),
        ..Input::default()
    }
}

/// The text of the file `name` among `files`.
fn file(files: &[(String, String)], name: &str) -> String {
    files
        .iter()
        .find(|(n, _)| n == name)
        .map(|(_, text)| text.clone())
        .unwrap_or_else(|| panic!("no file {name}"))
}

#[test]
fn the_files_compare_wins_with_losses_and_point_to_the_charts() {
    let a = analyze::<Toy>(
        &won_and_lost(12, 3),
        &Options {
            top: 1,
            players: vec![],
        },
    );
    let files = report_files(&a, 7);
    let signals = file(&files, "signals.md");
    assert!(
        signals.contains("**cells**: our wins and our losses separate from turn 1; at turn 3 the median is 12 for our wins and 3 for our losses (AUC 1.00, n = 5 and 5)."),
        "{signals}"
    );
    assert!(signals.contains("`wins-vs-losses-cells.svg`"), "{signals}");
    assert!(signals.contains("`turns-our-wins.csv`"), "{signals}");
    assert!(file(&files, "separation.csv").contains("wins-vs-losses,cells,false,3,1,12,3,5,5,1\n"));
    // The control: we are above red in every win and below in every loss.
    assert!(file(&files, "weaknesses_stays_below.csv").contains("cells,5,5,5,0,1\n"));
    let readme = file(&files, "README.md");
    assert!(readme.contains("7 SVG files"), "{readme}");
    assert!(readme.contains("`turns-all-top-players.csv`"), "{readme}");
    assert!(readme.contains("`separation.csv`:"), "{readme}");
    // Every table file named in the README is made.
    for line in readme
        .lines()
        .filter(|l| l.starts_with("- `") && l.contains(".csv`: "))
    {
        let name = line
            .trim_start_matches("- `")
            .split('`')
            .next()
            .expect("name");
        if !name.starts_with("turns-") {
            file(&files, name);
        }
    }
}

#[test]
fn signals_without_wins_or_losses_say_so() {
    let mut input = won_and_lost(12, 3);
    input.games.retain(|(_, trace)| trace.winner == Some(0));
    let a = analyze::<Toy>(
        &input,
        &Options {
            top: 1,
            players: vec![],
        },
    );
    let signals = file(&report_files(&a, 0), "signals.md");
    assert!(signals.contains("Nothing to compare"), "{signals}");
}
