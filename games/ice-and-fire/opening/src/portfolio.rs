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
    }
}

/// The sets, in the order the greedy choice added them. Each row: open,
/// contest, contest_cap, hq_div, horizon, min_turns_left, safe_spot,
/// contact_offset, contact_div, after_contact.
pub const PORTFOLIO: [Params; 16] = [
    set([3, 2, 20, 1, 19, 6, 8, 0, 1, 4]),
    set([6, 2, 20, 1, 19, 6, 8, 0, 1, 4]),
    set([3, 5, 20, 1, 11, 6, 8, 0, 1, 4]),
    set([0, 2, 20, 1, 19, 6, 8, 0, 2, 4]),
    set([3, 0, 20, 1, 19, 8, 8, 2, 1, 4]),
    set([3, 2, 20, 8, 19, 6, 8, 0, 2, 0]),
    set([3, 5, 13, 1, 19, 5, 8, 0, 1, 5]),
    set([3, 2, 7, 4, 19, 6, 8, 0, 1, 4]),
    set([5, 4, 20, 1, 12, 6, 8, 0, 2, 4]),
    set([3, 1, 11, 0, 19, 8, 8, 0, 1, 4]),
    set([3, 2, 20, 4, 19, 6, 8, 0, 1, 4]),
    set([3, 2, 11, 1, 15, 2, 3, 2, 2, 4]),
    set([2, 2, 20, 0, 19, 6, 8, 0, 1, 2]),
    set([3, 5, 20, 1, 19, 6, 5, 0, 1, 4]),
    set([5, 2, 20, 1, 19, 9, 8, 0, 1, 4]),
    set([3, 2, 20, 1, 18, 6, 8, 0, 3, 4]),
];
