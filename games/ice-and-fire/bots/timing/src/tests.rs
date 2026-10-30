use std::time::Duration;

use cg_core::rng::Rng;
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

use super::*;

#[test]
fn the_ramp_holds_answers_a_millisecond_longer_each_turn() {
    assert_eq!(target(1), None);
    assert_eq!(
        target(RAMP_FROM),
        Some(Duration::from_millis(RAMP_START_MS))
    );
    assert_eq!(
        target(RAMP_FROM + 8),
        Some(Duration::from_millis(RAMP_START_MS + 8))
    );
    // CodinGame's 50 ms is reached by turn 10, and the ramp stops growing.
    assert_eq!(target(10), Some(Duration::from_millis(50)));
    assert_eq!(target(1_000), Some(Duration::from_millis(RAMP_MAX_MS)));
}

#[test]
fn the_workload_reads_its_position_and_works_until_the_deadline() {
    let state = work_state().expect("the workload's position reads");
    assert!(state.outcome().is_none());
    assert!(unit(&state) > 0);
    let until = Instant::now() + Duration::from_millis(20);
    // How long the units took depends on the machine's load, so only that
    // some were done and that the deadline was reached are checked.
    let (done, _) = work_until(&state, until);
    assert!(done > 0);
    assert!(Instant::now() >= until);
    // A deadline already passed does no unit.
    let (done, _) = work_until(&state, Instant::now());
    assert_eq!(done, 0);
}

#[test]
fn the_map_reads_as_the_rules_check_bot_writes_it() {
    let map = generate(3, RefereeLeague::Bronze);
    let state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let text = map_text(&state);
    let (grid, spots) = text.split_at(SIZE * SIZE);
    assert_eq!(
        grid.chars().filter(|&c| c == '#').count(),
        map.void.iter().filter(|&&v| v).count()
    );
    assert!(grid.chars().all(|c| c == '#' || c == '.'));
    assert_eq!(spots.split_whitespace().count(), map.mine_spots.len());
}

#[test]
fn each_line_holds_every_turn_so_far() {
    let mut game = Game {
        seat: "red",
        map: "#.. 1,2".to_string(),
        turns: Vec::new(),
    };
    game.turns.push(Turn {
        turn: 1,
        target: None,
        read: 0.04,
        gap: None,
        policy: 0.3,
        answer: Some(500.37),
        speed: Some(18.94),
    });
    game.turns.push(Turn {
        turn: 2,
        target: Some(42),
        read: 0.05,
        gap: Some(101.26),
        policy: 0.1,
        answer: None,
        speed: None,
    });
    assert_eq!(
        game.line(),
        "timing v1 red | map #.. 1,2 | turn:target/read/gap/policy/answer/speed | \
         1:-/0.04/-/0.3/500.37/18.9 2:42/0.05/101.3/0.1/-/-"
    );
}

#[test]
fn the_bot_never_takes_the_enemy_hq_and_keeps_the_rest_of_its_turn() {
    // Greedy turns (red) against random turns (blue) on the referee's maps:
    // wherever the greedy turn would win, the peaceful one does not, and
    // where it would not, the peaceful one is the same turn.
    let mut wins_dropped = 0;
    for seed in 0..20u64 {
        let map = generate(seed, RefereeLeague::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let mut rng = Rng::new(seed ^ 0x9e37_79b9);
        let mut commands = Vec::new();
        for _ in 0..200 {
            if state.outcome().is_some() {
                break;
            }
            if state.to_act() == 0 {
                greedy_turn(&state, &mut commands);
                let mut greedy = state.clone();
                for &command in &commands {
                    greedy.apply(command);
                }
                let kept = peaceful(&state, &commands);
                let mut after = state.clone();
                for &command in &kept {
                    assert!(after.apply(command), "{command:?}");
                }
                assert_ne!(after.outcome(), Some(Outcome::Win(0)));
                if greedy.outcome() == Some(Outcome::Win(0)) {
                    wins_dropped += 1;
                } else {
                    assert_eq!(kept, commands);
                }
                state.play_turn(&kept);
            } else {
                state.random_turn(&mut rng, &mut commands);
                state.play_turn(&commands);
            }
        }
    }
    assert!(wins_dropped > 0, "no greedy win was found to drop");
}
