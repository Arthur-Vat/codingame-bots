//! Decisive moments that the metrics alone show, the same for every game.
//! A game adds its own (for example a threat to the HQ) when it replays.

use crate::trace::{Moment, Trace, SIDES};

/// Every ply after which a side's metric `key` fell by at least `min` and
/// to at most `(100 - share)`% of what it was, as a moment of `kind` that
/// favours the other side. The ply is the one whose play caused the fall.
pub fn large_drops(trace: &Trace, key: &str, share: i64, min: i64, kind: &str) -> Vec<Moment> {
    let mut out = Vec::new();
    for ply in 0..trace.plies.len().saturating_sub(1) {
        for (side, victim) in SIDES.iter().enumerate() {
            let (Some(before), Some(after)) =
                (trace.value(ply, side, key), trace.value(ply + 1, side, key))
            else {
                continue;
            };
            if before - after >= min && after * 100 <= before * (100 - share) {
                let actor = trace.plies[ply].to_act;
                let text = if actor == side {
                    format!("{victim}'s {key} fell from {before} to {after} during its own turn")
                } else {
                    format!(
                        "{} took {victim}'s {key} from {before} to {after}",
                        SIDES[actor]
                    )
                };
                out.push(Moment {
                    ply,
                    kind: kind.to_string(),
                    favours: Some(1 - side),
                    text,
                });
            }
        }
    }
    out
}

/// The lead in `key` (red's value less blue's) at the plies where red is to
/// play, one a turn, so that each player's own start of turn does not move
/// it back and forth. A lead change is when the side ahead by at least
/// `margin` changes; a swing is when the lead moves by at least `swing` in
/// one turn without changing hands. Both are found at the first red ply
/// that shows them.
pub fn lead_changes(trace: &Trace, key: &str, margin: i64, swing: i64) -> Vec<Moment> {
    let mut out = Vec::new();
    let mut holder: Option<usize> = None;
    let mut last: Option<i64> = None;
    for (ply, p) in trace.plies.iter().enumerate() {
        if p.to_act != 0 {
            continue;
        }
        let (Some(red), Some(blue)) = (trace.value(ply, 0, key), trace.value(ply, 1, key)) else {
            continue;
        };
        let lead = red - blue;
        let ahead = if lead >= margin {
            Some(0)
        } else if lead <= -margin {
            Some(1)
        } else {
            None
        };
        let mut changed = false;
        if let Some(side) = ahead {
            if holder.is_some_and(|h| h != side) {
                out.push(Moment {
                    ply,
                    kind: "lead change".to_string(),
                    favours: Some(side),
                    text: format!(
                        "{} now leads by {} {key}, at the start of turn {}",
                        SIDES[side],
                        lead.abs(),
                        p.game_turn
                    ),
                });
                changed = true;
            }
            holder = Some(side);
        }
        if let Some(before) = last {
            let moved = lead - before;
            if !changed && moved.abs() >= swing {
                let side = if moved > 0 { 0 } else { 1 };
                out.push(Moment {
                    ply,
                    kind: "swing".to_string(),
                    favours: Some(side),
                    text: format!(
                        "the lead moved {} {key} towards {} over the last turn, to {} {}",
                        moved.abs(),
                        SIDES[side],
                        SIDES[if lead >= 0 { 0 } else { 1 }],
                        lead.abs()
                    ),
                });
            }
        }
        last = Some(lead);
    }
    out
}

#[cfg(test)]
mod tests;
