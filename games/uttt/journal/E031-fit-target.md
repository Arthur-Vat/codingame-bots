---
id: E031
date: 2026-10-16
pull_request: "#151"
parent: uttt-v017
hypothesis: A network fitted to the search's own verdict on each position (the root's score after 5,000 iterations), or to its average with the game's result, learns to rank a position's moves as the search does rather than to predict who wins, so it agrees more with the search and the bot is stronger.
change: None released. The MCTS bot searched as `uttt-v017` does, with the networks of Train runs 38906159700 (target score) and 38906172194 (target mix), each width 256, 8 epochs on run 38799715918's data (E024's, the data of `uttt-v017`'s network), instead of `uttt-v017`'s network (target result).
release: none
sprt: not run: no gain in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Proposed to the owner on 2026-10-16 after [E028](E028-v016-self-play.md), whose network, like E023's, predicted results better but agreed less with the search's moves. He chose it the same day ("Go on with this"), with the plan: two fit-only runs on E024's data, move agreement on fresh games, a screen against `uttt-v017`, and an SPRT only on a gain.

**The target.** Every searched position of the self-play data holds the game's result for the side to move (1, 1/2 or 0) and the root's average score after the search. `fit-nnue --target result` fits the first, as every network so far; `score` fits the second, and `mix` their average. Only the target differs from `uttt-v017`'s network: the same data, split, copies dropped, width, epochs and seed.

**The fits** ([run 38906159700](../training/runs/38906159700/report.md), score; [run 38906172194](../training/runs/38906172194/report.md), mix), on the same held-out positions as `uttt-v017`'s network:

| | Score | Mix | `uttt-v017`'s (result) |
| --- | --- | --- | --- |
| Squared error against the result, all 4.8M held-out positions | 0.0684 | 0.0657 | 0.0652 |
| Move agreement, 49,860 positions spread evenly | 46.2% | 46.2% | 45.9% |
| Calibration gap against the result | 0.0411 | 0.0205 | 0.0110 |
| Dead first-layer units | 0 of 256 | 0 of 256 | 0 of 256 |

Worse at predicting results, as expected of networks not fitted to them, and a little better at choosing the move played, by 0.3 points, within the noise of the 49,860 positions (each game's positions are correlated).

**Move agreement on fresh games** of `uttt-v017`, the 600 games of each kind generated for E028 (seeds unrelated to the runs'), measured with `fit-nnue`'s report on their held-out games:

| Held-out positions | Score | Mix | `uttt-v017`'s |
| --- | --- | --- | --- |
| 8 plies sampled at temperature 2 (1,437 positions) | 46.5% | 45.2% | 47.1% |
| 6 random plies (1,278 positions) | 50.2% | 50.1% | 52.3% |

Lower for both, by 0.6 to 2.2 points, against about 1.3 points of standard error.

**Screening against `uttt-v017`** at CodinGame's limits (Claude's 4-core session, both bots compiled with Rust 1.90.0, 4-ply openings, seed 1, 3 games at a time, 100 pairs each):

| Network | Wins, draws, losses | Elo |
| --- | --- | --- |
| Score | 53, 89, 58 | -8.7 [-42.4, +24.8] |
| Mix | 53, 97, 50 | +5.2 [-24.7, +35.2] |

Each had 1 timeout, `uttt-v017` 3 and 1; 99.9% of the candidates' answers came within 95.1 and 94.6 ms. The bundles took 71,871 characters, as `uttt-v017`'s.

**Decision.** Dropped without an SPRT, per the plan agreed with the owner (an SPRT only on a gain): neither screen shows one, and an SPRT for +10 Elo would likely run for hours to reject or end inconclusive.

**What follows.** On this data, the target barely matters: the three networks play within noise of each other, so what the result labels teach and what the root scores teach is much the same here. E028's loss is then less about the target than about the data itself (`uttt-v016`'s more decisive games). The networks stay on the runs' branches until they are pruned; no bot change was committed.
