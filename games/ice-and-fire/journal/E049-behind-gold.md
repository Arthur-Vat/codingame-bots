---
id: E049
date: 2026-11-14
pull_request: "pending"
parent: ice-and-fire-v022
hypothesis: From turn 7, v019 to v021's gold counts twice in the evaluation in 93% to 100% of its turns (the shared pressure term counts it toward the chain to the enemy HQ while that chain is within 200 gold, E042's standing threat beyond), and behind by 3 active cells or more it holds a median 61 gold at turns 13 to 20 and spends 18 a turn where the top 1 spends 33; E035 found counting less of that gold everywhere lost, as it values the threat on the HQ, so counting a half or three quarters of it only for a side that is behind by 3 or 8 cells, which is far from winning by a strike, makes the search spend when it must and beats v022.
change: The plan bot's evaluation counts only 1, 2 or 3 quarters of a side's gold in the shared pressure term when the side is behind by at least 3 or 8 active cells.
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

Gold kept in the band where it counts twice (each column is a share of
the turns from turn 7, by the lead in active cells):

| Margin to the enemy HQ | v019 to v021 behind by 3+ | even | ahead by 3+ | zasmu ahead by 3+ |
| --- | ---: | ---: | ---: | ---: |
| Short by more than 200 (gold counts once) | 5% | 0% | 0% | 0% |
| Short by 1 to 200 (gold counts twice, pressure) | 93% | 97% | 77% | 87% |
| Over by 0 to 100 (gold counts twice, E042) | 1% | 3% | 23% | 13% |

E035 counted less of the gold in the pressure term for every side: three
quarters gained about +12 on v016 and nothing on v018, less lost. It did
not depend on the lead.

## The change

`bots/plan/src/eval.rs`: `BEHIND_CELLS`, the lead in active cells at or
below which a side's pressure term counts only `BEHIND_GOLD_QUARTERS`
quarters of its gold (`None`: never, v022, which the base commit keeps).
The shared term (`ice_and_fire_tactics::eval`) is unchanged; the bot adds
the difference.

## Screening

Pending.
