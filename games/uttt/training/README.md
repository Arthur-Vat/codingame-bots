# Training runs

Reports of the [Train workflow](../../../.github/workflows/train.yml)'s runs that Claude has used, one folder per run: `runs/<run id>/report.md`. Each records the commit and the settings it was trained with, so that it can be repeated, and its measurements. The weights stay on the run's branch until the Prune branches workflow deletes it ([ADR 0021](../../../docs/adr/0021-prune-finished-branches.md)); the weights a bot uses are in the bot.

[class_weights.rs](class_weights.rs) is not a run: it holds the class weights of E011's playout policy (`uttt-v008`, `uttt-v009`), which the bot no longer uses and the trainer's commands (`--policy`), tests and the Train workflow still read.

| Run | What | Used by |
| --- | --- | --- |
| [37795543128](runs/37795543128/report.md) | Value network 64-16, 800,000 games | E012 (dropped) |
| [37814220623](runs/37814220623/report.md) | Value network 128-32, 2,000,000 games, gate of ADR 0019 | ADR 0019's first gate |
| [37901943775](runs/37901943775/report.md) | Move models on 400,000 games of v010's search (run 37895007043's data) | E016 (dropped) |
| [37903237938](runs/37903237938/report.md) | The same data, with the destination's pattern and role | E016 (dropped) |
| [38562652097](runs/38562652097/report.md) | NNUE (189 → 256) × 2 → 10, 8 epochs on run 37814220623's data | E017 |
| [38634035143](runs/38634035143/report.md) | NNUE (189 → 512) × 2 → 10, 12 epochs on run 37814220623's data | E019 (dropped) |
| [38733114779](runs/38733114779/report.md) | NNUE (189 → 256) × 2 → 10, 8 epochs on 2M games of `uttt-v015`'s own search, 8 sampled opening plies | E023 (dropped) |
| [38784586293](runs/38784586293/report.md) | NNUE (189 → 512) × 2 → 10, 12 epochs on run 38733114779's data without its 468,682 copied games | E023 (dropped, screened at 512) |
| [38799715918](runs/38799715918/report.md) | NNUE (189 → 256) × 2 → 10, 8 epochs on 20 jobs of 55,000 games of `uttt-v015`'s own search and 2 side games each, leaving the game by a random move | E024 |
| [38858499582](runs/38858499582/report.md) | NNUE (189 → 512) × 2 → 10, 12 epochs on run 38799715918's data | E025 (dropped) |
| [38876178171](runs/38876178171/report.md) | NNUE (189 → 128) × 2 → 10, 8 epochs on run 38799715918's data | E027 (dropped) |
| [38883498642](runs/38883498642/report.md) | NNUE (189 → 256) × 2 → 30, output buckets by target and game phase, 8 epochs on run 38799715918's data | E029 (dropped) |
| [38875273472](runs/38875273472/report.md) | NNUE (189 → 256) × 2 → 10, 8 epochs on 16 jobs of 240 minutes of `uttt-v016`'s own search and 2 side games each | E028 (dropped) |
