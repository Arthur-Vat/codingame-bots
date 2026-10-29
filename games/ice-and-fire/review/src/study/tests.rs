use std::collections::BTreeMap;

use ice_and_fire_engine::{Command, League, State, SIZE};

use crate::replay::replay;
use crate::test_util::random_record;

use super::*;

#[test]
fn turns_to_afford_count_whole_incomes() {
    assert_eq!(turns_to_afford(10, 5, Some(30)), Some(4));
    assert_eq!(turns_to_afford(10, 5, Some(31)), Some(5));
    assert_eq!(turns_to_afford(30, 0, Some(30)), Some(0));
    assert_eq!(turns_to_afford(0, 0, Some(10)), None);
    assert_eq!(turns_to_afford(0, -3, Some(10)), None);
    assert_eq!(turns_to_afford(100, 5, None), None);
}

#[test]
fn the_pressure_is_the_shared_evaluations() {
    // The owner's example: 3 gold, +17 income, a chain of 140.
    assert_eq!(pressure(3, 17, Some(140)), -120);
    assert_eq!(pressure(3, 17, Some(180)), -160);
    assert_eq!(pressure(50, 10, Some(40)), 0);
    assert_eq!(pressure(0, 0, Some(500)), -200);
    assert_eq!(pressure(0, 0, None), -200);
}

#[test]
fn the_fit_finds_the_curve_it_is_given() {
    let mut counts = BTreeMap::new();
    for k in 1..=10i64 {
        let n = 10_000usize;
        let p = 0.8 * 0.7f64.powi((k - 1) as i32);
        counts.insert(k, (n, (n as f64 * p).round() as usize));
    }
    counts.insert(0, (10, 0));
    counts.insert(20, (10, 10));
    let (p, r, _) = fit_geometric(&counts, 10).expect("a fit");
    assert!((p - 0.8).abs() < 0.011, "{p}");
    assert!((r - 0.7).abs() < 0.011, "{r}");
}

/// Blue to act, as blue sees it: red's HQ at (0, 0) with cells (1, 0),
/// (2, 0), (0, 1) to (2, 1), a tower on (2, 0) and, with `second`, another
/// on (1, 1); blue holds column 3 and row 11 from it to its HQ, with a level
/// `level` unit on (3, 0) and `gold`.
fn blue_to_act(level: u8, gold: i64, second: bool) -> State {
    let mut text = format!("{gold}\n10\n20\n5\n");
    for y in 0..SIZE {
        let row: String = (0..SIZE)
            .map(|x| match (x, y) {
                (0..=2, 0..=1) => 'X',
                (3, _) => 'O',
                (4.., 11) => 'O',
                _ => '.',
            })
            .collect();
        text.push_str(&row);
        text.push('\n');
    }
    let towers = if second {
        "1 2 2 0\n1 2 1 1\n"
    } else {
        "1 2 2 0\n"
    };
    text.push_str(&format!(
        "{}\n0 0 11 11\n1 0 0 0\n{towers}",
        if second { 4 } else { 3 }
    ));
    text.push_str(&format!("1\n0 1 {level} 3 0\n"));
    State::read("0\n", &text, League::Bronze, 10).expect("a valid position")
}

#[test]
fn a_tower_blocks_what_it_alone_keeps_out() {
    let cell = |x: usize, y: usize| y * SIZE + x;
    let state = blue_to_act(1, 50, false);
    assert_eq!(state.to_act(), 1);
    let cover = sole_cover(&state, 0, cell(2, 0));
    assert!(cover.test(cell(2, 0)) && cover.test(cell(1, 0)) && cover.test(cell(2, 1)));
    assert_eq!(cover.count(), 3);
    assert_eq!(blocks(&state, 0, cell(2, 0)), (true, true));
    // Without gold, only the unit is kept out; a level 3 unit passes towers.
    assert_eq!(
        blocks(&blue_to_act(1, 5, false), 0, cell(2, 0)),
        (false, true)
    );
    assert_eq!(
        blocks(&blue_to_act(3, 5, false), 0, cell(2, 0)),
        (false, false)
    );
    // A second tower next to (1, 0) and (2, 1) protects them too: the first
    // alone protects its own cell only.
    let two = blue_to_act(1, 50, true);
    assert_eq!(sole_cover(&two, 0, cell(2, 0)).count(), 1);
    assert_eq!(blocks(&two, 0, cell(2, 0)), (true, true));
}

#[test]
fn the_study_follows_every_tower_threat_and_cut() {
    let mut towers = 0;
    for seed in 1..10 {
        let game = replay("g", random_record(seed, ["a", "b"]));
        let s = study(&game);
        assert_eq!(s.threats.len(), game.steps.len());
        assert_eq!(s.cuts.len(), game.steps.len());
        let mut built = 0;
        for step in &game.steps {
            let mut sim = step.state.clone();
            for &command in &step.commands {
                built += usize::from(
                    sim.apply(command) && matches!(command, Command::BuildTower { .. }),
                );
            }
        }
        assert_eq!(s.towers.len(), built, "seed {seed}");
        towers += built;
        for t in &s.towers {
            // A tower only makes cells dearer.
            if let (Some(before), Some(after)) = (t.cost_before, t.cost_after) {
                assert!(after >= before, "seed {seed}: {t:?}");
            }
            assert_eq!(t.won, game.record.winner == Some(t.builder));
            assert!(pressure_gain(t) >= 0, "{t:?}");
            assert!(t.enemy_turns.is_none_or(|n| n >= 1));
            for later in [t.block, t.unit_block, t.afforded] {
                assert!(later.is_none_or(|n| n >= 1), "{t:?}");
            }
        }
        for (ply, c) in s.cuts.iter().enumerate() {
            let defender = 1 - c.attacker;
            let before = game.steps[ply].state.active(defender).count();
            let after = game
                .steps
                .get(ply + 1)
                .map_or(&game.end, |s| &s.state)
                .active(defender)
                .count();
            assert_eq!(c.lost[0], Some(i64::from(before) - i64::from(after)));
            // The cut finder keeps its best cuts by score, so more gold need
            // not find more cells; the same gold finds the same cut.
            let income = game.steps[ply].state.income(c.attacker);
            if income <= 0 {
                assert_eq!((c.cells[0], c.cost[0]), (c.cells[2], c.cost[2]));
            }
            assert!(c
                .cost
                .iter()
                .zip(&c.cells)
                .all(|(&cost, &cells)| cells > 0 || cost == 0));
        }
        for t in &s.threats {
            assert_eq!(t.taken_in.is_some(), s.hq_taken_by == Some(t.attacker));
            assert_eq!(t.turns, turns_to_afford(t.gold, t.income, t.cost));
        }
    }
    assert!(towers > 0, "no tower in the random games");
}

#[test]
fn the_report_and_the_csv_cover_the_three_parts() {
    let games: Vec<GameStudy> = (1..4)
        .map(|seed| study(&replay("g", random_record(seed, ["a", "b"]))))
        .collect();
    let text = report(&games);
    for heading in [
        "# Towers and threats",
        "## Towers",
        "## Threats to the HQ",
        "## Cuts",
    ] {
        assert!(text.contains(heading), "{heading}");
    }
    let files = csv(&games);
    assert_eq!(files.len(), 3);
    let threats = &files[1].1;
    let rows: usize = games.iter().map(|g| g.threats.len()).sum();
    assert_eq!(threats.lines().count(), rows + 1);
}
