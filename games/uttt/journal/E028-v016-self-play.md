---
id: E028
date: 2026-10-16
pull_request: "#150"
parent: uttt-v017
hypothesis: A network fitted on self-play of `uttt-v016`, the stronger bot, with E024's side games and jobs that stop together at a time limit, judges positions better than `uttt-v016`'s network (fitted on `uttt-v015`'s games), so the bot is stronger.
change: None released. The MCTS bot searched as `uttt-v017` does, with the network of Train run 38875273472 (width 256, 8 epochs, target result, 16 jobs of 240 minutes of `uttt-v016`'s self-play with 2 side games per game) instead of run 38799715918's.
release: none
sprt: not run: clearly weaker in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Step D2 of [the roadmap](../ROADMAP.md#d-scale): more data of E024's kind, played by the newer release. The owner chose the run on 2026-10-16 ("Go on with the run, and update the handoff. I will start a new session"), within the plan he agreed that day: its report, move agreement on fresh games, a screen, then an experiment with its SPRT.

**The run** ([run 38875273472](../training/runs/38875273472/report.md)), started before `uttt-v017` existed, so its games are `uttt-v016`'s: 16 jobs that each played for 240 minutes (Train's new `minutes` input; every job stopped at 240 minutes), 5,000 iterations per move, 8 sampled opening plies at temperature 2, balance 0.9, 2 side games per game. That gave 3,103,648 games with the side games, of which 29,823 copies were dropped, and 83.8M positions fitted, against about 100M expected. The fit at width 256, 8 epochs, took 10 minutes.

**On its own held-out positions** (4.4M, games of `uttt-v016`), against `uttt-v016`'s network, which `uttt-v017` uses:

| | This network | `uttt-v017`'s |
| --- | --- | --- |
| Squared error | 0.0706 | 0.0728 |
| Cross-entropy, nats | 0.5752 | 0.5814 |
| Move agreement, 49,497 positions spread evenly | 46.2% | 47.3% |
| Calibration gap | 0.0097 | 0.0379 |

Better at predicting results and far better calibrated, but it agrees less with the moves the search played, at every ply from 4 on. The data differs from E024's: `uttt-v016`'s searches are more decisive (the root score's average distance from 0.5 is 0.174 in the first 4 plies, against 0.107 in `uttt-v015`'s games), and the first player wins 48.3% of the games against 39.1%.

**Move agreement on fresh games** of `uttt-v017` (PUCT on), 600 of each kind at 5,000 iterations, seeds unrelated to the run's, measured with `fit-nnue`'s report on its held-out games, both networks on the same positions:

| Held-out positions | Squared error, `uttt-v017`'s network / this one | Move agreement, `uttt-v017`'s / this one |
| --- | --- | --- |
| 8 plies sampled at temperature 2 (1,437 positions) | 0.0816 / 0.0792 | 47.1% / 46.3% |
| 6 random plies (1,278 positions) | 0.0746 / 0.0782 | 52.3% / 50.2% |

Agreement fell on both kinds, by 0.8 and 2.1 points (about 1.3 points of standard error each), the warning sign of E023.

**Screening against `uttt-v017`** at CodinGame's limits (Claude's 4-core session, both bots compiled with Rust 1.90.0, 4-ply openings, seed 1, 3 games at a time, 100 pairs): 36 wins, 82 draws, 82 losses, -81.4 Elo [-112.8, -51.2]. Each bot had 3 timeouts, from a busy machine; 99.9% of answers came within 99.6 ms for both.

**Decision.** Dropped without an SPRT, as the experiment checklist ([`.claude/skills/run-experiment/SKILL.md`](../../../.claude/skills/run-experiment/SKILL.md), step 3) says for a candidate clearly weaker in screening: the whole 95% interval lies below -50 Elo.

**What follows.** Three runs now say the same thing: a network that predicts results better is not a better guide for the search unless it keeps its agreement with the search's moves (E023 lost, E024 kept it and won, this one lost it). Data from a stronger, more decisive bot taught the network to judge results, not to rank a position's moves. Before more self-play, a better target for the fit is worth testing: the root's score (`--target score` or `mix`) instead of the game's result, or a measure of move agreement in the fit itself. The run's data is kept until about 2026-10-23, so fit-only runs on it are possible. The bot change is in this pull request's first commit (e8b7203), and the network stays on the run's branch `claude/train/38875273472` until it is pruned.
