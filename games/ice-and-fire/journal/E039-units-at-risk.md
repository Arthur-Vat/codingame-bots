---
id: E039
date: 2026-11-13
pull_request: "pending"
parent: ice-and-fire-v018
hypothesis: In v014's CodinGame games against the top 10, 31% of the gold we spent on trainings was dead after the opponent's next ply (18% for the top 10), and 37% of our long chains (4 or more trainings in a ply) lost three quarters of their units the next turn (25% for the top 10), while v018's evaluation counts a unit its whole training cost until it dies; counting less the units the enemy can kill at its next turn, those on cells it can take and those it would cut off, makes the search train units that live and beats v018.
change: The plan bot's evaluation counts each side's units on cells the enemy can take at its next turn (and, in some variants, on the cells such a capture cuts off) a share of their training cost less.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-13 ("Go on with 1, 2 and 12 in parallel
next"), after he pointed out that 5 units killed by one is a bad trade;
the second of the proposals from the study of chains and trades in the
collected CodinGame games of v014, run beside E038 (cells cut off counted
as threatened), whose journal entry gives the study's numbers.

## The change

`bots/plan/src/eval.rs`: `at_risk(state, seat, exposed)`, the cells of a
side whose units the enemy can kill at its next turn: the cells it can
take (E023, E037), and with `RISK_CUT_OFF` the active cells it cuts off
from the HQ by taking one of them (a flood from the HQ without that
cell). `risk` takes `RISK_PERCENT` of the training cost of the units on
them off their side's score (0 in v018, which the base commit keeps). A
test checks the cells at risk on a line.

## Screening

Pending.
