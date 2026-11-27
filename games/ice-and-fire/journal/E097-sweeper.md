---
id: E097
date: 2026-11-27
pull_request: "#NN"
parent: ice-and-fire-v033
hypothesis: A layer before the search that sends each unit behind the front to its own free cell, on its own path, takes the free land far from the front as the top players do without pulling units from the middle, and beats ice-and-fire-v033.
change: `sweep::sweep` matches our level-1 units more than `DIST` steps from the enemy's land to free cells as far from it, nearest pairs first, each cell and unit once, and steps each matched unit toward its cell (a neutral step first, never onto another unit's step); with `sweep::MODE` 1 its moves are played first and the search plans the rest of the turn, with 2 they are offered to the search as one macro action; 0, nothing, in v033.
release: pending
sprt: pending
elo: pending
pairs: pending
full_time: pending
decision: pending
cg_rank: not reported
---

## Why

E096 measured, on v032's CodinGame games against the top 10, that our
units crowd at game turns 8 to 12 and stand still later, that we take the
free land far from the front later than zasmu and karliso, and that the
more of our units rush into the middle the less far land we take; but no
term in the search's evaluation closed the gap (far land −16 to −64, unit
room level at a weight of 1, a unit kept at home level). The owner asked
for a structural change (2026-11-27). The search spends its time where the
score moves, near the front; units behind it get what is left. The top
players behave as if their units had roles.

## Change

`bots/plan/src/sweep.rs`, from game turn `FROM` (7, the search's first)
to `UNTIL` (20):

1. The units behind the front: ours, level 1, on active cells more than
   `DIST` (4) steps from the enemy's land, that can still move.
2. Their targets: the neutral cells as far from the enemy's land.
3. A matching: every (unit, target) pair within `RANGE` (6) steps, nearest
   first; a pair is kept when neither its unit nor its target is taken. No
   two units head for the same cell.
4. Each kept unit steps onto a neighbour one step nearer its target, a
   neutral one first (taken on the way), never a cell another unit steps
   onto or one of our units stands on: units spread over the free land
   rather than walk the same path (the owner's collaborative snake).
5. The moves the engine accepts, in order.

`session.rs` uses them by `MODE`: 1, fixed, plays them on a copy of the
position and lets the search plan the rest of the turn from there, the
sweeper's moves sent first; 2, offered, gives them to the search as one
macro action at the climbs' root (E077's guide), which it takes only when
it scores better.

## Local check

Pending: each form against v033 in local games, measured for far land, the
middle and crowding with E096's tools, before any screening (the owner's
order).
