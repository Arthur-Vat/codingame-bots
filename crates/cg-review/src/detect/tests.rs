use super::{large_drops, lead_changes};
use crate::test_util::game;

#[test]
fn a_large_drop_is_found_at_the_ply_that_caused_it() {
    // Blue falls from 20 to 12 during red's ply 2: 40% and 8 cells.
    let trace = game(&[(10, 20), (10, 20), (11, 20), (11, 12), (11, 11)], Some(0));
    let moments = large_drops(&trace, "cells", 30, 6, "large cut");
    assert_eq!(moments.len(), 1);
    assert_eq!(moments[0].ply, 2);
    assert_eq!(moments[0].favours, Some(0));
    assert_eq!(moments[0].text, "red took blue's cells from 20 to 12");
    // Too small in share or in cells: nothing.
    assert!(large_drops(&trace, "cells", 50, 6, "large cut").is_empty());
    assert!(large_drops(&trace, "cells", 30, 9, "large cut").is_empty());
}

#[test]
fn a_lead_change_needs_the_margin_and_a_swing_is_counted_once() {
    // Red plies: lead +10, +12, -15 (change), -16, +30 (change).
    let trace = game(
        &[
            (20, 10),
            (0, 0),
            (22, 10),
            (0, 0),
            (5, 20),
            (0, 0),
            (4, 20),
            (0, 0),
            (40, 10),
        ],
        Some(0),
    );
    let moments = lead_changes(&trace, "cells", 10, 100);
    let kinds: Vec<(usize, &str, Option<usize>)> = moments
        .iter()
        .map(|m| (m.ply, m.kind.as_str(), m.favours))
        .collect();
    assert_eq!(
        kinds,
        vec![(4, "lead change", Some(1)), (8, "lead change", Some(0))]
    );
    // A swing of 20 or more that keeps the leader: from +10 to +30.
    let trace = game(&[(20, 10), (0, 0), (40, 10)], Some(0));
    let moments = lead_changes(&trace, "cells", 10, 20);
    assert_eq!(moments.len(), 1);
    assert_eq!(moments[0].kind, "swing");
    assert_eq!(moments[0].favours, Some(0));
}
