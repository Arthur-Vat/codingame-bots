---
id: E093
date: 2026-11-27
pull_request: "#392"
parent: ice-and-fire-v032
hypothesis: Valuing each threatened cell by its importance (a cell, a mine on it, the unit on it), with and without E092's best cut, makes the search defend what it would lose and beats ice-and-fire-v032.
change: With `eval::CELL_VALUES`, a threatened cell costs 5 quarters, 9 more with a mine, and 7, 14 or 21 more with a unit of level 1, 2 or 3 (its training cost times 5/7), instead of `THREAT_QUARTERS` (5) whatever it holds; off in v032.
release: none
sprt: not run: weaker in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
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

In the same run as E092 (see its entry):

| Variant | Commit | Elo over v032 [95%] | Score | Tries a ms |
| --- | --- | ---: | ---: | ---: |
| `off` (every switch off: plays as v032) | `fa2beb3` | +2.2 [−6.9, +11.2] | 50.3% | 25.3 |
| `best-cut` (E092) | `c1cc082` | −20.0 [−35.2, −4.9] | 47.1% | 20.3 |
| `best-cut-values` (E092 and E093) | `63424bc` | −45.4 [−61.7, −29.4] | 43.5% | 20.0 |
| `values` (E093) | `8614862` | −13.0 [−28.5, +2.4] | 48.1% | 26.0 |

800 seat-swapped pairs per variant against `ice-and-fire-v032`, whole
games at CodinGame's limits, 4 shards, no faults on either side ([run 45962787462](https://github.com/Arthur-Vat/codingame-bots/actions/runs/45962787462)).
With the switches on, two tests that pin the evaluation's numbers fail, as
the formula changes (`cells_the_enemy_can_take_next_turn_count_less`,
`a_mine_counts_more_with_a_larger_territory`); they would have been
updated for a promoted variant.

## What was learned

Dropped. `values` costs no speed and loses 13 [−28.5, +2.4]; with the best
cut, which sums the importance of everything it severs (units included),
the loss grows to 45: the threat term then outweighs the rest of the
evaluation in many positions. A cell's importance as what its loss costs
is already counted by the shared evaluation once the loss happens; adding
it to the threat counts it again, before it happens. The owner's other
meaning of importance, the territory a cell opens, is measured in the
cell-values report's section "Sealed land".
