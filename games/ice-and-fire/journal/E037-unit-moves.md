---
id: E037
date: 2026-11-13
pull_request: "#295"
parent: ice-and-fire-v016
hypothesis: v016's threatened cells (E023) are those the enemy's chains of trainings reach for its gold plus income, so a unit at the front threatens nothing though its move costs no gold, while the top 10 of CodinGame remove 0.53 of enemy value per gold spent on units against our 0.35 and win the exchange (+30 against our -20 at turn 15), and most large cuts use moves (E019); letting the enemy's units step first in that reach makes the search value forward units and guard against them, and beats v016.
change: In the plan bot's evaluation (and its tower guard), the enemy's units of at least a chosen level on its active cells step first onto every neighbour they may enter, at no cost, before its chains of trainings, when the cells it can take at its next turn are found; the weight of a threatened cell is screened with it.
release: ice-and-fire-v017
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Asked by the owner on 2026-11-13 ("Can you do 1, 2, 3 in parallel?"),
the third of the middle-game proposals made that day from the report on
2,177 CodinGame games
([research/codingame/2026-11-11-v014-against-the-top-10.md](../research/codingame/2026-11-11-v014-against-the-top-10.md),
section 6), run beside E035 and E036.

## Why

`exposed` in `bots/plan/src/eval.rs` (E023) calls
`reach_within(state, enemy, false, budget)`: no moves, and the shared
search lets units step only for the player to act, which in the
evaluation is most often us. So the threatened cells, the mine bonus
(E033) and the tower guard (E026) ignore what the enemy's units can take
for free at its next turn, and our own forward units add nothing to the
enemy cells we threaten. The HQ threat test already counts moves.

## The change

`tactics/src/chain.rs`: `reach_within_moving`, the same search with any
set of units stepping first (`reach_within` calls it with the player to
act's movable units, as before). `bots/plan/src/eval.rs`: `reach(state,
side)`, the cells a side can take at its next turn, its units of at least
`MOVER_LEVEL` on its active cells stepping first, all of which can move
at its next turn; `exposed` and the tower guard (`actions.rs`) use it.
`THREAT_QUARTERS` is a threatened cell's weight in quarters. The
candidate sets `MOVER_LEVEL` to 2 (level-2 and level-3 units step) and
`THREAT_QUARTERS` to 5 (7 in v016), as the screened commit `599211f`.

A tactics test checks `reach_within_moving` against `reach_within` with no
movers and with the movable units of the player to act, and that every
unit stepping reaches at least as far; a plan test checks that a level-2
enemy unit with no gold to train threatens the cell next to it and a
level-1 unit does not. The candidate's code differs from the screened
commit only in comments and tests: its minified bundle is byte for byte
the bundle of `599211f` (41,243 characters).

## Screening

All against `ice-and-fire-v016` on GitHub (`screen.yml`, one calibrated
runner per variant and shard at CodinGame's speed and real limit, ADR 0048;
no timeout, crash or invalid answer except where noted). "Level n" lets
the enemy's units of level n and above step; "q" is a threatened cell's
weight in quarters (7 when not given).

First round, run 43449725616, seed 37037, 800 pairs each over two runners:

| Variant | Commit | Elo against v016 | Score |
| --- | --- | --- | ---: |
| level 1 | `5b82fe6` | −0.4 [−14.1, +13.2] | 49.9% |
| level 2 | `4ec6e6f` | **+24.4 [+11.2, +37.6]** | 53.5% |
| level 3 | `2d389d8` | +7.4 [−3.1, +17.9] | 51.1% |
| level 1, q 5 | `110d7a8` | −3.0 [−15.9, +9.8] | 49.6% |
| level 1, q 9 | `28a86e1` | **+17.4 [+4.0, +30.8]** | 52.5% |
| level 2, q 5 | `599211f` | **+24.8 [+11.4, +38.2]** | 53.6% |
| level 2, q 9 | `8da21fc` | +13.0 [−0.2, +26.3] | 51.9% |

The three best again, run 43453004938, seed 37038, 1,600 pairs each over
four runners:

| Variant | Commit | Elo against v016 | Score | Both rounds, 2,400 pairs |
| --- | --- | --- | ---: | --- |
| level 2 | `4ec6e6f` | **+17.8 [+8.9, +26.8]** | 52.6% | about +20 |
| level 2, q 5 | `599211f` | **+21.1 [+12.4, +29.8]** | 53.0% | about +22 |
| level 1, q 9 | `28a86e1` | +5.2 [−4.3, +14.8] (one timeout) | 50.7% | about +9 |

Letting level-1 units step did not help at weights 5 or 7, and at 9 only
in the first round; level-2 and level-3 units stepping helped at weights 5
and 7 alike. The candidate is
level 2, q 5.

E035 and E036 screened at the same time: their best variants (E035 three
quarters of the gold, about +13; E036 one turn of upkeep for level 2 and
3, about +10) are to be screened again on top of this candidate.
