use super::*;

fn run(strict: bool, input: &str) -> (Result<(), InputError>, String, String) {
    let (mut out, mut log) = (Vec::new(), Vec::new());
    let result = play(
        &mut Rng::new(1),
        strict,
        Input::new(input.as_bytes()),
        &mut out,
        &mut log,
    );
    (
        result,
        String::from_utf8(out).unwrap(),
        String::from_utf8(log).unwrap(),
    )
}

/// The first turn of a game where this bot moves first: all 81 cells.
fn first_turn() -> String {
    let mut input = "-1 -1\n81\n".to_string();
    for row in 0..9 {
        for col in 0..9 {
            input.push_str(&format!("{row} {col}\n"));
        }
    }
    input
}

#[test]
fn plays_a_valid_action_and_logs_the_check() {
    let (result, out, log) = run(false, &first_turn());
    assert_eq!(result, Err(InputError::Eof));
    assert_eq!(out.lines().count(), 1);
    assert!(
        log.contains("turn 1: same valid actions; 1 turns checked, 0 with differences"),
        "{log}"
    );
}

#[test]
fn keeps_playing_after_a_difference_unless_strict() {
    let wrong = "-1 -1\n2\n4 4\n9 9\n";
    let (result, out, log) = run(false, wrong);
    assert_eq!(result, Err(InputError::Eof));
    assert_eq!(out.lines().count(), 1);
    assert!(log.contains("DIFFERENCE on turn 1"), "{log}");

    let (result, out, _) = run(true, wrong);
    assert!(
        matches!(result, Err(InputError::Parse { .. })),
        "{result:?}"
    );
    assert!(out.is_empty());
}
