# NNUE fit

- Data: run 38906159700 of commit e0013802c83290bf5ee607ff211951f1497ef0a4, on the self-play data of run 38799715918
- Games: 3232297, of which 15974 dropped as copies of an earlier game (the same moves); positions: 91883414 fitted, 4836337 held out (every 20th game kept, whole)
- Network: (189 -> 256) x 2 -> 10 buckets, 53770 parameters; quantized as in uttt_engine::nnue, 54068 bytes packed in 27264 characters
- Training: target Score, 8 epochs, batches of 1024, Adam from 0.001 falling to a twentieth, seed 1, 4 threads, 708 s in all
- Compared network: `games/uttt/bots/mcts/src/nnue_weights.rs`, width 256
- Throughput: 1080204 positions per second of training (735067312 positions in 680 s, without the measurements)

## Held out, all 4836337 positions

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.499 | 0.1000 | 0.6931 |
| Network, float | 0.0685 | 0.6265 |
| Network, quantized | 0.0684 | 0.6264 |
| Compared network, quantized | 0.0652 | 0.6176 |

## Held out, 49860 positions spread evenly

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.499 | 0.0995 | 0.6931 |
| Network, float | 0.0682 | 0.6270 |
| Network, quantized | 0.0682 | 0.6269 |
| Compared network, quantized | 0.0651 | 0.6186 |

## Held out, 49860 positions spread evenly, in detail

The quantized networks, as the bot plays them.

By ply (moves played before the position): squared error against the result, and the share of the moves played that the network would choose, the child it scores best for the side to move. Openings' moves were drawn, not chosen, when self-play sampled them.

| Plies | Positions | Network: squared error | Network: agreement | Compared: squared error | Compared: agreement |
| --- | --- | --- | --- | --- | --- |
| 0-3 | 2195 | 0.0888 | 13.6% | 0.0859 | 14.1% |
| 4-7 | 2186 | 0.0853 | 21.2% | 0.0814 | 20.8% |
| 8-11 | 2424 | 0.0817 | 43.6% | 0.0772 | 43.7% |
| 12-15 | 2816 | 0.0823 | 44.2% | 0.0793 | 45.3% |
| 16-23 | 7113 | 0.0813 | 47.7% | 0.0774 | 47.9% |
| 24-31 | 9037 | 0.0816 | 48.5% | 0.0768 | 48.1% |
| 32+ | 24089 | 0.0529 | 50.6% | 0.0509 | 49.9% |
| All | | 0.0682 | 46.2% | 0.0651 | 45.9% |

Calibration: the positions by predicted score, with the average prediction and the average result.

| Predicted | Network: positions | predicted | result | Compared: positions | predicted | result |
| --- | --- | --- | --- | --- | --- | --- |
| 0.0-0.1 | 51 | 0.071 | 0.010 | 427 | 0.070 | 0.048 |
| 0.1-0.2 | 524 | 0.162 | 0.047 | 2153 | 0.160 | 0.135 |
| 0.2-0.3 | 2188 | 0.263 | 0.147 | 5415 | 0.256 | 0.258 |
| 0.3-0.4 | 8671 | 0.358 | 0.294 | 8129 | 0.351 | 0.362 |
| 0.4-0.5 | 13669 | 0.450 | 0.431 | 8905 | 0.450 | 0.460 |
| 0.5-0.6 | 12614 | 0.549 | 0.559 | 8736 | 0.549 | 0.536 |
| 0.6-0.7 | 9242 | 0.644 | 0.695 | 8258 | 0.649 | 0.633 |
| 0.7-0.8 | 2337 | 0.737 | 0.851 | 5251 | 0.744 | 0.741 |
| 0.8-0.9 | 491 | 0.836 | 0.956 | 2138 | 0.840 | 0.855 |
| 0.9-1.0 | 73 | 0.929 | 0.973 | 448 | 0.931 | 0.958 |

Average gap between prediction and result, weighted by positions: Network 0.0411; Compared 0.0110.

First-layer units over these positions, from both seats: Network, 0 dead (never above 0) and 0 constant (always at full scale) of 256; Compared, 0 dead (never above 0) and 0 constant (always at full scale) of 256.

## Training

| Epoch | Fitted positions, cross-entropy | Held out, cross-entropy | Held out, squared error |
| --- | --- | --- | --- |
| Start | | 0.6972 | 0.1020 |
| 1 | 0.6618 | 0.6312 | 0.0706 |
| 2 | 0.6594 | 0.6300 | 0.0700 |
| 3 | 0.6590 | 0.6297 | 0.0699 |
| 4 | 0.6586 | 0.6286 | 0.0694 |
| 5 | 0.6581 | 0.6275 | 0.0689 |
| 6 | 0.6576 | 0.6270 | 0.0687 |
| 7 | 0.6572 | 0.6268 | 0.0686 |
| 8 | 0.6570 | 0.6265 | 0.0685 |
| Quantized | | 0.6264 | 0.0684 |

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
