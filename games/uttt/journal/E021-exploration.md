---
id: E021
date: 2026-10-15
pull_request: "#NN"
parent: uttt-v013
hypothesis: E020's search, which judges every child at once and mixes minimax values with averages, needs less exploration than E017's, so a lower exploration constant spends its visits better and the bot is stronger at CodinGame's limits.
change: `EXPLORATION` 0.2 instead of 0.3 in the MCTS bot (and the analysis program); nothing else.
release: uttt-v014
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
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
