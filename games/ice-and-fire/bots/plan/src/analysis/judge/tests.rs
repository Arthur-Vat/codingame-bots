use cg_core::rng::Rng;
use ice_and_fire_engine::{League, State};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

use super::*;

/// Middle-game positions of a random game, as bank blocks with `played`.
fn blocks(played: Option<&str>) -> String {
    let map = generate(41, RefereeLeague::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let mut rng = Rng::new(41 ^ 0x9e37_79b9);
    let mut commands = Vec::new();
    let mut text = String::new();
    for ply in 0..60 {
        if state.outcome().is_some() {
            break;
        }
        if ply >= 24 && ply % 12 == 0 {
            let seat = state.to_act();
            text.push_str(&format!(
                "position g{ply} {ply} {}\n{}turn\n{}",
                state.game_turn(),
                state.initial_input(seat),
                state.turn_input(seat)
            ));
            if let Some(line) = played {
                text.push_str(&format!("played {line}\n"));
            }
            text.push_str("end\n");
        }
        state.random_turn(&mut rng, &mut commands);
        state.play_turn(&commands);
    }
    text
}

#[test]
fn the_played_line_is_read_and_the_turn_keeps_its_input() {
    let with = read_bank(&blocks(Some("MOVE 3 4 5;TRAIN 1 2 2"))).expect("a bank");
    let without = read_bank(&blocks(None)).expect("a bank");
    assert!(!with.is_empty());
    assert_eq!(with.len(), without.len());
    for (a, b) in with.iter().zip(&without) {
        assert_eq!(a.played.as_deref(), Some("MOVE 3 4 5;TRAIN 1 2 2"));
        assert_eq!(b.played, None);
        assert_eq!(a.turn, b.turn);
        assert_eq!(
            a.state().expect("state").turn_input(0),
            b.state().expect("state").turn_input(0)
        );
    }
    let wait = read_bank(&blocks(Some("WAIT"))).expect("a bank");
    assert_eq!(wait[0].played.as_deref(), Some("WAIT"));
}

#[test]
fn the_search_scores_its_own_plan_as_a_played_one() {
    let bank = read_bank(&blocks(None)).expect("a bank");
    let mut compared = 0;
    for position in &bank {
        let ours = judge(position, 60).expect("judged");
        assert_eq!(ours.played_score, None);
        assert_eq!(ours.same, "-");
        // The same position with the plan it chose as the played answer.
        let mut played = Position {
            name: position.name.clone(),
            ply: position.ply,
            game_turn: position.game_turn,
            initial: position.initial.clone(),
            turn: position.turn.clone(),
            played: Some(answer_line(&ours.ours)),
        };
        let again = judge(&played, 60).expect("judged");
        assert_eq!(again.ours, ours.ours, "the search repeats");
        assert_eq!(again.ours_score, ours.ours_score);
        assert_eq!(again.same, "yes");
        assert_eq!(
            again.played_score,
            Some(again.ours_score),
            "{}",
            position.name
        );
        // Doing nothing scores something too, and not above the search's plan.
        played.played = Some("WAIT".to_string());
        let wait = judge(&played, 60).expect("judged");
        let score = wait.played_score.expect("a score");
        assert!(score <= again.ours_score, "{score} > {}", again.ours_score);
        assert_eq!(wait.same, if ours.ours.is_empty() { "yes" } else { "no" });
        compared += 1;
    }
    assert!(compared > 0);
}

#[test]
fn a_line_has_the_documented_fields() {
    let bank = read_bank(&blocks(Some("WAIT"))).expect("a bank");
    let j = judge(&bank[0], 30).expect("judged");
    let text = line(&bank[0], &j);
    let fields: Vec<&str> = text.split('\t').collect();
    assert_eq!(fields.len(), 6, "{text}");
    assert_eq!(fields[0], bank[0].name);
    assert_eq!(fields[1], bank[0].ply.to_string());
    assert!(fields[2].starts_with("ours="));
    assert!(fields[3].starts_with("ours_score="));
    assert!(fields[4].starts_with("played_score="));
    assert!(fields[5].starts_with("same="));
}

#[test]
fn same_tells_sets_first_commands_and_neither() {
    let a = commands_of("TRAIN 1 2 2;MOVE 3 4 5");
    assert_eq!(same(&a, &commands_of("MOVE 3 4 5;TRAIN 1 2 2")), "yes");
    assert_eq!(same(&a, &commands_of("TRAIN 1 2 2;WAIT")), "first");
    assert_eq!(same(&a, &commands_of("MOVE 3 4 5")), "no");
    assert_eq!(commands_of("WAIT"), vec![]);
}

#[test]
fn bad_arguments_are_errors() {
    assert!(run(&[]).is_err());
    assert!(run(&["--tries".to_string()]).is_err());
    assert!(run(&["--nope".to_string()]).is_err());
    assert!(run(&["--positions".to_string(), "/nonexistent/bank".to_string()]).is_err());
}
