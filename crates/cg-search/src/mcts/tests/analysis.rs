//! Tests of `think` and `report`, and of the draws the search counts for
//! them (feature `analysis`).

use super::*;
// Reports and thinking, which the `analysis` feature adds.

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

/// The reply a line takes below `node`: the child the search would choose
/// there, if it has visits.
fn reply<G: Game>(mcts: &Mcts<G>, node: usize) -> Option<usize> {
    mcts.choose(node)
        .filter(|&child| mcts.nodes[child].visits > 0)
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
            assert_eq!(mv.draws, 0.0, "nim has no draws");
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
    assert_eq!(listed[0], mcts.nodes[mcts.choose(0).unwrap()].mv);
    let others: Vec<u32> = tree.into_iter().filter(|&mv| mv != listed[0]).collect();
    assert_eq!(&listed[1..], &others[..]);
}

#[test]
fn lines_follow_the_chosen_replies_and_respect_the_depth() {
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
                node = reply(&mcts, node).expect("the line stops at unvisited nodes");
                assert_eq!(*step, mcts.nodes[node].mv);
            }
            // A line that ends before the depth has reached a node without
            // visited children.
            if depth > 0 && mv.line.len() < depth {
                assert!(reply(&mcts, node).is_none());
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
fn a_move_that_loses_is_followed_by_its_refutation() {
    // From 12 stones every move loses; the answer that proves it leaves a
    // multiple of 4. Following the most visited reply could show another,
    // because the proven reply stops being visited once it is proven.
    for seed in 0..8 {
        let mut mcts = Mcts::new(1.0, seed);
        mcts.think(&nim(12), iterations(50_000));
        let report = mcts.report(4).unwrap();
        assert_eq!(report.proven, Some(false));
        for mv in &report.moves {
            assert_eq!(mv.proven, Some(false));
            assert!(mv.line.len() >= 2, "seed {seed}: {mv:?}");
            assert_eq!(mv.line[0], mv.mv);
            assert_eq!(mv.line[1], (12 - mv.mv) % 4, "seed {seed}: {mv:?}");
        }
    }
}

#[test]
fn a_report_describes_the_last_root_the_tree_was_left_at() {
    let mut mcts = Mcts::new(1.0, 1);
    mcts.search(&nim(30), &[1, 2, 3], iterations(500));
    assert_eq!(mcts.report(1).unwrap().visits, 500);
    // A winning move is answered at once and the tree stays as it was: the
    // report would describe the earlier root, so there is none.
    let result = mcts.search(&nim(3), &[1, 2, 3], iterations(500));
    assert_eq!((result.best, result.iterations), (3, 0));
    assert_eq!(mcts.report(1), None);
    // The earlier root is still continued, and reported again.
    let again = mcts.search(&nim(30), &[1, 2, 3], iterations(100));
    assert_eq!(again.reused, 500);
    assert_eq!(mcts.report(1).unwrap().visits, 600);
    // Thinking on the winning position searches it, and reports it.
    mcts.search(&nim(3), &[1, 2, 3], iterations(500));
    assert_eq!(mcts.report(1), None);
    assert!(mcts.think(&nim(3), iterations(500)) >= 1);
    assert_eq!(mcts.report(1).unwrap().moves[0].mv, 3);
}

#[test]
fn proven_moves_score_one_or_zero() {
    let mut mcts = Mcts::new(1.0, 1);
    mcts.think(&nim(13), iterations(50_000));
    let report = mcts.report(4).unwrap();
    assert_eq!(report.proven, Some(true));
    assert_eq!(report.moves[0].mv, 1, "13 stones: take 1");
    assert_eq!(report.moves[0].score, 1.0);
    assert_eq!(report.moves[0].proven, Some(true));
    assert_eq!(report.moves[0].draws, 0.0);
    for mv in &report.moves {
        match mv.proven {
            Some(true) => assert_eq!((mv.score, mv.draws), (1.0, 0.0)),
            Some(false) => assert_eq!((mv.score, mv.draws), (0.0, 0.0)),
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
fn counted_draws_are_plausible() {
    // Move 2 wins after either reply, which proves it and so the root:
    // thinking stops.
    let mut mcts = Mcts::new(1.0, 5);
    mcts.think(&table(), iterations(3_000));
    let report = mcts.report(2).unwrap();
    assert_eq!(report.proven, Some(true));
    assert_eq!(report.moves[0].mv, 2);
    // With only moves 0 and 1 offered, the opponent finds its best answers:
    // move 0 ends in a draw, move 1 in a loss.
    let mut mcts = Mcts::new(1.0, 5);
    mcts.search(&table(), &[0, 1], iterations(3_000));
    let report = mcts.report(2).unwrap();
    let find = |mv: u32| report.moves.iter().find(|m| m.mv == mv).unwrap().clone();
    let (drawn, lost) = (find(0), find(1));
    assert!(drawn.draws > 0.8, "{drawn:?}");
    assert!(lost.draws < 0.2, "{lost:?}");
    assert!(drawn.score > 0.5 && drawn.score < 0.7, "{drawn:?}");
    assert!(lost.score < 0.1, "{lost:?}");
    for mv in &report.moves {
        // Results are wins, draws and losses: draws cannot exceed what the
        // score leaves room for (a little slack for rounding to f32).
        let draws = mv.draws;
        assert!(draws <= 2.0 * mv.score + 1e-4, "{mv:?}");
        assert!(draws <= 2.0 * (1.0 - mv.score) + 1e-4, "{mv:?}");
    }
    // A game that is always drawn.
    let mut mcts = Mcts::new(0.5, 3);
    mcts.think(&Choice { played: false }, iterations(300));
    for mv in mcts.report(1).unwrap().moves {
        assert!((mv.score - 0.5).abs() < 1e-6, "{mv:?}");
        assert!((mv.draws - 1.0).abs() < 1e-6, "{mv:?}");
    }
    // A game without draws.
    let mut mcts = Mcts::new(1.0, 3);
    mcts.think(&nim(30), iterations(500));
    for mv in mcts.report(1).unwrap().moves {
        assert_eq!(mv.draws, 0.0);
    }
}
