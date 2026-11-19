use ice_and_fire_engine::{Bits, Command, League, State, SIZE};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

use crate::plan::{exposure, plan, MINE_FIRST, WALL_FIRST, WALL_MIN};

/// A Bronze position at `game_turn` as the player in seat 0 (red, to act)
/// sees it: mine spots (x, y), the map row by row (`#` void, `.` neutral,
/// `O` the player's active cells, `X` the opponent's), the gold of both and
/// the units (owner, id, level, x, y). The HQs are at (0, 0) and (11, 11).
fn position(
    game_turn: u32,
    spots: &[(u8, u8)],
    rows: [&str; 12],
    gold: [i64; 2],
    units: &[(u8, u32, u8, u8, u8)],
) -> State {
    let mut initial = format!("{}\n", spots.len());
    for (x, y) in spots {
        initial.push_str(&format!("{x} {y}\n"));
    }
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
    turn.push_str("2\n0 0 0 0\n1 0 11 11\n");
    turn.push_str(&format!("{}\n", units.len()));
    for (owner, id, level, x, y) in units {
        turn.push_str(&format!("{owner} {id} {level} {x} {y}\n"));
    }
    State::read(&initial, &turn, League::Bronze, game_turn).expect("a valid position")
}

/// The commands of the bot in `state`, checked to be accepted in order, and
/// the state after them.
fn planned(state: &State) -> (Vec<Command>, State) {
    let mut commands = Vec::new();
    plan(state, &mut commands);
    let mut after = state.clone();
    for &command in &commands {
        assert!(after.apply(command), "{command:?} refused in {state:?}");
    }
    (commands, after)
}

fn towers(commands: &[Command]) -> Vec<(u8, u8)> {
    commands
        .iter()
        .filter_map(|command| match *command {
            Command::BuildTower { x, y } => Some((x, y)),
            _ => None,
        })
        .collect()
}

fn mines(commands: &[Command]) -> Vec<(u8, u8)> {
    commands
        .iter()
        .filter_map(|command| match *command {
            Command::BuildMine { x, y } => Some((x, y)),
            _ => None,
        })
        .collect()
}

/// Red holds column 0 down to row 2 and rows 3 and 4; blue a loop of row 6,
/// columns 0 and 11 below it and row 11; row 5 is neutral, the rest void.
/// With 20 gold, blue reaches red's front at its next turn. No cut of
/// blue's land costs less than 40, and neither side can pay for its chain to
/// the other's HQ by its next turn with the gold of the tests (red's costs
/// 70 against an income of 27, blue's 60 against 32).
const FACING: [&str; 12] = [
    "O###########",
    "O###########",
    "O###########",
    "OOOOOOOOOOOO",
    "OOOOOOOOOOOO",
    "............",
    "XXXXXXXXXXXX",
    "X##########X",
    "X##########X",
    "X##########X",
    "X##########X",
    "XXXXXXXXXXXX",
];

#[test]
fn the_wall_guards_the_front_from_its_first_turn() {
    let state = position(WALL_FIRST, &[], FACING, [30, 20], &[]);
    let (commands, after) = planned(&state);
    let built = towers(&commands);
    assert!(!built.is_empty(), "{commands:?}");
    for &(_, y) in &built {
        assert!(y == 3 || y == 4, "a tower off the front: {commands:?}");
    }
    assert!(
        exposure(&after, 0) + built.len() as u32 * WALL_MIN <= exposure(&state, 0),
        "{commands:?}"
    );
    // Before its first turn, no wall.
    let early = position(WALL_FIRST - 1, &[], FACING, [30, 20], &[]);
    let (commands, _) = planned(&early);
    assert!(towers(&commands).is_empty(), "{commands:?}");
}

/// [`FACING`] with blue holding only row 6 and the right column below it:
/// with no gold, blue reaches none of red's cells at its next turn, so no
/// tower is worth building, and red's 15 gold left after the reserve pays
/// for no cut.
const APART: [&str; 12] = [
    "O###########",
    "O###########",
    "O###########",
    "OOOOOOOOOOOO",
    "OOOOOOOOOOOO",
    "............",
    "XXXXXXXXXXXX",
    "###########X",
    "###########X",
    "###########X",
    "###########X",
    "###########X",
];

#[test]
fn a_mine_goes_on_the_safest_spot_and_never_near_the_enemy() {
    // (0, 1) is 5 steps from blue's land, (5, 4) 2 steps.
    let state = position(MINE_FIRST, &[(0, 1), (5, 4)], APART, [20, 0], &[]);
    let (commands, _) = planned(&state);
    assert_eq!(mines(&commands), vec![(0, 1)], "{commands:?}");
    let exposed = position(MINE_FIRST, &[(5, 4)], APART, [20, 0], &[]);
    let (commands, _) = planned(&exposed);
    assert!(mines(&commands).is_empty(), "{commands:?}");
    // Before its first turn, no mine.
    let early = position(MINE_FIRST - 1, &[(0, 1)], APART, [20, 0], &[]);
    let (commands, _) = planned(&early);
    assert!(mines(&commands).is_empty(), "{commands:?}");
}

/// Red holds the top row and the right column down to (11, 7); three neutral
/// cells and blue's HQ are left. The chain costs 40, red has 30 and an income
/// of 19: it cannot win now but can next turn.
const BANKABLE: [&str; 12] = [
    "OOOOOOOOOOOO",
    "###########O",
    "###########O",
    "###########O",
    "###########O",
    "###########O",
    "###########O",
    "###########O",
    "###########.",
    "###########.",
    "###########.",
    "###########X",
];

#[test]
fn the_strike_keeps_the_gold_when_it_is_affordable_next_turn() {
    let state = position(12, &[], BANKABLE, [30, 20], &[]);
    let (kept, after) = planned(&state);
    assert!(kept.is_empty(), "{kept:?}");
    assert_eq!(after.gold(0), 30);
    // With the gold for it, the chain takes the HQ now.
    let state = position(12, &[], BANKABLE, [40, 20], &[]);
    let (_, after) = planned(&state);
    assert!(after.outcome().is_some());
}

#[test]
fn trainings_are_of_level_one() {
    let state = position(WALL_FIRST - 1, &[], FACING, [35, 0], &[]);
    let (commands, _) = planned(&state);
    let trained: Vec<u8> = commands
        .iter()
        .filter_map(|command| match *command {
            Command::Train { level, .. } => Some(level),
            _ => None,
        })
        .collect();
    assert!(!trained.is_empty(), "{commands:?}");
    assert!(trained.iter().all(|&level| level == 1), "{commands:?}");
}

/// Plays the bot against itself on the referee's maps: every command it
/// sends is accepted, in order, and it builds towers and mines.
#[test]
fn every_command_of_whole_games_is_accepted() {
    let games = if cfg!(debug_assertions) { 3 } else { 20 };
    let mut built = [0usize; 2];
    for seed in 0..games {
        let map = generate(seed, RefereeLeague::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        while state.outcome().is_none() {
            let (commands, _) = planned(&state);
            built[0] += towers(&commands).len();
            built[1] += mines(&commands).len();
            state.play_turn(&commands);
        }
    }
    assert!(built[0] > 0 && built[1] > 0, "{built:?}");
}

#[test]
fn unreadable_input_is_wait() {
    assert_eq!(crate::answer("0\n", "not a turn\n", 1), "WAIT");
}
