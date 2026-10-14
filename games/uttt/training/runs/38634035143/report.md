# NNUE fit

- Data: run 38634035143 of commit da4d95cd4de27235f929ab86649235231463067b, on the self-play data of run 37814220623
- Games: 2000000; positions: 79524896 fitted, 4182825 held out (every 20th game, whole)
- Network: (189 -> 512) x 2 -> 10 buckets, 107530 parameters; quantized as in uttt_engine::nnue, 108084 bytes packed in 54528 characters
- Training: target Result, 12 epochs, batches of 1024, Adam from 0.001 falling to a twentieth, seed 1, 4 threads, 3479 s in all
- Throughput: 280405 positions per second of training (954298752 positions in 3403 s, without the measurements)

## Held out, all 4182825 positions

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.487 | 0.1828 | 0.6928 |
| Network, float | 0.1493 | 0.6185 |
| Network, quantized | 0.1493 | 0.6186 |

## Held out, 49796 positions spread evenly

| Predictor | Squared error | Cross-entropy, nats |
| --- | --- | --- |
| The average result of fitted games, 0.487 | 0.1826 | 0.6928 |
| Network, float | 0.1492 | 0.6188 |
| Network, quantized | 0.1493 | 0.6190 |

## Training

| Epoch | Fitted positions, cross-entropy | Held out, cross-entropy | Held out, squared error |
| --- | --- | --- | --- |
| Start | | 0.6977 | 0.1852 |
| 1 | 0.6374 | 0.6300 | 0.1540 |
| 2 | 0.6282 | 0.6273 | 0.1529 |
| 3 | 0.6265 | 0.6265 | 0.1526 |
| 4 | 0.6255 | 0.6256 | 0.1522 |
| 5 | 0.6244 | 0.6244 | 0.1518 |
| 6 | 0.6233 | 0.6233 | 0.1513 |
| 7 | 0.6221 | 0.6222 | 0.1508 |
| 8 | 0.6209 | 0.6209 | 0.1503 |
| 9 | 0.6198 | 0.6199 | 0.1499 |
| 10 | 0.6188 | 0.6193 | 0.1496 |
| 11 | 0.6182 | 0.6187 | 0.1494 |
| 12 | 0.6178 | 0.6185 | 0.1493 |
| Quantized | | 0.6186 | 0.1493 |
