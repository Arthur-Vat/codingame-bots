use crate::profile::{analyze, GameMeta, Input, Options};
use crate::test_util::{game, Toy};

use super::*;

/// A game of two turns: we (blue) hold `ours` against red's 9 and build at
/// turn 1 only in a win.
fn played(id: u64, won: bool, ours: i64) -> (GameMeta, crate::trace::Trace) {
    let mut trace = game(&[(9, ours); 4], Some(usize::from(won)));
    trace.plies[1].commands = if won { "B1" } else { "x" }.to_string();
    let meta = GameMeta {
        game_id: id,
        kind: "ranking".to_string(),
        batch: None,
        our_seat: Some(1),
    };
    (meta, trace)
}

fn files(wins: usize, losses: usize) -> Vec<(String, String)> {
    let games = (0..wins)
        .map(|i| played(i as u64, true, 12))
        .chain((0..losses).map(|i| played(100 + i as u64, false, 3)))
        .collect();
    let a = analyze::<Toy>(
        &Input {
            games,
            ..Input::default()
        },
        &Options {
            top: 0,
            players: vec!["alpha".to_string()],
        },
    );
    report_files(&a, 4)
}

fn file(files: &[(String, String)], name: &str) -> String {
    files
        .iter()
        .find(|(n, _)| n == name)
        .map(|f| f.1.clone())
        .unwrap_or_else(|| panic!("no file {name}"))
}

#[test]
fn every_table_file_is_made_with_its_header_and_is_described() {
    let files = files(6, 6);
    for (name, what) in TABLES {
        assert!(what.ends_with('.'), "{name}");
        let text = file(&files, name);
        if name.ends_with(".csv") {
            let header = text.lines().next().expect("a header");
            assert!(!header.is_empty() && !header.contains(' '), "{name}");
        }
    }
    // Every column of a CSV is named in its description.
    for (name, what) in TABLES.iter().filter(|t| t.0.ends_with(".csv")) {
        let text = file(&files, name);
        for column in text.lines().next().expect("a header").split(',') {
            assert!(
                what.contains(&format!("`{column}`")) || column == "lead" || column == "rank",
                "{name}: {column}"
            );
        }
    }
}

#[test]
fn the_signals_name_event_gaps_with_their_charts() {
    let signals = file(&files(6, 6), "signals.md");
    assert!(signals.starts_with("# Signals"));
    // We build at turn 1 in every win and in no loss: a gap of 100 points.
    assert!(
        signals.contains("- **build**: the rate of our wins is 20 points higher or more at turn 1; widest at turn 1: 100% of our wins against 0% of our losses (100 points). Chart: `wins-vs-losses-event-build.svg`."),
        "{signals}"
    );
    assert!(signals.contains("`turns-our-wins.csv`"), "{signals}");
    // The same metric in the wins against the losses.
    assert!(
        signals.contains("**cells**: our wins and our losses"),
        "{signals}"
    );
}

#[test]
fn small_groups_have_no_gap_to_report() {
    let signals = file(&files(2, 2), "signals.md");
    assert!(
        signals.contains("No event differs by 20 points"),
        "{signals}"
    );
    assert!(!signals.contains("**build**"), "{signals}");
}

#[test]
fn the_readme_lists_the_groups_sets_and_charts() {
    let readme = file(&files(6, 6), "README.md");
    assert!(readme.contains("4 SVG files"), "{readme}");
    assert!(
        readme.contains("`turns-alpha.csv`: alpha (12 games)"),
        "{readme}"
    );
    assert!(
        readme.contains("`wins-vs-losses`: our wins (n = 6), our losses (n = 6)"),
        "{readme}"
    );
    assert!(
        readme.contains("`player-alpha`: us (n = 12), alpha (n = 12)"),
        "{readme}"
    );
}

#[test]
fn csv_cells_with_commas_are_quoted() {
    let mut csv = Csv::new(&["a", "b"]);
    csv.row(&["x,y".to_string(), "z".to_string()]);
    assert_eq!(csv.0, "a,b\n\"x,y\",z\n");
}
