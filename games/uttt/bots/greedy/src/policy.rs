//! How the greedy bot scores a move: by what it does at once, and by the
//! best immediate answer it leaves the opponent.

use uttt_engine::{Board, Move, MoveList, Status};

/// Winning the game.
const WIN_GAME: i32 = 1_000_000;
/// Winning a small board.
const WIN_BOARD: i32 = 100;
/// Letting the opponent win a small board on its next move.
const GIVE_BOARD: i32 = 80;
/// Sending the opponent to a closed board, so it may play anywhere.
const GIVE_FREE_CHOICE: i32 = 20;
/// Taking a cell that sits on several lines of its small board.
const CELL_VALUE: [i32; 9] = [3, 2, 3, 2, 4, 2, 3, 2, 3];

/// The score of `mv` for the player to move in `board`; higher is better.
/// `mv` must be legal.
pub fn score(board: &Board, mv: Move) -> i32 {
    let me = board.to_move();
    let mut after = *board;
    after.play(mv);
    match after.status() {
        Status::Win(seat) if seat == me => return WIN_GAME,
        Status::Win(_) => return -WIN_GAME,
        Status::Draw => return 0,
        Status::Ongoing => {}
    }

    let mut score = CELL_VALUE[mv.cell()];
    if after.small_winner(mv.board()) == Some(me) {
        score += WIN_BOARD;
    }
    if after.target().is_none() {
        score -= GIVE_FREE_CHOICE;
    }
    score - best_reply_threat(&after, me)
}

/// How bad the opponent's best immediate reply is for `me`.
fn best_reply_threat(after: &Board, me: usize) -> i32 {
    let mut replies = MoveList::new();
    after.legal_moves(&mut replies);
    let mut threat = 0;
    for &reply in replies.iter() {
        let mut next = *after;
        next.play(reply);
        match next.status() {
            Status::Win(seat) if seat != me => return WIN_GAME / 2,
            _ => {}
        }
        if next
            .small_winner(reply.board())
            .is_some_and(|seat| seat != me)
        {
            threat = GIVE_BOARD;
        }
    }
    threat
}

#[cfg(test)]
mod tests;
