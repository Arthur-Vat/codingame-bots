---
id: E020
date: 2026-10-14
pull_request: "#NN"
parent: uttt-v012
hypothesis: Judging a child in selection by half its minimax value and half the average of the values backed up through it is steadier than the minimax value alone, which one optimistic estimate can lift, so the bot is stronger at CodinGame's limits.
change: `MINIMAX_WEIGHT` 0.5 instead of 1.0 in the MCTS bot (and the analysis program); nothing else.
release: uttt-v013
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Step C3 of [the roadmap](../ROADMAP.md#c-search-with-the-nnue). The owner said to keep going on 2026-10-14 ("Merge 131 and keep going"), after the plan that named C3 as the next search step.

E018 added `minimax_weight` to `cg-search`: selection values a child at `minimax_weight · minimax + (1 − minimax_weight) · average`, where the average is the mean of the scores backed up through the child. `uttt-v012` uses the minimax value alone. In the write-up, the best weight drifted toward the average as its bot got stronger; its author did not know why.

Screening against `uttt-v012` in the arena (4-ply openings, 3 games at a time on Claude's 4-core session; at 20 ms the tolerance was 15 ms):

| `minimax_weight` | Time per move | Seed | Pairs | Wins, draws, losses | Elo against v012 |
| --- | --- | --- | --- | --- | --- |
| 0 | 20 ms | 1 | 100 | 42, 72, 86 | -77.7 [-113.8, -43.2] |
| 0.25 | 20 ms | 1 | 100 | 63, 77, 60 | +5.2 [-31.1, +41.6] |
| 0.5 | 20 ms | 1 | 100 | 66, 93, 41 | +43.7 [+14.0, +74.0] |
| 0.75 | 20 ms | 1 | 100 | 72, 90, 38 | +59.6 [+31.0, +89.1] |
| 0.9 | 20 ms | 1 | 100 | 57, 106, 37 | +34.9 [+6.3, +63.9] |
| 0.75 | CodinGame's limits | 1 | 50 | 17, 69, 14 | +10.4 [-23.5, +44.6] |
| 0.5 | CodinGame's limits | 1 | 50 | 31, 59, 10 | +74.1 [+37.0, +112.9] |
| 0.75 | CodinGame's limits | 2 | 50 | 24, 63, 13 | +38.4 [+4.5, +73.0] |
| 0.5 | CodinGame's limits | 2 | 50 | 34, 54, 12 | +77.7 [+39.1, +118.3] |

The average alone is clearly worse, unlike in the write-up's bot; a mix is better than either end. At 20 ms the best weight was about 0.75, at full time 0.5: a deeper search seems to want more of the average, the drift the write-up saw. The SPRT plays at CodinGame's limits, so the candidate takes 0.5.

Timeouts at full time: 3 for the candidate and 1 for v012 with seed 1, then 1 and 2 with seed 2; the same as v012 overall, the machine's noise. With seed 2, 99.9% of the candidate's answers took at most 94.5 ms (v012: 94.3 ms).

The bundle, 70,188 characters since #130, is unchanged in size.
