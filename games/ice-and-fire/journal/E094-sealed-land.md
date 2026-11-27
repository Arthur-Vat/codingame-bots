---
id: E094
date: 2026-11-27
pull_request: "pending"
parent: ice-and-fire-v032
hypothesis: Counting the neutral cells a side has sealed off from the enemy, which the top players hold more of than v032 and which tell the winner where the land is level, makes the search take the cells that open a larger territory, and beats ice-and-fire-v032.
change: With `eval::SEALED` quarters and `SEALED_UNTIL`, each neutral cell the enemy cannot reach from its active land without entering one of a side's cells counts for that side, net of the enemy's, until that game turn (`sealed`, one flood from each side's enemy); 0, nothing, in v032.
release: none
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner's cell values (2026-11-27): a cell's importance is also the
larger territory it opens. Measured as sealed land on v032's 2,350 test
games against the top 10 (research/codingame/2026-11-27-cell-values.md,
"Sealed land"): a sealed cell tells the winner a tenth to a fifth as well
as an active one (logistic fit, game turns 8 to 12); at game turn 10, in
games level in land, 3 or more sealed cells ahead score 0.80 against 0.66;
the top players seal more than v032 (zasmu 8.8 against 5.7, karliso 11.6
against 5.1, at game turn 10). E076's Voronoi term (every cell nearer to a
side at 1) was level in screening; sealed cells are the ones the enemy can
no longer reach.

## Change

`eval::sealed_cells(state, seat)`: the valid cells that are neither
side's, less those reached by a flood from the enemy's active land over
the cells that are not `seat`'s. `eval::sealed` adds `SEALED` quarters per
sealed cell, ours less theirs, until game turn `SEALED_UNTIL`. The owner
chose two weights (2026-11-27): about a fifth and a third of an active
cell, which the shared evaluation counts 7 (4 as a cell, 3 as its income):
6 and 9 quarters; each until game turn 14 and 20.

Speed (`ice-and-fire-bot-plan speed`, 2,000 tries on E090's 300 positions,
the fastest of 2 runs): `off` 22.6 tries a ms, `w9-t20` 24.1: no cost.

## Screening

Variants `off` (everything off), `w6-t14`, `w6-t20`, `w9-t14`, `w9-t20`
against v032, whole games at CodinGame's limits, 800 pairs each in 4
shards: run 45966202678.

Pending.
