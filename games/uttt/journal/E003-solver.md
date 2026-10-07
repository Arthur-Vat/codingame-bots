---
id: E003
date: 2026-10-07
pull_request: "#12"
parent: uttt-v002
hypothesis: Proving wins and losses in the tree (MCTS-Solver) adds strength, mostly in endgames, where random playouts misjudge forced lines and searches waste time on decided positions.
change: Terminal positions are proven won or lost; a node is lost for its mover when the opponent has a proven win, and won when every answer is proven lost. Iterations stop at proven nodes, selection skips proven losses, a proven win at the root is played at once, and the search stops when the root is proven. Draws are not proven.
release: uttt-v003
sprt: accepted
elo: +31.6 [+13.0, +50.4] at 20 ms
pairs: 479
full_time: +13.6 [-4.7, +31.9] over 500 pairs, no fault, slowest answer 87.5 ms
decision: promoted
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

Result, from the SPRT comment on #12: smoke test 200 wins in 200 games;
SPRT accepted after 479 pairs (411 wins, 225 draws, 323 losses), LLR
2.97; confirmation at CodinGame's limits 386 wins, 267 draws, 347 losses,
no fault.

At full time the gain is smaller and not significant on its own (+13.6,
interval from -4.7 to +31.9): v003 searches 82 ms against v002's 88, and
with more time the plain search finds more forced lines by itself. Both
measures point the same way, so this is not the contradiction ADR 0014
watches for, but the next confirmations should be compared with it.
The safer budget did its job: v003's slowest answer at full time was
87.5 ms, against 92.0 for v002.

The comment with these numbers was first missing: posting it failed on
GitHub's side, which also failed the check, and the stored result was
then reused without being posted. The SPRT workflow now posts each
result once, also when reused, and posting no longer decides the check.

