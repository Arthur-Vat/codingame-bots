---
id: E017
date: 2026-10-14
pull_request: "#126"
parent: uttt-v010
hypothesis: An NNUE that predicts results better than the old value network (step B's gate) judges the search's new leaves better than the pattern policy's playouts, so the bot is stronger at CodinGame's limits.
change: The MCTS bot searches `NnueBoard`: each new leaf gets the NNUE's estimate (accumulators built from scratch) instead of a playout, the pattern policy still orders each node's children, exploration 0.3 instead of 0.5.
release: uttt-v011
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
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

The SPRT result goes here.
