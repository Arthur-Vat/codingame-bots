---
id: E012
date: 2026-10-08
pull_request: "#32"
parent: uttt-v008
hypothesis: A value network trained on 800,000 self-play games, which beat v008's playouts by 134 Elo at 3,000 iterations per move (the gate of ADR 0018), makes the bot stronger at CodinGame's limits.
change: None released. The bot searched with the network of Train run 37795543128 at new leaves instead of playouts (`ValueBoard`), and, as ADR 0017's fallback, with an average of the network's estimate and a playout.
release: none
sprt: not run, local screening only
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Second stage of ADR 0016, first network. Train run 37795543128
(report in `games/uttt/training/runs/37795543128/`) played 800,000 self-play games of
v008's search at 10,000 iterations per move on 20 jobs. That gave 31.8
million positions to fit and 1.7 million held out, from every 20th game.
The 217-64-16-1 network trained for 8 epochs on the game results.

- **Held-out error:** the squared error against results fell from 0.1564
  after one epoch to 0.1533 after eight, and 0.1538 with weights rounded
  to 8 bits. An average of 32 playouts scores 0.1662 and the average
  result 0.1828.
- **No overfitting:** the fitted positions' cross-entropy, 0.6287, matched
  the held-out one, 0.6281.
- **The gate of ADR 0018:** at 3,000 iterations per move, the network's
  search (exploration 0.1) beat v008's by +133.9 Elo [+109.5, +159.7]
  over 400 pairs.

Screening against v008 in the arena, as the bot, locally (2-core
sandbox, 4-ply openings):

| Leaf estimate | Exploration | Time per move | Pairs | Elo against v008 |
| --- | --- | --- | --- | --- |
| Network | 0.1 | 20 ms | 100 | -20.9 [-63.0, +20.7] |
| Network | 0.2 | 20 ms | 100 | -24.4 [-66.3, +16.8] |
| Network | 0.3 | 20 ms | 100 | -20.9 [-66.1, +23.7] |
| Half network, half playout | 0.2 | 20 ms | 100 | +20.9 [-16.2, +58.5] |
| Half network, half playout | 0.3 | 20 ms | 100 | +27.9 [-14.1, +70.7] |
| Half network, half playout | 0.5 | 20 ms | 100 | +15.6 [-23.8, +55.5] |
| 70% network, 30% playout | 0.3 | 20 ms | 100 | +8.7 [-31.8, +49.4] |
| Network | 0.1 | 100 ms | 50 | -96.2 [-169.4, -30.6] |
| Half network, half playout | 0.3 | 100 ms | 50 | -20.9 [-76.2, +33.4] |

The network's lead shrinks as the search grows. It is +134 at 3,000
iterations, about -20 at 20 ms per move, and -96 at CodinGame's limits.
A likely reason: more iterations average the playouts' noise away,
while the network's own errors stay. Not measured.

The network does not buy more search either. Over a pair of self-play
games at full time, the bot with the network ran 105,000 iterations per
turn on average (106,000 to 124,000 on its first moves). The bot with
the mix ran 43,500, and v008 105,600 (68,000 on its first moves). Late in
the game, playouts are short and cost less than an evaluation.

No release and no SPRT. Two lessons:

- ADR 0018's games at 3,000 iterations judged the network far from the
  bot's budgets, so they passed a network that is weaker at those
  budgets.
- The network fits its training positions no better than held-out
  ones, which suggests it is too small to use more data.

The weights stay on the run's branch.
