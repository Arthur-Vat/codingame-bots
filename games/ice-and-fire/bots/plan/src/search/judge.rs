//! The judge's view of a search (analysis build only): how the search
//! scores a plan it did not find, so that the plan a top player played can
//! be compared with the plan the search chose.
//!
//! A plan's score for the final choice is `Search::final_score`: the worst
//! evaluation over the search's memory of replies and the plan's own replies
//! (E027). The plan the search chose gets that score as it stands. A played
//! plan is scored as a candidate would be: its commands applied in order
//! (those the rules accept) to the root, the opponent's replies to its end
//! state searched (`Search::answers`, which has no deadline here), and the
//! score taken against the final memory and those replies.

use std::cell::RefCell;

use ice_and_fire_engine::{Command, State};

use super::{Plan, Reply, Search};

thread_local! {
    /// The plan to score on this thread's next search, if any.
    static PLAYED: RefCell<Option<Vec<Command>>> = const { RefCell::new(None) };
    /// What the last search scored.
    static JUDGED: RefCell<Option<Judged>> = const { RefCell::new(None) };
}

/// The scores of one search.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Judged {
    /// The score of the plan the search chose.
    pub ours: i64,
    /// The score of the played plan, if one was given.
    pub played: Option<i64>,
}

/// Sets the played plan the next search on this thread scores.
pub fn set_played(commands: Option<Vec<Command>>) {
    PLAYED.with(|p| *p.borrow_mut() = commands);
    JUDGED.with(|j| *j.borrow_mut() = None);
}

/// What the last search on this thread scored, `None` if it did not get to
/// the final choice (a lethal plan, or an opening).
pub fn take() -> Option<Judged> {
    JUDGED.with(|j| j.borrow_mut().take())
}

/// Called by `think` at the final choice: `best` is the chosen plan's score
/// and index.
pub(super) fn observe(search: &Search, root: &State, memory: &[Reply], best: Option<(i64, usize)>) {
    let Some((ours, _)) = best else {
        return;
    };
    let played = PLAYED.with(|p| p.borrow().clone()).map(|commands| {
        let mut end = root.clone();
        let accepted: Vec<Command> = commands.into_iter().filter(|&c| end.apply(c)).collect();
        let own = search.answers(&end, &accepted);
        let plan = Plan {
            commands: accepted,
            end,
            first: None,
            own: Some(own),
        };
        search.final_score(&plan, memory)
    });
    JUDGED.with(|j| *j.borrow_mut() = Some(Judged { ours, played }));
}
