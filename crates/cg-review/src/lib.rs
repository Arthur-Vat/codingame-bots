//! Reviews of recorded games, shared by every game.
//!
//! A game's review binary (for example `games/ice-and-fire/review`) replays
//! a record with its engine and fills a [`trace::Trace`]: the board and both
//! sides' metrics at every ply, the phases and the decisive moments. This
//! crate does the rest, the same for every game:
//!
//! - [`text`]: the game file a reviewer reads (a Markdown page with every
//!   ply's board, metrics and commands) and the metrics as CSV;
//! - [`detect`]: decisive moments found from the metrics alone (large
//!   drops, lead changes and swings);
//! - [`summary`]: a batch's summary, from the files [`text`] wrote and the
//!   reviewers' notes: which metrics, at which point of the game, go with
//!   the winner, how often each kind of decisive moment decided the game,
//!   and whether the plays the reviewers judged good or bad moved the lead
//!   the way they said;
//! - [`excerpt`]: excerpt reviews (ADR 0044): each side's last few moves up
//!   to a game's decisive moment, a primer shared by the excerpts, a row per
//!   game, and the counts of the reviewers' judgments.

pub mod detect;
pub mod excerpt;
pub mod summary;
pub mod text;
pub mod trace;

#[cfg(test)]
mod test_util;
