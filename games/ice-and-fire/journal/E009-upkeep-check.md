---
id: E009
date: 2026-10-23
pull_request: "#193"
parent: ice-and-fire-v004
hypothesis: The first batch of game reviews saw armies wiped out by debt after buying level 3 units, so a plan search that never makes its income negative beats v004.
change: None released. Local screening only: v004 whose plan search refuses, in the middle game, an action after which the player's income is below zero and lower than before, unless it wins.
release: none
sprt: not run: no gain in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Chosen by the owner on 2026-10-23 ("Drop it, try upkeep check
(Recommended)", after [E008](E008-symmetric-opening.md)), from the ideas of
the first batch of game reviews
([summary](../reviews/2026-10-23-v004-v003/summary.md)): in two of its 20
games, a side bought three level 3 units (upkeep 20 each), its income went
below zero, and debt wiped out its army a few turns later.

## The change

A variant of the summary's third idea (which spared reconnections and the
chain to the HQ): a climb of the plan search refuses an action that leaves
the player's income below zero and lower than it was, unless the game is
won; the chain to the HQ is played before the search, as a lethal move. Moves cost
nothing, and reconnecting cut-off cells raises income, so neither is ever
refused. Two tests covered it (a level 3 refused where a level 2 is not, and
an income already negative that may stay but not fall).

## Screening

Against `ice-and-fire-v004` at CodinGame's limits, seed 171, 200 pairs, two
games at a time: 201 wins, 199 losses (50.2%), **+1.7 Elo [−10.5, +14.0]**,
no fault. 187 of the 200 pairs ended one game each: the two bots mostly
played the same games.

Replayed with the review tool (`ice-and-fire-review`; the run's records
were local and not kept), the 400 games show the rule working: v004's income went below zero in 52 of its games and debt
wiped out its army in 31; the candidate's in 1 and 1.

## What was learned

- **Debt is a symptom here, not a cause.** v004 lost 24 of its 31 games with
  a debt wipe-out; 19 of the 31 came in the last tenth of the game and 28 in
  its second half. The plan search buys level 3 units when the position is
  already lost, to take back what it can; preventing it changes nothing.
- A review's "bad play" can be right about the play and wrong about the
  game: the reviewers judged those purchases bad, and they were, but the
  games had been decided earlier, by the cuts of the early middle game
  ([E006](E006-necks.md), [E007](E007-reserve.md) and the batch's summary).
- The code (`breaks_upkeep` in the plan bot's `search.rs`, and its tests) is
  in this pull request's commit b3ff098, reverted by 0e1329c.
