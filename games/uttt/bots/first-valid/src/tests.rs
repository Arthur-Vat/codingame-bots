use super::*;

fn run(input: &str) -> Result<String, String> {
    let mut output = Vec::new();
    play(input.as_bytes(), &mut output)?;
    Ok(String::from_utf8(output).expect("moves are ASCII"))
}

#[test]
fn plays_the_first_valid_action_every_turn() {
    let input = "-1 -1\n3\n4 4\n0 0\n8 8\n4 4\n2\n3 3\n3 4\n";
    assert_eq!(run(input).unwrap(), "4 4\n3 3\n");
}

#[test]
fn stops_cleanly_when_the_game_ends() {
    assert_eq!(run("").unwrap(), "");
}

#[test]
fn accepts_windows_line_endings() {
    assert_eq!(run("-1 -1\r\n1\r\n2 7\r\n").unwrap(), "2 7\n");
}

#[test]
fn rejects_a_malformed_action() {
    let err = run("-1 -1\n1\n4 x\n").unwrap_err();
    assert!(err.contains("\"4 x\""), "{err}");
}

#[test]
fn rejects_a_turn_without_valid_actions() {
    let err = run("-1 -1\n0\n").unwrap_err();
    assert!(err.contains("no valid action"), "{err}");
}

#[test]
fn rejects_input_that_stops_mid_turn() {
    let err = run("-1 -1\n2\n4 4\n").unwrap_err();
    assert!(err.contains("middle of a turn"), "{err}");
}
