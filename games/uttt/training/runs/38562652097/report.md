# NNUE fit

- Data: run 38562652097 of commit b2c9e125678366416d52478ebe8082c75ccd79e6, on the self-play data of run 37814220623
- Games: 2000000; positions: 79524896 fitted, 4182825 held out (every 20th game, whole)
- Network: (189 -> 256) x 2 -> 10 buckets, 53770 parameters; quantized as in uttt_engine::nnue, 54068 bytes packed in 27264 characters
- Training: target Result, 8 epochs, batches of 1024, Adam from 0.001 falling to a twentieth, seed 1, 4 threads, 1572 s in all
- Throughput: 415618 positions per second of training (636199168 positions in 1531 s, without the measurements)

## Held out, all 4182825 positions

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.487 | 0.1828 | 0.6928 |
| Network, float | 0.1498 | 0.6197 |
| Network, quantized | 0.1498 | 0.6199 |

## Held out, 49796 positions spread evenly

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.487 | 0.1826 | 0.6928 |
| Network, float | 0.1497 | 0.6201 |
| Network, quantized | 0.1498 | 0.6202 |

## Training

| Epoch | Fitted positions, cross-entropy | Held out, cross-entropy | Held out, squared error |
| --- | --- | --- | --- |
| Start | | 0.6979 | 0.1853 |
| 1 | 0.6402 | 0.6327 | 0.1552 |
| 2 | 0.6300 | 0.6285 | 0.1534 |
| 3 | 0.6272 | 0.6266 | 0.1526 |
| 4 | 0.6254 | 0.6248 | 0.1519 |
| 5 | 0.6236 | 0.6231 | 0.1512 |
| 6 | 0.6219 | 0.6213 | 0.1504 |
| 7 | 0.6205 | 0.6202 | 0.1500 |
| 8 | 0.6197 | 0.6197 | 0.1498 |
| Quantized | | 0.6199 | 0.1498 |
