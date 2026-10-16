use super::*;

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

#[test]
fn a_node_is_as_small_as_its_fields_allow() {
    // 36 bytes with a one-byte move: the 32 of origin/main before the
    // analysis feature, plus the minimax value of a search that expands all
    // children at once (E018). It is the size of a node of a bot's tree, on
    // which its memory and speed depend. With the feature a node also holds
    // the share of draws. CI builds this crate's tests with the feature on
    // (a dependency on the crate itself turns it on), so only the 40 bytes
    // are checked there; the 36 are checked by `cargo test -p cg-search`
    // after removing that dependency.
    #[cfg(not(feature = "analysis"))]
    assert_eq!(std::mem::size_of::<Node<u8>>(), 36);
    #[cfg(feature = "analysis")]
    assert_eq!(std::mem::size_of::<Node<u8>>(), 40);
}

/// A search that expands all children at once, judging them by
/// `minimax_weight` between their minimax values and their averages.
fn jacekmax(minimax_weight: f64, seed: u64) -> Mcts<Nim> {
    let mut mcts = Mcts::new(1.0, seed);
    mcts.expand_all = true;
    mcts.minimax_weight = minimax_weight;
    mcts
}

#[test]
fn default_child_scores_are_exact_at_the_end_and_playouts_elsewhere() {
    let mut scores = Vec::new();
    nim(3).child_scores(&[1, 2, 3], &mut Rng::new(1), &mut scores);
    // Taking all three wins; the other two leave a pile that goes on.
    assert_eq!(scores.len(), 3);
    assert_eq!(scores[2], (1.0, true));
    assert!(scores[..2]
        .iter()
        .all(|&(score, over)| !over && (score == 0.0 || score == 1.0)));
}

#[test]
fn jacekmax_finds_and_proves_wins_and_losses() {
    for weight in [0.0, 0.5, 1.0] {
        for (pile, expected) in [(13, Some(true)), (12, Some(false)), (18, Some(true))] {
            let result =
                jacekmax(weight, 1).search(&nim(pile), &[1, 2, 3], Budget::Iterations(50_000));
            assert_eq!(
                result.proven, expected,
                "weight {weight}, pile {pile}: {result:?}"
            );
            assert!(
                result.iterations < 50_000,
                "stopped once proven: {result:?}"
            );
            if expected == Some(true) {
                assert_eq!(result.best, pile % 4);
            }
        }
    }
}

#[test]
fn jacekmax_keeps_minimax_values_of_the_children() {
    let mut mcts = jacekmax(1.0, 2);
    mcts.search(&nim(30), &[1, 2, 3], Budget::Iterations(3_000));
    // Every expanded node's minimax value is the best of its tried
    // children's for the player choosing there.
    let mut checked = 0;
    for index in 0..mcts.nodes.len() {
        let node = mcts.nodes[index];
        let tried: Vec<f32> = mcts
            .children(index)
            .filter(|&child| mcts.nodes[child].visits > 0)
            .map(|child| mcts.nodes[child].minimax)
            .collect();
        if !node.expanded || tried.is_empty() {
            continue;
        }
        let best = tried.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        assert_eq!(node.minimax, 1.0 - best, "node {index}");
        match node.proof {
            Proof::MoverWins => assert_eq!(node.minimax, 1.0),
            Proof::MoverLoses => assert_eq!(node.minimax, 0.0),
            Proof::Unknown => {}
        }
        checked += 1;
    }
    assert!(checked > 100, "{checked} nodes checked");
}

#[test]
fn jacekmax_gives_every_new_child_one_visit() {
    let mut mcts = jacekmax(1.0, 3);
    let result = mcts.search(&nim(30), &[1, 2, 3], Budget::Iterations(500));
    // Below the root, each expanded node's children got a visit each when
    // it was expanded.
    for index in 1..mcts.nodes.len() {
        if mcts.nodes[index].expanded {
            assert!(mcts
                .children(index)
                .all(|child| mcts.nodes[child].visits >= 1));
        }
    }
    assert!((0.0..=1.0).contains(&result.expected_score));
}

#[test]
fn the_minimax_weight_is_ignored_without_expand_all() {
    let plain = search(&nim(30), &[1, 2, 3], 2_000, 5);
    let mut mcts = Mcts::new(1.0, 5);
    mcts.minimax_weight = 1.0;
    let weighted = mcts.search(&nim(30), &[1, 2, 3], Budget::Iterations(2_000));
    assert_eq!(weighted, plain);
}

// Pinned results: what searches answered before the analysis feature existed.

/// What a few searches in a row, with the tree kept between them, answer.
fn pinned_run() -> Vec<String> {
    let mut lines = Vec::new();
    let mut mcts = Mcts::new(1.0, 5);
    let mut game = nim(60);
    for (iterations, reply) in [(2_000, 1), (1_000, 2), (500, 3), (700, 1), (300, 1)] {
        let result = mcts.search(&game, &[1, 2, 3], Budget::Iterations(iterations));
        lines.push(format!("{result:?}"));
        lines.push(format!("{:?}", mcts.root_visits().collect::<Vec<_>>()));
        // The same position again continues the whole tree.
        let again = mcts.search(&game, &[1, 2, 3], Budget::Iterations(50));
        lines.push(format!("{again:?}"));
        game.play(result.best);
        game.play(reply);
    }
    // Priors, draws and a candidate list that is not all the legal moves.
    let mut mcts = Mcts::new(0.5, 3);
    mcts.priors = true;
    mcts.prior_weight = 1.0;
    let result = mcts.search(
        &Choice { played: false },
        &[0, 1, 2],
        Budget::Iterations(300),
    );
    lines.push(format!("{result:?}"));
    lines.push(format!("{:?}", mcts.root_visits().collect::<Vec<_>>()));
    let mut mcts = Mcts::new(1.0, 8);
    let result = mcts.search(&nim(10), &[1, 3], Budget::Iterations(400));
    lines.push(format!("{result:?}"));
    lines.push(format!("{:?}", mcts.root_visits().collect::<Vec<_>>()));
    lines
}

/// The results below were produced by running `pinned_run` against the
/// search as it was on `origin/main` before the `analysis` feature existed
/// (a scratch copy of that tree). They hold with the feature off, which is
/// what bots run, and with it on, whose draw counting must not touch the
/// random stream: the same moves, scores, visits and node counts for the
/// same seed and iterations. CI runs it with the feature on only (the crate's
/// tests turn it on); the feature-off run was done by hand, without the
/// dev-dependency on the crate itself, and the byte-identical bundles of the
/// bots cover that path.
#[test]
fn a_search_answers_as_it_did_before_the_analysis_feature() {
    const BEFORE: [&str; 19] = [
        "SearchResult { best: 2, iterations: 2000, reused: 0, proven: None, expected_score: 0.4916864335536957, nodes: 2626 }",
        "[(3, 636), (2, 842), (1, 522)]",
        "SearchResult { best: 2, iterations: 50, reused: 2000, proven: None, expected_score: 0.49371418356895447, nodes: 2692 }",
        "SearchResult { best: 3, iterations: 1000, reused: 331, proven: None, expected_score: 0.5188915729522705, nodes: 1720 }",
        "[(3, 794), (1, 254), (2, 282)]",
        "SearchResult { best: 3, iterations: 50, reused: 1331, proven: None, expected_score: 0.5237529873847961, nodes: 1768 }",
        "SearchResult { best: 1, iterations: 500, reused: 186, proven: None, expected_score: 0.5104166269302368, nodes: 907 }",
        "[(3, 230), (1, 288), (2, 167)]",
        "SearchResult { best: 1, iterations: 50, reused: 686, proven: None, expected_score: 0.5050846934318542, nodes: 952 }",
        "SearchResult { best: 1, iterations: 700, reused: 56, proven: None, expected_score: 0.5738916993141174, nodes: 991 }",
        "[(2, 153), (1, 406), (3, 196)]",
        "SearchResult { best: 1, iterations: 50, reused: 756, proven: None, expected_score: 0.5664337277412415, nodes: 1042 }",
        "SearchResult { best: 2, iterations: 300, reused: 150, proven: None, expected_score: 0.5422885417938232, nodes: 592 }",
        "[(2, 201), (3, 106), (1, 142)]",
        "SearchResult { best: 2, iterations: 50, reused: 450, proven: None, expected_score: 0.5296803116798401, nodes: 655 }",
        "SearchResult { best: 2, iterations: 300, reused: 0, proven: None, expected_score: 0.5, nodes: 4 }",
        "[(2, 106), (1, 98), (0, 96)]",
        "SearchResult { best: 1, iterations: 134, reused: 0, proven: Some(false), expected_score: 0.0, nodes: 155 }",
        "[(3, 23), (1, 111)]",
    ];
    let now = pinned_run();
    for (index, (expected, got)) in BEFORE.iter().zip(&now).enumerate() {
        assert_eq!(expected, got, "line {index}");
    }
    assert_eq!(now.len(), BEFORE.len());
}

/// A game of two moves among three each, always drawn, whose first move m
/// is estimated at `[0.3, 0.5, 0.7][m]` for player 0.
#[derive(Clone, Debug, PartialEq)]
struct Estimated {
    played: Vec<u32>,
}

const ESTIMATES: [f64; 3] = [0.3, 0.5, 0.7];

impl Game for Estimated {
    type Move = u32;

    fn to_move(&self) -> usize {
        self.played.len() % 2
    }

    fn legal_moves(&self, moves: &mut Vec<u32>) {
        moves.clear();
        if self.played.len() < 2 {
            moves.extend([0, 1, 2]);
        }
    }

    fn play(&mut self, mv: u32) {
        self.played.push(mv);
    }

    fn score(&self) -> Option<f64> {
        (self.played.len() == 2).then_some(0.5)
    }

    fn child_scores(&self, moves: &[u32], _rng: &mut Rng, scores: &mut Vec<(f64, bool)>) {
        scores.clear();
        scores.extend(moves.iter().map(|&mv| {
            if self.played.is_empty() {
                (ESTIMATES[mv as usize], false)
            } else {
                (0.5, true)
            }
        }));
    }
}

fn puct(value_temperature: f64, prior_weight: f64) -> Mcts<Estimated> {
    let mut mcts = Mcts::new(0.5, 4);
    mcts.expand_all = true;
    mcts.minimax_weight = 0.5;
    mcts.value_temperature = value_temperature;
    mcts.prior_weight = prior_weight;
    mcts
}

fn puct_visits(mcts: &mut Mcts<Estimated>) -> Vec<u32> {
    mcts.search(
        &Estimated { played: Vec::new() },
        &[0, 1, 2],
        Budget::Iterations(400),
    );
    let mut visits = [0; 3];
    for (mv, n) in mcts.root_visits() {
        visits[mv as usize] = n;
    }
    visits.to_vec()
}

#[test]
fn puct_priors_are_a_softmax_of_the_estimates() {
    let mut mcts = puct(0.1, 1.0);
    puct_visits(&mut mcts);
    let weights = ESTIMATES.map(|estimate| ((estimate - 0.7) / 0.1).exp());
    let total: f64 = weights.iter().sum();
    let mut sum = 0.0;
    for child in mcts.children(0) {
        let node = mcts.nodes[child];
        let expected = (weights[node.mv as usize] / total) as f32;
        assert!((node.prior - expected).abs() < 1e-6, "{node:?}");
        sum += node.prior;
    }
    assert!((sum - 1.0).abs() < 1e-6);
    // Below the root, the children are all drawn: equal priors, for
    // player 1 as for player 0.
    let first = mcts.children(0).start;
    let below = mcts.children(first);
    assert_eq!(below.len(), 3);
    for child in below {
        assert!((mcts.nodes[child].prior - 1.0 / 3.0).abs() < 1e-6);
    }
}

#[test]
fn puct_favours_the_best_estimate_as_its_weight_grows() {
    // Every first move ends in a draw: only the priors tell them apart.
    let plain = puct_visits(&mut puct(0.1, 0.0));
    let light = puct_visits(&mut puct(0.1, 0.5));
    let heavy = puct_visits(&mut puct(0.1, 4.0));
    assert!(light[2] > plain[2], "{plain:?} {light:?}");
    assert!(heavy[2] > light[2], "{light:?} {heavy:?}");
    assert!(heavy[2] > heavy[1] && heavy[1] > heavy[0], "{heavy:?}");
    for visits in [plain, light, heavy] {
        assert_eq!(visits.iter().sum::<u32>(), 400);
    }
}

#[test]
fn a_zero_temperature_searches_as_without_puct() {
    // Without the game's priors, a prior weight changes nothing unless the
    // priors come from the estimates.
    for seed in 0..3 {
        let mut with = jacekmax(0.5, seed);
        with.prior_weight = 2.0;
        let mut without = jacekmax(0.5, seed);
        let a = with.search(&nim(30), &[1, 2, 3], Budget::Iterations(2_000));
        let b = without.search(&nim(30), &[1, 2, 3], Budget::Iterations(2_000));
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
        assert_eq!(
            with.root_visits().collect::<Vec<_>>(),
            without.root_visits().collect::<Vec<_>>()
        );
    }
}

#[cfg(feature = "analysis")]
mod analysis;
