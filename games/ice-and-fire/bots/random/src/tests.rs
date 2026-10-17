use cg_arena::referee::{Answer, GameSetup, Outcome, Referee};
use ice_and_fire_referee::IceAndFireReferee;

use super::*;

fn run(seed: u64, input: &str) -> Result<String, String> {
    let mut output = Vec::new();
    play(&mut Rng::new(seed), input.as_bytes(), &mut output)?;
    Ok(String::from_utf8(output).expect("answers are ASCII"))
}

const MAP: &str = "O...........\n............\n............\n............\n............\n\
                   ............\n............\n............\n............\n............\n\
                   ...........X\n...........X\n";

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

fn referee(seed: u64) -> IceAndFireReferee {
    IceAndFireReferee::new(&GameSetup {
        seed,
        opening_plies: 0,
    })
}

/// Plays whole games of the bot against itself on the referee's maps, one
/// call of `play` per turn, and checks that the referee accepts every answer.
#[test]
fn every_answer_is_accepted_by_the_referee() {
    let mut finished = 0;
    for seed in 0..40 {
        let mut referee = referee(seed);
        let initial = referee.initial_input(0);
        let mut rng = Rng::new(seed);
        while let Some(&seat) = referee.players_to_act().first() {
            let input = format!("{initial}{}", referee.turn_input(seat));
            let mut output = Vec::new();
            play(&mut rng, input.as_bytes(), &mut output).unwrap();
            let line = String::from_utf8(output).unwrap().trim().to_string();
            assert_eq!(line.lines().count(), 1, "seed {seed}: {line:?}");
            let answers = [Answer {
                seat,
                lines: vec![line.clone()],
            }];
            if let Err(err) = referee.play(&answers) {
                panic!("seed {seed}, seat {seat}: {line:?} refused: {}", err.reason);
            }
        }
        if matches!(referee.outcome(), Some(Outcome::Win(_) | Outcome::Draw)) {
            finished += 1;
        }
    }
    assert_eq!(finished, 40);
}

#[test]
fn plays_every_turn_until_the_input_ends() {
    let buildings = ["0 0 0 0", "1 0 11 11"];
    let input = format!(
        "2\n1 0\n10 11\n{}{}",
        turn(&buildings, &[]),
        turn(&buildings, &["0 1 1 1 0", "1 2 1 11 10"]),
    );
    let output = run(1, &input).unwrap();
    assert_eq!(output.lines().count(), 2, "{output}");
}

#[test]
fn same_seed_same_answers() {
    let buildings = ["0 0 0 0", "1 0 11 11"];
    let input = format!("0\n{}{}", turn(&buildings, &[]), turn(&buildings, &[]));
    assert_eq!(run(9, &input), run(9, &input));
    let answers: std::collections::HashSet<String> =
        (0..30).map(|seed| run(seed, &input).unwrap()).collect();
    assert!(answers.len() > 1, "{answers:?}");
}

#[test]
fn waits_on_a_turn_that_is_not_a_state() {
    // No HQ among the buildings: the engine cannot read it, so the bot waits.
    let input = format!("0\n{}", turn(&[], &[]));
    assert_eq!(run(1, &input).unwrap(), "WAIT\n");
    let input = format!("0\n{}", turn(&["0 0 5 5"], &[]));
    assert_eq!(run(1, &input).unwrap(), "WAIT\n");
}

#[test]
fn stops_cleanly_when_the_game_ends() {
    assert_eq!(run(1, "").unwrap(), "");
    assert_eq!(run(1, "0\n").unwrap(), "");
}

#[test]
fn accepts_windows_line_endings() {
    let input = format!("0\n{}", turn(&["0 0 0 0", "1 0 11 11"], &[])).replace('\n', "\r\n");
    let output = run(1, &input).unwrap();
    assert_eq!(output.lines().count(), 1, "{output}");
}

#[test]
fn rejects_bad_counts_and_input_that_stops_mid_turn() {
    let err = run(1, "two\n").unwrap_err();
    assert!(err.contains("mine spot count"), "{err}");
    let err = run(1, "100000\n").unwrap_err();
    assert!(err.contains("mine spot count"), "{err}");
    let input = format!("0\n20\n1\n20\n2\n{MAP}x\n");
    let err = run(1, &input).unwrap_err();
    assert!(err.contains("building count"), "{err}");
    let err = run(1, "1\n").unwrap_err();
    assert!(err.contains("middle of a turn"), "{err}");
    let err = run(1, "0\n20\n1\n").unwrap_err();
    assert!(err.contains("middle of a turn"), "{err}");
}

#[test]
fn never_panics_on_garbage_that_has_the_right_shape() {
    // Right line counts, wrong content: every turn is answered with WAIT.
    let junk = "a\nb\nc\nd\n".to_string() + &"?\n".repeat(12) + "0\n0\n";
    let input = format!("0\n{junk}{junk}");
    assert_eq!(run(1, &input).unwrap(), "WAIT\nWAIT\n");
}
