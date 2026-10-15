---
id: E021
date: 2026-10-15
pull_request: "#133"
parent: uttt-v013
hypothesis: E020's search, which judges every child at once and mixes minimax values with averages, needs less exploration than E017's, so a lower exploration constant spends its visits better and the bot is stronger at CodinGame's limits.
change: `EXPLORATION` 0.2 instead of 0.3 in the MCTS bot (and the analysis program); nothing else.
release: uttt-v014
sprt: accepted
elo: +18.2 [+5.2, +31.3] at CodinGame's limits
pairs: 458
full_time: +18.2 [+5.2, +31.3] (the SPRT's own, ADR 0020), no timeout in 916 games, 99.9% of answers within 92.0 ms
decision: promoted
cg_rank: not reported
---

Step C4 of [the roadmap](../ROADMAP.md#c-search-with-the-nnue), the exploration constant. The owner asked for C4 on 2026-10-15 ("Ok with C4"). The time budget, the other half of C4, is a later experiment.

The constant 0.3 was screened for E017's search, with one new leaf per iteration; E018 and E020 changed the search since. In E018's screening, 0.2 looked as good as 0.3 at 20 ms.

Screening against `uttt-v013` in the arena (4-ply openings, 3 games at a time on Claude's 4-core session; at 20 ms the tolerance was 15 ms):

| Exploration | Time per move | Seed | Pairs | Wins, draws, losses | Elo against v013 |
| --- | --- | --- | --- | --- | --- |
| 0.15 | 20 ms | 1 | 100 | 72, 88, 40 | +56.1 [+22.7, +90.6] |
| 0.2 | 20 ms | 1 | 100 | 63, 92, 45 | +31.4 [+1.8, +61.3] |
| 0.25 | 20 ms | 1 | 100 | 79, 75, 46 | +57.9 [+24.3, +92.5] |
| 0.4 | 20 ms | 1 | 100 | 46, 68, 86 | -70.4 [-109.0, -33.6] |
| 0.15 | CodinGame's limits | 1 | 50 | 24, 53, 23 | +3.5 [-42.4, +49.5] |
| 0.2 | CodinGame's limits | 1 | 50 | 26, 55, 19 | +24.4 [-10.4, +59.6] |
| 0.25 | CodinGame's limits | 1 | 50 | 13, 69, 18 | -17.4 [-46.9, +11.9] |
| 0.2 | CodinGame's limits | 2 | 50 | 28, 57, 15 | +45.4 [+3.4, +88.8] |

At 20 ms everything from 0.15 to 0.25 beats v013's 0.3, and 0.4 loses clearly. At full time the gains shrink and only 0.2 stays clearly ahead: 54 wins, 112 draws and 34 losses over its 100 pairs, about +35 Elo. The candidate takes 0.2; the SPRT decides whether the gain is real.

Timeouts at full time: 2 for the candidate in 200 games, 3 for v013 in the same games, the machine's noise. The bundle, 70,188 characters, is unchanged in size.

SPRT at CodinGame's limits ([run 38691637895](https://github.com/Arthur-Vat/codingame-bots/actions/runs/38691637895), at 2247cd3):

```
Candidate: uttt-v014. Baseline: uttt-v013.
Settings: time scale 1, tolerance 0 ms, opening plies 4, seed 1, timeouts allowed in up to 1% of games.

Smoke test against uttt-random: 200 wins, 0 draws, 0 losses over 100 pairs; no faults.

SPRT against uttt-v013:
uttt-v014 vs uttt-v013: 916 games, 458 complete pairs
  uttt-v014: 221 wins, 522 draws, 173 losses (score 52.6%)
  Elo of uttt-v014 over uttt-v013: +18.2 [+5.2, +31.3] (95%, from pairs)
  pairs by uttt-v014's points (0, 1/2, 1, 3/2, 2): [11, 86, 217, 132, 12]
  uttt-v014 faults: 0 timeouts, 0 crashes, 0 invalid answers
  uttt-v013 faults: 0 timeouts, 0 crashes, 0 invalid answers
  answers after the first (median, 99%, 99.9%): uttt-v014 90.1, 90.2, 92.0 ms; uttt-v013 90.1, 90.2, 92.0 ms
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 3.01 (bounds -2.94, 2.94) after 458 pairs
Verdict: ACCEPTED: uttt-v014 is stronger than uttt-v013
```

A real but smaller gain than the screening's +35, as expected from 100 pairs. More than half the games are draws between these two bots now. Next: the time budget, C4's other half.
