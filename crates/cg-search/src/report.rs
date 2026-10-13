//! What a search knows of its root, for analysis (cargo feature `analysis`).
//!
//! The types here are read from [`Mcts::report`](crate::Mcts::report) after
//! a search or [`Mcts::think`](crate::Mcts::think). Reading them changes
//! nothing in the search.

/// What the search knows of one root move, from the point of view of the
/// player to move at the root.
#[derive(Clone, Debug, PartialEq)]
pub struct MoveReport<M> {
    pub mv: M,
    pub visits: u32,
    /// Average score for the player to move: 1 a win, 0.5 a draw, 0 a loss.
    /// Exactly 1 or 0 when the move is proven.
    pub score: f64,
    /// Share of the visits' results that were draws (a result is a draw
    /// when its score is exactly 0.5); 0 when proven. Always `Some`: the
    /// search counts draws whenever it can report.
    pub draws: Option<f64>,
    /// `Some(true)` when the move is proven to win for the player to move,
    /// `Some(false)` proven to lose, `None` when not proven.
    pub proven: Option<bool>,
    /// The move, then the reply the search would choose at each ply below
    /// it (a proven win, else the most visited not proven lost, else the
    /// most visited; so a move that loses is followed by its refutation),
    /// while the reply has visits, up to `depth` moves in all.
    pub line: Vec<M>,
}

/// What the search knows of the root, after a search or `think`.
#[derive(Clone, Debug, PartialEq)]
pub struct Report<M> {
    /// The root's visits, earlier searches' included.
    pub visits: u64,
    /// Nodes in the tree.
    pub nodes: usize,
    /// `Some(true)` when the player to move is proven to win, `Some(false)`
    /// proven to lose, `None` when not proven.
    pub proven: Option<bool>,
    /// Every root move: first the one `search` would choose (a proven win,
    /// else the most visited not proven lost, else the most visited), then
    /// the others by visits, most first (ties in tree order). The lines
    /// below each move follow the same choice at every ply.
    pub moves: Vec<MoveReport<M>>,
}
