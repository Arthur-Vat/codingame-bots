---
id: E020
date: 2026-10-14
pull_request: "#132"
parent: uttt-v012
hypothesis: Judging a child in selection by half its minimax value and half the average of the values backed up through it is steadier than the minimax value alone, which one optimistic estimate can lift, so the bot is stronger at CodinGame's limits.
change: `MINIMAX_WEIGHT` 0.5 instead of 1.0 in the MCTS bot (and the analysis program); nothing else.
release: uttt-v013
sprt: accepted
elo: +88.7 [+54.9, +124.4] at CodinGame's limits
pairs: 34
full_time: +88.7 [+54.9, +124.4] (the SPRT's own, ADR 0020), no timeout in 68 games, 99.9% of answers within 93.0 ms
decision: promoted
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

Timeouts at full time: 3 for the candidate and 1 for v012 with seed 1, then 1 and 2 with seed 2: 4 against 3 in 200 games each, the machine's noise. With seed 2, 99.9% of the candidate's answers took at most 94.5 ms (v012: 94.3 ms).

The bundle, 70,188 characters since #130, is unchanged in size.

SPRT at CodinGame's limits ([run 38645624412](https://github.com/Arthur-Vat/codingame-bots/actions/runs/38645624412), at b8b78c5):

```
Candidate: uttt-v013. Baseline: uttt-v012.
Settings: time scale 1, tolerance 0 ms, opening plies 4, seed 1, timeouts allowed in up to 1% of games.

Smoke test against uttt-random: 200 wins, 0 draws, 0 losses over 100 pairs; no faults.

SPRT against uttt-v012:
uttt-v013 vs uttt-v012: 68 games, 34 complete pairs
  uttt-v013: 20 wins, 45 draws, 3 losses (score 62.5%)
  Elo of uttt-v013 over uttt-v012: +88.7 [+54.9, +124.4] (95%, from pairs)
  pairs by uttt-v013's points (0, 1/2, 1, 3/2, 2): [0, 0, 18, 15, 1]
  uttt-v013 faults: 0 timeouts, 0 crashes, 0 invalid answers
  uttt-v012 faults: 0 timeouts, 0 crashes, 0 invalid answers
  answers after the first (median, 99%, 99.9%): uttt-v013 90.1, 91.9, 93.0 ms; uttt-v012 90.1, 92.0, 93.1 ms
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 2.98 (bounds -2.94, 2.94) after 34 pairs
Verdict: ACCEPTED: uttt-v013 is stronger than uttt-v012
```

The candidate lost no pair in 34. Accepted after only 68 games, the interval is wide; even its lower end, +54.9, is a large gain for one constant. Next: step C4, the exploration constant and the time budget tuned again for this search.
