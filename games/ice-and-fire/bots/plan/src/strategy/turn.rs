//! The strategic turn (Step 4 of the frontline plan, E077): one plan made of
//! the frontline units' steps toward their objectives, then trainings at
//! their tips within the gold, then (with [`super::TRAININGS`] at 2) the
//! trainings the roles give the objectives no unit covers. The back-line
//! units are left to the search, which goes on climbing from the plan.
//!
//! Every command is checked on a scratch copy, in order, so the plan applies
//! whole on the state it was made for.

use ice_and_fire_engine::{Bits, Command, State};

use crate::actions::xy;
use crate::eval::threatened;
use ice_and_fire_opening::frontline::roles::Roles;

/// The gold of a level-1 unit.
const TRAIN_COST: i64 = 10;

/// The cell of the unit `id` of `seat`, if it lives.
fn cell_of(state: &State, seat: usize, id: u32) -> Option<usize> {
    state
        .units(seat)
        .iter()
        .find(|&c| state.unit_id(c) == Some(id))
}

/// The cells next to `from` one step nearer to `goal`, the cells not ours
/// first (a step that takes land), then by cell.
fn steps_toward(state: &State, seat: usize, from: usize, goal: usize) -> Vec<usize> {
    let map = state.map();
    let here = map.distance(from, goal);
    if here == u8::MAX || here == 0 {
        return Vec::new();
    }
    let mut out: Vec<usize> = (Bits::cell(from).neighbours() & map.valid)
        .iter()
        .filter(|&n| map.distance(n, goal) < here)
        .collect();
    out.sort_by_key(|&n| (state.owned(seat).test(n), n));
    out
}

/// Applies `command` on `scratch` and adds it to `out` if the rules accept
/// it; whether they did.
fn accepted(scratch: &mut State, out: &mut Vec<Command>, command: Command) -> bool {
    let ok = scratch.apply(command);
    if ok {
        out.push(command);
    }
    ok
}

/// How the strategic turn keeps the line behind its frontline units (E077,
/// round 2, after the diagnosis that a third of its steps left a cell the
/// enemy then entered).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rules {
    /// Also the trainings the roles give the objectives no unit covers.
    pub role_trainings: bool,
    /// A step off a cell the enemy can reach next turn needs that cell
    /// guarded at once: 0, no (round 1); 1, by a tower; 2, by a level-1
    /// training. A step whose guard the rules or the gold refuse is not
    /// played.
    pub guard: u8,
    /// The tip training of a unit that left a cell the enemy can reach goes
    /// on that cell instead, when it is still empty.
    pub fill_behind: bool,
    /// A step is not played when it raises the cells of ours the enemy can
    /// take at its next turn (`eval::threatened`).
    pub safe_step: bool,
}

/// The pieces of the strategic turn (E077, [`super::TURN`]'s 4): for each
/// frontline unit, its step toward its objective and the training at its
/// tip that the gold pays, each made alone from `state`, the units in role
/// order; pieces the rules refuse are left out.
pub fn pieces(state: &State, roles: &Roles, rules: Rules) -> Vec<Vec<Command>> {
    let mut out = Vec::new();
    for &(id, goal) in &roles.frontline {
        let one = Roles {
            frontline: vec![(id, goal)],
            ..Roles::default()
        };
        let piece = strategic_turn(
            state,
            &one,
            Rules {
                role_trainings: false,
                ..rules
            },
        );
        if !piece.is_empty() {
            out.push(piece);
        }
    }
    out
}

/// The step of the unit `id` from `from` to `to` on `scratch`, with the
/// guard `rules` ask for on `from` when the enemy reaches it (`exposed`):
/// the commands, or `None` when the step is not played.
fn step(
    scratch: &State,
    id: u32,
    from: usize,
    to: usize,
    exposed: bool,
    rules: Rules,
) -> Option<Vec<Command>> {
    let seat = scratch.to_act();
    let mut trial = scratch.clone();
    let mut commands = Vec::new();
    let (x, y) = xy(to);
    if !accepted(&mut trial, &mut commands, Command::Move { id, x, y }) {
        return None;
    }
    if exposed && rules.guard != 0 {
        let (fx, fy) = xy(from);
        let guard = if rules.guard == 1 {
            Command::BuildTower { x: fx, y: fy }
        } else {
            Command::Train {
                level: 1,
                x: fx,
                y: fy,
            }
        };
        if !accepted(&mut trial, &mut commands, guard) {
            return None;
        }
    }
    if rules.safe_step && threatened(&trial, seat) > threatened(scratch, seat) {
        return None;
    }
    Some(commands)
}

/// The commands of the strategic turn of the player to act in `state`, with
/// `roles` its roles this turn, by `rules`.
pub fn strategic_turn(state: &State, roles: &Roles, rules: Rules) -> Vec<Command> {
    let seat = state.to_act();
    let map = state.map();
    let mut scratch = state.clone();
    let mut out = Vec::new();
    // The cells the enemy can take at its next turn, as the turn starts.
    let reach = crate::eval::reach(state, 1 - seat);
    // The exposed cells our frontline units left unguarded, by unit.
    let mut left: Vec<(u32, usize)> = Vec::new();

    // 1. The frontline units step toward their objectives.
    for &(id, goal) in &roles.frontline {
        let Some(from) = cell_of(&scratch, seat, id) else {
            continue;
        };
        if !scratch.movable_units().test(from) {
            continue;
        }
        let exposed = reach.test(from);
        for to in steps_toward(&scratch, seat, from, goal) {
            if let Some(commands) = step(&scratch, id, from, to, exposed, rules) {
                for command in &commands {
                    scratch.apply(*command);
                }
                if exposed && rules.guard == 0 {
                    left.push((id, from));
                }
                out.extend(commands);
                break;
            }
        }
    }

    // 2. A training at the tip of each frontline unit: on a cell next to it
    // nearer its objective, the nearest first; with `fill_behind`, on the
    // exposed cell it left instead.
    for &(id, goal) in &roles.frontline {
        if scratch.gold(seat) < TRAIN_COST {
            break;
        }
        let Some(at) = cell_of(&scratch, seat, id) else {
            continue;
        };
        let behind = left.iter().find(|&&(u, _)| u == id).map(|&(_, c)| c);
        let cells = match behind {
            Some(cell) if rules.fill_behind => vec![cell],
            _ => steps_toward(&scratch, seat, at, goal),
        };
        for to in cells {
            let (x, y) = xy(to);
            if accepted(&mut scratch, &mut out, Command::Train { level: 1, x, y }) {
                break;
            }
        }
    }

    // 3. The objectives no unit covers: a training at our trainable cell
    // nearest each, after the moves.
    if rules.role_trainings {
        for &(_, goal) in &roles.trainings {
            if scratch.gold(seat) < TRAIN_COST {
                break;
            }
            let trainable = scratch.active(seat).neighbours() & map.valid;
            let mut cells: Vec<usize> = trainable
                .iter()
                .filter(|&c| map.distance(c, goal) != u8::MAX)
                .collect();
            cells.sort_by_key(|&c| (map.distance(c, goal), c));
            for to in cells {
                let (x, y) = xy(to);
                if accepted(&mut scratch, &mut out, Command::Train { level: 1, x, y }) {
                    break;
                }
            }
        }
    }
    out
}
