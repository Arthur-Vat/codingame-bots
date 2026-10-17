//! The policy as each league's bot plays it, against the reference referee,
//! fed with the referee's text exactly as CodinGame would send it.
//!
//! Over `SEEDS` maps, in each seat, the bot of a league must beat an opponent
//! that always waits every time, beat a random opponent in at least 95% of the
//! games, never send an answer the referee does not recognise, never send a
//! command that the state it sees refuses, and answer in under 20 ms in a
//! release build. The numbers are printed (`--nocapture`).

use std::time::{Duration, Instant};

use cg_arena::referee::{Answer, Outcome, Referee};
use cg_core::rng::Rng;
use ice_and_fire_engine::{answer_line, Command, League, State};
use ice_and_fire_policy::{answer, greedy_turn};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::{IceAndFireReferee, League as RefereeLeague};

/// Maps per seat and league. A debug build is much slower, so it plays fewer.
const SEEDS: u64 = if cfg!(debug_assertions) { 20 } else { 200 };

/// The time a turn must stay under (CodinGame allows 50 ms).
const SLOW: Duration = Duration::from_millis(20);

const LEAGUES: [League; 4] = [League::Wood3, League::Wood2, League::Wood1, League::Bronze];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Opponent {
    Wait,
    Random,
    Policy,
}

#[derive(Default)]
struct Report {
    /// The bot's seat won.
    won: bool,
    /// An answer of the bot that the referee rejected.
    invalid: Option<String>,
    /// Commands of the bot that its own state refused.
    refused: usize,
    slowest: Duration,
}

fn referee_league(league: League) -> RefereeLeague {
    match league {
        League::Wood3 => RefereeLeague::Wood3,
        League::Wood2 => RefereeLeague::Wood2,
        League::Wood1 => RefereeLeague::Wood1,
        League::Bronze => RefereeLeague::Bronze,
    }
}

/// Plays one game; the bot sits in `bot_seat`, the other seat is `opponent`
/// (the policy again, for self-play).
fn play_game(seed: u64, league: League, bot_seat: usize, opponent: Opponent) -> Report {
    let mut referee = IceAndFireReferee::from_map(
        generate(seed, referee_league(league)),
        referee_league(league),
    );
    let initial = referee.initial_input(0);
    let mut rng = Rng::new(seed ^ 0x5eed);
    let mut report = Report::default();
    let mut commands = Vec::new();
    while referee.outcome().is_none() {
        let seat = referee.players_to_act()[0];
        let text = referee.turn_input(seat);
        let turn = referee.game_turn();
        let line = if seat == bot_seat || opponent == Opponent::Policy {
            let start = Instant::now();
            let line = answer(&initial, &text, league, turn);
            if seat == bot_seat {
                let mut time = start.elapsed();
                // Tests run on many threads of a shared machine, so a turn
                // that looks slow is timed again: a thread descheduled in the
                // middle of one turn says nothing about the policy.
                for _ in 0..5 {
                    if time < SLOW {
                        break;
                    }
                    let again = Instant::now();
                    answer(&initial, &text, league, turn);
                    time = time.min(again.elapsed());
                }
                report.slowest = report.slowest.max(time);
                report.refused += refused(&initial, &text, league, turn);
            }
            line
        } else if opponent == Opponent::Wait {
            "WAIT".to_string()
        } else {
            let state = State::read(&initial, &text, league, turn).expect("the referee's text");
            state.random_turn(&mut rng, &mut commands);
            answer_line(&commands)
        };
        let answers = [Answer {
            seat,
            lines: vec![line],
        }];
        if let Err(err) = referee.play(&answers) {
            if err.seat == bot_seat || opponent == Opponent::Policy {
                report.invalid = Some(err.reason);
            }
            break;
        }
    }
    report.won = referee.outcome() == Some(Outcome::Win(bot_seat));
    report
}

/// How many commands of the policy's turn the state it plays on refuses.
fn refused(initial: &str, text: &str, league: League, turn: u32) -> usize {
    let mut state = State::read(initial, text, league, turn).expect("the referee's text");
    let mut commands: Vec<Command> = Vec::new();
    greedy_turn(&state, &mut commands);
    commands
        .into_iter()
        .filter(|&command| !state.apply(command))
        .count()
}

/// Plays `SEEDS` maps in each seat and returns the number of wins and games.
fn campaign(league: League, opponent: Opponent) -> (u64, u64, Duration) {
    let (mut wins, mut games, mut slowest) = (0, 0, Duration::ZERO);
    for seed in 0..SEEDS {
        for bot_seat in 0..2 {
            let report = play_game(seed, league, bot_seat, opponent);
            assert_eq!(
                report.invalid, None,
                "{league:?} seed {seed} seat {bot_seat}: unrecognised answer"
            );
            assert_eq!(
                report.refused, 0,
                "{league:?} seed {seed} seat {bot_seat}: refused commands"
            );
            wins += u64::from(report.won);
            games += 1;
            slowest = slowest.max(report.slowest);
        }
    }
    (wins, games, slowest)
}

fn beats_wait(league: League) {
    let (wins, games, _) = campaign(league, Opponent::Wait);
    eprintln!("{league:?} against WAIT: {wins}/{games}");
    assert_eq!(wins, games, "{league:?} must beat WAIT every time");
}

fn beats_random(league: League) {
    let (wins, games, _) = campaign(league, Opponent::Random);
    eprintln!("{league:?} against random: {wins}/{games}");
    assert!(
        wins * 100 >= games * 95,
        "{league:?} won only {wins}/{games} against random"
    );
}

#[test]
fn wood3_beats_wait() {
    beats_wait(League::Wood3);
}

#[test]
fn wood2_beats_wait() {
    beats_wait(League::Wood2);
}

#[test]
fn wood1_beats_wait() {
    beats_wait(League::Wood1);
}

#[test]
fn bronze_beats_wait() {
    beats_wait(League::Bronze);
}

#[test]
fn wood3_beats_random() {
    beats_random(League::Wood3);
}

#[test]
fn wood2_beats_random() {
    beats_random(League::Wood2);
}

#[test]
fn wood1_beats_random() {
    beats_random(League::Wood1);
}

#[test]
fn bronze_beats_random() {
    beats_random(League::Bronze);
}

/// Games of the policy against itself: every answer is recognised and every
/// command accepted, in every league.
#[test]
fn self_play_is_valid() {
    for league in LEAGUES {
        let (wins, games, _) = campaign(league, Opponent::Policy);
        eprintln!("{league:?} against itself: won by the bot's seat {wins}/{games}");
    }
}

/// A turn takes under 20 ms (CodinGame allows 50). Debug builds are too slow
/// to say.
#[test]
#[cfg_attr(
    debug_assertions,
    ignore = "timing is only meaningful in a release build"
)]
fn turns_take_under_20_ms() {
    for league in LEAGUES {
        let (_, _, slowest) = campaign(league, Opponent::Random);
        eprintln!("{league:?} slowest turn: {slowest:?}");
        assert!(slowest < SLOW, "{league:?}: {slowest:?}");
    }
}

#[test]
fn unreadable_input_is_answered_with_wait() {
    assert_eq!(answer("0\n", "not a turn", League::Wood3, 1), "WAIT");
    assert_eq!(answer("", "", League::Bronze, 1), "WAIT");
}

#[test]
fn the_io_loop_answers_each_turn_and_stops_at_the_end() {
    let referee =
        IceAndFireReferee::from_map(generate(3, RefereeLeague::Bronze), RefereeLeague::Bronze);
    let turn = referee.turn_input(0);
    let input = format!("{}{turn}{turn}", referee.initial_input(0));
    let mut output = Vec::new();
    ice_and_fire_policy::io::play(League::Bronze, input.as_bytes(), &mut output).unwrap();
    let output = String::from_utf8(output).unwrap();
    assert_eq!(output.lines().count(), 2);
    for line in output.lines() {
        assert!(!line.ends_with(';') && !line.is_empty(), "{line:?}");
    }

    // The same turn cut short is an error, not a crash.
    let cut = &input[..input.len() - 10];
    let mut output = Vec::new();
    assert!(ice_and_fire_policy::io::play(League::Bronze, cut.as_bytes(), &mut output).is_err());
}
