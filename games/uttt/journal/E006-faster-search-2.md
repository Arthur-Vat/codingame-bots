---
id: E006
date: 2026-10-08
pull_request: "#15"
parent: uttt-v005
hypothesis: A faster search plays better in the same time; E004 measured about 100 Elo per doubling of iterations, and a profile of v005 showed where time still went.
change: Selection reads sqrt(ln(visits)) from a table instead of computing a logarithm; playouts pick and play a small board and a cell directly, inlined, instead of encoding and decoding a move; and the two vectors that take turns holding the tree keep the same capacity, so that neither grows, copying the tree, during a search. The moves played are unchanged for a given random sequence.
release: uttt-v006
sprt: accepted
elo: +39.7 [+18.5, +61.3] at 20 ms
pairs: 360
full_time: +30.7 [+14.5, +46.9] over 500 pairs, no fault, slowest answer 88.7 ms
decision: promoted
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
searches fell to 1.4 ms. Before the test, this looked like the cause of
the slowest answers noted in E004 (90.0 ms against an 82 ms budget); the
test's result below says it was not the main one.

Before the test, locally at 20 ms per move (2-core sandbox, 10 ms of
tolerance, 4-ply openings, 200 pairs), against v005:

- Without the capacity fix: +28.7 Elo [-0.4, +58.3], and one timeout for
  the candidate.
- With it, the frozen release: +82.3 Elo [+53.3, +112.4], no timeout for
  the candidate, one for v005.

The two runs used different seeds, and their intervals barely overlap;
taken together they suggest a gain of about +50.

Result, from the SPRT comment on #15: smoke test 200 wins in 200 games;
SPRT accepted after 360 pairs (284 wins, 236 draws, 201 losses), LLR
2.98; confirmation at CodinGame's limits 333 wins, 422 draws, 245
losses, no fault.

The gain holds at full time (+30.7, interval from +14.5 to +46.9). It is
about what E004's rate of 100 Elo per doubling predicts for 1.29× more
iterations (+37), and smaller than the local runs at 20 ms suggested.

The slowest answers did not improve. At full time v006's slowest was
88.7 ms against 87.4 for v005 in the same run (87.0 and 90.0 in the
confirmations of E005 and E004). At 20 ms it was 25.3 ms against 20.0
for v005, the highest seen in an SPRT so far: the answer arrived within
the 25 ms limit (20 ms and 5 ms of tolerance), since the arena times
the answer after receiving it, but only just. Removing the vector's
growth shortened the searches' own overshoot in the local measurement,
so the remaining delays probably come from elsewhere on a busy machine:
the process waiting to be scheduled, or fresh memory being mapped (v006
reserves the tree's full capacity in both vectors). Not measured. No
game was lost on time in this test, and the owner saw none with v004 on
CodinGame. If a timeout ever appears, the next step is to log each
answer's time, not only the slowest.
