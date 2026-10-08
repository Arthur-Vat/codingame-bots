//! Self-play games between two searches, recording each searched
//! position's root visits: what a search longer than a turn's prefers.

use cg_core::rng::Rng;
use cg_search::{Budget, Mcts};
use uttt_engine::{Board, MoveList, Status};

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
}

impl SelfPlay {
    /// Plays one game from `seed`. Each side keeps its own search tree
    /// from move to move, as the bot does. Positions with a move that wins
    /// the game at once are not recorded: playouts play that move anyway.
    pub fn play(&self, seed: u64) -> GameRecord {
        let mut rng = Rng::new(seed);
        let mut searches = [
            Mcts::new(self.exploration, rng.next_u64()),
            Mcts::new(self.exploration, rng.next_u64()),
        ];
        let mut board = Board::new();
        let mut record = GameRecord::default();
        let mut moves = MoveList::new();
        while board.status() == Status::Ongoing {
            let mv = if record.moves.len() < self.opening_plies as usize {
                board.random_move(&mut rng)
            } else {
                board.legal_moves(&mut moves);
                let search = &mut searches[board.to_move()];
                let result = search.search(&board, &moves, Budget::Iterations(self.iterations));
                if board.game_winning_move().is_none() && result.iterations > 0 {
                    record.searched.push(Searched {
                        ply: record.moves.len() as u8,
                        visits: search.root_visits().collect(),
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

#[cfg(test)]
mod tests;
