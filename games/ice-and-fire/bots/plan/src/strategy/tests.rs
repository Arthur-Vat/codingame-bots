use ice_and_fire_engine::State;

use super::objectives::{counted, objectives, Kind, Objective, OBJECTIVE_MIN_VALUE};
use super::race::{race, Arrival, How, Race};
use super::roles::{assign, Memory};
use super::STRATEGY;
use crate::tests::position;

/// The corridor: red holds the top row and the right column down to (11, 9),
/// blue's HQ is alone at (11, 11) and (11, 10) is neutral.
const CORRIDOR: [&str; 12] = [
    "OOOOOOOOOOOO",
    "###########O",
    "###########O",
    "###########O",
    "###########O",
    "###########O",
    "###########O",
    "###########O",
    "###########O",
    "###########O",
    "###########.",
    "###########X",
];

fn cell(x: usize, y: usize) -> usize {
    y * 12 + x
}

fn objective(state: &State, seat: usize, at: usize) -> Objective {
    *objectives(state, seat)
        .iter()
        .find(|o| o.cell == at)
        .expect("the cell is an objective")
}

/// Objectives for a hand-made list, as `objectives` would give them.
fn goal(at: usize, value: i64) -> Objective {
    Objective {
        cell: at,
        kind: Kind::Band,
        held: false,
        value,
    }
}

fn arrival(turns: u32, how: How) -> Option<Arrival> {
    Some(Arrival { turns, how })
}

fn race_of(at: usize, ours: Option<Arrival>, theirs: Option<Arrival>) -> Race {
    let t = |a: Option<Arrival>| a.map_or(100, |a| i64::from(a.turns));
    Race {
        cell: at,
        ours,
        ours_training: ours.filter(|a| matches!(a.how, How::Train(_))),
        theirs,
        margin: t(theirs) - t(ours),
    }
}

#[test]
fn a_corridor_choke_decides_everything_behind_it() {
    let state = position(CORRIDOR, [0, 0], &[]);
    // Every choke of the corridor is red's and decides the same land: red's
    // 21 cells are nearer red with it, and the only cell red would lose to
    // blue were blue to hold it is the choke itself (the neutral cell
    // (11, 10) is level between the HQs either way).
    let all = objectives(&state, 0);
    assert!(!all.is_empty());
    assert!(all.iter().all(|o| o.kind == Kind::Choke && o.held));
    assert!(all.iter().all(|o| o.value == 20));
    // For blue, the same cell is worth the opposite swing.
    let at = all[0].cell;
    let blue = objective(&state, 1, at);
    assert!(!blue.held);
    assert_eq!(blue.value, -20);
}

#[test]
fn objectives_are_sorted_and_counted_by_value() {
    let state = position(CORRIDOR, [0, 0], &[]);
    let all = objectives(&state, 0);
    assert!(all
        .windows(2)
        .all(|w| (-w[0].value, w[0].cell) < (-w[1].value, w[1].cell)));
    assert!(counted(&all).iter().all(|o| o.value >= OBJECTIVE_MIN_VALUE));
    assert!(counted(&all).len() < all.len() || all.iter().all(|o| o.value >= 3));
    // Held cells say so.
    assert!(all
        .iter()
        .filter(|o| o.held)
        .all(|o| state.active(0).test(o.cell)));
}

/// Open ground: red's HQ in a corner, blue's in the other; `units` as given.
const OPEN: [&str; 12] = [
    "O...........",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
    "...........X",
];

/// Red holds rows 0 to 3, blue's HQ is alone in the far corner.
const LAND: [&str; 12] = [
    "OOOOOOOOOOOO",
    "OOOOOOOOOOOO",
    "OOOOOOOOOOOO",
    "OOOOOOOOOOOO",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
    "...........X",
];

#[test]
fn arrival_by_a_unit_or_by_a_training_with_the_wait() {
    let far = cell(8, 8);
    let obj = [goal(far, 10)];
    // Rich: a training on row 4 under the objective, 1 + 4.
    let state = position(LAND, [30, 0], &[]);
    let r = race(&state, 0, &obj)[0];
    assert_eq!(r.ours, arrival(5, How::Train(cell(8, 4))));
    // A unit of ours nearer than that wins; a tie goes to the unit.
    let near = [goal(cell(3, 5), 10)];
    let state = position(LAND, [30, 0], &[(0, 5, 1, 3, 3)]);
    let r = race(&state, 0, &near)[0];
    assert_eq!(r.ours, arrival(2, How::Unit(5)));
    assert_eq!(r.ours_training, arrival(2, How::Train(cell(3, 4))));
    // Poor with income 1 (the HQ alone): 10 gold needs 6 turns more.
    let state = position(OPEN, [4, 4], &[]);
    let r = race(&state, 0, &obj)[0];
    let d = u32::from(state.map().distance(cell(1, 0), far));
    assert_eq!(state.income(0), 1);
    assert_eq!(r.ours.map(|a| a.turns), Some(6 + 1 + d));
    // The enemy, with the same gold and income, is on the corner HQ.
    assert!(r.theirs.is_some());
}

#[test]
fn a_tie_goes_to_us_and_a_late_side_loses() {
    let at = cell(10, 7);
    let obj = [goal(at, 10)];
    // Our unit and the enemy's both 5 steps away: margin 0, ours.
    let state = position(LAND, [0, 0], &[(0, 1, 1, 9, 3), (1, 2, 1, 11, 11)]);
    let r = race(&state, 0, &obj)[0];
    assert_eq!(r.ours, arrival(5, How::Unit(1)));
    assert_eq!(r.theirs, arrival(5, How::Unit(2)));
    assert_eq!(r.margin, 0);
    // Not held objectives only.
    let held = [Objective {
        held: true,
        ..obj[0]
    }];
    assert!(race(&state, 0, &held).is_empty());
    let roles = assign(&state, 0, &obj, &[r], &mut Memory::default());
    assert_eq!(roles.frontline, vec![(1, at)]);
    // An objective the enemy reaches first: nobody is sent.
    let near_enemy = [goal(cell(10, 9), 10)];
    let r = race(&state, 0, &near_enemy)[0];
    assert_eq!(r.margin, -4);
    let roles = assign(&state, 0, &near_enemy, &[r], &mut Memory::default());
    assert!(roles.frontline.is_empty());
    assert_eq!(roles.back, vec![1]);
}

#[test]
fn one_frontline_unit_per_objective() {
    let (a, b) = (cell(6, 6), cell(2, 6));
    let obj = [goal(a, 9), goal(b, 8)];
    let state = position(
        LAND,
        [0, 0],
        &[(0, 1, 1, 6, 3), (0, 2, 1, 5, 3), (0, 3, 1, 2, 3)],
    );
    let races = [
        race_of(a, arrival(3, How::Unit(1)), arrival(9, How::Unit(9))),
        race_of(b, arrival(3, How::Unit(3)), arrival(9, How::Unit(9))),
    ];
    let mut memory = Memory::default();
    let roles = assign(&state, 0, &obj, &races, &mut memory);
    assert_eq!(roles.frontline, vec![(1, a), (3, b)]);
    assert_eq!(roles.back, vec![2]);
    assert_eq!(memory.objective_of.len(), 2);
}

#[test]
fn objectives_below_the_threshold_get_no_unit() {
    let at = cell(6, 6);
    let obj = [goal(at, OBJECTIVE_MIN_VALUE - 1)];
    let state = position(LAND, [0, 0], &[(0, 1, 1, 6, 3)]);
    let races = [race_of(at, arrival(3, How::Unit(1)), None)];
    let roles = assign(&state, 0, &obj, &races, &mut Memory::default());
    assert!(roles.frontline.is_empty() && roles.trainings.is_empty());
}

#[test]
fn a_training_serves_an_objective_a_unit_reaches_later() {
    let at = cell(3, 4);
    let obj = [goal(at, 9)];
    let state = position(LAND, [30, 0], &[(0, 1, 1, 0, 0)]);
    let races = race(&state, 0, &obj);
    let roles = assign(&state, 0, &obj, &races, &mut Memory::default());
    assert!(roles.frontline.is_empty());
    assert_eq!(roles.trainings, vec![(at, at)]);
    assert_eq!(roles.back, vec![1]);
}

#[test]
fn a_unit_keeps_its_objective_when_nothing_changes() {
    let at = cell(6, 6);
    let obj = [goal(at, 9)];
    let state = position(LAND, [0, 0], &[(0, 1, 1, 6, 3), (0, 2, 1, 1, 1)]);
    let races = [race_of(
        at,
        arrival(3, How::Unit(1)),
        arrival(9, How::Unit(9)),
    )];
    let mut memory = Memory::default();
    let first = assign(&state, 0, &obj, &races, &mut memory);
    assert_eq!(first.frontline, vec![(1, at)]);
    assert_eq!(assign(&state, 0, &obj, &races, &mut memory), first);
    // The race table naming the other unit changes nothing.
    let swapped = [race_of(
        at,
        arrival(9, How::Unit(2)),
        arrival(20, How::Unit(9)),
    )];
    assert_eq!(assign(&state, 0, &obj, &swapped, &mut memory), first);
}

#[test]
fn the_handover_goes_to_a_nearer_unit() {
    let at = cell(6, 6);
    let obj = [goal(at, 9)];
    let races = [race_of(
        at,
        arrival(3, How::Unit(2)),
        arrival(9, How::Unit(9)),
    )];
    let mut memory = Memory::default();
    memory.objective_of.insert(1, at);
    // Unit 1 is 7 steps away, unit 2 is 3 away.
    let state = position(LAND, [0, 0], &[(0, 1, 1, 2, 3), (0, 2, 1, 6, 3)]);
    let roles = assign(&state, 0, &obj, &races, &mut memory);
    assert_eq!(roles.frontline, vec![(2, at)]);
    assert_eq!(roles.back, vec![1]);
    assert_eq!(memory.objective_of.get(&2), Some(&at));
    assert_eq!(memory.objective_of.get(&1), None);
    // As near is no reason to switch.
    let mut memory = Memory::default();
    memory.objective_of.insert(1, at);
    let state = position(LAND, [0, 0], &[(0, 1, 1, 5, 3), (0, 2, 1, 7, 3)]);
    let roles = assign(&state, 0, &obj, &races, &mut memory);
    assert_eq!(roles.frontline, vec![(1, at)]);
}

#[test]
fn a_lost_objective_frees_its_unit() {
    let (a, b) = (cell(6, 6), cell(2, 6));
    let obj = [goal(a, 9), goal(b, 5)];
    let state = position(LAND, [0, 0], &[(0, 1, 1, 4, 3)]);
    let mut memory = Memory::default();
    let won = [
        race_of(a, arrival(5, How::Unit(1)), arrival(9, How::Unit(9))),
        race_of(b, arrival(5, How::Unit(1)), arrival(9, How::Unit(9))),
    ];
    let roles = assign(&state, 0, &obj, &won, &mut memory);
    assert_eq!(roles.frontline, vec![(1, a)]);
    // The enemy now reaches a first: the unit goes to b.
    let lost = [
        race_of(a, arrival(5, How::Unit(1)), arrival(2, How::Unit(9))),
        race_of(b, arrival(5, How::Unit(1)), arrival(9, How::Unit(9))),
    ];
    let roles = assign(&state, 0, &obj, &lost, &mut memory);
    assert_eq!(roles.frontline, vec![(1, b)]);
    // With nothing left to win, it is back line.
    let none = [race_of(
        a,
        arrival(5, How::Unit(1)),
        arrival(2, How::Unit(9)),
    )];
    let roles = assign(&state, 0, &obj[..1], &none, &mut memory);
    assert!(roles.frontline.is_empty());
    assert_eq!(roles.back, vec![1]);
    assert!(memory.objective_of.is_empty());
}

#[test]
fn a_dead_unit_leaves_the_memory() {
    let at = cell(6, 6);
    let obj = [goal(at, 9)];
    let state = position(LAND, [0, 0], &[(0, 2, 1, 1, 1)]);
    let races = [race_of(at, arrival(8, How::Unit(2)), None)];
    let mut memory = Memory::default();
    memory.objective_of.insert(1, at);
    let roles = assign(&state, 0, &obj, &races, &mut memory);
    assert_eq!(roles.frontline, vec![(2, at)]);
    assert_eq!(memory.objective_of.len(), 1);
}

#[test]
fn the_switch_is_off() {
    const { assert!(!STRATEGY) };
}
