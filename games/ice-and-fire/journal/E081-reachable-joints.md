---
id: E081
date: 2026-11-21
pull_request: "#NN"
parent: ice-and-fire-v031
hypothesis: A shape term that counts only the joints the enemy can reach soon (or weighs them down with their distance from it) stops the search from training units behind its own lines at the start of the middle game, keeps that gold for the front, and beats ice-and-fire-v031.
change: The shape term's joints are counted only within `SHAPE_NEAR` steps of the enemy's active land (`eval::SHAPE_REACH = 1`), or weighted down from `SHAPE_NEAR` to nothing at `SHAPE_FAR` steps (`SHAPE_REACH = 2`); 0, every joint, in v031.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

The owner saw trainings in the back line on E080's boards (2026-11-21)
and asked for a diagnosis and solutions. With the bot's analysis mode
`explain` (each training of the search's plan against the same plan
without it, term by term of the evaluation) on the 800 positions of game
turns 7 and 8 of 200 games of v031 against itself ([E080](E080-opening-roles.md)):
266 of the 764 trainings (35%) are 6 or more steps from the enemy's land,
(0, 1) as red 160 times, (10, 11) and (11, 9) as blue 84 times; each gains
+31.8, of which the shape term (E010: twice the side's joints plus the
most cells one capture cuts off) gives +39.2 and the rest of the
evaluation −7.4; 258 of the 266 would not pay without it. Our land joins
the HQ through one cell, which then "cuts off" everything; a unit beside
the HQ closes a loop. v031 plays `TRAIN 1 0 1` at game turn 7 in 80 of 100
games as red.

E012 (2026-10-23, on v005) tried a shape term counting only the cuts the
enemy can reach, rather than the joints, and was weaker in local
screening; the bot, the opening and the evaluation have changed much
since, and here the joints and the cut-off cells are both kept, only
weighted by the enemy's distance.

The owner chose (form, 2026-11-21) to build two fixes, "Reachable joints
only" and "Distance-weighted joints", and to judge them offline first,
then on GitHub.

## Change

In `games/ice-and-fire/bots/plan/src/eval.rs`, all off in v031:

- `SHAPE_REACH`: 0, every joint (v031); 1, only the joints at most
  `SHAPE_NEAR` steps (over valid cells) from the enemy's active land; 2,
  each joint weighted fully up to `SHAPE_NEAR` steps, then linearly less to
  nothing at `SHAPE_FAR` (`eval::joint_weight`, in quarters). A joint's
  weight applies to its count and to the cells it cuts off
  (`eval::weakness_weighted`); the cache of the weakness then also keys on
  the enemy's active land.
- The analysis mode `explain` (`bots/plan/src/analysis/explain.rs`,
  `eval::terms`), made for E080's diagnosis.

## Offline

The analysis mode `explain` with each variant's search (1,000 tries) on the
same 800 positions of game turns 7 and 8; trainings of the plans by their
steps from the enemy's active land:

| Variant | Switches | Trainings | 6+ steps | 3 to 5 steps | 1 or 2 steps | Shape gain of the 6+ ones |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| v031 | `SHAPE_REACH = 0` | 764 | 266 | 27 | 471 | +39.2 |
| k3 | 1, near 3 | 671 | 12 | 78 | 581 | 0.0 |
| k4 | 1, near 4 | 711 | 14 | 77 | 620 | 0.0 |
| k6 | 1, near 6 | 686 | 24 | 56 | 606 | +5.7 |
| w26 | 2, near 2, far 6 | 688 | 14 | 60 | 614 | 0.0 |
| w38 | 2, near 3, far 8 | 690 | 14 | 57 | 619 | −0.3 |

Every variant removes about 95% of the trainings behind our lines and
trains more near the enemy instead.

## Local full games

100 pairs a variant against v031 (`ice-and-fire-arena match`, seed
2026112106, time scale 1, tolerance 20 ms, 4 games at once):

| Variant | Elo over v031 [95%] | Answers after the first, median / 99% / 99.9% (v031 in the same games) | Timeouts |
| --- | --- | --- | ---: |
| v031 (control) | +6.9 [−14.6, +28.5] | 32.5 / 44.1 / 48.5 ms (31.3 / 44.2 / 48.3) | 0 |
| k3 | +63.2 [+19.1, +109.5] | 35.3 / 44.3 / 47.2 ms (27.4 / 43.9 / 45.8) | 0 |
| k4 | +59.6 [+15.9, +105.3] | 35.6 / 44.7 / 46.8 ms (28.9 / 44.4 / 46.9) | 1 |
| k6 | +34.9 [−6.7, +77.4] | 36.8 / 44.2 / 46.9 ms (30.5 / 44.1 / 47.0) | 0 |
| w26 | +45.4 [+8.3, +83.6] | 35.9 / 44.5 / 48.2 ms (29.0 / 44.3 / 45.8) | 0 |
| w38 | +34.9 [−10.9, +81.9] | 37.4 / 44.4 / 46.3 ms (31.2 / 44.0 / 47.1) | 0 |

The variants' median answer is a few milliseconds longer (the distances
from the enemy's land, one breadth-first search an evaluated side when the
cache misses), within the turn's 45 ms.

## Screening

The owner approved (form, 2026-11-21) one GitHub run of full games, 8
variants, 800 pairs each against v031 over 2 calibrated runners, seed
2026112107 (run 44889254786; a first run, 44888882551, failed to bundle: the analysis-only `eval::terms` broke the bundler's rule, fixed by moving it into the analysis module). `k` variants count a joint only within that
many steps of the enemy's active land (`SHAPE_REACH = 1`); `w` variants
weight it fully up to the first number of steps, then less to nothing at
the second (`SHAPE_REACH = 2`).

| Variant | Rule | Elo over v031 [95%] | Faults (variant / v031) |
| --- | --- | --- | ---: |
| k2 | ≤ 2 steps | +74.5 [+58.5, +90.8] | 1 / 0 |
| k3 | ≤ 3 steps | +75.4 [+59.2, +92.0] | 0 / 0 |
| k4 | ≤ 4 steps | +80.9 [+64.7, +97.5] | 0 / 0 |
| k5 | ≤ 5 steps | +57.9 [+41.8, +74.1] | 0 / 0 |
| k6 | ≤ 6 steps | +28.3 [+12.7, +44.0] | 0 / 0 |
| w26 | full ≤ 2, none from 6 | +80.9 [+65.1, +97.0] | 0 / 0 |
| w36 | full ≤ 3, none from 6 | +59.2 [+43.3, +75.4] | 0 / 0 |
| w38 | full ≤ 3, none from 8 | +53.0 [+36.8, +69.4] | 0 / 0 |

Every variant beats v031; the nearest cuts (2 to 4 steps) and the steep
weighting (full to 2 steps, none from 6) lead at +75 to +81, and the
further a joint may be from the enemy and still count, the less the gain
(k5 +57.9, k6 +28.3; w36 +59.2, w38 +53.0). The leaders, w26 and k4, and
k3 go on a second seed.
