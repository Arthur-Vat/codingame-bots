---
id: E018
date: 2026-10-14
pull_request: "#128"
parent: uttt-v011
hypothesis: Expanding all the children of a leaf at once and judging each with an incremental update of one accumulator ("jacekmax") evaluates many more positions per turn than E017's one leaf per iteration, and backing up their minimax value uses those estimates better than an average, so the bot is stronger at CodinGame's limits.
change: The first time the search reaches a node it creates all its children and gives each the NNUE's estimate as one visit (`Game::child_scores`, `NnueBoard` building the node's accumulator once); each node keeps the minimax value of its children, which selection uses instead of the average (`expand_all`, `minimax_weight` 1); the pattern policy no longer orders children.
release: uttt-v012
sprt: accepted
elo: +101.3 [+64.4, +140.6] at CodinGame's limits
pairs: 104
full_time: +101.3 [+64.4, +140.6] (the SPRT's own, ADR 0020), no timeout in 208 games, 99.9% of answers within 92.8 ms
decision: promoted
cg_rank: not reported
---

Step C2 of [the roadmap](../ROADMAP.md#c-search-with-the-nnue). The owner asked for it on 2026-10-14 ("Go on with 1, 2, 3").

- **Search (`cg-search`):** with `expand_all`, an iteration that reaches a node never expanded creates all its children, asks the game for their scores (`Game::child_scores`, exact where the game ends), gives each one visit with that score, proves those won or lost, and backs up the best child's score for the side to move instead of a playout's result. Each node also keeps a minimax value: its own estimate while it has no children, then one minus the best of its tried children's; after each iteration the path's values are updated from the leaf up, stopping at the first that does not change. Selection uses `minimax_weight · minimax + (1 − minimax_weight) · average` as each child's value, with weight 1 here (step C3 tunes it). Without `expand_all` the search is unchanged (the pinned results of `cg-search`'s tests hold). A node grows from 32 to 36 bytes.
- **Engine:** `NnueBoard::child_scores` builds the node's accumulators once and, for each child, copies them and adds the rows of the move (`Network::play`); a child where the side to move has a game-winning move still scores a win exactly. A test checks that each child's score equals the estimate from scratch.
- **Bot:** `EXPAND_ALL` true, `MINIMAX_WEIGHT` 1.0, `PRIORS` false: every child gets the network's estimate when it is created, so the pattern policy's order no longer matters. Its weights stay in the file for now; removing them is the next pull request. Exploration 0.3 and the time budget are v011's. The analysis program searches the same way.

Screening against `uttt-v011` in the arena (4-ply openings, seed 1, 3 games at a time on Claude's 4-core session):

| Exploration | Time per move | Tolerance | Pairs | Wins, draws, losses | Elo against v011 |
| --- | --- | --- | --- | --- | --- |
| 0.3 | 20 ms | 5 ms | 100 | 117, 36, 47 | +127 [+87, +171] |
| 0.3 | 20 ms | 15 ms | 100 | 114, 46, 40 | +135 [+92, +182] |
| 0.2 | 20 ms | 15 ms | 100 | 119, 52, 29 | +168 [+129, +213] |
| 0.5 | 20 ms | 15 ms | 100 | 112, 45, 43 | +125 [+84, +170] |
| 0.3 | CodinGame's limits | 0 | 50 | 56, 35, 9 | +177 [+124, +240] |

At 5 ms of tolerance both bots timed out about 20 times in 200 games (22 and 19), the machine's noise at 20 ms; at 15 ms, 7 and 8, then none. The three exploration constants are within each other's intervals, so the candidate keeps v011's 0.3 and step C4 tunes it. At full time the candidate had no fault, v011 two timeouts; 99.9% of the candidate's answers took at most 95.2 ms.

Iterations per 90 ms turn over 4 full-time games (seed 7, 85 searches each): a median of 21,000 for the candidate, against 42,000 for v011, but each of the candidate's iterations judges every child of a node, about 5 to 9 positions in the middle game and up to 81 after a free choice. The engine's speed example (`cargo run --release -p uttt-engine --example speed`, two runs each on Claude's session) gives 679,000 to 694,000 MCTS iterations per second with playouts, against 641,000 for `main` on the same machine: the search without `expand_all` is no slower.

The candidate's bundle takes 95,106 characters, leaving 4,894; the pattern weights, which it no longer uses, take about 10,000 of them.

SPRT at CodinGame's limits ([run 38586620224](https://github.com/Arthur-Vat/codingame-bots/actions/runs/38586620224), at e5248e7):

```
Candidate: uttt-v012. Baseline: uttt-v011.
Settings: time scale 1, tolerance 0 ms, opening plies 4, seed 1, timeouts allowed in up to 1% of games.

Smoke test against uttt-random: 200 wins, 0 draws, 0 losses over 100 pairs; no faults.

SPRT against uttt-v011:
uttt-v012 vs uttt-v011: 208 games, 104 complete pairs
  uttt-v012: 97 wins, 73 draws, 38 losses (score 64.2%)
  Elo of uttt-v012 over uttt-v011: +101.3 [+64.4, +140.6] (95%, from pairs)
  pairs by uttt-v012's points (0, 1/2, 1, 3/2, 2): [3, 13, 32, 34, 22]
  uttt-v012 faults: 0 timeouts, 0 crashes, 0 invalid answers
  uttt-v011 faults: 0 timeouts, 0 crashes, 0 invalid answers
  answers after the first (median, 99%, 99.9%): uttt-v012 90.1, 91.0, 92.8 ms; uttt-v011 90.1, 91.0, 93.0 ms
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 2.95 (bounds -2.94, 2.94) after 104 pairs
Verdict: ACCEPTED: uttt-v012 is stronger than uttt-v011
```

The largest gain at full time since E011 (+95.4), and the fastest SPRT to accept. It is smaller than the screening's +177, which had only 50 pairs. Next: step C3, selection mixing the minimax value with the average (`minimax_weight` below 1), screened locally first.
