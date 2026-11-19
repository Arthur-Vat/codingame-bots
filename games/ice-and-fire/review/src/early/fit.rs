//! The fit of the zasmu-likeness score (`early-fit`) and its check on arena
//! games (`early-check`).
//!
//! `early-fit` replays the CodinGame games and takes each side's features
//! ([`features`]) at the start of its ply of the chosen game turn. For each
//! seat it fits a logistic regression, on standardized features with an L2
//! penalty ([`PENALTY`]), that tells the reference player's sides (zasmu)
//! from the other players'; our own sides are left out of the fit. It
//! reports the fit's cross-validated AUC and whether the score predicts the
//! winner, and writes the models as the Rust source of
//! `games/ice-and-fire/early/src/score/weights.rs`.

use std::fmt::Write as _;
use std::path::PathBuf;

use ice_and_fire_early::score::{features, Model, FEATURES, MODELS, NAMES};
use ice_and_fire_early::zones::Zones;
use ice_and_fire_engine::Outcome;

use crate::profiles::{prepare, read_index, Request};
use crate::replay::{map_records, replay, Game};

/// The L2 penalty on the standardized weights: the loss is the sum of the
/// samples' log-losses plus `PENALTY / 2` times the squared weights (as
/// scikit-learn's default, `C = 1`).
pub const PENALTY: f64 = 1.0;

/// The folds of the cross-validation, by game id.
pub const FOLDS: u64 = 5;

/// One side of one game at the chosen turn.
#[derive(Clone, Debug)]
pub struct Sample {
    pub game_id: u64,
    pub seat: usize,
    /// Our side (a release), left out of the fit.
    pub ours: bool,
    /// The reference player's side.
    pub reference: bool,
    /// Whether the side won the game; `None` for a draw.
    pub won: Option<bool>,
    pub x: [f64; FEATURES],
}

/// The game's winner from the replayed end, else the record's.
fn winner(game: &Game, recorded: Option<usize>) -> Option<usize> {
    match game.end.outcome() {
        Some(Outcome::Win(w)) => Some(w),
        Some(Outcome::Draw) => None,
        None => recorded,
    }
}

/// Each side's features at the start of its ply of `turn`, if the game
/// reached it.
fn at_turn(game: &Game, turn: u32) -> [Option<[f64; FEATURES]>; 2] {
    let mut out = [None, None];
    let Some(first) = game.steps.first() else {
        return out;
    };
    let zones = Zones::of(&first.state);
    for step in &game.steps {
        let state = &step.state;
        if state.game_turn() > turn {
            break;
        }
        if state.game_turn() == turn {
            out[state.to_act()] = Some(features(state, state.to_act(), &zones));
        }
    }
    out
}

/// The samples of the CodinGame games of `request` at `turn`.
pub fn samples(request: &Request, reference: &str, turn: u32) -> Result<Vec<Sample>, String> {
    let index = request.data.join("index.jsonl");
    let (entries, _) = read_index(&index, request)?;
    let mut out = Vec::new();
    for entry in &entries {
        let Ok((meta, record)) = prepare(&request.data, entry, &request.me) else {
            continue;
        };
        let names = [
            record.players[0].name.clone(),
            record.players[1].name.clone(),
        ];
        let recorded = record.winner;
        let game = replay(&meta.game_id.to_string(), record);
        let winner = winner(&game, recorded);
        for (seat, x) in at_turn(&game, turn).into_iter().enumerate() {
            let Some(x) = x else { continue };
            out.push(Sample {
                game_id: meta.game_id,
                seat,
                ours: meta.our_seat == Some(seat),
                reference: names[seat] == reference,
                won: winner.map(|w| w == seat),
                x,
            });
        }
    }
    Ok(out)
}

/// Solves `a x = b` by Gaussian elimination with partial pivoting; `a` is
/// symmetric positive definite here, so it never meets a zero pivot.
fn solve(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Vec<f64> {
    let n = b.len();
    for col in 0..n {
        let pivot = (col..n)
            .max_by(|&i, &j| a[i][col].abs().total_cmp(&a[j][col].abs()))
            .unwrap_or(col);
        a.swap(col, pivot);
        b.swap(col, pivot);
        for row in col + 1..n {
            let f = a[row][col] / a[col][col];
            for k in col..n {
                a[row][k] -= f * a[col][k];
            }
            b[row] -= f * b[col];
        }
    }
    let mut x = vec![0.0; n];
    for row in (0..n).rev() {
        let s: f64 = (row + 1..n).map(|k| a[row][k] * x[k]).sum();
        x[row] = (b[row] - s) / a[row][row];
    }
    x
}

/// Fits a logistic regression of `ys` on `xs`, standardized, by Newton's
/// method; the bias is not penalized.
pub fn fit(xs: &[[f64; FEATURES]], ys: &[bool]) -> Model {
    let n = xs.len().max(1) as f64;
    let mut mean = [0.0; FEATURES];
    let mut scale = [1.0; FEATURES];
    for i in 0..FEATURES {
        mean[i] = xs.iter().map(|x| x[i]).sum::<f64>() / n;
        let var = xs.iter().map(|x| (x[i] - mean[i]).powi(2)).sum::<f64>() / n;
        // A constant feature keeps scale 1 (and gets weight 0).
        scale[i] = if var > 1e-12 { var.sqrt() } else { 1.0 };
    }
    let z: Vec<[f64; FEATURES]> = xs
        .iter()
        .map(|x| std::array::from_fn(|i| (x[i] - mean[i]) / scale[i]))
        .collect();
    // theta = [bias, weights...]
    let dim = FEATURES + 1;
    let mut theta = vec![0.0; dim];
    for _ in 0..50 {
        let mut grad = vec![0.0; dim];
        let mut hess = vec![vec![0.0; dim]; dim];
        for (zi, &y) in z.iter().zip(ys) {
            let row: Vec<f64> = std::iter::once(1.0).chain(zi.iter().copied()).collect();
            let t: f64 = row.iter().zip(&theta).map(|(a, b)| a * b).sum();
            let p = 1.0 / (1.0 + (-t).exp());
            let r = p - if y { 1.0 } else { 0.0 };
            let w = p * (1.0 - p);
            for a in 0..dim {
                grad[a] += r * row[a];
                for b in 0..dim {
                    hess[a][b] += w * row[a] * row[b];
                }
            }
        }
        for a in 1..dim {
            grad[a] += PENALTY * theta[a];
            hess[a][a] += PENALTY;
        }
        hess[0][0] += 1e-9;
        let step = solve(hess, grad);
        let mut largest: f64 = 0.0;
        for a in 0..dim {
            theta[a] -= step[a];
            largest = largest.max(step[a].abs());
        }
        if largest < 1e-10 {
            break;
        }
    }
    let positives = ys.iter().filter(|&&y| y).count() as f64;
    let negatives = ys.len() as f64 - positives;
    Model {
        mean,
        scale,
        weight: std::array::from_fn(|i| theta[i + 1]),
        bias: theta[0],
        prior: if positives > 0.0 && negatives > 0.0 {
            (positives / negatives).ln()
        } else {
            0.0
        },
    }
}

/// The area under the ROC curve of `scores` for `labels` (ties count
/// half); `None` without both labels.
pub fn auc(scores: &[f64], labels: &[bool]) -> Option<f64> {
    let mut pairs: Vec<(f64, bool)> = scores.iter().copied().zip(labels.iter().copied()).collect();
    pairs.sort_by(|a, b| a.0.total_cmp(&b.0));
    let positives = labels.iter().filter(|&&l| l).count() as f64;
    let negatives = labels.len() as f64 - positives;
    if positives == 0.0 || negatives == 0.0 {
        return None;
    }
    // The ranks, ties sharing their mean rank.
    let mut rank_sum = 0.0;
    let mut i = 0;
    while i < pairs.len() {
        let mut j = i;
        while j < pairs.len() && pairs[j].0 == pairs[i].0 {
            j += 1;
        }
        let mean_rank = (i + j + 1) as f64 / 2.0;
        rank_sum += pairs[i..j].iter().filter(|p| p.1).count() as f64 * mean_rank;
        i = j;
    }
    Some((rank_sum - positives * (positives + 1.0) / 2.0) / (positives * negatives))
}

/// The games' red-minus-blue differences of `value` and red's wins, for the
/// games whose two sides are both in `samples` and that `keep` accepts
/// (draws left out).
fn differences(
    samples: &[Sample],
    value: impl Fn(&Sample) -> f64,
    keep: impl Fn(u64) -> bool,
) -> (Vec<f64>, Vec<bool>) {
    let mut by_game: std::collections::BTreeMap<u64, [Option<&Sample>; 2]> = Default::default();
    for s in samples {
        by_game.entry(s.game_id).or_default()[s.seat] = Some(s);
    }
    let (mut diffs, mut wins) = (Vec::new(), Vec::new());
    for (id, sides) in by_game {
        if let [Some(red), Some(blue)] = sides {
            if let Some(won) = red.won {
                if keep(id) {
                    diffs.push(value(red) - value(blue));
                    wins.push(won);
                }
            }
        }
    }
    (diffs, wins)
}

fn show(auc: Option<f64>) -> String {
    auc.map_or_else(|| "-".to_string(), |a| format!("{a:.3}"))
}

/// The fitted models (red, blue) and the report of `early-fit`.
pub fn run(request: &Request, reference: &str, turn: u32) -> Result<([Model; 2], String), String> {
    let samples = samples(request, reference, turn)?;
    let mut report = String::new();
    let _ = writeln!(
        report,
        "# Zasmu-likeness fit\n\nReference player: {reference}; game turn {turn}; {} sides of {} CodinGame games (our sides left out of the fit).\n",
        samples.len(),
        samples.iter().map(|s| s.game_id).collect::<std::collections::BTreeSet<_>>().len()
    );
    let _ = writeln!(report, "| Seat | Sides fitted | Reference sides | Cross-validated AUC |\n| --- | ---: | ---: | ---: |");
    let mut models = [MODELS[0], MODELS[1]];
    // The score of each sample: out of fold for the fitted sides, the full
    // model for ours.
    let mut scores = vec![0.0; samples.len()];
    for seat in 0..2 {
        let fitted: Vec<usize> = (0..samples.len())
            .filter(|&i| samples[i].seat == seat && !samples[i].ours)
            .collect();
        let xs: Vec<_> = fitted.iter().map(|&i| samples[i].x).collect();
        let ys: Vec<_> = fitted.iter().map(|&i| samples[i].reference).collect();
        models[seat] = fit(&xs, &ys);
        for fold in 0..FOLDS {
            let train: Vec<usize> = fitted
                .iter()
                .copied()
                .filter(|&i| samples[i].game_id % FOLDS != fold)
                .collect();
            let model = fit(
                &train.iter().map(|&i| samples[i].x).collect::<Vec<_>>(),
                &train
                    .iter()
                    .map(|&i| samples[i].reference)
                    .collect::<Vec<_>>(),
            );
            for &i in fitted
                .iter()
                .filter(|&&i| samples[i].game_id % FOLDS == fold)
            {
                scores[i] = model.likeness(&samples[i].x);
            }
        }
        for i in (0..samples.len()).filter(|&i| samples[i].seat == seat && samples[i].ours) {
            scores[i] = models[seat].likeness(&samples[i].x);
        }
        let held: Vec<f64> = fitted.iter().map(|&i| scores[i]).collect();
        let _ = writeln!(
            report,
            "| {} | {} | {} | {} |",
            ["red", "blue"][seat],
            fitted.len(),
            ys.iter().filter(|&&y| y).count(),
            show(auc(&held, &ys))
        );
    }
    let with_reference: std::collections::BTreeSet<u64> = samples
        .iter()
        .filter(|s| s.reference)
        .map(|s| s.game_id)
        .collect();
    let index_of: std::collections::HashMap<(u64, usize), usize> = samples
        .iter()
        .enumerate()
        .map(|(i, s)| ((s.game_id, s.seat), i))
        .collect();
    let score_of = |s: &Sample| scores[index_of[&(s.game_id, s.seat)]];
    let _ = writeln!(
        report,
        "\nDoes the red-minus-blue difference at game turn {turn} predict red's win (AUC)?\n\n| Games | Count | Zasmu-likeness | Active cells |\n| --- | ---: | ---: | ---: |"
    );
    for (label, with) in [("without the reference player", false), ("with it", true)] {
        let keep = |id: u64| with_reference.contains(&id) == with;
        let (d, w) = differences(&samples, score_of, keep);
        let (a, _) = differences(&samples, |s| s.x[0], keep);
        let _ = writeln!(
            report,
            "| {label} | {} | {} | {} |",
            d.len(),
            show(auc(&d, &w)),
            show(auc(&a, &w))
        );
    }
    Ok((models, report))
}

/// The Rust source of `score/weights.rs` for `models` fitted at `turn`.
pub fn weights_source(models: &[Model; 2], turn: u32, reference: &str) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "//! Written by `ice-and-fire-review early-fit --reference {reference} --turn {turn}`\n//! on the CodinGame games of the data branch; do not edit by hand.\n\nuse super::{{Model, FEATURES}};\n\n/// The game turn the models were fitted at.\npub const SCORE_TURN: u32 = {turn};\n\n/// The models of red and blue: features {}.\npub static MODELS: [Model; 2] = [",
        NAMES.join(", ")
    );
    let list = |v: &[f64; FEATURES]| {
        v.iter()
            .map(|x| format!("{x:e}"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    for model in models {
        let _ = writeln!(
            out,
            "    Model {{\n        mean: [{}],\n        scale: [{}],\n        weight: [{}],\n        bias: {:e},\n        prior: {:e},\n    }},",
            list(&model.mean),
            list(&model.scale),
            list(&model.weight),
            model.bias,
            model.prior
        );
    }
    out.push_str("];\n\nconst _: () = assert!(FEATURES == 16);\n");
    out
}

/// A pairing's zasmu-likeness and active-cell differences (red minus
/// blue) and red's wins.
type Pairing = (Vec<f64>, Vec<f64>, Vec<bool>);

/// The report of `early-check`: on arena records, whether the
/// zasmu-likeness difference at the start of each side's ply of the
/// models' turn predicts the full game's winner, beside the active cells.
pub fn check(paths: &[PathBuf], jobs: usize) -> Result<String, String> {
    let turn = ice_and_fire_early::score::SCORE_TURN;
    let rows = map_records(paths, jobs, |game| {
        let w = winner(game, game.record.winner);
        let names = [
            game.record.players[0].name.clone(),
            game.record.players[1].name.clone(),
        ];
        (at_turn(game, turn), w, names)
    })?;
    let mut by_pairing: std::collections::BTreeMap<String, Pairing> = Default::default();
    for (sides, w, names) in rows {
        let ([Some(red), Some(blue)], Some(w)) = (sides, w) else {
            continue;
        };
        let mut pair = names.clone();
        pair.sort();
        for key in [format!("{} vs {}", pair[0], pair[1]), "all".to_string()] {
            let entry = by_pairing.entry(key).or_default();
            entry
                .0
                .push(MODELS[0].likeness(&red) - MODELS[1].likeness(&blue));
            entry.1.push(red[0] - blue[0]);
            entry.2.push(w == 0);
        }
    }
    let mut out = format!(
        "Does the red-minus-blue difference at the start of each side's ply of game turn {turn} predict the full game's winner (AUC)? Games decided before that turn are left out.\n\n| Pairing | Games | Zasmu-likeness | Active cells |\n| --- | ---: | ---: | ---: |\n"
    );
    for (key, (z, a, w)) in &by_pairing {
        let _ = writeln!(
            out,
            "| {key} | {} | {} | {} |",
            w.len(),
            show(auc(z, w)),
            show(auc(a, w))
        );
    }
    Ok(out)
}

#[cfg(test)]
mod tests;
