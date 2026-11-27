---
id: E101
date: 2026-11-27
pull_request: "none yet"
parent: ice-and-fire-v034
hypothesis: v034 ends a third of its turns with time left, and spends tries and replies on actions and answers that rarely matter; using the time left (a higher cap on climbs, a second look at the best plans' replies, climbs restarted from the best plan) or pruning (no far trainings, fewer replies) gives the search more of what decides games, and beats ice-and-fire-v034.
change: Switches in `search.rs` and `actions.rs` that leave v034 as it plays, and one variant commit for each, screened against ice-and-fire-v034.
release: none yet
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner asked to prune the search toward the plans and replies that
matter and to use the whole time again (2026-11-27), after E100 made the
same search about 45% cheaper. In 8 local self-play pairs of each release
at CodinGame's speed, from game turn 7, v034 makes 43% more tries a turn
than v033 (602 against 421), not the 80% the instructions allow: 26% of
its turns reach the cap of 32 climbs (`MAX_CLIMBS`; v033 5%), 75% try
every first action (57%), and 31% end before 30 ms (8%).

E099's counts in v033's games: level-1 trainings more than 3 steps from
the enemy's land were 13.6% of the tries and 0.8% of the actions chosen;
bronze's greedy turn decided 2 to 4% of the scores for about 6% of the
turn; one cut in the replies played as well as three (+7.0, level); the
opponent's own climb decided 82% of the scores. E098 found that more
climbs of the opponent for the plan chosen were level (+4.8) when they
took time from the search.

## Change

Switches, each leaving v034 as it plays (the same answers and scores on
the 2,031 positions of v033's CodinGame games at 600 tries):

- `MAX_CLIMBS` (32): the most climbs in a turn.
- `SPARE_CHECK`, `SPARE_PLANS` (none, 3): when the climbs end before
  their deadline, the best plans have their replies searched again with
  that many climbs of the opponent (each barred from the first actions of
  the earlier ones), while the longest reply search of the turn times
  that many still fits before the deadline.
- `RESTART` (off): once every first action has started a climb, the
  climbs start again from the best plan less one of its commands, each in
  turn.
- `FAR_TRAININGS` (none): trainings are offered only on cells at most
  that many steps from the enemy's land (for the opponent's climb too).
- `REPLY_GREEDY` (on) and `REPLY_CUTS` (3): the replies' greedy turn and
  number of cuts.

## Screening

Variants against `ice-and-fire-v034`, 800 seat-swapped pairs each, whole
games at CodinGame's limits, 2 shards
([run 46054521377](https://github.com/Arthur-Vat/codingame-bots/actions/runs/46054521377)):

| Variant | Change | Elo over v034 [95%] |
| --- | --- | ---: |
| `off` | none (control) | pending |
| `cap64` | `MAX_CLIMBS` 64 | pending |
| `capinf` | no cap on climbs | pending |
| `spare` | `SPARE_CHECK` 4 climbs, 3 plans | pending |
| `restart` | `RESTART` | pending |
| `nofar` | `FAR_TRAININGS` 3 | pending |
| `lean` | `REPLY_GREEDY` off, `REPLY_CUTS` 1 | pending |
