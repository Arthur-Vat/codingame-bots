---
id: E094
date: 2026-11-27
pull_request: "#394"
parent: ice-and-fire-v032
hypothesis: Counting the neutral cells a side has sealed off from the enemy, which the top players hold more of than v032 and which tell the winner where the land is level, makes the search take the cells that open a larger territory, and beats ice-and-fire-v032.
change: With `eval::SEALED` quarters and `SEALED_UNTIL`, each neutral cell the enemy cannot reach from its active land without entering one of a side's cells counts for that side, net of the enemy's, until that game turn (`sealed`, one flood from each side's enemy); 0, nothing, in v032.
release: none
sprt: not run: clearly weaker in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
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

800 seat-swapped pairs per variant against `ice-and-fire-v032`, whole
games at CodinGame's limits, 4 shards, no faults on either side
([run 45966202678](https://github.com/Arthur-Vat/codingame-bots/actions/runs/45966202678)):

| Variant | Commit | `SEALED` | `SEALED_UNTIL` | Elo over v032 [95%] | Score |
| --- | --- | ---: | ---: | ---: | ---: |
| `off` | `627f043` | 0 | 14 | +2.2 [−6.7, +11.1] | 50.3% |
| `w6-t14` | `30b225c` | 6 | 14 | −25.2 [−40.4, −10.2] | 46.4% |
| `w6-t20` | `ecf9994` | 6 | 20 | −35.3 [−50.2, −20.5] | 44.9% |
| `w9-t14` | `90059c7` | 9 | 14 | −54.3 [−69.8, −39.0] | 42.2% |
| `w9-t20` | `d47280f` | 9 | 20 | −60.5 [−76.2, −45.1] | 41.4% |

## What was learned

Dropped, and clearly: every variant loses, the more the heavier the
weight and the longer it counts, at no cost in speed. Sealed cells go with
winning in the games, but counting them makes the search close pockets
that are already safe instead of taking land and fighting at the front, as
the shape term made it train behind its own lines before E081. The
correlation runs mostly the other way: a side that wins the front seals
land behind it. A cell's opening value, as a term of the evaluation, does
not pay; if it helps at all, it would be in choosing where the opening's
units go before contact (E080's turn-1 planner), not in the middle game's
search.

The code stays on the branch `claude/ice-and-fire-e094-sealed-land`
(commits `627f043` to `d47280f`; `sealed_cells` and `sealed` in
`bots/plan/src/eval.rs`, the test `neutral_cells_behind_our_land_are_sealed`).
