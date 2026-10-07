# Ultimate Tic-Tac-Toe journal

One entry per experiment, failures included, so that ideas are not retried blindly. The process is in [docs/WORKFLOW.md](../../../docs/WORKFLOW.md) and its rules in [ADR 0012](../../../docs/adr/0012-evaluation.md).

- **Entry:** `ENNN-short-name.md`, numbered in order, started from [template.md](template.md).
- **Before the test:** the hypothesis, the change, and the candidate release the pull request adds.
- **After the test:** the SPRT result from the pull request's comment, and the decision. A promoted experiment keeps the line naming its release, which CI requires; a dropped one says `none` there.
- **After pasting:** the rank on CodinGame, reported by the owner.

## Entries

| Entry | Date | Hypothesis | SPRT | Decision |
| --- | --- | --- | --- | --- |
| [E001](E001-mcts.md) | 2026-10-07 | MCTS plays far better than greedy (first release, `uttt-v001`) | pending | pending |
