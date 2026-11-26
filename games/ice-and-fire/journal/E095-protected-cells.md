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

Pending.
