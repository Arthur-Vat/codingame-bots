//! The interface a game implements to plug into the studio
//! ([ADR 0025](../../../docs/adr/0025-studio.md)).
//!
//! An adapter holds no rules of its own. A position is rebuilt by replaying
//! the game's referee, and a human's legal moves come from the referee's turn
//! input for that seat. The adapter only translates between the referee's
//! text protocol and the JSON the board renderer and the move picker use.

use cg_arena::live::{LiveGame, ReplayError};
use cg_arena::record::RecordedAnswer;
use cg_arena::referee::{GameSetup, Referee};
use serde::Serialize;

/// Which game an adapter is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameInfo {
    /// The game's folder name, for example `uttt`. Records name their game
    /// with it.
    pub id: &'static str,
    /// The name shown to the user.
    pub name: &'static str,
}

/// One move a human can choose, as the front end shows it and the server
/// plays it.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HumanMove {
    /// The move as the board renderer understands it, for example a cell.
    pub action: serde_json::Value,
    /// The answer lines to send the referee when the move is chosen.
    pub lines: Vec<String>,
}

/// What a game gives the studio.
pub trait StudioGame: Send + Sync {
    /// Which game this is.
    fn info(&self) -> GameInfo;

    /// A fresh referee for this setup.
    fn new_referee(&self, setup: &GameSetup) -> Box<dyn Referee>;

    /// How many leading turns the referee imposes (an opening). The server
    /// plays them itself, and no human is asked for them. 0 for a game
    /// without one.
    fn opening_turns(&self, setup: &GameSetup) -> usize;

    /// What the board renderer draws: frame 0 is the start, then one frame
    /// per step. Turn-based games have one step per turn; a game whose turn
    /// has several steps returns several frames for it.
    ///
    /// The imposed turns ([`opening_turns`](StudioGame::opening_turns)) are
    /// part of the start: frame 0 is the position after them, and frame `i`
    /// shows `turns[..opening + i]`. The frames of a game with fewer turns
    /// than its opening are just frame 0.
    ///
    /// Fails, with the index of the turn, when the turns break the rules.
    /// `ReplayError::turn` counts all the turns, the opening's included.
    /// There is no frame for a turn inside the opening: a caller shows
    /// frame 0 of a fresh game (`frames(setup, &[])`) in that case.
    fn frames(
        &self,
        setup: &GameSetup,
        turns: &[Vec<RecordedAnswer>],
    ) -> Result<Vec<serde_json::Value>, ReplayError>;

    /// The moves a human in `seat` may play after `turns`, built from the
    /// referee's turn input for that seat. Empty when `seat` does not act
    /// now, and while the opening is not over.
    ///
    /// Fails, with the index of the turn, when the turns break the rules.
    fn human_moves(
        &self,
        setup: &GameSetup,
        turns: &[Vec<RecordedAnswer>],
        seat: usize,
    ) -> Result<Vec<HumanMove>, ReplayError>;
}

/// The game at the position after `turns`, replayed with a fresh referee.
pub fn live_game(
    game: &dyn StudioGame,
    setup: GameSetup,
    turns: &[Vec<RecordedAnswer>],
) -> Result<LiveGame, ReplayError> {
    LiveGame::replay(game.new_referee(&setup), setup, turns)
}
