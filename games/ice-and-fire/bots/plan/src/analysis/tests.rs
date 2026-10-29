use cg_core::rng::Rng;
use ice_and_fire_engine::{Command, League, Outcome, State, HQ};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;
use ice_and_fire_tactics::chain::cost_to;
use ice_and_fire_tactics::eval::{evaluate, WIN};

use super::*;

/// Positions of random games on the referee's maps, every ply up to 80.
fn random_states(seeds: std::ops::Range<u64>) -> Vec<State> {
    let mut out = Vec::new();
    for seed in seeds {
        let map = generate(seed, RefereeLeague::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let mut rng = Rng::new(seed ^ 0x9e37_79b9);
        let mut commands = Vec::new();
        for _ in 0..80 {
            if state.outcome().is_some() {
                break;
            }
            out.push(state.clone());
            state.random_turn(&mut rng, &mut commands);
            state.play_turn(&commands);
        }
    }
    out
}

/// The shared evaluation of a game not over, rebuilt from its parts: the
/// sides' holdings with the towers apart, the threat to the HQ, and the
/// pressure term of this module.
fn rebuilt(state: &State, seat: usize) -> i64 {
    let side = |s: usize| {
        let active = state.active(s);
        let units: i64 = (state.units(s) & active)
            .iter()
            .filter_map(|cell| state.unit_level(cell))
            .map(|level| 10 * i64::from(level))
            .sum();
        state.gold(s) + 3 * state.income(s) + 4 * i64::from(active.count()) + units
    };
    let mut score = side(seat) - side(1 - seat) + tower_term(state, seat);
    if seat == state.to_act() {
        let mut next = state.clone();
        next.end_turn();
        if next.outcome().is_none()
            && cost_to(&next, 1 - seat, true, HQ[seat]) <= next.gold(1 - seat)
        {
            score -= WIN / 2;
        }
    }
    score + pressure_term(state, seat)
}

#[test]
fn the_terms_removed_are_exactly_those_of_the_shared_evaluation() {
    let states = random_states(0..12);
    let mut with_towers = 0;
    let mut with_pressure = 0;
    for state in &states {
        if state.outcome().is_some() {
            continue;
        }
        for seat in 0..2 {
            assert_eq!(evaluate(state, seat), rebuilt(state, seat));
            with_towers += usize::from(tower_term(state, seat) != 0);
            with_pressure += usize::from(pressure_term(state, seat) != 0);
        }
    }
    assert!(states.len() > 500, "{} states", states.len());
    assert!(with_towers > 20, "{with_towers} with a tower term");
    assert!(with_pressure > 100, "{with_pressure} with a pressure term");
}

#[test]
fn each_variant_removes_its_term_from_the_bots_evaluation() {
    for state in random_states(20..24) {
        if state.outcome().is_some() {
            continue;
        }
        let seat = state.to_act();
        let full = eval(&state, seat);
        set_variant(Variant::NoPressure);
        let no_pressure = eval(&state, seat);
        set_variant(Variant::NoTowers);
        let no_towers = eval(&state, seat);
        set_variant(Variant::Full);
        assert_eq!(no_pressure, full - pressure_term(&state, seat));
        assert_eq!(no_towers, full - tower_term(&state, seat));
        assert_eq!(eval(&state, seat), full);
    }
}

#[test]
fn a_finished_game_keeps_its_score_in_every_variant() {
    let mut state = State::new(&[false; ice_and_fire_engine::CELLS], &[], League::Bronze);
    while state.outcome().is_none() {
        state.end_turn();
    }
    let full = eval(&state, 0);
    set_variant(Variant::NoPressure);
    assert_eq!(eval(&state, 0), full);
    set_variant(Variant::Full);
    assert!(matches!(
        state.outcome(),
        Some(Outcome::Win(_) | Outcome::Draw)
    ));
}

/// A bank of `states`, as `ice-and-fire-review bank` writes it.
fn bank_text(states: &[State]) -> String {
    let mut text = String::new();
    for (ply, state) in states.iter().enumerate() {
        let seat = state.to_act();
        text.push_str(&format!(
            "position game-{ply} {ply} {}\n{}turn\n{}end\n",
            state.game_turn(),
            state.initial_input(seat),
            state.turn_input(seat)
        ));
    }
    text
}

#[test]
fn a_bank_reads_back_the_positions_written() {
    let states: Vec<State> = random_states(30..31).into_iter().skip(20).take(5).collect();
    let bank = read_bank(&bank_text(&states)).expect("a bank");
    assert_eq!(bank.len(), states.len());
    for (position, state) in bank.iter().zip(&states) {
        let read = position.state().expect("a state");
        let seat = state.to_act();
        assert_eq!(read.turn_input(read.to_act()), state.turn_input(seat));
        assert_eq!(position.game_turn, state.game_turn());
    }
    assert!(read_bank("position a 1\n").is_err());
    assert!(read_bank("position a 1 2\n0\nturn\n").is_err());
}

#[test]
fn kinds_name_the_first_command() {
    let train = |level| Command::Train { level, x: 1, y: 0 };
    assert_eq!(kind(&[]), "nothing");
    assert_eq!(kind(&[train(2), train(1)]), "train 2");
    assert_eq!(kind(&[Command::BuildTower { x: 1, y: 1 }]), "tower");
    assert_eq!(kind(&[Command::Move { id: 3, x: 1, y: 1 }]), "move");
}

#[test]
fn the_bank_measures_every_variant_and_reports_them() {
    let states: Vec<State> = random_states(40..42)
        .into_iter()
        .filter(|s| s.game_turn() >= 12 && s.outcome().is_none())
        .step_by(7)
        .take(4)
        .collect();
    let bank = read_bank(&bank_text(&states)).expect("a bank");
    let measures: Vec<Measure> = bank
        .iter()
        .map(|p| measure(p, 30, 3).expect("a measure"))
        .collect();
    for (m, position) in measures.iter().zip(&bank) {
        assert_eq!(m.plans.len(), RUNS.len());
        // The full variant is the bot's own choice, each command accepted.
        let mut scratch = position.state().expect("a state");
        for &command in &m.plans[0] {
            assert!(scratch.apply(command), "{command:?}");
        }
    }
    // The same search twice gives the same plan.
    assert_eq!(
        plan(&bank[0], Variant::NoTowers, 30).expect("a plan"),
        measures[0].plans[3]
    );
    let text = report(&measures, 0, 30, 3);
    assert!(text.contains("| no pressure |"), "{text}");
    assert!(text.contains("## The evaluation's cost"), "{text}");
}
