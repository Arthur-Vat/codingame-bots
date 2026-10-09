# Training runs

Reports of the [Train workflow](../../../.github/workflows/train.yml)'s runs that Claude has used, one folder per run: `runs/<run id>/report.md`. Each records the commit and the settings it was trained with, so that it can be repeated, and its measurements. The weights stay on the run's branch until the Prune branches workflow deletes it ([ADR 0021](../../../docs/adr/0021-prune-finished-branches.md)); the weights a bot uses are in the bot.

[class_weights.rs](class_weights.rs) is not a run: it holds the class weights of E011's playout policy (`uttt-v008`, `uttt-v009`), which the bot no longer uses and the trainer's commands (`--policy`), tests and the Train workflow still read.

| Run | What | Used by |
| --- | --- | --- |
| [37795543128](runs/37795543128/report.md) | Value network 64-16, 800,000 games | E012 (dropped) |
| [37814220623](runs/37814220623/report.md) | Value network 128-32, 2,000,000 games, gate of ADR 0019 | ADR 0019's first gate |
| [37901943775](runs/37901943775/report.md) | Move models on 400,000 games of v010's search (run 37895007043's data) | E016 (dropped) |
| [37903237938](runs/37903237938/report.md) | The same data, with the destination's pattern and role | E016 (dropped) |
