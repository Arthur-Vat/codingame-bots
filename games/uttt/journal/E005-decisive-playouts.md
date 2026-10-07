---
id: E005
date: 2026-10-07
pull_request: "#14"
parent: uttt-v004
hypothesis: Playouts that take a game-winning move when there is one ("decisive moves") judge positions better than uniformly random ones, for almost no speed, since random playouts often miss a win in one and so misjudge positions with threats.
change: A table of the cells that complete a line finds, in a few lookups, the small boards where the player to move would win the game with a line of small boards; playouts play such a move when there is one, and the same random move as before otherwise. Wins on points, when the last open board closes, are not looked for.
release: uttt-v005
sprt: accepted
elo: +109.5 [+70.9, +151.0] at 20 ms
pairs: 154
full_time: +78.4 [+59.4, +98.0] over 500 pairs, no fault, slowest answer 87.0 ms
decision: promoted
cg_rank: pending
---

The idea comes from Teytaud and Teytaud, "On the huge benefit of
decisive moves in Monte-Carlo tree search algorithms" (2010): a decisive
move wins at once; an anti-decisive move blocks an opponent's decisive
move.

Three playout policies were screened locally before choosing the one to
test, each against v004 at 20 ms per move (2-core sandbox, 10 ms of
tolerance, 4-ply openings, 200 pairs):

| Policy | Elo against v004 |
| --- | --- |
| Decisive: win the game when possible, else random (this release) | +93.4 [+58.5, +130.2] |
| Decisive and anti-decisive: also avoid sending the opponent to a small board where it wins the game, or to a closed board that frees it, when another move exists | +72.2 [+42.0, +103.6] |
| Both, and also win a small board when possible without such a risk | +18.3 [-11.7, +48.5] |

Decisive alone then beat decisive and anti-decisive directly by +43.7
Elo [+19.6, +68.2] over 300 pairs, so the anti-decisive rule is not just
unhelpful but harmful here. It cost about 13% of the search's speed, and
it makes playouts avoid the moves that create threats, which may bias
their results. Greedily winning small boards hurt more still: playouts
that grab every small board misjudge which ones matter.

Speed: decisive playouts from the start are about 5% shorter (54.2 moves
instead of 58.9), which pays for the check, so the search runs about as
many iterations as before, or slightly more.

Tests check that a decisive move is a game-winning move whenever one
exists, against trying every legal move, and that it is otherwise the
same random move, drawn from the same random numbers, as a plain random
playout would play.

Before the test, the frozen release against v004, locally at 20 ms per
move (same settings, 100 pairs): +113.3 Elo [+68.4, +162.2].

Result, from the SPRT comment on #14: smoke test 200 wins in 200 games;
SPRT accepted after 154 pairs (176 wins, 50 draws, 82 losses), LLR
2.98; confirmation at CodinGame's limits 529 wins, 164 draws, 307
losses, no fault.

The gain holds at full time (+78.4, interval from +59.4 to +98.0). Draws
fell from 273 in 1,000 games in E004's confirmation to 164 here: the
stronger side now converts more of its threats. The slowest answer,
87.0 ms, is in line with v004's 86.8 ms in the same run.
