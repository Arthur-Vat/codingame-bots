---
id: E040
date: 2026-11-13
pull_request: "pending"
parent: ice-and-fire-v018
hypothesis: On our cuts of 3 or more enemy cells in v014's CodinGame games, v018's search predicts the evaluation after the opponent's reply too optimistically in about one case in ten (90th percentile of predicted less actual +51 on the cuts whose chain was wiped the next turn, +33 on other plies), and 46% of those cuts lost three quarters of their units the next turn (26% for the top 10); searching the opponent's replies harder after a plan of many trainings (more tries for its climb, more or longer cuts) finds the re-cuts it misses and beats v018.
change: When a plan has a chosen number of trainings or more, the opponent's replies to it are searched with more tries for the opponent's climb (300 in v018), more of its best cuts (3) or longer ones (5 trainings).
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
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

## Screening

Pending.
