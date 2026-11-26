---
id: E095
date: 2026-11-26
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: Valuing the distinct cells our towers protect (each tower's cell and its neighbours, a cell protected twice counting once), near the enemy or everywhere, instead of or beside a flat value per tower, places towers where they guard the front and coordinates them, and beats ice-and-fire-v032.
change: `COVER_WEIGHT` and `COVER_NEAR` in the shared evaluation (`tactics/src/eval.rs`, `covered`), with E092's `TOWER_VALUE` and E093's `EARLY_TOWER_VALUE`, on E093's branch; each at its v032 value is v032.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner (2026-11-26): count the cells our towers protect, the cell and
its neighbours, so that towers are coordinated and a cell protected twice
counts once. v032 counts 10 a tower wherever it stands. At the start of
game turn 13 in v032's 195 CodinGame games against zasmu (boards with two
towers or more), v032's 3.3 towers protect 13.4 active cells counted with
overlaps, 12.1 distinct (10% double); zasmu's 3.0 towers 12.5 and 11.5 (8%).
E092 found that removing the flat value alone loses (-34.9): a value per
protected cell gives it back where the tower guards.

## The change

`tactics/src/eval.rs`: `COVER_WEIGHT` (0 in v032) per distinct active cell
of a side protected by one of its active towers; with `COVER_NEAR`, only
those at most that many steps from the enemy's active cells or units.

## Screening

Each variant against `ice-and-fire-v032`, 1,600 pairs over two shards (the
owner's choice), full games, no faults (run 45911286905):

| Variant | Flat value per tower | Per protected cell | Which cells | Elo |
|---|---|---|---|---|
| cover2 | 0 | 2 | all | +16.3 [+6.2, +26.4] |
| cover3 | 0 | 3 | all | -2.0 [-12.1, +8.2] |
| cover3-near3 | 0 | 3 | within 3 steps of the enemy | -16.5 [-26.6, -6.4] |
| cover5-near3 | 0 | 5 | within 3 steps | -36.4 [-47.2, -25.7] |
| tv10-cover1-near3 | 10 | 1 | within 3 steps | -23.7 [-33.9, -13.6] |
| tv10-cover2-near3 | 10 | 2 | within 3 steps | -20.2 [-31.1, -9.4] |
| tv5-cover2-near3 | 5 | 2 | within 3 steps | -12.6 [-22.7, -2.5] |
| near4-fwd2-cover3-near3 | 0 | 3 | within 3 steps, with E094's near4-fwd2 | -6.5 [-17.1, +4.1] |

cover2, 2 a distinct protected cell in place of 10 a tower, is the first
variant of the day whose interval lies above 0 at 1,600 pairs. Counting
only the cells near the enemy loses in every form, the more the heavier;
3 a cell is level.
