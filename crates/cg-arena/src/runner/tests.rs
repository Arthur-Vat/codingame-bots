use super::*;
use crate::referee::{InvalidAnswer, TimeLimits};

/// A test game: players alternate for `length` turns, each answering "ok".
/// Answering "win" wins at once; anything else is invalid.
struct Countdown {
    length: u32,
    played: u32,
    winner: Option<usize>,
    inputs: Vec<String>,
}

impl Countdown {
    fn new(length: u32) -> Self {
        Countdown {
            length,
            played: 0,
            winner: None,
            inputs: Vec::new(),
        }
    }
}

impl Referee for Countdown {
    fn time_limits(&self) -> TimeLimits {
        TimeLimits {
            first_answer: Duration::from_millis(2000),
            later_answers: Duration::from_millis(300),
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
        self.inputs.push(answer.lines.join("|"));
        match answer.lines[0].as_str() {
            "ok" => {}
            "win" => self.winner = Some(answer.seat),
            other => {
                return Err(InvalidAnswer {
                    seat: answer.seat,
                    reason: format!("unknown answer {other:?}"),
                })
            }
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

const POLITE: &str = "while read line; do case \"$line\" in turn*) echo ok;; esac; done";

fn play(referee: &mut Countdown, bots: [&BotSpec; 2]) -> MatchRecord {
    run_match(referee, bots, 7, &MatchOptions::default()).expect("bots start")
}

#[test]
fn plays_a_full_game() {
    let a = shell("a", POLITE);
    let b = shell("b", POLITE);
    let mut referee = Countdown::new(6);
    let record = play(&mut referee, [&a, &b]);
    assert_eq!(record.end, EndReason::Finished);
    assert_eq!(record.winner, None);
    assert_eq!(record.turns, 6);
    assert_eq!(record.seats, ["a".to_string(), "b".to_string()]);
    assert!(record.max_answer_ms[0] > 0.0 && record.mean_answer_ms[1] > 0.0);
}

#[test]
fn sends_the_initial_input_once_before_the_first_turn() {
    // Answers "win" only if the first line it reads is its seat line.
    let a = shell(
        "a",
        "read first; read turn; [ \"$first\" = 'seat 0' ] && echo win",
    );
    let b = shell("b", POLITE);
    let record = play(&mut Countdown::new(6), [&a, &b]);
    assert_eq!(record.winner, Some(0), "{record:?}");
    assert_eq!(record.end, EndReason::Finished);
}

#[test]
fn a_timeout_loses() {
    let slow = shell("slow", "read seat; read turn; echo ok; read turn; sleep 2");
    let b = shell("b", POLITE);
    let record = play(&mut Countdown::new(6), [&slow, &b]);
    assert_eq!(record.winner, Some(1));
    assert_eq!(
        record.end,
        EndReason::Timeout {
            seat: 0,
            limit_ms: 300.0
        }
    );
}

#[test]
fn the_first_answer_gets_the_longer_limit() {
    // 500 ms is over the later limit (300 ms) but under the first (2000 ms).
    let a = shell("a", &format!("sleep 0.5; {POLITE}"));
    let b = shell("b", POLITE);
    let record = play(&mut Countdown::new(4), [&a, &b]);
    assert_eq!(record.end, EndReason::Finished, "{record:?}");
}

#[test]
fn time_scale_stretches_the_limits() {
    let slow = shell(
        "slow",
        "while read line; do case \"$line\" in turn*) sleep 0.4; echo ok;; esac; done",
    );
    let b = shell("b", POLITE);
    let options = MatchOptions {
        time_scale: 3.0,
        ..MatchOptions::default()
    };
    let record = run_match(&mut Countdown::new(4), [&slow, &b], 7, &options).unwrap();
    assert_eq!(record.end, EndReason::Finished, "{record:?}");
}

#[test]
fn a_crash_loses_and_reports_the_exit_status() {
    let b = shell("b", POLITE);
    let crash = shell("crash", "read seat; exit 3");
    let record = play(&mut Countdown::new(6), [&b, &crash]);
    // Seat 0 answers turn 0, then seat 1 crashes on turn 1.
    assert_eq!(record.winner, Some(0));
    match &record.end {
        EndReason::Crash { seat: 1, detail } => assert!(detail.contains('3'), "{detail}"),
        other => panic!("unexpected end {other:?}"),
    }
}

#[test]
fn an_invalid_answer_loses() {
    let rude = shell("rude", "while read line; do echo nope; done");
    let b = shell("b", POLITE);
    let record = play(&mut Countdown::new(6), [&rude, &b]);
    assert_eq!(record.winner, Some(1));
    assert_eq!(
        record.end,
        EndReason::Invalid {
            seat: 0,
            reason: "unknown answer \"nope\"".to_string()
        }
    );
}

#[test]
fn the_turn_limit_aborts_as_a_draw() {
    let a = shell("a", POLITE);
    let b = shell("b", POLITE);
    let options = MatchOptions {
        max_turns: 3,
        ..MatchOptions::default()
    };
    let record = run_match(&mut Countdown::new(100), [&a, &b], 7, &options).unwrap();
    assert_eq!(record.winner, None);
    assert!(
        matches!(record.end, EndReason::Aborted { .. }),
        "{record:?}"
    );
    assert_eq!(record.turns, 3);
}

#[test]
fn bots_receive_a_seed_derived_from_the_match_seed() {
    // The bot wins only if CG_SEED holds the expected value.
    let expected = bot_seed(7, 0, "a");
    let a = shell(
        "a",
        &format!("read seat; read turn; [ \"$CG_SEED\" = '{expected}' ] && echo win"),
    );
    let b = shell("b", POLITE);
    let record = play(&mut Countdown::new(6), [&a, &b]);
    assert_eq!(record.winner, Some(0), "{record:?}");
    assert_ne!(bot_seed(7, 0, "a"), bot_seed(7, 1, "a"));
    assert_ne!(bot_seed(7, 0, "a"), bot_seed(7, 0, "b"));
}

#[test]
fn a_missing_program_is_an_error_not_a_crash() {
    let missing = BotSpec::parse("ghost=/definitely/not/here").unwrap();
    let b = shell("b", POLITE);
    let error = run_match(
        &mut Countdown::new(2),
        [&missing, &b],
        1,
        &MatchOptions::default(),
    )
    .unwrap_err();
    assert!(error.to_string().contains("ghost"), "{error}");
}

#[test]
fn parses_bot_specs() {
    assert_eq!(
        BotSpec::parse("v1=target/release/bot --fast 3"),
        Ok(BotSpec {
            name: "v1".to_string(),
            program: "target/release/bot".to_string(),
            args: vec!["--fast".to_string(), "3".to_string()],
        })
    );
    assert!(BotSpec::parse("no-equals-sign").is_err());
    assert!(BotSpec::parse("name=").is_err());
    assert!(BotSpec::parse(" =bot").is_err());
}
