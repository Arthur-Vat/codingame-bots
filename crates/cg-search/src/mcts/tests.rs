use super::*;
use crate::report::Report;

/// Nim with one pile: take 1 to 3 stones; taking the last one wins. The
/// player to move wins exactly when the pile is not a multiple of 4, by
/// leaving one.
#[derive(Clone, Debug, PartialEq)]
struct Nim {
    pile: u32,
    to_move: usize,
}

impl Game for Nim {
    type Move = u32;

    fn to_move(&self) -> usize {
        self.to_move
    }

    fn legal_moves(&self, moves: &mut Vec<u32>) {
        moves.clear();
        moves.extend(1..=self.pile.min(3));
    }

    fn play(&mut self, take: u32) {
        assert!(
            (1..=self.pile.min(3)).contains(&take),
            "illegal move {take}"
        );
        self.pile -= take;
        self.to_move = 1 - self.to_move;
    }

    fn score(&self) -> Option<f64> {
        // The player who took the last stone is the one not to move.
        (self.pile == 0).then_some(if self.to_move == 1 { 1.0 } else { 0.0 })
    }
}

fn nim(pile: u32) -> Nim {
    Nim { pile, to_move: 0 }
}

fn search(game: &Nim, candidates: &[u32], iterations: u64, seed: u64) -> SearchResult<u32> {
    Mcts::new(1.0, seed).search(game, candidates, Budget::Iterations(iterations))
}

#[test]
fn finds_the_winning_move() {
    for pile in [5, 6, 7, 9, 10, 11, 13, 14, 15] {
        let result = search(&nim(pile), &[1, 2, 3], 20_000, 1);
        assert_eq!(result.best, pile % 4, "pile {pile}: {result:?}");
        assert!(result.expected_score > 0.8, "pile {pile}: {result:?}");
    }
}

#[test]
fn knows_a_lost_position_is_lost() {
    let result = search(&nim(12), &[1, 2, 3], 20_000, 1);
    assert!(result.expected_score < 0.3, "{result:?}");
}

#[test]
fn takes_an_immediate_win_as_player_1() {
    let game = Nim {
        pile: 2,
        to_move: 1,
    };
    let result = search(&game, &[1, 2], 200, 3);
    assert_eq!(result.best, 2);
    assert_eq!(result.expected_score, 1.0);
    assert_eq!(result.iterations, 0, "an immediate win needs no search");
}

#[test]
fn chooses_only_among_the_candidates() {
    // Taking 2 wins from 10, but only 1 and 3 are offered.
    for seed in 0..20 {
        let result = search(&nim(10), &[1, 3], 500, seed);
        assert!([1, 3].contains(&result.best), "{result:?}");
    }
}

#[test]
fn tries_every_candidate_even_with_no_budget() {
    let result = search(&nim(10), &[1, 2, 3], 0, 1);
    assert_eq!(result.iterations, 3);
    let result = Mcts::new(1.0, 1).search(
        &nim(10),
        &[1, 2, 3],
        Budget::Until(std::time::Instant::now()),
    );
    assert_eq!(result.iterations, 3);
}

#[test]
fn the_same_seed_gives_the_same_search() {
    let first = search(&nim(21), &[1, 2, 3], 3_000, 7);
    let second = search(&nim(21), &[1, 2, 3], 3_000, 7);
    assert_eq!(first, second);
}

#[test]
fn the_tree_grows_with_the_iterations() {
    let small = search(&nim(30), &[1, 2, 3], 100, 1);
    let large = search(&nim(30), &[1, 2, 3], 10_000, 1);
    assert_eq!(large.iterations, 10_000);
    assert!(large.nodes > small.nodes);
    // Each node is created by an expansion that an iteration triggers.
    assert!(large.nodes <= 1 + 3 * 10_000);
}

#[test]
fn a_searcher_can_be_reused() {
    let mut mcts = Mcts::new(1.0, 5);
    let first = mcts.search(&nim(9), &[1, 2, 3], Budget::Iterations(5_000));
    let second = mcts.search(&nim(6), &[1, 2, 3], Budget::Iterations(5_000));
    assert_eq!((first.best, second.best), (1, 2));
}

#[test]
fn default_playouts_reach_the_end() {
    let mut rng = Rng::new(1);
    for pile in 1..30 {
        let score = nim(pile).playout(&mut rng);
        assert!(score == 0.0 || score == 1.0);
    }
}

fn after(game: &Nim, takes: &[u32]) -> Nim {
    let mut next = game.clone();
    for &take in takes {
        next.play(take);
    }
    next
}

// Searches stay reliable on piles up to about 13 with a few thousand
// iterations; deeper Nim needs far more.

#[test]
fn keeps_the_subtree_of_the_new_position() {
    let mut mcts = Mcts::new(1.0, 3);
    let first = mcts.search(&nim(13), &[1, 2, 3], Budget::Iterations(20_000));
    assert_eq!((first.best, first.reused), (1, 0));
    // Our move, then the opponent's: two moves below the previous root.
    let position = after(&nim(13), &[1, 1]);
    let second = mcts.search(&position, &[1, 2, 3], Budget::Iterations(100));
    assert!(second.reused > 100, "{second:?}");
    assert!(second.nodes < first.nodes, "{second:?} after {first:?}");
    assert_eq!(second.best, 3, "11 stones: take 3. {second:?}");
    // Searching the same position again keeps the whole tree.
    let third = mcts.search(&position, &[1, 2, 3], Budget::Iterations(100));
    assert_eq!(third.reused, second.reused + second.iterations);
}

#[test]
fn keeps_the_subtree_one_move_below() {
    let mut mcts = Mcts::new(1.0, 4);
    mcts.search(&nim(30), &[1, 2, 3], Budget::Iterations(5_000));
    let result = mcts.search(&after(&nim(30), &[1]), &[1, 2, 3], Budget::Iterations(10));
    assert!(result.reused > 0, "{result:?}");
}

#[test]
fn starts_afresh_on_an_unrelated_position() {
    let mut mcts = Mcts::new(1.0, 5);
    mcts.search(&nim(30), &[1, 2, 3], Budget::Iterations(5_000));
    // 21 stones with player 1 to move is three moves below 30: out of reach.
    let far = after(&nim(30), &[3, 3, 3]);
    let result = mcts.search(&far, &[1, 2, 3], Budget::Iterations(10));
    assert_eq!(result.reused, 0);
    let other = mcts.search(&nim(13), &[1, 2, 3], Budget::Iterations(10));
    assert_eq!(other.reused, 0);
}

#[test]
fn starts_afresh_when_the_candidates_differ() {
    let mut mcts = Mcts::new(1.0, 6);
    let first = mcts.search(&nim(30), &[1, 2, 3], Budget::Iterations(5_000));
    let position = after(&nim(30), &[first.best, 1]);
    // Only two of the three legal moves are offered.
    let result = mcts.search(&position, &[1, 2], Budget::Iterations(10));
    assert_eq!(result.reused, 0);
    assert!([1, 2].contains(&result.best));
}

#[test]
fn a_whole_game_with_a_kept_tree_finds_the_winning_moves() {
    for seed in 0..10 {
        let mut mcts = Mcts::new(1.0, seed);
        let mut game = nim(13);
        let mut reused = 0;
        while game.score().is_none() {
            let mut moves = Vec::new();
            game.legal_moves(&mut moves);
            let result = mcts.search(&game, &moves, Budget::Iterations(3_000));
            if game.to_move() == 0 {
                assert_eq!(result.best, game.pile % 4, "pile {}: {result:?}", game.pile);
            }
            reused += result.reused;
            game.play(result.best);
        }
        assert_eq!(game.score(), Some(1.0));
        assert!(reused > 0);
    }
}

#[test]
fn proves_wins_and_losses_and_stops_early() {
    // Proving 13 and 12 stones takes a few hundred iterations, 18 a few
    // thousand; without proofs, 18 stones was solved 3 times in 10.
    for (pile, expected) in [(13, Some(true)), (12, Some(false)), (18, Some(true))] {
        let result = search(&nim(pile), &[1, 2, 3], 50_000, 1);
        assert_eq!(result.proven, expected, "pile {pile}: {result:?}");
        assert!(
            result.iterations < 50_000,
            "stopped once proven: {result:?}"
        );
        if expected == Some(true) {
            assert_eq!(result.best, pile % 4);
            assert_eq!(result.expected_score, 1.0);
        }
    }
}

#[test]
fn a_proven_root_needs_no_more_iterations() {
    let mut mcts = Mcts::new(1.0, 2);
    let first = mcts.search(&nim(9), &[1, 2, 3], Budget::Iterations(50_000));
    assert_eq!((first.best, first.proven), (1, Some(true)));
    // The same position again: the kept root is already proven.
    let again = mcts.search(&nim(9), &[1, 2, 3], Budget::Iterations(50_000));
    assert_eq!(
        (again.best, again.proven, again.iterations),
        (1, Some(true), 0)
    );
}

#[test]
fn unproven_searches_report_no_proof() {
    let result = search(&nim(30), &[1, 2, 3], 1_000, 1);
    assert_eq!(result.proven, None);
}

#[test]
fn the_sqrt_ln_table_holds_exactly_the_computed_values() {
    let table = sqrt_ln_table();
    assert_eq!(table.len(), SQRT_LN_TABLE_LEN);
    assert_eq!(table[0], 0.0, "0 visits count as 1");
    for n in (1..SQRT_LN_TABLE_LEN as u32)
        .step_by(97)
        .chain([1, 2, 65_535])
    {
        assert_eq!(
            table[n as usize].to_bits(),
            f64::from(n).ln().sqrt().to_bits()
        );
    }
}

#[test]
fn root_visits_add_up_to_the_root_visits() {
    let mut mcts = Mcts::new(1.0, 7);
    assert_eq!(mcts.root_visits().count(), 0);
    let result = mcts.search(&nim(30), &[1, 2, 3], Budget::Iterations(1_000));
    let visits: Vec<(u32, u32)> = mcts.root_visits().collect();
    let mut moves: Vec<u32> = visits.iter().map(|&(mv, _)| mv).collect();
    moves.sort_unstable();
    assert_eq!(moves, vec![1, 2, 3]);
    // A fresh root starts with its children, so every iteration visits one.
    let total: u32 = visits.iter().map(|&(_, count)| count).sum();
    assert_eq!(u64::from(total), result.iterations);
    let most = visits.iter().max_by_key(|&&(_, count)| count).unwrap().0;
    assert_eq!(most, result.best);
}

/// A game of one move among three, always drawn, with priors 1, 2 and 7.
#[derive(Clone, Debug, PartialEq)]
struct Choice {
    played: bool,
}

impl Game for Choice {
    type Move = u32;

    fn to_move(&self) -> usize {
        usize::from(self.played)
    }

    fn legal_moves(&self, moves: &mut Vec<u32>) {
        moves.clear();
        if !self.played {
            moves.extend([0, 1, 2]);
        }
    }

    fn play(&mut self, _mv: u32) {
        self.played = true;
    }

    fn score(&self) -> Option<f64> {
        self.played.then_some(0.5)
    }

    fn priors(&self, moves: &[u32], weights: &mut Vec<f32>) {
        weights.clear();
        weights.extend(moves.iter().map(|&mv| [1.0, 2.0, 7.0][mv as usize]));
    }
}

fn choice_visits(priors: bool, prior_weight: f64) -> Vec<(u32, u32)> {
    let mut mcts = Mcts::new(0.5, 3);
    mcts.priors = priors;
    mcts.prior_weight = prior_weight;
    mcts.search(
        &Choice { played: false },
        &[0, 1, 2],
        Budget::Iterations(300),
    );
    mcts.root_visits().collect()
}

#[test]
fn priors_order_the_children_and_bias_the_visits() {
    // Without priors, the game's priors are not asked for: even visits.
    let plain = choice_visits(false, 1.0);
    let counts: Vec<u32> = plain.iter().map(|&(_, n)| n).collect();
    assert!(counts.iter().all(|n| (99..=101).contains(n)), "{plain:?}");
    // Priors without a bias only order the children.
    let ordered = choice_visits(true, 0.0);
    let moves: Vec<u32> = ordered.iter().map(|&(mv, _)| mv).collect();
    assert_eq!(moves, vec![2, 1, 0]);
    // With a bias, the most promising move gets the most visits.
    let biased = choice_visits(true, 1.0);
    let visits = |mv: u32| biased.iter().find(|&&(m, _)| m == mv).unwrap().1;
    assert!(visits(2) > visits(1) && visits(1) > visits(0), "{biased:?}");
    assert_eq!(visits(0) + visits(1) + visits(2), 300);
}

#[test]
fn games_without_priors_search_as_before_when_priors_are_asked_for() {
    // Nim gives no priors: asking for them changes nothing.
    let plain = search(&nim(30), &[1, 2, 3], 2_000, 5);
    let mut mcts = Mcts::new(1.0, 5);
    mcts.priors = true;
    mcts.prior_weight = 1.0;
    let asked = mcts.search(&nim(30), &[1, 2, 3], Budget::Iterations(2_000));
    assert_eq!(asked, plain);
}

// Reports, thinking, and counting draws.

/// Player 0 picks a move, then player 1 does; the table gives player 0's
/// score. Player 1 prefers the lower score, so move 0 ends in a draw and
/// move 1 in a loss, and move 2 wins.
#[derive(Clone, Debug, PartialEq)]
struct Table {
    first: Option<usize>,
    second: Option<usize>,
}

const TABLE: [[f64; 2]; 3] = [[1.0, 0.5], [0.5, 0.0], [1.0, 1.0]];

impl Game for Table {
    type Move = u32;

    fn to_move(&self) -> usize {
        usize::from(self.first.is_some())
    }

    fn legal_moves(&self, moves: &mut Vec<u32>) {
        moves.clear();
        match self.first {
            None => moves.extend([0, 1, 2]),
            Some(_) if self.second.is_none() => moves.extend([0, 1]),
            Some(_) => {}
        }
    }

    fn play(&mut self, mv: u32) {
        if self.first.is_none() {
            self.first = Some(mv as usize);
        } else {
            self.second = Some(mv as usize);
        }
    }

    fn score(&self) -> Option<f64> {
        Some(TABLE[self.first?][self.second?])
    }
}

fn table() -> Table {
    Table {
        first: None,
        second: None,
    }
}

fn iterations(count: u64) -> Budget {
    Budget::Iterations(count)
}

/// The report with the draws removed, to compare searches that count draws
/// with those that do not.
fn without_draws(report: &Report<u32>) -> Report<u32> {
    let mut report = report.clone();
    for mv in &mut report.moves {
        mv.draws = None;
    }
    report
}

/// The most visited child of `node` with visits, the first on ties.
fn most_visited<G: Game, D: Draws>(mcts: &Mcts<G, D>, node: usize) -> Option<usize> {
    mcts.children(node)
        .filter(|&child| mcts.nodes[child].visits > 0)
        .fold(None, |best, child| match best {
            Some(b) if mcts.nodes[b].visits >= mcts.nodes[child].visits => Some(b),
            _ => Some(child),
        })
}

#[test]
fn a_node_of_a_search_that_does_not_count_draws_is_as_small_as_before() {
    // 32 bytes with a one-byte move before draws could be counted: the size
    // of a node of a bot's tree, on which its memory and speed depend.
    assert_eq!(
        std::mem::size_of::<Node<u8, <NoDraws as Draws>::Cell>>(),
        32
    );
    assert!(
        std::mem::size_of::<Node<u8, <CountDraws as Draws>::Cell>>()
            > std::mem::size_of::<Node<u8, <NoDraws as Draws>::Cell>>()
    );
}

#[test]
fn there_is_no_report_before_iterations_ran() {
    let mut mcts = Mcts::new(1.0, 1);
    assert_eq!(mcts.report(3), None);
    // A budget of nothing runs nothing and leaves nothing to report.
    assert_eq!(mcts.think(&nim(10), iterations(0)), 0);
    assert_eq!(mcts.report(3), None);
    // A search that wins at once leaves the tree as it was.
    mcts.search(&nim(3), &[1, 2, 3], iterations(10));
    assert_eq!(mcts.report(3), None);
}

#[test]
fn the_report_lists_the_chosen_move_first_then_the_others_by_visits() {
    for seed in 0..5 {
        let mut mcts = Mcts::new(1.0, seed);
        let result = mcts.search(&nim(30), &[1, 2, 3], iterations(1_500));
        let report = mcts.report(3).unwrap();
        assert_eq!(report.moves.len(), 3);
        assert_eq!(report.moves[0].mv, result.best);
        assert_eq!(report.moves[0].score, result.expected_score);
        assert_eq!(report.visits, result.iterations);
        assert_eq!(report.nodes, result.nodes);
        assert_eq!(report.proven, result.proven);
        let mut moves: Vec<u32> = report.moves.iter().map(|m| m.mv).collect();
        moves.sort_unstable();
        assert_eq!(moves, vec![1, 2, 3]);
        assert!(report.moves[1].visits >= report.moves[2].visits);
        // The chosen move has the most visits when nothing is proven.
        assert!(report.moves[0].visits >= report.moves[1].visits);
        let total: u64 = report.moves.iter().map(|m| u64::from(m.visits)).sum();
        assert_eq!(total, report.visits, "a fresh root: every visit is below");
        for mv in &report.moves {
            assert!((0.0..=1.0).contains(&mv.score));
            assert_eq!(mv.proven, None);
            assert_eq!(mv.draws, None, "draws are not counted");
        }
    }
}

#[test]
fn the_visits_of_a_reused_root_leave_out_its_first() {
    let mut mcts = Mcts::new(1.0, 4);
    mcts.search(&nim(30), &[1, 2, 3], iterations(5_000));
    let result = mcts.search(&after(&nim(30), &[1]), &[1, 2, 3], iterations(10));
    assert!(result.reused > 0);
    let report = mcts.report(1).unwrap();
    assert_eq!(report.visits, result.reused + result.iterations);
    let total: u64 = report.moves.iter().map(|m| u64::from(m.visits)).sum();
    // The visit that gave the node its children went to a playout.
    assert_eq!(total, report.visits - 1);
}

#[test]
fn ties_keep_the_tree_order() {
    // Nothing is proven and every child has one visit: the others come in
    // the order of the tree, which `root_visits` shows.
    let mut mcts = Mcts::new(1.0, 3);
    mcts.search(&nim(30), &[1, 2, 3], iterations(3));
    let tree: Vec<u32> = mcts.root_visits().map(|(mv, _)| mv).collect();
    let report = mcts.report(0).unwrap();
    let listed: Vec<u32> = report.moves.iter().map(|m| m.mv).collect();
    assert_eq!(listed[0], mcts.nodes[mcts.best_child()].mv);
    let others: Vec<u32> = tree.into_iter().filter(|&mv| mv != listed[0]).collect();
    assert_eq!(&listed[1..], &others[..]);
}

#[test]
fn lines_follow_the_most_visited_children_and_respect_the_depth() {
    let mut mcts = Mcts::new(1.0, 2);
    mcts.search(&nim(30), &[1, 2, 3], iterations(5_000));
    for depth in [0, 1, 2, 5, 40] {
        let report = mcts.report(depth).unwrap();
        for mv in &report.moves {
            assert!(mv.line.len() <= depth, "depth {depth}: {mv:?}");
            if depth == 0 {
                assert!(mv.line.is_empty());
            } else {
                assert_eq!(mv.line[0], mv.mv);
            }
            // Walk the tree along the line.
            let mut node = mcts
                .children(0)
                .find(|&child| mcts.nodes[child].mv == mv.mv)
                .unwrap();
            for step in mv.line.iter().skip(1) {
                node = most_visited(&mcts, node).expect("the line stops at unvisited nodes");
                assert_eq!(*step, mcts.nodes[node].mv);
            }
            // A line that ends before the depth has reached a node without
            // visited children.
            if depth > 0 && mv.line.len() < depth {
                assert!(most_visited(&mcts, node).is_none());
            }
        }
        if depth >= 4 {
            assert!(report.moves[0].line.len() >= 4, "5,000 iterations go deep");
        }
    }
    // A game that ends after one move has lines of one move.
    let mut mcts = Mcts::new(1.0, 1);
    mcts.search(&Choice { played: false }, &[0, 1, 2], iterations(30));
    for mv in mcts.report(5).unwrap().moves {
        assert_eq!(mv.line, vec![mv.mv]);
    }
}

#[test]
fn proven_moves_score_one_or_zero() {
    let mut mcts = Mcts::counting_draws(1.0, 1);
    mcts.think(&nim(13), iterations(50_000));
    let report = mcts.report(4).unwrap();
    assert_eq!(report.proven, Some(true));
    assert_eq!(report.moves[0].mv, 1, "13 stones: take 1");
    assert_eq!(report.moves[0].score, 1.0);
    assert_eq!(report.moves[0].proven, Some(true));
    assert_eq!(report.moves[0].draws, Some(0.0));
    for mv in &report.moves {
        match mv.proven {
            Some(true) => assert_eq!((mv.score, mv.draws), (1.0, Some(0.0))),
            Some(false) => assert_eq!((mv.score, mv.draws), (0.0, Some(0.0))),
            None => assert!(mv.score > 0.0 && mv.score < 1.0, "{mv:?}"),
        }
    }
}

#[test]
fn a_root_whose_moves_are_all_proven_lost_is_a_lost_position() {
    let mut mcts = Mcts::new(1.0, 1);
    mcts.think(&nim(12), iterations(50_000));
    let report = mcts.report(3).unwrap();
    assert_eq!(report.proven, Some(false));
    assert_eq!(report.moves.len(), 3);
    for mv in &report.moves {
        assert_eq!((mv.proven, mv.score), (Some(false), 0.0), "{mv:?}");
    }
    // The first is the most visited, as `search` plays it.
    let visits = report.moves.iter().map(|m| m.visits).max().unwrap();
    assert_eq!(report.moves[0].visits, visits);
}

#[test]
fn a_root_with_one_move_reports_it() {
    let mut mcts = Mcts::new(1.0, 1);
    assert_eq!(mcts.think(&nim(1), iterations(100)), 1);
    let report = mcts.report(3).unwrap();
    assert_eq!(report.moves.len(), 1);
    assert_eq!(report.proven, Some(true));
    assert_eq!(report.moves[0].mv, 1);
    assert_eq!(report.moves[0].proven, Some(true));
    assert_eq!(report.moves[0].line, vec![1]);
}

#[test]
fn thinking_continues_the_tree() {
    let mut mcts = Mcts::new(1.0, 9);
    assert_eq!(mcts.think(&nim(30), iterations(500)), 500);
    let first = mcts.report(2).unwrap();
    assert_eq!(first.visits, 500);
    assert_eq!(mcts.think(&nim(30), iterations(300)), 300);
    let second = mcts.report(2).unwrap();
    assert_eq!(second.visits, 800);
    assert!(second.nodes > first.nodes);
    // No iterations: nothing changes.
    assert_eq!(mcts.think(&nim(30), iterations(0)), 0);
    assert_eq!(mcts.report(2).unwrap(), second);
    // Thinking in pieces is thinking at once.
    let mut once = Mcts::new(1.0, 9);
    once.think(&nim(30), iterations(800));
    assert_eq!(once.report(2).unwrap(), second);
}

#[test]
fn thinking_continues_a_search_of_the_same_root_and_the_other_way() {
    let mut mcts = Mcts::new(1.0, 9);
    mcts.search(&nim(30), &[1, 2, 3], iterations(400));
    assert_eq!(mcts.think(&nim(30), iterations(100)), 100);
    assert_eq!(mcts.report(1).unwrap().visits, 500);
    let result = mcts.search(&nim(30), &[1, 2, 3], iterations(50));
    assert_eq!(result.reused, 500);
    assert_eq!(mcts.report(1).unwrap().visits, 550);
}

#[test]
fn thinking_searches_a_position_with_a_winning_move() {
    // Taking all 3 stones wins at once: `search` answers without iterating.
    let mut mcts = Mcts::new(1.0, 1);
    let result = mcts.search(&nim(3), &[1, 2, 3], iterations(100));
    assert_eq!((result.best, result.iterations), (3, 0));
    let ran = mcts.think(&nim(3), iterations(100));
    assert!(ran >= 1, "thinking searches");
    let report = mcts.report(3).unwrap();
    assert_eq!(report.proven, Some(true));
    assert_eq!(report.moves[0].mv, 3);
    assert_eq!(report.moves[0].proven, Some(true));
    assert_eq!(report.moves[0].score, 1.0);
}

#[test]
#[should_panic(expected = "the game is over")]
fn thinking_on_a_finished_game_panics() {
    let over = Nim {
        pile: 0,
        to_move: 1,
    };
    Mcts::new(1.0, 1).think(&over, iterations(1));
}

#[test]
fn draws_are_counted_only_when_asked_for() {
    let mut plain = Mcts::new(1.0, 5);
    plain.search(&table(), &[0, 1], iterations(500));
    let report = plain.report(2).unwrap();
    assert!(report.moves.iter().all(|m| m.draws.is_none()));
    let mut counting = Mcts::counting_draws(1.0, 5);
    counting.search(&table(), &[0, 1], iterations(500));
    let report = counting.report(2).unwrap();
    assert!(report.moves.iter().all(|m| m.draws.is_some()));
}

#[test]
fn counted_draws_are_plausible() {
    // Move 2 wins at once, so thinking proves the root and stops.
    let mut mcts = Mcts::counting_draws(1.0, 5);
    mcts.think(&table(), iterations(3_000));
    let report = mcts.report(2).unwrap();
    assert_eq!(report.proven, Some(true));
    assert_eq!(report.moves[0].mv, 2);
    // With only moves 0 and 1 offered, the opponent finds its best answers:
    // move 0 ends in a draw, move 1 in a loss.
    let mut mcts = Mcts::counting_draws(1.0, 5);
    mcts.search(&table(), &[0, 1], iterations(3_000));
    let report = mcts.report(2).unwrap();
    let find = |mv: u32| report.moves.iter().find(|m| m.mv == mv).unwrap().clone();
    let (drawn, lost) = (find(0), find(1));
    assert!(drawn.draws.unwrap() > 0.8, "{drawn:?}");
    assert!(lost.draws.unwrap() < 0.2, "{lost:?}");
    assert!(drawn.score > 0.5 && drawn.score < 0.7, "{drawn:?}");
    assert!(lost.score < 0.1, "{lost:?}");
    for mv in &report.moves {
        // Results are wins, draws and losses: draws cannot exceed what the
        // score leaves room for (a little slack for rounding to f32).
        let draws = mv.draws.unwrap();
        assert!(draws <= 2.0 * mv.score + 1e-4, "{mv:?}");
        assert!(draws <= 2.0 * (1.0 - mv.score) + 1e-4, "{mv:?}");
    }
    // A game that is always drawn.
    let mut mcts = Mcts::counting_draws(0.5, 3);
    mcts.think(&Choice { played: false }, iterations(300));
    for mv in mcts.report(1).unwrap().moves {
        assert!((mv.score - 0.5).abs() < 1e-6, "{mv:?}");
        assert!((mv.draws.unwrap() - 1.0).abs() < 1e-6, "{mv:?}");
    }
    // A game without draws.
    let mut mcts = Mcts::counting_draws(1.0, 3);
    mcts.think(&nim(30), iterations(500));
    for mv in mcts.report(1).unwrap().moves {
        assert_eq!(mv.draws, Some(0.0));
    }
}

#[test]
fn counting_draws_does_not_change_the_search() {
    for seed in 0..5 {
        let mut plain = Mcts::new(1.0, seed);
        let mut counting = Mcts::counting_draws(1.0, seed);
        // A whole game with a kept tree, so that reuse runs too.
        let mut game = nim(21);
        while game.score().is_none() {
            let mut moves = Vec::new();
            game.legal_moves(&mut moves);
            let a = plain.search(&game, &moves, iterations(700));
            let b = counting.search(&game, &moves, iterations(700));
            assert_eq!(a, b, "seed {seed}");
            let visits_a: Vec<_> = plain.root_visits().collect();
            let visits_b: Vec<_> = counting.root_visits().collect();
            assert_eq!(visits_a, visits_b);
            assert_eq!(
                plain.report(6).as_ref().map(without_draws),
                counting.report(6).as_ref().map(without_draws)
            );
            game.play(a.best);
        }
    }
    // Games with draws, with and without priors, and thinking.
    for (priors, weight) in [(false, 0.0), (true, 1.0)] {
        let mut plain = Mcts::new(0.5, 3);
        let mut counting = Mcts::counting_draws(0.5, 3);
        plain.priors = priors;
        counting.priors = priors;
        plain.prior_weight = weight;
        counting.prior_weight = weight;
        let start = Choice { played: false };
        assert_eq!(
            plain.search(&start, &[0, 1, 2], iterations(200)),
            counting.search(&start, &[0, 1, 2], iterations(200))
        );
        assert_eq!(plain.think(&start, iterations(50)), 50);
        assert_eq!(counting.think(&start, iterations(50)), 50);
        assert_eq!(
            plain.report(2).as_ref().map(without_draws),
            counting.report(2).as_ref().map(without_draws)
        );
    }
    let mut plain = Mcts::new(1.0, 8);
    let mut counting = Mcts::counting_draws(1.0, 8);
    plain.search(&table(), &[0, 1], iterations(1_000));
    counting.search(&table(), &[0, 1], iterations(1_000));
    assert_eq!(
        plain.report(2).as_ref().map(without_draws),
        counting.report(2).as_ref().map(without_draws)
    );
}
