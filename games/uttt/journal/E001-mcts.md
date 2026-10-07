---
id: E001
date: 2026-10-07
pull_request: pending
parent: greedy (no release yet)
hypothesis: Monte Carlo tree search with random playouts plays far better than the one-move-lookahead greedy bot.
change: First release, the mcts bot - UCT with exploration constant 0.5, random playouts, a fresh tree every turn, 88 ms per move on CodinGame.
release: uttt-v001
sprt: pending
elo: pending
pairs: pending
decision: pending
cg_rank: pending
---

The bot and the choice of the exploration constant are described in
`games/uttt/bots/mcts` and in pull request #7: 0.5 beat 1.0 by about
160 Elo at 20 ms and at 100 ms per move.

Expected: accepted against `greedy` after the minimum 30 pairs. The
rehearsal in a 2-core sandbox won 60 games out of 60.
