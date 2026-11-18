---
id: E068
date: 2026-11-18
pull_request: pending
parent: ice-and-fire-v027
hypothesis: Zasmu, first on CodinGame, holds its front with a wall of towers (75-85% of its frontier cells guarded at turns 15 to 25, against about 50% for the other top players and for us) and keeps its upkeep at 18% of its gross income; valuing each of our frontier cells guarded by one of our towers, and offering a tower wherever it would guard one, makes the search build that wall, lose fewer units to replies, and beats v027.
change: The plan bot's evaluation counts WALL for each of a side's active cells next to an enemy cell that is one of its active towers or next to one, and its search is offered a tower on every free border cell that would guard such a cell, besides E026's guarding towers.
release: none yet
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
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
