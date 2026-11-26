---
id: E084
date: 2026-11-26
pull_request: "#388"
parent: ice-and-fire-v032
hypothesis: An evaluation that counts the cells the enemy's next capture would cut off from our HQ, and values kept gold, makes the search keep a sound shape and the gold to answer the enemy's cut instead of spreading thin before it, and beats ice-and-fire-v032.
change: The cells cut off by the enemy's most damaging capture at its next turn cost each side, and kept gold counts more than its value, in the search's evaluation; each weight at 0 is v032.
release: none
sprt: none
elo: "+11.3 [-9.9, +32.6] (keep2, screening)"
pairs: 400
full_time: none
decision: dropped
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
games against v032, 400 pairs per variant over 2 shards, no fault on
either side:

| Variant | Elo over v032 [95%] |
| --- | --- |
| cut5 | -36.6 [-58.4, -15.1] |
| cut10 | -41.9 [-64.9, -19.2] |
| cut20 | -58.7 [-81.6, -36.4] |
| cut10-pairs | -92.5 [-114.2, -71.4] |
| cut10-keep1 | -21.7 [-43.7, +0.0] |
| cut10-keep2 | -54.3 [-75.2, -33.8] |
| keep2 | +11.3 [-9.9, +32.6] |

Counting the cut-off cells makes the search weaker at every weight, the
more the heavier, and most with pairs of captures; kept gold alone leans
positive without an interval above 0. A likely cause, to check on games:
the shape term (E010, E081) already charges the cells one loss cuts off
near the enemy, so the cut term counts them twice and the search plays too
defensively.

### Compared with zasmu

The owner (2026-11-26), on the results: compare with zasmu. v032's 195
CodinGame games against zasmu (the first of the leaderboard; v032 won 17),
replayed as the study of the lost fights, both sides read at the start of
their plies up to game turn 12 (medians, except trainings, towers built and
cells lost: means a ply):

| Game turn | Cells v032 / zasmu | Income | Gold kept after the ply | Trainings (level 2+) | Towers built | Cells lost at the enemy's next ply |
| --- | --- | --- | --- | --- | --- | --- |
| 5 | 10 / 11 | 7 / 8 | 7 / 2 | 0.03 / 0.97 | 0 / 0 | 0 / 0 |
| 7 | 17 / 20 | 13 / 15 | 0 / 3 | 1.56 (0.12) / 0.09 (0) | 0 / 0.76 | 0 / 0.03 |
| 8 | 23 / 25 | 17 / 20 | 2 / 8 | 0.34 (0.03) / 0.45 (0.15) | 0.57 / 0.31 | 0.29 / 0.30 |
| 9 | 27 / 29 | 21 / 25 | 9 / 13 | 0.33 (0.05) / 0.70 (0.26) | 0.60 / 0.39 | 0.64 / 0.81 |
| 10 | 29 / 32 | 26 / 29 | 16 / 17 | 0.70 (0.21) / 0.69 (0.27) | 0.50 / 0.45 | 1.17 / 1.10 |
| 11 | 32 / 35 | 29 / 33 | 10 / 17 | 0.82 (0.23) / 0.97 (0.42) | 0.70 / 0.35 | 2.22 / 1.53 |
| 12 | 33 / 38 | 31 / 34 | 11 / 22 | 0.87 (0.30) / 0.85 (0.32) | 0.66 / 0.58 | 2.75 / 1.58 |

- zasmu is a cell ahead from game turn 4 (8 to 7) and the lead grows (38
  to 33 at turn 12), with as many units or fewer and more income.
- At game turn 5 v032's opening saves its gold (it trains in 3% of the
  games) while zasmu trains (97%); at turn 7 v032 spends everything on
  units (1.56 trainings, 0 gold kept) while zasmu builds a tower in 76% of
  its games, before the fights, and keeps a little gold.
- In the fights (turns 9 to 12) zasmu trains more level-2 units (0.26 to
  0.42 a ply against 0.05 to 0.30) and keeps more gold (13 to 22 against 9
  to 16), and v032 loses more at turns 11 and 12 (2.2 and 2.8 cells a ply
  against 1.5 and 1.6). Large losses before turn 12: 0.89 a game for v032,
  0.74 for zasmu.

### Are the cuts unexpected?

The owner (2026-11-26): do we miss obvious replies of the opponent; are the
losses of game turns 6 to 12 unexpected? Two runs of the review tool on
v032's 1,950 CodinGame games, with v032's own search (its analysis build)
as the judge:

- `codingame-surprise` (6,000 of our plies, 600 tries a search): at game
  turns 9 to 12, the plies followed by a large removal (3 or more of our
  cells) have a median surprise (the actual score after the opponent's real
  reply less the worst the search expected) of 0 to +9.5, and the search's
  cut finder had found the cut in 321 of 323 of them; plies with a large
  surprise (-50 or less) are 1 to 4% of those turns. The cuts are foreseen:
  the search plays into them knowingly. (The tool's `pass` column is the
  plan's value before the reply, not a counterfactual of doing nothing.)
- `codingame-judge` (6,000 plies of v032 and zasmu, 2,000 tries a search,
  far more than CodinGame's 45 ms): at game turns 7 to 12, the search with
  more time plays the same plan as v032 played in 84% of its plies, with a
  median gap of 0 and a gap above 20 in 4%; more time does not change what
  v032 plays. In zasmu's place it plays zasmu's plan in 5% of the plies and
  rates its own a median 16 higher (above 20 in 39% of them, 80 higher from
  turn 13), though it spends as much gold and buys as many trainings,
  level-2 units and towers: the plans differ in where they put them. Our
  evaluation disagrees with the player that beats v032 in 178 of 195 games:
  the losses come from the evaluation, not from the search's time or its
  model of the replies.

## Result

Dropped: weaker or level in screening (+11.3 [-9.9, +32.6] at best, keep2; confirmed level in E088). What E083 to E088 taught is in
`research/2026-11-26-early-fights-lessons.md`.
