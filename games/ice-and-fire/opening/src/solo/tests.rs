use ice_and_fire_engine::{League, State};

use super::{beam, plan, play, severed, shape, value, Objective, REST, SETS};
use crate::econ::{Buy, Params};
use crate::plan::Step;
use crate::test_util::start;

/// Red alone holds (0, 0) to (3, 0), then (3, 1): one line from its HQ.
fn line() -> State {
    let mut turn = String::from("0\n5\n0\n1\n");
    turn.push_str("OOOO........\n...O........\n");
    for _ in 0..9 {
        turn.push_str("............\n");
    }
    turn.push_str("...........X\n");
    turn.push_str("2\n0 0 0 0\n1 0 11 11\n0\n");
    State::read("0\n", &turn, League::Bronze, 1).expect("a valid position")
}

#[test]
fn a_line_has_a_joint_at_every_cell_but_its_tip() {
    let state = line();
    let cut = severed(&state, 0);
    // (1, 0) cuts off (2, 0), (3, 0) and (3, 1); (2, 0) two; (3, 0) one.
    assert_eq!((cut[1], cut[2], cut[3], cut[15]), (3, 2, 1, 0));
    assert_eq!(cut[0], 0);
    let s = shape(&state, 0);
    assert_eq!((s.cells, s.joints, s.cut, s.tips), (5, 3, 3, 1));
    // From (3, 1) to (11, 11): 8 + 10 steps.
    assert_eq!(s.reach, 18);
    let o = Objective::DEFAULT;
    // Blue holds only its HQ: red is nearer most of the map.
    assert_eq!(s.voronoi, 91);
    assert_eq!(
        value(&state, 0, &o),
        4 * 5 + 3 * 5 - 2 * 3 - 2 * 3 - 2 - 6 * 18 + 91
    );
}

#[test]
fn the_beam_finds_a_plan_at_least_as_good_as_the_rules() {
    let sets = [Params::V003, Params::V003.with_shape(2, 6, 0)];
    let rest = Step {
        weights: 0,
        buy: Buy::Payback,
    };
    for seed in [1, 2, 3] {
        let state = start(seed);
        let o = Objective::DEFAULT;
        let rules = value(&play(&state, &sets, &[], rest, 7), 0, &o);
        let plan = beam(&state, &sets, rest, 7, 20, None, &o);
        assert!(plan.len() <= 7);
        let planned = value(&play(&state, &sets, &plan, rest, 7), 0, &o);
        assert!(planned >= rules, "seed {seed}: {planned} < {rules}");
        // The same search gives the same plan.
        assert_eq!(beam(&state, &sets, rest, 7, 20, None, &o), plan);
    }
}

#[test]
fn the_anytime_plan_is_never_worse_than_its_first_beam() {
    let state = start(4);
    let o = Objective::DEFAULT;
    let rated = |p: &[Step]| value(&play(&state, &SETS, p, REST, 7), 0, &o);
    let first = beam(&state, &SETS, REST, 7, 2, None, &o);
    let anytime = plan(&state, &SETS, REST, 7, 8, None, &o);
    assert!(rated(&anytime) >= rated(&first));
    assert!(rated(&anytime) >= rated(&[]));
}
