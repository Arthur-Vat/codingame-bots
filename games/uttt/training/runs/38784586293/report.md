# NNUE fit

- Data: run 38784586293 of commit b57b879c8d09ae8816120f3dba3a6b8fb28717e8, on the self-play data of run 38733114779
- Games: 2000000, of which 468682 dropped as copies of an earlier game (the same moves); positions: 70904893 fitted, 3734346 held out (every 20th game kept, whole)
- Network: (189 -> 512) x 2 -> 10 buckets, 107530 parameters; quantized as in uttt_engine::nnue, 108084 bytes packed in 54528 characters
- Training: target Result, 12 epochs, batches of 1024, Adam from 0.001 falling to a twentieth, seed 1, 4 threads, 3158 s in all
- Compared network: `games/uttt/bots/mcts/src/nnue_weights.rs`, width 256
- Throughput: 280438 positions per second of training (850858716 positions in 3034 s, without the measurements)

## Held out, all 3734346 positions

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.500 | 0.1113 | 0.6931 |
| Network, float | 0.0714 | 0.6083 |
| Network, quantized | 0.0715 | 0.6084 |
| Compared network, quantized | 0.0781 | 0.6239 |

## Held out, 49792 positions spread evenly

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.500 | 0.1116 | 0.6931 |
| Network, float | 0.0713 | 0.6073 |
| Network, quantized | 0.0714 | 0.6074 |
| Compared network, quantized | 0.0782 | 0.6233 |

## Held out, 49792 positions spread evenly, in detail

The quantized networks, as the bot plays them.

By ply (moves played before the position): squared error against the result, and the share of the moves played that the network would choose, the child it scores best for the side to move. Openings' moves were drawn, not chosen, when self-play sampled them.

| Plies | Positions | Network: squared error | Network: agreement | Compared: squared error | Compared: agreement |
| --- | --- | --- | --- | --- | --- |
| 0-3 | 4116 | 0.0822 | 19.1% | 0.0922 | 15.7% |
| 4-7 | 4104 | 0.0810 | 25.9% | 0.0882 | 25.5% |
| 8-11 | 4060 | 0.0866 | 39.4% | 0.0918 | 42.4% |
| 12-15 | 4063 | 0.0812 | 40.6% | 0.0874 | 42.9% |
| 16-23 | 8220 | 0.0807 | 43.9% | 0.0871 | 46.7% |
| 24-31 | 8184 | 0.0787 | 44.4% | 0.0867 | 45.3% |
| 32+ | 17045 | 0.0524 | 47.5% | 0.0586 | 48.5% |
| All | | 0.0714 | 41.0% | 0.0782 | 42.1% |

Calibration: the positions by predicted score, with the average prediction and the average result.

| Predicted | Network: positions | predicted | result | Compared: positions | predicted | result |
| --- | --- | --- | --- | --- | --- | --- |
| 0.0-0.1 | 146 | 0.081 | 0.034 | 177 | 0.066 | 0.065 |
| 0.1-0.2 | 2681 | 0.167 | 0.157 | 750 | 0.160 | 0.184 |
| 0.2-0.3 | 8144 | 0.251 | 0.257 | 3338 | 0.263 | 0.248 |
| 0.3-0.4 | 7916 | 0.349 | 0.361 | 11851 | 0.354 | 0.303 |
| 0.4-0.5 | 6376 | 0.448 | 0.462 | 9683 | 0.445 | 0.427 |
| 0.5-0.6 | 5944 | 0.551 | 0.535 | 10063 | 0.556 | 0.586 |
| 0.6-0.7 | 8549 | 0.655 | 0.652 | 10726 | 0.646 | 0.713 |
| 0.7-0.8 | 7472 | 0.749 | 0.750 | 2606 | 0.735 | 0.792 |
| 0.8-0.9 | 2425 | 0.832 | 0.841 | 508 | 0.839 | 0.897 |
| 0.9-1.0 | 139 | 0.922 | 0.942 | 90 | 0.929 | 0.967 |

Average gap between prediction and result, weighted by positions: Network 0.0085; Compared 0.0410.

First-layer units over these positions, from both seats: Network, 26 dead (never above 0) and 0 constant (always at full scale) of 512; Compared, 1 dead (never above 0) and 0 constant (always at full scale) of 256.

## Training

| Epoch | Fitted positions, cross-entropy | Held out, cross-entropy | Held out, squared error |
| --- | --- | --- | --- |
| Start | | 0.6966 | 0.1130 |
| 1 | 0.6167 | 0.6162 | 0.0746 |
| 2 | 0.6144 | 0.6146 | 0.0739 |
| 3 | 0.6128 | 0.6130 | 0.0733 |
| 4 | 0.6118 | 0.6124 | 0.0731 |
| 5 | 0.6111 | 0.6119 | 0.0729 |
| 6 | 0.6103 | 0.6112 | 0.0726 |
| 7 | 0.6096 | 0.6105 | 0.0723 |
| 8 | 0.6089 | 0.6098 | 0.0720 |
| 9 | 0.6082 | 0.6091 | 0.0717 |
| 10 | 0.6076 | 0.6088 | 0.0716 |
| 11 | 0.6072 | 0.6085 | 0.0715 |
| 12 | 0.6070 | 0.6083 | 0.0714 |
| Quantized | | 0.6084 | 0.0715 |

## Data

The fitted positions by ply (moves played before them), with the search's root score for the side to move.

| Plies | Positions | Root score's average distance from 0.5 | Root score beyond 0.3 or 0.7 | Beyond 0.1 or 0.9 |
| --- | --- | --- | --- | --- |
| 0-3 | 5819012 | 0.123 | 0.0% | 0.0% |
| 4-7 | 5819012 | 0.113 | 1.5% | 0.0% |
| 8-11 | 5819012 | 0.119 | 5.8% | 0.0% |
| 12-15 | 5819012 | 0.120 | 7.1% | 0.0% |
| 16-23 | 11638011 | 0.122 | 10.2% | 0.0% |
| 24-31 | 11630399 | 0.121 | 15.1% | 0.2% |
| 32+ | 24360435 | 0.106 | 22.8% | 6.5% |

The first player won 43.5% of the 1454753 games, drew 50.2% and lost 6.3%.

Distinct positions: at ply 4, 42891 of 1454753; at ply 8, 1294477 of 1454753; at ply 12, 1290993 of 1454753.
