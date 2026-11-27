---
id: E096
date: 2026-11-27
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: E095's cover2 (+5.4 [+0.9, +9.9] in its SPRT) and E094's near4-fwd2 (+8.4 [-2.9, +19.7] at 1,200 pairs) change different things, the value of a tower and where towers are offered and built, so together they beat ice-and-fire-v032 by the 10 Elo the SPRT tests.
change: E095's protected-cell value with E092's guard zone and E093's early step and guard, combined on E093's branch; each at its v032 value is v032.
release: pending
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

No new code: the switches of E092 to E095 on E093's branch.

## Screening

Pending.
