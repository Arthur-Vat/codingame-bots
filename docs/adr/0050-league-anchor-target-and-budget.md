# 0050. Every league rated from the game's first release, a nightly target of ±10 Elo, and a nightly budget per game

- Status: accepted (owner, 2026-10-31)
- Date: 2026-10-31
- Supersedes: [0038](0038-incremental-league.md), decision 4's anchor (the random bot at 0); [0049](0049-parallel-evaluations-and-nightly-league.md), decision 4's budget shared by all games (10 runners, 3 hours) and decision 4.4's ±15 Elo
- Scope: framework

## Context

[ADR 0038](0038-incremental-league.md) rates every league's bots by Bradley-Terry maximum likelihood with the random bot at 0 Elo, with 95% intervals. [ADR 0049](0049-parallel-evaluations-and-nightly-league.md) stops a night's precision share once every top-10 bot's interval is within ±15 Elo.

On 2026-10-31, the nightly planner tried on A Code of Ice and Fire's league showed that this target can never be met. Every bot beats the random bot in every game, so the random bot's rating is barely bounded. An interval measured from it is that one link's uncertainty, about the same for every bot: ±182 Elo for each of Ice and Fire's bots and about ±84 for Ultimate Tic-Tac-Toe's. No game between other bots shrinks it, and more games against the random bot are all won. Measured from a baseline, the result depends on the game: Ice and Fire's `bronze` gives about ±14, but Ultimate Tic-Tac-Toe's `greedy` still gives about ±75, since every release crushes it too. Measured from each game's first release, `v001`, the top 10's intervals are about ±12 to ±14 for Ice and Fire and ±8 to ±10 for Ultimate Tic-Tac-Toe (both leagues as of 2026-10-31).

The owner chose in the conversation the same day, in forms: "First release v001 (Recommended)" as the anchor rule for every game, and "±10 Elo (Recommended)" as the nightly target, since both games' top 10 were already within ±15 measured from `v001`. Asked then whether the nightly budget should be shared or reserved per game, he chose a budget per game: "For now 6 runners each (12 is divisible by 2,3,4,6), 3h. Then we will see how to split better when we will have many more games".

## Decision

1. **Each game's league is rated with its first release, `v001`, at 0 Elo.** Until a game has a `v001`, its random bot stays the anchor. Ratings then read as Elo above the first release. The leaderboard, the nightly planner's stop rule and its choice of pairings all use this anchor. It is the game's `LEAGUE_ANCHOR` in its `evaluation.env`.
2. **A night's precision share stops once every top-10 bot's interval is within ±10 Elo,** measured from that anchor, and every bot of the pool is rated.
3. **Each game's nightly league has its own budget: 6 runners for at most 3 hours.** A game never uses another's runners or time, even when its own night ends early. How to split the budget once there are many more games is a later decision.

## Consequences

- Every rating on the leaderboards changes by a constant: each game's numbers drop by `v001`'s former rating. The order and the gaps do not change.
- The random bot's own interval becomes the wide one, which is true: its rating is the poorly known one.
- A new game switches its `LEAGUE_ANCHOR` to `v001` in the first pull request that brings `v001`'s league results to `main`, and regenerates its leaderboard there: a release's pull request no longer plays the league ([ADR 0049](0049-parallel-evaluations-and-nightly-league.md), decision 3), so `v001` has no pairings before.
- A game whose top bots are just over ±10 gets precision games for a few nights, then mostly its new bots and the share for the other bots.
- Each game takes 6 of the 20 jobs a free account runs at once at night; the rest is left for CI and an SPRT.
