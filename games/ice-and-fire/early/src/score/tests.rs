use ice_and_fire_engine::{League, State};
use ice_and_fire_referee::map::generate;
use ice_and_fire_referee::League as RefereeLeague;

use super::*;

/// At the start, red holds its HQ alone and blue its HQ and a neighbour
/// (RULES.md): the features say so, and every model's scales are positive
/// so the score is a number.
#[test]
fn the_features_of_a_new_game() {
    let map = generate(5, RefereeLeague::Bronze);
    let state = State::new(&map.void, &map.mine_spots, League::Bronze);
    let zones = Zones::of(&state);
    let red = features(&state, 0, &zones);
    assert_eq!(red.len(), NAMES.len());
    assert_eq!(red[0], f64::from(state.active(0).count()));
    assert_eq!(red[3], 0.0);
    assert_eq!(red[6], state.gold(0) as f64);
    assert_eq!(red[14], f64::from(state.map().valid.count()));
    assert!(red[12] > 0.0 && red[12] <= f64::from(MAX_TO_CENTRE));
    for (seat, model) in MODELS.iter().enumerate() {
        assert!(model.scale.iter().all(|&s| s > 0.0));
        assert!(zasmu_likeness(&state, seat, &zones).is_finite());
    }
}

/// The log-odds is the bias plus the weighted standardized features.
#[test]
fn the_logit_standardizes_then_weighs() {
    let mut model = Model {
        mean: [1.0; FEATURES],
        scale: [2.0; FEATURES],
        weight: [0.0; FEATURES],
        bias: 0.5,
    };
    model.weight[0] = 3.0;
    let mut x = [1.0; FEATURES];
    x[0] = 5.0;
    assert!((model.logit(&x) - (0.5 + 3.0 * 2.0)).abs() < 1e-12);
}
