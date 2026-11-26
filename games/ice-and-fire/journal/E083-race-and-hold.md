---
id: E083
date: 2026-11-26
pull_request: "#NN"
parent: ice-and-fire-v032
hypothesis: An evaluation that counts the race to the contested cells with units (a unit at the front arrives before a chain of trainings), the map's goal cells of each front (entrance, centre, outpost) a side holds or reaches first, and units that hold the front line, makes the search push and hold the front instead of backing off, and beats ice-and-fire-v032.
change: Three terms of the search's evaluation until game turn 14, from the turn-1 planner of E080 (the owner, 2026-11-26, ideas 3, 4 and 5); each weight at 0 is v032.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

In the turn-1 planner's games against v020 (E080, "Adaptive plans
against v020"), our units backed off as soon as they lost a race, though a
unit standing on a cell blocks the enemy's level-1 units, and we were not
ready for v020's rush. The owner (2026-11-26) chose to bring the
planner's ideas into v032's evaluation, with aggressive play: the goal
cells of each front (3), a race in which units count and ties are not
losses (4), and holding the front line (5).

## The change

`bots/plan/src/front.rs`, three terms counted for a side less for the
enemy, until game turn [`FRONT_UNTIL`] (14, as the contested band of E076):

- **Race** (`RACE`, idea 4): each cell of the contested band (E073) counts
  for the side that arrives first. A side's arrival at a cell: 0 on its
  active land, 1 next to it (a training now), the steps of its nearest
  unit, and the steps from its land plus one through a chain of trainings
  (slower, and paid). Ties count for neither. It replaces the band cells
  held of E073 (`GOAL_OWNED`) in the variants that use it.
- **Goal cells** (`GOAL_E`, `GOAL_C`, `GOAL_O`, idea 3): the map's fronts,
  computed once a map as in E080's planner: the cells within a ring of
  even between the HQs, cut by place into the bottom-left, the middle and
  the top-right, a front with at least 4 of them; its centre, the
  contested cell nearest its middle (the most central for the middle
  front); its entrance, where our most central shortest path from the HQ
  to the centre enters the front's region for good; its outpost, two steps
  past the centre on the most central shortest path to the enemy's HQ. The
  enemy's are their mirrors (the maps are point-symmetric). Each front
  counts, for a side, the weight of its deepest goal cell (outpost, then
  centre, then entrance) the side holds actively or reaches first.
- **Holding the front** (`HOLD`, `IDLE_TO_FRONT`, idea 5): each unit of a
  side on a band cell it holds and the enemy cannot take at its next turn
  (E023's `exposed`) counts `HOLD`; with `IDLE_TO_FRONT`, E060's charge
  on idle units counts their steps to the front (the enemy's cells and the
  band cells not the side's) rather than to the nearest cell not theirs,
  so that units do not walk back to free cells behind the front.

## Screening

Six variants (weights: race; band cells held of E073; goal cells entrance,
centre, outpost; hold; idle toward the front):

| Variant | Commit | Race | Band held | Goal E / C / O | Hold | Idle to front |
| --- | --- | --- | --- | --- | --- | --- |
| race | 034262270b7e | 4 | 0 | 0 | 0 | no |
| goals | 99748648773a | 0 | 4 | 4 / 8 / 12 | 0 | no |
| hold | 8c015cea8695 | 0 | 4 | 0 | 4 | yes |
| all | 9b247402b4b5 | 4 | 0 | 4 / 8 / 12 | 4 | yes |
| all-light | b412b306c341 | 2 | 4 | 4 / 8 / 12 | 4 | yes |
| all-heavy | f86dde085bfc | 6 | 0 | 6 / 12 / 18 | 6 | yes |

A local check of all-heavy against v032, 10 pairs at CodinGame's limits
(seed 83): no fault, answers after the first 41.4 ms median (v032 37.3),
6 wins and 14 losses (too few games to judge).

The screening, on the owner's approval (2026-11-26): `screen.yml` run
45818893819, full games against v032, 400 pairs per variant over 2 shards.
