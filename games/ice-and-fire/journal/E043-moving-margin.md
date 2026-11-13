---
id: E043
date: 2026-11-13
pull_request: "pending"
parent: ice-and-fire-v019
hypothesis: v019's pressure term and its standing threat (E042) price a side's chain of trainings to the enemy HQ without its units' moves, which cost no gold, while E037 found that counting the moves of level-2 and level-3 units in the cells a side can take gained +15; counting those moves in the chain to the HQ as well prices threats as the top 10 make them and beats v019.
change: In the plan bot's evaluation, the chain to the enemy HQ behind the pressure term and the standing threat lets a side's units of at least a chosen level on its active cells step first, at no cost.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-13 ("Go on with 1, 2, 3 in parallel"), the
third of the proposals made after v019 (E042): the natural next step
from E042's +20.3.

## Why

E042 counts a side that can pay at its next turn for its cheapest chain
of trainings to the enemy HQ; that chain, like the pressure term's since
E015, is found without moves (`cost_to(.., false, ..)`), though the
shared evaluation's test of an HQ that falls next turn counts the moves
of the player to act. E037 showed that the cells a side can take are
priced better with its level-2 and level-3 units stepping first.

## The change

`tactics/src/chain.rs`: `cost_to_moving`, the chain's cost with any set
of units stepping first (`cost_to` calls it as before; the unused
`flood` wrapper goes). `tactics/src/eval.rs`: `margin_moving` and
`evaluate_with_movers`, the shared evaluation with the margins found that
way (`evaluate` and `evaluate_with_margins` unchanged). `bots/plan/src/eval.rs`:
`MARGIN_MOVER_LEVEL`, the lowest level of the units whose moves count (4:
none, v019, which the base commit keeps), for both sides. A test checks
that the chain's cost without movers is `cost_to`'s and never more with
them.

## Screening

Pending.
