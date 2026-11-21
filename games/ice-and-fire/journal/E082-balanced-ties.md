---
id: E082
date: 2026-11-21
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: An opening whose rules break ties between cells worth the same toward the side of the HQs' diagonal it holds less, rather than by the lowest cell index, no longer leaves the flank along its HQ's column to the enemy, and beats ice-and-fire-v032.
change: The `econ` rules' best cell for a unit or a training, and an idle unit's target, break ties by `econ::balance` (the side of the diagonal through the HQs where the side holds fewer cells), then the index (`econ::BALANCED_TIES`; false, the index alone, in v032).
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner suspected a bias when the flanks were measured on CodinGame
(E080's entry, "Who takes the flanks" and "Per seat"): in both seats we
lose more often the flank along our HQ's column (49% of red games and 43%
of blue games against the top 6 lose it by 5 or more Voronoi cells after
game turn 1) than the one along its row (41% and 20%); as red, losing it
costs most (won 37% when lost, 51% otherwise).

The check: `opening/examples/mirror.rs` plays v032's opening for both
sides, untimed, for 8 turns on 100 referee maps and on each map mirrored
across the HQs' diagonal (x and y swapped: the same game), and counts
red's active cells in the column flank (`x − y ≤ −4`) and the row flank
(`x − y ≥ 4`). With v032: 387 against 532 cells over the maps and their
mirrors, and 82 of the 100 mirrors not played as the mirror of the map.
The `econ` rules choose the best cell for a unit or a training, and an
idle unit's target, breaking ties by the lowest cell index, so from the HQ
at (0, 0) the row's (1, 0) beats the column's (0, 1) turn after turn.

The owner chose (form, 2026-11-21) to run this fix as its own experiment,
alongside E080's lane goals.

## Change

In `games/ice-and-fire/opening/src/econ.rs`, off in v032:
`BALANCED_TIES`, with `lean` (a land's cells above the HQs' diagonal less
those below it) and `balance` (on a tie, a cell on the side the land holds
less first). With it on, the mirror check gives 476 against 452 cells, the
same land in all (928 against 919); 83 mirrors still differ (other ties
remain: the beam's, the middle game's). With it off, the check gives v032's
387 and 532.

## Local duels

150 pairs each against v032 (seed 2026112109, time scale 1, tolerance
20 ms, 4 games at once), the control being the bot with the switch off:

| Bot | Full games | Cut at game turn 12 |
| --- | --- | --- |
| control (`BALANCED_TIES = false`) | −2.3 [−25.1, +20.4] | 0.0 [−15.7, +15.7] |
| `BALANCED_TIES = true` | +13.9 [−20.0, +48.1] | +25.5 [−6.0, +57.5] |

No fault; answers after the first 36.0 / 44.3 / 46.0 ms (median, 99%,
99.9%) against v032's 35.7 / 44.4 / 46.3. Both lean up, neither interval
excludes 0.
