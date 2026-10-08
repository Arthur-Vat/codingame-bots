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
    for (pile, expected) in [
        (13, Some(Outcome::Win)),
        (12, Some(Outcome::Loss)),
        (18, Some(Outcome::Win)),
    ] {
        let result = search(&nim(pile), &[1, 2, 3], 50_000, 1);
        assert_eq!(result.proven, expected, "pile {pile}: {result:?}");
        assert!(
            result.iterations < 50_000,
            "stopped once proven: {result:?}"
        );
        if expected == Some(Outcome::Win) {
            assert_eq!(result.best, pile % 4);
            assert_eq!(result.expected_score, 1.0);
        }
    }
}

#[test]
fn a_proven_root_needs_no_more_iterations() {
    let mut mcts = Mcts::new(1.0, 2);
    let first = mcts.search(&nim(9), &[1, 2, 3], Budget::Iterations(50_000));
    assert_eq!((first.best, first.proven), (1, Some(Outcome::Win)));
    // The same position again: the kept root is already proven.
    let again = mcts.search(&nim(9), &[1, 2, 3], Budget::Iterations(50_000));
    assert_eq!(
        (again.best, again.proven, again.iterations),
        (1, Some(Outcome::Win), 0)
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

/// Tic-tac-toe, a game whose value with best play is a draw: cells 0 to 8,
/// seat 0 plays first.
#[derive(Clone, Debug, PartialEq)]
struct TicTacToe {
    marks: [u16; 2],
    to_move: usize,
}

const LINES: [u16; 8] = [0o007, 0o070, 0o700, 0o111, 0o222, 0o444, 0o421, 0o124];

impl TicTacToe {
    fn from(moves: &[u8]) -> TicTacToe {
        let mut game = TicTacToe {
            marks: [0; 2],
            to_move: 0,
        };
        for &cell in moves {
            game.play(cell);
        }
        game
    }

    fn winner(&self) -> Option<usize> {
        (0..2).find(|&seat| LINES.iter().any(|&line| self.marks[seat] & line == line))
    }

    /// The exact value for the player to move: 1, 0 or -1.
    fn minimax(&self) -> i32 {
        match self.score() {
            Some(score) => {
                let for_seat_0 = (score * 2.0) as i32 - 1;
                if self.to_move == 0 {
                    for_seat_0
                } else {
                    -for_seat_0
                }
            }
            None => {
                let mut moves = Vec::new();
                self.legal_moves(&mut moves);
                moves
                    .iter()
                    .map(|&cell| {
                        let mut next = self.clone();
                        next.play(cell);
                        -next.minimax()
                    })
                    .max()
                    .expect("a game that goes on has moves")
            }
        }
    }
}

impl Game for TicTacToe {
    type Move = u8;

    fn to_move(&self) -> usize {
        self.to_move
    }

    fn legal_moves(&self, moves: &mut Vec<u8>) {
        moves.clear();
        let taken = self.marks[0] | self.marks[1];
        moves.extend((0..9).filter(|&cell| taken & (1 << cell) == 0));
    }

    fn play(&mut self, cell: u8) {
        assert_eq!((self.marks[0] | self.marks[1]) & (1 << cell), 0);
        self.marks[self.to_move] |= 1 << cell;
        self.to_move = 1 - self.to_move;
    }

    fn score(&self) -> Option<f64> {
        match self.winner() {
            Some(0) => Some(1.0),
            Some(_) => Some(0.0),
            None if self.marks[0] | self.marks[1] == 0o777 => Some(0.5),
            None => None,
        }
    }
}

#[test]
fn proves_a_draw_and_stops() {
    // X in the centre, O in a corner: a draw with best play.
    let game = TicTacToe::from(&[4, 0]);
    assert_eq!(game.minimax(), 0);
    let mut moves = Vec::new();
    game.legal_moves(&mut moves);
    let result = Mcts::new(1.0, 3).search(&game, &moves, Budget::Iterations(200_000));
    assert_eq!(result.proven, Some(Outcome::Draw), "{result:?}");
    assert!(
        result.iterations < 200_000,
        "stopped once proven: {result:?}"
    );
    assert_eq!(result.expected_score, 0.5);
    let mut next = game.clone();
    next.play(result.best);
    assert_eq!(
        next.minimax(),
        0,
        "the move played keeps the draw: {result:?}"
    );
}

#[test]
fn proofs_agree_with_minimax_including_draws() {
    let mut rng = Rng::new(5);
    let mut counts = [0; 3];
    for _ in 0..300 {
        let mut game = TicTacToe::from(&[]);
        let mut moves = Vec::new();
        // Three or four random moves, then a search of what is left.
        for _ in 0..3 + rng.below(2) {
            game.legal_moves(&mut moves);
            game.play(*rng.pick(&moves).unwrap());
        }
        if game.score().is_some() {
            continue;
        }
        game.legal_moves(&mut moves);
        let result =
            Mcts::new(1.0, rng.next_u64()).search(&game, &moves, Budget::Iterations(100_000));
        let expected = match game.minimax() {
            1 => Outcome::Win,
            0 => Outcome::Draw,
            _ => Outcome::Loss,
        };
        assert_eq!(result.proven, Some(expected), "{game:?}: {result:?}");
        if expected != Outcome::Loss {
            let mut next = game.clone();
            next.play(result.best);
            assert_eq!(-next.minimax(), game.minimax(), "{game:?}: {result:?}");
        }
        counts[expected as usize] += 1;
    }
    assert!(counts.iter().all(|&count| count > 10), "{counts:?}");
}
