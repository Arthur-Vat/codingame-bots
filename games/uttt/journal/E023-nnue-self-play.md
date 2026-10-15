---
id: E023
date: 2026-10-15
pull_request: "#140"
parent: uttt-v015
hypothesis: A network of the same width fitted on the bot's own games (`uttt-v015`'s search and network, 2M games) instead of `uttt-v010`'s judges the positions the bot reaches better, so the bot is stronger.
change: None released. The MCTS bot searched as `uttt-v015` does, with the network of Train run 38733114779 (width 256, 8 epochs, target result) instead of run 38562652097's.
release: none
sprt: not run: -177.2 Elo [-223.9, -135.8] in local screening at full time
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Step D2 of [the roadmap](../ROADMAP.md#d-scale). The owner asked for it on 2026-10-15 ("Merge it and fit it and do the experiment"), after #139.

The network is [run 38733114779](../training/runs/38733114779/report.md)'s:
- **Data:** 2M games of `uttt-v015`'s own search at 5,000 iterations, openings of 8 plies drawn in proportion to the visits.
- **Fit:** held-out squared error 0.0693 and cross-entropy 0.6152, against 0.1062 and 0.6931 for the constant.
- **Copies:** about a third of those games copied another (fixed for later runs by #139). This network was fitted before that fix.

**Screening against `uttt-v015`** (Claude's 4-core session, 4-ply openings, seed 1, 3 games at a time):
- **At 20 ms per move (time scale 0.2), 200 pairs:** -51.6 Elo [-85.5, -18.7], but with 153 and 146 timeouts in 400 games, so it says little.
- **At CodinGame's limits, 100 pairs:** 31 wins, 44 draws, 125 losses, -177.2 Elo [-223.9, -135.8]. Each bot had 1 timeout, and 99.9% of answers came within 96.6 ms. Clearly weaker, so no release and no SPRT.

**Why.** Both networks were measured with `fit-nnue`'s report (#139) on held-out positions of 600 fresh games of `uttt-v015` at 5,000 iterations, in two kinds:

| Held-out positions | Squared error, `uttt-v015`'s network / this one | Move agreement, `uttt-v015`'s / this one | Dead units of 256 |
| --- | --- | --- | --- |
| 8 plies sampled at temperature 2 (1,473 positions) | 0.0722 / 0.0713 | 42.4% / 38.2% | 6 / 31 |
| 6 random plies (1,320 positions) | 0.0592 / 0.0586 | 44.6% / 40.2% | 6 / 39 |

The new network predicts the results of the bot's games a little better. But it chooses the move the search played less often. That share counts how often its best-scored child is the move played.

The search judges every child of a node, including the bad moves that self-play never plays. A network fitted only on the positions of strong, narrow games learns to judge the positions the bot plays into, not the ones it must reject. The data's sameness shows in its many dead units.

As in E019, a lower error did not play better, and move agreement tracked strength here where the error did not.

**What follows**, for the owner to choose:
- data that also covers the positions after moves the bot would not play, for example random moves inside self-play games, or positions recorded from the children the search judges;
- a mix of these games with games of weaker or more varied play;
- move agreement checked on such positions before any experiment, since the fit's report now gives it: on this data, both networks that played weaker had the lower agreement, whatever their error.

**The same data at width 512.** The owner also asked for the planned fit at 512 on this data. [Run 38784586293](../training/runs/38784586293/report.md) fitted it for 12 epochs, after #139 dropped the 468,682 copied games, 23% of the 2M.
- **On its held-out positions:** squared error 0.0715 against 0.0781 for `uttt-v015`'s network, and a calibration gap of 0.0085 against 0.0410.
- **But:** move agreement 41.0% against 42.1%, and 26 dead units of 512.
- **Screened** the same way (`Network<512>` in the bot's `main.rs`, 97,725 characters): 52 wins, 64 draws, 84 losses in 100 pairs, -56.1 Elo [-95.8, -17.7], 1 timeout each. Weaker too, though by less than the 256.

The width helps on this data, but the data is what holds both networks back, so no 512-against-256 test was worth an SPRT.

The networks stay on the runs' branches, `claude/train/38733114779` and `claude/train/38784586293`, and their reports are in `training/runs/`.
