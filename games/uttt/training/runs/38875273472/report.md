# NNUE fit

- Data: run 38875273472 of commit 85c10c4d9115c5169cb2bad2951157e3ac7131a4, 16 jobs of at most 200000 games in 240 minutes at 5000 iterations, 8 opening plies, 2 side games per game
- Games: 3103648, of which 29823 dropped as copies of an earlier game (the same moves); positions: 83789419 fitted, 4405211 held out (every 20th game kept, whole)
- Network: (189 -> 256) x 2 -> 10 buckets, 53770 parameters; quantized as in uttt_engine::nnue, 54068 bytes packed in 27264 characters
- Training: target Result, 8 epochs, batches of 1024, Adam from 0.001 falling to a twentieth, seed 1, 4 threads, 599 s in all
- Compared network: `games/uttt/bots/mcts/src/nnue_weights.rs`, width 256
- Throughput: 1162038 positions per second of training (670315352 positions in 577 s, without the measurements)

## Held out, all 4405211 positions

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.499 | 0.1242 | 0.6931 |
| Network, float | 0.0705 | 0.5749 |
| Network, quantized | 0.0706 | 0.5752 |
| Compared network, quantized | 0.0728 | 0.5814 |

## Held out, 49497 positions spread evenly

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.499 | 0.1240 | 0.6931 |
| Network, float | 0.0707 | 0.5762 |
| Network, quantized | 0.0708 | 0.5764 |
| Compared network, quantized | 0.0730 | 0.5826 |

## Held out, 49497 positions spread evenly, in detail

The quantized networks, as the bot plays them.

By ply (moves played before the position): squared error against the result, and the share of the moves played that the network would choose, the child it scores best for the side to move. Openings' moves were drawn, not chosen, when self-play sampled them.

| Plies | Positions | Network: squared error | Network: agreement | Compared: squared error | Compared: agreement |
| --- | --- | --- | --- | --- | --- |
| 0-3 | 2441 | 0.0893 | 19.1% | 0.0921 | 18.3% |
| 4-7 | 2261 | 0.0825 | 24.0% | 0.0867 | 25.3% |
| 8-11 | 2531 | 0.0773 | 44.7% | 0.0807 | 47.5% |
| 12-15 | 3010 | 0.0832 | 47.2% | 0.0872 | 49.6% |
| 16-23 | 7514 | 0.0790 | 47.6% | 0.0826 | 48.9% |
| 24-31 | 9474 | 0.0806 | 49.9% | 0.0839 | 50.4% |
| 32+ | 22266 | 0.0583 | 49.4% | 0.0588 | 50.4% |
| All | | 0.0708 | 46.2% | 0.0730 | 47.3% |

Calibration: the positions by predicted score, with the average prediction and the average result.

| Predicted | Network: positions | predicted | result | Compared: positions | predicted | result |
| --- | --- | --- | --- | --- | --- | --- |
| 0.0-0.1 | 1060 | 0.071 | 0.060 | 368 | 0.068 | 0.023 |
| 0.1-0.2 | 4570 | 0.157 | 0.155 | 2091 | 0.161 | 0.105 |
| 0.2-0.3 | 6769 | 0.251 | 0.251 | 5627 | 0.254 | 0.194 |
| 0.3-0.4 | 6632 | 0.348 | 0.364 | 8264 | 0.352 | 0.307 |
| 0.4-0.5 | 6017 | 0.450 | 0.466 | 8527 | 0.450 | 0.439 |
| 0.5-0.6 | 5945 | 0.549 | 0.538 | 8411 | 0.550 | 0.554 |
| 0.6-0.7 | 6169 | 0.651 | 0.637 | 8358 | 0.649 | 0.693 |
| 0.7-0.8 | 6892 | 0.746 | 0.742 | 5500 | 0.745 | 0.811 |
| 0.8-0.9 | 4447 | 0.843 | 0.858 | 1967 | 0.837 | 0.914 |
| 0.9-1.0 | 996 | 0.930 | 0.944 | 384 | 0.930 | 0.966 |

Average gap between prediction and result, weighted by positions: Network 0.0097; Compared 0.0379.

First-layer units over these positions, from both seats: Network, 0 dead (never above 0) and 0 constant (always at full scale) of 256; Compared, 0 dead (never above 0) and 0 constant (always at full scale) of 256.

## Training

| Epoch | Fitted positions, cross-entropy | Held out, cross-entropy | Held out, squared error |
| --- | --- | --- | --- |
| Start | | 0.6971 | 0.1262 |
| 1 | 0.5964 | 0.5888 | 0.0760 |
| 2 | 0.5872 | 0.5844 | 0.0742 |
| 3 | 0.5841 | 0.5822 | 0.0734 |
| 4 | 0.5822 | 0.5802 | 0.0725 |
| 5 | 0.5803 | 0.5781 | 0.0717 |
| 6 | 0.5784 | 0.5764 | 0.0711 |
| 7 | 0.5771 | 0.5754 | 0.0707 |
| 8 | 0.5763 | 0.5749 | 0.0705 |
| Quantized | | 0.5752 | 0.0706 |

## Data

The fitted positions by ply (moves played before them), with the search's root score for the side to move.

| Plies | Positions | Root score's average distance from 0.5 | Root score beyond 0.3 or 0.7 | Beyond 0.1 or 0.9 |
| --- | --- | --- | --- | --- |
| 0-3 | 3905528 | 0.174 | 42.5% | 0.0% |
| 4-7 | 3905528 | 0.147 | 25.4% | 0.0% |
| 8-11 | 4223173 | 0.156 | 30.5% | 0.0% |
| 12-15 | 5067639 | 0.158 | 31.6% | 0.1% |
| 16-23 | 12670161 | 0.156 | 30.9% | 0.2% |
| 24-31 | 16030875 | 0.125 | 19.7% | 0.7% |
| 32+ | 37986515 | 0.114 | 21.7% | 9.8% |

The first player won 48.3% of the 2920239 games, drew 38.7% and lost 13.1%.

Distinct positions: at ply 4, 53000 of 976382; at ply 8, 955847 of 976382; at ply 12, 1166675 of 1188123.
