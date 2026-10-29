use std::fs;

use crate::test_util::{random_record, temp_dir};
use crate::trace::trace;

use super::*;

#[test]
fn a_replay_has_the_traces_plies_and_their_answers() {
    for seed in 1..4 {
        let record = random_record(seed, ["a", "b"]);
        let plies = trace("g", &record).plies.len();
        let game = replay("g", record);
        assert_eq!(game.steps.len(), plies);
        assert!(game.end.outcome().is_some());
        for (ply, step) in game.steps.iter().enumerate() {
            assert_eq!(step.ms, Some(ply as f64 + 1.0));
            // Each ply starts where the one before ended.
            if let Some(next) = game.steps.get(ply + 1) {
                let mut state = step.state.clone();
                state.play_turn(&step.commands);
                assert_eq!(state.turn_input(0), next.state.turn_input(0));
            }
        }
    }
}

#[test]
fn records_come_back_in_order_whatever_the_jobs() {
    let dir = temp_dir("replay");
    let mut paths = Vec::new();
    for seed in 1..6 {
        let path = dir.join(format!("game-{seed}.json"));
        let text = serde_json::to_string(&random_record(seed, ["a", "b"])).expect("json");
        fs::write(&path, text).expect("a record");
        paths.push(path);
    }
    let one = map_records(&paths, 1, |g| (g.name.clone(), g.steps.len())).expect("replays");
    let three = map_records(&paths, 3, |g| (g.name.clone(), g.steps.len())).expect("replays");
    assert_eq!(one, three);
    assert_eq!(one[0].0, "game-1");
    assert_eq!(one[4].0, "game-5");
    paths.push(dir.join("missing.json"));
    assert!(map_records(&paths, 2, |g| g.steps.len()).is_err());
    fs::remove_dir_all(&dir).expect("cleaned up");
}
