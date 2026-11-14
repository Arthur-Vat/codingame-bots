use ice_and_fire_engine::{Command, League, State};

use super::*;

/// An open map after `plies` plies of passing: red's gold grows by 1 a turn.
fn rich(plies: usize) -> State {
    let mut state = State::new(&[false; ice_and_fire_engine::CELLS], &[], League::Bronze);
    for _ in 0..plies {
        state.play_turn(&[]);
    }
    state
}

#[test]
fn a_single_line_is_cut_next_to_the_hq() {
    let mut state = rich(60);
    for x in 1..5u8 {
        assert!(state.apply(Command::Train { level: 1, x, y: 0 }));
    }
    // Cells (1..5, 0), each with a unit: price 20. The cheapest cut nearest
    // the HQ takes (1, 0) and removes the whole line.
    let cut = cut_to(&state, 0, 4, 1000).expect("a cut");
    assert_eq!((cut.cost, cut.links, cut.cells), (20, 1, 4));
    assert_eq!(best_affordable(&state, 0, 19), None);
    assert_eq!(best_affordable(&state, 0, 20).map(|c| c.cells), Some(4));
}

#[test]
fn a_double_link_needs_two_captures() {
    let mut state = rich(170);
    // A 2-wide band, columns 1 to 4, rows 0 and 1, and (0, 1).
    assert!(state.apply(Command::Train {
        level: 1,
        x: 0,
        y: 1
    }));
    for x in 1..5u8 {
        for y in 0..2u8 {
            assert!(
                state.apply(Command::Train { level: 1, x, y }),
                "train {x} {y}"
            );
        }
    }
    let cut = cut_to(&state, 0, 4, 1000).expect("a cut");
    assert_eq!((cut.links, cut.cost), (2, 40));
    // The two cells next to the HQ, (1, 0) and (0, 1): every other cell goes.
    assert_eq!(cut.cells, 9);
    // No single capture removes as much: 39 gold buys one unit cell only.
    assert!(best_affordable(&state, 0, 39).is_none_or(|c| c.cells < 9));
}

#[test]
fn a_tower_raises_the_price_of_the_cells_it_protects() {
    let mut state = rich(80);
    assert!(state.apply(Command::Train {
        level: 1,
        x: 0,
        y: 1
    }));
    assert_eq!(price(&state, 0, 12), 20);
    state.play_turn(&[]);
    state.play_turn(&[]);
    let id = state.unit_id(12).expect("the unit");
    assert!(state.apply(Command::Move { id, x: 0, y: 2 }));
    assert_eq!(price(&state, 0, 12), 10, "an empty cell");
    assert!(state.apply(Command::BuildTower { x: 0, y: 1 }));
    assert_eq!(price(&state, 0, 12), 30, "the tower's cell");
    assert_eq!(price(&state, 0, 24), 30, "a cell the tower protects");
    assert_eq!(price(&state, 0, 0), 30, "the HQ next to the tower");
}

#[test]
fn the_way_to_a_cell_is_paid_and_a_unit_next_to_it_steps_in_free() {
    let mut state = rich(60);
    for x in 1..5u8 {
        assert!(state.apply(Command::Train { level: 1, x, y: 0 }));
    }
    let prices = reach_prices(&state, 0);
    // Blue's land is its corner: a red unit cell costs 20 plus a long walk.
    assert!(prices[1] > 100);
    // Far more than any cut a modest budget pays: nothing is affordable.
    assert_eq!(best_with(&state, 0, 100, &prices), None);
}

#[test]
fn a_free_cell_joins_a_cut_only_when_the_cut_needs_it() {
    let mut state = rich(80);
    for x in 1..5u8 {
        assert!(state.apply(Command::Train { level: 1, x, y: 0 }));
    }
    // A leaf off the line, (2, 1), free to capture; the rest 20 each.
    assert!(state.apply(Command::Train {
        level: 1,
        x: 2,
        y: 1
    }));
    let mut prices = [INFINITE; CELLS];
    for cell in state.active(0).iter() {
        prices[cell] = 20;
    }
    prices[14] = 0;
    // To (4, 0): (1, 0) alone, which also cuts the leaf off: 5 cells.
    let cut = cut_with(&state, 0, 4, 1000, &prices).expect("a cut");
    assert_eq!((cut.cost, cut.links, cut.cells), (20, 1, 5));
    // A free cell on the HQ's side, (0, 1), is no part of the cut.
    assert!(state.apply(Command::Train {
        level: 1,
        x: 0,
        y: 1
    }));
    prices[12] = 0;
    let cut = cut_with(&state, 0, 4, 1000, &prices).expect("a cut");
    assert_eq!((cut.cost, cut.links, cut.cells), (20, 1, 5));
    // A free cell the cut needs stays in it: (1, 0) free.
    prices[1] = 0;
    let cut = cut_with(&state, 0, 4, 1000, &prices).expect("a cut");
    assert_eq!((cut.cost, cut.links, cut.cells), (0, 1, 5));
}
