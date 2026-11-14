---
id: E047
date: 2026-11-14
pull_request: "pending"
parent: ice-and-fire-v022
hypothesis: Once ahead, the top 10 of CodinGame spend about a third of their gold on towers and the top 1 two thirds (turns 13 to 20), where v019 to v021 spend about a tenth when ahead by 8 cells or more and put a quarter in mines; a threatened cell of a side that leads costs it more in the evaluation, so that the search guards what it took with towers, and beats v022.
change: The plan bot's evaluation charges each threatened cell of a side that leads by at least 3 or 8 active cells 8 or 12 quarters rather than 5 (THREAT_QUARTERS, E037).
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-14 ("Merge 316, and go on with all
proposals in parallel"), one of three experiments run in parallel on
`ice-and-fire-v022` (E047 to E049) after a study of the spending of each
side by its lead in active cells, in the collected CodinGame games, and
of the top 1 (zasmu) in its 297 games against us.

## Why

A scratch scan of every turn from turn 7 (the gold held and spent, what
it bought, both sides' active cells and the margin to the enemy HQ), split
by the lead in active cells at the start of the turn. The top 10 appear
only in their games against us, and most of zasmu's against v014; its
spending was alike against v014 and against v019 to v021.

| Share of the gold spent | v019 to v021 | Top 10 | zasmu |
| --- | ---: | ---: | ---: |
| Towers, turns 7 to 12, ahead by 8 or more | 11% | 34% | 38% (ahead by 3 or more) |
| Towers, turns 13 to 20, ahead by 8 or more | 10% | 32% | 66% (ahead by 3 or more) |
| Mines, turns 7 to 12, ahead by 8 or more | 28% | 17% | 21% (ahead by 3 or more) |
| Mines, turns 13 to 20, ahead by 8 or more | 25% | 14% | 10% (ahead by 3 or more) |
| Gold spent a turn, turns 13 to 20 | 17.5 | 26 to 31 | 33.1 |

The evaluation counts a threatened cell, one the enemy can take at its
next turn, 5 quarters below a safe one (E023, E037), whoever leads. A
side that leads has more to lose and less need to take more; the top 10
then spend on towers.

## The change

`bots/plan/src/eval.rs`: `LEAD_CELLS`, the lead in active cells from which
a side's threatened cells cost `LEAD_THREAT_QUARTERS` quarters each rather
than `THREAT_QUARTERS` (`None`: never, v022, which the base commit keeps).

## Screening

Pending.
