use super::*;

fn run(input: &str) -> Result<String, String> {
    let mut output = Vec::new();
    play(input.as_bytes(), &mut output)?;
    Ok(String::from_utf8(output).expect("answers are ASCII"))
}

const MAP: &str = "O...........\n............\n............\n............\n............\n\
                   ............\n............\n............\n............\n............\n\
                   ...........X\n...........X\n";

/// One turn's input with the given building and unit lines.
fn turn(buildings: &[&str], units: &[&str]) -> String {
    let mut text = format!("21\n1\n20\n2\n{MAP}{}\n", buildings.len());
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
fn waits_every_turn() {
    let input = format!(
        "2\n1 0\n10 11\n{}{}",
        turn(&["0 0 0 0", "1 0 11 11"], &[]),
        turn(&["0 0 0 0", "1 0 11 11"], &["0 1 1 1 0", "1 2 1 11 10"]),
    );
    assert_eq!(run(&input).unwrap(), "WAIT\nWAIT\n");
}

#[test]
fn reads_games_without_mine_spots() {
    let input = format!("0\n{}", turn(&["0 0 0 0", "1 0 11 11"], &[]));
    assert_eq!(run(&input).unwrap(), "WAIT\n");
}

#[test]
fn stops_cleanly_when_the_game_ends() {
    assert_eq!(run("").unwrap(), "");
    assert_eq!(run("0\n").unwrap(), "");
}

#[test]
fn accepts_windows_line_endings() {
    let input = format!("0\n{}", turn(&[], &[])).replace('\n', "\r\n");
    assert_eq!(run(&input).unwrap(), "WAIT\n");
}

#[test]
fn rejects_a_bad_count() {
    let err = run("two\n").unwrap_err();
    assert!(err.contains("mine spot count"), "{err}");
    let input = format!("0\n21\n1\n20\n2\n{MAP}x\n");
    let err = run(&input).unwrap_err();
    assert!(err.contains("building count"), "{err}");
}

#[test]
fn rejects_input_that_stops_mid_turn() {
    let err = run("1\n").unwrap_err();
    assert!(err.contains("middle of a turn"), "{err}");
    let err = run("0\n21\n1\n").unwrap_err();
    assert!(err.contains("middle of a turn"), "{err}");
    let input = format!("0\n21\n1\n20\n2\n{MAP}1\n");
    let err = run(&input).unwrap_err();
    assert!(err.contains("middle of a turn"), "{err}");
}
