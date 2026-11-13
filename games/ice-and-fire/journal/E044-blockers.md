---
id: E044
date: 2026-11-13
pull_request: "#312"
parent: ice-and-fire-v020
hypothesis: In the collected CodinGame games, the cells we lose on a threatened turn are mostly cut off (61%), our defence (towers on the border in 60% of cases, on a joint in 22%) holds the cells it guards but only 6% of the cells we then lose were in its zone, and our losses are the same whether we defend or not (4.8 and 4.6 cells), while the top 10 put towers on joints (35%) and lose less when they defend (2.3 against 3.2 cells); the search already finds the opponent's replies that enter our territory, so offering blockers on the cells those replies enter (towers there or next to them, or a level-2 unit there) lets it defend where the enemy strikes and beats v020.
change: The plan bot's search is also offered, at every step of a climb, a tower on each of our free cells that the opponent's remembered replies enter (towers next to them and level-2 trainings screened).
release: ice-and-fire-v021
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-13 ("Go on with that order", after a study
of defensive turns: the study first, then defence aimed at the actual
threat).

## Why

A scan of the collected games (scratch commands of the review tool, not
committed) measured each turn of either side: the cells the enemy can take
at its next turn (its units stepping first) and those it would cut off,
what the side spent on defence, and what the enemy's next turn took. In
games against the top 10, from turn 6:

| | Us | Top 10 |
| --- | ---: | ---: |
| Turns with 4 or more cells at risk, or the HQ in reach | 73% | 72% |
| A tower built on such a turn | 27% | 42% |
| Cells lost at the enemy's next turn | 4.7 | 2.8 |
| ... when the side spent on defence / did not | 4.8 / 4.6 | 2.3 / 3.2 |
| Of the cells lost after defending, cut off rather than taken | 61% | 56% |
| ... inside the defended zone (a tower and its neighbours, a unit) | 6% | 9% |
| Towers on a joint / on the border | 22% / 60% | 35% / 36% |

(v014's test games, where we play red; v019's ranking games, with both
seats, gave the same pattern.) Our defence holds where it stands, but the
enemy cuts elsewhere, often next to it; 95% of the cells lost were at risk
after our turn. v018's search sees those replies (the study of chains and
trades, E038's entry) but its generator offers towers only where they
lower the cells the enemy can take (E026), mostly on the border; E041
offered towers and level-2 units at every exposed joint and gained
nothing.

## The change

`bots/plan/src/search.rs`: `blockers`, called at every step of a climb,
adds to the actions, for each of our active cells that a reply in the
search's memory enters by a training or a move: with `BLOCK_TOWERS` a
tower on it if it is free (and with `BLOCK_NEIGHBOURS` on our free cells
next to it), with `BLOCK_LEVEL` a training at that level on it if it is
empty; each once (`blockers_of`). All off in v020, which the base commit
keeps. A test checks the blockers of a reply on a line. The candidate
sets `BLOCK_TOWERS` alone, as the screened commit `dee7a3a`; its minified
bundle is byte for byte the bundle of `dee7a3a` (42,457 characters).

## Screening

All against `ice-and-fire-v020` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer).

First round, run 43520093527, seed 44044, 800 pairs each over two runners:

| Variant | Commit | Towers | Next to them | Level-2 units | Elo against v020 | Score |
| --- | --- | --- | --- | --- | --- | ---: |
| t | `dee7a3a` | yes | no | no | **+19.6 [+7.0, +32.2]** | 52.8% |
| tn | `e567115` | yes | yes | no | +2.6 [−10.0, +15.2] | 50.4% |
| u2 | `716d2cd` | no | no | yes | **−11.3 [−21.9, −0.7]** | 48.4% |
| tu2 | `e5ec898` | yes | no | yes | **+14.3 [+2.3, +26.4]** | 52.1% |
| tnu2 | `2025286` | yes | yes | yes | **+14.3 [+2.5, +26.2]** | 52.1% |

The three best again, run 43521512106, seed 44045, 1,600 pairs each over
four runners:

| Variant | Commit | Elo against v020 | Score | Both rounds, 2,400 pairs |
| --- | --- | --- | ---: | --- |
| t | `dee7a3a` | **+27.0 [+18.7, +35.3]** | 53.9% | about +25 |
| tu2 | `e5ec898` | **+25.7 [+17.2, +34.1]** | 53.7% | about +22 |
| tnu2 | `2025286` | **+17.6 [+9.4, +25.9]** | 52.5% | about +16 |

Towers on the cells the replies enter gained, alone or with level-2 units;
level-2 units alone lost, and towers on the neighbouring cells as well
diluted the gain. The candidate is t, the simplest form and the best in
both rounds.
