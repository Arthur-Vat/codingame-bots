use ice_and_fire_engine::{League, State};

use crate::replay::replay;
use crate::test_util::random_record;
use crate::trace::trace;

use super::*;

#[test]
fn the_middle_game_is_the_traces() {
    let mut seen = 0;
    for seed in 1..8 {
        let record = random_record(seed, ["a", "b"]);
        let t = trace("g", &record);
        let game = replay("g", record);
        let plies = middle_plies(&game);
        match t.middle {
            Some(from) => {
                assert_eq!(plies.start, from, "seed {seed}");
                assert_eq!(plies.end, t.endgame.unwrap_or(t.plies.len()), "seed {seed}");
                seen += plies.len();
            }
            None => assert!(plies.is_empty()),
        }
    }
    assert!(seen > 0);
}

#[test]
fn a_draw_is_sorted_reproducible_and_without_repeats() {
    let candidates: Vec<(usize, usize)> =
        (0..10).flat_map(|g| (0..30).map(move |p| (g, p))).collect();
    let a = draw(candidates.clone(), 50, 7);
    assert_eq!(a, draw(candidates.clone(), 50, 7));
    assert_ne!(a, draw(candidates.clone(), 50, 8));
    assert_eq!(a.len(), 50);
    assert!(a.windows(2).all(|w| w[0] < w[1]));
    assert_eq!(draw(candidates, 1000, 7).len(), 300);
}

#[test]
fn a_block_reads_back_as_the_position_of_the_player_to_act() {
    let game = replay("game-x", random_record(3, ["a", "b"]));
    for ply in [0, 1, 7] {
        let text = block(&game, ply);
        let state = &game.steps[ply].state;
        let seat = state.to_act();
        let mut lines = text.lines();
        assert_eq!(
            lines.next(),
            Some(format!("position game-x {ply} {}", state.game_turn()).as_str())
        );
        let body: Vec<&str> = lines.collect();
        let turn_at = body.iter().position(|l| *l == "turn").expect("a turn line");
        assert_eq!(body.last(), Some(&"end"));
        let initial = body[..turn_at].join("\n") + "\n";
        let turn = body[turn_at + 1..body.len() - 1].join("\n") + "\n";
        let read =
            State::read(&initial, &turn, League::Bronze, state.game_turn()).expect("a state");
        assert_eq!(read.to_act(), seat);
        assert_eq!(read.turn_input(seat), state.turn_input(seat));
    }
}
