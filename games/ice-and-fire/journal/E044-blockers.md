---
id: E044
date: 2026-11-13
pull_request: "pending"
parent: ice-and-fire-v020
hypothesis: In the collected CodinGame games, the cells we lose on a threatened turn are mostly cut off (61%), our defence (towers on the border in 60% of cases, on a joint in 22%) holds the cells it guards but only 6% of the cells we then lose were in its zone, and our losses are the same whether we defend or not (4.8 and 4.6 cells), while the top 10 put towers on joints (35%) and lose less when they defend (2.3 against 3.2 cells); the search already finds the opponent's replies that enter our territory, so offering blockers on the cells those replies enter (towers there or next to them, or a level-2 unit there) lets it defend where the enemy strikes and beats v020.
change: The plan bot's search is also offered, at every step of a climb, towers on our cells that the opponent's remembered replies enter (and, in some variants, on our free cells next to them) or a level-2 training on those cells.
release: pending
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
keeps. A test checks the blockers of a reply on a line.

## Screening

Pending.
