---
id: E008
date: 2026-10-08
pull_request: pending
parent: uttt-v007
hypothesis: The exploration constant c = 0.5 was tuned on 2026-10-07 for a search about 2× slower, before decisive playouts; both change how much exploration pays, so another value may now be better. A different c late in the game may also help, since values are more decided there.
change: None released. Local screening only: v007 with another constant, or with another constant after its 13th searched turn, against v007.
release: none
sprt: not run, local screening only
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Screening, locally at 20 ms per move (2-core sandbox, 10 ms of
tolerance, 4-ply openings), each variant against v007 (c = 0.5) over 400
pairs:

| Variant | Elo against v007 |
| --- | --- |
| c = 0.4 | -23.1 [-43.6, -2.6] |
| c = 0.6 | +2.6 [-17.7, +22.9] |
| c = 0.7 | -34.0 [-53.2, -15.0] |
| c = 0.5, then 0.35 after the 13th searched turn | -41.5 [-61.2, -21.9] |
| c = 0.5, then 0.7 after the 13th searched turn | -17.4 [-37.6, +2.7] |

The best constant is still between 0.5 and 0.6: the curve is flat there,
and a gain of a few Elo would need thousands of pairs to prove, more than
it is worth. No release.

Exploration by phase (the owner's question): the second half of the game
is the more sensitive to too little exploration. Lowering c there cost
41 Elo, more than lowering it for the whole game (0.4: -23), and raising
it did not help either. Nothing here points to a schedule by phase; 0.5
is close to best in both halves. A schedule by number of legal moves, or
an exploration term that adapts to each node's spread of results (UCB1-
Tuned), would be different experiments.

Both bots lost a few games on time in each run (2 to 21 of 800), as
usual on this busy machine; the runs count them as losses for both.

The bot's comment on its exploration constant keeps the 2026-10-07
measurements; this entry holds the new ones, so that release candidates
in flight keep matching their source.
