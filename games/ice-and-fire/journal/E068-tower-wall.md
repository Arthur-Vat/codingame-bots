---
id: E068
date: 2026-11-18
pull_request: "#349"
parent: ice-and-fire-v027
hypothesis: Zasmu, first on CodinGame, holds its front with a wall of towers (75-85% of its frontier cells guarded at turns 15 to 25, against about 50% for the other top players and for us) and keeps its upkeep at 18% of its gross income; valuing each of our frontier cells guarded by one of our towers (offering a tower wherever it would guard one was screened too, and is off in the release) makes the search build that wall, lose fewer units to replies, and beats v027.
change: The plan bot's evaluation counts 8 (WALL) for each of a side's active cells next to an enemy cell that is one of its active towers or next to one; the search's towers stay E026's (offering a tower on every free border cell that would guard such a cell, WALL_TOWERS, screened weaker and is off).
release: ice-and-fire-v028
sprt: accepted
elo: +41.9 [+20.9, +63.2]
pairs: 400
full_time: +41.9 [+20.9, +63.2] (the SPRT's own, at CodinGame's limits)
decision: promoted
cg_rank: not reported
---

Asked by the owner on 2026-11-18: curriculum level 1 of the zasmu study
([research/codingame/2026-11-18-zasmu-strategy.md](../research/codingame/2026-11-18-zasmu-strategy.md)),
"Level 0 + Level 1", then the variants "W = 2, 4, 6 + W = 4 filtered" and
v027 as the baseline (forms).

## Why

From the study, zasmu against us:

- it builds 16 to 19 towers a game, against about 5 for the players ranked 2 to 6 and for us;
- 75 to 85% of its frontier cells are guarded by a tower at turns 15 to 25, against about 50%;
- its level 1 units live 4 to 5 turns, against 2 to 3;
- it loses 7 to 8 gold of units per reply at turns 11 to 20, against 12 for the others and 14 for us;
- its upkeep stays at 18% of gross income.

Our search builds a tower only where it lowers the cells the enemy can take at its next turn, or raises the enemy's chain to our HQ when that chain is nearly paid for (E026). The evaluation counts an active tower at 10, against the 15 gold it costs. A wall that pays off over several turns is therefore never built.

## The change

`bots/plan/src/eval.rs`: a term `wall`, worth `WALL` times the active cells of a side that touch an enemy cell and are one of its active towers or next to one. It is added for the seat and taken off for the enemy.

`bots/plan/src/actions.rs`: with `WALL_TOWERS`, a tower is also offered on each free border cell whose own cell or neighbours include such a frontier cell not yet guarded.

## Screening

Four variants against v027, each 800 pairs on `screen.yml`:

| Variant | WALL | WALL_TOWERS |
| --- | --- | --- |
| w2 | 2 | true |
| w4 | 4 | true |
| w6 | 6 | true |
| w4f | 4 | false (E026's towers only) |

### First screening (2026-11-18, run 44399254464)

800 pairs each against v027, 4 shards, no faults:

| Variant | Commit | Elo over v027 [95%] |
| --- | --- | --- |
| w2 | `fa55bcb` | +13.5 [-0.4, +27.4] |
| w4 | `d0cf4ab` | +30.0 [+15.4, +44.8] |
| w6 | `f950936` | +45.4 [+30.8, +60.2] |
| w4f | `d47d1a4` | +42.8 [+28.3, +57.4] |

The gain grows with `WALL`. At `WALL` 4, keeping E026's filter beats offering a tower on every frontier cell by about 13 Elo: the wider offer seems to cost search time or to tempt the search into towers that guard nothing threatened. The owner chose a second screening ("Second screening", form): `WALL` 6, 8 and 10 with the filter, and 8 with the wider offer.

### Second screening (2026-11-18, run 44405127201)

| Variant | Commit | Elo over v027 [95%] |
| --- | --- | --- |
| w6f | `9d2438c` | +39.3 [+24.8, +53.9] |
| w8f | `8609dba` | +41.9 [+27.2, +56.7] |
| w10f | `73c7367` | +45.4 [+30.0, +61.0] |
| w8 | `b82612f` | +27.9 [+13.6, +42.2] |

With E026's filter, `WALL` 4 to 10 form a plateau of about +40, within the noise of each run (±15). The wider offer is weaker and erratic (+30.0, +45.4, +27.9 at 4, 6, 8). The owner chose "W 8, filter kept" (form, 2026-11-19), the middle of the plateau, frozen as `ice-and-fire-v028` (43,714 characters).

## SPRT

Run [44475443122](https://github.com/Arthur-Vat/codingame-bots/actions/runs/44475443122), at b9417ea:

```
ice-and-fire-v028 vs ice-and-fire-v027: 800 games, 400 complete pairs
  ice-and-fire-v028: 448 wins, 0 draws, 352 losses (score 56.0%)
  Elo of ice-and-fire-v028 over ice-and-fire-v027: +41.9 [+20.9, +63.2] (95%, from pairs)
  ice-and-fire-v028 faults: 0 timeouts, 0 crashes, 0 invalid answers
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 3.24 (bounds -2.94, 2.94) after 400 pairs
Verdict: ACCEPTED: ice-and-fire-v028 is stronger than ice-and-fire-v027
```

The SPRT agrees with the screening of w8f (+41.9 both). Kept nit from the bot review: the test `a_tower_counts_wall_for_each_frontier_cell_it_guards` takes the frontier from `frontier` itself and does not check that `eval` uses the term; fix it with the next change to the bot.
