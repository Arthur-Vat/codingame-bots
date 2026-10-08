---
id: E014
date: 2026-10-08
pull_request: "#34"
parent: uttt-v008
hypothesis: Trying each node's children in the order of the learned playout policy, most promising first, spends the search's first visits of a node on better moves; the policy was fitted to the moves longer searches prefer.
change: The search asks the game for priors when it creates a node's children, and orders them by the priors; for Ultimate Tic-Tac-Toe the priors are the playout policy's class weights (`Board::move_weights`). A bias toward high priors in selection is available in `cg-search`, but the bot leaves it at 0.
release: uttt-v009
sprt: accepted
elo: +35.6 [+15.5, +55.9] at 20 ms
pairs: 382
full_time: +9.7 [-6.1, +25.6] over 500 pairs, 1 timeout in 1,000 games, 99.9% of answers within 92.7 ms
decision: promoted
cg_rank: pending
---

Option C of the owner's choice on 2026-10-08: the learned policy as a
guide inside the tree, without a network; a first step toward ADR 0016's
third stage, where a network gives these priors.

Before, a node's children were tried in random order, each once before
any is tried twice. Now they are tried in the order of their priors; the
search is otherwise unchanged. Children's priors are their policy
weight over the sum of their siblings', and a bias of `weight · prior /
(visits + 1)` can be added to a child's bound (progressive bias); the
bias stays 0, after the screening below.

Screening, locally against v008 (2-core sandbox, 4-ply openings):

| Bias weight | Time per move | Seed | Pairs | Elo against v008 |
| --- | --- | --- | --- | --- |
| 0 (order only) | 20 ms | 21 | 100 | +50.7 [+7.5, +95.6] |
| 0 (order only) | 20 ms | 22 | 100 | +50.7 [+8.6, +94.4] |
| 0.3 | 20 ms | 21 | 100 | +29.6 [-11.9, +72.0] |
| 1.0 | 20 ms | 21 | 100 | +34.9 [-7.2, +78.0] |
| 0 (order only) | 100 ms | 23 | 50 | -3.5 [-52.5, +45.4] |
| 0 (order only) | 100 ms | 24 | 60 | +11.6 [-32.4, +55.9] |

The order alone gains about +50 at 20 ms, but only about +5 at
CodinGame's limits over 110 pairs, within a wide interval. With more
time, nodes get enough visits that the order of their first tries
matters less; the SPRT at 20 ms will likely accept, and the confirmation
at full time measures what is left.

Speed: the engine's benchmark of 100 ms searches gave 787,000 and
935,000 iterations per second on this branch, 851,000 and 870,000 on
`main`, within the machine's noise; nodes grew from 24 to 32 bytes.

Result, from the SPRT comment on #34: smoke test 200 wins in 200 games;
SPRT accepted after 382 pairs (324 wins, 194 draws, 246 losses), LLR
2.95; confirmation at CodinGame's limits 318 wins, 392 draws, 290
losses, one timeout for v009.

At 20 ms the gain, +35.6, is below the screening's +50.7 but clear. At
full time it shrinks to +9.7, interval from -6.1 to +25.6: the
confirmation shows v009 is not weaker, as ADR 0014 asks, but proves no
gain on its own; the screening's +5 over 110 pairs pointed the same way.
As expected, ordering children by the policy matters most when nodes
have few visits, which longer searches give them. The one timeout in
1,000 games is a tenth of ADR 0015's cap; answers took as long as
v008's (99.9% within 92.7 ms for both).
