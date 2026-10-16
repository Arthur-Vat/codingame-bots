---
id: E024
date: 2026-10-16
pull_request: "#142"
parent: uttt-v015
hypothesis: A network fitted on the bot's own games together with side games that leave each game by a random legal move also learns the positions after moves the bot would not play, which the search must judge to reject them, so it agrees more with the search's moves and the bot is stronger.
change: The MCTS bot searches as `uttt-v015` does, with the network of Train run 38799715918 (width 256, 8 epochs, target result, 20 jobs of 55,000 games of `uttt-v015` and 2 side games each) instead of run 38562652097's.
release: uttt-v016
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

Step D2 of [the roadmap](../ROADMAP.md#d-scale), after [E023](E023-nnue-self-play.md). The owner chose the data on 2026-10-15 ("D2: new training data", "Random moves in self-play", run size "20 x 55,000, 2 side"): #141 added side games to self-play, and the plan was that Train run, a check of move agreement, then one experiment.

**The data and the fit** ([run 38799715918](../training/runs/38799715918/report.md)): 20 jobs of 55,000 games of `uttt-v015`'s own search at 5,000 iterations, 8 opening plies sampled at temperature 2, balance 0.9, and 2 side games per game. That gave 3,232,297 games with the side games, of which 15,974 copies were dropped, and 91.9M positions fitted. On all its held-out positions (4.8M), the network scored:
- squared error 0.0652 against 0.0697 for `uttt-v015`'s network;
- cross-entropy 0.6176 against 0.6285.

On the report's sample of 49,860 of them, spread evenly:
- move agreement 45.9% against 44.4%;
- calibration gap 0.0110 against 0.0195;
- no dead units.

**Move agreement on fresh games**, before any game against the release, as E023 advised. 600 new games of `uttt-v015` at 5,000 iterations of each kind (seeds unrelated to the run's), measured with `fit-nnue`'s report on its held-out games, both networks on the same positions:

| Held-out positions | Squared error, `uttt-v015`'s network / this one | Move agreement, `uttt-v015`'s / this one | Dead units of 256 |
| --- | --- | --- | --- |
| 8 plies sampled at temperature 2 (1,534 positions) | 0.0637 / 0.0626 | 39.3% / 39.2% | 7 / 3 |
| 6 random plies (1,347 positions) | 0.0450 / 0.0399 | 46.1% / 48.8% | 3 / 2 |

E023's network had lost 4 points of agreement on such positions and had 31 to 39 dead units. This one keeps its agreement on the bot's own openings and gains on random ones, where the positions are less like the bot's games, with fewer dead units. The samples are small (30 held-out games of each kind, about 1.3 points of standard error on agreement).

**Screening against `uttt-v015`** at CodinGame's limits (Claude's 4-core session, 4-ply openings, seed 1, 3 games at a time, 100 pairs): 70 wins, 90 draws, 40 losses, +52.5 Elo [+21.1, +84.8]. The candidate had 2 timeouts and `uttt-v015` none; 99.9% of answers came within 98.2 ms for the candidate and 97.8 ms for `uttt-v015`, on a busy machine. Only the weights changed, so the bot's speed is the same.

The bundle takes 70,456 characters, as before.
