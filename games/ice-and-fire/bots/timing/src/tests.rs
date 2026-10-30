use std::time::Duration;

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
fn the_workload_reads_its_position_and_works() {
    let state = work_state().expect("the workload's position reads");
    assert!(state.outcome().is_none());
    assert!(unit(&state) > 0);
    let (done, took) = workload(Duration::from_millis(5)).expect("a workload");
    assert!(done > 0);
    assert!(took >= Duration::from_millis(5));
}
