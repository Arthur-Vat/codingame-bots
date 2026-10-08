---
id: E011
date: 2026-10-08
pull_request: pending
parent: uttt-v007
hypothesis: Playouts whose first moves follow a policy learned from what longer searches prefer judge positions better than random ones, enough to pay for their cost; E005 showed that knowledge in playouts pays, and that rules picked by hand do not.
change: After the game-winning check, the first 16 moves of each playout are drawn with weights by move class (wins its small board, blocks, gives a free choice, sends the opponent to a board it can win, takes the centre), fitted by `uttt-trainer` to 86,657 positions of 2,000 self-play games at 10,000 iterations per move and sharpened (temperature 0.5); later moves are decisive random moves, as before.
release: uttt-v008
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: pending
---

First stage of ADR 0016. The weights were fitted locally, with the
commands the Train workflow runs (seeds 101 and 202, 1,000 games each);
on positions held out of the fit, the cross-entropy against the search's
visit shares fell from 1.866 nats for uniform moves to 1.775, and the
probability given to the search's preferred move rose from 0.172 to
0.212. Relative to a move with no feature, the fit (at temperature 1)
favours winning a small board ×3.5 and blocking ×3, and avoids giving a
free choice ×0.27 and sending the opponent where it can win a board
×0.63.

Quality against cost, locally against v007:

| Variant | Setting | Pairs | Elo against v007 |
| --- | --- | --- | --- |
| Policy for the whole playout, first version | 5,000 iterations per move for both | 200 | +87.8 [+57.0, +120.0] |
| Same, faster version | 20,000 iterations per move for both | 150 | +75.3 [+41.2, +110.8] |
| Same, first version | 20 ms per move | 200 | -149.3 [-182.0, -118.8] |
| Same, faster version | 20 ms per move | 200 | -22.6 [-50.8, +5.3] |

At equal iterations the policy is clearly better; at equal time its cost
ate the gain. The first version cost 3.7 times more per iteration in the
bot's games; keeping each seat's small-board threats up to date in the
board, and drawing moves branch-free in one pass, brought policy
playouts from the start from about 345,000 to 440,000 per second,
against about 1,400,000 for decisive ones.

Then the policy's reach and sharpness, at 20 ms per move, 200 pairs each
against v007 (same seed), with the faster version:

| Policy moves per playout | Temperature | Elo against v007 |
| --- | --- | --- |
| All | 1 | +0.9 [-27.4, +29.2] |
| 16 | 1 | +18.3 [-10.8, +47.6] |
| 8 | 1 | +29.6 [+1.8, +57.8] |
| 4 | 1 | +21.7 [-6.9, +50.7] |
| 8 | 2 (flatter) | -21.7 [-51.7, +7.9] |
| 8 | 0.5 (sharper) | +48.1 [+19.6, +77.2] |
| 8 | 0.33 | +49.8 [+23.2, +77.1] |
| 16 | 0.5 (this release) | +53.4 [+25.0, +82.5] |

Using the policy for the first moves only keeps most of its knowledge
for a fraction of its cost, and sharper weights help. With eight
variants screened, the best result overstates the gain; the SPRT
measures it afresh.

The bundle grows to 79.6 kB of the 100 kB allowed: the engine now holds
the policy, and comments make up much of the file.
