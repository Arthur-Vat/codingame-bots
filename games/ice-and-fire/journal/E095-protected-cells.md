---
id: E095
date: 2026-11-26
pull_request: "#389"
parent: ice-and-fire-v032
hypothesis: Valuing the distinct cells our towers protect (each tower's cell and its neighbours, a cell protected twice counting once), near the enemy or everywhere, instead of or beside a flat value per tower, places towers where they guard the front and coordinates them, and beats ice-and-fire-v032.
change: The shared evaluation (`tactics/src/eval.rs`) counts 2 for each distinct active cell a side's towers protect (`COVER_WEIGHT`, `covered`) in place of 10 for each of its active towers.
release: ice-and-fire-v033
sprt: inconclusive
elo: "+5.4 [+0.9, +9.9]"
pairs: 8000
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

The variants were screened on the branch `claude/ice-and-fire-e093-early-hold`
(with the switches of E092 to E094, the dropped experiments of the same
day, all off), where `COVER_WEIGHT` (0 in v032) counted each distinct
active cell of a side protected by one of its active towers, `COVER_NEAR`
limited them to those near the enemy, and `TOWER_VALUE` and
`EARLY_TOWER_VALUE` held the 10 a tower.

The candidate, `ice-and-fire-v033`, is cover2 alone on `main`:
`tactics/src/eval.rs` counts `COVER_WEIGHT` (2) for each active cell of
`covered` (a side's active towers' cells and the cells next to them, among
its active cells) in place of 10 an active tower. At 300 tries a search, it
chooses the same plan as the screened commit (`fdbf02798bdd`) on 300 of
v032's CodinGame positions out of 300.

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

## SPRT

On #389 (run 45953687366), against `ice-and-fire-v032`, in blocks of 10
pairs over 8 calibrated runners:

```
ice-and-fire-v033 vs ice-and-fire-v032: 16000 games, 8000 complete pairs
  ice-and-fire-v033: 8124 wins, 0 draws, 7876 losses (score 50.8%)
  Elo of ice-and-fire-v033 over ice-and-fire-v032: +5.4 [+0.9, +9.9] (95%, from pairs)
  ice-and-fire-v033 faults: 3 timeouts, 0 crashes, 0 invalid answers
  ice-and-fire-v032 faults: 1 timeouts, 0 crashes, 0 invalid answers
SPRT elo0=0 elo1=10 alpha=0.05 beta=0.05: LLR 0.74 (bounds -2.94, 2.94) after 8000 pairs
Verdict: INCONCLUSIVE after 8000 pairs
```

A gain, its interval above 0, but about a third of the screening's +16.3
and below the +10 the test is set for. The owner (2026-11-27): stack E094's
near4-fwd2 on cover2 and screen the combination before a new candidate.

