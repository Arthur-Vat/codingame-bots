---
id: E038
date: 2026-11-13
pull_request: "pending"
parent: ice-and-fire-v018
hypothesis: In v014's CodinGame games against the top 10, 31% of the gold we spent on trainings was dead after the opponent's next ply (18% for the top 10), and 46% of our long chains that cut 3 or more cells lost three quarters of their units the next turn (26% for the top 10), mostly to one capture at a joint that cut our chain off (median 4 cells cut off); v018's threatened cells count only the cells the enemy can take, not those its capture cuts off, so counting the cells cut off makes the search keep its chains holdable and beats v018.
change: The plan bot's evaluation also counts, for each side, the active cells the enemy cuts off from the HQ by taking one of the cells it can take at its next turn (all such cuts, or the largest), at a weight of their own.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-13 ("Go on with 1, 2 and 12 in parallel
next"), the first of the proposals from the study of chains and trades
in the 3,025 collected CodinGame games of v014 (below), run beside E039
(units at risk).

## Why

The owner saw long chains of our bot inside the enemy's land that cut
nothing and were cut off at once. A scan of the collected games (a
scratch command of the review tool, not committed), for every ply of
either side that trains units, measured what was dead after the
opponent's next ply. In the 2,625 games against the top 10:

| | Us | Top 10 |
| --- | ---: | ---: |
| Gold of trainings dead after the opponent's next ply | 31% | 18% |
| Long chains (4 or more trainings in a ply) a game | 1.0 | 0.8 |
| Long chains with three quarters of their units dead next turn | 37% | 25% |
| ... of 6 or 7 units | 43% | 19% |
| ... cutting 3 or more enemy cells | 46% | 26% |
| ... cutting nothing | 21% | 17% |

The reply that wipes a chain is most often one training (median) at a
joint, which cuts off a median 4 of our cells. v018's search sees such
replies: on the chains that cut nothing, its predicted worst case after
the opponent's replies was no more optimistic than the evaluation after
the real reply. But its evaluation counts a threatened cell 5/4 (E023,
E037) and an enemy cell taken about 14, and the cells behind a joint not
at all.

## The change

`bots/plan/src/eval.rs`: `cut_off(state, seat, exposed)`, the active
cells of `seat`, other than the exposed ones, that the enemy cuts off from
the HQ by taking one exposed cell (a flood from the HQ without that cell,
for each exposed cell with two or more active neighbours): their union, or
with `CUT_LARGEST` the largest single cut. Each costs its side
`CUT_QUARTERS` quarters (0 in v018, which the base commit keeps). A test
checks the cells cut off on a line.

## Screening

Pending.
