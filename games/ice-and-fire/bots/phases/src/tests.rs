use cg_core::rng::Rng;
use ice_and_fire_engine::{Bits, Command, League, State, SIZE};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

use crate::plan::{margin, plan};

/// A Bronze position as the player in seat 0 (red, to act) sees it: the map
/// row by row (`#` void, `.` neutral, `O`/`o` the player's active and
/// inactive cells, `X`/`x` the opponent's), the gold of both, the buildings
/// other than the HQs (owner, type 1 mine or 2 tower, x, y) and the units
/// (owner, id, level, x, y).
fn position(
    rows: [&str; 12],
    gold: [i64; 2],
    buildings: &[(u8, u8, u8, u8)],
    units: &[(u8, u32, u8, u8, u8)],
) -> State {
    let mut active = [Bits::EMPTY; 2];
    for (y, row) in rows.iter().enumerate() {
        for (x, symbol) in row.bytes().enumerate() {
            match symbol {
                b'O' => active[0].set(y * SIZE + x),
                b'X' => active[1].set(y * SIZE + x),
                _ => {}
            }
        }
    }
    let mut income = [0i64; 2];
    for (owner, total) in income.iter_mut().enumerate() {
        *total = i64::from(active[owner].count());
        for &(who, kind, x, y) in buildings {
            let cell = usize::from(y) * SIZE + usize::from(x);
            if usize::from(who) == owner && kind == 1 && active[owner].test(cell) {
                *total += 4;
            }
        }
        for &(who, _, level, x, y) in units {
            let cell = usize::from(y) * SIZE + usize::from(x);
            if usize::from(who) == owner && active[owner].test(cell) {
                *total -= [1, 4, 20][usize::from(level) - 1];
            }
        }
    }
    let mut turn = format!("{}\n{}\n{}\n{}\n", gold[0], income[0], gold[1], income[1]);
    for row in rows {
        turn.push_str(row);
        turn.push('\n');
    }
    let mut all = vec![(0, 0, 0, 0), (1, 0, 11, 11)];
    all.extend_from_slice(buildings);
    turn.push_str(&format!("{}\n", all.len()));
    for (owner, kind, x, y) in all {
        turn.push_str(&format!("{owner} {kind} {x} {y}\n"));
    }
    turn.push_str(&format!("{}\n", units.len()));
    for (owner, id, level, x, y) in units {
        turn.push_str(&format!("{owner} {id} {level} {x} {y}\n"));
    }
    State::read("0\n", &turn, League::Bronze, 10).expect("a valid position")
}

/// The commands of the bot in `state`, checked to be accepted in order.
fn planned(state: &State) -> (Vec<Command>, State) {
    let mut commands = Vec::new();
    plan(state, &mut commands);
    let mut after = state.clone();
    for &command in &commands {
        assert!(after.apply(command), "{command:?} refused in {state:?}");
    }
    (commands, after)
}

/// A corridor along the top row and the right column.
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

#[test]
fn takes_the_hq_with_a_chain_of_trainings() {
    let state = position(CORRIDOR, [25, 20], &[], &[]);
    let (commands, after) = planned(&state);
    assert_eq!(commands.len(), 2, "{commands:?}");
    assert_eq!(after.outcome(), Some(ice_and_fire_engine::Outcome::Win(0)));
}

#[test]
fn steps_into_the_hq_with_a_unit() {
    let rows = [
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
        "###########O",
        "###########X",
    ];
    // No gold: only the unit at (11, 10) can win.
    let state = position(rows, [0, 20], &[], &[(0, 5, 1, 11, 10)]);
    let (commands, after) = planned(&state);
    assert_eq!(
        commands,
        vec![Command::Move {
            id: 5,
            x: 11,
            y: 11
        }]
    );
    assert_eq!(after.outcome(), Some(ice_and_fire_engine::Outcome::Win(0)));
}

#[test]
fn defends_the_hq_when_the_enemy_can_take_it() {
    // Blue holds the corridor from its HQ to (3, 0) with a unit on (3, 0): it
    // steps to (2, 0) and trains on (1, 0) and the HQ with its 20 gold.
    let rows = [
        "OOXXXXXXXXXX",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
        "###########X",
    ];
    let state = position(rows, [40, 20], &[], &[(1, 7, 1, 3, 0)]);
    assert!(margin(&state, 0) <= 0, "the position holds no threat");
    let (commands, after) = planned(&state);
    assert!(!commands.is_empty());
    assert!(margin(&after, 0) > 0, "still exposed after {commands:?}");
}

#[test]
fn does_nothing_without_gold_or_units() {
    let state = position(CORRIDOR, [0, 0], &[], &[]);
    let (commands, _) = planned(&state);
    assert!(commands.is_empty(), "{commands:?}");
}

#[test]
fn unreadable_input_is_wait() {
    assert_eq!(crate::answer("0\n", "garbage\n", League::Bronze, 1), "WAIT");
}

fn start(seed: u64) -> State {
    let map = generate(seed, RefereeLeague::Bronze);
    State::new(&map.void, &map.mine_spots, League::Bronze)
}

/// Plays the bot against random turns on the referee's maps, in both seats:
/// every command it sends is accepted, in order, by the engine.
#[test]
fn every_command_of_whole_games_is_accepted() {
    let games = if cfg!(debug_assertions) { 6 } else { 40 };
    let mut rng = Rng::new(0x1ce_f1fe);
    let mut random = Vec::new();
    let mut sent = 0;
    for seed in 0..games {
        for bot_seat in 0..2 {
            let mut state = start(seed);
            while state.outcome().is_none() {
                if state.to_act() == bot_seat {
                    let (commands, _) = planned(&state);
                    sent += commands.len();
                    state.play_turn(&commands);
                } else {
                    state.random_turn(&mut rng, &mut random);
                    state.play_turn(&random);
                }
            }
        }
    }
    assert!(sent > 0);
}
