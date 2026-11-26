use std::time::{Duration, Instant};

use cg_arena::referee::{Answer, GameSetup, Outcome as GameOutcome, Referee};
use cg_core::rng::Rng;
use ice_and_fire_engine::{Bits, Command, League, Outcome, State, SIZE};
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
pub(crate) fn position(rows: [&str; 12], gold: [i64; 2], units: &[(u8, u32, u8, u8, u8)]) -> State {
    position_with_spots("0\n", rows, gold, units)
}

/// [`position`] on a map whose mine spots are `spots`, the initial input's
/// text: their count, then a line `x y` for each.
fn position_with_spots(
    spots: &str,
    rows: [&str; 12],
    gold: [i64; 2],
    units: &[(u8, u32, u8, u8, u8)],
) -> State {
    position_at(10, spots, rows, gold, units)
}

/// [`position_with_spots`] read at game turn `turn`.
fn position_at(
    turn_number: u32,
    spots: &str,
    rows: [&str; 12],
    gold: [i64; 2],
    units: &[(u8, u32, u8, u8, u8)],
) -> State {
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
    State::read(spots, &turn, League::Bronze, turn_number).expect("a valid position")
}

/// The shared evaluation as the bot reads it: its margins count the moves
/// of the units of at least `MARGIN_MOVER_LEVEL` (E043), and the standing
/// threat on each HQ (E042) is added.
fn bot_shared(state: &State, seat: usize) -> i64 {
    use crate::eval::{standing, MARGIN_MOVER_LEVEL};
    use ice_and_fire_engine::Bits;
    let movers = |side: usize| {
        let mut bits = Bits::EMPTY;
        for cell in (state.units(side) & state.active(side)).iter() {
            if state
                .unit_level(cell)
                .is_some_and(|level| level >= MARGIN_MOVER_LEVEL)
            {
                bits |= Bits::cell(cell);
            }
        }
        bits
    };
    let (score, margins) = ice_and_fire_tactics::eval::evaluate_with_movers(
        state,
        seat,
        [movers(seat), movers(1 - seat)],
    );
    score + standing(margins[0]) - standing(margins[1])
}

/// The standing threat term of `seat`'s score (E042): its own less the
/// enemy's.
fn standing_term(state: &State, seat: usize) -> i64 {
    use crate::eval::standing;
    use ice_and_fire_tactics::eval::margin;
    standing(margin(state, seat)) - standing(margin(state, 1 - seat))
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
    // E027: that plan, found as the tries ran out, gets its own replies
    // searched before it is played, without changing the memory.
    assert_eq!(short.checks, 1);
}

#[test]
fn a_search_that_learns_nothing_new_climbs_from_other_first_actions() {
    // E014: once the memory stops growing, each climb is barred from the
    // first actions of the plans found before, so with room enough the
    // search goes on to its most climbs (it stopped before E014), and the
    // later climbs find other plans. The position is seed 2's (seed 3's
    // until E023, whose evaluation ends a climb there on an empty plan).
    let state = &positions(2, &[30])[1];
    let thought = think(state, evals(1_000_000), &mut opening());
    assert_eq!(thought.climbs, 32);
    assert!(thought.plans > 8, "{} plans", thought.plans);
    assert!(thought.memory <= 8);
    let mut scratch = state.clone();
    for &command in &thought.commands {
        assert!(scratch.apply(command), "{command:?}");
    }
}

#[test]
fn takes_a_cheap_gain_the_hill_climb_must_find() {
    // Room to spread and gold for trainings, the enemy near enough that the
    // middle game has begun and the plan search plays, not the opening: the plan is not empty and
    // spends gold on territory. Red's cells close a loop when it grows, so
    // a new cell adds no joint to its shape and is a strict gain with or
    // without a threat (a thin line's new cell scores 0: +4 for the cell,
    // -4 for its joint).
    let state = position(
        [
            "OOO.........",
            "OOOO........",
            "............",
            "...XXXXXXXXX",
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
    // 800 ms on the first turn, 45 ms on the others (40 before E055).
    for (turn, ms) in [(1, 0.800), (2, 0.045), (30, 0.045)] {
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
fn a_level_two_unit_counts_less_than_its_cost() {
    // E020: red's level-2 unit counts LEVEL2_DISCOUNT less than the shared
    // evaluation's 20; a level-1 unit counts as it does there. Red can pay
    // its upkeep, so no debt is counted.
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
    for (level, discount) in [(1, 0), (2, crate::eval::LEVEL2_DISCOUNT)] {
        let state = position(rows, [30, 20], &[(0, 1, level, 2, 0)]);
        let shared = bot_shared(&state, 0) + crate::eval::shape(&state, 0);
        assert_eq!(crate::eval::eval(&state, 0), shared - discount);
    }
}

#[test]
fn a_level_three_unit_is_charged_one_turn_of_upkeep() {
    use crate::eval::{strong_units, LEVEL2_DISCOUNT, UPKEEP_TURNS};
    // E036: the shared evaluation charges three turns of upkeep through its
    // weight of income; a level-3 unit is charged one (its upkeep of 20
    // given back twice), a level-2 unit three, less E020's discount.
    assert_eq!(UPKEEP_TURNS, [3, 1]);
    let rows = [
        "OOOO........",
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
    let units = |level: u8| position(rows, [100, 20], &[(0, 1, level, 2, 0)]);
    assert_eq!(strong_units(&units(1), 0), 0);
    assert_eq!(strong_units(&units(2), 0), -LEVEL2_DISCOUNT);
    assert_eq!(strong_units(&units(3), 0), 2 * 20);
    // Each side's term counts for it and against the other.
    let state = units(3);
    let shared = ice_and_fire_tactics::eval::evaluate(&state, 0) + crate::eval::shape(&state, 0);
    assert_eq!(crate::eval::eval(&state, 0) - shared, 2 * 20);
    assert_eq!(crate::eval::eval(&state, 1) + shared, -2 * 20);
    // Only units on active cells pay upkeep (RULES.md, "Income"), so a
    // level-3 unit on a cell cut off from the HQ gets nothing back.
    let mut cut = rows;
    cut[0] = "OOOO.o......";
    let state = position(cut, [100, 20], &[(0, 1, 3, 5, 0)]);
    assert!(!state.active(0).test(5));
    assert_eq!(strong_units(&state, 0), 0);
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
    // The level-3 unit's upkeep beyond its turns (E036) is given back
    // whether or not it can be paid.
    let back = 2 * 20;
    let state = position(rows, [5, 20], &[(0, 1, 3, 2, 0)]);
    assert_eq!(state.income(0), -17);
    let shared = bot_shared(&state, 0) + crate::eval::shape(&state, 0);
    assert_eq!(crate::eval::eval(&state, 0), shared - 2 * 30 + back);
    // With the gold to pay it, nothing is taken.
    let state = position(rows, [17, 20], &[(0, 1, 3, 2, 0)]);
    let shared = bot_shared(&state, 0) + crate::eval::shape(&state, 0);
    assert_eq!(crate::eval::eval(&state, 0), shared + back);
    // Blue's debt counts for red; so does the discount of its level-2 unit
    // (E020).
    rows[0] = "O...........";
    let state = position(rows, [5, 0], &[(1, 1, 2, 11, 10)]);
    assert_eq!(state.income(1), 2 - 4);
    let shared = bot_shared(&state, 0) + crate::eval::shape(&state, 0);
    assert_eq!(
        crate::eval::eval(&state, 0),
        shared + 2 * 20 + crate::eval::LEVEL2_DISCOUNT
    );
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
    if crate::eval::SHAPE_REACH == 0 {
        assert_eq!(crate::eval::shape(&state, 0), -2 * 2);
        assert_eq!(crate::eval::shape(&state, 1), 2 * 2);
    } else {
        // E081: (1,0) is far beyond SHAPE_NEAR steps of blue's land, and
        // blue's two cells have no joint.
        assert_eq!(crate::eval::shape(&state, 0), 0);
        assert_eq!(crate::eval::shape(&state, 1), 0);
    }
    // A square has no joint.
    let mut rows = rows;
    rows[0] = "OO..........";
    rows[1] = "OO..........";
    let state = position(rows, [20, 20], &[]);
    assert_eq!(crate::eval::shape(&state, 0), 0);
}

#[test]
fn cells_the_enemy_can_take_next_turn_count_less() {
    use crate::eval::{eval, threatened};
    // E023: blue's chains along the corridor cost 10 for (11, 10), then 10
    // a cell of red's from (11, 9) up. With gold 20 and income 1 blue can
    // pay 20: it threatens (11, 9).
    let state = position(CORRIDOR, [0, 20], &[]);
    assert_eq!(threatened(&state, 0), 1);
    // With 10 gold more, (11, 8) too.
    let state = position(CORRIDOR, [0, 30], &[]);
    assert_eq!(threatened(&state, 0), 2);
    // Red (income 21) threatens blue's one cell, its HQ, for 20.
    assert_eq!(threatened(&state, 1), 1);
    // Each threatened cell costs its side THREAT_QUARTERS / 4 (E023: 7/4,
    // 0.25 of a cell's value of 7; E037: 5/4). Red's standing threat on
    // blue's HQ counts too (E042), and the opening's goal (E073).
    let shared = ice_and_fire_tactics::eval::evaluate(&state, 0)
        + crate::eval::shape(&state, 0)
        + standing_term(&state, 0)
        + crate::eval::goal(&state, 0);
    assert_eq!(
        eval(&state, 0),
        shared + crate::eval::threat_quarters(&state) * (1 - 2) / 4
    );
    // Far from each other, nothing is threatened.
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
    let state = position(rows, [30, 20], &[]);
    assert_eq!((threatened(&state, 0), threatened(&state, 1)), (0, 0));
}

#[test]
fn an_enemy_unit_of_level_two_threatens_the_cells_it_can_step_onto() {
    use crate::eval::{threatened, MOVER_LEVEL};
    // E037: blue holds the bottom row from (6, 11) to its HQ, with a unit
    // on (6, 11) next to red's (5, 11) and no gold: its budget, 0 plus an
    // income of 5 or 2, pays for no training, but its unit can step.
    let rows = [
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOOXXXXXX",
    ];
    assert_eq!(MOVER_LEVEL, 2);
    let threats = |level: u8| {
        let state = position(rows, [0, 0], &[(1, 1, level, 6, 11)]);
        threatened(&state, 0)
    };
    // A level-1 unit's moves are left out; a level-2 one can take (5, 11),
    // the one red cell next to it.
    assert_eq!(threats(1), 0);
    assert_eq!(threats(2), 1);
}

#[test]
fn a_mine_counts_more_with_a_larger_territory() {
    use crate::eval::{eval, mine_bonus, threatened};
    // E033: a mine counts one more than its income for every 4 active cells
    // of its side.
    assert_eq!(
        [3, 4, 21, 36, 48].map(mine_bonus),
        [0, 1, 5, 9, 12],
        "the bonus by territory"
    );
    // What the bot's evaluation adds to the shared one, the shape term and
    // the opening's goal (E073, early turns).
    let own = |state: &State, seat: usize| {
        eval(state, seat)
            - ice_and_fire_tactics::eval::evaluate(state, seat)
            - crate::eval::shape(state, seat)
            - standing_term(state, seat)
            - crate::eval::goal(state, seat)
    };
    // Red holds 21 cells of the corridor and blue its HQ alone. Blue (20
    // gold, income 1) reaches (11, 9) for 20, and red blue's HQ: one
    // threatened cell each, so without a mine the bot's terms add nothing.
    let spots = "2\n5 0\n11 9\n";
    let start = position_with_spots(spots, CORRIDOR, [30, 20], &[]);
    assert_eq!((threatened(&start, 0), threatened(&start, 1)), (1, 1));
    assert_eq!(own(&start, 0), 0);
    // A mine on (5, 0), out of blue's reach: the shared evaluation counts
    // what it changes (15 gold less, 4 of income more), the bot's own adds
    // the bonus of a side with 21 cells (5), and E058's 6 since the
    // position is read early in the game (by turn 15), and takes as much
    // from blue's score.
    let mut state = start.clone();
    let income = state.income(0);
    assert!(state.apply(Command::BuildMine { x: 5, y: 0 }));
    assert_eq!((state.gold(0), state.income(0)), (15, income + 4));
    assert_eq!((threatened(&state, 0), threatened(&state, 1)), (1, 1));
    assert_eq!(own(&state, 0), 5 + 6);
    assert_eq!(own(&state, 1), -(5 + 6));
    // A mine on (11, 9), which blue can take at its next turn, adds nothing.
    let mut state = start.clone();
    assert!(state.apply(Command::BuildMine { x: 11, y: 9 }));
    assert_eq!((threatened(&state, 0), threatened(&state, 1)), (1, 1));
    assert_eq!(own(&state, 0), 0);
    // On a territory of 3 cells E033's bonus is nothing: the mine counts
    // only E058's 6 early in the game.
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
    let mut state = position_with_spots("1\n1 0\n", rows, [30, 20], &[]);
    assert!(state.apply(Command::BuildMine { x: 1, y: 0 }));
    assert_eq!(own(&state, 0), 6);
}

/// The towers among the actions of the player to act.
fn towers_offered(state: &State) -> Vec<(u8, u8)> {
    let mut actions = Vec::new();
    actions_of(state, &mut actions);
    actions
        .iter()
        .filter_map(|action| match action.commands() {
            [Command::BuildTower { x, y }] => Some((*x, *y)),
            _ => None,
        })
        .collect()
}

#[test]
fn towers_are_offered_only_where_they_guard_something() {
    // E026: blue (gold 20, income 1) can take (11, 9), the end of red's
    // corridor and its one border cell; a tower there keeps it out.
    let state = position(CORRIDOR, [50, 20], &[]);
    assert_eq!(crate::eval::threatened(&state, 0), 1);
    assert_eq!(towers_offered(&state), vec![(11, 9)]);
    // Far from blue, with nothing threatened and blue's chain to red's HQ
    // far beyond two incomes, no border cell gets a tower.
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
    let state = position(rows, [50, 20], &[]);
    assert_eq!(crate::eval::threatened(&state, 0), 0);
    assert!(towers_offered(&state).is_empty());
}

#[test]
fn a_tower_guards_the_hq_against_a_chain_affordable_within_two_turns() {
    // E026's HQ guard: blue holds rows 8 to 11 (income 48, no gold). Its
    // chain to red's HQ crosses 7 neutral cells: 80 gold, more than its 48
    // at its next turn, so no red cell is threatened, but within the 96 of
    // two turns. A tower at (1, 0) protects the HQ, which then costs 30:
    // the chain rises to 100 and the tower is offered. One at (2, 0)
    // protects (1, 0) only and leaves the chain as it was.
    let red = "OOO.........";
    let free = "............";
    let blue = "XXXXXXXXXXXX";
    let rows = [
        red, free, free, free, free, free, free, free, blue, blue, blue, blue,
    ];
    let state = position(rows, [50, 0], &[]);
    assert_eq!(state.income(1), 48);
    assert_eq!(crate::eval::threatened(&state, 0), 0);
    assert_eq!(
        ice_and_fire_tactics::chain::cost_to(&state, 1, false, ice_and_fire_engine::HQ[0]),
        80
    );
    // With E092's guard on the next turn alone, 48 does not pay for 80.
    // With E092's guard zone binding the HQ guard, (1, 0) is too far from
    // blue's land.
    let far = crate::actions::GUARD_NEAR_HQ && !crate::actions::guard_zone(&state).test(1);
    let want = if crate::actions::HQ_GUARD_TURNS >= 2 && !far {
        vec![(1, 0)]
    } else {
        vec![]
    };
    assert_eq!(towers_offered(&state), want);
}

/// Positions of random games, every third ply up to 90, on several maps.
fn many_positions() -> Vec<State> {
    let plies: Vec<u32> = (1..30).map(|n| 3 * n).collect();
    (0..16).flat_map(|seed| positions(seed, &plies)).collect()
}

#[test]
fn the_weakness_counts_the_joints_and_cuts_of_severed() {
    // E028: the evaluation's own search, and its cache, give the joints plus
    // the most cells one capture cuts off, as `severed` counts them.
    let mut checked = 0;
    for state in many_positions() {
        for seat in 0..2 {
            let cut = ice_and_fire_opening::solo::severed(&state, seat);
            let (mut joints, mut most) = (0, 0);
            for cell in state.active(seat).iter() {
                if cut[cell] > 0 {
                    joints += 1;
                    most = most.max(cut[cell]);
                }
            }
            let active = state.active(seat);
            let hq = ice_and_fire_engine::HQ[seat];
            assert_eq!(crate::eval::weakness_of(active, hq), joints + most);
            // Through the cache, twice.
            let other = 1 - seat;
            let want = 2
                * (crate::eval::weakness_of(state.active(other), ice_and_fire_engine::HQ[other])
                    - (joints + most));
            // The bot's path, through the cache, twice: every joint
            // counted (v031), or each weighted by its steps from the
            // enemy's land (E081).
            let want = if crate::eval::SHAPE_REACH == 0 {
                want
            } else {
                let weighted = |side: usize| {
                    let d = ice_and_fire_opening::econ::distances(&state, state.active(1 - side));
                    let mut w = [0u8; ice_and_fire_engine::CELLS];
                    for (w, &steps) in w.iter_mut().zip(d.iter()) {
                        *w = crate::eval::joint_weight(
                            crate::eval::SHAPE_REACH,
                            crate::eval::SHAPE_NEAR,
                            crate::eval::SHAPE_FAR,
                            steps,
                        );
                    }
                    crate::eval::weakness_weighted(
                        state.active(side),
                        ice_and_fire_engine::HQ[side],
                        &w,
                    )
                };
                2 * (weighted(other) - weighted(seat))
            };
            assert_eq!(crate::eval::shape(&state, seat), want);
            assert_eq!(crate::eval::shape(&state, seat), want);
            checked += 1;
        }
    }
    assert!(checked > 500, "{checked} sides checked");
}

#[test]
fn towers_are_offered_where_a_copy_of_the_state_says_they_guard() {
    // E028: the generator skips the copy where a tower cannot lower the
    // cells the enemy can take; it offers the same towers as E026's check
    // on a copy of the state.
    use ice_and_fire_engine::state::BUILD_COST;
    use ice_and_fire_engine::HQ;
    use ice_and_fire_tactics::chain::cost_to;
    let mut offered = 0;
    for state in many_positions() {
        let me = state.to_act();
        let enemy = 1 - me;
        // Only where towers are affordable, as the generator offers them.
        if state.outcome().is_some() || state.gold(me) < BUILD_COST {
            continue;
        }
        let threats = crate::eval::threatened(&state, me);
        let chain = cost_to(&state, enemy, false, HQ[me]);
        let free = state.owned(me) & state.near_active(me) & state.empty_cells();
        let border = state.map().valid.and_not(state.owned(me)).neighbours();
        let mut want = Vec::new();
        for cell in ((free & border).and_not(state.map().spots)).iter() {
            let (x, y) = crate::actions::xy(cell);
            let mut after = state.clone();
            if !after.apply(Command::BuildTower { x, y }) {
                continue;
            }
            // E068: a tower that would guard a frontier cell of ours not yet
            // guarded is offered too.
            let walls = crate::actions::WALL_TOWERS && {
                let (front, guarded) = crate::eval::frontier(&state, me);
                let zone = Bits::cell(cell) | Bits::cell(cell).neighbours();
                !(zone & front.and_not(guarded)).is_empty()
            };
            let near = crate::actions::guard_zone(&state).test(cell);
            let guards = walls
                || near
                    && if crate::actions::GUARD_BY_CUT {
                        crate::actions::best_cut(&after) < crate::actions::best_cut(&state)
                    } else {
                        crate::eval::threatened(&after, me) < threats
                    }
                || (crate::actions::HQ_GUARD
                    && (near || !crate::actions::GUARD_NEAR_HQ)
                    && chain
                        <= state.gold(enemy)
                            + crate::actions::HQ_GUARD_TURNS * state.income(enemy).max(0)
                    && cost_to(&after, enemy, false, HQ[me]) > chain);
            if guards {
                want.push(Command::BuildTower { x, y });
            }
        }
        let mut actions = Vec::new();
        actions_of(&state, &mut actions);
        let got: Vec<Command> = actions
            .iter()
            .filter_map(|a| match a {
                Action::One(c @ Command::BuildTower { .. }) => Some(*c),
                _ => None,
            })
            .collect();
        assert_eq!(got, want);
        offered += got.len();
    }
    assert!(offered > 0, "no tower offered in the positions");
}

#[test]
fn climbs_out_of_first_actions_go_on_from_the_others() {
    // E029: in some middle-game positions the climbs run out of first
    // actions that improve the plan, and the search then climbs from each of
    // the others; its answer stays a list of commands the rules accept.
    let mut forced = 0;
    for state in many_positions().into_iter().step_by(4) {
        if state.outcome().is_some() {
            continue;
        }
        let thought = think(&state, evals(400), &mut opening());
        forced += thought.forced;
        let mut scratch = state.clone();
        for &command in &thought.commands {
            assert!(scratch.apply(command), "{command:?}");
        }
    }
    assert!(forced > 0, "no position reached the forced climbs");
}

#[test]
fn the_opening_builds_no_mine() {
    // E034: on the referee's maps, both seats play the bot's turns from the
    // start (opening then middle game, a few tries a turn); no command of
    // a turn the opening played is a mine.
    for seed in 1..=4 {
        let map = generate(seed, RefereeLeague::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let mut openings = [opening(), opening()];
        let mut opening_turns = 0;
        for _ in 0..24 {
            if state.outcome().is_some() {
                break;
            }
            let seat = state.to_act();
            let thought = think(&state, evals(50), &mut openings[seat]);
            let commands = thought.commands;
            if thought.opening {
                opening_turns += 1;
                assert!(
                    !commands
                        .iter()
                        .any(|c| matches!(c, Command::BuildMine { .. })),
                    "seed {seed}: the opening built a mine: {commands:?}"
                );
            }
            state.play_turn(&commands);
        }
        assert!(
            opening_turns >= 4,
            "seed {seed}: {opening_turns} opening turns"
        );
    }
}

#[test]
fn a_side_that_can_pay_for_its_chain_to_the_enemy_hq_holds_a_threat() {
    use crate::eval::{standing, HQ_BONUS, HQ_MARGIN_CAP, HQ_MARGIN_HALVES};
    use ice_and_fire_tactics::eval::{evaluate, evaluate_with_margins, margin};
    // E042: nothing while the side cannot pay; once it can, the bonus and
    // HQ_MARGIN_HALVES halves of the gold left, up to the cap.
    assert_eq!(standing(-1), 0);
    assert_eq!(standing(i64::MIN), 0);
    assert_eq!(standing(0), HQ_BONUS);
    assert_eq!(
        standing(10 * HQ_MARGIN_CAP),
        HQ_BONUS + HQ_MARGIN_HALVES * HQ_MARGIN_CAP / 2
    );
    // On the corridor, red (income 21) pays 20 for blue's HQ: its margin is
    // its gold and income less 20; blue cannot reach red's HQ for 30.
    let state = position(CORRIDOR, [0, 30], &[]);
    assert_eq!(margin(&state, 0), 21 - 20);
    assert!(margin(&state, 1) < 0);
    let (score, margins) = evaluate_with_margins(&state, 0);
    assert_eq!(score, evaluate(&state, 0));
    assert_eq!(margins, [margin(&state, 0), margin(&state, 1)]);
}

#[test]
fn an_enemy_unit_of_level_two_moves_toward_our_hq() {
    use crate::eval::{eval, shape, strong_units, threatened};
    use ice_and_fire_engine::{Bits, SIZE};
    use ice_and_fire_tactics::eval::{margin, margin_moving};
    // E043: blue's level-2 unit on (6, 11) steps first onto red's (5, 11),
    // which makes blue's chain to red's HQ 10 cheaper; its margin stays
    // within the pressure term's 200, so the term changes too.
    let rows = [
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOO......",
        "OOOOOOXXXXXX",
    ];
    let state = position(rows, [0, 0], &[(1, 1, 2, 6, 11)]);
    let unit = Bits::cell(11 * SIZE + 6);
    let still = margin(&state, 1);
    assert_eq!(margin_moving(&state, 1, unit), still + 10);
    assert!((-200..-10).contains(&still), "{still}");
    // The bot's evaluation is the shared one with both sides' movers and the
    // terms these positions hold (no debt, no mine), the opening's goal
    // included (E073).
    let expected = bot_shared(&state, 0) + shape(&state, 0) + strong_units(&state, 0)
        - strong_units(&state, 1)
        + crate::eval::threat_quarters(&state) * (threatened(&state, 1) - threatened(&state, 0))
            / 4
        + crate::eval::goal(&state, 0);
    assert_eq!(eval(&state, 0), expected);
}

#[test]
fn the_cells_a_reply_enters_get_blockers() {
    use crate::search::blockers_of;
    use ice_and_fire_engine::Command;
    // E044: red's line from its HQ to (5, 0); a reply of blue's trains on
    // (3, 0) and moves onto (4, 0): towers on those two, on their free
    // neighbours with `neighbours`, and level-2 trainings on them.
    let rows = [
        "OOOOOO......",
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
    let state = position(rows, [100, 20], &[]);
    let reply = [
        Command::Train {
            level: 1,
            x: 3,
            y: 0,
        },
        Command::Move { id: 9, x: 4, y: 0 },
        Command::Train {
            level: 1,
            x: 7,
            y: 7,
        },
    ];
    let names = |settings| {
        let mut out = Vec::new();
        blockers_of(&state, std::iter::once(&reply[..]), settings, &mut out);
        let mut names: Vec<String> = out
            .iter()
            .map(|action| ice_and_fire_engine::answer_line(action.commands()))
            .collect();
        names.sort();
        names
    };
    assert_eq!(
        names((true, false, None)),
        ["BUILD TOWER 3 0", "BUILD TOWER 4 0"]
    );
    assert_eq!(
        names((true, true, None)),
        [
            "BUILD TOWER 2 0",
            "BUILD TOWER 3 0",
            "BUILD TOWER 4 0",
            "BUILD TOWER 5 0"
        ]
    );
    assert_eq!(
        names((false, false, Some(2))),
        ["TRAIN 2 3 0", "TRAIN 2 4 0"]
    );
    assert!(names((false, false, None)).is_empty());
    // An action already offered is not offered twice.
    let mut out = vec![crate::actions::Action::One(Command::BuildTower {
        x: 3,
        y: 0,
    })];
    blockers_of(
        &state,
        std::iter::once(&reply[..]),
        (true, false, None),
        &mut out,
    );
    assert_eq!(out.len(), 2);
}

#[test]
fn no_training_is_offered_on_our_own_active_cells() {
    use crate::actions::OWN_CELLS;
    use ice_and_fire_engine::SIZE;
    // E045: with OWN_CELLS 2, the generator offers trainings only on cells
    // that are not active for us; towers on our cells stay offered.
    assert_eq!(OWN_CELLS, 2);
    let state = position(CORRIDOR, [50, 20], &[]);
    let mut actions = Vec::new();
    actions_of(&state, &mut actions);
    let active = state.active(0);
    let mut trainings = 0;
    for action in &actions {
        if let [Command::Train { x, y, .. }] = action.commands() {
            trainings += 1;
            let cell = usize::from(*y) * SIZE + usize::from(*x);
            assert!(!active.test(cell), "a training on our cell ({x}, {y})");
        }
    }
    assert!(trainings > 0, "no training at all");
    assert_eq!(towers_offered(&state), vec![(11, 9)]);
}

#[test]
fn the_opening_buys_units_that_pay_back_by_turn_18() {
    use ice_and_fire_opening::econ::unit_return;
    // E056: the opening's purchases must pay back by game turn 18 (15 in
    // v023). A unit trained at game turn 6 that takes one new cell pays
    // back by turn 18, not by turn 15.
    assert_eq!(crate::OPENING_HORIZON, 18);
    assert!(unit_return(18 - 6, 1) > 0);
    assert!(unit_return(15 - 6, 1) <= 0);
}

#[test]
fn a_safe_mine_counts_more_early_in_the_middle_game() {
    use crate::eval::{eval, threatened, EARLY_MINE_BONUS, EARLY_MINE_PER_CELLS, EARLY_MINE_TURN};
    // E058: until game turn 15 a safe mine counts 6 more than E033's bonus.
    assert_eq!((EARLY_MINE_TURN, EARLY_MINE_BONUS), (15, 6));
    assert_eq!(EARLY_MINE_PER_CELLS, crate::eval::MINE_PER_CELLS);
    // As in `a_mine_counts_more_with_a_larger_territory`: a mine on (5, 0),
    // out of blue's reach, with red's 21 cells: 21 / 4 + 6 = 11 until game
    // turn 15, E033's 21 / 4 = 5 alone from turn 16.
    let own = |state: &State, seat: usize| {
        eval(state, seat)
            - ice_and_fire_tactics::eval::evaluate(state, seat)
            - crate::eval::shape(state, seat)
            - standing_term(state, seat)
    };
    let spots = "2\n5 0\n11 9\n";
    for (turn, bonus) in [(15, 21 / 4 + 6), (16, 21 / 4)] {
        let start = position_at(turn, spots, CORRIDOR, [30, 20], &[]);
        assert_eq!(start.game_turn(), turn);
        let mut state = start.clone();
        assert!(state.apply(Command::BuildMine { x: 5, y: 0 }));
        assert_eq!((threatened(&state, 0), threatened(&state, 1)), (1, 1));
        assert_eq!(own(&state, 0) - own(&start, 0), bonus, "turn {turn}");
        assert_eq!(own(&state, 1) - own(&start, 1), -bonus, "turn {turn}");
    }
    // On a territory of 3 cells E033's bonus is nothing: the mine counts 6
    // at turn 15 and nothing at turn 16.
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
    for (turn, bonus) in [(15, 6), (16, 0)] {
        let start = position_at(turn, "1\n1 0\n", rows, [30, 20], &[]);
        let mut state = start.clone();
        assert!(state.apply(Command::BuildMine { x: 1, y: 0 }));
        assert_eq!(own(&state, 0) - own(&start, 0), bonus, "turn {turn}");
    }
}

#[test]
fn a_unit_far_from_land_to_take_costs_its_side() {
    use crate::eval::{eval, IDLE_CAP, IDLE_LEVEL, IDLE_STEP};
    // E060: each unit costs its side 2 for every step beyond the first to
    // the nearest cell that is not its side's, counted up to 4 steps.
    assert_eq!((IDLE_STEP, IDLE_CAP, IDLE_LEVEL), (2, 4, 3));
    // Red's corridor: the nearest cell not red's is the neutral (11, 10). A
    // unit on (11, 7) is 3 steps from it (2 x 2 = 4); one on (5, 0) is far
    // beyond 4 steps (4 x 2 = 8). Neither changes what blue can take, nor
    // anything else the evaluation counts.
    let near = position(CORRIDOR, [0, 20], &[(0, 1, 1, 11, 7)]);
    let far = position(CORRIDOR, [0, 20], &[(0, 1, 1, 5, 0)]);
    assert_eq!(eval(&near, 0) - eval(&far, 0), 8 - 4);
    assert_eq!(eval(&near, 1) - eval(&far, 1), -(8 - 4));
}

#[test]
fn the_search_takes_over_when_the_opening_would_keep_its_gold() {
    use crate::search::{HANDOVER_GOLD, HANDOVER_IDLE, HANDOVER_IDLE_FROM};
    // E067: on the referee's maps, from the hand-over's first turn, no turn
    // that the opening played keeps the gold of a training without buying,
    // and once the search has played, the opening never plays again.
    if !HANDOVER_IDLE {
        return;
    }
    let mut handed = 0;
    for seed in 1..=6 {
        let map = generate(seed, RefereeLeague::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let mut openings = [opening(), opening()];
        let mut searched = [false; 2];
        for _ in 0..24 {
            if state.outcome().is_some() {
                break;
            }
            let seat = state.to_act();
            let thought = think(&state, evals(50), &mut openings[seat]);
            let buys = thought.commands.iter().any(|c| {
                matches!(
                    c,
                    Command::Train { .. } | Command::BuildMine { .. } | Command::BuildTower { .. }
                )
            });
            if thought.opening {
                assert!(!searched[seat], "seed {seed}: the opening played again");
                let rich = state.gold(seat) >= HANDOVER_GOLD;
                let late = state.game_turn() >= HANDOVER_IDLE_FROM;
                assert!(buys || !rich || !late, "seed {seed}: gold kept");
            } else {
                if !searched[seat] && !crate::tests::middle(&state) {
                    handed += 1;
                }
                searched[seat] = true;
            }
            state.play_turn(&thought.commands);
        }
    }
    assert!(handed > 0, "no hand-over before the middle game");
}

/// Whether the middle game has begun for the player to act.
fn middle(state: &State) -> bool {
    ice_and_fire_opening::middle::reached(state)
}

#[test]
fn the_search_plays_from_game_turn_seven() {
    use crate::search::HANDOVER_TURN;
    // E067: on the referee's maps the opening plays no turn from game turn
    // 7 on, even before the middle game, and every earlier turn of the
    // opening's while it has not ended; some game reaches turn 7 before its
    // middle game, so the rule does change who plays.
    assert_eq!(HANDOVER_TURN, Some(7));
    let mut handed = 0;
    for seed in 1..=6 {
        let map = generate(seed, RefereeLeague::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let mut openings = [opening(), opening()];
        let mut searched = [false; 2];
        for _ in 0..24 {
            if state.outcome().is_some() {
                break;
            }
            let seat = state.to_act();
            let early = state.game_turn() < 7;
            let before_middle = !middle(&state);
            let thought = think(&state, evals(50), &mut openings[seat]);
            if thought.opening {
                assert!(early, "seed {seed}: the opening played turn 7 or later");
                assert!(!searched[seat], "seed {seed}: the opening played again");
            } else {
                if !searched[seat] && before_middle {
                    assert!(!early, "seed {seed}: the search played too soon");
                    handed += 1;
                }
                searched[seat] = true;
            }
            state.play_turn(&thought.commands);
        }
    }
    assert!(handed > 0, "no hand-over before the middle game");
}

#[test]
fn a_tower_counts_wall_for_each_frontier_cell_it_guards() {
    // E068: the wall term counts WALL for each active cell of a side next
    // to an enemy cell that is one of its active towers or next to one.
    use crate::eval::frontier;
    let mut checked = 0;
    for state in many_positions() {
        if state.outcome().is_some() {
            continue;
        }
        for seat in 0..2 {
            let (front, guarded) = frontier(&state, seat);
            let towers = state.towers() & state.active(seat);
            let mut want = 0;
            for cell in front.iter() {
                let zone = Bits::cell(cell) | Bits::cell(cell).neighbours();
                if !(zone & towers).is_empty() {
                    want += 1;
                }
            }
            assert_eq!(guarded.count(), want);
            assert!(guarded.and_not(front).is_empty());
            checked += usize::from(want > 0);
        }
    }
    assert!(checked > 0, "no guarded frontier in the positions");
}

#[test]
fn the_strike_reserve_keeps_a_strike_affordable_next_turn() {
    // E069: when the reserve holds at the start of the turn, the plan played
    // wins or leaves our chain to the enemy HQ affordable within
    // STRIKE_TURNS turns of income.
    use crate::search::{strike_reserve, strike_within, STRIKE_TURNS};
    let mut checked = 0;
    for state in many_positions() {
        if state.outcome().is_some() || !strike_reserve(&state) {
            continue;
        }
        let thought = think(&state, evals(300), &mut opening());
        if thought.opening {
            continue;
        }
        let mut end = state.clone();
        for &command in &thought.commands {
            assert!(end.apply(command), "{command:?} refused");
        }
        let me = state.to_act();
        assert!(
            end.outcome() == Some(Outcome::Win(me)) || strike_within(&end, me, STRIKE_TURNS),
            "{:?}",
            thought.commands
        );
        checked += 1;
    }
    assert!(
        checked > 0 || STRIKE_TURNS == 0,
        "no position with a reserve"
    );
}

#[test]
fn step_and_guard_steps_into_a_neutral_cell_and_guards_the_cell_left() {
    // E070: a unit steps from a cell the enemy can reach into a neutral
    // cell, then the cell it left gets a tower (1) or a level-1 unit (2).
    use crate::actions::STEP_GUARD;
    let mut found = 0;
    for state in many_positions() {
        let mut actions = Vec::new();
        actions_of(&state, &mut actions);
        let me = state.to_act();
        for action in &actions {
            let Action::Many(commands) = action else {
                continue;
            };
            let [Command::Move { x, y, .. }, guard] = commands[..] else {
                continue;
            };
            let to = usize::from(y) * SIZE + usize::from(x);
            let (gx, gy) = match guard {
                Command::BuildTower { x, y } if STEP_GUARD == 1 => (x, y),
                Command::Train { level: 1, x, y } if STEP_GUARD == 2 => (x, y),
                _ => continue,
            };
            let from = usize::from(gy) * SIZE + usize::from(gx);
            if crate::actions::FALL_BACK != 0
                && (state.owned(me).test(to) || state.owned(1 - me).test(to))
            {
                // E076's fall back and guard, checked by its own test.
                continue;
            }
            assert!(!state.owned(me).test(to) && !state.owned(1 - me).test(to));
            assert!(crate::eval::reach(&state, 1 - me).test(from));
            let mut after = state.clone();
            assert!(
                after.apply(commands[0]) && after.apply(commands[1]),
                "{commands:?}"
            );
            assert!(after.owned(me).test(to));
            found += 1;
        }
    }
    assert!(found > 0 || STEP_GUARD == 0, "no step and guard offered");
}

#[test]
fn the_opening_goal_counts_the_band_cells_each_side_holds() {
    use crate::eval::{goal, GOAL, GOAL_CHOKE, GOAL_GUARD, GOAL_OWNED, GOAL_UNTIL};
    // E073 (v030): the goal is the band cells held, GOAL_OWNED each, the
    // side's less the enemy's, until GOAL_UNTIL.
    assert_eq!((GOAL, GOAL_GUARD, GOAL_CHOKE), (0, 0, 0), "v030's goal");
    // Red reaches the middle along row 5; blue holds its corner.
    let rows = [
        "OOOO........",
        "OOOO........",
        "OOOO........",
        "OOOO........",
        "OOOO........",
        "OOOOOOO.....",
        "............",
        "............",
        "........XXXX",
        "........XXXX",
        "........XXXX",
        "........XXXX",
    ];
    let state = position_at(GOAL_UNTIL, "0\n", rows, [20, 20], &[]);
    let rings = ice_and_fire_opening::solo::rings(&state);
    let band = |seat: usize| {
        state
            .active(seat)
            .iter()
            .filter(|&c| rings[c] <= crate::eval::GOAL_BAND)
            .count() as i64
    };
    // Red's row 5 reaches cells of the band; blue's corner holds none.
    assert!(band(0) > band(1), "{} {}", band(0), band(1));
    assert_eq!(goal(&state, 0), GOAL_OWNED * (band(0) - band(1)));
    assert_eq!(goal(&state, 1), -goal(&state, 0));
    // After GOAL_UNTIL it counts nothing.
    let later = position_at(GOAL_UNTIL + 1, "0\n", rows, [20, 20], &[]);
    assert_eq!(goal(&later, 0), 0);
}

#[test]
fn fall_back_and_guard_steps_onto_an_owned_cell_and_guards_the_cell_left() {
    // E076: a unit on a cell the enemy can reach steps onto an owned cell
    // (back, along the border, or any, by FALL_BACK), then a tower goes on
    // the cell it left.
    use crate::actions::FALL_BACK;
    use ice_and_fire_engine::HQ;
    let mut found = 0;
    for state in many_positions() {
        let mut actions = Vec::new();
        actions_of(&state, &mut actions);
        let me = state.to_act();
        for action in &actions {
            let Action::Many(commands) = action else {
                continue;
            };
            let [Command::Move { x, y, .. }, Command::BuildTower { x: gx, y: gy }] = commands[..]
            else {
                continue;
            };
            let to = usize::from(y) * SIZE + usize::from(x);
            if !state.owned(me).test(to) && !state.owned(1 - me).test(to) {
                continue; // E070's step and guard
            }
            assert_ne!(
                std::hint::black_box(FALL_BACK),
                0,
                "a fall back offered with FALL_BACK 0"
            );
            let from = usize::from(gy) * SIZE + usize::from(gx);
            assert!(crate::eval::reach(&state, 1 - me).test(from));
            let map = state.map();
            match FALL_BACK {
                1 => assert!(
                    state.owned(me).test(to)
                        && map.distance(to, HQ[1 - me]) > map.distance(from, HQ[1 - me])
                ),
                2 => assert!(state.owned(me).test(to) && state.owned(1 - me).neighbours().test(to)),
                _ => {}
            }
            let mut after = state.clone();
            assert!(
                after.apply(commands[0]) && after.apply(commands[1]),
                "{commands:?}"
            );
            assert!(after.towers().test(from));
            found += 1;
        }
    }
    assert!(found > 0 || FALL_BACK == 0, "no fall back offered");
}

#[test]
fn the_rush_trains_a_line_from_the_cell_nearest_the_enemy_hq() {
    // E076: chains of level-1 trainings into neutral cells, each next to
    // the one before, starting next to our active land.
    use crate::actions::RUSH_LENGTH;
    let mut found = 0;
    for state in many_positions() {
        let mut actions = Vec::new();
        actions_of(&state, &mut actions);
        let me = state.to_act();
        let neutral = state
            .map()
            .valid
            .and_not(state.owned(me) | state.owned(1 - me));
        for action in &actions {
            let Action::Many(commands) = action else {
                continue;
            };
            let cells: Option<Vec<usize>> = commands
                .iter()
                .map(|c| match *c {
                    Command::Train { level: 1, x, y } => {
                        Some(usize::from(y) * SIZE + usize::from(x))
                    }
                    _ => None,
                })
                .collect();
            let Some(cells) = cells else {
                continue;
            };
            if !cells.iter().all(|&c| neutral.test(c)) {
                continue; // a cut or the lethal chain
            }
            // E076's defence chains are up to 3 long when RUSH_LENGTH is 0.
            let longest = std::hint::black_box(RUSH_LENGTH).max(if crate::actions::RUSH_DEFEND {
                3
            } else {
                0
            });
            assert!(cells.len() >= 2 && cells.len() <= longest, "{commands:?}");
            assert!(state.active(me).neighbours().test(cells[0]));
            for pair in cells.windows(2) {
                assert!(Bits::cell(pair[0]).neighbours().test(pair[1]));
            }
            let mut after = state.clone();
            for &c in commands {
                assert!(after.apply(c), "{commands:?}");
            }
            found += 1;
        }
    }
    assert!(found > 0 || RUSH_LENGTH == 0, "no rush offered");
}

#[test]
fn the_rush_heads_for_chokes_then_band_cells_no_unit_of_ours_claims() {
    // E076, RUSH_GOAL (the owner's choice, 2026-11-20): the goals are the
    // middle chokes we do not hold that none of our units claims (no
    // farther from it than our nearest trainable cell and nearer than
    // every enemy unit), else the band cells alike, the nearest first.
    use crate::actions::{rush_goals, RUSH_STARTS};
    let mut found = 0;
    for state in many_positions() {
        let me = state.to_act();
        let map = state.map();
        let neutral = map.valid.and_not(state.owned(me) | state.owned(1 - me));
        let trainable: Vec<usize> =
            (state.active(me).neighbours() & neutral & state.enterable(me, 1))
                .iter()
                .collect();
        if trainable.is_empty() {
            continue;
        }
        let (band, chokes) = crate::eval::goal_zones(&state);
        let reach = |goal: usize| {
            trainable
                .iter()
                .map(|&c| map.distance(c, goal))
                .min()
                .unwrap()
        };
        let claimed = |goal: usize| {
            state.units(me).iter().any(|u| {
                let d = map.distance(u, goal);
                d <= reach(goal)
                    && state
                        .units(1 - me)
                        .iter()
                        .all(|e| d < map.distance(e, goal))
            })
        };
        let lost = |goal: usize| {
            crate::actions::RUSH_SKIP_ENEMY
                && (state.owned(1 - me).test(goal)
                    || state
                        .units(1 - me)
                        .iter()
                        .any(|e| map.distance(e, goal) < reach(goal)))
        };
        let goals = rush_goals(&state, &trainable);
        assert!(goals.len() <= RUSH_STARTS);
        let open_chokes: Vec<usize> = chokes
            .and_not(state.owned(me))
            .iter()
            .filter(|&g| reach(g) != u8::MAX && !claimed(g) && !lost(g))
            .collect();
        for (i, &goal) in goals.iter().enumerate() {
            assert!(!state.owned(me).test(goal) && !claimed(goal) && !lost(goal));
            if open_chokes.is_empty() {
                assert!(band.test(goal));
            } else {
                assert!(chokes.test(goal));
            }
            if i > 0 {
                assert!(reach(goals[i - 1]) <= reach(goal));
            }
            found += 1;
        }
    }
    assert!(found > 0, "no goal found");
}

#[test]
fn a_leapfrog_steps_a_unit_onto_the_rush_start_and_trains_beyond_it() {
    // E076, RUSH_LEAP: a movable unit next to a rush chain's first cell steps
    // onto it, then level-1 trainings follow the chain from there.
    use crate::actions::RUSH_LEAP;
    let mut found = 0;
    for state in many_positions() {
        let mut actions = Vec::new();
        actions_of(&state, &mut actions);
        let me = state.to_act();
        let neutral = state
            .map()
            .valid
            .and_not(state.owned(me) | state.owned(1 - me));
        for action in &actions {
            let Action::Many(commands) = action else {
                continue;
            };
            let Some((Command::Move { x, y, .. }, rest)) = commands.split_first() else {
                continue;
            };
            if rest.is_empty()
                || !rest
                    .iter()
                    .all(|c| matches!(c, Command::Train { level: 1, .. }))
            {
                continue;
            }
            let start = usize::from(*y) * SIZE + usize::from(*x);
            let cells: Vec<usize> = rest
                .iter()
                .filter_map(|c| match *c {
                    Command::Train { x, y, .. } => Some(usize::from(y) * SIZE + usize::from(x)),
                    _ => None,
                })
                .collect();
            if !neutral.test(start) || !cells.iter().all(|&c| neutral.test(c)) {
                continue; // another macro action (E070 guards a cell of ours)
            }
            assert!(
                std::hint::black_box(RUSH_LEAP),
                "a leapfrog offered with RUSH_LEAP off"
            );
            let mut last = start;
            for &cell in &cells {
                assert!(Bits::cell(last).neighbours().test(cell), "{commands:?}");
                last = cell;
            }
            let mut after = state.clone();
            for &c in commands {
                assert!(after.apply(c), "{commands:?}");
            }
            found += 1;
        }
    }
    assert!(
        found > 0 || !RUSH_LEAP || crate::actions::RUSH_LENGTH == 0,
        "no leapfrog offered"
    );
}

#[test]
fn the_land_term_counts_the_cells_nearer_each_side() {
    // E076, VORONOI: each valid cell nearer to a side's active land than to
    // the enemy's counts VORONOI for it, until VORONOI_UNTIL.
    use crate::eval::{voronoi, VORONOI, VORONOI_UNTIL};
    let mut checked = 0;
    for state in many_positions() {
        let a = voronoi(&state, 0);
        assert_eq!(a, -voronoi(&state, 1));
        if state.game_turn() > VORONOI_UNTIL {
            assert_eq!(a, 0);
            continue;
        }
        let map = state.map();
        let near = |c: usize, side: usize| {
            state
                .active(side)
                .iter()
                .map(|a| map.distance(a, c))
                .min()
                .unwrap_or(u8::MAX)
        };
        let mut net = 0;
        for c in map.valid.iter() {
            let (r, b) = (near(c, 0), near(c, 1));
            net += i64::from(r < b) - i64::from(b < r);
        }
        assert_eq!(a, VORONOI * net);
        checked += 1;
    }
    assert!(checked > 0);
}

#[test]
fn the_contested_centre_cells_are_those_the_enemy_can_grab() {
    // E076, RUSH_DEFEND: chokes and band cells the enemy reaches before our
    // land (not owned by either side), or ours that its next turn can take.
    use crate::actions::{contested, RUSH_STARTS};
    let mut found = 0;
    for state in many_positions() {
        let me = state.to_act();
        let map = state.map();
        let (band, chokes) = crate::eval::goal_zones(&state);
        let reach = crate::eval::reach(&state, 1 - me);
        let from_land = |c: usize| {
            state
                .active(me)
                .iter()
                .map(|a| map.distance(a, c))
                .min()
                .unwrap_or(u8::MAX)
        };
        let cells = contested(&state);
        assert!(cells.len() <= 2 * RUSH_STARTS);
        for &c in &cells {
            assert!(band.test(c) || chokes.test(c));
            if state.owned(me).test(c) {
                assert!(state.active(me).test(c) && reach.test(c));
            } else {
                assert!(!state.owned(1 - me).test(c));
                assert!(state
                    .units(1 - me)
                    .iter()
                    .any(|e| map.distance(e, c) < from_land(c)));
            }
            found += 1;
        }
    }
    assert!(found > 0, "no contested cell");
}

#[test]
fn a_joints_weight_falls_with_its_distance_from_the_enemy() {
    use crate::eval::joint_weight;
    assert!((0..20).all(|d| joint_weight(0, 3, 6, d) == 4));
    assert_eq!(
        (0..6).map(|d| joint_weight(1, 3, 6, d)).collect::<Vec<_>>(),
        vec![4, 4, 4, 4, 0, 0]
    );
    assert_eq!(
        (1..8).map(|d| joint_weight(2, 2, 6, d)).collect::<Vec<_>>(),
        vec![4, 4, 3, 2, 1, 0, 0]
    );
    assert_eq!(joint_weight(2, 3, 6, u8::MAX), 0);
}

#[test]
fn weighted_weakness_counts_only_the_weighted_joints() {
    use crate::eval::{weakness_of, weakness_weighted};
    use ice_and_fire_engine::{Bits, CELLS};
    for state in many_positions().into_iter().take(200) {
        for seat in 0..2 {
            let active = state.active(seat);
            let hq = ice_and_fire_engine::HQ[seat];
            assert_eq!(
                weakness_weighted(active, hq, &[4; CELLS]),
                weakness_of(active, hq)
            );
            assert_eq!(weakness_weighted(active, hq, &[0; CELLS]), 0);
        }
    }
    // A line from the HQ at (0, 0) along the top row: every cell but the
    // last is a joint. Weighting only (5, 0) counts it and the 6 cells
    // beyond it.
    let mut line = Bits::EMPTY;
    for x in 0..12 {
        line.set(x);
    }
    let mut weights = [0u8; CELLS];
    weights[5] = 4;
    assert_eq!(weakness_weighted(line, 0, &weights), 1 + 6);
    assert_eq!(weakness_of(line, 0), 10 + 10);
}

#[test]
fn only_joints_near_the_enemy_count_from_v032() {
    // Red's line (0,0) to (3,0); blue's land comes down column 11 and along
    // row 2 to (3,2). Red's joints (1,0) and (2,0) are 4 and 3 steps from
    // blue's land; blue's joints (4,2) and (5,2) are 3 and 4 steps from
    // red's.
    let rows = [
        "OOOO........",
        "...........X",
        "...XXXXXXXXX",
        "...........X",
        "...........X",
        "...........X",
        "...........X",
        "...........X",
        "...........X",
        "...........X",
        "...........X",
        "...........X",
    ];
    let state = position(rows, [20, 20], &[]);
    let weakness = |seat: usize| {
        let d = ice_and_fire_opening::econ::distances(&state, state.active(1 - seat));
        let mut w = [0u8; ice_and_fire_engine::CELLS];
        for (w, &steps) in w.iter_mut().zip(d.iter()) {
            *w = crate::eval::joint_weight(1, 3, 6, steps);
        }
        crate::eval::weakness_weighted(state.active(seat), ice_and_fire_engine::HQ[seat], &w)
    };
    // Each side: only the joint 3 steps away counts, cutting off the one
    // cell beyond it ((3,0) for red, (3,2) for blue).
    assert_eq!(weakness(0), 1 + 1);
    assert_eq!(weakness(1), 1 + 1);
    // Every joint counted (v031): red's (1,0) cuts off 2 cells.
    assert_eq!(
        crate::eval::weakness_of(state.active(0), ice_and_fire_engine::HQ[0]),
        2 + 2
    );
}
