---
id: E009
date: 2026-10-08
pull_request: pending
parent: uttt-v007
hypothesis: Proving draws as well as wins and losses gives the search exact values in drawn endgames, which matter more as draws grow common (38% of the games between v007 and v006 at full time).
change: None released. The solver proved finished draws and nodes whose answers are all proven with a draw as the opponent's best, and selection valued a proven draw at exactly 0.5. The code stays in the history, commit d3db1ae, reverted in the same pull request.
release: none
sprt: not run, local screening only
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Tests on the branch: on tic-tac-toe, every proof agreed with an
exhaustive search, wins, draws and losses alike. On 279 Ultimate
Tic-Tac-Toe endgames with at most 7 empty cells, every proof was right
and the move played kept the exact value; 29 of the 31 drawn endgames
were proven. A draw is proven only once every other answer is proven
too, and a search starves those answers of visits next to a proven draw,
so some draws stay unproven; the move played is still right.

Screening, locally (2-core sandbox, 4-ply openings), against v007:

| Variant | Time per move | Pairs | Elo against v007 |
| --- | --- | --- | --- |
| Proven draw counted as exactly 0.5 | 20 ms | 400 | -2.2 [-21.6, +17.2] |
| Proven draw with its usual average and exploration bonus | 20 ms | 400 | -14.8 [-33.8, +4.2] |
| Proven draw counted as exactly 0.5 | 100 ms | 150 | +4.6 [-23.2, +32.5] |

No measurable gain at either speed, so no release and no SPRT: a test of
a gain this small would need thousands of pairs. Drawn endgames are
probably rare enough, and already well judged by playouts, that exact
values change few moves.
