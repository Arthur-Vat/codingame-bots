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
