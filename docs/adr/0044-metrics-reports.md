# 0044. Metrics reports: large batches measured by code, interpreted by Haiku

- Status: accepted (owner, 2026-10-23)
- Date: 2026-10-23
- Completes: [0043](0043-game-reviews-by-haiku.md), decision 5 (another kind of batch beside the reviews of full games)
- Scope: framework

## Context

ADR 0043 set up reviews of full games by Haiku agents, one agent a game, 20 games a batch. On 2026-10-23 the owner found them costly and asked first for many more games read only around their decisive moment. A trial showed that a Haiku agent reading 20 such excerpts, one file at a time, costs about as much as the full reviews it was to replace, since each file it reads stays in its context. The owner then asked instead to "analyze all metrics during the game duration ... many 50 metrics monitored, and do an analysis of those over time ... directly on a large batch ... first with code only, and then an interpretation by a model", and chose in forms the same day: 1,000 games for the first batch, Haiku for the interpretation, and a Markdown report with a page of charts. Seeing the first report, the owner worried that "large aggregates ... could mask tendencies" and asked for the analysis by game length, and for other dimensions.

## Decision

1. **Code measures about 50 metrics a side at every ply.** A game's review tool replays each record and measures, for each side at the start of every ply, its economy, territory, the shape of its territory, where its units stand, the tactical threats on both sides, and what it did in its last turn. Each metric has a one-line meaning in the tool. No model computes a number.
2. **Code turns a large batch into a report.** For each metric the report says from which turn it tells the winner (the winner's value higher, or lower, in at least 70% of the games still running, a tie counting half), how strongly at a few turns, which metrics tell it best at turns 8, 12, 16 and 20, and how the batch's two bots differ turn by turn. The same numbers, turn by turn, feed a page of charts.
3. **The report cuts the batch into slices, and lines games up two ways.** So that an average over the whole batch does not hide a tendency (the owner's concern, 2026-10-23), the report repeats its analysis for slices of the games: by length (the shortest, regular and longest thirds), by seats, by which bot won, and by whether the side ahead at turn 10 kept its lead (forms of the same day: "Seat", "Which bot won", "Comebacks"); and it lines the games up both from their start and on their last turn ("Both views").
4. **One Haiku agent interprets the report.** The project agent `review-summarizer` reads the report alone, not the games, and returns what it says about how these games are won and lost, how the two bots differ, and ideas for the next experiment; the main session saves its reply unchanged.
5. **A batch keeps only its report.** The repository keeps the batch's README, the report and the interpretation; neither the records nor the per-game files, which the arena's seed and the tool rebuild. The charts page is published as a private artifact, linked from the README.
6. **Full reviews stay available.** A batch of full games (ADR 0043) still answers questions about how particular games went; the owner or a session picks the kind of batch.

## Consequences

- A batch costs local compute (playing the games at CodinGame's limits, then measuring them) and one Haiku reply on a report, against one Haiku agent per game for full reviews.
- A metric tells only what it measures: the report shows what goes with winning and how the bots differ, not why a given play was bad. Reading games, by a full batch or by the main session, stays the way to see the why.
- Telling the winner is correlation: a metric can follow from winning (cells late in a game) rather than cause it. The report's turns help tell the two apart, the earlier the more causal, and the interpretation must say so. Early turns carry the seats' difference too: each side is measured at the start of its own turn, and blue starts with one more cell.
- Slices by length, by winner or by lead are cut by the outcome: they show how games of each kind differ, not what caused them to be of that kind.
- Another game gets metrics reports by measuring its own metrics in its review binary; the report is written by the shared crate `cg-review` for every game.
