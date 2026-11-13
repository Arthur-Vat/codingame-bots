---
id: E043
date: 2026-11-13
pull_request: "pending"
parent: ice-and-fire-v019
hypothesis: v019's pressure term and its standing threat (E042) price a side's chain of trainings to the enemy HQ without its units' moves, which cost no gold, while E037 found that counting the moves of level-2 and level-3 units in the cells a side can take gained +15; counting those moves in the chain to the HQ as well prices threats as the top 10 make them and beats v019.
change: In the plan bot's evaluation, the chain to the enemy HQ behind the pressure term and the standing threat lets each side's level-2 and level-3 units on its active cells step first, at no cost (levels 1 to 3, and larger shares of E042's margin, screened).
release: ice-and-fire-v020
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

The candidate sets `MARGIN_MOVER_LEVEL` to 2, E042's share unchanged,
as the screened commit `769cdee`; two older tests whose positions hold a
level-2 or level-3 unit now compare the evaluation with the shared one as
the bot reads it (its movers and its standing threat). The candidate's
minified bundle is byte for byte the bundle of `769cdee` (41,684
characters).

## Screening

All against `ice-and-fire-v019` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR
0048; no timeout, crash or invalid answer). "Level" is
`MARGIN_MOVER_LEVEL` (4: no moves), "halves" E042's `HQ_MARGIN_HALVES`
(2 in v019).

First round, run 43499952610, seed 43043, 800 pairs each over two runners:

| Variant | Commit | Level | Halves | Elo against v019 | Score |
| --- | --- | ---: | ---: | --- | ---: |
| l1 | `4834fd1` | 1 | 2 | −1.7 [−14.0, +10.5] | 49.8% |
| l2 | `769cdee` | 2 | 2 | +10.4 [−2.2, +23.1] | 51.5% |
| l3 | `d587c0c` | 3 | 2 | **+9.1 [+0.2, +18.1]** | 51.3% |
| l2h4 | `a0d9ae8` | 2 | 4 | **+20.0 [+6.5, +33.6]** | 52.9% |
| l1h4 | `7c046b2` | 1 | 4 | +10.4 [−2.6, +23.5] | 51.5% |

The three best again, run 43501606830, seed 43044, 1,600 pairs each over
four runners:

| Variant | Commit | Elo against v019 | Score | Both rounds, 2,400 pairs |
| --- | --- | --- | ---: | --- |
| l2h4 | `a0d9ae8` | +5.6 [−3.2, +14.5] | 50.8% | about +10 |
| l2 | `769cdee` | **+11.1 [+2.1, +20.1]** | 51.6% | about +11 |
| l3 | `d587c0c` | +3.0 [−3.6, +9.7] | 50.4% | about +5 |

Neighbours and a control, run 43501610241, seed 43045, 800 pairs each:

| Variant | Commit | Level | Halves | Elo against v019 | Score |
| --- | --- | ---: | ---: | --- | ---: |
| h4 | `73ec86a` | 4 | 4 | −2.6 [−11.1, +5.9] | 49.6% |
| l3h4 | `49fbf7f` | 3 | 4 | **+14.3 [+3.7, +25.0]** | 52.1% |
| l2h6 | `b03bb7c` | 2 | 6 | +12.2 [−1.2, +25.5] | 51.7% |

The control (a larger share without moves) gained nothing, and l2h4's
first-round +20.0 fell to +5.6: the gain comes from the moves. The
candidate is l2, the simplest form, which held in both rounds.
