//! The interface between the arena and a game's rules.
//!
//! A referee speaks CodinGame's text protocol: it produces each player's
//! input as text and judges the text the player answers. It allows several
//! players to act in the same turn, so simultaneous-move games fit too.

use std::time::Duration;

/// Number of seats. Seat 0 moves first in turn-based games.
pub const SEATS: usize = 2;

/// What a referee needs to set up one game.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameSetup {
    /// Seed for everything random in the game: map, shuffles, opening.
    pub seed: u64,
    /// Moves the referee imposes at the start of the game, to vary the
    /// positions bots face. 0 plays the game exactly as on CodinGame. Each
    /// game decides how it imposes them; games without openings ignore it.
    pub opening_plies: u32,
}

/// Builds the referee of a new game. Shared by the arena's worker threads.
pub trait RefereeFactory: Fn(&GameSetup) -> Box<dyn Referee> + Sync {}

impl<F: Fn(&GameSetup) -> Box<dyn Referee> + Sync> RefereeFactory for F {}

/// How a finished game ended for the players.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The player in this seat won.
    Win(usize),
    Draw,
}

/// Time a player has to answer, measured from when its input starts being
/// sent until its last answer line is read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeLimits {
    /// For the player's first answer of the game.
    pub first_answer: Duration,
    /// For every later answer.
    pub later_answers: Duration,
}

/// The lines one player answered in a turn.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Answer {
    pub seat: usize,
    pub lines: Vec<String>,
}

/// An answer the rules reject. The player who sent it loses the game.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidAnswer {
    pub seat: usize,
    pub reason: String,
}

/// A game's rules as the arena sees them.
///
/// Each turn the arena asks [`players_to_act`](Referee::players_to_act),
/// sends each of them its [`turn_input`](Referee::turn_input), collects
/// [`answer_lines`](Referee::answer_lines) lines from each, then calls
/// [`play`](Referee::play) with all the answers. It stops as soon as
/// [`outcome`](Referee::outcome) returns a result.
pub trait Referee {
    /// The game's time limits.
    fn time_limits(&self) -> TimeLimits;

    /// Text sent once to `seat`, just before its first turn input. Most
    /// games send their map or settings here.
    fn initial_input(&self, seat: usize) -> String {
        let _ = seat;
        String::new()
    }

    /// The seats that must answer this turn, in order.
    fn players_to_act(&self) -> Vec<usize>;

    /// The turn's input for `seat`: whole lines, each ending with `\n`.
    fn turn_input(&self, seat: usize) -> String;

    /// How many lines `seat` must answer this turn.
    fn answer_lines(&self, seat: usize) -> usize {
        let _ = seat;
        1
    }

    /// Applies this turn's answers, given in `players_to_act` order.
    fn play(&mut self, answers: &[Answer]) -> Result<(), InvalidAnswer>;

    /// The result once the game is over, `None` before.
    fn outcome(&self) -> Option<Outcome>;
}
