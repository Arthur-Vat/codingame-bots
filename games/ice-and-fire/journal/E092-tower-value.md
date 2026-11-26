---
id: E092
date: 2026-11-26
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: Towers offered only where they shrink the enemy's best cut, the HQ guard only against a chain the enemy can pay at its next turn, and no value for a tower beyond what it protects, stop the towers that parry no immediate threat, and beat ice-and-fire-v032.
change: Three switches, `GUARD_BY_CUT` and `HQ_GUARD_TURNS` (`bots/plan/src/actions.rs`) and `TOWER_VALUE` (`tactics/src/eval.rs`); each at its v032 value is v032.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner (2026-11-26), on six of v032's CodinGame games against zasmu
shown action by action: some towers, among them those near our HQ, parry
no immediate threat. In game 927639950, at game turn 12, v032 built a
tower at (3, 2), five steps from zasmu: the enemy's best cut after the turn
is 4 cells with or without it (a tower at (6, 4) would have left 2), but it
lowered from 24 to 21 the cells zasmu's chains of trainings could reach,
which the threat term (5/4 a cell) and E026's guard rule
count, and a tower counts 10 in the shared evaluation for its 15 gold.

Of v032's 6,150 towers up to game turn 13 in its 1,950 CodinGame games
(E090's tower study), 464 (8%) stand within 5 steps of its HQ (zasmu: 15 of
672, 2%); of those within 3 steps, 19% lower the enemy's best cut, 55%
only the count of reachable cells, 26% neither.

## The change

- `GUARD_BY_CUT` (off in v032): E026's guard offers a tower only when it
  lowers the active cells of ours the enemy's best cut removes once our
  turn ends, instead of `threatened`, the cells its chains could reach.
  It runs the cut finder for each candidate: 13.8 tries a millisecond on
  the bench bank, against 21.7.
- `HQ_GUARD_TURNS` (2 in v032): the HQ guard offers a tower when the
  enemy's chain to our HQ costs at most its gold plus this many incomes; 1
  is its next turn alone.
- `TOWER_VALUE` (10 in v032): what an active tower counts in the shared
  evaluation; 0 leaves a tower worth only what it protects.

## Screening

Each variant against `ice-and-fire-v032`, 400 pairs, full games (run
45888969443):

| Variant | Settings | Elo | Timeouts |
|---|---|---|---|
| guardcut | `GUARD_BY_CUT` | -60.5 [-78.9, -42.5] | 0 |
| hqnow | `HQ_GUARD_TURNS` 1 | +3.5 [-9.3, +16.2] | 0 |
| value0 | `TOWER_VALUE` 0 | -34.9 [-55.6, -14.3] | 0 |
| hqnow-value0 | both | -34.9 [-56.5, -13.5] | 0 |
| all3 | the three | -58.7 [-81.2, -36.8] | 2 |

Towers offered only against the best cut lose heavily, as does a tower
worth nothing beyond what it protects. The HQ guard on the next turn alone
is level with v032 (its narrow interval: it changes few games). The towers
the owner found useless are not free to remove: the 10 a tower counts, and
the guard on the cells the enemy's chains could reach, stand in for
protection the search's few modelled replies do not see (E087: the replies
it misses are scattered cuts, which a deep tower may still stop). The cut
finder also costs guardcut a third of its tries.

## Round 2: a guard zone near the border

The owner (2026-11-26): a quicker stand-in for guardcut, from a
precomputed distance, since a tower nearer the border should guard best.
`GUARD_NEAR` (0 in v032): E026's guard offers a tower only on cells at
most that many steps, over non-void cells, from the enemy's active cells or
units (the enemy's land and units grown that many steps on bitboards, once
a generation); with `GUARD_NEAR_HQ`, the HQ guard too. 20.1 tries a
millisecond on the bench bank with 2 steps (v032 21.7). In E090's tower
study, 19% of v032's towers stood 3 or more steps from the enemy.

Each variant against `ice-and-fire-v032`, 400 pairs, full games, no
faults on either side (run 45891095696):

| Variant | Settings | Elo |
|---|---|---|
| near2 | `GUARD_NEAR` 2 | +17.4 [+0.6, +34.2] |
| near3 | `GUARD_NEAR` 3 | +17.4 [+1.0, +33.9] |
| near2-hq | `GUARD_NEAR` 2, `GUARD_NEAR_HQ` | +7.8 [-9.6, +25.3] |
| near3-hq | `GUARD_NEAR` 3, `GUARD_NEAR_HQ` | +5.2 [-11.3, +21.7] |

The guard zone without the HQ guard's limit is the first change of the day
whose interval lies above 0 at 400 pairs; binding the HQ guard too gives
part of it back.
