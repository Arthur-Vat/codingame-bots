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
