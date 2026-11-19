//! `ice-and-fire-arena-early`: the arena's commands on duels cut at the
//! early-game judge's game turn (phase 0 of the early-game plan,
//! `games/ice-and-fire/research/codingame/2026-11-19-early-game.md`).
//!
//! Each game is played as CodinGame plays it until both sides have started
//! their ply of [`SCORE_TURN`] (12); each side's zasmu-likeness is taken at
//! the start of that ply, as `ice-and-fire-review early-fit` fitted it, and
//! the side with the higher one wins (equal scores draw). A game that ends
//! earlier (an HQ taken, a fault) keeps its real result. A match, an SPRT
//! or a screening run with this binary judges early games only, many times
//! faster than full games; whether that judge agrees with full games is
//! checked with `ice-and-fire-review early-check` on full-game records.
//!
//! Its records carry the game id `ice-and-fire`, as the full arena's, and
//! the scored winner as their winner.

use std::process::ExitCode;

use cg_arena::referee::{Answer, GameSetup, InvalidAnswer, Outcome, Referee, TimeLimits};
use ice_and_fire_early::score::{zasmu_likeness, SCORE_TURN};
use ice_and_fire_early::zones::Zones;
use ice_and_fire_engine::{League, State};
use ice_and_fire_referee::IceAndFireReferee;

/// A full game's referee whose game ends once both sides' zasmu-likeness
/// is known.
struct Cut {
    inner: IceAndFireReferee,
    zones: Option<Zones>,
    scores: [Option<f64>; 2],
    decided: Option<Outcome>,
}

impl Cut {
    /// `seat`'s zasmu-likeness now, read from the input it receives, as a
    /// bot would.
    fn score(&mut self, seat: usize) -> f64 {
        let state = State::read(
            &self.inner.initial_input(seat),
            &self.inner.turn_input(seat),
            League::Bronze,
            self.inner.game_turn(),
        )
        .unwrap_or_else(|e| panic!("the referee's input does not read: {e}"));
        let zones = *self.zones.get_or_insert_with(|| Zones::of(&state));
        zasmu_likeness(&state, seat, &zones)
    }
}

impl Referee for Cut {
    fn time_limits(&self) -> TimeLimits {
        self.inner.time_limits()
    }

    fn initial_input(&self, seat: usize) -> String {
        self.inner.initial_input(seat)
    }

    fn players_to_act(&self) -> Vec<usize> {
        if self.decided.is_some() {
            Vec::new()
        } else {
            self.inner.players_to_act()
        }
    }

    fn turn_input(&self, seat: usize) -> String {
        self.inner.turn_input(seat)
    }

    fn answer_lines(&self, seat: usize) -> usize {
        self.inner.answer_lines(seat)
    }

    fn play(&mut self, answers: &[Answer]) -> Result<(), InvalidAnswer> {
        if self.inner.game_turn() == SCORE_TURN {
            if let Some(&seat) = self.inner.players_to_act().first() {
                if self.scores[seat].is_none() {
                    self.scores[seat] = Some(self.score(seat));
                }
            }
        }
        self.inner.play(answers)?;
        if self.inner.outcome().is_none() {
            if let [Some(red), Some(blue)] = self.scores {
                self.decided = Some(match red.total_cmp(&blue) {
                    std::cmp::Ordering::Greater => Outcome::Win(0),
                    std::cmp::Ordering::Less => Outcome::Win(1),
                    std::cmp::Ordering::Equal => Outcome::Draw,
                });
            }
        }
        Ok(())
    }

    fn outcome(&self) -> Option<Outcome> {
        self.inner.outcome().or(self.decided)
    }
}

fn new_referee(setup: &GameSetup) -> Box<dyn Referee> {
    Box::new(Cut {
        inner: IceAndFireReferee::new(setup),
        zones: None,
        scores: [None, None],
        decided: None,
    })
}

fn main() -> ExitCode {
    // The full arena's name, so that records carry the game id
    // `ice-and-fire` (the arena takes it from the name).
    cg_arena::cli::main("ice-and-fire-arena", new_referee)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two players who wait: the game runs to the start of blue's ply of
    /// the score's turn and ends after it, won by the higher score.
    #[test]
    fn a_game_of_waits_ends_after_blues_ply_of_the_score_turn() {
        let setup = GameSetup {
            seed: 11,
            opening_plies: 0,
        };
        let mut referee = new_referee(&setup);
        let mut plies = 0;
        while referee.outcome().is_none() {
            let seat = referee.players_to_act()[0];
            referee
                .play(&[Answer {
                    seat,
                    lines: vec!["WAIT".to_string()],
                }])
                .unwrap();
            plies += 1;
        }
        assert_eq!(plies, 2 * SCORE_TURN);
        assert!(referee.players_to_act().is_empty());
        // The same game in the full referee, scored at blue's start of
        // the score turn by the engine.
        let mut full = IceAndFireReferee::new(&setup);
        let mut scores = [0.0; 2];
        for _ in 0..2 * SCORE_TURN {
            let seat = full.players_to_act()[0];
            if full.game_turn() == SCORE_TURN {
                let state = State::read(
                    &full.initial_input(seat),
                    &full.turn_input(seat),
                    League::Bronze,
                    SCORE_TURN,
                )
                .unwrap();
                scores[seat] = zasmu_likeness(&state, seat, &Zones::of(&state));
            }
            full.play(&[Answer {
                seat,
                lines: vec!["WAIT".to_string()],
            }])
            .unwrap();
        }
        assert!(full.outcome().is_none());
        let expected = if scores[0] > scores[1] { 0 } else { 1 };
        assert_eq!(referee.outcome(), Some(Outcome::Win(expected)));
    }
}
