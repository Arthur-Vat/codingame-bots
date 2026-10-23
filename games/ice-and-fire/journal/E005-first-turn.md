---
id: E005
date: 2026-10-22
pull_request: "#187 (archive of the code, closed)"
parent: ice-and-fire-v004
hypothesis: More of the first turn's 1,000 ms, spent on a finer choice of the opening's weights or on a better-informed score, gives a better position at the middle game and beats v004.
change: None released. Local screening only: v004 with the opening's weights searched per map, with other settings of the middle-game score, with a look-ahead into the middle game, or with a compactness weight.
release: none
sprt: not run: no gain in local screening
elo: none
pairs: none
full_time: none
decision: dropped
cg_rank: none
---

Asked by the owner on 2026-10-22, right after E004: "Can't you keep going
with experiments to have a good planning on the first turn? Keep doing
experiments with various heuristics, by using different time limits. You
should see an improvement when more time is spent thinking."

## The tools

On the branch of #187 (closed, kept as an archive):

- **A weight search per map** (`rollout::search` in the crate
  `ice-and-fire-opening`, `Planner::Search`): hill climbing over the
  opening's ten weights from the portfolio's best set, each candidate
  judged by a rollout to the middle game against v003's opening and E004's
  score.
- **Settings of the score** (`score::Settings`): how many turns into the
  middle game the economy is counted, the gold of a Voronoi cell, the
  weight of gold.
- **A look-ahead into the middle game** (`Planner::Deep`): the choice of
  weights judged after a few turns of the plan search itself, which moved
  into a crate of its own, `ice-and-fire-search`, for this.
- **A compactness weight** in the opening: a free cell's value grows with
  the number of our cells next to it.

## Results

**Time, in the harness** (E004's: maps 2,001 to 2,300, both seats; the
score of each player against v004's choice of weights, econ, fill and
rush, and their mean): v004's opening +19.1; the weight search at 20 ms
+18.1, 100 ms +19.5, 300 ms +19.6, 600 ms +19.6. The search finds its best
by 100 ms; more time changes nothing.

**Against `ice-and-fire-v004`** at CodinGame's limits (seed 111, 200 pairs
each, two games at a time on a 4-CPU machine):

| Variant | Elo | Faults |
| --- | --- | --- |
| Weight search, 300 ms | −3.5 [−26.6, +19.6] | none |
| Economy counted 1 turn into the middle game (v004: 3) | −27.9 [−55.7, −0.4] | 2 timeouts |
| Economy counted 5 turns in | −1.7 [−27.0, +23.5] | none |
| A Voronoi cell worth 0 gold (v004: 1) | −5.2 [−34.4, +23.9] | 1 timeout |
| A Voronoi cell worth 3 gold | −24.4 [−52.6, +3.6] | none |
| Gold counted twice | −36.6 [−67.1, −6.6] | none |
| Look-ahead, 2 turns × 50 plans, 600 ms | −36.6 [−70.2, −3.7] | none |
| Look-ahead, 3 turns × 100 plans, 900 ms | −12.2 [−45.9, +21.4] | 40 timeouts |
| Compactness weight 2 | −6.9 [−37.5, +23.5] | 1 timeout |
| Compactness weight 4 | −20.9 [−52.3, +10.2] | none |

None is better than v004; the look-ahead at 900 ms also ran out of time in
a tenth of its games.

## What was learned

- More time on the first turn does not help this opening: the weight
  search saturates by 100 ms, and v004's portfolio choice (about 20 ms) is
  as good in the arena.
- To find out why, the owner asked for a qualitative study ("simulate maybe
  100 games ... try to interpret what is going on"; "Text boards, ~30
  maps"): [research/game-study.md](../research/game-study.md). The
  positions at the end of the opening are the same, by every measure, in
  the games v004 goes on to win and to lose; games are decided later, by a
  cut through a neck of the territory that grows in the early middle game.
  A score of the opening's end cannot see a difference that is not there.
- The study's tool, the arena example `boards`, is in #187. Its proposed
  heuristics led to [E006](E006-necks.md) and [E007](E007-reserve.md).
