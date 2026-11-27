---
id: E090
date: 2026-11-27
pull_request: "pending"
parent: ice-and-fire-v032
hypothesis: Counting as threatened the cells behind a cut the enemy can afford, not only the cells its chains enter, makes the search guard the cells it actually loses and beats ice-and-fire-v032.
change: With `eval::THREAT_CUTS`, the exposed cells of a side (E023, E037) also include its active cells that the loss of any one exposed cell cuts off from its HQ, at the same `THREAT_QUARTERS` each; off in v032.
release: none
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner's cell values (2026-11-27): a cell's value is its income while
we keep it, and whether we keep it depends on its price, the cheapest
chain of trainings by which the enemy takes it or cuts it off. Step 0
([research/codingame/2026-11-27-cell-values.md](../research/codingame/2026-11-27-cell-values.md))
measured it on v032's 2,350 test games against the top 10: the price with
cuts ranks the cells lost at the next ply with an AUC of 0.85 to 0.93,
ahead of the direct price that `threatened` counts (0.82 to 0.92), and 49%
(game turns 7 to 12) to 65% (from turn 13) of the cells lost are cut off,
not captured. The shape term (E010, E081) counts joints whatever the enemy
can afford; this term counts what it can afford now.

## Change

`eval::threat` replaces the body of `exposed`: the cells the enemy's
chains reach for its gold plus income, after its movers step (as before),
and with `THREAT_CUTS` the active cells cut off by the loss of any one of
them (`with_severed`: one flood from the HQ per exposed cell with at least
two active neighbours). The mine term and the tower offers read the same
cells.

## Screening

Variants screened together with E091's (one run, against v032, whole
games at CodinGame's limits): `off` (the change with every switch off,
which must play as v032), `cuts` (this entry), `cuts-margin` and `margin`
(E091).

Pending.
