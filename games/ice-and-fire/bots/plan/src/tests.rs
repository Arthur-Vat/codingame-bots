use std::time::{Duration, Instant};

use cg_arena::referee::{Answer, GameSetup, Outcome as GameOutcome, Referee};
use cg_core::rng::Rng;
use ice_and_fire_engine::{Command, League, Outcome, State, SIZE};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::{IceAndFireReferee, League as RefereeLeague};

use crate::actions::{generate as actions_of, Action};
use crate::search::{think, Limit};

/// The bot's opening, new for each call, with narrow beams: the tests'
/// limits have no deadline.
fn opening() -> ice_and_fire_opening::planned::Planned {
    crate::opening_of(2)
}

/// A limit with no deadline: only the number of plans scored ends the
/// climbs, so a search is deterministic.
fn evals(max_evals: u64) -> Limit {
    Limit {
        soft: None,
        hard: None,
        max_evals,
    }
}

/// A Bronze position as the player in seat 0 (red, to act) sees it: the map
/// row by row (`#` void, `.` neutral, `O`/`o` red's active and inactive
/// cells, `X`/`x` blue's), the gold of both and the units (owner, id, level,
/// x, y). No building but the HQs.
fn position(rows: [&str; 12], gold: [i64; 2], units: &[(u8, u32, u8, u8, u8)]) -> State {
    let mut income = [0i64; 2];
    let mut text = String::new();
    for row in rows {
        assert_eq!(row.len(), SIZE, "{row:?}");
        for symbol in row.bytes() {
            match symbol {
                b'O' => income[0] += 1,
                b'X' => income[1] += 1,
                _ => {}
            }
        }
        text.push_str(row);
        text.push('\n');
    }
    for &(owner, _, level, x, y) in units {
        let active = rows[usize::from(y)].as_bytes()[usize::from(x)];
        let seat = usize::from(owner);
        if active == b"OX"[seat] {
            income[seat] -= [1, 4, 20][usize::from(level) - 1];
        }
    }
    let mut turn = format!("{}\n{}\n{}\n{}\n", gold[0], income[0], gold[1], income[1]);
    turn.push_str(&text);
    turn.push_str("2\n0 0 0 0\n1 0 11 11\n");
    turn.push_str(&format!("{}\n", units.len()));
    for (owner, id, level, x, y) in units {
        turn.push_str(&format!("{owner} {id} {level} {x} {y}\n"));
    }
    State::read("0\n", &turn, League::Bronze, 10).expect("a valid position")
}

/// Red holds the top row and the right column down to (11, 9); blue's HQ is
/// alone at (11, 11) and (11, 10) is neutral.
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

/// A position with big armies in contact: red holds rows 0 to 5, blue rows 6
/// to 11, all active, and red has `gold`.
fn heavy(gold: i64) -> State {
    let red = "OOOOOOOOOOOO";
    let blue = "XXXXXXXXXXXX";
    position(
        [
            red, red, red, red, red, red, blue, blue, blue, blue, blue, blue,
        ],
        [gold, 20],
        &[],
    )
}

/// Positions of games on the referee's Bronze maps, played with random turns:
/// the start and then after each of `plies` turns, both seats in turn.
fn positions(seed: u64, plies: &[u32]) -> Vec<State> {
    let map = generate(seed, RefereeLeague::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let mut rng = Rng::new(seed ^ 0x2545_f491);
    let mut commands = Vec::new();
    let mut out = vec![state.clone()];
    let mut played = 0;
    for &until in plies {
        while played < until && state.outcome().is_none() {
            state.random_turn(&mut rng, &mut commands);
            state.play_turn(&commands);
            played += 1;
        }
        if state.outcome().is_some() {
            break;
        }
        out.push(state.clone());
    }
    out
}

/// The game turns of the four phases of a game: the opening, early, middle
/// and late game (a game lasts about 17 turns, 34 plies).
const PHASES: [u32; 4] = [4, 12, 28, 60];

#[test]
fn every_command_is_accepted_in_order() {
    let mut checked = 0;
    for seed in 0..14 {
        for state in positions(seed, &PHASES) {
            let thought = think(&state, evals(100), &mut opening());
            let mut scratch = state.clone();
            for &command in &thought.commands {
                assert!(scratch.apply(command), "seed {seed}: {command:?} refused");
            }
            checked += 1;
        }
    }
    assert!(checked >= 50, "{checked} positions");
}

#[test]
fn every_action_generated_is_a_command_of_the_rules() {
    // Single commands are accepted unless an earlier one of a macro action
    // changed the state; the single ones all are accepted on the state they
    // were generated for.
    for seed in 0..8 {
        for state in positions(seed, &PHASES) {
            let mut list = Vec::new();
            actions_of(&state, &mut list);
            for action in &list {
                if let Action::One(command) = action {
                    assert!(state.clone().apply(*command), "seed {seed}: {command:?}");
                }
            }
        }
    }
}

#[test]
fn plays_a_chain_of_trainings_that_takes_the_enemy_hq() {
    // 25 gold pay two level 1 trainings: (11, 10), then the HQ.
    let state = position(CORRIDOR, [25, 20], &[]);
    let thought = think(&state, evals(50), &mut opening());
    let mut scratch = state.clone();
    for &command in &thought.commands {
        assert!(scratch.apply(command));
    }
    assert_eq!(scratch.outcome(), Some(Outcome::Win(0)));
    assert_eq!(thought.climbs, 0, "lethal is taken before any search");
}

#[test]
fn a_unit_next_to_the_enemy_hq_steps_in() {
    let mut rows = CORRIDOR;
    rows[10] = "###########O";
    let state = position(rows, [0, 20], &[(0, 1, 1, 11, 10)]);
    let thought = think(&state, evals(50), &mut opening());
    assert_eq!(
        thought.commands,
        vec![Command::Move {
            id: 1,
            x: 11,
            y: 11
        }]
    );
}

#[test]
fn does_not_play_when_the_game_is_over() {
    let mut state = position(CORRIDOR, [25, 20], &[]);
    assert!(state.apply(Command::Train {
        level: 1,
        x: 11,
        y: 10
    }));
    assert!(state.apply(Command::Train {
        level: 1,
        x: 11,
        y: 11
    }));
    assert!(state.outcome().is_some());
    assert!(think(&state, evals(50), &mut opening()).commands.is_empty());
}

#[test]
fn the_search_respects_its_deadline_on_a_heavy_position() {
    let budget = Duration::from_millis(40);
    for gold in [10, 30, 50] {
        let state = heavy(gold);
        let start = Instant::now();
        let limit = Limit {
            soft: Some(start + budget.mul_f64(0.75)),
            hard: Some(start + budget),
            max_evals: u64::MAX,
        };
        let thought = think(&state, limit, &mut opening());
        let elapsed = start.elapsed();
        // Tests run unoptimised: one try of an action may take a while, so
        // the margin is generous. The arena measures the real answers.
        assert!(
            elapsed < budget + Duration::from_millis(150),
            "gold {gold}: {elapsed:?}"
        );
        assert!(thought.evals > 0);
        let mut scratch = state.clone();
        for &command in &thought.commands {
            assert!(scratch.apply(command));
        }
    }
}

#[test]
fn the_same_input_gives_the_same_answer() {
    for seed in 0..6 {
        for state in positions(seed, &PHASES) {
            let a = think(&state, evals(150), &mut opening());
            let b = think(&state, evals(150), &mut opening());
            assert_eq!(a.commands, b.commands, "seed {seed}");
            assert_eq!((a.climbs, a.evals, a.memory), (b.climbs, b.evals, b.memory));
        }
    }
}

#[test]
fn a_search_with_more_room_climbs_again_with_a_memory() {
    // On a middle-game position the first climb's plan gets replies, which
    // the next climb plays against.
    let state = &positions(3, &[30])[1];
    let thought = think(state, evals(5_000), &mut opening());
    assert!(thought.climbs >= 2, "{} climbs", thought.climbs);
    assert!(thought.memory >= 1);
    assert!(thought.memory <= 8);
    // And a tight limit still answers, with the first climb cut short.
    let short = think(state, evals(5), &mut opening());
    assert_eq!(short.climbs, 1);
    assert_eq!(short.memory, 0);
}

#[test]
fn takes_a_cheap_gain_the_hill_climb_must_find() {
    // Room to spread and gold for trainings, the enemy near enough that the
    // middle game has begun and the plan search plays, not the opening: the plan is not empty and
    // spends gold on territory.
    let state = position(
        [
            "OOOO........",
            "O...........",
            "....XXXXXXXX",
            "...........X",
            "...........X",
            "...........X",
            "...........X",
            "...........X",
            "...........X",
            "...........X",
            "...........X",
            "...........X",
        ],
        [30, 30],
        &[],
    );
    assert!(ice_and_fire_opening::middle::reached(&state));
    let thought = think(&state, evals(500), &mut opening());
    assert!(!thought.commands.is_empty());
    assert!(thought
        .commands
        .iter()
        .any(|c| matches!(c, Command::Train { .. })));
}

#[test]
fn the_limit_of_a_turn_follows_the_time_scale() {
    let scale = cg_core::time::time_scale().min(100.0);
    // 800 ms on the first turn, 40 ms on the others.
    for (turn, ms) in [(1, 0.800), (2, 0.040), (30, 0.040)] {
        let start = Instant::now();
        let limit = crate::limit(start, turn);
        let (soft, hard) = (limit.soft.unwrap(), limit.hard.unwrap());
        assert!(start < soft && soft < hard);
        let budget = hard - start;
        let expected = Duration::from_secs_f64(ms * scale);
        assert!(budget >= expected && budget < expected + Duration::from_millis(1));
    }
}

// --- The loop ---

const MAP: &str = "O...........\n............\n............\n............\n............\n\
                   ............\n............\n............\n............\n............\n\
                   ...........X\n...........X\n";

fn run(input: &str) -> Result<String, String> {
    let mut output = Vec::new();
    crate::session::play(&|_, _| evals(40), opening(), input.as_bytes(), &mut output)?;
    Ok(String::from_utf8(output).expect("answers are ASCII"))
}

/// One turn's input with the given building and unit lines.
fn turn(buildings: &[&str], units: &[&str]) -> String {
    let mut text = format!("20\n1\n20\n2\n{MAP}{}\n", buildings.len());
    for line in buildings {
        text.push_str(line);
        text.push('\n');
    }
    text.push_str(&format!("{}\n", units.len()));
    for line in units {
        text.push_str(line);
        text.push('\n');
    }
    text
}

#[test]
fn every_answer_is_accepted_by_the_referee() {
    let mut finished = 0;
    for seed in 0..6 {
        let mut referee = IceAndFireReferee::new(&GameSetup {
            seed,
            opening_plies: 0,
        });
        let initial = referee.initial_input(0);
        while let Some(&seat) = referee.players_to_act().first() {
            let input = format!("{initial}{}", referee.turn_input(seat));
            let line = run(&input).unwrap().trim().to_string();
            assert_eq!(line.lines().count(), 1, "seed {seed}: {line:?}");
            let answers = [Answer {
                seat,
                lines: vec![line.clone()],
            }];
            if let Err(err) = referee.play(&answers) {
                panic!("seed {seed}, seat {seat}: {line:?} refused: {}", err.reason);
            }
        }
        if matches!(
            referee.outcome(),
            Some(GameOutcome::Win(_) | GameOutcome::Draw)
        ) {
            finished += 1;
        }
    }
    assert_eq!(finished, 6);
}

#[test]
fn plays_every_turn_until_the_input_ends() {
    let buildings = ["0 0 0 0", "1 0 11 11"];
    let input = format!(
        "2\n1 0\n10 11\n{}{}",
        turn(&buildings, &[]),
        turn(&buildings, &["0 1 1 1 0", "1 2 1 11 10"]),
    );
    let output = run(&input).unwrap();
    assert_eq!(output.lines().count(), 2, "{output}");
}

#[test]
fn waits_on_a_turn_that_is_not_a_state() {
    // No HQ among the buildings: the engine cannot read it, so the bot waits.
    let input = format!("0\n{}", turn(&[], &[]));
    assert_eq!(run(&input).unwrap(), "WAIT\n");
    let input = format!("0\n{}", turn(&["0 0 5 5"], &[]));
    assert_eq!(run(&input).unwrap(), "WAIT\n");
}

#[test]
fn stops_cleanly_when_the_game_ends() {
    assert_eq!(run("").unwrap(), "");
    assert_eq!(run("0\n").unwrap(), "");
}

#[test]
fn accepts_windows_line_endings() {
    let input = format!("0\n{}", turn(&["0 0 0 0", "1 0 11 11"], &[])).replace('\n', "\r\n");
    let output = run(&input).unwrap();
    assert_eq!(output.lines().count(), 1, "{output}");
}

#[test]
fn rejects_bad_counts_and_input_that_stops_mid_turn() {
    let err = run("two\n").unwrap_err();
    assert!(err.contains("mine spot count"), "{err}");
    let err = run("100000\n").unwrap_err();
    assert!(err.contains("mine spot count"), "{err}");
    let input = format!("0\n20\n1\n20\n2\n{MAP}x\n");
    let err = run(&input).unwrap_err();
    assert!(err.contains("building count"), "{err}");
    let err = run("1\n").unwrap_err();
    assert!(err.contains("middle of a turn"), "{err}");
    let err = run("0\n20\n1\n").unwrap_err();
    assert!(err.contains("middle of a turn"), "{err}");
}

#[test]
fn never_panics_on_garbage_that_has_the_right_shape() {
    let junk = "a\nb\nc\nd\n".to_string() + &"?\n".repeat(12) + "0\n0\n";
    let input = format!("0\n{junk}{junk}");
    assert_eq!(run(&input).unwrap(), "WAIT\nWAIT\n");
}

#[test]
fn a_side_that_cannot_pay_its_upkeep_counts_its_units_as_lost() {
    // Red holds 3 cells and a level 3 unit: income 3 - 20 = -17, and 5 gold
    // cannot pay it, so the unit dies at red's next start of turn.
    let mut rows = [
        "OOO.........",
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
        "...........X",
    ];
    let state = position(rows, [5, 20], &[(0, 1, 3, 2, 0)]);
    assert_eq!(state.income(0), -17);
    let shared = ice_and_fire_tactics::eval::evaluate(&state, 0) + crate::eval::shape(&state, 0);
    assert_eq!(crate::eval::eval(&state, 0), shared - 2 * 30);
    // With the gold to pay it, nothing is taken.
    let state = position(rows, [17, 20], &[(0, 1, 3, 2, 0)]);
    let shared = ice_and_fire_tactics::eval::evaluate(&state, 0) + crate::eval::shape(&state, 0);
    assert_eq!(crate::eval::eval(&state, 0), shared);
    // Blue's debt counts for red.
    rows[0] = "O...........";
    let state = position(rows, [5, 0], &[(1, 1, 2, 11, 10)]);
    assert_eq!(state.income(1), 2 - 4);
    let shared = ice_and_fire_tactics::eval::evaluate(&state, 0) + crate::eval::shape(&state, 0);
    assert_eq!(crate::eval::eval(&state, 0), shared + 2 * 20);
}

#[test]
fn a_cell_that_cuts_others_off_weakens_its_side() {
    // Red's line (0,0)-(1,0)-(2,0): (1,0) is a joint that cuts 1 cell off,
    // a weakness of 2; blue's two cells have none.
    let rows = [
        "OOO.........",
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
        "...........X",
    ];
    let state = position(rows, [20, 20], &[]);
    assert_eq!(crate::eval::shape(&state, 0), -2 * 2);
    assert_eq!(crate::eval::shape(&state, 1), 2 * 2);
    // A square has no joint.
    let mut rows = rows;
    rows[0] = "OO..........";
    rows[1] = "OO..........";
    let state = position(rows, [20, 20], &[]);
    assert_eq!(crate::eval::shape(&state, 0), 0);
}
