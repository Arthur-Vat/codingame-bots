---
id: E004
date: 2026-10-07
pull_request: pending
parent: uttt-v003
hypothesis: A faster search plays better with the same time, since MCTS gains strength with every doubling of its iterations.
change: Playouts pick each random move straight from the board's masks, with lookup tables for cell counts, instead of listing every legal move; nodes keep their average score and 1/sqrt(visits), so selection needs no division or square root per child. The moves played are unchanged for a given random sequence.
release: uttt-v004
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: pending
---

Where the time went, measured with callgrind on the bundled bot: listing
legal moves in playouts took about half of it, and selection, which
reads all 81 children of the root with a division and a square root
each, most of the rest of the tree's share.

Speed, one thread: random playouts from the start went from about
441,000 to 985,000 per second; MCTS iterations from the start from
about 350,000 to 578,000 per second. The bundled bot's first search
(892 ms) runs about 556,000 iterations instead of 335,000.

A test checks that `random_move` picks exactly the move that choosing
from `legal_moves` would pick with the same random draw, over 300 random
games, so playouts play as before, only faster.

Before the test, locally at 20 ms per move (2-core sandbox, 10 ms of
tolerance, 4-ply openings, 200 pairs): +70.4 Elo [+40.2, +101.8]
against v003.
