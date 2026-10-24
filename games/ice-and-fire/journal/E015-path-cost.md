---
id: E015
date: 2026-10-24
pull_request: "#NN"
parent: ice-and-fire-v006
hypothesis: Three quarters of the middle-game search's time goes to the shortest chain of trainings to the enemy HQ, computed three times per evaluation by a Dijkstra over arrays; the same cost found on bitboards lets the search score more plans in its time, which beats v006.
change: The shared evaluation (`ice_and_fire_tactics::eval`) takes the cost of the cheapest chain to an HQ from `chain::cost_to`, which grows the reached cells one step of 10 gold at a time on bitboards and stops at the target, instead of `Reach::new`; the costs are the same on every cell. The search, its limits and the evaluation's terms are v006's.
release: ice-and-fire-v007
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-10-24: "could you analyze what takes time during
the thinking step (except in the early game)? What are the priorities for
optimization? Is the state/board representation efficient?", then, from
the answer, "Bitboard path cost (Recommended)".

## Where the time went

A profile of v006's middle game under callgrind (self-play, 110 middle-game
turns of 4 games, each turn scoring 330 plans, as a real 24 ms turn did)
counted its instructions:

| Part | Share of the middle game |
| --- | ---: |
| The evaluation of a position | about 87% |
| of which `Reach::new`, three a position (the two sides' chains to the enemy HQ, and the threat to ours after the turn) | about 76% |
| of which the shape term | about 8% |
| The engine's moves and trainings (with the active cells' flood) | about 9% |
| The actions' list, with the cut finder | about 6% |

`Reach::new` is a Dijkstra over the 144 cells that picks its next cell by
scanning a list, about 62,000 instructions a call, where the evaluation
needs one number. The state is already bitboards (owners, active cells,
units by owner and level, mines, towers; three 64-bit words holding whole
rows), so a copy is cheap and the engine's work was about 10 to 15% of the
time.

## The change

Entering a cell costs 10 gold times the lowest level that may enter it, so
every cost is a multiple of 10: `cost_to` keeps four bitboards of the cells
first reached at the next four costs, settles them in order, and stops at
the target. A test checks it against `Reach::new` on every cell of about
480,000 (seed, league, position, seat, with or without the units' steps)
from random games in Bronze, Wood 1 and Wood 3. The tactics' timing example
(3,560 positions of random games) measures 0.2 µs a call against 4.6 µs, and
0.4 against 5.3 with the units' steps.

In self-play with the real limits the search scored 592 plans a turn
against v006's 329, and under callgrind the same work took 3.7 times fewer
instructions in the middle game. The rest is now spread out: the shape term
about 31%, the engine's moves with the flood about 34%, the actions' list
about 21%, the chains' paths about 20% (the parts overlap).

## Screening

Against `ice-and-fire-v006` at CodinGame's limits, 400 seat-swapped pairs
each, seed 15015, two games at a time:

| Variant | Elo against v006 | Score as red | Score as blue | Variant's timeouts | Answers after the first (median, 99%, 99.9%) |
| --- | --- | ---: | ---: | ---: | --- |
| cost_to | **+54.3 [+34.4, +74.6]** | 45.0% | 70.5% | 2 | 7.0, 32.1, 34.6 ms |
| cost_to, 64 climbs (32) | **+52.5 [+33.6, +71.7]** | 40.5% | 74.5% | 2 | 7.0, 32.1, 34.3 ms |

The cap on the climbs does not bind: with either cap the median answer
takes 7 ms, against v006's 22.6 ms, so most turns end before both the cap
and the time limit. The candidate keeps 32.

## Confirmation

On 600 new pairs, seed 15016: **+37.8 [+22.9, +52.8]**, 41.5% as red and
69.3% as blue, 2 timeouts in 1,200 games (v006: 2), answers after the
first (median, 99%, 99.9%) of 6.8, 32.2 and 34.7 ms. The owner chose the
candidate ("Candidate v007 + SPRT (Recommended)").

The function was then written more compactly, with the same results (the
same test), so that the paste-ready file fits: 99,887 characters, 1,012
more than v006's.

## SPRT

Pending.
