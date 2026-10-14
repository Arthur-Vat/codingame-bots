---
id: E017
date: 2026-10-14
pull_request: "#126"
parent: uttt-v010
hypothesis: An NNUE that predicts results better than the old value network (step B's gate) judges the search's new leaves better than the pattern policy's playouts, so the bot is stronger at CodinGame's limits.
change: The MCTS bot searches `NnueBoard`: each new leaf gets the NNUE's estimate (accumulators built from scratch) instead of a playout, the pattern policy still orders each node's children, exploration 0.3 instead of 0.5.
release: uttt-v011
sprt: accepted
elo: +38.0 [+17.2, +59.0] at CodinGame's limits
pairs: 372
full_time: +38.0 [+17.2, +59.0] (the SPRT's own, ADR 0020), no timeout in 744 games, 99.9% of answers within 92.7 ms
decision: promoted
cg_rank: not reported
---

Step C1 of [the roadmap](../ROADMAP.md#c-search-with-the-nnue), the baseline for the next steps of C. The owner asked for it on 2026-10-14.

- **The network:** Train run 38562652097 ([report](../training/runs/38562652097/report.md)), (189 → 256) × 2 → 10, fitted for 8 epochs on run 37814220623's 2 million games of v008's search. Quantized, its held-out squared error is 0.1498, against 0.1522 for E012's larger value network on the same positions. It is in the bot as `nnue_weights.rs`, packed in 27,264 characters.
- **Room:** #125 packed the pattern weights and took E012's value network out of the bots. The candidate's bundle takes 91,929 characters.
- **Search:** a side to move with a game-winning move scores a win exactly; any other new leaf gets `Network::evaluate_board`. Tree reuse, proofs and the time budget are v010's. The analysis program searches the same way.

Screening against `uttt-v010` in the arena (4-ply openings, seed 1, 3 games at a time on Claude's 4-core session, which other work also loaded; at 20 ms the arena's time tolerance was 15 ms, except 5 ms for the first row):

| Exploration | Time per move | Pairs | Wins, draws, losses | Elo against v010 |
| --- | --- | --- | --- | --- |
| 0.5 | 20 ms | 100 | 92, 23, 85 | +12 [-33, +58] |
| 0.3 | 20 ms | 100 | 108, 28, 64 | +78 [+31, +128] |
| 0.2 | 20 ms | 100 | 99, 34, 67 | +56 [+18, +96] |
| 0.3 | CodinGame's limits | 50 | 47, 26, 27 | +70 [+19, +126] |

Unlike E012's network, this one keeps its lead at full time. Iterations per 90 ms turn, averaged over 4 full-time games: 66,000 early, 38,000 in the middle and 83,000 late for the candidate, against 54,000, 69,000 and 74,000 for v010. Mid-game evaluations cost more than the short playouts there; step C2's incremental updates target that.

SPRT at CodinGame's limits ([run 38575778146](https://github.com/Arthur-Vat/codingame-bots/actions/runs/38575778146), at 460259c):

```
Candidate: uttt-v011. Baseline: uttt-v010.
Settings: time scale 1, tolerance 0 ms, opening plies 4, seed 1, timeouts allowed in up to 1% of games.

Smoke test against uttt-random: 200 wins, 0 draws, 0 losses over 100 pairs; no faults.

SPRT against uttt-v010:
uttt-v011 vs uttt-v010: 744 games, 372 complete pairs
  uttt-v011: 334 wins, 157 draws, 253 losses (score 55.4%)
  Elo of uttt-v011 over uttt-v010: +38.0 [+17.2, +59.0] (95%, from pairs)
  pairs by uttt-v011's points (0, 1/2, 1, 3/2, 2): [37, 44, 159, 65, 67]
  uttt-v011 faults: 0 timeouts, 0 crashes, 0 invalid answers
  uttt-v010 faults: 0 timeouts, 0 crashes, 0 invalid answers
  answers after the first (median, 99%, 99.9%): uttt-v011 90.1, 90.8, 92.7 ms; uttt-v010 90.1, 90.7, 92.9 ms
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 2.97 (bounds -2.94, 2.94) after 372 pairs
Verdict: ACCEPTED: uttt-v011 is stronger than uttt-v010
```

The largest gain at full time since E011 (+95.4), from the first network the bot keeps. It is smaller than the screening's +70, which had only 50 pairs. Next: step C2, incremental updates of the accumulators.
