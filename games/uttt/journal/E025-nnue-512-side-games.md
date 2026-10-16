---
id: E025
date: 2026-10-16
pull_request: "#143"
parent: uttt-v016
hypothesis: On E024's data, which covers positions after moves the bot would not play, a network twice as wide (512) judges positions better than `uttt-v016`'s 256, and that outweighs its slower evaluations, so the bot is stronger.
change: None released. The MCTS bot searched as `uttt-v016` does, with the network of Train run 38858499582 (width 512, 12 epochs on run 38799715918's data) instead of run 38799715918's 256.
release: none
sprt: not run: no gain in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Step D2 of [the roadmap](../ROADMAP.md#d-scale), the first step of the width plan agreed on 2026-10-15: fit 512 on the newest data and test it against the 256. The owner chose it on 2026-10-16 ("Go on with option 1"), after [E024](E024-side-games.md).

**The fit** ([run 38858499582](../training/runs/38858499582/report.md)) is fit-only on E024's data (run 38799715918, the same split and copies dropped): 12 epochs instead of 8, 81 minutes. Compared with `uttt-v016`'s network on the same held-out positions:

| | This network (512) | `uttt-v016`'s (256) |
| --- | --- | --- |
| Squared error, all 4.8M held-out positions | 0.0646 | 0.0652 |
| Cross-entropy, nats | 0.6160 | 0.6176 |
| Move agreement, 49,860 positions spread evenly | 46.0% | 45.9% |
| Calibration gap | 0.0125 | 0.0110 |
| Dead first-layer units | 33 of 512 | 0 of 256 |

A little better at predicting results, the same at choosing the search's move. Doubling the width bought less here than going from E023's data to E024's did (0.0697 to 0.0652).

**Screening against `uttt-v016`** at CodinGame's limits (Claude's 4-core session, 4-ply openings, seed 1, 3 games at a time, 100 pairs): 54 wins, 87 draws, 59 losses, -8.7 Elo [-39.1, +21.6]. Each bot had 5 timeouts, from a busy machine; 99.9% of answers came within 95.6 ms for the candidate and 99.9 ms for `uttt-v016`. With the network, the bundle took 97,725 characters (`Network<512>` and `Mcts<NnueBoard<512>>` in the bot's `main.rs`).

**Decision.** The owner dropped it without an SPRT on 2026-10-16 ("Drop it, no SPRT (Recommended)"). An SPRT for +10 Elo would likely have run for hours to reject or end inconclusive.

**What follows.** As in E019, a wider network's small gain in prediction does not pay for its slower evaluation: each evaluation costs about twice as much, and the network took two thirds to three quarters of the time at 256. So, per the plan of 2026-10-15, width waits, and the effort goes to evaluation speed (step D3) and to data. The entropy coder and the low-bit weights, planned only if 512 won, wait too.

The bot change is in this pull request's first commit, and the network stays on the run's branch `claude/train/38858499582` until it is pruned.
