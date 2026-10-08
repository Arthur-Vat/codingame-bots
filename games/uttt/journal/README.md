# Ultimate Tic-Tac-Toe journal

One entry per experiment, failures included, so that ideas are not retried blindly. The process is in [docs/WORKFLOW.md](../../../docs/WORKFLOW.md) and its rules in [ADR 0012](../../../docs/adr/0012-evaluation.md).

- **Entry:** `ENNN-short-name.md`, numbered in order, started from [template.md](template.md).
- **Before the test:** the hypothesis, the change, and the candidate release the pull request adds.
- **After the test:** the SPRT result from the pull request's comment, and the decision. A promoted experiment keeps the line naming its release, which CI requires; a dropped one says `none` there.
- **After pasting:** the rank on CodinGame, reported by the owner.

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
| [E013](E013-opening-book.md) | 2026-10-08 | An opening book of deeper searches for the first moves adds strength | Not run: 20 times more search on the first 4 moves is worth +4.8 Elo [-19.9, +29.6] | Dropped |
