//! Self-play games between two searches, recording at each searched
//! position what a search longer than a turn's found: its root score and,
//! if wanted, the visits of each move.

use cg_core::rng::Rng;
use cg_search::{Budget, Game, Mcts};
use uttt_engine::nnue::Network;
use uttt_engine::search::NnueBoard;
use uttt_engine::{Board, Move, Status};
use uttt_policy::{PatternBoard, PatternPolicy, PlayoutPolicy, PolicyBoard};

use crate::data::{GameRecord, Searched};

/// How the first moves of a game are chosen, for variety.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opening {
    /// Legal moves drawn uniformly, without searching; their positions are
    /// not recorded.
    Random,
    /// Searched and recorded as every later move, but drawn at random in
    /// proportion to the root's visits instead of the most visited, as
    /// AlphaZero does; a move that wins the game is still played. The draw
    /// ignores proofs, which searches rarely reach in an opening, so it may
    /// pick a move proven lost.
    Sampled,
}

/// How self-play games are played.
#[derive(Clone, Copy, Debug)]
pub struct SelfPlay {
    /// Search iterations per move.
    pub iterations: u64,
    /// Moves at the start of each game chosen by `opening`.
    pub opening_plies: u32,
    pub opening: Opening,
    /// Drops a game whose first search after the opening expects a score
    /// above this for the side to move, or below 1 minus this: one side
    /// is then nearly sure to win already.
    pub balance: Option<f64>,
    /// The search's exploration constant, as in the bot.
    pub exploration: f64,
    /// Whether the search creates all the children of a node at once and
    /// backs up their minimax value, as the bot does since E018; it needs
    /// `network` to estimate them.
    pub expand_all: bool,
    /// The weight of a child's minimax value in selection, as in the bot
    /// (E020).
    pub minimax_weight: f64,
    /// The NNUE that judges the search's new nodes instead of playouts, as
    /// in the bot since E017; it replaces `policy` and `patterns`.
    pub network: Option<&'static Network<256>>,
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
    /// Plays one game from `seed`, or `None` if `balance` drops it. Each
    /// side keeps its own search tree from move to move, as the bot does.
    /// Positions with a move that wins the game at once are not recorded:
    /// the search plays that move without searching.
    pub fn play(&self, seed: u64) -> Option<GameRecord> {
        match (self.network, self.patterns, self.policy) {
            (Some(network), _, _) => {
                self.play_as(seed, false, |board| NnueBoard::new(board, network))
            }
            (None, Some(policy), _) => {
                self.play_as(seed, true, |board| PatternBoard { board, policy })
            }
            (None, None, Some(policy)) => {
                self.play_as(seed, false, |board| PolicyBoard { board, policy })
            }
            (None, None, None) => self.play_as(seed, false, |board| board),
        }
    }

    /// Plays one game, searching `position(board)` at each move, with the
    /// game's priors if `priors`.
    fn play_as<G: Game<Move = Move>>(
        &self,
        seed: u64,
        priors: bool,
        position: impl Fn(Board) -> G,
    ) -> Option<GameRecord> {
        let mut rng = Rng::new(seed);
        let mut searches = [
            Mcts::new(self.exploration, rng.next_u64()),
            Mcts::new(self.exploration, rng.next_u64()),
        ];
        for search in &mut searches {
            search.priors = priors;
            search.expand_all = self.expand_all;
            search.minimax_weight = self.minimax_weight;
        }
        let mut board = Board::new();
        let mut record = GameRecord::default();
        let mut moves = Vec::new();
        while board.status() == Status::Ongoing {
            let ply = record.moves.len();
            let opening = ply < self.opening_plies as usize;
            let mv = if opening && self.opening == Opening::Random {
                board.random_move(&mut rng)
            } else {
                let root = position(board);
                root.legal_moves(&mut moves);
                let search = &mut searches[board.to_move()];
                let result = search.search(&root, &moves, Budget::Iterations(self.iterations));
                if let Some(balance) = self.balance {
                    let score = result.expected_score;
                    let first = ply == self.opening_plies as usize;
                    if first && result.iterations > 0 && (score > balance || score < 1.0 - balance)
                    {
                        return None;
                    }
                }
                let mv = if opening && result.iterations > 0 {
                    sample(search.root_visits(), &mut rng).unwrap_or(result.best)
                } else {
                    result.best
                };
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
                mv
            };
            record.moves.push(mv);
            board.play(mv);
        }
        Some(record)
    }
}

/// A move drawn in proportion to its visits, or `None` without visits.
fn sample(visits: impl Iterator<Item = (Move, u32)>, rng: &mut Rng) -> Option<Move> {
    let visits: Vec<(Move, u32)> = visits.collect();
    let total: u64 = visits.iter().map(|&(_, n)| u64::from(n)).sum();
    if total == 0 {
        return None;
    }
    let mut left = rng.below(total);
    for (mv, n) in visits {
        if left < u64::from(n) {
            return Some(mv);
        }
        left -= u64::from(n);
    }
    unreachable!("the draw is below the total")
}

/// Reads the playout weights from Rust source written by `uttt-trainer
/// fit-policy`, such as `games/uttt/training/class_weights.rs`.
pub fn read_policy_weights(source: &str) -> Result<[u32; uttt_policy::CLASSES], String> {
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
        .map_err(|_| format!("{count} weights instead of {}", uttt_policy::CLASSES))
}

/// The text between the quotes of the constant `name` in Rust source,
/// as written.
fn read_constant(source: &str, name: &str) -> Option<String> {
    let start = source
        .find(&format!("{name}: &str"))
        .and_then(|at| source[at..].find('"').map(|offset| at + offset + 1))?;
    let end = source[start..].find('"')?;
    Some(source[start..start + end].to_string())
}

/// Reads a pattern policy from Rust source written by `uttt-trainer
/// fit-patterns`, such as `games/uttt/training/pattern_weights.rs`: the packed
/// weights of `PATTERN_PACKED` or, in older files and releases, the base64
/// characters of `PATTERN_TEXT`. The policy draws every move of a playout.
/// The packed characters are read as they are: some of them are white space
/// to Unicode.
pub fn read_pattern_policy(source: &str) -> Result<PatternPolicy, String> {
    if let Some(packed) = read_constant(source, "PATTERN_PACKED") {
        let bytes = cg_pack::unpack(&packed).ok_or("the packed weights do not unpack")?;
        PatternPolicy::decode_packed(&bytes)
    } else if let Some(text) = read_constant(source, "PATTERN_TEXT") {
        // Base64 characters only: no line continuations or white space.
        let text: String = text
            .chars()
            .filter(|c| !c.is_whitespace() && *c != '\\')
            .collect();
        PatternPolicy::decode(&text)
    } else {
        Err("no `PATTERN_PACKED: &str` or `PATTERN_TEXT: &str` in the source".to_string())
    }
}

/// Reads the NNUE of `uttt_engine::nnue`, width 256, from Rust source
/// written by `uttt-trainer fit-nnue`, such as
/// `games/uttt/bots/mcts/src/nnue_weights.rs` (the bot's), or from a
/// release: the packed characters of `NNUE_WEIGHTS`.
pub fn read_network(source: &str) -> Result<Network<256>, String> {
    let packed =
        read_constant(source, "NNUE_WEIGHTS").ok_or("no `NNUE_WEIGHTS: &str` in the source")?;
    let bytes = cg_pack::unpack(&packed).ok_or("the packed weights do not unpack")?;
    Network::<256>::from_bytes(&bytes).map_err(|err| format!("not a network of width 256: {err}"))
}

#[cfg(test)]
mod tests;
