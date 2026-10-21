use cg_core::rng::Rng;
use ice_and_fire_engine::{Bits, Command, League, Outcome, State, SIZE};
use ice_and_fire_policy::greedy_turn;
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;
use ice_and_fire_tactics::cut::cuts;

use crate::plan::margin;
use crate::{plan, Steps};

/// The steps of the four reference bots, and of bronze.
const LADDER: [Steps; 5] = [
    Steps::NONE,
    Steps {
        tower_chain: true,
        ..Steps::NONE
    },
    Steps {
        tower_chain: true,
        defence: true,
        ..Steps::NONE
    },
    Steps {
        tower_chain: true,
        defence: true,
        cuts: true,
        banking: false,
    },
    Steps::ALL,
];

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
    position_with_spots(&[], rows, gold, buildings, units)
}

/// [`position`] with mine spots on the given cells (x, y).
fn position_with_spots(
    spots: &[(u8, u8)],
    rows: [&str; 12],
    gold: [i64; 2],
    buildings: &[(u8, u8, u8, u8)],
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
    State::read(&initial, &turn, League::Bronze, 10).expect("a valid position")
}

/// The commands of the bot in `state`, checked to be accepted in order, and
/// the state after them.
fn planned(state: &State, steps: Steps) -> (Vec<Command>, State) {
    let mut commands = Vec::new();
    plan(state, steps, &mut commands);
    let mut after = state.clone();
    for &command in &commands {
        assert!(after.apply(command), "{command:?} refused in {state:?}");
    }
    (commands, after)
}

fn train(level: u8, x: u8, y: u8) -> Command {
    Command::Train { level, x, y }
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
fn no_step_plays_the_turn_of_bronze() {
    let mut rng = Rng::new(0x00b1_200e);
    let mut random = Vec::new();
    let mut bronze = Vec::new();
    let mut ours = Vec::new();
    let seeds = if cfg!(debug_assertions) { 12 } else { 60 };
    let mut compared = 0;
    for seed in 0..seeds {
        let map = generate(seed, RefereeLeague::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        while state.outcome().is_none() {
            greedy_turn(&state, &mut bronze);
            plan(&state, Steps::NONE, &mut ours);
            assert_eq!(ours, bronze, "seed {seed}, {state:?}");
            compared += 1;
            // Both seats play on from random turns, so the positions vary.
            state.random_turn(&mut rng, &mut random);
            state.play_turn(&random);
        }
    }
    assert!(compared > 100, "{compared}");
}

#[test]
fn takes_the_hq_with_a_chain_of_trainings() {
    let state = position(CORRIDOR, [25, 20], &[], &[]);
    for steps in LADDER {
        let (commands, after) = planned(&state, steps);
        assert_eq!(commands.len(), 2, "{steps:?}: {commands:?}");
        assert_eq!(after.outcome(), Some(Outcome::Win(0)));
    }
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
    for steps in LADDER {
        let (commands, after) = planned(&state, steps);
        assert_eq!(
            commands,
            vec![Command::Move {
                id: 5,
                x: 11,
                y: 11
            }]
        );
        assert_eq!(after.outcome(), Some(Outcome::Win(0)));
    }
}

/// Blue holds the corridor from its HQ to (6, 0) and has a unit on (6, 0):
/// it steps to (5, 0) and trains on (4, 0) to the HQ with its 40 gold.
const THREATENED: [&str; 12] = [
    "OOOOOXXXXXXX",
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

#[test]
fn defends_the_hq_when_the_enemy_can_take_it() {
    // The enemy's unit on (5, 0), next to our territory, and 40 gold: its
    // chain along row 0 costs 40 to the HQ.
    let state = position(THREATENED, [60, 40], &[], &[(1, 7, 1, 5, 0)]);
    assert!(margin(&state, 0) <= 0, "the position holds no threat");
    for steps in &LADDER[2..] {
        let (commands, after) = planned(&state, *steps);
        assert!(!commands.is_empty());
        assert!(
            margin(&after, 0) > 0,
            "{steps:?}: exposed after {commands:?}"
        );
    }
}

#[test]
fn does_nothing_without_gold_or_units() {
    let state = position(CORRIDOR, [0, 0], &[], &[]);
    for steps in LADDER {
        let (commands, _) = planned(&state, steps);
        assert!(commands.is_empty(), "{steps:?}: {commands:?}");
    }
}

#[test]
fn unreadable_input_is_wait() {
    assert_eq!(
        crate::answer("0\n", "garbage\n", League::Bronze, 1, Steps::ALL),
        "WAIT"
    );
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
fn banking_keeps_the_gold_when_the_chain_is_affordable_next_turn() {
    let state = position(BANKABLE, [30, 20], &[], &[]);
    let without = Steps {
        banking: false,
        ..Steps::ALL
    };
    let (spent, _) = planned(&state, without);
    assert!(!spent.is_empty(), "the position must tempt a training");
    let (kept, _) = planned(&state, Steps::ALL);
    assert!(kept.is_empty(), "{kept:?}");
}

/// Red holds the top row and the right column down to (11, 3), blue the rest
/// of the column and a row of 11 cells off it, behind (11, 5): training on
/// (11, 4) and then (11, 5) cuts the whole row off.
const CUTTABLE: [&str; 12] = [
    "OOOOOOOOOOOO",
    "###########O",
    "###########O",
    "###########O",
    "###########.",
    "XXXXXXXXXXXX",
    "###########X",
    "###########X",
    "###########X",
    "###########X",
    "###########X",
    "###########X",
];

#[test]
fn a_cut_is_played_when_one_scores_enough() {
    let state = position(CUTTABLE, [40, 20], &[], &[]);
    let found = cuts(&state, 4, 35, 8);
    assert!(
        found.first().is_some_and(|cut| cut.score >= 10),
        "no cut scores 10: {found:?}"
    );
    let cutter = Steps {
        cuts: true,
        ..Steps::NONE
    };
    let (commands, after) = planned(&state, cutter);
    assert!(commands.contains(&train(1, 11, 5)), "{commands:?}");
    // The row is out of the enemy's territory.
    let lost = state.active(1).count() - after.active(1).count();
    assert!(lost >= 11, "{lost}");
}

/// Blue's level 3 unit sits on (6, 1), a pocket joined to the corridor at
/// (6, 0) only. Killing it saves blue 20 gold of upkeep, which pays for its
/// chain to our HQ.
const BYSTANDER: [&str; 12] = [
    "OOOOOOXXXXXX",
    "OOOOOOX####X",
    "OOOOOO#####X",
    "OOOOOO#####X",
    "OOOOOO#####X",
    "OOOOOO#####X",
    "###########X",
    "###########X",
    "###########X",
    "###########X",
    "###########X",
    "###########X",
];

#[test]
fn the_last_check_undoes_spending_that_exposes_the_hq() {
    let state = position(BYSTANDER, [30, 50], &[], &[(1, 3, 3, 6, 1)]);
    assert!(margin(&state, 0) > 0, "the position starts exposed");
    let (_, exposed) = planned(&state, Steps::NONE);
    assert!(
        margin(&exposed, 0) <= 0,
        "without the check, bronze's spending must expose the HQ"
    );
    let guard = Steps {
        defence: true,
        ..Steps::NONE
    };
    let (commands, after) = planned(&state, guard);
    assert!(margin(&after, 0) > 0, "{commands:?}");
    assert!(commands.is_empty(), "{commands:?}");
}

/// Red holds (0, 0) to (2, 0) and (0, 1), (1, 1), with a mine spot on (1, 1);
/// blue's level 3 unit stands on (3, 0), three steps from red's HQ.
const GATE: [&str; 12] = [
    "OOOXXXXXXXXX",
    "OO#########X",
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

fn gate(enemy_gold: i64) -> State {
    position_with_spots(&[(1, 1)], GATE, [40, enemy_gold], &[], &[(1, 7, 3, 3, 0)])
}

fn count(commands: &[Command], what: fn(&Command) -> bool) -> usize {
    commands.iter().filter(|command| what(command)).count()
}

fn is_tower(command: &Command) -> bool {
    matches!(command, Command::BuildTower { .. })
}

fn is_mine(command: &Command) -> bool {
    matches!(command, Command::BuildMine { .. })
}

#[test]
fn a_defence_leaves_the_tower_by_the_hq_to_mines() {
    // The enemy, with 30 gold, could take the HQ: guard's defence builds a
    // tower, and the enemy unit stays within 3 steps of the HQ. Bronze's
    // tower by the HQ is not built on top of it: the gold goes to the mine.
    let state = gate(30);
    assert!(margin(&state, 0) <= 0, "the position holds no threat");
    let (commands, after) = planned(&state, LADDER[2]);
    assert_eq!(count(&commands, is_tower), 1, "{commands:?}");
    assert!(
        commands.contains(&Command::BuildMine { x: 1, y: 1 }),
        "{commands:?}"
    );
    assert!(margin(&after, 0) > 0, "{commands:?}");
}

#[test]
fn without_a_defence_the_tower_by_the_hq_is_bronzes() {
    // With no gold for a chain the enemy is no threat, so no defence is
    // played; its unit is within 3 steps all the same, and bronze's rule
    // builds the tower by the HQ instead of the mine.
    let state = gate(0);
    assert!(margin(&state, 0) > 0, "the position holds a threat");
    let (commands, _) = planned(&state, LADDER[2]);
    assert_eq!(count(&commands, is_tower), 1, "{commands:?}");
    assert_eq!(count(&commands, is_mine), 0, "{commands:?}");
    let (bronze, _) = planned(&state, Steps::NONE);
    assert_eq!(commands, bronze);
}

/// Blue's tower on (9, 0) makes (8, 0) to (10, 0) cost a level 3 unit each,
/// and a blue unit on (11, 5) a level 2 one. Taking the tower first makes
/// (10, 0) cheap, so the chain to blue's HQ costs 270 where the static
/// estimate says 290. Red has 280.
fn tower_world() -> State {
    let rows = [
        "O.......XXXX",
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
    position(rows, [280, 20], &[(1, 2, 9, 0)], &[(1, 1, 1, 11, 5)])
}

#[test]
fn the_tower_chain_takes_the_hq_behind_an_enemy_tower() {
    let state = tower_world();
    let chain = Steps {
        tower_chain: true,
        ..Steps::NONE
    };
    let (commands, after) = planned(&state, chain);
    assert_eq!(after.outcome(), Some(Outcome::Win(0)), "{commands:?}");
    // Bronze's static chain cannot pay for the HQ.
    let (commands, after) = planned(&state, Steps::NONE);
    assert_eq!(after.outcome(), None, "{commands:?}");
}

fn start(seed: u64) -> State {
    let map = generate(seed, RefereeLeague::Bronze);
    State::new(&map.void, &map.mine_spots, League::Bronze)
}

/// Plays each bot of the ladder against random turns on the referee's maps,
/// in both seats: every command it sends is accepted, in order, by the
/// engine.
#[test]
fn every_command_of_whole_games_is_accepted() {
    let games = if cfg!(debug_assertions) { 4 } else { 30 };
    let mut rng = Rng::new(0x1ce_f1fe);
    let mut random = Vec::new();
    let mut sent = 0;
    for steps in LADDER {
        for seed in 0..games {
            for bot_seat in 0..2 {
                let mut state = start(seed);
                while state.outcome().is_none() {
                    if state.to_act() == bot_seat {
                        let (commands, _) = planned(&state, steps);
                        sent += commands.len();
                        state.play_turn(&commands);
                    } else {
                        state.random_turn(&mut rng, &mut random);
                        state.play_turn(&random);
                    }
                }
            }
        }
    }
    assert!(sent > 0);
}
