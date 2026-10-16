# NNUE fit

- Data: run 38883498642 of commit 3b5778889156dbe5104362866ce44e19eabf4c7a, on the self-play data of run 38799715918
- Games: 3232297, of which 15974 dropped as copies of an earlier game (the same moves); positions: 91883414 fitted, 4836337 held out (every 20th game kept, whole)
- Network: (189 -> 256) x 2 -> 30 buckets, 64030 parameters; quantized as in uttt_engine::nnue, 64388 bytes packed in 32512 characters
- Training: target Result, 8 epochs, batches of 1024, Adam from 0.001 falling to a twentieth, seed 1, 4 threads, 1919 s in all
- Compared network: `games/uttt/bots/mcts/src/nnue_weights.rs`, width 256
- Throughput: 398955 positions per second of training (735067312 positions in 1842 s, without the measurements)

## Held out, all 4836337 positions

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.499 | 0.1000 | 0.6931 |
| Network, float | 0.0653 | 0.6177 |
| Network, quantized | 0.0653 | 0.6180 |
| Compared network, quantized | 0.0652 | 0.6176 |

## Held out, 49860 positions spread evenly

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.499 | 0.0995 | 0.6931 |
| Network, float | 0.0650 | 0.6183 |
| Network, quantized | 0.0651 | 0.6185 |
| Compared network, quantized | 0.0651 | 0.6186 |

## Held out, 49860 positions spread evenly, in detail

The quantized networks, as the bot plays them.

By ply (moves played before the position): squared error against the result, and the share of the moves played that the network would choose, the child it scores best for the side to move. Openings' moves were drawn, not chosen, when self-play sampled them.

| Plies | Positions | Network: squared error | Network: agreement | Compared: squared error | Compared: agreement |
| --- | --- | --- | --- | --- | --- |
| 0-3 | 2195 | 0.0858 | 14.3% | 0.0859 | 14.1% |
| 4-7 | 2186 | 0.0813 | 20.3% | 0.0814 | 20.8% |
| 8-11 | 2424 | 0.0769 | 43.5% | 0.0772 | 43.7% |
| 12-15 | 2816 | 0.0794 | 44.9% | 0.0793 | 45.3% |
| 16-23 | 7113 | 0.0774 | 48.7% | 0.0774 | 47.9% |
| 24-31 | 9037 | 0.0763 | 47.8% | 0.0768 | 48.1% |
| 32+ | 24089 | 0.0510 | 49.9% | 0.0509 | 49.9% |
| All | | 0.0651 | 45.9% | 0.0651 | 45.9% |

Calibration: the positions by predicted score, with the average prediction and the average result.

| Predicted | Network: positions | predicted | result | Compared: positions | predicted | result |
| --- | --- | --- | --- | --- | --- | --- |
| 0.0-0.1 | 419 | 0.069 | 0.050 | 427 | 0.070 | 0.048 |
| 0.1-0.2 | 2091 | 0.159 | 0.133 | 2153 | 0.160 | 0.135 |
| 0.2-0.3 | 5421 | 0.256 | 0.257 | 5415 | 0.256 | 0.258 |
| 0.3-0.4 | 8229 | 0.352 | 0.362 | 8129 | 0.351 | 0.362 |
| 0.4-0.5 | 9058 | 0.450 | 0.462 | 8905 | 0.450 | 0.460 |
| 0.5-0.6 | 8726 | 0.549 | 0.537 | 8736 | 0.549 | 0.536 |
| 0.6-0.7 | 8155 | 0.648 | 0.634 | 8258 | 0.649 | 0.633 |
| 0.7-0.8 | 5212 | 0.744 | 0.744 | 5251 | 0.744 | 0.741 |
| 0.8-0.9 | 2104 | 0.841 | 0.851 | 2138 | 0.840 | 0.855 |
| 0.9-1.0 | 445 | 0.932 | 0.960 | 448 | 0.931 | 0.958 |

Average gap between prediction and result, weighted by positions: Network 0.0105; Compared 0.0110.

First-layer units over these positions, from both seats: Network, 2 dead (never above 0) and 0 constant (always at full scale) of 256; Compared, 0 dead (never above 0) and 0 constant (always at full scale) of 256.

## Training

| Epoch | Fitted positions, cross-entropy | Held out, cross-entropy | Held out, squared error |
| --- | --- | --- | --- |
| Start | | 0.6987 | 0.1027 |
| 1 | 0.6348 | 0.6288 | 0.0698 |
| 2 | 0.6260 | 0.6255 | 0.0685 |
| 3 | 0.6242 | 0.6236 | 0.0676 |
| 4 | 0.6226 | 0.6219 | 0.0670 |
| 5 | 0.6210 | 0.6206 | 0.0664 |
| 6 | 0.6194 | 0.6191 | 0.0658 |
| 7 | 0.6183 | 0.6181 | 0.0654 |
| 8 | 0.6176 | 0.6177 | 0.0653 |
| Quantized | | 0.6180 | 0.0653 |

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
