use ice_and_fire_engine::{League, Outcome, State, CELLS, SIZE};

use super::*;
use crate::test_util::{random_positions, ready_to_win, unit_value, won_state, Position};

fn train(level: u8, x: u8, y: u8) -> Command {
    Command::Train { level, x, y }
}

/// Red holds (0, 0) and (1, 0). Blue's territory is a snake from its HQ along
/// the bottom row and up column 2 to (2, 0), next to red, and from there a
/// dead end east: a mine at (3, 0), a level 1 unit at (4, 0) and a level 2
/// unit at (5, 0). The cell (2, 0) is the only one joining the dead end to
/// blue's HQ: taking it cuts off four cells, two units and a mine.
fn dead_end() -> State {
    let column = "##X#########";
    Position {
        rows: [
            "OOXXXX######",
            column,
            column,
            column,
            column,
            column,
            column,
            column,
            column,
            column,
            column,
            "##XXXXXXXXXX",
        ],
        gold: [50, 20],
        buildings: &[(1, 1, 3, 0)],
        units: &[(1, 1, 1, 4, 0), (1, 2, 2, 5, 0)],
    }
    .state()
}

fn apply_all(state: &State, commands: &[Command]) -> State {
    let mut after = state.clone();
    for (index, &command) in commands.iter().enumerate() {
        assert!(after.apply(command), "command {index} {command:?} refused");
    }
    after
}

/// The fields of a cut, measured again from the states.
fn measure(before: &State, commands: &[Command]) -> Cut {
    let me = before.to_act();
    let opp = 1 - me;
    let after = apply_all(before, commands);
    let buildings =
        |state: &State| i64::from(((state.mines() | state.towers()) & state.active(opp)).count());
    let cost: i64 = commands
        .iter()
        .map(|command| match command {
            Command::Train { level, .. } => 10 * i64::from(*level),
            other => panic!("not a training: {other:?}"),
        })
        .sum();
    let cells = i64::from(before.active(opp).count()) - i64::from(after.active(opp).count());
    let units = unit_value(before, opp) - unit_value(&after, opp);
    let buildings = buildings(before) - buildings(&after);
    let income = after.income(me) - before.income(me);
    let score = if after.outcome() == Some(Outcome::Win(me)) {
        i64::MAX / 2
    } else {
        3 * cells + units + 10 * buildings + 2 * income - cost
    };
    Cut {
        commands: commands.to_vec(),
        cost,
        cells,
        units,
        buildings,
        income,
        score,
    }
}

#[test]
fn the_cut_that_disconnects_a_region_with_units_comes_first() {
    let state = dead_end();
    let found = cuts(&state, 4, 50, 8);
    assert!(!found.is_empty());
    assert_eq!(
        found[0],
        Cut {
            commands: vec![train(1, 2, 0)],
            cost: 10,
            cells: 4,
            units: 30,
            buildings: 1,
            income: 0,
            score: 3 * 4 + 30 + 10 - 10,
        }
    );
    // The scores are in decreasing order, and the fields are what the
    // commands do.
    for pair in found.windows(2) {
        assert!(pair[0].score >= pair[1].score);
    }
    for cut in &found {
        assert_eq!(*cut, measure(&state, &cut.commands));
    }
}

#[test]
fn the_budget_the_limit_and_the_length_bound_the_cuts() {
    let state = dead_end();
    assert!(cuts(&state, 4, 9, 8).is_empty(), "no gold for a training");
    assert!(cuts(&state, 4, 0, 8).is_empty());
    assert!(cuts(&state, 4, -10, 8).is_empty());
    assert!(cuts(&state, 0, 50, 8).is_empty());
    assert!(cuts(&state, 4, 50, 0).is_empty());
    assert_eq!(cuts(&state, 4, 50, 1).len(), 1);
    let one = cuts(&state, 1, 50, 8);
    assert!(one.iter().all(|cut| cut.commands.len() == 1));
    assert_eq!(one.len(), 1, "only (2, 0) is next to red's territory");
    let long = cuts(&state, 4, 50, 100);
    assert!(long.iter().any(|cut| cut.commands.len() > 1));
    assert!(long.iter().all(|cut| cut.cost <= 50));
    let cheap = cuts(&state, 4, 20, 100);
    assert!(cheap.iter().all(|cut| cut.cost <= 20));
}

#[test]
fn the_same_set_of_cells_is_kept_once() {
    let state = dead_end();
    let found = cuts(&state, 4, 50, 1000);
    let mut sets: Vec<Vec<usize>> = found
        .iter()
        .map(|cut| {
            let mut cells: Vec<usize> = cut
                .commands
                .iter()
                .map(|command| match *command {
                    Command::Train { x, y, .. } => usize::from(y) * SIZE + usize::from(x),
                    other => panic!("not a training: {other:?}"),
                })
                .collect();
            cells.sort_unstable();
            cells
        })
        .collect();
    let all = sets.len();
    sets.sort();
    sets.dedup();
    assert_eq!(sets.len(), all);
}

#[test]
fn taking_the_hq_scores_highest() {
    let state = ready_to_win();
    let found = cuts(&state, 2, 25, 8);
    assert_eq!(found[0].score, i64::MAX / 2);
    assert_eq!(found[0].commands, vec![train(1, 11, 10), train(1, 11, 11)]);
    assert_eq!(found[0].cost, 20);
    assert_eq!(found[0].cells, 1);
    // Taking the cell next to the HQ alone removes nothing.
    assert_eq!(found.len(), 1);
}

#[test]
fn a_finished_game_has_no_cuts() {
    assert!(cuts(&won_state(), 4, 100, 8).is_empty());
}

#[test]
fn nothing_to_cut_gives_nothing() {
    let state = State::new(&[false; CELLS], &[], League::Bronze);
    assert!(cuts(&state, 4, 20, 8).is_empty());
}

#[test]
fn the_search_stops_at_the_node_limit() {
    // On an open map from a start with a lot of gold the search is large; it
    // must return, deterministically, with results that apply.
    let mut state = State::new(&[false; CELLS], &[], League::Bronze);
    let mut commands = Vec::new();
    let mut rng = cg_core::rng::Rng::new(3);
    for _ in 0..30 {
        state.random_turn(&mut rng, &mut commands);
        state.play_turn(&commands);
    }
    if state.outcome().is_none() {
        let found = cuts(&state, 6, 10_000, 8);
        assert_eq!(found, cuts(&state, 6, 10_000, 8));
        for cut in &found {
            apply_all(&state, &cut.commands);
        }
    }
}

const SEEDS: u64 = if cfg!(debug_assertions) { 60 } else { 200 };

const PLIES: [u32; 5] = [0, 4, 10, 20, 40];

const LEAGUES: [League; 4] = [League::Wood3, League::Wood2, League::Wood1, League::Bronze];

#[test]
fn every_cut_of_a_random_position_applies_and_matches_its_measure() {
    let mut total = 0;
    for seed in 0..SEEDS {
        let league = LEAGUES[(seed % 4) as usize];
        for state in random_positions(seed, league, &PLIES) {
            let gold = state.gold(state.to_act());
            let found = cuts(&state, 3, gold, 8);
            assert_eq!(
                found,
                cuts(&state, 3, gold, 8),
                "seed {seed}: deterministic"
            );
            assert!(found.len() <= 8);
            for pair in found.windows(2) {
                assert!(pair[0].score >= pair[1].score, "seed {seed}");
                if pair[0].score == pair[1].score {
                    assert!(pair[0].cost <= pair[1].cost, "seed {seed}");
                }
            }
            for cut in &found {
                total += 1;
                assert!(cut.cost <= gold, "seed {seed}");
                assert!(!cut.commands.is_empty() && cut.commands.len() <= 3);
                assert!(cut.cells > 0 || cut.units > 0, "seed {seed}");
                assert_eq!(*cut, measure(&state, &cut.commands), "seed {seed}");
            }
        }
    }
    assert!(total > 100, "{total} cuts");
}

#[test]
fn the_cuts_of_late_positions_apply() {
    for seed in 0..20 {
        for state in random_positions(seed, League::Bronze, &[60, 80, 99]) {
            let gold = state.gold(state.to_act());
            for cut in &cuts(&state, 4, gold, 8) {
                apply_all(&state, &cut.commands);
            }
        }
    }
}
