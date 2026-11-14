use cg_core::rng::Rng;
use ice_and_fire_engine::{answer_line, League, State};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

use super::*;
use crate::analysis::judge::judge;

/// Middle-game positions of a random game, as bank blocks with `played` and
/// `reply` lines when given.
fn blocks(played: Option<&str>, reply: Option<&str>) -> String {
    let map = generate(43, RefereeLeague::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let mut rng = Rng::new(43 ^ 0x9e37_79b9);
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
            if let Some(line) = reply {
                text.push_str(&format!("reply {line}\n"));
            }
            text.push_str("end\n");
        }
        state.random_turn(&mut rng, &mut commands);
        state.play_turn(&commands);
    }
    text
}

#[test]
fn the_reply_line_is_read_after_the_played_line() {
    let both = read_bank(&blocks(Some("WAIT"), Some("MOVE 3 4 5"))).expect("a bank");
    let none = read_bank(&blocks(None, None)).expect("a bank");
    assert!(!both.is_empty());
    for (a, b) in both.iter().zip(&none) {
        assert_eq!(a.played.as_deref(), Some("WAIT"));
        assert_eq!(a.reply.as_deref(), Some("MOVE 3 4 5"));
        assert_eq!((b.played.as_deref(), b.reply.as_deref()), (None, None));
        assert_eq!(a.turn, b.turn);
    }
}

#[test]
fn a_passing_reply_is_the_pass_and_the_expected_is_the_worst_modeled() {
    let bank = read_bank(&blocks(None, None)).expect("a bank");
    let mut judged = 0;
    for position in &bank {
        // Our own plan as the played one, the opponent passing.
        let Ok(ours) = judge(position, 60) else {
            continue;
        };
        let mut with = Position {
            played: Some(answer_line(&ours.ours)),
            reply: Some("WAIT".to_string()),
            ..position.clone()
        };
        let Ok(s) = surprise(&with, 60) else {
            continue;
        };
        assert_eq!(s.actual, s.pass);
        assert!(s.modeled > 0);
        assert!(s.expected <= s.modeled_best);
        assert_eq!(s.expected, ours.ours_score, "the same plan, the same score");
        // A reply that is not a command at all is the pass too.
        with.reply = Some(String::new());
        assert_eq!(surprise(&with, 60).expect("judged").actual, s.pass);
        judged += 1;
    }
    assert!(judged > 0, "some position was judged");
}
