# Move models fit

- Data: run 37901943775 of commit a43b5c72e5564fe4fdd4316b0d8aad4b8a1524b6, on the self-play data of run 37895007043
- Games: 400000; positions: 15600135 fitted (119974020 moves), 821775 held out (every 20th game, whole)
- Settings: 4 epochs, batches of 256 positions, Adam at 0.01, penalty 0.00001, seed 1; weights written divided by temperature 0.33; 340 s in all

| Model | Weights | Seen in fitted positions | Held-out cross-entropy, nats | Probability of the search's favourite move | Cross-entropy, weights rounded to quarters | Base64 characters | Entropy-coded, about |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Uniform | 0 | | 1.9026 | 0.167 | | | |
| classes | 32 | 24 | 1.8062 | 0.210 | 1.8083 |  |  |
| patterns | 26275 | 26275 | 1.7331 | 0.238 | 1.7350 | 26275 | 18243 |
| destinations-7 | 36785 | 36778 | 1.7322 | 0.238 | 1.7342 | 36785 | 23159 |
| phases-2 | 52550 | 49540 | 1.7338 | 0.236 | 1.7358 | 52550 | 29612 |
| destination-patterns | 27858 | 27858 | 1.7008 | 0.254 | 1.7045 | 27858 | 18596 |
| rich | 38380 | 38371 | 1.6934 | 0.260 | 1.6972 | 38380 | 21940 |
| large | 64134 | 58200 | 1.6891 | 0.261 | 1.6954 | 64134 | 34017 |

## Held-out cross-entropy by epoch

| Model | By epoch |
| --- | --- |
| classes | 1.8064, 1.8065, 1.8065, 1.8062 |
| patterns | 1.7335, 1.7332, 1.7335, 1.7331 |
| destinations-7 | 1.7326, 1.7322, 1.7326, 1.7322 |
| phases-2 | 1.7343, 1.7340, 1.7342, 1.7338 |
| destination-patterns | 1.7011, 1.7012, 1.7016, 1.7008 |
| rich | 1.6935, 1.6936, 1.6940, 1.6934 |
| large | 1.6896, 1.6897, 1.6898, 1.6891 |
