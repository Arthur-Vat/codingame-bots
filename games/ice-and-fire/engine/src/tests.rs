use cg_arena::referee::{Answer, Outcome as RefereeOutcome, Referee};
use cg_core::rng::Rng;
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::{IceAndFireReferee, League as RefereeLeague};

use super::*;

const LEAGUES: [League; 4] = [League::Wood3, League::Wood2, League::Wood1, League::Bronze];

fn referee_league(league: League) -> RefereeLeague {
    match league {
        League::Wood3 => RefereeLeague::Wood3,
        League::Wood2 => RefereeLeague::Wood2,
        League::Wood1 => RefereeLeague::Wood1,
        League::Bronze => RefereeLeague::Bronze,
    }
}

/// A map with only the cells in `open` (and the corner blocks of both HQs)
/// not void.
fn state_with_voids(open: &[(usize, usize)], league: League) -> State {
    let mut void = [true; CELLS];
    for y in 0..3 {
        for x in 0..3 {
            void[y * SIZE + x] = false;
            void[(SIZE - 1 - y) * SIZE + SIZE - 1 - x] = false;
        }
    }
    for &(x, y) in open {
        void[y * SIZE + x] = false;
    }
    State::new(&void, &[], league)
}

/// A referee and an engine state on the same map, played in lockstep with
/// random turns of the engine.
struct Pair {
    referee: IceAndFireReferee,
    state: State,
}

impl Pair {
    fn new(seed: u64, league: League) -> Pair {
        let map = generate(seed, referee_league(league));
        Pair {
            referee: IceAndFireReferee::from_map(map.clone(), referee_league(league)),
            state: State::new(&map.void, &map.mine_spots, league),
        }
    }

    fn step(&mut self, rng: &mut Rng) {
        let mut commands = Vec::new();
        self.state.random_turn(rng, &mut commands);
        let seat = self.state.to_act();
        let answer = Answer {
            seat,
            lines: vec![answer_line(&commands)],
        };
        self.referee.play(&[answer]).expect("a valid answer");
        self.state.play_turn(&commands);
    }

    fn initial(&self) -> String {
        self.referee.initial_input(0)
    }
}

/// What a bot could tell two states apart by, except the unit counter.
fn assert_same_position(read: &State, state: &State, context: &str) {
    let seat = state.to_act();
    assert_eq!(read.to_act(), seat, "{context}");
    assert_eq!(read.game_turn(), state.game_turn(), "{context}");
    for owner in 0..2 {
        assert_eq!(read.gold(owner), state.gold(owner), "{context}");
        assert_eq!(read.income(owner), state.income(owner), "{context}");
        assert_eq!(read.owned(owner), state.owned(owner), "{context}");
        assert_eq!(read.active(owner), state.active(owner), "{context}");
        assert_eq!(read.units(owner), state.units(owner), "{context}");
        assert_eq!(read.turn_input(owner), state.turn_input(owner), "{context}");
    }
    assert_eq!(read.mine, state.mine, "{context}");
    assert_eq!(read.tower, state.tower, "{context}");
    assert_eq!(read.level, state.level, "{context}");
    assert_eq!(read.movable, state.units(seat), "{context}");
    assert_eq!(
        state.movable & state.units(seat),
        state.units(seat),
        "{context}"
    );
    assert!(read.next_unit_id() <= state.next_unit_id(), "{context}");
    assert_eq!(read.initial_input(0), state.initial_input(0), "{context}");
}

#[test]
fn answer_line_joins_with_semicolons() {
    assert_eq!(answer_line(&[]), "WAIT");
    assert_eq!(answer_line(&[Command::Wait]), "WAIT");
    assert_eq!(
        answer_line(&[
            Command::Move { id: 3, x: 4, y: 5 },
            Command::Train {
                level: 2,
                x: 0,
                y: 11
            },
            Command::BuildMine { x: 1, y: 0 },
            Command::BuildTower { x: 2, y: 2 },
        ]),
        "MOVE 3 4 5;TRAIN 2 0 11;BUILD MINE 1 0;BUILD TOWER 2 2"
    );
    assert!(!answer_line(&[Command::Wait, Command::Wait]).ends_with(';'));
}

#[test]
fn commands_have_the_texts_of_the_protocol() {
    assert_eq!(Command::Move { id: 3, x: 4, y: 5 }.to_text(), "MOVE 3 4 5");
    assert_eq!(
        Command::Train {
            level: 1,
            x: 1,
            y: 0
        }
        .to_text(),
        "TRAIN 1 1 0"
    );
    assert_eq!(
        Command::BuildMine { x: 9, y: 8 }.to_text(),
        "BUILD MINE 9 8"
    );
    assert_eq!(
        Command::BuildTower { x: 7, y: 6 }.to_text(),
        "BUILD TOWER 7 6"
    );
    assert_eq!(Command::Wait.to_text(), "WAIT");
}

#[test]
fn read_rebuilds_the_state_a_referee_sent_to_either_seat() {
    let mut rng = Rng::new(5);
    let mut compared = 0;
    for league in LEAGUES {
        for seed in 0..40 {
            let mut pair = Pair::new(seed, league);
            while pair.referee.outcome().is_none() {
                let seat = pair.state.to_act();
                let read = State::read(
                    &pair.initial(),
                    &pair.referee.turn_input(seat),
                    league,
                    pair.referee.game_turn(),
                )
                .unwrap_or_else(|error| panic!("{seed} {league:?}: {error}"));
                assert_same_position(&read, &pair.state, &format!("{seed} {league:?}"));
                compared += 1;
                pair.step(&mut rng);
            }
        }
    }
    assert!(compared > 4_000);
}

#[test]
fn a_state_read_plays_on_like_the_original() {
    let mut rng = Rng::new(6);
    for seed in 0..30 {
        let mut pair = Pair::new(seed, League::Bronze);
        for _ in 0..40 {
            pair.step(&mut rng);
        }
        if pair.referee.outcome().is_some() {
            continue;
        }
        let mut read = State::read(
            &pair.initial(),
            &pair.referee.turn_input(pair.state.to_act()),
            League::Bronze,
            pair.referee.game_turn(),
        )
        .unwrap();
        // The same commands on both give the same inputs (ids differ only in
        // the counter, which neither input shows).
        let mut commands = Vec::new();
        for _ in 0..20 {
            read.random_turn(&mut rng, &mut commands);
            pair.state.play_turn(&commands);
            read.play_turn(&commands);
            for seat in 0..2 {
                assert_eq!(read.turn_input(seat), pair.state.turn_input(seat), "{seed}");
            }
            if read.outcome().is_some() {
                break;
            }
        }
        assert_eq!(read.outcome(), pair.state.outcome(), "{seed}");
    }
}

fn valid_turn_input() -> (String, String) {
    let mut rng = Rng::new(9);
    let mut pair = Pair::new(3, League::Bronze);
    for _ in 0..30 {
        pair.step(&mut rng);
    }
    (pair.initial(), pair.referee.turn_input(pair.state.to_act()))
}

#[test]
fn truncated_or_damaged_text_is_an_error_and_never_a_panic() {
    let (initial, turn) = valid_turn_input();
    let read = |initial: &str, turn: &str| State::read(initial, turn, League::Bronze, 1);
    assert!(read(&initial, &turn).is_ok());

    // Every whole-line prefix is missing something.
    let lines: Vec<&str> = turn.lines().collect();
    for count in 0..lines.len() {
        let prefix = lines[..count].join("\n");
        assert!(read(&initial, &prefix).is_err(), "{count} lines");
    }
    let initial_lines: Vec<&str> = initial.lines().collect();
    for count in 0..initial_lines.len() {
        let prefix = initial_lines[..count].join("\n");
        assert!(read(&prefix, &turn).is_err(), "{count} initial lines");
    }

    // Every byte prefix, and every byte replaced by garbage, is an error or
    // a state, not a panic.
    for end in 0..=turn.len() {
        let _ = read(&initial, &turn[..end]);
    }
    for end in 0..=initial.len() {
        let _ = read(&initial[..end], &turn);
    }
    for at in 0..turn.len() {
        for garbage in ["-", "9", "x", " ", "\n", "é"] {
            let mut damaged = turn.clone();
            damaged.replace_range(at..=at, garbage);
            let _ = read(&initial, &damaged);
        }
    }
}

#[test]
fn read_names_what_is_wrong() {
    let (initial, turn) = valid_turn_input();
    let read = |initial: &str, turn: &str| State::read(initial, turn, League::Bronze, 1);
    assert!(read("", &turn).unwrap_err().contains("mine spot count"));
    assert!(read(&initial, "").unwrap_err().contains("gold"));
    assert!(read("7\n1 1\n", &turn).unwrap_err().contains("mine spot"));
    assert!(read("1\n12 0\n", &turn)
        .unwrap_err()
        .contains("off the map"));
    assert!(read("1\n-1 0\n", &turn)
        .unwrap_err()
        .contains("off the map"));
    assert!(read("1\n1 x\n", &turn)
        .unwrap_err()
        .contains("not an integer"));
    assert!(read("1000\n", &turn).unwrap_err().contains("out of range"));
    assert!(read(&format!("{initial}1 1\n"), &turn)
        .unwrap_err()
        .contains("extra"));
    assert!(read(&initial, &format!("{turn}0\n"))
        .unwrap_err()
        .contains("extra"));
    assert!(read(&initial, &turn.replacen('\n', " 1\n", 1)).is_err());
    assert!(read(&initial, &turn.replacen('O', "?", 1)).is_err());
    // A row is shorter.
    assert!(read(&initial, &turn.replacen('.', "", 1)).is_err());
}

/// The text of a turn for red, from a map and lists of buildings and units.
fn turn_text(map: [&str; 12], buildings: &[&str], units: &[&str]) -> String {
    let mut text = String::from("20\n5\n20\n5\n");
    for row in map {
        text.push_str(row);
        text.push('\n');
    }
    text.push_str(&format!("{}\n", buildings.len()));
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

const OPEN_MAP: [&str; 12] = [
    "OO..........",
    "O...........",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
    "............",
    "...........X",
    "..........XX",
    "........XXXX",
];

#[test]
fn read_checks_the_buildings_and_units() {
    let hqs = ["0 0 0 0", "1 0 11 11"];
    let read = |buildings: &[&str], units: &[&str]| {
        State::read(
            "0\n",
            &turn_text(OPEN_MAP, buildings, units),
            League::Bronze,
            1,
        )
    };
    assert!(read(&hqs, &[]).is_ok());
    assert!(read(&["1 0 11 11"], &[]).unwrap_err().contains("own HQ"));
    assert!(read(&["0 0 5 5"], &[]).is_err());
    assert!(read(&["0 0 11 11"], &[]).is_err());
    assert!(read(&["0 0 0 0", "1 0 0 0"], &[]).is_err());
    assert!(read(&["0 0 0 0", "0 3 1 0"], &[]).is_err());
    assert!(read(&["0 0 0 0", "0 1 5 5"], &[]).is_err());
    assert!(read(&["0 0 0 0", "0 1 1 0", "0 2 1 0"], &[]).is_err());
    assert!(read(&["0 0 0 0", "5 1 1 0"], &[]).is_err());
    assert!(read(&["0 0 0 0", "0 1 12 0"], &[]).is_err());
    assert!(read(&hqs, &["0 5 1 1 0"]).is_ok());
    assert!(read(&hqs, &["0 5 1 1 0", "0 6 1 1 0"]).is_err());
    assert!(read(&hqs, &["0 5 1 1 0", "0 5 1 0 1"]).is_err());
    assert!(read(&hqs, &["0 5 0 1 0"]).is_err());
    assert!(read(&hqs, &["0 5 4 1 0"]).is_err());
    assert!(read(&hqs, &["0 5 1 6 6"]).is_err());
    assert!(read(&hqs, &["2 5 1 1 0"]).is_err());
    assert!(read(&hqs, &["0 -5 1 1 0"]).is_err());
    assert!(read(&hqs, &["0 5 1 1"]).is_err());
}

#[test]
fn the_seat_comes_from_the_bots_own_hq() {
    let red = State::read(
        "0\n",
        &turn_text(OPEN_MAP, &["0 0 0 0", "1 0 11 11"], &[]),
        League::Bronze,
        7,
    )
    .unwrap();
    assert_eq!((red.to_act(), red.game_turn()), (0, 7));
    let blue_map = [
        "XX..........",
        "X...........",
        "............",
        "............",
        "............",
        "............",
        "............",
        "............",
        "............",
        "...........O",
        "..........OO",
        "........OOOO",
    ];
    let blue = State::read(
        "0\n",
        &turn_text(blue_map, &["1 0 0 0", "0 0 11 11"], &[]),
        League::Bronze,
        7,
    )
    .unwrap();
    assert_eq!(blue.to_act(), 1);
    assert_eq!(blue.gold(1), 20);
}

#[test]
fn the_unit_counter_follows_the_largest_id_seen() {
    let hqs = ["0 0 0 0", "1 0 11 11"];
    let read = |units: &[&str]| {
        State::read("0\n", &turn_text(OPEN_MAP, &hqs, units), League::Bronze, 1).unwrap()
    };
    assert_eq!(read(&[]).next_unit_id(), 1);
    let mut state = read(&["0 7 1 1 0", "1 42 1 10 11"]);
    assert_eq!(state.next_unit_id(), 43);
    assert!(state.apply(Command::Train {
        level: 1,
        x: 0,
        y: 2
    }));
    assert_eq!(state.next_unit_id(), 44);
    assert!(state.turn_input(0).contains("\n0 43 1 0 2\n"));
    // Ids may be large; the counter does not wrap.
    let state = read(&["0 2147483646 1 1 0"]);
    assert_eq!(state.next_unit_id(), 2_147_483_647);
    assert!(State::read(
        "0\n",
        &turn_text(OPEN_MAP, &hqs, &["0 4294967295 1 1 0"]),
        League::Bronze,
        1
    )
    .is_err());
}

#[test]
fn a_game_with_many_units_has_no_cap_to_overflow() {
    // Red owns and fills every cell it can: units on 130 cells.
    let rows: Vec<String> = (0..12)
        .map(|y| {
            (0..12)
                .map(|x| if x + y >= 20 { 'X' } else { 'O' })
                .collect()
        })
        .collect();
    let map: [&str; 12] = std::array::from_fn(|y| rows[y].as_str());
    let mut units = Vec::new();
    let mut id = 1;
    for y in 0..12 {
        for x in 0..12 {
            if x + y < 20 && (x, y) != (0, 0) {
                units.push(format!("0 {id} 1 {x} {y}"));
                id += 1;
            }
        }
    }
    let unit_lines: Vec<&str> = units.iter().map(String::as_str).collect();
    let text = turn_text(map, &["0 0 0 0", "1 0 11 11"], &unit_lines);
    let mut state = State::read("0\n", &text, League::Bronze, 1).unwrap();
    assert_eq!(state.unit_count(0), units.len());
    assert!(units.len() > 120);
    assert_eq!(state.military_power(0), 20 + 10 * units.len() as i64);
    for seat in 0..2 {
        let again = State::read("0\n", &state.turn_input(seat), League::Bronze, 1);
        assert!(again.is_ok() || seat == 1, "{:?}", again.err());
    }
    // Every unit on the move; no panic, and the counter keeps going.
    let mut rng = Rng::new(1);
    let mut commands = Vec::new();
    for _ in 0..4 {
        state.random_turn(&mut rng, &mut commands);
        state.play_turn(&commands);
    }
    assert!(state.unit_count(0) + state.unit_count(1) <= CELLS);
}

#[test]
fn a_game_that_runs_100_turns_ends_after_blues_hundredth() {
    for league in LEAGUES {
        let mut pair = Pair::new(1, league);
        for turn in 1..=200u32 {
            assert_eq!(pair.state.outcome(), None, "{league:?} turn {turn}");
            assert_eq!(pair.state.game_turn(), turn.div_ceil(2), "{league:?}");
            let answer = Answer {
                seat: pair.state.to_act(),
                lines: vec!["WAIT".to_string()],
            };
            pair.referee.play(&[answer]).unwrap();
            pair.state.play_turn(&[Command::Wait]);
        }
        let expected = pair.referee.outcome().map(|outcome| match outcome {
            RefereeOutcome::Win(seat) => Outcome::Win(seat),
            RefereeOutcome::Draw => Outcome::Draw,
        });
        assert!(expected.is_some());
        assert_eq!(pair.state.outcome(), expected, "{league:?}");
        assert_eq!(pair.state.game_turn(), 100);
        // Nothing happens once the game is over.
        let before = pair.state.turn_input(0);
        pair.state.play_turn(&[Command::Train {
            level: 1,
            x: 1,
            y: 0,
        }]);
        pair.state.end_turn();
        assert!(!pair.state.apply(Command::Train {
            level: 1,
            x: 1,
            y: 0
        }));
        assert_eq!(pair.state.turn_input(0), before);
    }
}

#[test]
fn equal_military_power_after_100_turns_is_a_draw() {
    // Both HQs walled in by void cells: nothing changes, and blue's extra
    // cell is the only difference, so make both equal by removing it.
    let mut void = [true; CELLS];
    for cell in [0, CELLS - 1] {
        void[cell] = false;
    }
    let mut state = State::new(&void, &[], League::Bronze);
    for _ in 0..200 {
        state.play_turn(&[]);
    }
    assert_eq!(state.outcome(), Some(Outcome::Draw));
    assert_eq!(state.military_power(0), state.military_power(1));
}

#[test]
fn a_wipe_out_kills_the_army_and_keeps_the_income_shown() {
    // Blue holds six cells, 60 gold of upkeep and no gold: at its start of
    // turn its income is 6 - 60, its gold would be below 0, and its units die.
    let map = [
        "OO..........",
        "O...........",
        "............",
        "............",
        "............",
        "............",
        "............",
        "............",
        "............",
        ".........XXX",
        ".........XXX",
        ".........XXX",
    ];
    let mut text = turn_text(
        map,
        &["0 0 0 0", "1 0 11 11"],
        &["1 1 3 11 10", "1 2 3 10 11", "1 3 3 10 10"],
    );
    text = text.replacen("20\n5\n20\n5\n", "20\n5\n0\n-54\n", 1);
    let mut state = State::read("0\n", &text, League::Bronze, 1).unwrap();
    assert_eq!(state.military_power(1), 90);
    state.play_turn(&[]);
    assert_eq!(state.to_act(), 1);
    assert_eq!(state.unit_count(1), 0);
    assert_eq!(state.gold(1), 0);
    assert_eq!(state.income(1), 9 - 60);
    assert_eq!(state.military_power(1), 0);
    let input = state.turn_input(1);
    assert!(input.starts_with("0\n-51\n"), "{input}");
    assert!(input.ends_with("\n0\n"), "{input}");
}

#[test]
fn a_capture_that_cuts_territory_kills_the_units_there_at_their_owners_turn() {
    // Blue holds row 11 from x = 8, with a unit at its far end. Red's unit
    // takes (10, 11), which cuts (9, 11) and (8, 11) from the HQ.
    let map = [
        "OO..........",
        "O...........",
        "............",
        "............",
        "............",
        "............",
        "............",
        "............",
        "............",
        "............",
        "..........o.",
        "........XXXX",
    ];
    let text = turn_text(
        map,
        &["0 0 0 0", "1 0 11 11"],
        &["0 1 1 10 10", "1 2 1 8 11"],
    );
    let mut state = State::read("0\n", &text, League::Bronze, 1).unwrap();
    assert_eq!(state.unit_count(1), 1);
    assert!(state.apply(Command::Move {
        id: 1,
        x: 10,
        y: 11
    }));
    // The cut takes effect at once, the unit dies at blue's start of turn.
    assert_eq!(state.unit_count(1), 1);
    assert_eq!(state.active(1).count(), 1);
    state.end_turn();
    assert_eq!(state.to_act(), 1);
    assert_eq!(state.unit_count(1), 0);
    assert_eq!(state.income(1), 1);
}

#[test]
fn a_map_without_a_path_between_cells_still_plays() {
    // Only the two corner blocks are open: no path between the HQs.
    let mut state = state_with_voids(&[], League::Bronze);
    assert_eq!(state.map().distance(0, CELLS - 1), crate::map::NONE);
    let mut rng = Rng::new(3);
    let mut commands = Vec::new();
    for _ in 0..50 {
        state.random_turn(&mut rng, &mut commands);
        let mut replay = state.clone();
        for &command in &commands {
            assert!(replay.apply(command));
        }
        state.play_turn(&commands);
    }
    assert_eq!(state.outcome(), None);
}

#[test]
fn a_unit_does_not_move_toward_a_cell_it_has_no_path_to() {
    let mut state = state_with_voids(&[], League::Bronze);
    assert!(state.apply(Command::Train {
        level: 1,
        x: 1,
        y: 0
    }));
    state.end_turn();
    state.play_turn(&[]);
    assert_eq!(state.to_act(), 0);
    assert!(!state.apply(Command::Move {
        id: 1,
        x: 11,
        y: 11
    }));
    // A cell in its own block is reachable.
    assert!(state.apply(Command::Move { id: 1, x: 2, y: 2 }));
}

#[test]
fn an_hq_with_no_open_neighbour_has_no_command() {
    let mut void = [true; CELLS];
    for cell in [0, CELLS - 1] {
        void[cell] = false;
    }
    let mut state = State::new(&void, &[], League::Bronze);
    let mut rng = Rng::new(4);
    let mut commands = vec![Command::Wait];
    for _ in 0..10 {
        state.random_turn(&mut rng, &mut commands);
        assert!(commands.is_empty());
        assert_eq!(answer_line(&commands), "WAIT");
        state.play_turn(&commands);
    }
    assert!(!state.apply(Command::Train {
        level: 1,
        x: 1,
        y: 0
    }));
    // Both have 20 gold and an HQ, and earn 1 a turn: equal power.
    assert_eq!(state.random_playout(&mut rng), Outcome::Draw);
}

#[test]
fn a_command_that_fails_changes_nothing() {
    let mut rng = Rng::new(8);
    let mut failures = 0;
    for league in LEAGUES {
        for seed in 0..20 {
            let mut state = Pair::new(seed, league).state;
            let mut commands = Vec::new();
            while state.outcome().is_none() {
                state.random_turn(&mut rng, &mut commands);
                state.play_turn(&commands);
                for _ in 0..8 {
                    let command = match rng.below(4) {
                        0 => Command::Move {
                            id: rng.below(u64::from(state.next_unit_id()) + 2) as u32,
                            x: rng.below(14) as u8,
                            y: rng.below(14) as u8,
                        },
                        1 => Command::Train {
                            level: rng.below(5) as u8,
                            x: rng.below(14) as u8,
                            y: rng.below(14) as u8,
                        },
                        2 => Command::BuildMine {
                            x: rng.below(14) as u8,
                            y: rng.below(14) as u8,
                        },
                        _ => Command::BuildTower {
                            x: rng.below(14) as u8,
                            y: rng.below(14) as u8,
                        },
                    };
                    let before = state.clone();
                    if !state.apply(command) {
                        failures += 1;
                        for seat in 0..2 {
                            assert_eq!(state.turn_input(seat), before.turn_input(seat));
                        }
                        assert_eq!(state.movable, before.movable);
                        assert_eq!(state.outcome(), before.outcome());
                    } else {
                        state = before;
                    }
                }
            }
        }
    }
    assert!(failures > 1_000);
}

#[test]
fn random_turns_only_hold_commands_that_succeed() {
    let mut rng = Rng::new(10);
    let mut commands = Vec::new();
    let mut kinds = [0u32; 4];
    for league in LEAGUES {
        for seed in 0..30 {
            let mut state = Pair::new(seed, league).state;
            while state.outcome().is_none() {
                state.random_turn(&mut rng, &mut commands);
                let mut replay = state.clone();
                for &command in &commands {
                    assert!(replay.apply(command), "{seed} {league:?} {command:?}");
                    kinds[match command {
                        Command::Move { .. } => 0,
                        Command::Train { .. } => 1,
                        Command::BuildMine { .. } => 2,
                        Command::BuildTower { .. } => 3,
                        Command::Wait => unreachable!("random_turn never waits"),
                    }] += 1;
                }
                state.play_turn(&commands);
            }
        }
    }
    assert!(kinds.iter().all(|&count| count > 0), "{kinds:?}");
}

#[test]
fn an_unknown_or_foreign_move_is_dropped_but_a_trained_unit_cannot_move() {
    let text = turn_text(
        OPEN_MAP,
        &["0 0 0 0", "1 0 11 11"],
        &["0 5 1 1 0", "1 6 1 10 11"],
    );
    let mut state = State::read("0\n", &text, League::Bronze, 1).unwrap();
    let before = state.turn_input(0);
    // The opponent's unit, an unknown id, and the id the next unit will get.
    state.play_turn(&[
        Command::Move { id: 6, x: 9, y: 11 },
        Command::Move { id: 99, x: 5, y: 5 },
        Command::Train {
            level: 1,
            x: 0,
            y: 2,
        },
        Command::Move { id: 7, x: 5, y: 5 },
    ]);
    let after = state.turn_input(0);
    assert_ne!(before, after);
    assert!(after.contains("\n0 7 1 0 2\n"), "{after}");
    assert!(after.contains("\n1 6 1 10 11\n"), "{after}");
}
