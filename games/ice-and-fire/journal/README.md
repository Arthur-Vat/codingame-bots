# A Code of Ice and Fire journal

One entry per experiment, failures included, so that ideas are not retried blindly. The process is in [docs/WORKFLOW.md](../../../docs/WORKFLOW.md) and its rules in [ADR 0012](../../../docs/adr/0012-evaluation.md).

- **Entry:** `ENNN-short-name.md`, numbered in order, started from [template.md](template.md).
- **Before the test:** the hypothesis, the change, and the candidate release the pull request adds.
- **After the test:** the SPRT result from the pull request's comment, and the decision. A promoted experiment keeps the line naming its release, which CI requires; a dropped one says `none` there.
- **After pasting:** the rank on CodinGame, only when the owner reports one ([ADR 0036](../../../docs/adr/0036-ranks-optional.md)).

## Entries

| Entry | Date | Hypothesis | SPRT | Decision |
| --- | --- | --- | --- | --- |
| [E001](E001-phases.md) | 2026-10-19 | A turn in phases (lethal chains, a defence of the HQ, cuts, banking) beats the greedy `bronze` bot (first release, `ice-and-fire-v001`) | Pending | Pending |
