use ice_and_fire_engine::Command;

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
    let rows: Vec<&str> = text
        .lines()
        .filter(|l| l.starts_with("| x |") || l.starts_with("| y |"))
        .collect();
    assert_eq!(rows.len(), 2, "{text}");
    assert!(rows[0].starts_with("| x | 4 |"), "{text}");
}
