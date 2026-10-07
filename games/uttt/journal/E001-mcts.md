---
id: E001
date: 2026-10-07
pull_request: "#8"
parent: greedy (no release yet)
hypothesis: Monte Carlo tree search with random playouts plays far better than the one-move-lookahead greedy bot.
change: First release, the mcts bot - UCT with exploration constant 0.5, random playouts, a fresh tree every turn, 88 ms per move on CodinGame.
release: uttt-v001
sprt: accepted
elo: no estimate from a sweep; 60 wins in 60 games put v001 at least about 500 Elo above greedy (95% lower bound)
pairs: 30, the minimum
decision: promoted
cg_rank: pending
---

The bot and the choice of the exploration constant are described in
`games/uttt/bots/mcts` and in pull request #7: 0.5 beat 1.0 by about
160 Elo at 20 ms and at 100 ms per move.

Expected: accepted against `greedy` after the minimum 30 pairs. The
rehearsal in a 2-core sandbox won 60 games out of 60.

Result, from the SPRT comment on #8 (20 ms per move plus 5 ms of
tolerance, 4-ply openings, seed 1):

- Smoke test against random: 200 wins in 200 games, no fault; slowest
  answer 20.2 ms, within the tolerance.
- SPRT against greedy: 60 wins in 60 games, no fault; LLR 3403.5 against
  a bound of 2.94 after 30 pairs: accepted.

A sweep gives no Elo estimate; the League rates v001 against random and
greedy once merged. Next: paste v001 into CodinGame and record its rank.
