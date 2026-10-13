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
//!
//! A search may also use the game's priors ([`Game::priors`]): a node's
//! children are then tried in order of their priors, most promising first,
//! and selection adds `prior_weight · prior / (visits + 1)` to each
//! child's bound, a bias that fades as the child is visited (progressive
//! bias). Without priors, children are tried in random order.
//!
//! The search can describe what it knows without changing what it does:
//! [`Mcts::report`] gives every root move's visits, average score, proof
//! and most visited line ([`crate::report`]), and [`Mcts::think`] keeps
//! iterating on one position, with no minimum of iterations and without
//! playing a winning move at once, for an analysis that wants to see the
//! whole root. Counting how many results were draws costs a number per
//! node, so only a search built with [`Mcts::counting_draws`] pays for it;
//! a bot's [`Mcts::new`] does not, and both choose the same moves.

use std::marker::PhantomData;
use std::sync::OnceLock;

use cg_core::rng::Rng;

use crate::budget::Budget;
use crate::game::Game;
use crate::report::{MoveReport, Report};

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

/// Whether a search counts the draws among its results. A search that does
/// not ([`NoDraws`], the default) keeps nothing per node, so its tree is
/// as large as if the feature did not exist.
pub trait Draws {
    /// What a node keeps; nothing for [`NoDraws`].
    type Cell: Copy + Default + std::fmt::Debug;

    /// Adds one result to `cell`, which now holds `visits` of them.
    fn record(cell: &mut Self::Cell, draw: bool, visits: f32);

    /// The share of the results that were draws, if counted.
    fn share(cell: &Self::Cell) -> Option<f64>;
}

/// Does not count draws: no cost, for bots.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NoDraws;

impl Draws for NoDraws {
    type Cell = ();

    #[inline(always)]
    fn record(_cell: &mut (), _draw: bool, _visits: f32) {}

    fn share(_cell: &()) -> Option<f64> {
        None
    }
}

/// Counts draws: a result is a draw when its score is exactly 0.5. Each node
/// keeps the average of the draw indicator, updated like its value. It never
/// touches the random stream, so the search plays the same moves.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CountDraws;

impl Draws for CountDraws {
    type Cell = f32;

    #[inline]
    fn record(cell: &mut f32, draw: bool, visits: f32) {
        *cell += (f32::from(u8::from(draw)) - *cell) / visits;
    }

    fn share(cell: &f32) -> Option<f64> {
        Some(f64::from(*cell))
    }
}

/// One position of the tree, reached by `mv`. `C` is what the search keeps
/// to count draws ([`Draws::Cell`]).
#[derive(Clone, Copy, Debug)]
struct Node<M, C> {
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
    /// The share of the parent's priors that this move has; 0 without
    /// priors, which leaves no bias.
    prior: f32,
    /// `prior_weight · prior / (visits + 1)`, kept for selection.
    bias: f32,
    /// The share of the visits' results that were draws, if counted.
    draws: C,
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

/// A Monte Carlo tree search, reusable from turn to turn. `D` says whether
/// it counts draws: [`NoDraws`] by default, [`CountDraws`] to report them.
pub struct Mcts<G: Game, D: Draws = NoDraws> {
    /// The exploration constant `c` of UCB1: a child's bound is its average
    /// score plus `c · sqrt(ln(parent visits) / visits)`.
    pub exploration: f64,
    /// Whether to ask the game for priors when a node gets its children.
    pub priors: bool,
    /// The weight of the priors' bias in a child's bound; 0 for none.
    pub prior_weight: f64,
    rng: Rng,
    nodes: Vec<Node<G::Move, D::Cell>>,
    /// The position at the root of `nodes`, once a search has run.
    root: Option<G>,
    /// Whether the last search or `think` left the tree at its own root,
    /// which is what [`Mcts::report`] describes: a search that returns a
    /// winning move at once leaves the tree as it was, and clears it.
    current: bool,
    /// Spare storage for moving a subtree to the front, kept to avoid
    /// allocating every turn.
    spare: Vec<Node<G::Move, D::Cell>>,
    origin: Vec<u32>,
    moves: Vec<G::Move>,
    /// The priors of `moves`, and the moves sorted by them.
    weights: Vec<f32>,
    ranked: Vec<(f32, G::Move)>,
    path: Vec<u32>,
    /// [`sqrt_ln_table`], shared by all searches.
    sqrt_ln: &'static [f64],
    counting: PhantomData<D>,
}

impl<G: Game> Mcts<G, NoDraws> {
    /// A search that does not count draws.
    pub fn new(exploration: f64, seed: u64) -> Self {
        Self::with_draws(exploration, seed)
    }
}

impl<G: Game> Mcts<G, CountDraws> {
    /// A search that counts draws, for reports; it chooses the same moves
    /// as [`Mcts::new`] with the same seed.
    pub fn counting_draws(exploration: f64, seed: u64) -> Self {
        Self::with_draws(exploration, seed)
    }
}

impl<G: Game, D: Draws> Mcts<G, D> {
    fn with_draws(exploration: f64, seed: u64) -> Self {
        Mcts {
            exploration,
            priors: false,
            prior_weight: 0.0,
            rng: Rng::new(seed),
            nodes: Vec::new(),
            root: None,
            current: false,
            spare: Vec::new(),
            origin: Vec::new(),
            moves: Vec::new(),
            weights: Vec::new(),
            ranked: Vec::new(),
            path: Vec::new(),
            sqrt_ln: sqrt_ln_table(),
            counting: PhantomData,
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
                self.current = false;
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

        self.prepare(root, candidates);
        let reused = u64::from(self.nodes[0].visits);
        let iterations = self.run(root, candidates.len(), budget);

        let best = &self.nodes[self.choose(0).expect("the root has children")];
        let proven = self.proven();
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
        self.prepare(root, &moves);
        self.run(root, 0, budget)
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
        let (score, proven) = match child.proof {
            Proof::MoverWins => (1.0, Some(true)),
            Proof::MoverLoses => (0.0, Some(false)),
            Proof::Unknown => (f64::from(child.value), None),
        };
        let draws = D::share(&child.draws).map(|share| if proven.is_some() { 0.0 } else { share });
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
            draws,
            proven,
            line,
        }
    }

    /// Makes the tree start at `root`: the kept one when it holds `root`,
    /// otherwise a fresh one with `candidates`, which must be legal there.
    fn prepare(&mut self, root: &G, candidates: &[G::Move]) {
        if !self.reuse(root, candidates) {
            self.start(root, candidates);
        }
        self.root = Some(root.clone());
        self.current = true;
    }

    /// Iterates on `root` until the root is proven, or `budget` is spent
    /// after at least `minimum` iterations. Returns the iterations run.
    fn run(&mut self, root: &G, minimum: usize, budget: Budget) -> u64 {
        let mut iterations = 0;
        while self.nodes[0].proof == Proof::Unknown
            && ((iterations as usize) < minimum || !budget.is_spent(iterations))
        {
            self.iterate(root);
            iterations += 1;
        }
        iterations
    }

    /// The child of `node` that a player there would choose, as `search`
    /// chooses at the root: a proven win, else the most tried not proven
    /// lost, else the most tried. `None` when `node` has no children.
    fn choose(&self, node: usize) -> Option<usize> {
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
            prior: 0.0,
            bias: 0.0,
            draws: D::Cell::default(),
        });
        self.moves.clear();
        self.moves.extend_from_slice(candidates);
        self.expand(0, root);
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
                self.expand(node, &state);
            }
            node = self.select(node);
            state.play(self.nodes[node].mv);
            self.path.push(node as u32);
        };
        let prior_weight = self.prior_weight as f32;
        for &index in &self.path {
            let node = &mut self.nodes[index as usize];
            node.visits += 1;
            let mover_score = if node.mover == 0 { score } else { 1.0 - score };
            let visits = node.visits as f32;
            node.value += (mover_score as f32 - node.value) / visits;
            node.inv_sqrt_visits = visits.sqrt().recip();
            node.bias = prior_weight * node.prior / (visits + 1.0);
            D::record(&mut node.draws, score == 0.5, visits);
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

    /// Gives `node`, whose position is `state`, one child per move in
    /// `self.moves`: in order of the game's priors when the search uses
    /// them, otherwise in random order.
    fn expand(&mut self, node: usize, state: &G) {
        self.rng.shuffle(&mut self.moves);
        let count = self.moves.len();
        self.weights.clear();
        if self.priors {
            state.priors(&self.moves, &mut self.weights);
        }
        let total: f32 = self.weights.iter().sum();
        self.ranked.clear();
        if self.weights.len() == count && total > 0.0 {
            self.ranked.extend(
                self.weights
                    .iter()
                    .zip(&self.moves)
                    .map(|(&weight, &mv)| (weight / total, mv)),
            );
            // Most promising first; ties keep no particular order.
            self.ranked.sort_unstable_by(|a, b| b.0.total_cmp(&a.0));
        } else {
            self.ranked.extend(self.moves.iter().map(|&mv| (0.0, mv)));
        }
        let to_move = state.to_move() as u8;
        let prior_weight = self.prior_weight as f32;
        let first_child = self.nodes.len() as u32;
        for &(prior, mv) in &self.ranked {
            self.nodes.push(Node {
                mv,
                mover: to_move,
                proof: Proof::Unknown,
                expanded: false,
                first_child: 0,
                children: 0,
                visits: 0,
                value: 0.0,
                inv_sqrt_visits: 0.0,
                prior,
                bias: prior_weight * prior,
                draws: D::Cell::default(),
            });
        }
        let parent = &mut self.nodes[node];
        parent.expanded = true;
        parent.first_child = first_child;
        parent.children = count as u32;
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
            let bound = child.value + spread * child.inv_sqrt_visits + child.bias;
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
