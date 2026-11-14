---
id: E060
date: 2026-11-14
pull_request: "pending"
parent: ice-and-fire-v025
hypothesis: At game turns 11 to 20 of our ranking games against the top 10, their units walk through their own land toward free or enemy cells 1.5 to 2 times as often as ours (12 to 15 unit-turns a game against 7 to 9), ours stand still next to free land more often (14 against 9 to 10) and take about 9 fewer cells a game; against the top 1, zasmu, his walks that end in a capture are twice as many and longer (4.0 a game, 2.0 turns, against our 2.0 and 1.3). The middle game scores a plan on the position after one turn, where a step toward free land changes nothing; charging each unit for its distance to the nearest cell not its side's makes each step of such a walk worth it, so idle units walk to the land they can take, and beats v025.
change: The plan bot's evaluation charges each unit of a side on its active cells 1, 2, 3 or 4 for every step beyond the first to the nearest cell not its side's, capped at 4 or 6 steps, for units of every level or level 1 only.
release: ice-and-fire-v026
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-14 (a form after the analysis of income,
upkeep and units in v024's and v025's CodinGame games: "Experiment on idle
units (Recommended)"); the owner had noticed, watching the top 1's games,
units that move for several turns before reaching an empty cell.

## Why

A scratch scan of the collected games that follows every unit by its id
(each own turn: took a cell, walked without taking one, or stayed, with or
without free land within 3 steps), turns 11 to 20, per game against the
top 10:

| Release | Side | Took a cell | Walked | Stayed, free land within 3 | Stayed, none near |
| --- | --- | ---: | ---: | ---: | ---: |
| v024 | us | 24.5 | 8.7 | 14.3 | 14.1 |
| v024 | top 10 | 30.6 | 14.6 | 8.9 | 9.8 |
| v025 | us | 21.9 | 7.3 | 14.2 | 11.6 |
| v025 | top 10 | 31.4 | 12.4 | 10.4 | 9.3 |

Against zasmu (44 games of v023 and v024): his units walked 25 unit-turns a
game at turns 11 to 20 against our 10.5, and his walks that ended in a
capture were 4.0 a game, 2.0 turns long (1.5 of 2 turns or more), against
our 2.0 and 1.3 (0.2). The same scan found that upkeep does not lose our
games (in our losses we pay less of it than the top 10, at the same share
of income; we lack gross income, that is land).

## The change

`bots/plan/src/eval.rs`: `IDLE_STEP` (0: nothing, v025, which the base
commit keeps), `IDLE_CAP` and `IDLE_LEVEL`: each unit of a side, on its
active cells and of at most that level, costs the side `IDLE_STEP` for
every step beyond the first between it and the nearest cell that is not
the side's (neutral or enemy), counted up to `IDLE_CAP` steps. Moves into
our own cells are already among the search's actions.

## Screening

Against `ice-and-fire-v025` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer). "Step" is `IDLE_STEP`, "cap"
`IDLE_CAP`, "levels" the units counted (`IDLE_LEVEL`).

First round, run 43605711478, seed 60060, 800 pairs each over two runners:

| Variant | Commit | Step | Cap | Levels | Elo against v025 | Score |
| --- | --- | ---: | ---: | --- | --- | ---: |
| s2 | `5cdac67` | 2 | 4 | all | **+56.5 [+43.7, +69.5]** | 58.1% |
| s2l1 | `89e741b` | 2 | 4 | 1 | **+55.2 [+41.7, +68.9]** | 57.9% |
| s2c6 | `fbca528` | 2 | 6 | all | **+50.3 [+36.8, +64.0]** | 57.2% |
| s4 | `2c89651` | 4 | 4 | all | **+49.4 [+36.0, +62.9]** | 57.1% |
| s1 | `8da5405` | 1 | 4 | all | **+47.6 [+35.0, +60.4]** | 56.8% |

The two best again with a neighbour, run 43607475218, seed 60061, 1,600
pairs each over four runners:

| Variant | Commit | Step | Levels | Elo against v025 | Score | Both rounds |
| --- | --- | ---: | --- | --- | ---: | --- |
| s2 | `5cdac67` | 2 | all | **+66.3 [+56.8, +75.8]** | 59.4% | about +63 over 2,400 pairs |
| s3 | `c15b1ca` | 3 | all | **+64.1 [+54.2, +74.1]** | 59.1% | |
| s2l1 | `89e741b` | 2 | 1 | **+54.5 [+44.6, +64.5]** | 57.8% | about +55 |

Every form gained about +50 or more: units that walk toward the land they
can take, as the top bots' do, are worth more than any weight of the term.
The candidate is s2 (`IDLE_STEP = 2`, `IDLE_CAP = 4`, every level), whose
bundle is the same, byte for byte, as the screened commit's. The term
costs the search about 15% of its speed (41.6 tries a millisecond at a
fixed number of tries, against v025's 48.5), which the screenings, at
CodinGame's speed, include.

## SPRT

Pending.
