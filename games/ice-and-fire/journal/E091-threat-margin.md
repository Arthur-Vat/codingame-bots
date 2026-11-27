---
id: E091
date: 2026-11-27
pull_request: "pending"
parent: ice-and-fire-v032
hypothesis: Weighing each threatened cell by how much gold the enemy has to spare when it reaches it, as the cells lost in v032's games against the top 10 say, beats ice-and-fire-v032.
change: With `eval::DEEP_MARGIN = 50`, a threatened cell that the enemy reaches with 50 gold to spare costs `DEEP_QUARTERS` (7) quarters and the others `SHALLOW_QUARTERS` (4), instead of `THREAT_QUARTERS` (5) each; with and without E090's cuts; off in v032.
release: none
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

In step 0 of the cell values
([research/codingame/2026-11-27-cell-values.md](../research/codingame/2026-11-27-cell-values.md)),
a cell the attacker can pay for is lost at that ply 17 to 28% of the
time when its price is within 40 gold of the attacker's gold, and 27 to
44% when it is 50 gold or more within; a cell just out of reach is lost
1.5 to 3% of the time, one 40 gold beyond almost never. E023's single
weight (a loss of a quarter) is right on average; this splits it.

## Change

`eval::threat` runs the enemy's reach a second time with its budget less
`DEEP_MARGIN` (nothing when the budget is below it); those cells, with
E090's severed cells when `THREAT_CUTS` is on, cost `DEEP_QUARTERS`, the
other exposed cells `SHALLOW_QUARTERS`. Two variants: `cuts-margin` (with
E090's cuts) and `margin` (without).

## Screening

Pending: see E090's entry, screened in the same run.
