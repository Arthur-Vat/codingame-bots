# NNUE fit

- Data: run 38799715918 of commit e99f88f8e295896d141f63850d8e34971714a43e, 20 jobs of 55000 games at 5000 iterations, 8 opening plies, 2 side games per game
- Games: 3232297, of which 15974 dropped as copies of an earlier game (the same moves); positions: 91883414 fitted, 4836337 held out (every 20th game kept, whole)
- Network: (189 -> 256) x 2 -> 10 buckets, 53770 parameters; quantized as in uttt_engine::nnue, 54068 bytes packed in 27264 characters
- Training: target Result, 8 epochs, batches of 1024, Adam from 0.001 falling to a twentieth, seed 1, 4 threads, 1568 s in all
- Compared network: `games/uttt/bots/mcts/src/nnue_weights.rs`, width 256
- Throughput: 492551 positions per second of training (735067312 positions in 1492 s, without the measurements)

## Held out, all 4836337 positions

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.499 | 0.1000 | 0.6931 |
| Network, float | 0.0651 | 0.6173 |
| Network, quantized | 0.0652 | 0.6176 |
| Compared network, quantized | 0.0697 | 0.6285 |

## Held out, 49860 positions spread evenly

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.499 | 0.0995 | 0.6931 |
| Network, float | 0.0650 | 0.6183 |
| Network, quantized | 0.0651 | 0.6186 |
| Compared network, quantized | 0.0695 | 0.6291 |

## Held out, 49860 positions spread evenly, in detail

The quantized networks, as the bot plays them.

By ply (moves played before the position): squared error against the result, and the share of the moves played that the network would choose, the child it scores best for the side to move. Openings' moves were drawn, not chosen, when self-play sampled them.

| Plies | Positions | Network: squared error | Network: agreement | Compared: squared error | Compared: agreement |
| --- | --- | --- | --- | --- | --- |
| 0-3 | 2195 | 0.0859 | 14.1% | 0.0903 | 11.9% |
| 4-7 | 2186 | 0.0814 | 20.8% | 0.0857 | 21.0% |
| 8-11 | 2424 | 0.0772 | 43.7% | 0.0817 | 43.6% |
| 12-15 | 2816 | 0.0793 | 45.3% | 0.0817 | 43.0% |
| 16-23 | 7113 | 0.0774 | 47.9% | 0.0811 | 46.5% |
| 24-31 | 9037 | 0.0768 | 48.1% | 0.0814 | 45.6% |
| 32+ | 24089 | 0.0509 | 49.9% | 0.0556 | 48.7% |
| All | | 0.0651 | 45.9% | 0.0695 | 44.4% |

Calibration: the positions by predicted score, with the average prediction and the average result.

| Predicted | Network: positions | predicted | result | Compared: positions | predicted | result |
| --- | --- | --- | --- | --- | --- | --- |
| 0.0-0.1 | 427 | 0.070 | 0.048 | 373 | 0.068 | 0.055 |
| 0.1-0.2 | 2153 | 0.160 | 0.135 | 1310 | 0.158 | 0.162 |
| 0.2-0.3 | 5415 | 0.256 | 0.258 | 3717 | 0.259 | 0.257 |
| 0.3-0.4 | 8129 | 0.351 | 0.362 | 9305 | 0.356 | 0.343 |
| 0.4-0.5 | 8905 | 0.450 | 0.460 | 11671 | 0.449 | 0.438 |
| 0.5-0.6 | 8736 | 0.549 | 0.536 | 11280 | 0.552 | 0.572 |
| 0.6-0.7 | 8258 | 0.649 | 0.633 | 8189 | 0.644 | 0.676 |
| 0.7-0.8 | 5251 | 0.744 | 0.741 | 2921 | 0.740 | 0.792 |
| 0.8-0.9 | 2138 | 0.840 | 0.855 | 880 | 0.840 | 0.911 |
| 0.9-1.0 | 448 | 0.931 | 0.958 | 214 | 0.931 | 0.963 |

Average gap between prediction and result, weighted by positions: Network 0.0110; Compared 0.0195.

First-layer units over these positions, from both seats: Network, 0 dead (never above 0) and 0 constant (always at full scale) of 256; Compared, 1 dead (never above 0) and 0 constant (always at full scale) of 256.

## Training

| Epoch | Fitted positions, cross-entropy | Held out, cross-entropy | Held out, squared error |
| --- | --- | --- | --- |
| Start | | 0.6972 | 0.1020 |
| 1 | 0.6333 | 0.6267 | 0.0689 |
| 2 | 0.6247 | 0.6244 | 0.0680 |
| 3 | 0.6234 | 0.6230 | 0.0674 |
| 4 | 0.6220 | 0.6212 | 0.0667 |
| 5 | 0.6204 | 0.6200 | 0.0662 |
| 6 | 0.6189 | 0.6187 | 0.0657 |
| 7 | 0.6178 | 0.6177 | 0.0652 |
| 8 | 0.6172 | 0.6173 | 0.0651 |
| Quantized | | 0.6176 | 0.0652 |

## Data

The fitted positions by ply (moves played before them), with the search's root score for the side to move.

| Plies | Positions | Root score's average distance from 0.5 | Root score beyond 0.3 or 0.7 | Beyond 0.1 or 0.9 |
| --- | --- | --- | --- | --- |
| 0-3 | 4132352 | 0.107 | 0.0% | 0.0% |
| 4-7 | 4132352 | 0.091 | 1.3% | 0.0% |
| 8-11 | 4447070 | 0.102 | 5.0% | 0.0% |
| 12-15 | 5286523 | 0.104 | 6.5% | 0.0% |
| 16-23 | 13092164 | 0.110 | 10.1% | 0.0% |
| 24-31 | 16431641 | 0.118 | 16.6% | 0.4% |
| 32+ | 44361312 | 0.112 | 24.4% | 8.3% |

The first player won 39.1% of the 3055469 games, drew 47.4% and lost 13.5%.

Distinct positions: at ply 4, 53845 of 1033088; at ply 8, 1023625 of 1033088; at ply 12, 1232595 of 1242835.
