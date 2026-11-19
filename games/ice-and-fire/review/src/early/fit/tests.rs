use super::*;

/// A perfect ranking scores 1, its reverse 0, a constant one half.
#[test]
fn auc_of_simple_rankings() {
    let labels = [false, false, true, true];
    assert_eq!(auc(&[1.0, 2.0, 3.0, 4.0], &labels), Some(1.0));
    assert_eq!(auc(&[4.0, 3.0, 2.0, 1.0], &labels), Some(0.0));
    assert_eq!(auc(&[1.0; 4], &labels), Some(0.5));
    assert_eq!(auc(&[1.0, 2.0], &[true, true]), None);
}

/// On data drawn from a known logistic model, the fit finds the direction
/// of the weights, gives a constant feature no weight, and ranks well.
#[test]
fn the_fit_recovers_a_logistic_model() {
    let mut seed = 7u64;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    let (mut xs, mut ys) = (Vec::new(), Vec::new());
    for _ in 0..4000 {
        let mut x = [3.0; FEATURES];
        x[0] = next() * 10.0;
        x[1] = next();
        let t = 0.8 * (x[0] - 5.0) - 2.0 * (x[1] - 0.5);
        let p = 1.0 / (1.0 + (-t).exp());
        xs.push(x);
        ys.push(next() < p);
    }
    let model = fit(&xs, &ys);
    assert!(model.weight[0] > 0.0 && model.weight[1] < 0.0, "{model:?}");
    assert_eq!(model.weight[2], 0.0);
    // The raw slope of feature 0 is weight / scale, about 0.8.
    let slope = model.weight[0] / model.scale[0];
    assert!((slope - 0.8).abs() < 0.15, "{slope}");
    let scores: Vec<f64> = xs.iter().map(|x| model.logit(x)).collect();
    assert!(auc(&scores, &ys).unwrap() > 0.85);
}

/// The written source names every feature and both seats.
#[test]
fn the_weights_source_lists_both_models() {
    let source = weights_source(&[MODELS[0], MODELS[1]], 12, "zasmu");
    assert!(source.contains("SCORE_TURN: u32 = 12"));
    assert_eq!(source.matches("Model {").count(), 2);
    assert!(source.contains("to_centre"));
}
