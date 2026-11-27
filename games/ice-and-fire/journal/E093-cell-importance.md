---
id: E093
date: 2026-11-27
pull_request: "pending"
parent: ice-and-fire-v032
hypothesis: Valuing each threatened cell by its importance (a cell, a mine on it, the unit on it), with and without E092's best cut, makes the search defend what it would lose and beats ice-and-fire-v032.
change: With `eval::CELL_VALUES`, a threatened cell costs 5 quarters, 9 more with a mine, and 7, 14 or 21 more with a unit of level 1, 2 or 3 (its training cost times 5/7), instead of `THREAT_QUARTERS` (5) whatever it holds; off in v032.
release: none
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner asked (2026-11-27) for a cell value estimate based on a cell's
importance in the game. The threat term counts every cell the enemy can
take alike, while losing a mine costs its income of 4 a turn and losing a
unit its training cost. The shared evaluation counts a cell 7 (4 as a
cell, 3 as its income), a mine 12 more (its income three times) and a unit
its training cost; a threatened cell costs 5 of its 7 since E037, so each
part is scaled by the same 5/7.

## Change

`eval::importance(state, cell)`: `CELL_QUARTERS` (5), plus
`MINE_QUARTERS` (9) for a mine, plus `unit_quarters(level)` for a unit.
With `CELL_VALUES`, the exposed cells cost the sum of their importance,
and E092's best cut sums the importance of the cells it severs. Two
variants: `values` (without E092) and `best-cut-values` (with it).

## Screening

Pending: see E092's entry.
