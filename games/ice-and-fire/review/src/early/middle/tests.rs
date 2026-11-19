use ice_and_fire_engine::{Command, League, SIZE};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

use super::*;

fn command_at(cell: usize) -> (u8, u8) {
    ((cell % SIZE) as u8, (cell / SIZE) as u8)
}

/// Red trains one unit beside its HQ on turn 1 and walks it, one step a
/// turn, to the nearest middle choke; blue waits. The choke is taken by a
/// move, without a detour, every move heads toward the middle, and the
/// training is on a neutral cell.
#[test]
fn a_unit_walked_straight_to_a_middle_choke() {
    let seed = (0..200)
        .find(|&seed| {
            let map = generate(seed, RefereeLeague::Bronze);
            let state = State::new(&map.void, &map.mine_spots, League::Bronze);
            Zones::of(&state).middle_chokes().count() >= MIN_MIDDLE_CHOKES
        })
        .expect("a map with middle chokes");
    let map = generate(seed, RefereeLeague::Bronze);
    let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let mid = Zones::of(&state).middle_chokes();
    let dist = steps_from(&state, mid);
    // The neighbour of red's HQ nearest the middle.
    let start = (Bits::cell(HQ[0]).neighbours() & state.map().valid)
        .iter()
        .min_by_key(|&c| dist[c])
        .unwrap();
    let mut steps = Vec::new();
    let mut at = start;
    for _ in 0..40 {
        if state.outcome().is_some() || state.game_turn() > 20 {
            break;
        }
        let commands = if state.to_act() == 1 {
            Vec::new()
        } else if state.game_turn() == 1 {
            let (x, y) = command_at(start);
            vec![Command::Train { level: 1, x, y }]
        } else if dist[at] > 0 {
            let next = (Bits::cell(at).neighbours() & state.map().valid)
                .iter()
                .find(|&c| dist[c] + 1 == dist[at])
                .unwrap();
            let id = state.unit_id(at).unwrap();
            let (x, y) = command_at(next);
            at = next;
            vec![Command::Move { id, x, y }]
        } else {
            Vec::new()
        };
        let before = state.clone();
        state.play_turn(&commands);
        steps.push(Step {
            state: before,
            commands,
            ms: None,
        });
    }
    let reach = 1 + u32::from(dist[start]);
    let [red, blue] = sides(&steps, &state, 20).unwrap();
    assert_eq!(red.first_mid, Some(reach));
    assert_eq!(red.by_move, 1);
    assert_eq!(red.travels, vec![(u32::from(dist[start]), 0)]);
    assert_eq!((red.trainings, red.on_neutral), (1, 1));
    assert_eq!(red.gaps, vec![0]);
    assert_eq!(red.moves, u32::from(dist[start]));
    assert_eq!(red.toward, red.moves);
    assert_eq!(
        blue,
        SideStats {
            gold: blue.gold,
            held: 0.0,
            ..Default::default()
        }
    );
}

/// Random games: in every ply, the cells a side takes from the enemy are
/// those the enemy loses; gains less losses give each side's land at the
/// end; and some cells are retaken in some game.
#[test]
fn land_flows_add_up_in_random_games() {
    let mut retaken = 0;
    for seed in 0..30u64 {
        let map = generate(seed, RefereeLeague::Bronze);
        let mut state = State::new(&map.void, &map.mine_spots, League::Bronze);
        let start = [state.owned(0).count(), state.owned(1).count()];
        let mut rng = cg_core::rng::Rng::new(seed);
        let mut steps = Vec::new();
        let mut commands = Vec::new();
        while state.outcome().is_none() && state.game_turn() <= 30 {
            commands.clear();
            state.random_turn(&mut rng, &mut commands);
            let before = state.clone();
            state.play_turn(&commands);
            steps.push(Step {
                state: before,
                commands: commands.clone(),
                ms: None,
            });
        }
        let turns = 30;
        let f = flows(&steps, &state, turns);
        for s in 0..2 {
            for t in 1..=turns as usize {
                assert_eq!(
                    f[s].enemy[t] + f[s].retaken[t],
                    f[1 - s].lost[t],
                    "seed {seed}"
                );
            }
            let gained: u32 = (1..=turns as usize)
                .map(|t| f[s].neutral[t] + f[s].enemy[t] + f[s].retaken[t])
                .sum();
            let lost: u32 = f[s].lost.iter().sum();
            if state.outcome().is_none() {
                assert_eq!(
                    start[s] + gained - lost,
                    state.owned(s).count(),
                    "seed {seed}"
                );
            }
            retaken += f[s].retaken.iter().sum::<u32>();
        }
    }
    assert!(retaken > 0);
}

/// A pooled group lists its members; a plain name is its own group.
#[test]
fn groups_parse() {
    assert_eq!(
        parse_group("top=a+b"),
        ("top".to_string(), vec!["a".to_string(), "b".to_string()])
    );
    assert_eq!(
        parse_group("zasmu"),
        ("zasmu".to_string(), vec!["zasmu".to_string()])
    );
}
