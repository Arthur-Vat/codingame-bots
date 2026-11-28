use ice_and_fire_engine::{Bits, Command, CELLS};

use crate::replay::replay;
use crate::test_util::random_record;

use super::*;

#[test]
fn a_side_counts_what_the_rules_accepted_and_its_times() {
    let game = replay("g", random_record(5, ["x", "y"]));
    let sides = sides(&game);
    for seat in 0..2 {
        let side = &sides[seat];
        assert_eq!(side.bot, ["x", "y"][seat]);
        let mut towers = 0;
        let mut trained = [0; 3];
        let mut turns = 0;
        for step in game.steps.iter().filter(|s| s.state.to_act() == seat) {
            turns += 1;
            let mut sim = step.state.clone();
            for &command in &step.commands {
                if sim.apply(command) {
                    match command {
                        Command::BuildTower { .. } => towers += 1,
                        Command::Train { level, .. } => trained[usize::from(level) - 1] += 1,
                        _ => {}
                    }
                }
            }
        }
        assert_eq!(side.towers, towers);
        assert_eq!(side.trained, trained);
        assert_eq!(side.first_ms, Some(seat as f64 + 1.0));
        assert_eq!(side.ms.len(), turns - 1);
        assert_eq!(side.kept.len(), turns - 1);
        assert!(side.kept_share.iter().all(|&k| (0..=100).contains(&k)));
    }
    let total: f64 = sides.iter().map(|s| s.points).sum();
    assert_eq!(total, 1.0);
}

#[test]
fn the_report_has_a_row_a_bot_with_its_scores_by_seat() {
    let games: Vec<[Side; 2]> = (1..5)
        .map(|seed| {
            let names = if seed % 2 == 0 {
                ["x", "y"]
            } else {
                ["y", "x"]
            };
            sides(&replay("g", random_record(seed, names)))
        })
        .collect();
    let text = report(&games);
    assert!(text.contains("4 games."), "{text}");
    // The first table only: the progress tables follow it.
    let rows: Vec<&str> = text
        .split("## Progress")
        .next()
        .unwrap_or_default()
        .lines()
        .filter(|l| l.starts_with("| x |") || l.starts_with("| y |"))
        .collect();
    assert_eq!(rows.len(), 2, "{text}");
    assert!(rows[0].starts_with("| x | 4 |"), "{text}");
}

#[test]
fn the_crowding_counts_own_units_next_to_each_and_units_with_no_free_neutral_cell() {
    // The measure of each ply of game turns 5 to 12 follows the definitions,
    // read cell by cell.
    let game = replay("g", random_record(7, ["x", "y"]));
    let sides = sides(&game);
    let mut read_some = false;
    for (seat, mine) in sides.iter().enumerate() {
        for turn in CROWD_TURNS {
            let step = game
                .steps
                .iter()
                .find(|s| s.state.to_act() == seat && s.state.game_turn() == turn);
            let read = mine.crowd[(turn - *CROWD_TURNS.start()) as usize];
            let Some(step) = step else {
                assert_eq!(read, None);
                continue;
            };
            read_some = true;
            let state = &step.state;
            let (mut units, mut adjacent, mut shut) = (0, 0, 0);
            for cell in state.units(seat).iter() {
                units += 1;
                let mut free = false;
                for other in (0..CELLS).filter(|&c| Bits::cell(cell).neighbours().test(c)) {
                    adjacent += usize::from(state.units(seat).test(other));
                    free |= state.map().valid.test(other)
                        && !state.owned(0).test(other)
                        && !state.owned(1).test(other)
                        && !state.units(0).test(other)
                        && !state.units(1).test(other);
                }
                shut += usize::from(!free);
            }
            let expected = Crowd {
                units,
                adjacent,
                shut,
            };
            assert_eq!(read, Some(expected), "seat {seat} turn {turn}");
        }
        if let Some(step) = game
            .steps
            .iter()
            .find(|s| s.state.to_act() == seat && s.state.game_turn() == 12)
        {
            let state = &step.state;
            assert_eq!(mine.units12, Some(state.units(seat).count() as usize));
            let lead =
                i64::from(state.owned(seat).count()) - i64::from(state.owned(1 - seat).count());
            assert_eq!(mine.lead12, Some(lead));
        }
    }
    assert!(read_some, "the game is too short");
}

#[test]
fn the_report_gives_the_progress_tables() {
    let games: Vec<[Side; 2]> = (1..4)
        .map(|seed| sides(&replay("g", random_record(seed, ["x", "y"]))))
        .collect();
    let text = report(&games);
    assert!(text.contains("## Progress"), "{text}");
    assert!(text.contains("Lead at 12 (median)"), "{text}");
    assert!(text.contains("| x |"), "{text}");
}
