use super::*;
use crate::referee::TimeLimits;
use crate::runner::{bot_seed, run_match, MatchOptions};

/// A test game: players alternate, each answering "ok N". The answer
/// "ok 3" wins at once; one that does not start with "ok" is invalid; after
/// `length` turns the game is a draw.
struct Countdown {
    length: u32,
    played: u32,
    winner: Option<usize>,
}

impl Countdown {
    fn new(length: u32) -> Self {
        Countdown {
            length,
            played: 0,
            winner: None,
        }
    }

    fn boxed(length: u32) -> Box<dyn Referee> {
        Box::new(Countdown::new(length))
    }
}

impl Referee for Countdown {
    fn time_limits(&self) -> TimeLimits {
        TimeLimits {
            first_answer: Duration::from_millis(5000),
            later_answers: Duration::from_millis(2000),
        }
    }

    fn initial_input(&self, seat: usize) -> String {
        format!("seat {seat}\n")
    }

    fn players_to_act(&self) -> Vec<usize> {
        vec![(self.played % 2) as usize]
    }

    fn turn_input(&self, _seat: usize) -> String {
        format!("turn {}\n", self.played)
    }

    fn play(&mut self, answers: &[Answer]) -> Result<(), InvalidAnswer> {
        let answer = &answers[0];
        let line = answer.lines[0].as_str();
        if !line.starts_with("ok") {
            return Err(InvalidAnswer {
                seat: answer.seat,
                reason: format!("unknown answer {line:?}"),
            });
        }
        if line == "ok 3" {
            self.winner = Some(answer.seat);
        }
        self.played += 1;
        Ok(())
    }

    fn outcome(&self) -> Option<Outcome> {
        if let Some(seat) = self.winner {
            Some(Outcome::Win(seat))
        } else if self.played >= self.length {
            Some(Outcome::Draw)
        } else {
            None
        }
    }
}

fn shell(name: &str, script: &str) -> BotSpec {
    BotSpec {
        name: name.to_string(),
        program: "sh".to_string(),
        args: vec!["-c".to_string(), script.to_string()],
    }
}

/// Answers "ok N" at its Nth turn: it has state, so replaying it matters.
fn counter(name: &str) -> BotSpec {
    shell(
        name,
        "n=0; while read line; do case \"$line\" in turn*) n=$((n+1)); echo \"ok $n\";; esac; done",
    )
}

const SEED: u64 = 7;
const LIMIT: Duration = Duration::from_secs(5);

fn setup() -> GameSetup {
    GameSetup {
        seed: SEED,
        opening_plies: 0,
    }
}

fn settings(seat: usize, name: &str) -> BotSettings {
    BotSettings {
        seed: bot_seed(SEED, seat, name),
        time_scale: 1.0,
        fixed_iters: None,
        show_stderr: false,
    }
}

/// The game `run_match` plays between two counters: seat 0 wins at turn 4.
fn played_by_the_arena() -> crate::runner::MatchRecord {
    let (a, b) = (counter("a"), counter("b"));
    run_match(
        &mut Countdown::new(10),
        [&a, &b],
        SEED,
        &MatchOptions::default(),
    )
    .expect("bots start")
}

fn without_times(turns: &[Vec<RecordedAnswer>]) -> Vec<Vec<(usize, Vec<String>)>> {
    turns
        .iter()
        .map(|turn| {
            turn.iter()
                .map(|answer| (answer.seat, answer.lines.clone()))
                .collect()
        })
        .collect()
}

#[test]
fn a_live_game_plays_like_run_match() {
    let record = played_by_the_arena();
    assert_eq!(record.winner, Some(0));

    let specs = [counter("a"), counter("b")];
    let mut bots: Vec<LiveBot> = specs
        .iter()
        .enumerate()
        .map(|(seat, spec)| LiveBot::spawn(spec, &settings(seat, &spec.name)).unwrap())
        .collect();
    let mut game = LiveGame::new(Countdown::boxed(10), setup());
    while !game.to_act().is_empty() {
        let mut answers = Vec::new();
        for seat in game.to_act() {
            let (lines, ms) = bots[seat]
                .ask(
                    &game.input_for(seat),
                    game.answer_lines(seat),
                    game.time_limit(seat),
                )
                .unwrap();
            answers.push(RecordedAnswer { seat, lines, ms });
        }
        game.play(answers).unwrap();
    }
    assert_eq!(game.outcome(), Some(Outcome::Win(0)));
    assert_eq!(
        without_times(game.turns()),
        without_times(&record.recorded_turns)
    );
    assert_eq!(bots[0].answers(), 3);
    assert_eq!(bots[1].answers(), 2);
    assert_eq!(game.setup(), setup());
}

#[test]
fn the_first_answer_of_a_seat_has_the_longer_limit_and_the_initial_input() {
    let mut game = LiveGame::new(Countdown::boxed(10), setup());
    assert_eq!(game.to_act(), vec![0]);
    assert_eq!(game.input_for(0), "seat 0\nturn 0\n");
    assert_eq!(game.time_limit(0), Duration::from_millis(5000));
    assert_eq!(game.answer_lines(0), 1);
    let answer = |seat| RecordedAnswer {
        seat,
        lines: vec!["ok 1".to_string()],
        ms: 1.0,
    };
    game.play(vec![answer(0)]).unwrap();
    assert_eq!(game.to_act(), vec![1]);
    assert_eq!(game.input_for(1), "seat 1\nturn 1\n");
    assert_eq!(game.time_limit(1), Duration::from_millis(5000));
    game.play(vec![answer(1)]).unwrap();
    assert_eq!(game.input_for(0), "turn 2\n");
    assert_eq!(game.time_limit(0), Duration::from_millis(2000));
}

#[test]
fn a_finished_game_has_no_one_to_act() {
    let record = played_by_the_arena();
    let game = LiveGame::replay(Countdown::boxed(10), setup(), &record.recorded_turns).unwrap();
    assert_eq!(game.outcome(), Some(Outcome::Win(0)));
    assert!(game.to_act().is_empty());
}

#[test]
fn replaying_a_prefix_gives_the_same_position() {
    let record = played_by_the_arena();
    let turns = &record.recorded_turns;
    assert_eq!(turns.len(), 5);
    let mut original = LiveGame::new(Countdown::boxed(10), setup());
    for k in 0..=turns.len() {
        let replayed = LiveGame::replay(Countdown::boxed(10), setup(), &turns[..k]).unwrap();
        assert_eq!(replayed.turns(), &turns[..k]);
        assert_eq!(replayed.outcome(), original.outcome());
        assert_eq!(replayed.to_act(), original.to_act());
        for seat in 0..2 {
            assert_eq!(replayed.input_for(seat), original.input_for(seat), "{k}");
            assert_eq!(replayed.time_limit(seat), original.time_limit(seat));
            assert_eq!(replayed.answer_lines(seat), original.answer_lines(seat));
        }
        if k < turns.len() {
            original.play(turns[k].clone()).unwrap();
        }
    }
}

#[test]
fn an_invalid_answer_records_nothing_and_ends_the_game() {
    let mut game = LiveGame::new(Countdown::boxed(10), setup());
    let nope = RecordedAnswer {
        seat: 0,
        lines: vec!["nope".to_string()],
        ms: 1.0,
    };
    let invalid = game.play(vec![nope.clone()]).unwrap_err();
    assert_eq!(invalid.seat, 0);
    assert!(game.turns().is_empty());
    assert_eq!(game.failure(), Some(&invalid));
    assert!(game.to_act().is_empty());
    assert_eq!(game.outcome(), None);
    assert_eq!(game.play(vec![nope.clone()]).unwrap_err(), invalid);
    // A replay of the turns before it, from a fresh referee, carries on.
    let mut fresh = LiveGame::replay(Countdown::boxed(10), setup(), game.turns()).unwrap();
    assert_eq!(fresh.to_act(), vec![0]);
    assert!(fresh.play(vec![nope]).is_err());
}

#[test]
fn replaying_invalid_turns_is_an_error() {
    let turn = vec![RecordedAnswer {
        seat: 0,
        lines: vec!["nope".to_string()],
        ms: 1.0,
    }];
    let error = LiveGame::replay(Countdown::boxed(10), setup(), &[turn]).err();
    assert_eq!(
        error.map(|error| (error.turn, error.invalid.seat)),
        Some((0, 0))
    );
}

#[test]
fn replaying_a_record_ending_in_an_invalid_turn_names_that_turn() {
    let ok = |seat| {
        vec![RecordedAnswer {
            seat,
            lines: vec!["ok 1".to_string()],
            ms: 1.0,
        }]
    };
    let nope = vec![RecordedAnswer {
        seat: 0,
        lines: vec!["nope".to_string()],
        ms: 1.0,
    }];
    let turns = vec![ok(0), ok(1), nope];
    let error = LiveGame::replay(Countdown::boxed(10), setup(), &turns)
        .err()
        .expect("the last turn is invalid");
    assert_eq!(error.turn, turns.len() - 1);
    assert!(error.to_string().starts_with("turn 2: "), "{error}");
    let game = LiveGame::replay(Countdown::boxed(10), setup(), &turns[..error.turn]).unwrap();
    assert_eq!(game.turns().len(), 2);
    assert_eq!(game.to_act(), vec![0]);
}

fn answer(seat: usize) -> RecordedAnswer {
    RecordedAnswer {
        seat,
        lines: vec!["ok 1".to_string()],
        ms: 1.0,
    }
}

#[test]
fn answers_from_the_wrong_seat_are_refused_without_playing() {
    let mut game = LiveGame::new(Countdown::boxed(10), setup());
    let invalid = game.play(vec![answer(1)]).unwrap_err();
    assert_eq!(invalid.seat, 1);
    assert!(invalid.reason.contains("seat 1"), "{}", invalid.reason);
    assert!(game.turns().is_empty());
    assert!(game.failure().is_none());
    assert_eq!(game.to_act(), vec![0]);
    // Too many answers are refused too, and the game stays usable.
    let invalid = game.play(vec![answer(0), answer(1)]).unwrap_err();
    assert_eq!(invalid.seat, 1);
    assert!(game.turns().is_empty());
    game.play(vec![answer(0)]).unwrap();
    assert_eq!(game.turns().len(), 1);
    assert_eq!(game.to_act(), vec![1]);
}

#[test]
fn an_empty_turn_is_refused_without_playing() {
    let mut game = LiveGame::new(Countdown::boxed(10), setup());
    let invalid = game.play(Vec::new()).unwrap_err();
    assert_eq!(invalid.seat, 0);
    assert!(game.turns().is_empty());
    assert!(game.failure().is_none());
    game.play(vec![answer(0)]).unwrap();
}

#[test]
fn a_play_after_the_end_is_refused() {
    let record = played_by_the_arena();
    let mut game = LiveGame::replay(Countdown::boxed(10), setup(), &record.recorded_turns).unwrap();
    let turns = game.turns().len();
    let invalid = game.play(vec![answer(1)]).unwrap_err();
    assert_eq!(invalid.seat, 1);
    assert!(invalid.reason.contains("over"), "{}", invalid.reason);
    assert!(game.play(Vec::new()).is_err());
    assert_eq!(game.turns().len(), turns);
    assert_eq!(game.outcome(), Some(Outcome::Win(0)));
    assert!(game.failure().is_none());
}

#[test]
fn resync_restarts_a_bot_that_replays_the_record() {
    let record = played_by_the_arena();
    let turns = &record.recorded_turns;
    // Each bot is restarted at a position where it is to act, and must then
    // answer what the original bot answered at that point.
    for (seat, name, played, next) in [(0, "a", 4, 4), (1, "b", 3, 3)] {
        let spec = counter(name);
        let mut bot = resync_bot(
            &spec,
            &settings(seat, name),
            Countdown::boxed(10),
            setup(),
            &turns[..played],
            seat,
            LIMIT,
        )
        .unwrap();
        assert_eq!(bot.answers(), 2 - seat as u32);
        let game = LiveGame::replay(Countdown::boxed(10), setup(), &turns[..played]).unwrap();
        assert_eq!(game.to_act(), vec![seat]);
        let (lines, _) = bot
            .ask(&game.input_for(seat), game.answer_lines(seat), LIMIT)
            .unwrap();
        assert_eq!(lines, turns[next][0].lines);
    }
}

#[test]
fn resync_without_any_turn_gives_a_fresh_bot() {
    let a = counter("a");
    let bot = resync_bot(
        &a,
        &settings(0, "a"),
        Countdown::boxed(10),
        setup(),
        &[],
        0,
        LIMIT,
    )
    .unwrap();
    assert_eq!(bot.answers(), 0);
}

#[test]
fn resync_reports_the_turn_where_a_bot_diverges() {
    let record = played_by_the_arena();
    let turns = &record.recorded_turns;
    // Always answers "ok 99", where the record has "ok 1", "ok 2", ...
    let stubborn = shell(
        "stubborn",
        "while read line; do case \"$line\" in turn*) echo 'ok 99';; esac; done",
    );
    for (seat, first_turn) in [(0, 0), (1, 1)] {
        let error = resync_bot(
            &stubborn,
            &settings(seat, "stubborn"),
            Countdown::boxed(10),
            setup(),
            turns,
            seat,
            LIMIT,
        )
        .err()
        .expect("the bot diverges");
        match error {
            ResyncError::Diverged {
                turn,
                expected,
                got,
            } => {
                assert_eq!(turn, first_turn);
                assert_eq!(expected, vec!["ok 1".to_string()]);
                assert_eq!(got, vec!["ok 99".to_string()]);
            }
            other => panic!("unexpected error {other}"),
        }
    }
}

#[test]
fn resync_reports_a_bot_that_stops_answering() {
    let record = played_by_the_arena();
    let crash = shell("crash", "read seat; exit 3");
    let error = resync_bot(
        &crash,
        &settings(0, "crash"),
        Countdown::boxed(10),
        setup(),
        &record.recorded_turns,
        0,
        LIMIT,
    )
    .err()
    .expect("the bot crashes");
    match error {
        ResyncError::Bot(LiveError::Closed { detail }) => assert!(detail.contains('3'), "{detail}"),
        other => panic!("unexpected error {other}"),
    }
    let mute = shell("mute", "sleep 5");
    let error = resync_bot(
        &mute,
        &settings(0, "mute"),
        Countdown::boxed(10),
        setup(),
        &record.recorded_turns,
        0,
        Duration::from_millis(100),
    )
    .err()
    .expect("the bot is silent");
    assert!(matches!(error, ResyncError::Bot(LiveError::Timeout)));
}

#[test]
fn resync_reports_recorded_turns_that_break_the_rules() {
    let turn = |seat, line: &str| {
        vec![RecordedAnswer {
            seat,
            lines: vec![line.to_string()],
            ms: 1.0,
        }]
    };
    let turns = [turn(0, "ok 1"), turn(1, "nope")];
    let a = counter("a");
    let error = resync_bot(
        &a,
        &settings(0, "a"),
        Countdown::boxed(10),
        setup(),
        &turns,
        0,
        LIMIT,
    )
    .err()
    .expect("the record is invalid");
    assert!(
        matches!(&error, ResyncError::Invalid(invalid) if invalid.turn == 1),
        "{error}"
    );
    assert!(error.to_string().contains("nope"));
}

#[test]
fn a_missing_program_is_a_spawn_error() {
    let missing = BotSpec::parse("ghost=/definitely/not/here").unwrap();
    let error = resync_bot(
        &missing,
        &settings(0, "ghost"),
        Countdown::boxed(10),
        setup(),
        &[],
        0,
        LIMIT,
    )
    .err()
    .expect("no such program");
    assert!(matches!(error, ResyncError::Spawn(_)), "{error}");
}

/// Asks a bot that answers "ok" and the value of `CG_FIXED_ITERS`.
fn fixed_iters_seen_by_bot(fixed_iters: Option<u64>) -> String {
    let echo = shell(
        "echo",
        "read seat; read turn; echo \"ok ${CG_FIXED_ITERS-unset}\"",
    );
    let settings = BotSettings {
        fixed_iters,
        ..settings(0, "echo")
    };
    let mut bot = LiveBot::spawn(&echo, &settings).unwrap();
    let (lines, _) = bot.ask("seat 0\nturn 0\n", 1, LIMIT).unwrap();
    lines[0].clone()
}

#[test]
fn fixed_iterations_reach_the_bot() {
    assert_eq!(fixed_iters_seen_by_bot(Some(123)), "ok 123");
}

#[test]
fn without_fixed_iterations_the_bot_inherits_the_environment() {
    let inherited = std::env::var("CG_FIXED_ITERS").unwrap_or_else(|_| "unset".to_string());
    assert_eq!(fixed_iters_seen_by_bot(None), format!("ok {inherited}"));
}

#[test]
fn the_seed_and_time_scale_reach_the_bot() {
    let echo = shell(
        "echo",
        "read seat; read turn; echo \"$CG_SEED $CG_TIME_SCALE\"",
    );
    let settings = BotSettings {
        seed: 42,
        time_scale: 0.5,
        fixed_iters: None,
        show_stderr: false,
    };
    let mut bot = LiveBot::spawn(&echo, &settings).unwrap();
    let (lines, _) = bot.ask("seat 0\nturn 0\n", 1, LIMIT).unwrap();
    assert_eq!(lines, vec!["42 0.5".to_string()]);
}

#[test]
fn a_timeout_and_a_crash_are_distinguished() {
    let slow = shell("slow", "read seat; sleep 5");
    let mut bot = LiveBot::spawn(&slow, &settings(0, "slow")).unwrap();
    let error = bot
        .ask("seat 0\n", 1, Duration::from_millis(100))
        .unwrap_err();
    assert_eq!(error, LiveError::Timeout);
    assert_eq!(bot.answers(), 0);

    let crash = shell("crash", "read seat; exit 4");
    let mut bot = LiveBot::spawn(&crash, &settings(0, "crash")).unwrap();
    match bot.ask("seat 0\n", 1, LIMIT).unwrap_err() {
        LiveError::Closed { detail } => assert!(detail.contains('4'), "{detail}"),
        other => panic!("unexpected error {other}"),
    }
}

#[test]
fn resync_reports_its_progress_and_can_be_cancelled() {
    let record = played_by_the_arena();
    let turns = &record.recorded_turns;
    let a = counter("a");
    let mut seen = Vec::new();
    let bot = resync_bot_with(
        &a,
        &settings(0, "a"),
        Countdown::boxed(10),
        setup(),
        &turns[..5],
        0,
        LIMIT,
        &mut |done, total| {
            seen.push((done, total));
            true
        },
    )
    .unwrap();
    // Seat 0 answered in turns 0, 2 and 4.
    assert_eq!(seen, vec![(1, 3), (2, 3), (3, 3)]);
    assert_eq!(bot.answers(), 3);

    let mut calls = 0;
    let error = resync_bot_with(
        &a,
        &settings(0, "a"),
        Countdown::boxed(10),
        setup(),
        &turns[..5],
        0,
        LIMIT,
        &mut |_, _| {
            calls += 1;
            calls < 2
        },
    )
    .err()
    .expect("the replay was cancelled");
    assert!(matches!(error, ResyncError::Cancelled), "{error}");
    assert_eq!(calls, 2);
}
