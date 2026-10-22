# A Code of Ice and Fire journal

One entry per experiment, failures included, so that ideas are not retried blindly. The process is in [docs/WORKFLOW.md](../../../docs/WORKFLOW.md) and its rules in [ADR 0012](../../../docs/adr/0012-evaluation.md).

- **Entry:** `ENNN-short-name.md`, numbered in order, started from [template.md](template.md).
- **Before the test:** the hypothesis, the change, and the candidate release the pull request adds.
- **After the test:** the SPRT result from the pull request's comment, and the decision. A promoted experiment keeps the line naming its release, which CI requires; a dropped one says `none` there.
- **After pasting:** the rank on CodinGame, only when the owner reports one ([ADR 0036](../../../docs/adr/0036-ranks-optional.md)).

## Entries

| Entry | Date | Hypothesis | SPRT | Decision |
| --- | --- | --- | --- | --- |
| [E001](E001-ladder.md) | 2026-10-19 | A ladder of reference bots, each adding one step (lethal chains aware of towers, a defence of the HQ, cuts, banking) to bronze's turn, ends in a bot that beats the greedy `bronze` bot (first release, `ice-and-fire-v001`, from `banker`) | Accepted, +196.5 Elo; +207.0 at full time | Promoted: `ice-and-fire-v001` |
| [E002](E002-plan.md) | 2026-10-21 | A search over the whole turn after the contest's winner (hill climbing over single actions and cuts, against a memory of the opponent's replies), the `plan` bot, beats `ice-and-fire-v001` (candidate `ice-and-fire-v002`) | Accepted, +126.4 Elo at CodinGame's limits | Promoted: `ice-and-fire-v002` |
| [E003](E003-opening.md) | 2026-10-22 | An opening by rules (units take free cells, trainings and mines bought while they pay back) until the territories meet, and a debt check in the evaluation, beat `ice-and-fire-v002` (candidate `ice-and-fire-v003`) | Accepted, +299.8 Elo at CodinGame's limits | Promoted: `ice-and-fire-v003` |
| [E004](E004-planned-opening.md) | 2026-10-22 | v003's opening with the weights, chosen on the first turn from a portfolio, that reach the middle game a heuristic score rates best (the economy, then the Voronoi share) beats v003's fixed weights (candidate `ice-and-fire-v004`) | Accepted, +51.9 Elo at CodinGame's limits | Promoted: `ice-and-fire-v004` |
