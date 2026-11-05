# 0052. The league's precision share chosen from the whole covariance of the ratings

- Status: accepted (owner, 2026-11-04)
- Date: 2026-11-04
- Supersedes: [0049](0049-parallel-evaluations-and-nightly-league.md), decision 4.2's "pairs of bots near the top of the ratings whose order is least certain"
- Scope: framework

## Context

[ADR 0049](0049-parallel-evaluations-and-nightly-league.md), decision 4.2, gives 80% of a night's remaining budget to pairs of bots near the top whose order is least certain, so that each night shrinks the top intervals most. [ADR 0050](0050-league-anchor-target-and-budget.md) measures those intervals from the game's first release, `v001`, and stops the share once every top-10 bot is within ±10 Elo.

A Code of Ice and Fire's first four nights (2026-11-01 to 2026-11-04, about 30,000 pairs each, branch `league-results`) barely moved the top intervals: `v014`'s went from ±11.4 to ±11.1 on the last night. The planner had estimated ±4.2 for it after that night. It treated each bot's precision as its own number, so it counted a game between two top bots as if it made both ratings more certain against `v001`. The top bots are hundreds of Elo above `v001`, reached through a few links of the chain of releases. They share the uncertainty of those links, and a game between two of them does not reduce it. A projection from the whole inverse of the Fisher information, the matrix whose diagonal gives the leaderboard's own intervals, predicted the real margins of that night within 0.1 Elo.

On a copy of the results before the last night, a planner that chooses each block from the whole matrix, among any pair of rated bots, projects every top-10 interval at ±8.2 Elo or less after one night, at the nightly settings of `evaluation.env`; among top neighbours only, ±11.0. The owner chose in the conversation on 2026-11-04, in a form: "Full matrix, any pair (Recommended)".

## Decision

1. **The precision share may play any pair of rated bots of the pool.** Each block goes to the pair that shrinks the top bots' intervals most, measured from the anchor, weighted toward the top as before. A pair far below the top, or between a top bot and one far below it, is chosen when it shrinks the top intervals more than a game between neighbours.
2. **The planner reads every interval from the whole covariance of the ratings.** It starts from the covariance of the stored results, the inverse of their Fisher information. It adds the night's lines one by one as if they scored as expected at today's ratings, `new` lines first, then each block it chooses, then the share for the other bots. The margins it reports as estimated after the night are read from that covariance.
3. **The rest of ADR 0049, decision 4, stays.** The `new` lines come first, the other bots keep their 20%, and the whole precision share is still spent while the real target is not met, since real results can leave the margins a little over the projection.

## Consequences

- Most of a night's precision share can go to pairs that are not near the top, such as `v003` against `v001`. Those games are not about the top bots' order: they make every top rating more certain against the anchor.
- A night's report says how far the night should move the top intervals, so a target that one night cannot reach shows in the plan.
- Games between bots far apart in rating teach little each, and the planner weighs that: it chooses them only where nothing else shrinks the top intervals more.
- The planner's choice costs more arithmetic, well under a second a night: one row of the covariance for each candidate pair, and one update of the matrix for each block.
