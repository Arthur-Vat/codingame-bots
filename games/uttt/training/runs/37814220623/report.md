# Value network fit

- Data: run 37814220623 of commit 2a2d9868bf7f2956d50505987fb97d4a9dbbdd9c, 20 jobs of 100000 games at 10000 iterations, 6 opening plies
- Games: 2000000; positions: 79524896 fitted, 4182825 held out (every 20th game, whole)
- Network: 217-128-32-1, 32065 parameters; weights rounded to 8 bits, 42788 characters of base64
- Training: target Result, 8 epochs, batches of 1024, Adam from 0.001 falling to a twentieth, penalty 0, seed 1, 4 threads, 2403 s in all
- Playouts: `games/uttt/bots/mcts/src/weights.rs`, 16 policy moves each

**Gate of ADR 0019: passes with 50% network, 50% playout.**

## Against the playouts, at the bot's budgets

A search with the network (weights rounded, exploration 0.3) against one with the playouts above (exploration 0.5), 900 ms on each side's first move, then 90 ms per move, 300 pairs of games from 6 random opening moves, 4 games at a time:

| Leaf estimate | Wins, draws, losses | Elo | Pairs by the network's points (0, ½, 1, 1½, 2) |
| --- | --- | --- | --- |
| Network alone | 183, 134, 283 | -58.5 Elo [-84.1, -33.4] | [69, 67, 86, 51, 27] |
| 50% network, 50% playout | 200, 203, 197 | +1.7 Elo [-20.0, +23.5] | [26, 73, 103, 68, 30] |

## Predictions on held-out positions

On 49796 positions of held-out games, against the game's result for the side to move. Measured this way, the network with rounded weights is worth more than 32 playouts. A single playout's error is mostly its own noise, which a search averages away, so only the games above judge the network (ADRs 0018 and 0019).

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.487 | 0.1826 | |
| Average of 1 playouts | 0.3755 | |
| Average of 2 playouts | 0.2672 | |
| Average of 4 playouts | 0.2136 | |
| Average of 8 playouts | 0.1862 | |
| Average of 16 playouts | 0.1725 | |
| Average of 32 playouts | 0.1659 | |
| Network | 0.1515 | 0.6238 |
| Network, weights rounded | 0.1523 | 0.6258 |

## Training

| Epoch | Fitted positions, cross-entropy | Held out, cross-entropy | Held out, squared error |
| --- | --- | --- | --- |
| Start | | 0.6976 | 0.1852 |
| 1 | 0.6349 | 0.6297 | 0.1542 |
| 2 | 0.6291 | 0.6279 | 0.1535 |
| 3 | 0.6276 | 0.6268 | 0.1530 |
| 4 | 0.6264 | 0.6256 | 0.1525 |
| 5 | 0.6254 | 0.6246 | 0.1520 |
| 6 | 0.6244 | 0.6237 | 0.1517 |
| 7 | 0.6237 | 0.6233 | 0.1515 |
| 8 | 0.6233 | 0.6230 | 0.1514 |
| Rounded | | 0.6249 | 0.1522 |
