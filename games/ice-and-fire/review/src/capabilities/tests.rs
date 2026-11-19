use ice_and_fire_engine::{Command, League, State, HQ};
use ice_and_fire_tactics::chain::chain_to;

use super::*;

/// Red to act at game turn 20 on an open map with no mine spot: red holds
/// the cells where `red(x, y)`, blue those where `blue(x, y)`, no unit and
/// no building but the HQs. `money` is red's gold and income, then blue's;
/// the incomes must be the cell counts, as the engine recomputes them at the
/// next turn.
fn position(
    red: impl Fn(usize, usize) -> bool,
    blue: impl Fn(usize, usize) -> bool,
    money: [i64; 4],
) -> State {
    let mut turn = String::new();
    for value in money {
        turn.push_str(&format!("{value}\n"));
    }
    for y in 0..12 {
        for x in 0..12 {
            turn.push(if red(x, y) {
                'O'
            } else if blue(x, y) {
                'X'
            } else {
                '.'
            });
        }
        turn.push('\n');
    }
    turn.push_str("2\n0 0 0 0\n1 0 11 11\n0\n");
    State::read("0\n", &turn, League::Bronze, 20).expect("a valid position")
}

/// Each side holds the 3×3 corner of its HQ (9 cells, income 9): red with
/// `gold`, blue with `their_gold`.
fn apart(gold: i64, their_gold: i64) -> State {
    position(
        |x, y| x < 3 && y < 3,
        |x, y| x > 8 && y > 8,
        [gold, 9, their_gold, 9],
    )
}

#[test]
fn a_chain_red_can_pay_now_is_a_finish_test_that_the_chain_passes() {
    let cost = chain_cost(&apart(0, 0), 0);
    assert!(cost > 0 && cost != UNREACHABLE);
    let state = apart(cost, 0);
    assert_eq!(classify(&state, &[], false), vec![Capability::Finish]);
    let chain = chain_to(&state, HQ[1], cost).expect("a chain within the gold");
    let won = after(&state, &chain.commands);
    assert!(passes(Capability::Finish, &state, &won, &won));
    let waited = after(&state, &[]);
    assert!(!passes(Capability::Finish, &state, &waited, &waited));
}

#[test]
fn saving_keeps_the_strike_on_track_and_a_tower_at_the_back_does_not() {
    let cost = chain_cost(&apart(0, 0), 0);
    for k in [1, 2] {
        let state = apart(cost - 9 * k, 0);
        assert_eq!(
            classify(&state, &[], false),
            vec![Capability::Save(k as u8)]
        );
        let waited = after(&state, &[]);
        assert!(passes(Capability::Save(k as u8), &state, &waited, &waited));
        let spent = after(&state, &[Command::BuildTower { x: 0, y: 1 }]);
        assert!(spent.gold(0) < state.gold(0), "the tower was built");
        assert!(!passes(Capability::Save(k as u8), &state, &spent, &spent));
    }
    // Three turns away is no test.
    assert!(classify(&apart(cost - 27, 0), &[], false).is_empty());
}

#[test]
fn a_saving_side_that_the_enemy_can_strike_next_turn_must_defend_instead() {
    let cost = chain_cost(&apart(0, 0), 0);
    let theirs = chain_cost(&apart(0, 0), 1);
    let state = apart(cost - 9, theirs - 9);
    assert_eq!(classify(&state, &[], false), vec![Capability::Defend(1)]);
}

#[test]
fn a_tower_on_the_hq_path_defends_and_waiting_does_not() {
    let theirs = chain_cost(&apart(0, 0), 1);
    for k in [1, 2] {
        // Blue's gold at its k-th next turn pays its chain exactly.
        let state = apart(15, theirs - 9 * k);
        assert_eq!(
            classify(&state, &[], false),
            vec![Capability::Defend(k as u8)]
        );
        let waited = after(&state, &[]);
        assert!(!passes(
            Capability::Defend(k as u8),
            &state,
            &waited,
            &waited
        ));
        let guarded = after(&state, &[Command::BuildTower { x: 1, y: 0 }]);
        assert!(guarded.towers().test(1), "the tower was built");
        assert!(passes(
            Capability::Defend(k as u8),
            &state,
            &guarded,
            &guarded
        ));
    }
}

#[test]
fn a_tower_at_the_front_is_a_wall_test_of_the_reference_only() {
    // The territories touch: red holds rows 0-5 (72 cells), blue row 6
    // and the column of its HQ (17 cells), so blue has 20 gold at its turn:
    // two of red's rows, but not a cell a tower guards.
    let state = position(
        |_, y| y < 6,
        |x, y| y == 6 || (x == 11 && y > 6),
        [30, 72, 3, 17],
    );
    let wall = [Command::BuildTower { x: 3, y: 4 }];
    let back = [Command::BuildTower { x: 3, y: 0 }];
    // Red can also pay its chain with one more income: a save test too.
    let save = vec![Capability::Save(1)];
    assert_eq!(
        classify(&state, &wall, true),
        [save.clone(), vec![Capability::Wall]].concat()
    );
    assert_eq!(classify(&state, &wall, false), save);
    assert_eq!(classify(&state, &back, true), save);

    let walled = after(&state, &wall);
    let waited = after(&state, &[]);
    assert!(exposure(&walled, 0) < exposure(&waited, 0));
    assert!(passes(Capability::Wall, &state, &walled, &walled));
    assert!(passes(Capability::Wall, &state, &walled, &waited));
    assert!(!passes(Capability::Wall, &state, &waited, &walled));
}

fn test(capability: Capability, game_id: u64, ply: usize) -> Test {
    Test {
        capability,
        game_id,
        ply,
        turn: ply as u32 / 2 + 1,
        mover: "zasmu".to_string(),
    }
}

#[test]
fn a_suite_reads_back_what_it_wrote() {
    let tests = vec![
        test(Capability::Finish, 7, 40),
        test(Capability::Save(2), 7, 30),
        test(Capability::Defend(1), 8, 21),
        test(Capability::Wall, 9, 25),
    ];
    let text = suite_text("two lines\nof comment", &tests);
    assert!(text.starts_with("# two lines\n# of comment\ncapability\t"));
    assert_eq!(read_suite(&text), Ok(tests));
    assert!(read_suite("jump\t1\t2\t3\tx\n").is_err());
    assert!(read_suite("finish\t1\t2\t3\n").is_err());
}

#[test]
fn the_draw_caps_each_capability_and_each_game() {
    let mut all = Vec::new();
    for game in 0..10 {
        for ply in 0..5 {
            all.push(test(Capability::Save(1), game, ply));
            all.push(test(Capability::Finish, game, ply));
        }
    }
    let drawn = draw(all.clone(), 6, 2, 1);
    assert_eq!(drawn.len(), 12);
    for c in [Capability::Save(1), Capability::Finish] {
        let of: Vec<&Test> = drawn.iter().filter(|t| t.capability == c).collect();
        assert_eq!(of.len(), 6);
        for game in 0..10 {
            assert!(of.iter().filter(|t| t.game_id == game).count() <= 2);
        }
    }
    let mut sorted = drawn.clone();
    sorted.sort();
    assert_eq!(drawn, sorted);
    assert_eq!(draw(all, 6, 2, 1), drawn, "the same seed draws the same");
}

mod end_to_end {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::{Path, PathBuf};

    use crate::profiles::Request as Base;
    use crate::test_util::temp_dir;

    use super::super::*;

    const FINISHED: &str = include_str!("../../testdata/914386214.json");
    const TIMEOUT: &str = include_str!("../../testdata/914333422.json");

    /// A data directory of the two real logs (Gabbek against Tarleya).
    fn data_dir(name: &str) -> PathBuf {
        let dir = temp_dir(name);
        fs::create_dir_all(dir.join("logs")).expect("logs");
        fs::write(dir.join("logs/914386214.json"), FINISHED).expect("log");
        fs::write(dir.join("logs/914333422.json"), TIMEOUT).expect("log");
        let line = |id: u64, kind: &str, seat: u64, names: [&str; 2]| {
            format!(
                "{{\"game_id\":{id},\"kind\":\"{kind}\",\"puzzle\":\"p\",\"our_seat\":{seat},\"players\":[{{\"seat\":0,\"name\":\"{}\"}},{{\"seat\":1,\"name\":\"{}\"}}],\"game_options\":\"seed=1\"}}\n",
                names[0], names[1]
            )
        };
        let index = [
            line(914386214, "ranking", 1, ["Gabbek", "Tarleya"]),
            line(914333422, "test", 0, ["Tarleya", "Gabbek"]),
        ]
        .concat();
        fs::write(dir.join("index.jsonl"), index).expect("index");
        dir
    }

    fn base(dir: &Path) -> Base {
        Base {
            data: dir.to_path_buf(),
            me: "Tarleya".to_string(),
            kinds: Vec::new(),
            batch: None,
            release: None,
            our_seat: None,
            top: 0,
            players: Vec::new(),
            leaderboard: None,
            jobs: 2,
        }
    }

    /// A bot that reads the protocol and answers `WAIT` every turn, or one
    /// that exits at once.
    fn bot(dir: &Path, name: &str, waits: bool) -> String {
        let body = if waits {
            "read n; i=0; while [ $i -lt $n ]; do read l; i=$((i+1)); done\n\
             while true; do\n\
               for j in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16; do read l || exit 0; done\n\
               read b; i=0; while [ $i -lt $b ]; do read l; i=$((i+1)); done\n\
               read u; i=0; while [ $i -lt $u ]; do read l; i=$((i+1)); done\n\
               echo WAIT\n\
             done"
        } else {
            "exit 0"
        };
        let file = dir.join(format!("{name}.sh"));
        fs::write(&file, format!("#!/bin/sh\n{body}\n")).expect("script");
        fs::set_permissions(&file, fs::Permissions::from_mode(0o755)).expect("mode");
        format!("{name}={}", file.display())
    }

    fn request(dir: &Path, suite: PathBuf, bots: Vec<String>) -> Request {
        Request {
            base: base(dir),
            suite,
            bots,
            tries: 0,
            reference: "Gabbek".to_string(),
            csv: Some(dir.join("verdicts.csv")),
        }
    }

    #[test]
    fn a_bot_that_waits_gets_the_verdicts_of_waiting() {
        let dir = data_dir("capabilities-wait");
        let (text, summary) = run_suite(&SuiteRequest {
            base: base(&dir),
            reference: "Gabbek".to_string(),
            per: 100,
            per_game: 100,
            seed: 1,
        })
        .expect("a suite");
        assert!(
            summary.starts_with("2 games replayed, 0 left out"),
            "{summary}"
        );
        let tests = read_suite(&text).expect("the suite reads back");
        assert!(!tests.is_empty());
        assert!(tests
            .iter()
            .all(|t| t.mover == "Gabbek" || t.mover == "Tarleya"));
        assert!(tests
            .iter()
            .filter(|t| t.capability == Capability::Wall)
            .all(|t| t.mover == "Gabbek"));
        let suite = dir.join("suite.tsv");
        fs::write(&suite, &text).expect("suite");

        let report = run(&request(&dir, suite, vec![bot(&dir, "waiter", true)])).expect("a report");
        assert!(report.contains("| waiter |"), "{report}");
        assert!(!report.contains("did not answer"), "{report}");

        // Each verdict is that of WAIT, computed on the replayed game.
        let (games, _) = load_games(
            &base(&dir),
            &read_index(&dir.join("index.jsonl"), &base(&dir))
                .expect("index")
                .0,
        );
        let games: BTreeMap<u64, Game> = games.into_iter().collect();
        let csv = fs::read_to_string(dir.join("verdicts.csv")).expect("csv");
        let rows: Vec<&str> = csv.lines().skip(1).collect();
        assert_eq!(rows.len(), tests.len());
        for (t, row) in tests.iter().zip(rows) {
            let step = &games[&t.game_id].steps[t.ply];
            let waited = after(&step.state, &[]);
            let played = after(&step.state, &step.commands);
            let expect = passes(t.capability, &step.state, &waited, &played);
            assert!(row.ends_with(if expect { ",1" } else { ",0" }), "{row}");
        }
    }

    #[test]
    fn a_bot_that_does_not_answer_fails_and_a_stale_test_is_refused() {
        let dir = data_dir("capabilities-silent");
        let (text, _) = run_suite(&SuiteRequest {
            base: base(&dir),
            reference: "Gabbek".to_string(),
            per: 3,
            per_game: 3,
            seed: 1,
        })
        .expect("a suite");
        let suite = dir.join("suite.tsv");
        fs::write(&suite, &text).expect("suite");
        let report = run(&request(
            &dir,
            suite.clone(),
            vec![bot(&dir, "silent", false)],
        ))
        .expect("a report");
        assert!(report.contains("did not answer"), "{report}");
        assert!(report.contains("| 0% ("), "{report}");

        // A test whose position does not test its capability.
        let first = read_suite(&text).expect("suite")[0].clone();
        let stale = Test {
            capability: if first.capability == Capability::Finish {
                Capability::Wall
            } else {
                Capability::Finish
            },
            ..first
        };
        fs::write(&suite, suite_text("stale", &[stale])).expect("suite");
        let err = run(&request(&dir, suite, vec![bot(&dir, "silent", false)])).unwrap_err();
        assert!(err.contains("no longer gives"), "{err}");
    }
}
