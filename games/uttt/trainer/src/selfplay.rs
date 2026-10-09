//! Self-play games between two searches, recording at each searched
//! position what a search longer than a turn's found: its root score and,
//! if wanted, the visits of each move.

use cg_core::rng::Rng;
use cg_search::{Budget, Game, Mcts};
use uttt_engine::search::{PatternBoard, PolicyBoard};
use uttt_engine::{Board, Move, PatternPolicy, PlayoutPolicy, Status};

use crate::data::{GameRecord, Searched};

/// How self-play games are played.
#[derive(Clone, Copy, Debug)]
pub struct SelfPlay {
    /// Search iterations per move.
    pub iterations: u64,
    /// Random moves at the start of each game, for variety; not recorded
    /// as searched.
    pub opening_plies: u32,
    /// The search's exploration constant, as in the bot.
    pub exploration: f64,
    /// The playout policy, as in the bot; decisive playouts when `None`.
    pub policy: Option<&'static PlayoutPolicy>,
    /// A pattern policy (E015) for the playouts and, as in the bot, the
    /// order of each node's children; it replaces `policy` when set.
    pub patterns: Option<&'static PatternPolicy>,
    /// Whether to record the visits of each move, which the playout policy
    /// is fitted to; the value network needs only the root's score.
    pub record_visits: bool,
}

impl SelfPlay {
    /// Plays one game from `seed`. Each side keeps its own search tree
    /// from move to move, as the bot does. Positions with a move that wins
    /// the game at once are not recorded: the search plays that move
    /// without searching.
    pub fn play(&self, seed: u64) -> GameRecord {
        match (self.patterns, self.policy) {
            (Some(policy), _) => self.play_as(seed, true, |board| PatternBoard { board, policy }),
            (None, Some(policy)) => {
                self.play_as(seed, false, |board| PolicyBoard { board, policy })
            }
            (None, None) => self.play_as(seed, false, |board| board),
        }
    }

    /// Plays one game, searching `position(board)` at each move, with the
    /// game's priors if `priors`.
    fn play_as<G: Game<Move = Move>>(
        &self,
        seed: u64,
        priors: bool,
        position: impl Fn(Board) -> G,
    ) -> GameRecord {
        let mut rng = Rng::new(seed);
        let mut searches = [
            Mcts::new(self.exploration, rng.next_u64()),
            Mcts::new(self.exploration, rng.next_u64()),
        ];
        for search in &mut searches {
            search.priors = priors;
        }
        let mut board = Board::new();
        let mut record = GameRecord::default();
        let mut moves = Vec::new();
        while board.status() == Status::Ongoing {
            let mv = if record.moves.len() < self.opening_plies as usize {
                board.random_move(&mut rng)
            } else {
                let root = position(board);
                root.legal_moves(&mut moves);
                let search = &mut searches[board.to_move()];
                let result = search.search(&root, &moves, Budget::Iterations(self.iterations));
                if board.game_winning_move().is_none() && result.iterations > 0 {
                    record.searched.push(Searched {
                        ply: record.moves.len() as u8,
                        score: Some(result.expected_score as f32),
                        visits: if self.record_visits {
                            search.root_visits().collect()
                        } else {
                            Vec::new()
                        },
                    });
                }
                result.best
            };
            record.moves.push(mv);
            board.play(mv);
        }
        record
    }
}

/// Reads the playout weights from Rust source written by `uttt-trainer
/// fit-policy`, such as the bot's `weights.rs`.
pub fn read_policy_weights(source: &str) -> Result<[u32; uttt_engine::board::CLASSES], String> {
    let start = source
        .find("PLAYOUT_WEIGHTS")
        .and_then(|at| source[at..].find("= [").map(|offset| at + offset + 3))
        .ok_or("no `PLAYOUT_WEIGHTS = [` in the source")?;
    let end = source[start..]
        .find(']')
        .ok_or("the weights' list does not end")?;
    let weights = source[start..start + end]
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(|item| {
            item.parse::<u32>()
                .map_err(|err| format!("{item:?}: {err}"))
        })
        .collect::<Result<Vec<u32>, String>>()?;
    let count = weights.len();
    weights
        .try_into()
        .map_err(|_| format!("{count} weights instead of {}", uttt_engine::board::CLASSES))
}

/// Reads the pattern weights' text from Rust source written by
/// `uttt-trainer fit-patterns`, such as the bot's `pattern_weights.rs`:
/// the string after `PATTERN_TEXT`, without its line continuations.
pub fn read_pattern_text(source: &str) -> Result<String, String> {
    let start = source
        .find("PATTERN_TEXT: &str")
        .and_then(|at| source[at..].find('"').map(|offset| at + offset + 1))
        .ok_or("no `PATTERN_TEXT: &str` in the source")?;
    let end = source[start..]
        .find('"')
        .ok_or("the weights' text does not end")?;
    Ok(source[start..start + end]
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '\\')
        .collect())
}

#[cfg(test)]
mod tests;
