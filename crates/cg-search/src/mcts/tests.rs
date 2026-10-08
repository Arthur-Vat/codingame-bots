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
