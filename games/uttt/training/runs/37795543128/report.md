# Value network fit

- Data: run 37795543128 of commit e7714420b1736a0a11963ef3233ea3e74fcc9c3b, 20 jobs of 40000 games at 10000 iterations, 6 opening plies
- Games: 800000; positions: 31806900 fitted, 1672980 held out (every 20th game, whole)
- Network: 217-64-16-1, 15009 parameters; weights rounded to 8 bits, 20044 characters of base64
- Training: target Result, 8 epochs, batches of 1024, Adam from 0.001 falling to a twentieth, penalty 0, seed 1, 4 threads, 231 s in all
- Playouts: `games/uttt/bots/mcts/src/weights.rs`, 16 policy moves each

**Gate of ADR 0018: passes.** At 3000 iterations per move, the network's search scored +133.9 Elo [+109.5, +159.7] against the playouts' over 400 pairs of games: not clearly weaker.

## Against the playouts, at equal iterations

A search with the network (weights rounded, exploration 0.1) against one with the playouts above (exploration 0.5), 3000 iterations per move each, 400 pairs of games from 6 random opening moves: 508 wins, 78 draws, 214 losses for the network, +133.9 Elo [+109.5, +159.7]. Pairs by the network's points (0, ½, 1, 1½, 2): [36, 14, 132, 56, 162].

## Predictions on held-out positions

On 49206 positions of held-out games, against the game's result for the side to move. Measured this way, the network with rounded weights is worth more than 32 playouts. A single playout's error is mostly its own noise, which a search averages away, so only the games above judge the network (ADR 0018).

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.487 | 0.1828 | |
| Average of 1 playouts | 0.3726 | |
| Average of 2 playouts | 0.2653 | |
| Average of 4 playouts | 0.2131 | |
| Average of 8 playouts | 0.1857 | |
| Average of 16 playouts | 0.1731 | |
| Average of 32 playouts | 0.1662 | |
| Network | 0.1528 | 0.6269 |
| Network, weights rounded | 0.1534 | 0.6282 |

## Training

| Epoch | Fitted positions, cross-entropy | Held out, cross-entropy | Held out, squared error |
| --- | --- | --- | --- |
| Start | | 0.7121 | 0.1920 |
| 1 | 0.6419 | 0.6354 | 0.1564 |
| 2 | 0.6351 | 0.6334 | 0.1556 |
| 3 | 0.6333 | 0.6321 | 0.1550 |
| 4 | 0.6320 | 0.6307 | 0.1545 |
| 5 | 0.6308 | 0.6298 | 0.1541 |
| 6 | 0.6299 | 0.6289 | 0.1537 |
| 7 | 0.6291 | 0.6284 | 0.1535 |
| 8 | 0.6287 | 0.6281 | 0.1533 |
| Rounded | | 0.6291 | 0.1538 |
