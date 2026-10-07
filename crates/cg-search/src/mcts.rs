//! Monte Carlo tree search with the UCT selection rule.
//!
//! Each iteration walks down the tree from the root, at each node taking
//! the child with the best upper confidence bound (UCB1), until it reaches a
//! child never tried. It plays the game to the end from there (the
//! playout), then adds the result to every node on the way. A node gets its
//! children the second time an iteration reaches it, all at once, in random
//! order. The move played is the root child tried most often, except that
//! a move winning the game at once is played without searching.
//!
//! The tree lives in one vector, children of a node next to each other, and
//! is rebuilt for every search.

use cg_core::rng::Rng;

use crate::budget::Budget;
use crate::game::Game;

/// One position of the tree, reached by `mv`.
#[derive(Clone, Debug)]
struct Node<M> {
    /// The move that leads here; unused at the root.
    mv: M,
    /// The player who played `mv`, whose point of view `total` takes.
    mover: u8,
    /// Whether the children have been created; a node of a finished game
    /// has none.
    expanded: bool,
    first_child: u32,
    children: u32,
    visits: u32,
    /// Sum of the mover's scores over the visits.
    total: f64,
}

/// What a search found.
#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult<M> {
    /// The root move tried most often.
    pub best: M,
    /// Iterations run.
    pub iterations: u64,
    /// The average score of `best` for the player to move, from 0 to 1.
    pub expected_score: f64,
    /// Nodes in the tree.
    pub nodes: usize,
}

/// A Monte Carlo tree search, reusable from turn to turn.
pub struct Mcts<G: Game> {
    /// The exploration constant `c` of UCB1: a child's bound is its average
    /// score plus `c · sqrt(ln(parent visits) / visits)`.
    pub exploration: f64,
    rng: Rng,
    nodes: Vec<Node<G::Move>>,
    moves: Vec<G::Move>,
    path: Vec<u32>,
}

impl<G: Game> Mcts<G> {
    pub fn new(exploration: f64, seed: u64) -> Self {
        Mcts {
            exploration,
            rng: Rng::new(seed),
            nodes: Vec::new(),
            moves: Vec::new(),
            path: Vec::new(),
        }
    }

    /// Searches `root` and returns the best of `candidates`, which must be
    /// legal there (CodinGame's list, for example). Returns a candidate that
    /// wins at once without searching; otherwise runs at least one
    /// iteration per candidate, then until `budget` is spent.
    ///
    /// # Panics
    ///
    /// If `candidates` is empty or the game is over.
    pub fn search(
        &mut self,
        root: &G,
        candidates: &[G::Move],
        budget: Budget,
    ) -> SearchResult<G::Move> {
        assert!(!candidates.is_empty(), "nothing to choose from");
        assert!(root.score().is_none(), "the game is over");
        let win = if root.to_move() == 0 { 1.0 } else { 0.0 };
        for &mv in candidates {
            let mut next = root.clone();
            next.play(mv);
            if next.score() == Some(win) {
                return SearchResult {
                    best: mv,
                    iterations: 0,
                    expected_score: 1.0,
                    nodes: 0,
                };
            }
        }

        self.nodes.clear();
        let mover = (1 - root.to_move()) as u8;
        self.nodes.push(Node {
            mv: candidates[0],
            mover,
            expanded: false,
            first_child: 0,
            children: 0,
            visits: 0,
            total: 0.0,
        });
        self.moves.clear();
        self.moves.extend_from_slice(candidates);
        self.expand(0, root.to_move());

        let mut iterations = 0;
        while (iterations as usize) < candidates.len() || !budget.is_spent(iterations) {
            self.iterate(root);
            iterations += 1;
        }

        let root_node = &self.nodes[0];
        let children = root_node.first_child..root_node.first_child + root_node.children;
        let best = children
            .map(|index| &self.nodes[index as usize])
            .max_by_key(|child| child.visits)
            .expect("the root has children");
        SearchResult {
            best: best.mv,
            iterations,
            expected_score: best.total / f64::from(best.visits.max(1)),
            nodes: self.nodes.len(),
        }
    }

    /// One iteration: selection, expansion, playout, backpropagation.
    fn iterate(&mut self, root: &G) {
        let mut state = root.clone();
        let mut node = 0;
        self.path.clear();
        self.path.push(0);
        let score = loop {
            if let Some(score) = state.score() {
                break score;
            }
            if !self.nodes[node].expanded {
                if self.nodes[node].visits == 0 {
                    // A new leaf: estimate it with one playout.
                    break state.playout(&mut self.rng);
                }
                state.legal_moves(&mut self.moves);
                self.expand(node, state.to_move());
            }
            node = self.select(node);
            state.play(self.nodes[node].mv);
            self.path.push(node as u32);
        };
        for &index in &self.path {
            let node = &mut self.nodes[index as usize];
            node.visits += 1;
            node.total += if node.mover == 0 { score } else { 1.0 - score };
        }
    }

    /// Gives `node` one child per move in `self.moves`, in random order.
    fn expand(&mut self, node: usize, to_move: usize) {
        self.rng.shuffle(&mut self.moves);
        let first_child = self.nodes.len() as u32;
        for &mv in &self.moves {
            self.nodes.push(Node {
                mv,
                mover: to_move as u8,
                expanded: false,
                first_child: 0,
                children: 0,
                visits: 0,
                total: 0.0,
            });
        }
        let parent = &mut self.nodes[node];
        parent.expanded = true;
        parent.first_child = first_child;
        parent.children = self.moves.len() as u32;
    }

    /// The child of `node` with the highest UCB1 bound; an untried child
    /// first.
    fn select(&self, node: usize) -> usize {
        let parent = &self.nodes[node];
        let first = parent.first_child as usize;
        let children = &self.nodes[first..first + parent.children as usize];
        let log_visits = f64::from(parent.visits.max(1)).ln();
        let mut best = 0;
        let mut best_bound = f64::NEG_INFINITY;
        for (offset, child) in children.iter().enumerate() {
            if child.visits == 0 {
                return first + offset;
            }
            let visits = f64::from(child.visits);
            let bound = child.total / visits + self.exploration * (log_visits / visits).sqrt();
            if bound > best_bound {
                best_bound = bound;
                best = offset;
            }
        }
        first + best
    }
}

#[cfg(test)]
mod tests;
