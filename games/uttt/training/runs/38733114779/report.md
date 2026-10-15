# NNUE fit

- Data: run 38733114779 of commit 4b4e5f09007d212866e67dc69e2351d85a48c4a5, 20 jobs of 100000 games at 5000 iterations, 8 opening plies
- Games: 2000000; positions: 93096915 fitted, 4899398 held out (every 20th game, whole)
- Network: (189 -> 256) x 2 -> 10 buckets, 53770 parameters; quantized as in uttt_engine::nnue, 54068 bytes packed in 27264 characters
- Training: target Result, 8 epochs, batches of 1024, Adam from 0.001 falling to a twentieth, seed 1, 4 threads, 1207 s in all
- Throughput: 636760 positions per second of training (744775320 positions in 1170 s, without the measurements)

## Held out, all 4899398 positions

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.500 | 0.1062 | 0.6931 |
| Network, float | 0.0693 | 0.6152 |
| Network, quantized | 0.0693 | 0.6152 |

## Held out, 49994 positions spread evenly

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.500 | 0.1051 | 0.6931 |
| Network, float | 0.0689 | 0.6167 |
| Network, quantized | 0.0689 | 0.6168 |

## Training

| Epoch | Fitted positions, cross-entropy | Held out, cross-entropy | Held out, squared error |
| --- | --- | --- | --- |
| Start | | 0.6962 | 0.1077 |
| 1 | 0.6219 | 0.6205 | 0.0715 |
| 2 | 0.6201 | 0.6195 | 0.0711 |
| 3 | 0.6191 | 0.6189 | 0.0708 |
| 4 | 0.6182 | 0.6178 | 0.0704 |
| 5 | 0.6171 | 0.6168 | 0.0700 |
| 6 | 0.6162 | 0.6161 | 0.0697 |
| 7 | 0.6155 | 0.6154 | 0.0694 |
| 8 | 0.6151 | 0.6152 | 0.0693 |
| Quantized | | 0.6152 | 0.0693 |

## Data

The fitted positions by ply (moves played before them), with the search's root score for the side to move.

| Plies | Positions | Root score's average distance from 0.5 | Root score beyond 0.3 or 0.7 | Beyond 0.1 or 0.9 |
| --- | --- | --- | --- | --- |
| 0-3 | 7600000 | 0.127 | 0.0% | 0.0% |
| 4-7 | 7600000 | 0.117 | 1.3% | 0.0% |
| 8-11 | 7600000 | 0.123 | 5.2% | 0.0% |
| 12-15 | 7600000 | 0.124 | 6.6% | 0.0% |
| 16-23 | 15199992 | 0.126 | 10.1% | 0.0% |
| 24-31 | 15190524 | 0.123 | 15.2% | 0.2% |
| 32+ | 32306399 | 0.102 | 21.8% | 6.2% |

The first player won 41.9% of the 1900000 games, drew 52.5% and lost 5.6%.

Distinct positions: at ply 4, 42854 of 1900000; at ply 8, 1300991 of 1900000; at ply 12, 1297449 of 1900000.
