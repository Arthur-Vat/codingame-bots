use ice_and_fire_engine::State;

use ice_and_fire_engine::Bits;
use ice_and_fire_opening::econ::distances;

use super::objectives::{
    counted, objectives, objectives_valued, Kind, Objective, OBJECTIVE_MIN_VALUE,
};
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

/// The valid cells strictly nearer to `mine` than to `theirs`.
fn count(state: &State, mine: Bits, theirs: Bits) -> i64 {
    let (dm, dt) = (distances(state, mine), distances(state, theirs));
    state
        .map()
        .valid
        .iter()
        .filter(|&c| dm[c] != u8::MAX && dm[c] < dt[c])
        .count() as i64
}

/// The part of `land` joined to red's HQ.
fn joined(land: Bits) -> Bits {
    let mut reached = Bits::cell(0) & land;
    loop {
        let next = reached | (reached.neighbours() & land);
        if next == reached {
            return reached;
        }
        reached = next;
    }
}

#[test]
fn a_held_corridor_choke_is_the_land_its_loss_would_cost() {
    let state = position(CORRIDOR, [0, 0], &[]);
    let all = objectives(&state, 0);
    // The five chokes are red's; losing one cuts the corridor behind it
    // off the HQ, and red keeps the cells nearer it than blue's HQ: 21 of
    // them now, 15, 16, 16, 17, 17 after. Ranked by value, then cell.
    let got: Vec<(usize, i64)> = all.iter().map(|o| (o.cell, o.value)).collect();
    assert_eq!(got, vec![(9, 6), (10, 5), (11, 5), (23, 4), (35, 4)]);
    assert!(all.iter().all(|o| o.kind == Kind::Choke && o.held));
    // Blue gains nothing by a cell red holds (a level cell is nobody's).
    assert!(objectives(&state, 1)
        .iter()
        .all(|o| !o.held && o.value == 0));
}

#[test]
fn the_swing_is_the_land_held_by_us_less_held_by_the_enemy() {
    let state = position(CORRIDOR, [0, 0], &[]);
    let red = objectives_valued(&state, 0, true);
    assert!(red.iter().all(|o| o.value == 20));
    assert!(objectives_valued(&state, 1, true)
        .iter()
        .all(|o| o.value == -20));
}

#[test]
fn the_land_gained_is_the_difference_of_two_counts() {
    let state = position(LAND, [0, 0], &[]);
    let (mine, theirs) = (state.active(0), state.active(1));
    let all = objectives(&state, 0);
    assert!(all.iter().any(|o| !o.held));
    for o in &all {
        let c = Bits::cell(o.cell);
        let expected = if o.held {
            let without = joined(mine.and_not(c));
            count(&state, mine, theirs) - count(&state, without, theirs)
        } else {
            count(&state, mine | c, theirs) - count(&state, mine, theirs)
        };
        assert_eq!(o.value, expected, "cell {}", o.cell);
        assert!(o.value >= 0);
    }
}

#[test]
fn objectives_are_ranked_and_counted_without_a_threshold() {
    let state = position(LAND, [0, 0], &[]);
    let all = objectives(&state, 0);
    assert!(all.windows(2).all(|w| {
        (-w[0].value, w[0].kind != Kind::Choke, w[0].cell)
            < (-w[1].value, w[1].kind != Kind::Choke, w[1].cell)
    }));
    let list = [goal(1, 5), goal(2, 2), goal(3, 0)];
    let kept = counted(&list);
    if OBJECTIVE_MIN_VALUE.is_none() {
        assert_eq!(kept.len(), 2);
    }
    assert!(kept.iter().all(|o| o.value > 0));
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
    // The enemy's income is paid at the start of its turn: gold 4 and
    // income 1 need 5 more turns (not 6), 9 and 1 none.
    let theirs = |gold: i64| {
        let state = position(OPEN, [4, gold], &[]);
        race(&state, 0, &obj)[0].theirs.map(|a| a.turns)
    };
    let d = u32::from(state.map().distance(cell(10, 11), far));
    assert_eq!(theirs(4), Some(5 + 1 + d));
    assert_eq!(theirs(9), Some(1 + d));
    assert_eq!(theirs(30), Some(1 + d));
}

#[test]
fn without_income_or_gold_a_side_cannot_train() {
    let far = cell(8, 8);
    let obj = [goal(far, 10)];
    // Red's only unit on the HQ costs the upkeep: income 0, gold 4.
    let state = position(OPEN, [4, 4], &[(0, 1, 1, 0, 0)]);
    assert_eq!(state.income(0), 0);
    let r = race(&state, 0, &obj)[0];
    assert_eq!(r.ours_training, None);
    assert_eq!(r.ours.map(|a| a.how), Some(How::Unit(1)));
    // The same for the enemy, whose gold is short even with its income.
    let state = position(OPEN, [4, 4], &[(1, 2, 1, 11, 11)]);
    assert_eq!(state.income(1), 0);
    let r = race(&state, 0, &obj)[0];
    assert_eq!(r.theirs.map(|a| a.how), Some(How::Unit(2)));
}

#[test]
fn on_a_point_symmetric_map_the_seats_mirror() {
    let state = position(OPEN, [0, 0], &[]);
    let red = objectives(&state, 0);
    let blue = objectives(&state, 1);
    assert!(!red.is_empty());
    assert_eq!(red.len(), blue.len());
    for o in &red {
        let mirror = blue
            .iter()
            .find(|b| b.cell == 143 - o.cell)
            .expect("the mirrored cell is an objective");
        assert_eq!(
            (mirror.value, mirror.kind, mirror.held),
            (o.value, o.kind, o.held)
        );
    }
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
fn an_objective_gaining_nothing_gets_no_unit() {
    let at = cell(6, 6);
    let obj = [goal(at, 0)];
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

#[test]
fn a_training_cell_serves_one_objective_and_the_gold_pays_each() {
    let (a, b) = (cell(3, 8), cell(4, 8));
    let obj = [goal(a, 9), goal(b, 8)];
    let slow = arrival(9, How::Train(cell(3, 4)));
    let races = [race_of(a, slow, None), race_of(b, slow, None)];
    // The same cell for both: the best ranked has it.
    let rich = position(LAND, [30, 0], &[]);
    let roles = assign(&rich, 0, &obj, &races, &mut Memory::default());
    assert_eq!(roles.trainings, vec![(cell(3, 4), a)]);
    // Two cells, gold for one: the better objective is paid.
    let races = [
        race_of(a, arrival(9, How::Train(cell(3, 4))), None),
        race_of(b, arrival(9, How::Train(cell(4, 4))), None),
    ];
    let poor = position(LAND, [19, 0], &[]);
    let roles = assign(&poor, 0, &obj, &races, &mut Memory::default());
    assert_eq!(roles.trainings, vec![(cell(3, 4), a)]);
    // Gold for both.
    let rich = position(LAND, [20, 0], &[]);
    let roles = assign(&rich, 0, &obj, &races, &mut Memory::default());
    assert_eq!(roles.trainings, vec![(cell(3, 4), a), (cell(4, 4), b)]);
}
