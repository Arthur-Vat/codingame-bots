---
id: E006
date: 2026-10-08
pull_request: pending
parent: uttt-v005
hypothesis: A faster search plays better in the same time; E004 measured about 100 Elo per doubling of iterations, and a profile of v005 showed where time still went.
change: Selection reads sqrt(ln(visits)) from a table instead of computing a logarithm; playouts pick and play a small board and a cell directly, inlined, instead of encoding and decoding a move; and the two vectors that take turns holding the tree keep the same capacity, so that neither grows, copying the tree, during a search. The moves played are unchanged for a given random sequence.
release: uttt-v006
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: pending
---

Where the time went, measured with callgrind on self-play games of
20,000 iterations per move (instructions, not time): playouts about
half, selection about a third, the logarithm of the parent's visits in
selection alone about 7%, and calls to `play` and `random_move` that the
compiler did not inline.

Each change was timed on the same self-play games (two games, 20,000
iterations per move, best of five runs), which also check that the moves
and expected scores stay the same:

| Change | Time | Speed-up |
| --- | --- | --- |
| v005 | 1.60 s | |
| Table for sqrt(ln(visits)) | 1.38 s | 1.16× |
| `play` and `random_move` always inlined | 1.34 s | 1.03× |
| Playouts play cells without encoding moves | 1.18 s | 1.13× |
| Total | | 1.35× |

Two other changes made no measurable difference and were dropped:
testing the rare cases of selection (a proven or untried child) in one
branch, and giving untried and proven children infinite bounds so that
selection reads one bound per child without branches.

In the bundled bots at CodinGame's limits (self-play, 4 pairs each),
v005 ran 77,447 iterations per searched turn on average, the candidate
100,261: 1.29× more.

The tree's capacity: when a subtree is kept, the tree moves to a second
vector, and the smaller of the two used to grow during the next search
by copying the whole tree. Measured on self-play with bot-like budgets
(892 ms then 82 ms per move), searches overshot their deadline by up to
6 ms, each time at a doubling of the vector (524,288 nodes); the first
search builds about 1.8 million nodes. With the spare vector given the
tree's capacity in a fresh allocation, the worst overshoot over 350
searches fell to 1.4 ms. This is likely the cause of the slowest answers
noted in E004 (90.0 ms against an 82 ms budget).

Before the test, locally at 20 ms per move (2-core sandbox, 10 ms of
tolerance, 4-ply openings, 200 pairs), against v005:

- Without the capacity fix: +28.7 Elo [-0.4, +58.3], and one timeout for
  the candidate.
- With it, the frozen release: +82.3 Elo [+53.3, +112.4], no timeout for
  the candidate, one for v005.

The two runs used different seeds, and their intervals barely overlap;
taken together they suggest a gain of about +50.
