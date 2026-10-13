//! What the search can tell of its root, and thinking on one position: the
//! parts of [`Mcts`] that only an analysis uses, compiled only with the
//! crate's `analysis` feature. Bots do not use it and the bundler drops it
//! with the lines of the search that refer to it, so a bot's search is the
//! same code as without this module.
//!
//! Everything here reads the tree or runs the search's own iterations; none
//! of it changes the random stream or what `search` does. The search counts
//! the draws among its results (a result is a draw when its score is
//! exactly 0.5) in a number kept by each node, updated like its value.

use super::{Mcts, Proof};
use crate::budget::Budget;
use crate::game::Game;
use crate::report::{MoveReport, Report};

/// Adds one result, of player 0's `score`, to a node's average of the draw
/// indicator; `visits` counts it.
#[inline]
pub(super) fn record_draw(draws: &mut f32, score: f64, visits: f32) {
    *draws += (f32::from(u8::from(score == 0.5)) - *draws) / visits;
}

impl<G: Game> Mcts<G> {
    /// Runs iterations on `root` with all its legal moves until `budget` is
    /// spent or the root is proven, keeping and reusing the tree exactly as
    /// [`Mcts::search`] does (a root equal to the previous one continues its
    /// tree, so calling `think` again keeps thinking). Unlike `search`, it
    /// never returns early on a move that wins at once and needs no minimum
    /// of iterations. Returns the iterations run by this call.
    ///
    /// # Panics
    ///
    /// If the game is over.
    pub fn think(&mut self, root: &G, budget: Budget) -> u64 {
        assert!(root.score().is_none(), "the game is over");
        let mut moves = Vec::new();
        root.legal_moves(&mut moves);
        if !self.reuse(root, &moves) {
            self.start(root, &moves);
        }
        self.root = Some(root.clone());
        self.set_current(true);
        let mut iterations = 0;
        while self.nodes[0].proof == Proof::Unknown && !budget.is_spent(iterations) {
            self.iterate(root);
            iterations += 1;
        }
        iterations
    }

    /// Records whether the tree is at the root of the last search or `think`
    /// (what `report` describes). The search calls it, in lines marked for
    /// the analysis feature, because an attribute cannot mark an assignment.
    pub(super) fn set_current(&mut self, current: bool) {
        self.current = current;
    }

    /// The root's report, with lines of at most `depth` moves; `None` before
    /// any search or [`Mcts::think`] ran iterations, and after a `search`
    /// that returned a winning move at once, which leaves the tree as it
    /// was (the report would describe an earlier root).
    pub fn report(&self, depth: usize) -> Option<Report<G::Move>> {
        if !self.current || self.nodes.is_empty() || self.nodes[0].visits == 0 {
            return None;
        }
        let best = self.choose(0).expect("the root has children");
        let mut others: Vec<usize> = self.children(0).filter(|&index| index != best).collect();
        // Stable: ties stay in tree order.
        others.sort_by_key(|&index| std::cmp::Reverse(self.nodes[index].visits));
        let moves = std::iter::once(best)
            .chain(others)
            .map(|index| self.move_report(index, depth))
            .collect();
        Some(Report {
            visits: u64::from(self.nodes[0].visits),
            nodes: self.nodes.len(),
            proven: self.proven(),
            moves,
        })
    }

    /// The report of the root move held by node `index`.
    fn move_report(&self, index: usize, depth: usize) -> MoveReport<G::Move> {
        let child = &self.nodes[index];
        let (score, draws, proven) = match child.proof {
            Proof::MoverWins => (1.0, 0.0, Some(true)),
            Proof::MoverLoses => (0.0, 0.0, Some(false)),
            Proof::Unknown => (f64::from(child.value), f64::from(child.draws), None),
        };
        let mut line = Vec::new();
        let mut current = index;
        while line.len() < depth {
            line.push(self.nodes[current].mv);
            // The reply `search` would choose, if it has visits: a proven
            // win, which is the refutation of a move that loses.
            match self.choose(current) {
                Some(reply) if self.nodes[reply].visits > 0 => current = reply,
                _ => break,
            }
        }
        MoveReport {
            mv: child.mv,
            visits: child.visits,
            score,
            draws: Some(draws),
            proven,
            line,
        }
    }

    /// The child of `node` that a player there would choose, as `search`
    /// chooses at the root: a proven win, else the most tried not proven
    /// lost, else the most tried. `None` when `node` has no children.
    pub(super) fn choose(&self, node: usize) -> Option<usize> {
        let children = || self.children(node);
        children()
            .find(|&index| self.nodes[index].proof == Proof::MoverWins)
            .or_else(|| {
                children()
                    .filter(|&index| self.nodes[index].proof != Proof::MoverLoses)
                    .max_by_key(|&index| self.nodes[index].visits)
            })
            .or_else(|| children().max_by_key(|&index| self.nodes[index].visits))
    }

    /// `Some(true)` when the player to move at the root is proven to win,
    /// `Some(false)` to lose.
    fn proven(&self) -> Option<bool> {
        match self.nodes[0].proof {
            Proof::Unknown => None,
            // The root's mover is the opponent of the player to move.
            Proof::MoverWins => Some(false),
            Proof::MoverLoses => Some(true),
        }
    }
}
