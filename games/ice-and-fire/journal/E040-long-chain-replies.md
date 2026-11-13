---
id: E040
date: 2026-11-13
pull_request: "pending"
parent: ice-and-fire-v018
hypothesis: On our cuts of 3 or more enemy cells in v014's CodinGame games, v018's search predicts the evaluation after the opponent's reply too optimistically in about one case in ten (90th percentile of predicted less actual +51 on the cuts whose chain was wiped the next turn, +33 on other plies), and 46% of those cuts lost three quarters of their units the next turn (26% for the top 10); searching the opponent's replies harder after a plan of many trainings (more tries for its climb, more or longer cuts) finds the re-cuts it misses and beats v018.
change: None released. Screening only: when a plan has a chosen number of trainings or more, the opponent's replies to it searched with more tries for the opponent's climb (300 in v018), more of its best cuts (3) or longer ones (5 trainings).
release: none
sprt: not run: no gain in screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-11-13 ("More reply tries for long chains"),
after E038 and E039, which put the danger of being cut into the
evaluation and lost. Run beside E041 (defenders in the generator) and a
study of our replies to the top 10's long chains.

## Why

E038's entry gives the study of chains and trades in v014's CodinGame
games. A check of v018's search on those games (its `judge` mode, 600
tries a position, the evaluation at our next turn as the outcome) found
its predicted worst case after the opponent's replies close to the
outcome on our chains that cut nothing, and more optimistic in the tail
on our cuts: on the cuts whose chain was wiped the next turn, the 90th
percentile of predicted less actual is +51 (+33 on other plies that lost
a chain); on cuts that lost 8 or more cells, +100 (+75). E038 and E039
showed that a static term for that danger counts it twice; this
experiment looks for the missing replies instead.

## The change

`bots/plan/src/search.rs`: `replies` and `answers` take the plan's
trainings; from `LONG_PLAN` trainings (`None` in v018, which the base
commit keeps) the opponent's climb gets `LONG_REPLY_EVALS` tries, and
its best `LONG_REPLY_CUTS` cuts of up to `LONG_CUT_LENGTH` trainings are
tried. More tries cost time, so the search makes fewer climbs in a turn
with long plans.

The code is commit `010ebcc` of this branch's history (at v018's
settings), each variant one commit before it; this pull request merges
`main` and takes its code back, keeping the entry.

## Screening

Against `ice-and-fire-v018` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer). "From n" is `LONG_PLAN`;
tries, cuts and length are the opponent's climb's tries and its cuts
against such plans (300, 3 and 5 in v018).

First round, run 43485189584, seed 40040, 800 pairs each over two runners:

| Variant | Commit | From | Tries, cuts, length | Elo against v018 | Score |
| --- | --- | ---: | --- | --- | ---: |
| t4e600 | `5a8d505` | 4 | 600, 3, 5 | −0.9 [−9.3, +7.6] | 49.9% |
| t4e1200 | `eb7b29d` | 4 | 1,200, 3, 5 | −1.3 [−8.5, +5.9] | 49.8% |
| t3e600 | `24ebd77` | 3 | 600, 3, 5 | +1.3 [−6.9, +9.5] | 50.2% |
| t4c6 | `c3b38ad` | 4 | 300, 6, 5 | +8.3 [−1.0, +17.5] | 51.2% |
| t4l7 | `8a69a57` | 4 | 300, 3, 7 | −5.6 [−12.9, +1.6] | 49.2% |
| t4all | `0de89a0` | 4 | 1,200, 6, 7 | −1.3 [−11.0, +8.4] | 49.8% |

The leader again, run 43488313266, seed 40041, 1,600 pairs over four
runners: t4c6 +1.1 [−5.3, +7.4] (50.2%).

## What was learned

Searching the replies to long plans harder changed little: the
intervals are narrow because plans of 4 or more trainings are a small
share of the turns and most games play alike, and no form gained. The
search's optimism on our cuts (about one case in ten, from the study)
does not come from too few tries or too few or short cuts in its reply
search. The owner's choice before the confirmation ("Confirm the 3
leaders (Recommended)") was to send a leader that held to the SPRT and
drop the rest; none held.
