---
id: E084
date: 2026-11-26
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: An evaluation that counts the cells the enemy's next capture would cut off from our HQ, and values kept gold, makes the search keep a sound shape and the gold to answer the enemy's cut instead of spreading thin before it, and beats ice-and-fire-v032.
change: The cells cut off by the enemy's most damaging capture at its next turn cost each side, and kept gold counts more than its value, in the search's evaluation; each weight at 0 is v032.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The study of v032's 1,950 CodinGame games
(`research/codingame/2026-11-26-early-fights`, on E083's branch): before
game turn 12 the cells lost in our lost games are as often cut off from
the HQ as captured (4.7 and 4.6 a game, against 1.7 and 2.8 in our wins);
in the median large loss (3 or more cells in one enemy ply), v032 spends
its 32 gold down to 5 on cells it then loses to a cut of 2 trainings paid
from the enemy's 38. v032's threat term (E023, E037) counts the cells the
enemy can take at its next turn, not the cells their loss cuts off. The
owner (2026-11-26): count cuts in the threat first, and try ways of
spending: training, towers, or saving when the opponent has no real threat
or we have no opportunity.

## The change

In `bots/plan/src/eval.rs`, for each side, less the enemy's:

- `CUT_QUARTERS`: the active cells the enemy's most damaging capture
  among the cells it can take at its next turn (E037's `exposed`) cuts off
  from the HQ, the captured cell left out, cost this many quarters each
  ([`cut_off`]); with `CUT_PAIRS`, two neighbouring exposed cells taken
  together are tried too (a chain of two trainings). Towers that guard the
  chain's cells shrink the enemy's reach, so they count through it.
- `KEEP_QUARTERS`: each gold kept, up to `KEEP_CAP` (30), counts this many
  quarters beyond its value of 1, so that a training or a tower must gain
  more than its cost, or the gold is saved.

## Screening

| Variant | Commit | Cut (quarters a cell) | Pairs | Kept gold (quarters a gold) |
| --- | --- | --- | --- | --- |
| cut5 | b094d89a34ec | 5 | no | 0 |
| cut10 | 75784e67e76a | 10 | no | 0 |
| cut20 | 7627ce2619af | 20 | no | 0 |
| cut10-pairs | e4b0dd20bfc2 | 10 | yes | 0 |
| cut10-keep1 | e51b7a247e6f | 10 | no | 1 |
| cut10-keep2 | 42bd22da31b1 | 10 | no | 2 |
| keep2 | ee2c6ed07ffc | 0 | no | 2 |

A local check of cut10-pairs against v032, 8 pairs at CodinGame's limits
(seed 84): no fault, answers after the first 42.8 ms median (v032 41.4),
99.9% within 57.6 ms; 4 wins and 12 losses (too few games to judge).

`screen.yml` run 45836629632, on the owner's approval (2026-11-26), full
games against v032, 400 pairs per variant over 2 shards: pending.
