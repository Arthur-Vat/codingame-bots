---
id: E096
date: 2026-11-27
pull_request: "#389"
parent: ice-and-fire-v032
hypothesis: E095's cover2 (+5.4 [+0.9, +9.9] in its SPRT) and E094's near4-fwd2 (+8.4 [-2.9, +19.7] at 1,200 pairs) change different things, the value of a tower and where towers are offered and built, so together they beat ice-and-fire-v032 by the 10 Elo the SPRT tests.
change: The shared evaluation counts 2 for each distinct active cell a side's towers protect in place of 10 a tower (E095), a tower guarding the cells the enemy can take is offered only within 3 steps of the enemy's active cells or units (E092), and up to game turn 10 the step and guard is also offered from units within 2 steps of them (E093).
release: ice-and-fire-v033
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

E095's candidate, `ice-and-fire-v033` (cover2: 2 for each distinct active
cell a side's towers protect, in place of 10 a tower), ended its SPRT
inconclusive at +5.4 [+0.9, +9.9] after 8,000 pairs (#389). The owner
(2026-11-27) chose to stack E094's near4-fwd2 on it (guard towers only
within 4 steps of the enemy; up to game turn 10, the step and guard also
from units within 2 steps of it) and screen the combination before a new
candidate.

cover2 was chosen on the screening's own games (the game's seed), which is
part of why its screening said +16.3 and its SPRT +5.4. This screening
plays other games (seed 20261127), with cover2 alone as the control.

## The change

The variants were screened on the branch `claude/ice-and-fire-e093-early-hold`,
with the switches of E092 to E095 (the others off). The candidate,
`ice-and-fire-v033`, is cover2-near3-fwd2 alone on #389, the pull request
of E095's first candidate:

- `tactics/src/eval.rs` (E095): `COVER_WEIGHT` (2) for each active cell of
  `covered` (a side's active towers' cells and the cells next to them,
  among its active cells) in place of 10 an active tower.
- `bots/plan/src/actions.rs` (E092): `GUARD_NEAR` (3): E026's guard offers
  a tower for the cells the enemy can take only on cells at most 3 steps,
  over non-void cells, from the enemy's active cells or units
  (`guard_zone`); the HQ guard is not bound by it.
- `bots/plan/src/actions.rs` (E093): up to game turn `EARLY_UNTIL` (10),
  E070's step and guard is also offered from our units on cells at most
  `STEP_GUARD_NEAR` (2) steps from the enemy's active cells or units.

At 300 tries a search, it chooses the same plan as the screened commit
(`2d5027badbcd`) on 300 of v032's CodinGame positions out of 300.

## Screening

Each variant against `ice-and-fire-v032`, 1,600 pairs over two shards,
seed 20261127, full games, no faults (run 45961697357). All carry cover2
(`COVER_WEIGHT` 2, `TOWER_VALUE` and `EARLY_TOWER_VALUE` 0):

| Variant | Added to cover2 | Elo |
|---|---|---|
| cover2 | nothing (the control) | +12.4 [+2.4, +22.4] |
| cover2-near4 | `GUARD_NEAR` 4 | +13.5 [+3.4, +23.6] |
| cover2-fwd2 | `STEP_GUARD_NEAR` 2 | +15.4 [+5.1, +25.8] |
| cover2-near3-fwd2 | `GUARD_NEAR` 3, `STEP_GUARD_NEAR` 2 | +19.1 [+9.0, +29.3] |
| cover2-near4-fwd2 | `GUARD_NEAR` 4, `STEP_GUARD_NEAR` 2 | +9.3 [-0.8, +19.5] |
| cover2-near5-fwd2 | `GUARD_NEAR` 5, `STEP_GUARD_NEAR` 2 | +19.1 [+8.6, +29.7] |
| cover2-near4-fwd3 | `GUARD_NEAR` 4, `STEP_GUARD_NEAR` 3 | +4.1 [-6.2, +14.4] |
| cover2-near4-fwd2-hqnow | the same, `HQ_GUARD_TURNS` 1 | +15.2 [+4.8, +25.6] |

The owner chose cover2-near3-fwd2 as the candidate (2026-11-27). Every variant leans above v032 on these new games, seven of eight with an
interval above 0; cover2 alone measures +12.4 here (+16.3 on the
screening's games, +5.4 in its SPRT). The differences between the variants
(4 to 19) are within their intervals: the guard zone and the early step
and guard add little or nothing that this screening can tell from cover2.
