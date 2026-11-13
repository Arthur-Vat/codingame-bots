---
id: E039
date: 2026-11-13
pull_request: "pending"
parent: ice-and-fire-v018
hypothesis: In v014's CodinGame games against the top 10, 31% of the gold we spent on trainings was dead after the opponent's next ply (18% for the top 10), and 37% of our long chains (4 or more trainings in a ply) lost three quarters of their units the next turn (25% for the top 10), while v018's evaluation counts a unit its whole training cost until it dies; counting less the units the enemy can kill at its next turn, those on cells it can take and those it would cut off, makes the search train units that live and beats v018.
change: None released. Screening only: the plan bot's evaluation counting each side's units on cells the enemy can take at its next turn (and, in some variants, on the cells such a capture cuts off) a share of their training cost less.
release: none
sprt: not run: weaker in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
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

The code is commit `5d011a5` of this branch's history (at v018's
settings), each variant one commit before it; this pull request merges
`main` and takes its code back, keeping the entry.

## Screening

Against `ice-and-fire-v018` on GitHub (`screen.yml`, run 43481344954,
seed 39039, 800 pairs each over two calibrated runners at CodinGame's
speed and real limit, ADR 0048; no timeout, crash or invalid answer).
"Taken" counts the units on cells the enemy can take; "cut off" also those
on the cells such a capture cuts off; the share is of their training
cost.

| Variant | Commit | Elo against v018 | Score |
| --- | --- | --- | ---: |
| taken, 25% | `769df48` | −9.6 [−22.5, +3.3] | 48.6% |
| taken, 50% | `a6059ea` | **−46.7 [−60.6, −33.0]** | 43.3% |
| taken, 100% | `3708286` | **−143.1 [−159.5, −127.2]** | 30.5% |
| cut off, 25% | `d335b5c` | **−52.1 [−65.6, −38.7]** | 42.6% |
| cut off, 50% | `8cffc1d` | **−69.1 [−83.3, −55.1]** | 40.2% |
| cut off, 75% | `5e8b222` | **−120.1 [−135.3, −105.3]** | 33.4% |
| cut off, 100% | `969b7f6` | **−176.7 [−194.5, −159.6]** | 26.6% |

Every variant lost, and the more so the heavier the weight, so the
experiment stops here without a release (the skill `run-experiment`, step
3: "if all are, stop").

## What was learned

As for E038, run beside it: the search already scores each plan by the
worst evaluation after the opponent's replies, which hold the cuts and
kills of units that the term tries to foresee, so a static discount of
the units the enemy could kill at its next turn counts the danger twice,
at the leaves where we answer first; the heavier it is, the more the
search keeps its units out of contact. E038's entry discusses the study's
efficiency gap.
