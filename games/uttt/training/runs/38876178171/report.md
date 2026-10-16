# NNUE fit

- Data: run 38876178171 of commit 85c10c4d9115c5169cb2bad2951157e3ac7131a4, on the self-play data of run 38799715918
- Games: 3232297, of which 15974 dropped as copies of an earlier game (the same moves); positions: 91883414 fitted, 4836337 held out (every 20th game kept, whole)
- Network: (189 -> 128) x 2 -> 10 buckets, 26890 parameters; quantized as in uttt_engine::nnue, 27060 bytes packed in 13696 characters
- Training: target Result, 8 epochs, batches of 1024, Adam from 0.001 falling to a twentieth, seed 1, 4 threads, 840 s in all
- Compared network: `games/uttt/bots/mcts/src/nnue_weights.rs`, width 256
- Throughput: 909209 positions per second of training (735067312 positions in 808 s, without the measurements)

## Held out, all 4836337 positions

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.499 | 0.1000 | 0.6931 |
| Network, float | 0.0695 | 0.6280 |
| Network, quantized | 0.0695 | 0.6282 |
| Compared network, quantized | 0.0652 | 0.6176 |

## Held out, 49860 positions spread evenly

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.499 | 0.0995 | 0.6931 |
| Network, float | 0.0692 | 0.6285 |
| Network, quantized | 0.0693 | 0.6288 |
| Compared network, quantized | 0.0651 | 0.6186 |

## Held out, 49860 positions spread evenly, in detail

The quantized networks, as the bot plays them.

By ply (moves played before the position): squared error against the result, and the share of the moves played that the network would choose, the child it scores best for the side to move. Openings' moves were drawn, not chosen, when self-play sampled them.

| Plies | Positions | Network: squared error | Network: agreement | Compared: squared error | Compared: agreement |
| --- | --- | --- | --- | --- | --- |
| 0-3 | 2195 | 0.0857 | 13.1% | 0.0859 | 14.1% |
| 4-7 | 2186 | 0.0819 | 18.4% | 0.0814 | 20.8% |
| 8-11 | 2424 | 0.0781 | 34.6% | 0.0772 | 43.7% |
| 12-15 | 2816 | 0.0808 | 33.7% | 0.0793 | 45.3% |
| 16-23 | 7113 | 0.0813 | 39.2% | 0.0774 | 47.9% |
| 24-31 | 9037 | 0.0815 | 41.0% | 0.0768 | 48.1% |
| 32+ | 24089 | 0.0564 | 44.9% | 0.0509 | 49.9% |
| All | | 0.0693 | 39.7% | 0.0651 | 45.9% |

Calibration: the positions by predicted score, with the average prediction and the average result.

| Predicted | Network: positions | predicted | result | Compared: positions | predicted | result |
| --- | --- | --- | --- | --- | --- | --- |
| 0.0-0.1 | 185 | 0.075 | 0.046 | 427 | 0.070 | 0.048 |
| 0.1-0.2 | 1422 | 0.163 | 0.145 | 2153 | 0.160 | 0.135 |
| 0.2-0.3 | 5351 | 0.256 | 0.256 | 5415 | 0.256 | 0.258 |
| 0.3-0.4 | 8889 | 0.351 | 0.354 | 8129 | 0.351 | 0.362 |
| 0.4-0.5 | 9322 | 0.450 | 0.458 | 8905 | 0.450 | 0.460 |
| 0.5-0.6 | 8720 | 0.549 | 0.537 | 8736 | 0.549 | 0.536 |
| 0.6-0.7 | 8980 | 0.648 | 0.638 | 8258 | 0.649 | 0.633 |
| 0.7-0.8 | 5215 | 0.743 | 0.738 | 5251 | 0.744 | 0.741 |
| 0.8-0.9 | 1561 | 0.837 | 0.858 | 2138 | 0.840 | 0.855 |
| 0.9-1.0 | 215 | 0.927 | 0.953 | 448 | 0.931 | 0.958 |

Average gap between prediction and result, weighted by positions: Network 0.0079; Compared 0.0110.

First-layer units over these positions, from both seats: Network, 0 dead (never above 0) and 0 constant (always at full scale) of 128; Compared, 0 dead (never above 0) and 0 constant (always at full scale) of 256.

## Training

| Epoch | Fitted positions, cross-entropy | Held out, cross-entropy | Held out, squared error |
| --- | --- | --- | --- |
| Start | | 0.7003 | 0.1035 |
| 1 | 0.6378 | 0.6358 | 0.0728 |
| 2 | 0.6341 | 0.6337 | 0.0719 |
| 3 | 0.6327 | 0.6324 | 0.0713 |
| 4 | 0.6315 | 0.6311 | 0.0708 |
| 5 | 0.6303 | 0.6302 | 0.0704 |
| 6 | 0.6292 | 0.6292 | 0.0700 |
| 7 | 0.6282 | 0.6283 | 0.0696 |
| 8 | 0.6277 | 0.6280 | 0.0695 |
| Quantized | | 0.6282 | 0.0695 |

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
