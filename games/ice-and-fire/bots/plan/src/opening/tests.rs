use ice_and_fire_engine::{Bits, Command, League, State, HQ, SIZE};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

use super::{distances, gap, turn, unit_return, GAP};

/// The start of the game on the referee's Bronze map of `seed`.
fn start(seed: u64) -> State {
    let map = generate(seed, RefereeLeague::Bronze);
    State::new(&map.void, &map.mine_spots, League::Bronze)
}

/// The cell of `(x, y)`.
fn at(x: u8, y: u8) -> usize {
    usize::from(y) * SIZE + usize::from(x)
}

/// Plays the opening for both sides from the start of the map of `seed`,
/// calling `check` on each state and the opening's commands for it, until the
/// opening ends. Returns the state at which it ended.
fn play_opening(seed: u64, mut check: impl FnMut(&State, &[Command])) -> State {
    let mut state = start(seed);
    while let Some(commands) = turn(&state) {
        check(&state, &commands);
        state.play_turn(&commands);
        assert!(
            state.game_turn() <= 30,
            "seed {seed}: the opening never ends"
        );
    }
    state
}

#[test]
fn every_command_of_the_opening_is_accepted_in_order() {
    for seed in 1..=60 {
        play_opening(seed, |state, commands| {
            let mut scratch = state.clone();
            for &command in commands {
                assert!(
                    scratch.apply(command),
                    "seed {seed}, turn {}: {command:?} refused",
                    state.game_turn()
                );
            }
        });
    }
}

#[test]
fn the_opening_ends_when_the_territories_are_near() {
    for seed in 1..=60 {
        let end = play_opening(seed, |state, _| assert!(gap(state) > GAP));
        assert!(
            end.outcome().is_some() || gap(&end) <= GAP,
            "seed {seed}: the opening stopped at a gap of {}",
            gap(&end)
        );
        assert!(turn(&end).is_none());
    }
}

#[test]
fn units_take_free_cells_and_never_the_same_one() {
    for seed in 1..=60 {
        play_opening(seed, |state, commands| {
            let me = state.to_act();
            let free = state.map().valid.and_not(state.owned(0) | state.owned(1));
            let mut scratch = state.clone();
            let mut taken = Bits::EMPTY;
            for &command in commands {
                let before = scratch.owned(me);
                scratch.apply(command);
                let new = scratch.owned(me).and_not(before);
                assert!((new & taken).is_empty(), "seed {seed}: a cell taken twice");
                assert_eq!(
                    new.and_not(free),
                    Bits::EMPTY,
                    "seed {seed}: a cell not free"
                );
                taken |= new;
            }
            // A unit with a free neighbour at the start of the turn moves,
            // unless the turn's other commands took all its free neighbours.
            for from in state.movable_units().iter() {
                let options = (Bits::cell(from).neighbours() & free).and_not(taken);
                if !options.is_empty() {
                    let (id, _) = state.unit_at(from).expect("a unit");
                    assert!(
                        commands
                            .iter()
                            .any(|c| matches!(c, Command::Move { id: moved, .. } if *moved == id)),
                        "seed {seed}, turn {}: unit {id} stayed beside a free cell",
                        state.game_turn()
                    );
                }
            }
        });
    }
}

#[test]
fn the_opening_trains_level_1_only_and_keeps_its_income() {
    for seed in 1..=60 {
        play_opening(seed, |state, commands| {
            for command in commands {
                if let Command::Train { level, .. } = command {
                    assert_eq!(*level, 1, "seed {seed}");
                }
            }
            let me = state.to_act();
            let mut scratch = state.clone();
            for &command in commands {
                scratch.apply(command);
            }
            assert!(scratch.income(me) >= 0, "seed {seed}: income below 0");
            assert!(scratch.gold(me) >= 0, "seed {seed}");
        });
    }
}

#[test]
fn the_first_turn_trains_two_units_next_to_the_start() {
    for seed in 1..=20 {
        let mut state = start(seed);
        for (seat, hq) in HQ.into_iter().enumerate() {
            let commands = turn(&state).expect("the opening plays the first turn");
            let cells: Vec<usize> = commands
                .iter()
                .filter_map(|c| match *c {
                    Command::Train { level: 1, x, y } => Some(at(x, y)),
                    _ => None,
                })
                .collect();
            assert_eq!(cells.len(), 2, "seed {seed}, seat {seat}: {commands:?}");
            for cell in cells {
                // Within two steps of the HQ (a training may follow another),
                // one more for blue, which starts on (11, 10) as well.
                assert!(
                    state.map().distance(hq, cell) <= 2 + seat as u8,
                    "seed {seed}, seat {seat}: a training at {cell}"
                );
            }
            state.play_turn(&commands);
        }
    }
}

#[test]
fn a_mine_is_built_next_to_the_hq_when_the_opening_lasts() {
    // On most maps the mine spot next to the HQ, (1, 0) for red and
    // (10, 11) for blue, gets its mine before the opening ends.
    let mut built = 0;
    for seed in 1..=60 {
        let end = play_opening(seed, |_, _| {});
        for (seat, (x, y)) in [(0, (1, 0)), (1, (10, 11))] {
            if (end.mines() & end.owned(seat)).test(at(x, y)) {
                built += 1;
            }
        }
    }
    assert!(built >= 30, "{built} mines of 120 next to the HQs");
}

#[test]
fn a_unit_pays_back_after_five_turns_of_new_cells() {
    // A unit that takes k cells, one a turn, returns k*h - k(k+1)/2 - 10.
    assert_eq!(unit_return(10, 0), -10);
    assert_eq!(unit_return(10, 1), -1);
    assert_eq!(unit_return(5, 4), 0);
    assert_eq!(unit_return(6, 5), 5);
    // At most h - 1 cells count before the horizon.
    assert_eq!(unit_return(5, 9), unit_return(5, 4));
}

#[test]
fn distances_and_gap_count_steps_over_the_map() {
    let state = start(1);
    let from_red = distances(&state, Bits::cell(HQ[0]));
    assert_eq!(from_red[HQ[0]], 0);
    assert_eq!(from_red[at(1, 0)], 1);
    assert_eq!(
        from_red[HQ[1]],
        state.map().distance(HQ[0], HQ[1]),
        "the same distance as the map's"
    );
    // At the start, red owns its HQ and blue (11, 11) and (11, 10).
    assert_eq!(gap(&state), state.map().distance(HQ[0], at(11, 10)));
}
