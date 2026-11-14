---
id: E048
date: 2026-11-14
pull_request: "#318"
parent: ice-and-fire-v022
hypothesis: When ahead by 8 active cells or more, v019 to v021 put a quarter of their gold in mines (E033's bonus grows with the territory) where the top 10 of CodinGame put a sixth and the top 1 a tenth after turn 12, spending on towers instead; capping that bonus, or counting it per 6 cells or not at all once ahead, leaves the gold for land and towers and beats v022.
change: None released. Screening only: the plan bot's evaluation caps E033's mine bonus at 4 or 6, or counts it one per 6 active cells or not at all for a side that leads by at least 3 or 8 active cells.
release: none
sprt: not run: lost in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
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

E033 counts a safe mine one more for every 4 active cells of its side
(`mine_bonus`): 9 at 36 cells, against a mine's cost of 15 and its income
of 4 a turn. A large territory is usually a lead, so the bonus is largest
when the top 10 spend least on mines.

## The change

`bots/plan/src/eval.rs`: `MINE_BONUS_CAP`, the most a mine counts beyond its
income (`None`: no cap, v022), and `MINE_LEAD`, the lead in active cells
from which a side's mines count one more per `AHEAD_MINE_PER_CELLS` cells
rather than per `MINE_PER_CELLS` (`None`: never, v022). The base commit
keeps v022.

## Screening

Against `ice-and-fire-v022` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer).

Run 43569161619, seed 48048, 800 pairs each over two runners:

| Variant | Commit | Cap | Once ahead by | Mine bonus then | Elo against v022 | Score |
| --- | --- | --- | ---: | --- | --- | ---: |
| a8n | `3951600` | none | 8 | none | −4.3 [−14.6, +5.9] | 49.4% |
| a8p6 | `a6e6443` | none | 8 | one per 6 cells | −4.8 [−14.0, +4.4] | 49.3% |
| a3p6 | `573139e` | none | 3 | one per 6 cells | **−13.9 [−24.8, −3.0]** | 48.0% |
| c6 | `0d01c2b` | 6 | never | | **−40.6 [−52.1, −29.1]** | 44.2% |
| c4 | `37320e2` | 4 | never | | **−45.4 [−57.6, −33.4]** | 43.5% |

No variant gained; none was confirmed.

## What was learned

Capping E033's bonus whatever the lead lost about 40: mines on a large
territory pay. Counting them less only once well ahead changed nothing
measurable, so our larger share of mines when ahead is not what loses
games; it is spent once the game is mostly won (we win 90% to 97% of the
turns played ahead by 8 cells or more). The knob was removed: the pull
request holds the journal entry alone.
