//! The portfolio of weights of the opening (E004): the 16 sets of `econ`'s
//! weights that, chosen per map by a rollout against the model, scored best
//! by the opening's score in the harness (maps 1 to 1,000 against `econ`,
//! `fill` and `rush`, picked greedily from 120 candidates around v003's and
//! the offline-tuned weights, S1). The planners' steps may use any of them.

use crate::econ::Params;

/// A set of weights; `horizon` etc. as in [`Params`].
const fn set(w: [i64; 10]) -> Params {
    Params {
        open: w[0],
        contest: w[1],
        contest_cap: w[2] as u8,
        hq_div: w[3],
        horizon: w[4],
        min_turns_left: w[5],
        safe_spot: w[6] as u8,
        contact_offset: w[7],
        contact_div: w[8],
        after_contact: w[9],
        group: 0,
        thin: 0,
        spot: 0,
        inside: 0,
        toward: 0,
    }
}

/// The sets, in the order the greedy choice added them. Each row: open,
/// contest, contest_cap, hq_div, horizon, min_turns_left, safe_spot,
/// contact_offset, contact_div, after_contact.
pub const PORTFOLIO: [Params; 16] = [
    set([3, 2, 16, 2, 15, 4, 8, 3, 2, 0]),
    set([0, 2, 16, 2, 15, 5, 8, 3, 2, 0]),
    set([3, 5, 16, 2, 15, 10, 8, 3, 3, 0]),
    set([1, 2, 16, 2, 15, 7, 3, 2, 2, 0]),
    set([6, 2, 19, 0, 15, 7, 8, 3, 2, 0]),
    set([3, 2, 12, 0, 15, 6, 3, 2, 2, 0]),
    set([3, 2, 16, 1, 15, 7, 8, 4, 2, 0]),
    set([3, 2, 16, 1, 15, 7, 8, 2, 1, 0]),
    set([3, 2, 15, 8, 23, 7, 8, 3, 2, 0]),
    set([3, 2, 16, 2, 10, 8, 5, 3, 3, 0]),
    set([1, 0, 15, 2, 17, 7, 8, 3, 2, 0]),
    set([3, 2, 7, 2, 10, 7, 8, 3, 2, 2]),
    set([3, 2, 19, 2, 15, 7, 7, 3, 2, 0]),
    set([3, 5, 16, 2, 15, 8, 8, 3, 2, 0]),
    set([1, 2, 16, 8, 15, 7, 8, 3, 1, 0]),
    set([3, 2, 12, 2, 15, 6, 3, 2, 2, 0]),
];
