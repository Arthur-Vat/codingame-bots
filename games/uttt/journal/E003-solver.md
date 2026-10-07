---
id: E003
date: 2026-10-07
pull_request: pending
parent: uttt-v002
hypothesis: Proving wins and losses in the tree (MCTS-Solver) adds strength, mostly in endgames, where random playouts misjudge forced lines and searches waste time on decided positions.
change: Terminal positions are proven won or lost; a node is lost for its mover when the opponent has a proven win, and won when every answer is proven lost. Iterations stop at proven nodes, selection skips proven losses, a proven win at the root is played at once, and the search stops when the root is proven. Draws are not proven.
release: uttt-v003
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: pending
---

v003 also carries the safer time budget of #11: 82 ms of the 100 allowed
instead of 88, about 7% less search time, a cost of a few Elo that the
test measures together with the solver.

Checks before the test: on Nim, positions of 13 and 18 stones are proven
in a few hundred and a few thousand iterations (18 stones was solved 3
times in 10 by the plain search); on Ultimate Tic-Tac-Toe endgames with
at most 7 empty cells, every proof agrees with an exhaustive search, wins
and losses are all proven and draws never are.

Before the test, locally at 20 ms per move (2-core sandbox, 10 ms of
tolerance, 4-ply openings, 200 pairs): +25.2 Elo [-5.6, +56.5] against
v002, the time cut included.
