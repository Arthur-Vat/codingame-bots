# Ultimate Tic-Tac-Toe journal

One entry per experiment, failures included, so that ideas are not retried blindly. The process is in [docs/WORKFLOW.md](../../../docs/WORKFLOW.md) and its rules in [ADR 0012](../../../docs/adr/0012-evaluation.md).

- **Entry:** `ENNN-short-name.md`, numbered in order, started from [template.md](template.md).
- **Before the test:** the hypothesis, the change, and the candidate release the pull request adds.
- **After the test:** the SPRT result from the pull request's comment, and the decision. A promoted experiment keeps the line naming its release, which CI requires; a dropped one says `none` there.
- **After pasting:** the rank on CodinGame, only when the owner reports one ([ADR 0036](../../../docs/adr/0036-ranks-optional.md)).

## Entries

| Entry | Date | Hypothesis | SPRT | Decision |
| --- | --- | --- | --- | --- |
| [E001](E001-mcts.md) | 2026-10-07 | MCTS plays far better than greedy (first release, `uttt-v001`) | Accepted, 60–0 after 30 pairs | Promoted: `uttt-v001` |
| [E002](E002-tree-reuse.md) | 2026-10-07 | Keeping the search tree between turns adds strength (`uttt-v002`) | Accepted, +49.6 Elo at 20 ms, +73.3 at full time | Promoted: `uttt-v002` |
| [E003](E003-solver.md) | 2026-10-07 | Proving wins and losses in the tree adds strength (`uttt-v003`) | Accepted, +31.6 Elo at 20 ms, +13.6 at full time | Promoted: `uttt-v003` |
| [E004](E004-faster-search.md) | 2026-10-07 | A faster search plays better in the same time (`uttt-v004`) | Accepted, +87.5 Elo at 20 ms, +74.4 at full time | Promoted: `uttt-v004` |
| [E005](E005-decisive-playouts.md) | 2026-10-07 | Playouts that take a game-winning move judge positions better (`uttt-v005`) | Accepted, +109.5 Elo at 20 ms, +78.4 at full time | Promoted: `uttt-v005` |
| [E006](E006-faster-search-2.md) | 2026-10-08 | A faster search, again, plays better in the same time (`uttt-v006`) | Accepted, +39.7 Elo at 20 ms, +30.7 at full time | Promoted: `uttt-v006` |
| [E007](E007-time-budget.md) | 2026-10-08 | Searching 90 ms of 100 instead of 82 is worth its rare timeouts (`uttt-v007`) | Accepted, +17.2 Elo at 20 ms, +14.6 at full time, 1 timeout in 1,000 games | Promoted: `uttt-v007` |
| [E008](E008-exploration.md) | 2026-10-08 | Another exploration constant, or one for the late game, suits today's faster search | Not run: local screening found nothing better than 0.5 | Dropped |
| [E009](E009-draw-proofs.md) | 2026-10-08 | Proving draws gives exact values in drawn endgames | Not run: no measurable gain in local screening | Dropped |
| [E011](E011-playout-policy.md) | 2026-10-08 | Playouts whose first moves follow a learned policy judge positions better (`uttt-v008`) | Accepted, +45.4 Elo at 20 ms, +95.4 at full time | Promoted: `uttt-v008` |
| [E012](E012-value-network.md) | 2026-10-08 | A value network trained on 800,000 self-play games makes the bot stronger | Not run: in local screening, -21 Elo at 20 ms and -96 at full time against v008 | Dropped |
| [E013](E013-opening-book.md) | 2026-10-08 | An opening book of deeper searches for the first moves adds strength | Not run: 20 times more search on the first 4 moves is worth +4.8 Elo [-19.9, +29.6] | Dropped |
| [E014](E014-tree-priors.md) | 2026-10-08 | Trying children in the learned policy's order spends visits on better moves (`uttt-v009`) | Accepted, +35.6 Elo at 20 ms, +9.7 at full time | Promoted: `uttt-v009` |
| [E015](E015-pattern-policy.md) | 2026-10-08 | A policy with a weight for each small-board pattern, cell and destination guides playouts and the tree better than 32 classes (`uttt-v010`) | Accepted, +12.8 Elo at 20 ms, +5.6 at full time (first run: +19.2 and +11.8) | Promoted: `uttt-v010` |
| [E016](E016-context-policy.md) | 2026-10-09 | A larger move policy filling the file's free bytes guides playouts better | Not run: better predictions, but 20% fewer iterations; -49 Elo at 20 ms, about -6 at full time | Dropped |
| [E017](E017-nnue-leaves.md) | 2026-10-14 | An NNUE judging the search's new leaves instead of playouts makes the bot stronger (`uttt-v011`) | Accepted, +38.0 Elo at full time | Promoted: `uttt-v011` |
| [E018](E018-jacekmax.md) | 2026-10-14 | Expanding all the children of a node at once, judged by incremental NNUE updates, with minimax values in selection ("jacekmax"), makes the bot stronger (`uttt-v012`) | Accepted, +101.3 Elo at full time | Promoted: `uttt-v012` |
| [E019](E019-nnue-512.md) | 2026-10-14 | An NNUE twice as wide (512) judges positions better and makes the bot stronger | Not run: no gain in local screening, -27.9 Elo at 20 ms and -3.5 at full time against v012 | Dropped |
| [E020](E020-minimax-weight.md) | 2026-10-14 | Selection on half the minimax value and half the average backed up through a child is steadier than the minimax value alone (`uttt-v013`) | Accepted, +88.7 Elo at full time | Promoted: `uttt-v013` |
| [E021](E021-exploration.md) | 2026-10-15 | E020's search needs less exploration: a constant of 0.2 instead of 0.3 (`uttt-v014`) | Accepted, +18.2 Elo at full time | Promoted: `uttt-v014` |
| [E022](E022-walk-updates.md) | 2026-10-15 | Accumulators updated move by move as the search walks down, instead of built from scratch at each expanded node, give the same search more iterations per turn (`uttt-v015`) | Accepted, +7.6 Elo at full time | Promoted: `uttt-v015` |
| [E023](E023-nnue-self-play.md) | 2026-10-15 | A network fitted on the bot's own games (`uttt-v015`, 2M games) judges the positions the bot reaches better | Not run: -177 Elo at full time in local screening (at width 512 on the same data, -56); lower error, lower move agreement | Dropped |
| [E024](E024-side-games.md) | 2026-10-16 | A network fitted on the bot's own games and side games that leave them by a random move judges the positions the search must reject, so the bot is stronger (`uttt-v016`) | Accepted, +58.0 Elo at full time | Promoted: `uttt-v016` |
| [E025](E025-nnue-512-side-games.md) | 2026-10-16 | On E024's data, a network twice as wide (512) judges positions better and makes the bot stronger | Not run: -8.7 Elo at full time in local screening against v016 | Dropped |
| [E026](E026-fused-children.md) | 2026-10-16 | Children evaluated from their parent's accumulators, without copying them, give the same search about 18% more iterations (`uttt-v017`) | Pending; +5.2 Elo at full time in local screening | Pending |
