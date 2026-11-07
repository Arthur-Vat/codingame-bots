use crate::profile::{analyze, chart, report_files, Analysis, GameMeta, Input, Options, Profiling};
use crate::test_util::{game, Toy};
use crate::trace::{Metric, Moment, Trace};

use super::*;

/// The metrics of the made-up game, in the order of the arrays below:
/// killed, lost, exchange, upkeep, gross, contact, cells.
const KEYS: [&str; 7] = [
    "killed", "lost", "exchange", "upkeep", "gross", "contact", "cells",
];

type Values = [i64; 7];

/// The economy game's hooks: efficiency of cells (and of kills) over what
/// is spent, roles `economy` (builds) and `army` (training).
struct Eco;

impl Profiling for Eco {
    fn turns() -> Vec<u32> {
        vec![1, 2, 3]
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
    fn lower_is_better(_: &str) -> bool {
        false
    }
    fn compare_turns() -> Vec<u32> {
        vec![2, 3]
    }
    fn resource() -> &'static str {
        "cells"
    }
    fn spending(trace: &Trace, ply: usize) -> Vec<(String, i64)> {
        Toy::spending(trace, ply)
    }
    fn economy() -> Option<EconomyHooks> {
        Some(EconomyHooks {
            balance: "exchange",
            killed: "killed",
            lost: "lost",
            efficiency: vec![
                Efficiency {
                    name: "cells per resource",
                    gained: vec!["cells"],
                    spent: vec!["build", "train"],
                },
                Efficiency {
                    name: "kills per training",
                    gained: vec!["killed"],
                    spent: vec!["train"],
                },
            ],
            roles: vec![
                Role {
                    name: "economy",
                    categories: vec!["build"],
                },
                Role {
                    name: "army",
                    categories: vec!["train"],
                },
            ],
            upkeep: "upkeep",
            gross_income: "gross",
            contact: "contact",
            income: "gross",
            decisive: vec!["cut"],
        })
    }
}

/// A game of three turns between alpha (red) and us (blue). `states` are
/// the values of red and blue at the end of turns 1, 2 and 3 (read as the
/// report reads them: the snapshots of plies 1 and 2 show turn 1, of plies
/// 3 and 4 turn 2, of ply 5 turn 3; ply 0 shows zeros). `red` and `blue` are
/// each side's answers in its three turns.
fn eco(
    id: u64,
    states: [[Values; 2]; 3],
    red: [&str; 3],
    blue: [&str; 3],
    winner: usize,
) -> (GameMeta, Trace) {
    let mut trace = game(&[(0, 0); 6], Some(winner));
    trace.name = id.to_string();
    trace.metrics = KEYS
        .iter()
        .map(|k| Metric {
            key: (*k).to_string(),
            meaning: format!("The {k}."),
            none: None,
        })
        .collect();
    let shown = [None, Some(0), Some(0), Some(1), Some(1), Some(2)];
    for (ply, state) in trace.plies.iter_mut().zip(shown) {
        ply.metrics = match state {
            None => [vec![0; 7], vec![0; 7]],
            Some(t) => [states[t][0].to_vec(), states[t][1].to_vec()],
        };
    }
    for t in 0..3 {
        trace.plies[2 * t].commands = red[t].to_string();
        trace.plies[2 * t + 1].commands = blue[t].to_string();
    }
    let meta = GameMeta {
        game_id: id,
        kind: "ranking".to_string(),
        our_seat: Some(1),
        ..GameMeta::default()
    };
    (meta, trace)
}

/// Red builds and trains in turn 1 (3 spent), blue trains twice in turn 1
/// and builds in turn 2 (4 spent, 2 of it on training).
fn standard(id: u64, red_killed: i64, winner: usize) -> (GameMeta, Trace) {
    let red_end = [red_killed, 10, red_killed - 10, 3, 10, 1, 9];
    eco(
        id,
        [
            [[10, 0, 10, 1, 4, 0, 3], [0, 10, -10, 0, 4, 0, 3]],
            [[20, 10, 10, 2, 8, 1, 6], [10, 20, -10, 1, 6, 1, 4]],
            [red_end, [10, 30, -20, 1, 6, 1, 4]],
        ],
        ["B1;T1", "x", "x"],
        ["T1;T2", "B1", "x"],
        winner,
    )
}

fn run(games: Vec<(GameMeta, Trace)>) -> Analysis {
    analyze::<Eco>(
        &Input {
            games,
            ..Input::default()
        },
        &Options {
            top: 0,
            players: vec!["alpha".to_string()],
        },
    )
}

fn two_games() -> Analysis {
    run(vec![standard(1, 30, 0), standard(2, 50, 1)])
}

fn row<'a>(rows: &'a [ExchangeRow], group: &str) -> &'a ExchangeRow {
    rows.iter()
        .find(|r| r.group == group && r.phase == "early game")
        .expect("a row")
}

#[test]
fn the_exchange_is_read_at_the_end_of_each_phase() {
    let e = two_games().economy;
    // No middle game or endgame in the made-up games: three phases listed,
    // the last two without sides.
    assert_eq!(
        e.exchange
            .iter()
            .filter(|r| r.group == "top players")
            .count(),
        3
    );
    let top = row(&e.exchange, "top players");
    // Red's killed at the end: 30 and 50.
    assert_eq!(
        (top.games, top.killed, top.lost),
        (2, Some(40.0), Some(10.0))
    );
    assert_eq!(top.balance, Some(30.0));
    let us = row(&e.exchange, "us, same games");
    assert_eq!(
        (us.killed, us.lost, us.balance),
        (Some(10.0), Some(30.0), Some(-20.0))
    );
    let late = e
        .exchange
        .iter()
        .find(|r| r.group == "top players" && r.phase == "endgame")
        .expect("a row");
    assert_eq!((late.games, late.balance), (0, None));
}

#[test]
fn efficiency_is_the_gain_so_far_over_the_spending_so_far() {
    let e = two_games().economy;
    let at = |group: &str, name: &str, turn: u32| {
        e.efficiency
            .iter()
            .find(|r| r.group == group && r.name == name && r.turn == turn)
            .unwrap_or_else(|| panic!("{group} {name} {turn}"))
            .clone()
    };
    // Red: 3 cells for 3 spent at turn 1, 6 for 3 at turn 2.
    assert_eq!(at("top players", "cells per resource", 1).median, Some(1.0));
    assert_eq!(at("top players", "cells per resource", 2).median, Some(2.0));
    // Blue spent 2 at turn 1 (cells 3) and 4 at turn 2 (cells 4).
    assert_eq!(
        at("us, same games", "cells per resource", 1).median,
        Some(1.5)
    );
    assert_eq!(
        at("us, same games", "cells per resource", 2).median,
        Some(1.0)
    );
    // Kills over training only: red 20 killed for 1, blue 10 for 2.
    assert_eq!(
        at("top players", "kills per training", 2).median,
        Some(20.0)
    );
    assert_eq!(
        at("us, same games", "kills per training", 2).median,
        Some(5.0)
    );
    assert_eq!(at("top players", "kills per training", 2).games, 2);
}

#[test]
fn nothing_spent_gives_no_efficiency() {
    let (meta, trace) = eco(
        1,
        [[[5, 0, 5, 0, 4, 0, 3]; 2]; 3],
        ["x", "x", "x"],
        ["x", "x", "x"],
        0,
    );
    let e = run(vec![(meta, trace)]).economy;
    let first = e
        .efficiency
        .iter()
        .find(|r| r.group == "top players")
        .expect("a row");
    assert_eq!((first.games, first.median), (0, None));
}

#[test]
fn the_mix_is_the_share_of_spending_by_role() {
    let e = two_games().economy;
    let mix = |group: &str, role: &str| {
        e.mix
            .iter()
            .find(|r| r.group == group && r.phase == "early game" && r.role == role)
            .unwrap_or_else(|| panic!("{group} {role}"))
            .clone()
    };
    // Red spent 2 on a build and 1 on training, in 3 plies of 2 games.
    let (economy, army) = (mix("top players", "economy"), mix("top players", "army"));
    assert_eq!(economy.plies, 6);
    assert!((economy.share.expect("a share") - 200.0 / 3.0).abs() < 1e-9);
    assert!((army.share.expect("a share") - 100.0 / 3.0).abs() < 1e-9);
    assert!((economy.spent_per_ply - 4.0 / 6.0).abs() < 1e-9);
    // Blue: 2 on training and 2 on a build.
    assert_eq!(mix("us, same games", "army").share, Some(50.0));
    // Nothing is "other" here.
    assert!(e.mix.iter().all(|r| r.role != OTHER));
    // A phase with no plies has no share.
    let late = e
        .mix
        .iter()
        .find(|r| r.group == "top players" && r.phase == "endgame")
        .expect("a row");
    assert_eq!((late.plies, late.share), (0, None));
}

#[test]
fn spending_outside_every_role_is_listed_as_other() {
    let (meta, mut trace) = standard(1, 30, 0);
    trace.plies[0].commands = "B1;T1;Tx".to_string();
    // Both trainings are "train", which the roles of `NoArmy` lack.
    struct NoArmy;
    impl Profiling for NoArmy {
        fn turns() -> Vec<u32> {
            Eco::turns()
        }
        fn opening_plies() -> usize {
            1
        }
        fn key_metrics() -> Vec<&'static str> {
            Eco::key_metrics()
        }
        fn events(_: &Trace, _: usize) -> Vec<String> {
            Vec::new()
        }
        fn opening(_: &Trace, _: usize, _: usize) -> Vec<String> {
            Vec::new()
        }
        fn lower_is_better(_: &str) -> bool {
            false
        }
        fn compare_turns() -> Vec<u32> {
            Eco::compare_turns()
        }
        fn resource() -> &'static str {
            "cells"
        }
        fn spending(trace: &Trace, ply: usize) -> Vec<(String, i64)> {
            Toy::spending(trace, ply)
        }
        fn economy() -> Option<EconomyHooks> {
            let mut hooks = Eco::economy()?;
            hooks.roles.truncate(1);
            Some(hooks)
        }
    }
    let a = analyze::<NoArmy>(
        &Input {
            games: vec![(meta, trace)],
            ..Input::default()
        },
        &Options {
            top: 0,
            players: vec!["alpha".to_string()],
        },
    );
    let other = a
        .economy
        .mix
        .iter()
        .find(|r| r.group == "top players" && r.phase == "early game" && r.role == OTHER)
        .expect("an other row");
    // Two builds' worth (2) on economy, two trainings (2) on no role.
    assert_eq!(other.share, Some(50.0));
}

#[test]
fn the_upkeep_burden_is_the_share_of_gross_income_the_upkeep_eats() {
    let e = two_games().economy;
    let burden = |group: &str| {
        e.upkeep
            .iter()
            .find(|r| r.group == group && r.phase == "early game")
            .expect("a row")
            .clone()
    };
    // Red's plies start at the snapshots of plies 0, 2 and 4: zeros (no
    // gross income: left out), turn 1 (1 of 4) and turn 2 (2 of 8): 25%.
    let top = burden("top players");
    assert_eq!(top.plies, 6);
    assert_eq!(top.burden, Some(25.0));
    // Blue's start at plies 1, 3 and 5: 0 of 4, 1 of 6 and 1 of 6.
    let us = burden("us, same games");
    assert!((us.burden.expect("a burden") - (0.0 + 100.0 / 6.0 + 100.0 / 6.0) / 3.0).abs() < 1e-9);
}

#[test]
fn tempo_counts_from_first_contact_to_the_first_decisive_moment_in_favour() {
    let (m1, mut t1) = standard(1, 30, 0);
    // Red's cut at ply 4 (turn 3) favours red; a cut favouring blue at
    // ply 3 (turn 2) and a moment of another kind are ignored for red.
    t1.moments = vec![
        Moment {
            ply: 4,
            kind: "cut".to_string(),
            favours: Some(0),
            text: String::new(),
        },
        Moment {
            ply: 3,
            kind: "cut".to_string(),
            favours: Some(1),
            text: String::new(),
        },
        Moment {
            ply: 1,
            kind: "lead change".to_string(),
            favours: Some(0),
            text: String::new(),
        },
    ];
    let (m2, t2) = standard(2, 50, 1);
    let e = run(vec![(m1, t1), (m2, t2)]).economy;
    let tempo = |group: &str| {
        e.tempo
            .iter()
            .find(|r| r.group == group)
            .expect("a row")
            .clone()
    };
    let top = tempo("top players");
    // Red's contact is above zero from turn 2 in both games.
    assert_eq!(
        (top.games, top.contact_games, top.contact_turn),
        (2, 2, Some(2.0))
    );
    assert_eq!((top.decisive_games, top.decisive_turn), (1, Some(3.0)));
    assert_eq!((top.both_games, top.to_decisive), (1, Some(1.0)));
    // Blue: contact from turn 2, a decisive moment at ply 3 (turn 2).
    let us = tempo("us, same games");
    assert_eq!(
        (us.contact_turn, us.decisive_turn, us.to_decisive),
        (Some(2.0), Some(2.0), Some(0.0))
    );
}

#[test]
fn a_lead_is_binned_by_its_size_against_the_median_lead() {
    let leads = [
        (-10, false),
        (-2, false),
        (0, false),
        (3, true),
        (8, true),
        (20, true),
    ];
    let bins = lead_bins("m", 10, &leads);
    let got: Vec<(&str, usize, usize)> = bins
        .iter()
        .map(|b| (b.bin.as_str(), b.games, b.wins))
        .collect();
    // The sizes 10, 2, 3, 8 and 20 have a median of 8.
    assert_eq!(
        got,
        [
            ("behind by more than 8", 1, 0),
            ("behind by up to 8", 1, 0),
            ("level", 1, 0),
            ("ahead by up to 8", 2, 2),
            ("ahead by more than 8", 1, 1),
        ]
    );
    assert!(bins.iter().all(|b| b.metric == "m" && b.turn == 10));
    // All level: one bin; none: no rows.
    assert_eq!(lead_bins("m", 1, &[(0, true), (0, false)]).len(), 1);
    assert!(lead_bins("m", 1, &[]).is_empty());
}

#[test]
fn the_value_of_a_lead_reads_our_games_at_each_turn() {
    // Our (blue's) gross income is 4 at turn 1 against red's 4: level; at
    // turn 2 it is 6 against 8: behind by 2, and we lose game 1.
    let e = two_games().economy;
    let at = |turn: u32| -> Vec<&LeadRow> {
        e.lead
            .iter()
            .filter(|r| r.metric == "gross" && r.turn == turn)
            .collect()
    };
    assert_eq!(at(1).len(), 1);
    assert_eq!(
        (at(1)[0].bin.as_str(), at(1)[0].games, at(1)[0].wins),
        ("level", 2, 1)
    );
    // Both games: behind by 2 (the median size), one of them won.
    assert_eq!(
        (at(2)[0].bin.as_str(), at(2)[0].games, at(2)[0].wins),
        ("behind by up to 2", 2, 1)
    );
    // The trace's lead metric is the test game's "cells".
    assert!(e.lead.iter().any(|r| r.metric == "cells"));
}

#[test]
fn a_lower_is_better_metric_flips_the_lead() {
    struct Lower;
    impl Profiling for Lower {
        fn turns() -> Vec<u32> {
            vec![2]
        }
        fn opening_plies() -> usize {
            1
        }
        fn key_metrics() -> Vec<&'static str> {
            Vec::new()
        }
        fn events(_: &Trace, _: usize) -> Vec<String> {
            Vec::new()
        }
        fn opening(_: &Trace, _: usize, _: usize) -> Vec<String> {
            Vec::new()
        }
        fn lower_is_better(key: &str) -> bool {
            key == "gross"
        }
        fn compare_turns() -> Vec<u32> {
            vec![2]
        }
        fn resource() -> &'static str {
            "cells"
        }
        fn spending(_: &Trace, _: usize) -> Vec<(String, i64)> {
            Vec::new()
        }
        fn economy() -> Option<EconomyHooks> {
            Eco::economy()
        }
    }
    let a = analyze::<Lower>(
        &Input {
            games: vec![standard(1, 30, 1)],
            ..Input::default()
        },
        &Options {
            top: 0,
            players: vec!["alpha".to_string()],
        },
    );
    // Our gross 6 against 8: less is better, so we lead by 2: "ahead".
    let row = a
        .economy
        .lead
        .iter()
        .find(|r| r.metric == "gross" && r.turn == 2)
        .expect("a row");
    assert_eq!((row.bin.as_str(), row.wins), ("ahead by up to 2", 1));
}

#[test]
fn the_curves_hold_the_exchange_the_efficiencies_and_the_burden() {
    let e = two_games().economy;
    let set = &e.sets[0];
    assert_eq!(set.slug, "economy-top-vs-us");
    let keys: Vec<&str> = set.curves.iter().map(|c| c.key.as_str()).collect();
    assert_eq!(
        keys,
        [
            "exchange",
            "killed",
            "lost",
            "cells per resource",
            "kills per training",
            BURDEN
        ]
    );
    let exchange = &set.curves[0];
    // Turn 3: us -20 in both games, the top players +20 and +40.
    assert_eq!(exchange.lines[0][2].median, Some(-20.0));
    assert_eq!(exchange.lines[1][2].median, Some(30.0));
    let burden = set.curves.last().expect("a curve");
    assert!(burden.lower_is_better);
    // Turn 3, blue: 1 of 6.
    assert!((burden.lines[0][2].median.expect("a value") - 100.0 / 6.0).abs() < 1e-9);
    // The second set is our wins against our losses, one game each; and
    // each focus player has a set of its own, not charted.
    assert_eq!(e.sets[1].slug, "economy-wins-vs-losses");
    assert_eq!(e.sets[1].games, [1, 1]);
    assert_eq!(e.players.len(), 1);
    assert_eq!(e.players[0].slug, "economy-player-alpha");
}

#[test]
fn the_charts_include_the_economy_sets() {
    let a = two_games();
    let names: Vec<String> = chart::files(&a).into_iter().map(|f| f.0).collect();
    assert!(names.contains(&"economy-top-vs-us-exchange.svg".to_string()));
    assert!(names.contains(&"economy-top-vs-us-cells-per-resource.svg".to_string()));
    assert!(names.contains(&"economy-wins-vs-losses-upkeep-burden.svg".to_string()));
    assert!(!names.iter().any(|n| n.starts_with("economy-player")));
}

#[test]
fn the_verdict_puts_the_two_explanations_side_by_side_in_numbers() {
    let e = two_games().economy;
    let v = e
        .verdicts
        .iter()
        .find(|v| v.slug == "economy-top-vs-us")
        .expect("a verdict");
    assert_eq!(
        v.text,
        "us and top players, at turn 3 (n = 2 and 2): exchange us -20, top players +30; cells per resource us 1, top players 3; kills per training us 5, top players 40; upkeep burden us 16.667%, top players 30%."
    );
    // One verdict for each set, the players' too.
    assert_eq!(e.verdicts.len(), 3);
}

#[test]
fn a_game_without_economy_hooks_has_no_economy() {
    let a = analyze::<Toy>(
        &Input {
            games: vec![standard(1, 30, 0)],
            ..Input::default()
        },
        &Options {
            top: 0,
            players: vec!["alpha".to_string()],
        },
    );
    assert_eq!(a.economy, Economy::default());
    // The files are still written, with their headers.
    let files = report_files(&a, 0);
    let (_, exchange) = files
        .iter()
        .find(|f| f.0 == "economy_exchange.csv")
        .expect("a file");
    assert_eq!(exchange, "group,phase,games,killed,lost,balance\n");
    let (_, signals) = files.iter().find(|f| f.0 == "signals.md").expect("a file");
    assert!(signals.contains("No economy numbers"));
}

#[test]
fn the_tables_and_signals_carry_the_numbers() {
    let a = two_games();
    let files = report_files(&a, 4);
    let text = |name: &str| {
        files
            .iter()
            .find(|f| f.0 == name)
            .map(|f| f.1.clone())
            .unwrap_or_else(|| panic!("no file {name}"))
    };
    assert!(text("economy_exchange.csv").contains("top players,early game,2,40,10,30\n"));
    assert!(text("economy_efficiency.csv").contains("top players,cells per resource,2,2,2\n"));
    assert!(text("economy_mix.csv").contains("top players,early game,6,economy,0.667,66.667\n"));
    assert!(text("economy_upkeep.csv").contains("top players,early game,6,25\n"));
    assert!(text("economy_tempo.csv").contains("top players,2,2,2,0,,0,\n"));
    assert!(text("economy_lead.csv").contains("gross,1,level,2,1\n"));
    assert!(text("economy_curves.csv").contains("economy-top-vs-us,exchange,3,-20,30,2,2\n"));
    let signals = text("signals.md");
    assert!(signals.contains("## Economy or fighting"));
    assert!(signals.contains("exchange us -20, top players +30"));
    assert!(signals.contains("- top players, early game: economy 67%, army 33% (6 plies)."));
    assert!(signals.contains("first contact at turn 2 (2 of 2 games)"));
    assert!(signals.contains("- gross at turn 1: level: 1 of 2."));
    assert!(text("README.md").contains("## Economy or fighting"));
}
