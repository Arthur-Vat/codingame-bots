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

Pending.
