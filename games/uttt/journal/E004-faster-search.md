---
id: E004
date: 2026-10-07
pull_request: "#13"
parent: uttt-v003
hypothesis: A faster search plays better with the same time, since MCTS gains strength with every doubling of its iterations.
change: Playouts pick each random move straight from the board's masks, with lookup tables for cell counts, instead of listing every legal move; nodes keep their average score and 1/sqrt(visits), so selection needs no division or square root per child. The moves played are unchanged for a given random sequence.
release: uttt-v004
sprt: accepted
elo: +87.5 [+53.8, +123.0] at 20 ms
pairs: 154
full_time: +74.4 [+56.7, +92.5] over 500 pairs, no fault, slowest answer 90.0 ms
decision: promoted
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

Result, from the SPRT comment on #13: smoke test 200 wins in 200 games;
SPRT accepted after 154 pairs (168 wins, 48 draws, 92 losses), LLR
2.96; confirmation at CodinGame's limits 469 wins, 273 draws, 258
losses, no fault.

Unlike E003, the gain holds at full time (+74.4, interval from +56.7 to
+92.5): more iterations still pay off with five times the time.

To watch: v004's slowest answer at full time was 90.0 ms, against 86.7
for v003, with the same 82 ms budget. The bot checks its deadline after
every iteration, from the moment it reads its first line, so the extra
time is either one slow iteration or time the bot's clock does not see.
A suspect, not yet measured: the tree's node array grows by doubling,
and the iteration that crosses a size copies the whole array, which is
larger now that the search is faster. There is no fault in 1,000 games,
but the margin to CodinGame's 100 ms is 10 ms.
