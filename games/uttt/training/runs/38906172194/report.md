# NNUE fit

- Data: run 38906172194 of commit e0013802c83290bf5ee607ff211951f1497ef0a4, on the self-play data of run 38799715918
- Games: 3232297, of which 15974 dropped as copies of an earlier game (the same moves); positions: 91883414 fitted, 4836337 held out (every 20th game kept, whole)
- Network: (189 -> 256) x 2 -> 10 buckets, 53770 parameters; quantized as in uttt_engine::nnue, 54068 bytes packed in 27264 characters
- Training: target Mix, 8 epochs, batches of 1024, Adam from 0.001 falling to a twentieth, seed 1, 4 threads, 1840 s in all
- Compared network: `games/uttt/bots/mcts/src/nnue_weights.rs`, width 256
- Throughput: 416763 positions per second of training (735067312 positions in 1764 s, without the measurements)

## Held out, all 4836337 positions

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.499 | 0.1000 | 0.6931 |
| Network, float | 0.0656 | 0.6193 |
| Network, quantized | 0.0657 | 0.6195 |
| Compared network, quantized | 0.0652 | 0.6176 |

## Held out, 49860 positions spread evenly

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.499 | 0.0995 | 0.6931 |
| Network, float | 0.0655 | 0.6201 |
| Network, quantized | 0.0656 | 0.6204 |
| Compared network, quantized | 0.0651 | 0.6186 |

## Held out, 49860 positions spread evenly, in detail

The quantized networks, as the bot plays them.

By ply (moves played before the position): squared error against the result, and the share of the moves played that the network would choose, the child it scores best for the side to move. Openings' moves were drawn, not chosen, when self-play sampled them.

| Plies | Positions | Network: squared error | Network: agreement | Compared: squared error | Compared: agreement |
| --- | --- | --- | --- | --- | --- |
| 0-3 | 2195 | 0.0865 | 15.3% | 0.0859 | 14.1% |
| 4-7 | 2186 | 0.0824 | 21.4% | 0.0814 | 20.8% |
| 8-11 | 2424 | 0.0785 | 45.7% | 0.0772 | 43.7% |
| 12-15 | 2816 | 0.0798 | 45.5% | 0.0793 | 45.3% |
| 16-23 | 7113 | 0.0785 | 48.1% | 0.0774 | 47.9% |
| 24-31 | 9037 | 0.0778 | 48.1% | 0.0768 | 48.1% |
| 32+ | 24089 | 0.0509 | 50.1% | 0.0509 | 49.9% |
| All | | 0.0656 | 46.2% | 0.0651 | 45.9% |

Calibration: the positions by predicted score, with the average prediction and the average result.

| Predicted | Network: positions | predicted | result | Compared: positions | predicted | result |
| --- | --- | --- | --- | --- | --- | --- |
| 0.0-0.1 | 166 | 0.072 | 0.024 | 427 | 0.070 | 0.048 |
| 0.1-0.2 | 1052 | 0.159 | 0.080 | 2153 | 0.160 | 0.135 |
| 0.2-0.3 | 3976 | 0.260 | 0.208 | 5415 | 0.256 | 0.258 |
| 0.3-0.4 | 8791 | 0.353 | 0.329 | 8129 | 0.351 | 0.362 |
| 0.4-0.5 | 10720 | 0.450 | 0.445 | 8905 | 0.450 | 0.460 |
| 0.5-0.6 | 10525 | 0.549 | 0.543 | 8736 | 0.549 | 0.536 |
| 0.6-0.7 | 9255 | 0.648 | 0.658 | 8258 | 0.649 | 0.633 |
| 0.7-0.8 | 4089 | 0.740 | 0.790 | 5251 | 0.744 | 0.741 |
| 0.8-0.9 | 1097 | 0.838 | 0.918 | 2138 | 0.840 | 0.855 |
| 0.9-1.0 | 189 | 0.929 | 0.960 | 448 | 0.931 | 0.958 |

Average gap between prediction and result, weighted by positions: Network 0.0205; Compared 0.0110.

First-layer units over these positions, from both seats: Network, 0 dead (never above 0) and 0 constant (always at full scale) of 256; Compared, 0 dead (never above 0) and 0 constant (always at full scale) of 256.

## Training

| Epoch | Fitted positions, cross-entropy | Held out, cross-entropy | Held out, squared error |
| --- | --- | --- | --- |
| Start | | 0.6972 | 0.1020 |
| 1 | 0.6481 | 0.6253 | 0.0682 |
| 2 | 0.6434 | 0.6242 | 0.0677 |
| 3 | 0.6427 | 0.6235 | 0.0674 |
| 4 | 0.6419 | 0.6222 | 0.0669 |
| 5 | 0.6411 | 0.6211 | 0.0664 |
| 6 | 0.6403 | 0.6202 | 0.0660 |
| 7 | 0.6397 | 0.6195 | 0.0657 |
| 8 | 0.6393 | 0.6193 | 0.0656 |
| Quantized | | 0.6195 | 0.0657 |

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
