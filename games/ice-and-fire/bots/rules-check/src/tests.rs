use cg_arena::referee::{Answer, GameSetup, Outcome, Referee};
use cg_core::rng::Rng;
use ice_and_fire_engine::{answer_line, League, State, CELLS, SIZE};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::{IceAndFireReferee, League as RefereeLeague};

use super::*;
use crate::checker::{check_prediction, check_received, Prediction, Report};
use crate::session::Session;

/// Games per opponent, as required: at least 300 seeds in Bronze.
const SEEDS: u64 = 300;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Opponent {
    Wait,
    Random,
}

fn referee_for(seed: u64) -> IceAndFireReferee {
    IceAndFireReferee::new(&GameSetup {
        seed,
        opening_plies: 0,
    })
}

/// What a game of the bot against the referee produced.
#[derive(Default)]
struct Played {
    turns: u32,
    checks: u32,
    log: Vec<String>,
    differences: Vec<String>,
    outcome: Option<Outcome>,
    wipe_outs: u32,
}

impl Played {
    fn last_totals(&self) -> &str {
        self.log
            .iter()
            .rev()
            .find(|line| line.contains(" checks, "))
            .map_or("", String::as_str)
    }
}

/// Plays the bot in `bot_seat` against `opponent` on the referee, with the
/// referee's exact text as input.
fn play_game(seed: u64, bot_seat: usize, opponent: Opponent) -> Played {
    let mut referee = referee_for(seed);
    let initial = referee.initial_input(bot_seat);
    let mut session = Session::new(initial.clone(), seed ^ 0x5eed);
    let mut rng = Rng::new(seed ^ 0xc0ffee);
    let mut played = Played::default();
    let mut had_units = false;
    while let Some(&seat) = referee.players_to_act().first() {
        let input = referee.turn_input(seat);
        let line = if seat == bot_seat {
            let state = State::read(&initial, &input, League::Bronze, 1).unwrap();
            if had_units && state.unit_count(seat) == 0 && state.gold(seat) == 0 {
                played.wipe_outs += 1;
            }
            had_units = state.unit_count(seat) > 0;
            let turn = session.play_turn(&input);
            played.turns += 1;
            for line in turn.log {
                if line.contains("DIFFERENCE") {
                    played.differences.push(line.clone());
                }
                played.log.push(line);
            }
            turn.answer
        } else if opponent == Opponent::Random {
            let state =
                State::read(&referee.initial_input(seat), &input, League::Bronze, 1).unwrap();
            let mut commands = Vec::new();
            state.random_turn(&mut rng, &mut commands);
            answer_line(&commands)
        } else {
            "WAIT".to_string()
        };
        referee
            .play(&[Answer {
                seat,
                lines: vec![line.clone()],
            }])
            .unwrap_or_else(|err| panic!("seed {seed}: the referee rejects {line:?}: {err:?}"));
    }
    played.outcome = referee.outcome();
    played.checks = session_checks(&played);
    played
}

/// The checks made, from the last totals line.
fn session_checks(played: &Played) -> u32 {
    played
        .last_totals()
        .split(", ")
        .nth(1)
        .and_then(|part| part.split(' ').next())
        .and_then(|number| number.parse().ok())
        .unwrap_or(0)
}

/// Plays all the seeds against one kind of opponent and asserts there is no
/// difference. Returns the games.
fn no_difference_against(opponent: Opponent) -> Vec<(usize, Played)> {
    let mut games = Vec::new();
    for seed in 0..SEEDS {
        let bot_seat = (seed % 2) as usize;
        let played = play_game(seed, bot_seat, opponent);
        assert!(
            played.differences.is_empty(),
            "seed {seed}, bot {}: {:#?}",
            bot_seat,
            played.differences
        );
        assert!(played.turns >= 1 && played.checks > 10 * played.turns);
        games.push((bot_seat, played));
    }
    games
}

/// The result the bot's log expects, checked against the referee's.
fn assert_expectations(games: &[(usize, Played)]) -> (u32, u32) {
    let (mut captures, mut powers) = (0, 0);
    for (seat, played) in games {
        let outcome = played.outcome.expect("the game is over");
        let captured = played.log.iter().find(|l| l.contains("EXPECT win"));
        if let Some(line) = captured {
            captures += 1;
            let winner = if line.contains("win for red") { 0 } else { 1 };
            assert_eq!(outcome, Outcome::Win(winner), "{line}");
            assert!(line.ends_with("(HQ captured)"), "{line}");
        }
        if let Some(line) = played.log.iter().find(|l| l.contains("military power red")) {
            // Turn 100: the bot saw its last turn.
            if *seat == 1 && captured.is_none() {
                powers += 1;
                let expected = match outcome {
                    Outcome::Win(0) => "win for red",
                    Outcome::Win(_) => "win for blue",
                    Outcome::Draw => "draw",
                };
                assert!(
                    line.contains(&format!("expected result: {expected}")),
                    "{line}"
                );
            }
        }
    }
    (captures, powers)
}

#[test]
fn agrees_with_the_referee_against_a_waiting_opponent() {
    let games = no_difference_against(Opponent::Wait);
    // Against a player that waits, the random bot captures the HQ in every
    // game, so few games reach turn 100.
    let (captures, _) = assert_expectations(&games);
    assert!(captures > 0, "no HQ capture was expected");
}

#[test]
fn agrees_with_the_referee_against_a_random_opponent() {
    let games = no_difference_against(Opponent::Random);
    let (captures, powers) = assert_expectations(&games);
    assert!(
        captures > 0 && powers > 0,
        "{captures} captures, {powers} power results"
    );
    let wipe_outs: u32 = games.iter().map(|(_, played)| played.wipe_outs).sum();
    println!(
        "{} games, {} turns, {} checks, {wipe_outs} wipe-outs, {captures} captures",
        games.len(),
        games.iter().map(|(_, p)| p.turns).sum::<u32>(),
        games.iter().map(|(_, p)| p.checks).sum::<u32>()
    );
}

#[test]
fn the_last_line_of_a_game_sums_it_up() {
    let played = play_game(3, 0, Opponent::Wait);
    let last = played.log.last().unwrap();
    assert_eq!(
        *last,
        format!(
            "rules-check: turn {}, {} checks, 0 differences",
            played.turns, played.checks
        )
    );
}

// --- The checker catches errors ---

/// Plays turns `1..=turns` of a game against a random opponent and returns
/// the session and the input of the next turn, if the game lasts that long.
fn until(seed: u64, bot_seat: usize, turns: u32) -> Option<(Session, String)> {
    let mut referee = referee_for(seed);
    let initial = referee.initial_input(bot_seat);
    let mut session = Session::new(initial.clone(), seed);
    let mut rng = Rng::new(seed ^ 0xfeed);
    let mut seen = 0;
    while let Some(&seat) = referee.players_to_act().first() {
        let input = referee.turn_input(seat);
        let line = if seat == bot_seat {
            if seen == turns {
                return Some((session, input));
            }
            seen += 1;
            session.play_turn(&input).answer
        } else {
            let state = State::read(&initial, &input, League::Bronze, 1).unwrap();
            let mut commands = Vec::new();
            state.random_turn(&mut rng, &mut commands);
            answer_line(&commands)
        };
        referee
            .play(&[Answer {
                seat,
                lines: vec![line],
            }])
            .expect("the answers are valid");
    }
    None
}

fn differences(log: &[String]) -> Vec<&String> {
    log.iter().filter(|l| l.contains("DIFFERENCE")).collect()
}

/// A doctored input must make the bot report a difference, while the same
/// input undoctored makes none. `doctor` returns `None` when the input has
/// nothing to doctor; the first of the seeds where it applies is used.
fn catches(turns: u32, doctor: impl Fn(&[String]) -> Option<Vec<String>>) {
    for seed in 0..40 {
        for seat in 0..2 {
            let Some((mut session, input)) = until(seed, seat, turns) else {
                continue;
            };
            let lines: Vec<String> = input.lines().map(str::to_string).collect();
            let Some(doctored) = doctor(&lines) else {
                continue;
            };
            let doctored = doctored.join("\n") + "\n";
            assert_ne!(doctored, input);
            let log = session.play_turn(&doctored).log;
            assert!(
                !differences(&log).is_empty(),
                "seed {seed}, seat {seat}: no difference reported\n{doctored}"
            );

            let (mut clean, input) = until(seed, seat, turns).unwrap();
            let log = clean.play_turn(&input).log;
            assert!(differences(&log).is_empty(), "{log:#?}");
            return;
        }
    }
    panic!("no input could be doctored");
}

/// Index of the first unit line (after the unit count) and the unit lines.
fn unit_section(lines: &[String]) -> (usize, usize) {
    let buildings: usize = lines[16].trim().parse().unwrap();
    let count_at = 17 + buildings;
    (count_at, lines[count_at].trim().parse().unwrap())
}

fn number(line: &str) -> i64 {
    line.trim().parse().unwrap()
}

#[test]
fn catches_a_wrong_income() {
    catches(5, |lines| {
        let mut lines = lines.to_vec();
        lines[1] = (number(&lines[1]) + 1).to_string();
        Some(lines)
    });
    catches(5, |lines| {
        let mut lines = lines.to_vec();
        lines[3] = (number(&lines[3]) - 1).to_string();
        Some(lines)
    });
}

#[test]
fn catches_the_gold_off_by_one() {
    for change in [1, -1] {
        catches(5, move |lines| {
            let mut lines = lines.to_vec();
            lines[0] = (number(&lines[0]) + change).to_string();
            Some(lines)
        });
    }
}

#[test]
fn catches_an_extra_cell_of_the_bot() {
    catches(5, |lines| {
        let mut lines = lines.to_vec();
        let (row, column) = (4..16).find_map(|row| {
            lines[row]
                .find('.')
                .filter(|&column| row > 8 || column > 6)
                .map(|column| (row, column))
        })?;
        lines[row].replace_range(column..=column, "O");
        Some(lines)
    });
}

#[test]
fn catches_a_cell_of_the_bot_that_vanishes() {
    catches(8, |lines| {
        let mut lines = lines.to_vec();
        // An own cell that is not an HQ.
        let (row, column) = (4..16).rev().find_map(|row| {
            let column = lines[row].rfind('O')?;
            ((row, column) != (4, 0) && (row, column) != (15, 11)).then_some((row, column))
        })?;
        lines[row].replace_range(column..=column, ".");
        Some(lines)
    });
}

#[test]
fn catches_a_unit_moved_without_a_command() {
    catches(8, |lines| {
        let mut lines = lines.to_vec();
        let (count_at, count) = unit_section(&lines);
        let own = (count_at + 1..count_at + 1 + count).find(|&at| lines[at].starts_with("0 "))?;
        let words: Vec<i64> = lines[own]
            .split_whitespace()
            .map(|word| word.parse().unwrap())
            .collect();
        // A free active cell of the bot elsewhere.
        let occupied: Vec<(i64, i64)> = (count_at + 1..count_at + 1 + count)
            .map(|at| {
                let w: Vec<i64> = lines[at]
                    .split_whitespace()
                    .map(|word| word.parse().unwrap())
                    .collect();
                (w[3], w[4])
            })
            .collect();
        let (x, y) = (0..SIZE as i64)
            .flat_map(|y| (0..SIZE as i64).map(move |x| (x, y)))
            .find(|&(x, y)| {
                lines[4 + y as usize].as_bytes()[x as usize] == b'O'
                    && !occupied.contains(&(x, y))
                    && (x, y) != (0, 0)
                    && (x, y) != (11, 11)
            })?;
        lines[own] = format!("0 {} {} {x} {y}", words[1], words[2]);
        Some(lines)
    });
}

#[test]
fn catches_a_unit_that_vanishes() {
    catches(8, |lines| {
        let mut lines = lines.to_vec();
        let (count_at, count) = unit_section(&lines);
        let own = (count_at + 1..count_at + 1 + count).find(|&at| lines[at].starts_with("0 "))?;
        lines.remove(own);
        lines[count_at] = (count - 1).to_string();
        Some(lines)
    });
}

#[test]
fn catches_an_opponent_unit_with_an_old_id() {
    catches(8, |lines| {
        let mut lines = lines.to_vec();
        let (count_at, count) = unit_section(&lines);
        let foe = (count_at + 1..count_at + 1 + count).find(|&at| lines[at].starts_with("1 "))?;
        let words: Vec<&str> = lines[foe].split_whitespace().collect();
        // Id 0 is below every id in the game, so the unit cannot be new.
        lines[foe] = format!("1 0 {} {} {}", words[2], words[3], words[4]);
        Some(lines)
    });
}

#[test]
fn reports_input_that_does_not_parse_and_waits() {
    let (mut session, input) = until(1, 0, 3).unwrap();
    let garbage = input.replace('O', "Z");
    let turn = session.play_turn(&garbage);
    assert_eq!(turn.answer, "WAIT");
    assert!(
        differences(&turn.log)[0].contains("the input does not parse"),
        "{:?}",
        turn.log
    );
    // The bot keeps going, without a prediction to check against.
    let (_, good) = until(1, 0, 4).unwrap();
    let turn = session.play_turn(&good);
    assert!(differences(&turn.log).is_empty(), "{:?}", turn.log);
    assert_ne!(turn.answer, "");
}

/// A turn's input as red, for hand-made scenarios: the bot owns `own` cells,
/// blue its HQ and (11, 10), and `units` are lines `owner id level x y`.
fn red_input(gold: i64, income: i64, own: &[(usize, usize)], units: &[&str]) -> String {
    let mut text = format!("{gold}\n{income}\n20\n2\n");
    for y in 0..SIZE {
        for x in 0..SIZE {
            text.push(if own.contains(&(x, y)) {
                'O'
            } else if (x, y) == (11, 11) || (x, y) == (11, 10) {
                'X'
            } else {
                '.'
            });
        }
        text.push('\n');
    }
    text.push_str("2\n0 0 0 0\n1 0 11 11\n");
    text.push_str(&format!("{}\n", units.len()));
    for unit in units {
        text.push_str(&format!("{unit}\n"));
    }
    text
}

fn read_red(input: &str) -> State {
    State::read("0\n", input, League::Bronze, 2).unwrap()
}

/// Checks `now` against a prediction of the bot's state after its commands.
fn compare(predicted: &str, now: &str) -> Vec<String> {
    let after = read_red(predicted);
    let prediction = Prediction {
        first_new_id: after.next_unit_id(),
        after,
        applied: Vec::new(),
        opponent_ids: Vec::new(),
    };
    let now = read_red(now);
    let mut report = Report::default();
    check_received(&mut report, &now, Some(&prediction));
    check_prediction(&mut report, &now, &prediction);
    report.differences
}

#[test]
fn a_wipe_out_is_not_a_difference() {
    // A level 3 unit costs 20 a turn; two cells earn 2. With 5 gold the bot
    // ends below 0: its gold becomes 0, the unit dies, and the income shown
    // is the one from before (RULES.md, "Start of a player's turn").
    let cells = [(0, 0), (1, 0)];
    let predicted = red_input(5, 1, &cells, &["0 3 3 1 0"]);
    assert_eq!(
        compare(&predicted, &red_input(0, -18, &cells, &[])),
        Vec::<String>::new()
    );
    // Gold that is not 0, or a unit that stays, is a difference.
    assert!(!compare(&predicted, &red_input(2, -18, &cells, &[])).is_empty());
    assert!(!compare(&predicted, &red_input(0, -18, &cells, &["0 3 3 1 0"])).is_empty());
    // Income that the units cannot explain is a difference too.
    assert!(!compare(&predicted, &red_input(0, 7, &cells, &[])).is_empty());
}

// --- The MAP line ---

/// Reads a MAP line back into a seat, the void cells and the spots.
fn parse_map(line: &str) -> (usize, [bool; CELLS], Vec<(usize, usize)>) {
    let words: Vec<&str> = line
        .strip_prefix("rules-check: MAP ")
        .expect("a MAP line")
        .split(' ')
        .collect();
    let seat = match words[0] {
        "red" => 0,
        "blue" => 1,
        other => panic!("seat {other}"),
    };
    assert_eq!(words[1].chars().count(), CELLS);
    let mut void = [false; CELLS];
    for (cell, symbol) in words[1].chars().enumerate() {
        assert!(symbol == '#' || symbol == '.');
        void[cell] = symbol == '#';
    }
    let spots = words[2..]
        .iter()
        .map(|spot| {
            let (x, y) = spot.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    (seat, void, spots)
}

#[test]
fn the_map_line_round_trips_to_the_map() {
    for seed in 0..100 {
        let map = generate(seed, RefereeLeague::Bronze);
        for bot_seat in 0..2 {
            let referee = referee_for(seed);
            // The bot sees the map once, on its first turn.
            let (mut session, input) = until(seed, bot_seat, 0).unwrap();
            let turn = session.play_turn(&input);
            let maps: Vec<&String> = turn.log.iter().filter(|l| l.contains(" MAP ")).collect();
            assert_eq!(maps.len(), 1, "{:?}", turn.log);
            let (seat, void, spots) = parse_map(maps[0]);
            assert_eq!(seat, bot_seat);
            assert_eq!(void, map.void, "seed {seed}");
            assert_eq!(spots, map.mine_spots, "seed {seed}");
            assert!(referee
                .initial_input(0)
                .starts_with(&format!("{}\n", spots.len())));
            // Only the first turn logs the map.
            let next = session.play_turn(&input);
            assert!(next.log.iter().all(|l| !l.contains(" MAP ")));
        }
    }
}

// --- Input and output ---

fn run(strict: bool, input: &str) -> (Result<(), String>, String, String) {
    let (mut out, mut log) = (Vec::new(), Vec::new());
    let result = play(7, strict, input.as_bytes(), &mut out, &mut log);
    (
        result,
        String::from_utf8(out).unwrap(),
        String::from_utf8(log).unwrap(),
    )
}

/// The input of the bot's first two turns as red, as the referee sends it.
fn two_turns() -> String {
    let mut referee = referee_for(4);
    let mut input = referee.initial_input(0);
    // The seed that `run` gives the bot.
    let mut session = Session::new(input.clone(), 7);
    for _ in 0..2 {
        let text = referee.turn_input(0);
        input.push_str(&text);
        let answer = session.play_turn(&text).answer;
        for seat in 0..2 {
            let line = if seat == 0 {
                answer.clone()
            } else {
                "WAIT".to_string()
            };
            referee
                .play(&[Answer {
                    seat,
                    lines: vec![line],
                }])
                .unwrap();
        }
    }
    input
}

#[test]
fn plays_and_logs_every_turn_to_the_end_of_the_input() {
    let (result, out, log) = run(false, &two_turns());
    assert_eq!(result, Ok(()));
    assert_eq!(out.lines().count(), 2);
    assert!(log.contains("rules-check: MAP red "), "{log}");
    assert!(log.contains("rules-check: turn 1, "), "{log}");
    assert!(log.contains(", 0 differences"), "{log}");
    assert!(log
        .lines()
        .last()
        .unwrap()
        .starts_with("rules-check: turn 2, "));
    assert!(!log.contains("DIFFERENCE"), "{log}");
}

#[test]
fn accepts_windows_line_endings() {
    let (result, out, log) = run(false, &two_turns().replace('\n', "\r\n"));
    assert_eq!(result, Ok(()));
    assert_eq!(out.lines().count(), 2);
    assert!(!log.contains("DIFFERENCE"), "{log}");
}

#[test]
fn stops_cleanly_when_the_game_ends() {
    assert_eq!(run(true, "").0, Ok(()));
    assert_eq!(run(true, "0\n").0, Ok(()));
}

#[test]
fn a_difference_is_an_error_only_in_strict_mode() {
    let input = two_turns().replacen("\n21\n1\n20\n2\n", "\n21\n9\n20\n2\n", 1);
    let (result, out, log) = run(false, &input);
    assert_eq!(result, Ok(()));
    assert_eq!(out.lines().count(), 2);
    assert!(
        log.contains("DIFFERENCE turn 1: income of the bot"),
        "{log}"
    );

    let (result, out, log) = run(true, &input);
    assert!(result.unwrap_err().contains("strict mode"));
    assert!(out.is_empty());
    assert!(log.contains("DIFFERENCE"), "{log}");
}

#[test]
fn input_that_stops_mid_turn_is_a_difference() {
    let input = two_turns();
    let cut: String = input.lines().take(30).map(|l| format!("{l}\n")).collect();
    let (result, _, log) = run(false, &cut);
    assert_eq!(result, Ok(()));
    assert!(log.contains("DIFFERENCE"), "{log}");
    assert!(run(true, &cut).0.is_err());
}

#[test]
fn bad_counts_never_make_the_bot_wait_or_panic() {
    let (result, out, log) = run(false, "x\n21\n1\n20\n2\n............\n");
    assert_eq!(result, Ok(()));
    assert!(out.is_empty(), "{out}");
    assert!(log.contains("DIFFERENCE"), "{log}");
    let junk = format!("0\n{}", "5\n".repeat(40));
    let (_, out, log) = run(false, &junk);
    assert!(out.lines().all(|line| line == "WAIT"), "{out}");
    assert!(log.contains("DIFFERENCE"), "{log}");
}

#[test]
fn a_turn_takes_far_less_than_50_ms() {
    let started = std::time::Instant::now();
    let played = play_game(11, 1, Opponent::Random);
    let per_turn = started.elapsed() / (2 * played.turns.max(1));
    assert!(
        per_turn < std::time::Duration::from_millis(20),
        "{per_turn:?}"
    );
}
