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
//! The tree lives in one vector, children of a node next to each other. It
//! is kept from one search to the next: when the new position is one or two
//! moves below the previous root (the bot's move, then the opponent's), the
//! subtree of that position becomes the new tree, with the visits it
//! already has; otherwise the search starts afresh.
//!
//! The search also proves wins and losses (MCTS-Solver): a position where
//! the game is won or lost is proven, a node is lost for the player who
//! moved into it when the opponent has a proven winning answer, and won
//! when every answer is proven lost. Iterations that reach a proven node
//! use its exact result, selection never picks an answer proven lost when
//! another exists, a proven win at the root is played at once, and the
//! search stops when the root is proven. Draws are not proven.

use std::sync::OnceLock;

use cg_core::rng::Rng;

use crate::budget::Budget;
use crate::game::Game;

/// Visit counts below this have `sqrt(ln(visits))` in a table.
const SQRT_LN_TABLE_LEN: usize = 1 << 16;

/// `sqrt(ln(n))` for every `n` below `SQRT_LN_TABLE_LEN` (`n = 0` counts as
/// 1), computed exactly as selection would, once for all searches: the
/// logarithm was a large part of selection's time.
fn sqrt_ln_table() -> &'static [f64] {
    static TABLE: OnceLock<Vec<f64>> = OnceLock::new();
    TABLE.get_or_init(|| {
        (0..SQRT_LN_TABLE_LEN as u32)
            .map(|n| f64::from(n.max(1)).ln().sqrt())
            .collect()
    })
}

/// What is proven about a node, for the player who moved into it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Proof {
    Unknown,
    /// The mover wins whatever the opponent does.
    MoverWins,
    /// The opponent has a winning answer.
    MoverLoses,
}

/// One position of the tree, reached by `mv`.
#[derive(Clone, Copy, Debug)]
struct Node<M> {
    /// The move that leads here; unused at the root.
    mv: M,
    /// The player who played `mv`, whose point of view `value` and `proof`
    /// take.
    mover: u8,
    proof: Proof,
    /// Whether the children have been created; a node of a finished game
    /// has none.
    expanded: bool,
    first_child: u32,
    children: u32,
    visits: u32,
    /// The mover's average score over the visits.
    value: f32,
    /// `1 / sqrt(visits)`, kept so that selection, which reads every child
    /// of a node, needs no division or square root.
    inv_sqrt_visits: f32,
}

/// What a search found.
#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult<M> {
    /// The root move tried most often.
    pub best: M,
    /// Iterations run by this search.
    pub iterations: u64,
    /// Visits the root already had from earlier searches.
    pub reused: u64,
    /// `Some(true)` when the player to move is proven to win, `Some(false)`
    /// when proven to lose, `None` when not proven.
    pub proven: Option<bool>,
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
    /// The position at the root of `nodes`, once a search has run.
    root: Option<G>,
    /// Spare storage for moving a subtree to the front, kept to avoid
    /// allocating every turn.
    spare: Vec<Node<G::Move>>,
    origin: Vec<u32>,
    moves: Vec<G::Move>,
    path: Vec<u32>,
    /// [`sqrt_ln_table`], shared by all searches.
    sqrt_ln: &'static [f64],
}

impl<G: Game> Mcts<G> {
    pub fn new(exploration: f64, seed: u64) -> Self {
        Mcts {
            exploration,
            rng: Rng::new(seed),
            nodes: Vec::new(),
            root: None,
            spare: Vec::new(),
            origin: Vec::new(),
            moves: Vec::new(),
            path: Vec::new(),
            sqrt_ln: sqrt_ln_table(),
        }
    }

    /// Searches `root` and returns the best of `candidates`, which must be
    /// legal there (CodinGame's list, for example). Returns a candidate that
    /// wins at once without searching; otherwise keeps what earlier
    /// searches learned about `root`, runs at least one iteration per
    /// candidate, then runs until `budget` is spent.
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
                    reused: 0,
                    proven: Some(true),
                    expected_score: 1.0,
                    nodes: self.nodes.len(),
                };
            }
        }

        if !self.reuse(root, candidates) {
            self.start(root, candidates);
        }
        self.root = Some(root.clone());
        let reused = u64::from(self.nodes[0].visits);

        let mut iterations = 0;
        while self.nodes[0].proof == Proof::Unknown
            && ((iterations as usize) < candidates.len() || !budget.is_spent(iterations))
        {
            self.iterate(root);
            iterations += 1;
        }

        // A proven win, else the most tried answer not proven lost.
        let children = || self.children(0).map(|index| &self.nodes[index]);
        let best = children()
            .find(|child| child.proof == Proof::MoverWins)
            .or_else(|| {
                children()
                    .filter(|child| child.proof != Proof::MoverLoses)
                    .max_by_key(|child| child.visits)
            })
            .or_else(|| children().max_by_key(|child| child.visits))
            .expect("the root has children");
        let proven = match self.nodes[0].proof {
            Proof::Unknown => None,
            // The root's mover is the opponent of the player to move.
            Proof::MoverWins => Some(false),
            Proof::MoverLoses => Some(true),
        };
        let expected_score = match best.proof {
            Proof::MoverWins => 1.0,
            Proof::MoverLoses => 0.0,
            Proof::Unknown => f64::from(best.value),
        };
        SearchResult {
            best: best.mv,
            iterations,
            reused,
            proven,
            expected_score,
            nodes: self.nodes.len(),
        }
    }

    /// The root's moves and their visits after the last search that ran
    /// iterations, earlier searches' visits included; nothing before the
    /// first search. A search that returns a winning move at once leaves
    /// the tree as it was.
    pub fn root_visits(&self) -> impl Iterator<Item = (G::Move, u32)> + '_ {
        let children = if self.nodes.is_empty() {
            0..0
        } else {
            self.children(0)
        };
        children.map(|index| (self.nodes[index].mv, self.nodes[index].visits))
    }

    /// A fresh tree: the root and one child per candidate.
    fn start(&mut self, root: &G, candidates: &[G::Move]) {
        self.nodes.clear();
        self.nodes.push(Node {
            mv: candidates[0],
            mover: (1 - root.to_move()) as u8,
            proof: Proof::Unknown,
            expanded: false,
            first_child: 0,
            children: 0,
            visits: 0,
            value: 0.0,
            inv_sqrt_visits: 0.0,
        });
        self.moves.clear();
        self.moves.extend_from_slice(candidates);
        self.expand(0, root.to_move());
    }

    /// Makes the node of `root`, if the tree holds it within two moves of
    /// its current root, the new root, and drops the rest of the tree.
    /// Returns whether it did. The kept root must offer exactly
    /// `candidates`.
    fn reuse(&mut self, root: &G, candidates: &[G::Move]) -> bool {
        let Some(node) = self.find(root) else {
            return false;
        };
        let kept = &self.nodes[node];
        let same_moves = kept.expanded
            && kept.children as usize == candidates.len()
            && self
                .children(node)
                .all(|child| candidates.contains(&self.nodes[child].mv));
        if !same_moves {
            return false;
        }
        if node != 0 {
            self.keep_subtree(node);
        }
        true
    }

    /// The node holding `position`: the root or a node one or two moves
    /// below it.
    fn find(&self, position: &G) -> Option<usize> {
        let previous = self.root.as_ref()?;
        if previous == position {
            return Some(0);
        }
        for child in self.children(0) {
            let mut after_one = previous.clone();
            after_one.play(self.nodes[child].mv);
            if &after_one == position {
                return Some(child);
            }
            for grandchild in self.children(child) {
                let mut after_two = after_one.clone();
                after_two.play(self.nodes[grandchild].mv);
                if &after_two == position {
                    return Some(grandchild);
                }
            }
        }
        None
    }

    /// Moves the subtree of `node` to the front of the tree, `node` first,
    /// and drops every other node.
    fn keep_subtree(&mut self, node: usize) {
        // The two vectors take turns holding the tree. Giving the spare one
        // the tree's capacity, in a fresh allocation, means that neither
        // has to grow during a later search: growing copies the whole tree,
        // a pause of several milliseconds once it holds a million nodes.
        let capacity = self.nodes.capacity();
        if self.spare.capacity() < capacity {
            self.spare = Vec::with_capacity(capacity);
        }
        if self.origin.capacity() < capacity {
            self.origin = Vec::with_capacity(capacity);
        }
        self.spare.clear();
        self.origin.clear();
        self.spare.push(self.nodes[node]);
        self.origin.push(node as u32);
        let mut next = 0;
        while next < self.spare.len() {
            let old = self.nodes[self.origin[next] as usize];
            if old.expanded {
                self.spare[next].first_child = self.spare.len() as u32;
                for child in old.first_child..old.first_child + old.children {
                    self.spare.push(self.nodes[child as usize]);
                    self.origin.push(child);
                }
            }
            next += 1;
        }
        std::mem::swap(&mut self.nodes, &mut self.spare);
    }

    /// The indices of the children of `node`.
    fn children(&self, node: usize) -> std::ops::Range<usize> {
        let node = &self.nodes[node];
        let first = node.first_child as usize;
        first
            ..first
                + if node.expanded {
                    node.children as usize
                } else {
                    0
                }
    }

    /// One iteration: selection, expansion, playout, backpropagation, and
    /// proofs.
    fn iterate(&mut self, root: &G) {
        let mut state = root.clone();
        let mut node = 0;
        let mut proved = false;
        self.path.clear();
        self.path.push(0);
        let score = loop {
            let current = self.nodes[node];
            let mover_score = |wins: bool| {
                if wins == (current.mover == 0) {
                    1.0
                } else {
                    0.0
                }
            };
            match current.proof {
                Proof::MoverWins => break mover_score(true),
                Proof::MoverLoses => break mover_score(false),
                Proof::Unknown => {}
            }
            if let Some(score) = state.score() {
                if node != 0 && score != 0.5 {
                    let wins = score == mover_score(true);
                    self.nodes[node].proof = if wins {
                        Proof::MoverWins
                    } else {
                        Proof::MoverLoses
                    };
                    proved = true;
                }
                break score;
            }
            if !current.expanded {
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
            let mover_score = if node.mover == 0 { score } else { 1.0 - score };
            let visits = node.visits as f32;
            node.value += (mover_score as f32 - node.value) / visits;
            node.inv_sqrt_visits = visits.sqrt().recip();
        }
        if proved {
            self.prove_ancestors();
        }
    }

    /// Proves the nodes on the current path, from the leaf up, as far as
    /// their children allow: a node whose opponent has a proven winning
    /// answer is lost for its mover, and one whose answers are all proven
    /// lost is won.
    fn prove_ancestors(&mut self) {
        for depth in (0..self.path.len() - 1).rev() {
            let node = self.path[depth] as usize;
            let mut all_lost = true;
            let mut proof = Proof::Unknown;
            for child in self.children(node) {
                match self.nodes[child].proof {
                    Proof::MoverWins => {
                        proof = Proof::MoverLoses;
                        break;
                    }
                    Proof::MoverLoses => {}
                    Proof::Unknown => all_lost = false,
                }
            }
            if proof == Proof::Unknown && all_lost {
                proof = Proof::MoverWins;
            }
            if proof == Proof::Unknown {
                return;
            }
            self.nodes[node].proof = proof;
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
                proof: Proof::Unknown,
                expanded: false,
                first_child: 0,
                children: 0,
                visits: 0,
                value: 0.0,
                inv_sqrt_visits: 0.0,
            });
        }
        let parent = &mut self.nodes[node];
        parent.expanded = true;
        parent.first_child = first_child;
        parent.children = self.moves.len() as u32;
    }

    /// The child of `node` with the highest UCB1 bound, an untried child
    /// first, never a child proven lost for the player choosing unless all
    /// are.
    fn select(&self, node: usize) -> usize {
        let parent = &self.nodes[node];
        let first = parent.first_child as usize;
        let children = &self.nodes[first..first + parent.children as usize];
        // UCB1: value + c * sqrt(ln(parent visits) / visits).
        let visits = parent.visits.max(1);
        let sqrt_ln = match self.sqrt_ln.get(visits as usize) {
            Some(&value) => value,
            None => f64::from(visits).ln().sqrt(),
        };
        let spread = (self.exploration * sqrt_ln) as f32;
        let mut best = 0;
        let mut best_bound = f32::NEG_INFINITY;
        for (offset, child) in children.iter().enumerate() {
            match child.proof {
                Proof::MoverLoses => continue,
                Proof::MoverWins => return first + offset,
                Proof::Unknown => {}
            }
            if child.visits == 0 {
                return first + offset;
            }
            let bound = child.value + spread * child.inv_sqrt_visits;
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
